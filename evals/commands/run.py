#!/usr/bin/env python3
# /// script
# dependencies = ["pyyaml"]
# ///
"""Command plan-only eval runner.

Provenance: .mochiko/brainstorms/command-plan-only-eval/record.md (D1-D11, accepted
2026-08-27) with the brainstorm-probe amendments (evals/commands/brainstorm-probe/).
Maintainer-side advisory tooling (GI-019 trace via the harness session); never shipped
(GI-020). Sibling of the skill runner evals/run.py; judge patterns adapted from it.

One run = one headless `claude -p` session in an ephemeral workdir (fixture files +
a provisioned plugins/mochiko tree), invoking the command as the prompt under a pinned
form-only wrapper (D11). The session plans; it never executes (allow-list fence, D7;
user gates described, never awaited, D9). Grading: deterministic asserts (load gate,
name resolution, cap-hit) + a Haiku rule-coverage checklist over the D8 plan-observable
subset + a stub-detection axis + a position-swapped Sonnet pairwise read. Judges are
advisory (harness D2): judged degradation never sets a nonzero exit code; only broken
mechanics (missing prereg, load-gate failure) do.

Probe-settled invocation (2026-08-27): NO --bare (it skips stored auth by design);
isolation = --setting-sources "" + neutral cwd; --allowedTools Read,Grep,Glob is a
permission fence (roster stays visible, calls are denied); max-turns 40 with cap-hit
warning.

Usage (run via `uv run evals/commands/run.py ...`):
  partition <cmd> --old-ref <git-ref>       four ID-keyed rubric buckets (D6)
  check-rubric <cmd>                        observable.yaml covers the schema exactly (D8)
  check-fixtures <cmd>                      every path a fixture references exists
  plan-run <cmd> <golden-id> [--arm post|pre|nocmd] [--old-ref REF] [--out DIR]
  grid <cmd> [--replicates 3] [--old-ref REF] [--control] [--out NAME]
  judge <cmd> <run-name>                    coverage + stub + pairwise over stored plans
  report <cmd> <run-name>                   bucket diff, pass^k, noise guard
"""

import argparse
import datetime
import hashlib
import json
import pathlib
import re
import shutil
import subprocess
import sys
import tempfile

try:
    import yaml
except ImportError:
    print("error: PyYAML unavailable — run via `uv run evals/commands/run.py ...`",
          file=sys.stderr)
    sys.exit(2)

REPO = pathlib.Path(__file__).resolve().parent.parent.parent
CMD_EVALS = REPO / "evals" / "commands"
PLUGIN = REPO / "plugins" / "mochiko"
WRAPPER = CMD_EVALS / "wrapper.md"

SESSION_MODEL = "sonnet"    # session under test (skill-harness R7 carried over)
CHECKLIST_MODEL = "haiku"   # coverage + stub judge
PAIRWISE_MODEL = "sonnet"   # pairwise judge
MAX_TURNS = 40              # probe finding 10: 25 hit the cap; headroom + warning
JUDGE_CHUNK = 15            # skill-harness staged-001 finding: big arrays misparse
ARMS = ["pre", "post", "nocmd"]


def die(msg: str) -> None:
    print(f"error: {msg}", file=sys.stderr)
    sys.exit(1)


# ---------- schema access ----------

def schema_text(cmd: str, old_ref: str | None = None) -> str:
    rel = f"plugins/mochiko/schemas/{cmd}.yaml"
    if old_ref is None:
        return (REPO / rel).read_text()
    proc = subprocess.run(["git", "-C", str(REPO), "show", f"{old_ref}:{rel}"],
                          capture_output=True, text=True)
    if proc.returncode != 0:
        die(f"git show {old_ref}:{rel} failed: {proc.stderr.strip()}")
    return proc.stdout


def common_blocks(old_ref: str | None = None) -> dict:
    """{common.<slug>: block} from schemas/common.yaml, or {} where the file does not
    exist (pre-extends refs). Same resolution semantics as the command .md instructs:
    a stub inherits every field; a locally declared field replaces the inherited one."""
    rel = "plugins/mochiko/schemas/common.yaml"
    if old_ref is None:
        path = REPO / rel
        if not path.is_file():
            return {}
        raw = path.read_text()
    else:
        proc = subprocess.run(["git", "-C", str(REPO), "show", f"{old_ref}:{rel}"],
                              capture_output=True, text=True)
        if proc.returncode != 0:
            return {}
        raw = proc.stdout
    doc = yaml.safe_load(raw)
    return {b["id"]: b for b in doc.get("rules") or []}


