#!/usr/bin/env python3
"""Skill-compression eval runner.

Provenance: .mochiko/brainstorms/skill-compression-tooling/record.md (D8 as amended).
Maintainer-side advisory tooling (GI-019 trace recorded); never shipped (GI-020).

One run = one isolated claude -p session loading a synthesized minimal plugin that
carries only the skill variant under test (R4), executing one golden prompt in a
throwaway workspace. Grading: scripted assertions (deterministic) + rule-coverage
checklist via a Haiku judge (binary + quoted evidence, pass^k) + a position-swapped
pairwise Sonnet read. Judges are advisory (harness D2): judged degradation never sets a
nonzero exit code.

Execution modes (probe-settled, R5):
- sandbox (default): sessions run inside the Docker AI sandbox `claude-mochiko` via
  `sbx exec`, using the sandbox's stored subscription auth — no API key. `--bare` is
  dropped there (it skips stored credentials by design); isolation comes from a neutral
  cwd inside the sandbox (/tmp/eval-*), outside every project-discovery ancestor chain.
  The plugin is staged host-side under evals/.work/ — workspaces mount at identical
  absolute paths inside the sandbox.
- --local: the original `claude -p --bare` path; requires ANTHROPIC_API_KEY (metered).
"""

import argparse
import datetime
import json
import pathlib
import re
import shlex
import shutil
import subprocess
import sys
import tempfile
import uuid

REPO = pathlib.Path(__file__).resolve().parent.parent
EVALS = REPO / "evals"
WORK = EVALS / ".work"
PLUGIN_SKILLS = REPO / "plugins" / "mochiko" / "skills"

SANDBOX = "claude-mochiko"  # Docker AI sandbox name (sbx CLI); --local disables
SESSION_MODEL = "sonnet"    # model under test (R7)
CHECKLIST_MODEL = "haiku"   # checklist judge (D8)
PAIRWISE_MODEL = "sonnet"   # pairwise judge (D8)
MAX_TURNS = 30
ARMS = ["noskill", "baseline", "armA", "armB"]

LOCAL_MODE = False  # set by --local


def die(msg: str) -> None:
    print(f"error: {msg}", file=sys.stderr)
    sys.exit(1)


def chars(path: pathlib.Path) -> int:
    """Ledger-canonical measure: chars, never `wc -c` bytes."""
    return len(path.read_text())


def synth_plugin(dest: pathlib.Path, skill_name: str, source: pathlib.Path | None) -> pathlib.Path:
    """Minimal per-run plugin dir carrying only the skill under test (R4).

    source=None synthesizes the no-skill control plugin — same manifest, empty skills/ —
    so every arm runs with identical flags.
    """
    plug = dest / "eval-plugin"
    (plug / ".claude-plugin").mkdir(parents=True)
    (plug / ".claude-plugin" / "plugin.json").write_text(json.dumps({
        "name": "eval-subject",
        "version": "0.0.0",
        "description": "synthesized per-run eval plugin (never shipped)",
    }))
    (plug / "skills").mkdir()
    if source is not None:
        shutil.copytree(source, plug / "skills" / skill_name)
    return plug


def arm_source(skill: str, arm: str) -> pathlib.Path | None:
    if arm == "noskill":
        return None
    if arm == "baseline":
        return PLUGIN_SKILLS / skill
    return EVALS / skill / "variants" / arm


def sbx_sh(script: str, timeout: int = 1800) -> subprocess.CompletedProcess:
    return subprocess.run(["sbx", "exec", SANDBOX, "sh", "-c", script],
                          capture_output=True, text=True, timeout=timeout)


def parse_stream(stdout: str) -> dict:
    init, result = None, None
    for line in stdout.splitlines():
        line = line.strip()
        if not line:
            continue
        try:
            ev = json.loads(line)
        except json.JSONDecodeError:
            continue
        if ev.get("type") == "system" and ev.get("subtype") == "init":
            init = ev
        if ev.get("type") == "result":
            result = ev
    return {"init": init, "result": result}