def extract_rules(cmd: str, old_ref: str | None = None) -> dict:
    """{id: {text(resolved), class, labels, section}} from a command schema.
    Resolves `extends: common.<slug>` stubs against schemas/common.yaml; ${var}
    substitution runs on the resolved text from the COMMAND's vars block."""
    doc = yaml.safe_load(schema_text(cmd, old_ref))
    variables = doc.get("vars") or {}
    commons = None
    out = {}
    for sec in doc.get("sections") or []:
        for r in sec.get("rules") or []:
            if r.get("extends"):
                if commons is None:
                    commons = common_blocks(old_ref)
                base = commons.get(r["extends"])
                if base is None:
                    die(f"{r['id']} extends unknown block {r['extends']}")
                r = {**base, **r}
            if "text" not in r:
                die(f"{r['id']} has no text and no resolvable extends")
            text = r["text"]
            for k, v in variables.items():
                text = text.replace("${%s}" % k, str(v))
            out[r["id"]] = {"text": text.strip(), "class": r.get("class"),
                            "labels": r.get("labels") or [], "section": sec["id"]}
    if not out:
        die(f"no rules extracted for {cmd}" + (f" at {old_ref}" if old_ref else ""))
    return out


def partition(cmd: str, old_ref: str) -> dict:
    """D6 four-bucket, ID-keyed. Mechanical, no LLM."""
    old, new = extract_rules(cmd, old_ref), extract_rules(cmd)
    return {
        "unchanged": sorted(i for i in old.keys() & new.keys()
                            if old[i]["text"] == new[i]["text"]),
        "changed": sorted(i for i in old.keys() & new.keys()
                          if old[i]["text"] != new[i]["text"]),
        "removed": sorted(old.keys() - new.keys()),
        "added": sorted(new.keys() - old.keys()),
    }


def load_rubric(cmd: str) -> dict:
    """D8 observable subset (the rubric) + out-of-instrument remainder, validated
    against the live schema: every rule in exactly one list, no unknown IDs."""
    path = CMD_EVALS / cmd / "observable.yaml"
    if not path.is_file():
        die(f"{path} missing — the D8 partition must exist before judging")
    doc = yaml.safe_load(path.read_text())
    obs = {e["id"]: e.get("why", "") for e in doc.get("observable") or []}
    out = {e["id"]: e.get("why", "") for e in doc.get("out_of_instrument") or []}
    rules = extract_rules(cmd)
    dupes = obs.keys() & out.keys()
    unknown = (obs.keys() | out.keys()) - rules.keys()
    missing = rules.keys() - obs.keys() - out.keys()
    if dupes or unknown or missing:
        die(f"observable.yaml invalid — dupes:{sorted(dupes)} unknown:{sorted(unknown)} "
            f"uncovered:{sorted(missing)}")
    return {"observable": obs, "out_of_instrument": out, "rules": rules}


# ---------- fixtures & goldens ----------

def load_goldens(cmd: str) -> list:
    path = CMD_EVALS / cmd / "evals.json"
    if not path.is_file():
        die(f"{path} missing")
    return json.loads(path.read_text())


def fixture_dir(cmd: str, golden: dict) -> pathlib.Path:
    d = CMD_EVALS / cmd / "fixtures" / golden["fixture"]
    if not d.is_dir():
        die(f"fixture dir missing: {d}")
    return d


def check_fixtures(cmd: str) -> int:
    """Probe finding 9: every path a fixture file references must exist in that
    fixture (internal consistency). References = markdown links + backtick paths."""
    bad = 0
    for g in load_goldens(cmd):
        fx = fixture_dir(cmd, g)
        for f in sorted(fx.rglob("*")):
            if not f.is_file():
                continue
            text = f.read_text(errors="replace")
            refs = re.findall(r"\]\(([^)#:]+?\.(?:md|yaml|json))\)", text)
            refs += re.findall(r"`((?:\.mochiko|\.claude|contracts)[\w./-]+?\.(?:md|yaml|json))`", text)
            for ref in refs:
                target = (f.parent / ref) if not ref.startswith((".mochiko", ".claude")) \
                    else (fx / ref)
                if not target.exists():
                    print(f"BROKEN {f.relative_to(fx)} -> {ref}  ({g['fixture']})")
                    bad += 1
    print(f"fixture consistency: {'OK' if bad == 0 else f'{bad} broken reference(s)'}")
    return bad


# ---------- session ----------

def provision_workdir(dest: pathlib.Path, fixture: pathlib.Path,
                      old_ref: str | None) -> pathlib.Path:
    """Fixture files + a provisioned plugins/mochiko tree (working tree, or a
    git-archived old ref for the pre arm). D4 as amended (C4)."""
    shutil.copytree(fixture, dest, dirs_exist_ok=True)
    plug = dest / "plugins" / "mochiko"
    if old_ref is None:
        shutil.copytree(PLUGIN, plug)
    else:
        plug.mkdir(parents=True)
        ar = subprocess.run(["git", "-C", str(REPO), "archive", old_ref,
                             "plugins/mochiko"], capture_output=True)
        if ar.returncode != 0:
            die(f"git archive {old_ref} failed: {ar.stderr.decode()[-500:]}")
        subprocess.run(["tar", "-x", "--strip-components", "2", "-C", str(plug)],
                       input=ar.stdout, check=True)
    return plug


def wrapper_text() -> str:
    if not WRAPPER.is_file():
        die(f"{WRAPPER} missing — the pinned wrapper is part of the instrument")
    return WRAPPER.read_text()


def pins(plugin_dir: pathlib.Path) -> dict:
    """C3: versions pinned into every run's meta and the baseline."""
    manifest = plugin_dir / ".claude-plugin" / "plugin.json"
    if not manifest.is_file():
        manifest = plugin_dir / "plugin.json"
    ver = json.loads(manifest.read_text()).get("version")
    cli = subprocess.run(["claude", "--version"], capture_output=True,
                         text=True).stdout.strip()
    return {"plugin_version": ver, "cli": cli, "session_model": SESSION_MODEL,
            "wrapper_sha256": hashlib.sha256(wrapper_text().encode()).hexdigest()[:16]}


def parse_stream(stdout: str) -> dict:
    init = result = None
    tool_calls = []
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
        elif ev.get("type") == "assistant":
            for b in ev.get("message", {}).get("content", []) or []:
                if isinstance(b, dict) and b.get("type") == "tool_use":
                    tool_calls.append(b.get("name"))
        elif ev.get("type") == "result":
            result = ev
    return {"init": init, "result": result, "tool_calls": tool_calls}


def plan_session(cmd: str, golden: dict, arm: str, old_ref: str | None) -> dict:
    """One plan-only session. Returns plan text + meta + deterministic asserts."""
    with tempfile.TemporaryDirectory(prefix="cmdeval-") as td:
        wd = pathlib.Path(td) / "ws"
        wd.mkdir()
        plug = provision_workdir(wd, fixture_dir(cmd, golden), old_ref)
        run_pins = pins(plug)
        if arm == "nocmd":
            prompt = golden["control_prompt"]
        else:
            args_part = golden.get("args", "")
            prompt = f"/mochiko:{cmd} {args_part}".strip()
        argv = ["claude", "-p", prompt,
                "--plugin-dir", str(plug),           # absolute; authoritative (N4)
                "--setting-sources", "",             # isolation, auth kept (probe f.1)
                "--allowedTools", "Read,Grep,Glob",  # D7 permission fence
                "--permission-mode", "dontAsk",
                "--max-turns", str(MAX_TURNS),
                "--model", SESSION_MODEL,
                "--output-format", "stream-json", "--verbose",
                "--append-system-prompt", wrapper_text()]
        proc = subprocess.run(argv, cwd=wd, capture_output=True, text=True, timeout=1800)
        out = parse_stream(proc.stdout)
        init, result = out["init"], out["result"]
        if result is None:
            die(f"no result event (exit {proc.returncode}): {proc.stderr[-800:]}")
        plan = result.get("result") or ""
        # Blocking load gate (I1): the pair under test visible in the init event.
        loaded = [(p.get("name"), p.get("version"))
                  for p in (init or {}).get("plugins", [])]
        load_ok = ("mochiko", run_pins["plugin_version"]) in loaded
        asserts = {
            "load_gate": load_ok,
            "loaded_plugins": loaded,
            "cap_hit": result.get("num_turns") == MAX_TURNS,   # warning, not failure
            "fence_breach": sorted({t for t in out["tool_calls"]
                                    if t not in ("Read", "Grep", "Glob")}),
            "name_resolution": name_resolution(plan) if arm != "nocmd" else [],
            "auth_failure": "Not logged in" in plan,
        }
        if asserts["auth_failure"] or not load_ok:
            die(f"run invalid — load_gate:{load_ok} loaded:{loaded} "
                f"auth_failure:{asserts['auth_failure']} (arm {arm}, {golden['id']})")
        return {"golden": golden["id"], "arm": arm, "plan": plan, "pins": run_pins,
                "asserts": asserts, "cost_usd": result.get("total_cost_usd"),
                "num_turns": result.get("num_turns"),
                "duration_ms": result.get("duration_ms")}