def claude_args(prompt: str, model: str, max_turns: int, stream: bool,
                plugin: pathlib.Path | None) -> list:
    # Recorded divergence from the record's "dontAsk" wording: I2 found dontAsk denies
    # writes absent allow rules; R5 mandates settling flags empirically, and acceptEdits
    # is the build's call under that mandate. The probe verifies it.
    args = ["claude", "-p", prompt, "--model", model,
            "--permission-mode", "acceptEdits", "--max-turns", str(max_turns),
            # Probe finding: the sandbox carries a user-level mochiko plugin install, which
            # would load the real skill beside the variant. Empty setting-sources drops
            # user/project config (and with it installed plugins) while keeping stored auth.
            "--setting-sources", "",
            "--output-format", "stream-json" if stream else "json"]
    if stream:
        args += ["--verbose"]
    if LOCAL_MODE:
        args.insert(1, "--bare")  # local mode: hermetic, needs ANTHROPIC_API_KEY
    if plugin is not None:
        args += ["--plugin-dir", str(plugin)]
    return args


def run_session(prompt: str, *, model: str, plugin: pathlib.Path | None,
                max_turns: int = MAX_TURNS, stream: bool = False,
                want_workspace: bool = False) -> dict:
    """One claude -p session. Returns {init, result, workspace(Path|None)}."""
    if LOCAL_MODE:
        with tempfile.TemporaryDirectory(prefix="eval-") as td:
            ws = pathlib.Path(td) / "ws"
            ws.mkdir()
            proc = subprocess.run(claude_args(prompt, model, max_turns, stream, plugin),
                                  cwd=ws, capture_output=True, text=True, timeout=1800)
            if proc.returncode != 0 and not proc.stdout.strip():
                die(f"claude spawn failed (exit {proc.returncode}): {proc.stderr[-2000:]}")
            out = parse_stream(proc.stdout) if stream else {"init": None,
                                                            "result": json.loads(proc.stdout)}
            if want_workspace:
                keep = WORK / f"ws-{uuid.uuid4().hex[:12]}"
                shutil.copytree(ws, keep)
                out["workspace"] = keep
            return out
    # sandbox mode
    rid = uuid.uuid4().hex[:12]
    ws_sbx = f"/tmp/eval-{rid}"
    inner = shlex.join(claude_args(prompt, model, max_turns, stream, plugin))
    proc = sbx_sh(f"mkdir -p {ws_sbx} && cd {ws_sbx} && {inner}")
    if proc.returncode != 0 and not proc.stdout.strip():
        die(f"sbx exec failed (exit {proc.returncode}): {proc.stderr[-2000:]}")
    payload = proc.stdout
    if not stream:
        m = re.search(r"\{.*\}", payload, re.DOTALL)  # wrapper may prepend status lines
        out = {"init": None, "result": json.loads(m.group(0)) if m else {}}
    else:
        out = parse_stream(payload)
    if want_workspace:
        keep = WORK / f"ws-{rid}"
        keep.mkdir(parents=True)
        cp = sbx_sh(f'cp -a {ws_sbx}/. {shlex.quote(str(keep))}/')
        if cp.returncode != 0:
            print(f"warn: artifact copy-back failed for {ws_sbx}: {cp.stderr[-300:]}",
                  file=sys.stderr)
        out["workspace"] = keep
    sbx_sh(f"rm -rf {ws_sbx}", timeout=60)
    return out


def extract_json(text: str):
    """Lenient JSON extraction from judge output."""
    m = re.search(r"\{.*\}|\[.*\]", text, re.DOTALL)
    if not m:
        return None
    try:
        return json.loads(m.group(0))
    except json.JSONDecodeError:
        return None


def run_assertions(assertions: list, workspace: pathlib.Path) -> list:
    out = []
    for a in assertions:
        kind = a["type"]
        ok, detail = False, ""
        target = workspace / a.get("path", "")
        if kind == "file_exists":
            ok = target.is_file()
        elif kind == "contains":
            ok = target.is_file() and re.search(a["pattern"], target.read_text()) is not None
        elif kind == "not_contains":
            ok = target.is_file() and re.search(a["pattern"], target.read_text()) is None
        else:
            detail = f"unknown assertion type {kind}"
        out.append({**a, "passed": ok, "detail": detail})
    return out