def name_resolution(plan: str) -> list:
    """M1: every mochiko:<skill> / agent the plan names must exist. Blocking-capable.
    Neutrality-safe: checks names the plan chose, never prescribes them."""
    skills = {p.name for p in (PLUGIN / "skills").iterdir() if p.is_dir()}
    agents = {p.stem for p in (PLUGIN / "agents").glob("*.md")}
    bad = []
    for m in set(re.findall(r"mochiko:([a-z][a-z0-9-]+)", plan)):
        if m not in skills and m not in agents and m not in {
                "architecture", "brainstorm", "feature", "implement",
                "setup", "specify", "mochiko"}:
            bad.append(f"mochiko:{m}")
    return sorted(bad)


# ---------- judges (adapted from evals/run.py; advisory) ----------

def judge_session(prompt: str, model: str) -> str:
    with tempfile.TemporaryDirectory(prefix="cmdjudge-") as td:
        proc = subprocess.run(
            ["claude", "-p", prompt, "--model", model, "--max-turns", "1",
             "--setting-sources", "", "--output-format", "json"],
            cwd=td, capture_output=True, text=True, timeout=600)
        try:
            return json.loads(proc.stdout).get("result", "") or ""
        except json.JSONDecodeError:
            return ""


def extract_json(text: str):
    m = re.search(r"\[.*\]|\{.*\}", text, re.DOTALL)
    if not m:
        return None
    try:
        return json.loads(m.group(0))
    except json.JSONDecodeError:
        return None


def scrub_rule_ids(plan: str, rule_ids) -> str:
    """Smoke finding: sessions cite rule IDs despite the wrapper's register line.
    Scrub every known ID before judging so the coverage judge cannot lexically match
    and must grade embodiment (D3). Mechanical and content-neutral."""
    for rid in sorted(rule_ids, key=len, reverse=True):
        plan = plan.replace(rid, "[rule]")
    # extends stubs: sessions may cite the common.<slug> source ID instead
    plan = re.sub(r"\bcommon\.[a-z][a-z0-9.-]*\b", "[rule]", plan)
    return plan


def judge_coverage(rule_items: list, plan: str, model: str = CHECKLIST_MODEL) -> list:
    """One binary per observable rule: does the PLAN's content EMBODY the rule —
    reflected in its actions, absent, or contradicted (D2/D3). Quoted plan-line
    evidence. Chunked + one retry (skill-harness staged-001 finding)."""
    out = []
    for i in range(0, len(rule_items), JUDGE_CHUNK):
        chunk = rule_items[i:i + JUDGE_CHUNK]
        prompt = (
            "You are grading a command's PLANNED-run action plan against rules the "
            "command is bound to. The plan speaks in concrete actions and may never "
            "cite rule IDs — grade EMBODIMENT: does the plan's described behavior "
            "enact the rule? For EACH rule return a JSON array entry "
            "{\"id\": ..., \"verdict\": \"reflected\"|\"absent\"|\"contradicted\", "
            "\"evidence\": \"<verbatim quote from the plan proving the verdict, or "
            "empty for absent>\"}. Every rule id exactly once. Output ONLY the JSON "
            "array.\n\nRULES:\n" + json.dumps(chunk, indent=1)
            + "\n\nPLAN:\n" + plan[:120_000])
        byid = {}
        for _ in range(2):
            verdicts = extract_json(judge_session(prompt, model))
            if isinstance(verdicts, list):
                for v in verdicts:
                    if isinstance(v, dict) and v.get("id") and v.get("verdict"):
                        byid.setdefault(v["id"], v)
            if all(r["id"] in byid for r in chunk):
                break
        out += [byid.get(r["id"], {"id": r["id"], "verdict": None,
                                   "evidence": "MISSING"}) for r in chunk]
    return out


def judge_stub(plan: str, model: str = CHECKLIST_MODEL) -> list:
    """D2 as amended (I6): per numbered phase, substantive vs nominal stub."""
    prompt = (
        "The artifact below is an action plan with numbered phases. For EACH numbered "
        "phase return a JSON array entry {\"phase\": \"<its number/title>\", "
        "\"substantive\": true|false, \"evidence\": \"<verbatim quote>\"} — "
        "substantive means the phase carries concrete scenario-specific content; a "
        "one-line generic stub is false. Output ONLY the JSON array.\n\nPLAN:\n"
        + plan[:120_000])
    verdicts = extract_json(judge_session(prompt, model))
    return verdicts if isinstance(verdicts, list) else []


def judge_pairwise(text_a: str, text_b: str, model: str = PAIRWISE_MODEL) -> dict:
    """Blind A/B with position swap (skill-harness pattern, verbatim mechanics)."""
    def ask(first, second):
        prompt = ("Two action plans answer the same command invocation. Which is the "
                  "better plan overall — more faithful, more concretely actionable? "
                  "Reply ONLY JSON {\"winner\": \"1\"|\"2\"|\"tie\", \"reason\": "
                  "\"<one sentence>\"}.\n\nPLAN 1:\n" + first[:60_000]
                  + "\n\nPLAN 2:\n" + second[:60_000])
        return extract_json(judge_session(prompt, model)) or {}
    v1, v2 = ask(text_a, text_b), ask(text_b, text_a)
    w1, w2 = v1.get("winner"), v2.get("winner")
    agree = (w1 == "1" and w2 == "2") or (w1 == "2" and w2 == "1") or (w1 == w2 == "tie")
    return {"first_order": v1, "swapped": v2, "position_consistent": agree}


# ---------- commands ----------

def rundir(cmd: str, name: str) -> pathlib.Path:
    return CMD_EVALS / cmd / "runs" / name


def cmd_grid(cmd: str, replicates: int, old_ref: str | None, control: bool,
             out: str | None) -> None:
    prereg = CMD_EVALS / cmd / "preregistration.md"
    if not prereg.is_file():
        die(f"{prereg} missing — no grid without a preregistered read rule "
            "(build 2; skill-harness R6 carried over)")
    load_rubric(cmd)                       # fail early on a broken partition
    if check_fixtures(cmd):
        die("fixture consistency check failed")
    goldens = load_goldens(cmd)
    arms = (["pre", "post"] if old_ref else ["post"]) + (["nocmd"] if control else [])
    name = out or datetime.datetime.now().strftime("%Y%m%d-%H%M%S")
    rd = rundir(cmd, name)
    rd.mkdir(parents=True, exist_ok=True)
    runs, total = [], 0.0
    for g in goldens:
        for arm in arms:
            for r in range(1, replicates + 1):
                print(f"run {g['id']}/{arm}/r{r} ...", flush=True)
                e = plan_session(cmd, g, arm, old_ref if arm == "pre" else None)
                e["replicate"] = r
                (rd / f"{g['id']}-{arm}-r{r}.plan.md").write_text(e.pop("plan"))
                runs.append(e)
                total += e.get("cost_usd") or 0.0
                if e["asserts"]["cap_hit"]:
                    print(f"  WARN cap-hit at {MAX_TURNS} turns")
                if e["asserts"]["fence_breach"]:
                    print(f"  WARN fence breach attempted: {e['asserts']['fence_breach']}")
    meta = {"command": cmd, "old_ref": old_ref, "replicates": replicates,
            "arms": arms, "total_cost_usd": round(total, 4),
            "partition": partition(cmd, old_ref) if old_ref else None,
            "runs": runs}
    (rd / "summary.json").write_text(json.dumps(meta, indent=1))
    print(f"grid done: {rd}  (${total:.2f})")