JUDGE_CHUNK = 15  # rules per judge call — an 82-entry single-shot array was ~9% unparseable


def judge_checklist(rules: list, artifact_text: str, expected_output: str = "") -> list:
    """One binary per rule, quoted evidence. Advisory. Chunked + one retry per chunk
    (staged-001 finding: single-shot 82-rule arrays hit ~9% parse/MISSING failures,
    which pass^k reads as kills)."""
    out = []
    for i in range(0, len(rules), JUDGE_CHUNK):
        chunk = rules[i:i + JUDGE_CHUNK]
        prompt = (
            "You are grading an artifact against a rule checklist. For EACH of the "
            f"{len(chunk)} rules return a JSON array entry "
            "{\"id\": ..., \"passed\": true|false, \"evidence\": \"<verbatim quote "
            "from the artifact proving the verdict, or empty>\"}. Binary only, no scores. "
            "Every rule id must appear exactly once. Output ONLY the JSON array.\n\nRULES:\n"
            + json.dumps(chunk, indent=1)
            + ("\n\nEXPECTED OUTPUT (golden's intent, context only):\n" + expected_output
               if expected_output else "")
            + "\n\nARTIFACT:\n" + artifact_text[:120_000]
        )
        byid = {}
        for attempt in range(2):  # one retry on parse failure / missing ids
            res = run_session(prompt, model=CHECKLIST_MODEL, plugin=None, max_turns=1)
            verdicts = extract_json(res["result"].get("result", "") or "")
            if isinstance(verdicts, list):
                for v in verdicts:
                    if isinstance(v, dict) and v.get("id") and v.get("passed") is not None:
                        byid.setdefault(v["id"], v)
            if all(r["id"] in byid for r in chunk):
                break
        out += [byid.get(r["id"], {"id": r["id"], "passed": None, "evidence": "MISSING"})
                for r in chunk]
    return out


def cmd_rejudge(skill: str, out: str) -> None:
    """Re-score stored artifacts with the current judge — no sessions, judges only."""
    base = EVALS / skill
    rundir = base / "runs" / out
    summ = rundir / "summary.json"
    if not summ.is_file():
        die(f"{summ} missing")
    data = json.loads(summ.read_text())
    rules = json.loads((base / "rules.json").read_text())
    goldens = {g["id"]: g for g in json.loads((base / "evals.json").read_text())}
    results = data["runs"]
    for e in results:
        art_file = rundir / f"{e['arm']}-{e['golden']}-r{e['replicate']}.artifact.txt"
        if not art_file.is_file():
            print(f"skip {art_file.name}: artifact missing")
            continue
        print(f"rejudge {e['arm']}/{e['golden']}/r{e['replicate']} ...", flush=True)
        e["checklist"] = judge_checklist(
            rules, art_file.read_text(),
            goldens.get(e["golden"], {}).get("expected_output", ""))
    arms = [a for a in ARMS if any(e["arm"] == a for e in results)]
    total_cost = sum(e.get("cost_usd") or 0.0 for e in results)
    summary = summarize(skill, arms, data["replicates"], rules, results, total_cost)
    summary["pairwise"] = data.get("pairwise", [])
    summ.write_text(json.dumps({"runs": results, **summary}, indent=1))
    (rundir / "report.md").write_text(render_report(skill, out, summary))
    print(f"rejudged: {rundir / 'report.md'}")


def judge_pairwise(golden_id: str, text_a: str, text_b: str) -> dict:
    """Blind A/B with position swap. Advisory."""
    def ask(first, second):
        prompt = (
            "Two artifacts answer the same task. Which is better overall? Reply ONLY JSON "
            "{\"winner\": \"1\"|\"2\"|\"tie\", \"reason\": \"<one sentence>\"}.\n\n"
            "ARTIFACT 1:\n" + first[:60_000] + "\n\nARTIFACT 2:\n" + second[:60_000]
        )
        res = run_session(prompt, model=PAIRWISE_MODEL, plugin=None, max_turns=1)
        return extract_json(res["result"].get("result", "") or "") or {}
    v1 = ask(text_a, text_b)
    v2 = ask(text_b, text_a)  # position swap
    w1, w2 = v1.get("winner"), v2.get("winner")
    agree = (w1 == "1" and w2 == "2") or (w1 == "2" and w2 == "1") or (w1 == w2 == "tie")
    return {"golden": golden_id, "first_order": v1, "swapped": v2, "position_consistent": agree}