def cmd_judge(cmd: str, name: str, judge_model: str = CHECKLIST_MODEL,
              pairwise_model: str = PAIRWISE_MODEL) -> None:
    rd = rundir(cmd, name)
    meta = json.loads((rd / "summary.json").read_text())
    rub = load_rubric(cmd)
    items = [{"id": i, "text": rub["rules"][i]["text"]}
             for i in sorted(rub["observable"])]
    meta["judge_models"] = {"coverage": judge_model, "stub": judge_model,
                            "pairwise": pairwise_model}
    plans = {}
    for e in meta["runs"]:
        key = (e["golden"], e["arm"], e["replicate"])
        plans[key] = (rd / f"{e['golden']}-{e['arm']}-r{e['replicate']}.plan.md").read_text()
        print(f"judge {key} ...", flush=True)
        scrubbed = scrub_rule_ids(plans[key], rub["rules"].keys())
        e["coverage"] = judge_coverage(items, scrubbed, judge_model)
        e["stub"] = judge_stub(scrubbed, judge_model)
    meta["pairwise"] = []
    if "pre" in meta["arms"]:
        for g in {e["golden"] for e in meta["runs"]}:
            for r in range(1, meta["replicates"] + 1):
                a, b = plans.get((g, "pre", r)), plans.get((g, "post", r))
                if a and b:
                    print(f"pairwise {g}/r{r} ...", flush=True)
                    meta["pairwise"].append(
                        {"golden": g, "replicate": r,
                         **judge_pairwise(a, b, pairwise_model)})
    (rd / "summary.json").write_text(json.dumps(meta, indent=1))
    print(f"judged: {rd / 'summary.json'}")


def passk(entries: list, rule_id: str) -> bool | None:
    """pass^k: reflected in ALL replicates. None = never judged."""
    vs = [v["verdict"] for e in entries for v in e.get("coverage", [])
          if v["id"] == rule_id]
    if not vs:
        return None
    return all(v == "reflected" for v in vs)


def flaky(entries: list, rule_id: str) -> bool:
    vs = {v["verdict"] for e in entries for v in e.get("coverage", [])
          if v["id"] == rule_id}
    return len(vs) > 1


def cmd_report(cmd: str, name: str) -> None:
    rd = rundir(cmd, name)
    meta = json.loads((rd / "summary.json").read_text())
    rub = load_rubric(cmd)
    obs = sorted(rub["observable"])
    part = meta.get("partition") or {"unchanged": obs, "changed": [], "removed": [],
                                     "added": []}
    lines = [f"# Plan-only eval report — {cmd} / {name}", "",
             f"Arms: {meta['arms']} · replicates {meta['replicates']} · "
             f"cost ${meta['total_cost_usd']}", "",
             "Advisory (harness D2): nothing below sets an exit code.", ""]
    goldens = sorted({e["golden"] for e in meta["runs"]})
    for g in goldens:
        lines.append(f"## {g}")
        by_arm = {a: [e for e in meta["runs"] if e["golden"] == g and e["arm"] == a]
                  for a in meta["arms"]}
        pre, post = by_arm.get("pre", []), by_arm.get("post", [])
        # D6 buckets over the observable subset; pre arm = comparison substrate (V2).
        regressions, adoptions, ghosts, flaky_ids = [], [], [], []
        for rid in obs:
            if flaky(post, rid) or (pre and flaky(pre, rid)):
                flaky_ids.append(rid)
            if rid in part["unchanged"] and pre:
                if passk(pre, rid) and not passk(post, rid):
                    regressions.append(rid)
            if rid in part["added"]:
                adoptions.append((rid, passk(post, rid)))
            if rid in part["removed"] and passk(post, rid):
                ghosts.append(rid)
        covered = sum(1 for rid in obs if passk(post, rid))
        lines.append(f"- post coverage (pass^k): {covered}/{len(obs)}")
        if pre:
            pcov = sum(1 for rid in obs if passk(pre, rid))
            lines.append(f"- pre coverage (pass^k): {pcov}/{len(obs)}")
            lines.append(f"- **unchanged-bucket regressions:** "
                         f"{regressions or 'none'}")
        if part["added"]:
            lines.append(f"- added-rule adoption: "
                         + ", ".join(f"{r}={'LANDED' if ok else 'DEAD-TEXT'}"
                                     for r, ok in adoptions))
        if ghosts:
            lines.append(f"- **removed rules still surfacing:** {ghosts}")
        if part["changed"]:
            lines.append(f"- changed-text (graded vs NEW text, advisory): "
                         f"{[r for r in part['changed'] if r in obs]}")
        # F2 noise guard, verbatim discipline: replicate disagreement flags noise.
        lines.append(f"- flaky rules (replicate disagreement — noise-guard input): "
                     f"{len(flaky_ids)}" + (f" {flaky_ids}" if flaky_ids else ""))
        for e in post:
            stubs = [s for s in e.get("stub", []) if s.get("substantive") is False]
            if stubs:
                lines.append(f"- stub phases (r{e['replicate']}): "
                             f"{[s.get('phase') for s in stubs]}")
        caps = [f"{e['arm']}/r{e['replicate']}" for e in pre + post
                if e["asserts"]["cap_hit"]]
        if caps:
            lines.append(f"- WARN cap-hit runs: {caps}")
        nres = sorted({n for e in pre + post for n in e["asserts"]["name_resolution"]})
        if nres:
            lines.append(f"- **unresolvable names in plans:** {nres}")
        lines.append("")
    for p in meta.get("pairwise", []):
        w = p["first_order"].get("winner"), p["swapped"].get("winner")
        lines.append(f"- pairwise {p['golden']}/r{p['replicate']}: {w} "
                     f"(position_consistent={p['position_consistent']})")
    (rd / "report.md").write_text("\n".join(lines) + "\n")
    print(f"report: {rd / 'report.md'}")


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__)
    sub = ap.add_subparsers(dest="cmd", required=True)
    for name in ("partition", "check-rubric", "check-fixtures", "plan-run", "grid",
                 "judge", "report"):
        p = sub.add_parser(name)
        p.add_argument("command")
        if name == "partition":
            p.add_argument("--old-ref", required=True)
        if name == "plan-run":
            p.add_argument("golden")
            p.add_argument("--arm", default="post", choices=ARMS)
            p.add_argument("--old-ref")
            p.add_argument("--out")
        if name == "grid":
            p.add_argument("--replicates", type=int, default=3)
            p.add_argument("--old-ref")
            p.add_argument("--control", action="store_true")
            p.add_argument("--out")
        if name in ("judge", "report"):
            p.add_argument("run_name")
        if name == "judge":
            p.add_argument("--judge-model", default=CHECKLIST_MODEL,
                           help="coverage+stub judge (ruled default: haiku)")
            p.add_argument("--pairwise-model", default=PAIRWISE_MODEL,
                           help="pairwise judge (ruled default: sonnet)")
    a = ap.parse_args()
    if a.cmd == "partition":
        print(json.dumps(partition(a.command, a.old_ref), indent=1))
    elif a.cmd == "check-rubric":
        rub = load_rubric(a.command)
        print(f"rubric OK: {len(rub['observable'])} observable, "
              f"{len(rub['out_of_instrument'])} out-of-instrument, "
              f"{len(rub['rules'])} total")
    elif a.cmd == "check-fixtures":
        sys.exit(1 if check_fixtures(a.command) else 0)
    elif a.cmd == "plan-run":
        goldens = {g["id"]: g for g in load_goldens(a.command)}
        if a.golden not in goldens:
            die(f"unknown golden {a.golden}; have {sorted(goldens)}")
        if a.arm == "pre" and not a.old_ref:
            die("--arm pre needs --old-ref")
        e = plan_session(a.command, goldens[a.golden], a.arm,
                         a.old_ref if a.arm == "pre" else None)
        out = pathlib.Path(a.out) if a.out else rundir(a.command, "adhoc")
        out.mkdir(parents=True, exist_ok=True)
        (out / f"{a.golden}-{a.arm}.plan.md").write_text(e.pop("plan"))
        (out / f"{a.golden}-{a.arm}.meta.json").write_text(json.dumps(e, indent=1))
        print(json.dumps(e["asserts"], indent=1))
        print(f"saved: {out}  (${e.get('cost_usd')})")
    elif a.cmd == "grid":
        cmd_grid(a.command, a.replicates, a.old_ref, a.control, a.out)
    elif a.cmd == "judge":
        cmd_judge(a.command, a.run_name, a.judge_model, a.pairwise_model)
    elif a.cmd == "report":
        cmd_report(a.command, a.run_name)


if __name__ == "__main__":
    main()