def collect_artifact(workspace: pathlib.Path) -> str:
    parts = []
    for f in sorted(workspace.rglob("*")):
        if f.is_file() and f.stat().st_size < 200_000:
            parts.append(f"=== {f.relative_to(workspace)} ===\n{f.read_text(errors='replace')}")
    return "\n\n".join(parts) or "(no files produced)"


def stage_plugin(skill: str, arm: str) -> pathlib.Path:
    src = arm_source(skill, arm)
    stage = WORK / f"plug-{arm}-{uuid.uuid4().hex[:8]}"
    stage.mkdir(parents=True)
    return synth_plugin(stage, skill, src)


def cmd_probe(skill: str) -> None:
    """R5: settle invocation mechanics empirically before any priced grid."""
    src = arm_source(skill, "baseline")
    if not src.is_dir():
        die(f"no such skill: {src}")
    plug = stage_plugin(skill, "baseline")
    try:
        res = run_session("Create a file named probe.txt containing exactly: probe-ok",
                          model=SESSION_MODEL, plugin=plug, max_turns=6, stream=True,
                          want_workspace=True)
        init, result, ws = res["init"], res["result"], res.get("workspace")
        print(f"probe findings ({'local' if LOCAL_MODE else 'sandbox ' + SANDBOX}):")
        print(f"  init event seen:      {init is not None}")
        if init:
            print(f"  plugin_errors:        {init.get('plugin_errors', 'FIELD-ABSENT')}")
            skills = init.get("skills") or init.get("slash_commands") or []
            hit = [s for s in map(str, skills) if skill in s or "eval-subject" in s]
            print(f"  skill visible:        {hit or f'NOT FOUND in {len(skills)} entries'}")
        wrote = ws is not None and (ws / "probe.txt").is_file()
        print(f"  write landed:         {wrote}")
        if result:
            print(f"  total_cost_usd:       {result.get('total_cost_usd')}")
            print(f"  is_error:             {result.get('is_error')}")
        if not wrote:
            print("  NOTE: write denied even under acceptEdits — investigate before any grid.")
        if ws:
            shutil.rmtree(ws, ignore_errors=True)
    finally:
        shutil.rmtree(plug.parent, ignore_errors=True)


def cmd_grid(skill: str, replicates: int, arms: list, out: str | None = None) -> None:
    base = EVALS / skill
    prereg = base / "preregistration.md"
    if not prereg.is_file():
        die(f"{prereg} missing — the ship bar must be pre-registered before any grid (R6/R9)")
    if len(prereg.read_text().strip()) < 200:
        die(f"{prereg} looks empty — record the ship bar and delivered-chars arithmetic first")
    goldens = json.loads((base / "evals.json").read_text())
    rules = json.loads((base / "rules.json").read_text())
    for arm in arms:
        s = arm_source(skill, arm)
        if arm != "noskill" and not (s and s.is_dir()):
            die(f"arm {arm}: source missing ({s})")

    stamp = out or datetime.datetime.now().strftime("%Y%m%d-%H%M%S")
    rundir = base / "runs" / stamp
    rundir.mkdir(parents=True, exist_ok=True)
    results, total_cost = [], 0.0
    prior = rundir / "summary.json"
    if prior.is_file():  # staged grid: keep prior arms' results, replace rerun arms
        old = json.loads(prior.read_text())
        results = [e for e in old.get("runs", []) if e["arm"] not in arms]
        total_cost = sum(e.get("cost_usd") or 0.0 for e in results)
        print(f"appending to {stamp}: kept {len(results)} prior runs "
              f"({sorted({e['arm'] for e in results})})")

    for arm in arms:
        plug = stage_plugin(skill, arm)
        try:
            for g in goldens:
                for rep in range(1, replicates + 1):
                    label = f"{arm}/{g['id']}/r{rep}"
                    print(f"run {label} ...", flush=True)
                    res = run_session(g["prompt"], model=SESSION_MODEL, plugin=plug,
                                      stream=True, want_workspace=True)
                    init, result, ws = res["init"], res["result"], res.get("workspace")
                    artifact = collect_artifact(ws) if ws else "(no workspace)"
                    truncated = len(artifact) > 120_000
                    cost = (result or {}).get("total_cost_usd") or 0.0
                    total_cost += cost
                    entry = {
                        "arm": arm, "golden": g["id"], "replicate": rep,
                        "plugin_errors": (init or {}).get("plugin_errors"),
                        "assertions": run_assertions(g.get("assertions", []), ws)
                                      if ws else [],
                        "checklist": judge_checklist(rules, artifact,
                                                     g.get("expected_output", "")),
                        "cost_usd": cost,
                        "artifact_truncated": truncated,
                    }
                    (rundir / f"{arm}-{g['id']}-r{rep}.artifact.txt").write_text(artifact)
                    results.append(entry)
                    if ws:
                        shutil.rmtree(ws, ignore_errors=True)
        finally:
            shutil.rmtree(plug.parent, ignore_errors=True)

    all_arms = [a for a in ARMS if any(e["arm"] == a for e in results)]
    pairwise = []
    if "baseline" in all_arms:
        art = lambda arm, gid: (rundir / f"{arm}-{gid}-r1.artifact.txt")
        for arm in [a for a in all_arms if a not in ("noskill", "baseline")]:
            for g in goldens:
                fa, fb = art("baseline", g["id"]), art(arm, g["id"])
                if fa.is_file() and fb.is_file():
                    v = judge_pairwise(g["id"], fa.read_text(), fb.read_text())
                    pairwise.append({"arm": arm, **v})

    summary = summarize(skill, all_arms, replicates, rules, results, total_cost)
    summary["pairwise"] = pairwise
    (rundir / "summary.json").write_text(json.dumps({"runs": results, **summary}, indent=1))
    (rundir / "report.md").write_text(render_report(skill, stamp, summary))
    print(f"\nreport: {rundir / 'report.md'}  (est. spend ${total_cost:.2f} — client-side estimate)")
    # Advisory posture: judged degradation never fails the process. Mechanical baseline
    # assertion failures do.
    if summary["baseline_assertion_failures"]:
        sys.exit(2)


def summarize(skill, arms, replicates, rules, results, total_cost) -> dict:
    held = {}  # (arm, rule_id) -> pass^k over all goldens x replicates
    for arm in arms:
        for r in rules:
            verdicts = [v.get("passed") for e in results if e["arm"] == arm
                        for v in e["checklist"] if v.get("id") == r["id"]]
            held[(arm, r["id"])] = bool(verdicts) and all(v is True for v in verdicts)
    pruned = [r["id"] for r in rules if held.get(("noskill", r["id"]))]  # R3
    live = [r for r in rules if r["id"] not in pruned]
    lost = {arm: [r["id"] for r in live
                  if held.get(("baseline", r["id"])) and not held.get((arm, r["id"]))]
            for arm in arms if arm not in ("noskill", "baseline")}
    floor_lost = {arm: [i for i in ids if any(r["id"] == i and r.get("class") == "floor"
                                              for r in rules)]
                  for arm, ids in lost.items()}
    parse_failures = sum(1 for e in results for v in e["checklist"] if v.get("passed") is None)
    truncations = sum(1 for e in results if e.get("artifact_truncated"))
    baseline_fail = [f"{e['golden']}/r{e['replicate']}: {a.get('path', a['type'])}"
                     for e in results if e["arm"] == "baseline"
                     for a in e["assertions"] if not a["passed"]]
    return {
        "skill": skill, "arms": arms, "replicates": replicates,
        "rules_total": len(rules), "rules_pruned_by_noskill": pruned,
        "rules_lost_per_arm": lost, "floor_rules_lost_per_arm": floor_lost,
        "baseline_assertion_failures": baseline_fail,
        "judge_parse_failures": parse_failures,
        "artifact_truncations": truncations,
        "held": {f"{a}:{rid}": v for (a, rid), v in held.items()},
        "est_total_cost_usd": round(total_cost, 2),
        # Body loads on invoke; references load on demand — reported separately, never
        # summed into one "delivered" figure (R9 arithmetic lives in preregistration.md).
        "skill_chars_per_arm": {
            arm: {"body": chars(src / "SKILL.md") if (src / "SKILL.md").is_file() else 0,
                  "references": sum(chars(f) for f in src.rglob("references/*.md"))}
            if (src := arm_source(skill, arm)) and src.is_dir() else {"body": 0, "references": 0}
            for arm in arms},
    }


def render_report(skill, stamp, s) -> str:
    lines = [f"# Eval report — {skill} ({stamp})", "",
             f"Rules: {s['rules_total']} total, {len(s['rules_pruned_by_noskill'])} pruned by "
             f"the no-skill control (they measure the model, not the skill).", ""]
    for arm, ids in s["rules_lost_per_arm"].items():
        floors = s["floor_rules_lost_per_arm"].get(arm, [])
        verdict = "KILLED (floor rule lost)" if floors else (
            f"{len(ids)} rules lost" if ids else "no rules lost")
        lines.append(f"- **{arm}** — {verdict}" + (f": {', '.join(ids)}" if ids else ""))
    for pv in s.get("pairwise", []):
        w1 = pv["first_order"].get("winner"); w2 = pv["swapped"].get("winner")
        lines.append(f"- pairwise {pv['arm']}/{pv['golden']}: {w1}/{w2} "
                     f"({'position-consistent' if pv['position_consistent'] else 'POSITION-INCONSISTENT'})")
    for arm, c in s.get("skill_chars_per_arm", {}).items():
        if c["body"]:
            lines.append(f"- chars {arm}: body {c['body']:,} · references {c['references']:,}")
    lines += ["", f"Baseline scripted-assertion failures: "
              f"{s['baseline_assertion_failures'] or 'none'}",
              f"Judge parse failures: {s.get('judge_parse_failures', 0)} · "
              f"artifact truncations: {s.get('artifact_truncations', 0)}",
              f"Estimated spend: ${s['est_total_cost_usd']} (client-side estimate)", "",
              "Judged results are ADVISORY. The ship decision is the user's ratification "
              "against `preregistration.md`."]
    return "\n".join(lines)


def cmd_report(skill: str) -> None:
    runs = sorted((EVALS / skill / "runs").glob("*/summary.json"))
    if not runs:
        die("no runs found")
    data = json.loads(runs[-1].read_text())
    print(render_report(skill, runs[-1].parent.name,
                        {k: data[k] for k in data if k != "runs"}))


def main() -> None:
    global LOCAL_MODE
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--local", action="store_true",
                    help="run with --bare + ANTHROPIC_API_KEY instead of the sandbox")
    sub = ap.add_subparsers(dest="cmd", required=True)
    for name in ("probe", "grid", "report", "rejudge"):
        p = sub.add_parser(name)
        p.add_argument("skill")
        if name == "rejudge":
            p.add_argument("--out", required=True)
        if name == "grid":
            p.add_argument("--replicates", type=int, default=3)
            p.add_argument("--arms", default=",".join(ARMS))
            p.add_argument("--out", default=None,
                           help="run-dir name under runs/ to append into (staged grids)")
    args = ap.parse_args()
    LOCAL_MODE = args.local
    WORK.mkdir(exist_ok=True)
    if args.cmd == "probe":
        cmd_probe(args.skill)
    elif args.cmd == "grid":
        cmd_grid(args.skill, args.replicates, args.arms.split(","), args.out)
    elif args.cmd == "rejudge":
        cmd_rejudge(args.skill, args.out)
    else:
        cmd_report(args.skill)


if __name__ == "__main__":
    main()
