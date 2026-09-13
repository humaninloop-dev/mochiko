#!/usr/bin/env python3
"""Skill eval runner — the skill target of the one eval layer.

Provenance: .mochiko/brainstorms/skill-compression-tooling/record.md (D8 as amended) and
.mochiko/brainstorms/primitive-eval-harness-v2/record.md (D1 vocabulary, D12/C3 the re-keyed
inventories). Maintainer-side advisory tooling (GI-019 trace recorded); never shipped (GI-020).
Vocabulary shared with the command and persona targets: evals/README.md.

One run = one isolated `claude -p` session on the host's subscription auth, loading the
provisioned plugin tree (`plugins/mochiko` at the working tree for `post`, a git-archived
`--old-ref` for `pre`, no plugin at all for the bare-model `noskill` control), executing one
golden prompt in a throwaway workspace seeded from the golden's fixture. The plugin is
provisioned OUTSIDE the workspace (persona-runner finding 2026-09-08). Rules ride the plugin's
migration log rendered by `mochiko-cli` at fire (v0.107.0 end state), so the whole plugin is the
unit under test, never a synthesized single-skill copy — the pre-v0.107.0 `variants/` staging is
retired with the converged arms (2026-09-11).

Grading: scripted assertions (deterministic, may block) + rule-coverage checklist via a Haiku
judge (binary + quoted evidence, pass^k over valid runs) — advisory (harness D2). A run whose
load gate fails (plugin not loaded, skill never fired, rules not delivered, wrong model, auth
failure) is recorded `invalid` and excluded from every read; the report counts it.

Modes: host (default; `claude -p` on the stored subscription, `--setting-sources ""` so no
user-level plugin loads beside the provisioned one) · --local (`--bare` + ANTHROPIC_API_KEY,
metered). The pre-convergence Docker-sandbox mode is retired: a host tempfile plugin path is
invisible inside the sandbox, so every sandbox run would spend and then fail the load gate
(runner review 2026-09-11).
"""

import argparse
import datetime
import hashlib
import json
import os
import pathlib
import re
import shutil
import subprocess
import sys
import tempfile
import uuid

REPO = pathlib.Path(__file__).resolve().parent.parent
EVALS = REPO / "evals"
WORK = EVALS / ".work"
PLUGIN = REPO / "plugins" / "mochiko"
PLUGIN_SKILLS = PLUGIN / "skills"

SESSION_MODEL = "sonnet"    # model under test (R7)
CHECKLIST_MODEL = "haiku"   # checklist judge (D8)
PAIRWISE_MODEL = "sonnet"   # pairwise judge (D8; opt-in — position-biased in both pilots)
MAX_TURNS = 40
PERMISSION_MODE = "acceptEdits"
# python is pre-allowed for the skills that ship a checker script (review-plan-artifacts
# runs scripts/check-artifacts.py as its tier-1 pre-assert; the real seat runs it too).
ALLOWED_TOOLS = "Skill,Bash(mochiko-cli:*),Bash(mochiko-cli *),Bash(python3:*),Bash(python3 *),Bash(python:*),Bash(python *)"
ARMS = ["noskill", "pre", "post"]
LEGACY_ARMS = ["baseline", "armA", "armB"]   # readable in pre-convergence runs, never run

MODE = "host"  # host | local


def die(msg: str) -> None:
    print(f"error: {msg}", file=sys.stderr)
    sys.exit(1)


def sha(text: str) -> str:
    return hashlib.sha256(text.encode()).hexdigest()[:16]


# ---------- provisioning ----------

def provision_plugin(dest: pathlib.Path, old_ref: str | None) -> pathlib.Path:
    """plugins/mochiko at the working tree or a git-archived old ref, at `dest/mochiko`."""
    plug = dest / "mochiko"
    if old_ref is None:
        shutil.copytree(PLUGIN, plug)
    else:
        plug.mkdir(parents=True)
        ar = subprocess.run(["git", "-C", str(REPO), "archive", old_ref, "plugins/mochiko"],
                            capture_output=True)
        if ar.returncode != 0:
            die(f"git archive {old_ref} failed: {ar.stderr.decode()[-500:]}")
        subprocess.run(["tar", "-x", "--strip-components", "2", "-C", str(plug)],
                       input=ar.stdout, check=True)
    return plug


def fixture_dir(skill: str, golden: dict) -> pathlib.Path | None:
    name = golden.get("fixture")
    if not name:
        return None
    d = EVALS / skill / "fixtures" / name
    if not d.is_dir():
        die(f"golden {golden['id']}: fixture {d} missing")
    return d


def rendered_rules(skill: str, plug: pathlib.Path) -> tuple:
    """What `mochiko-cli` renders for the skill's `!` lines at fire, with the provisioned
    tree as the plugin root — the delivered rule text (empty for a pre-log body). Returns
    (text, ok): a render whose line fails (bad root, grammar out of range — the `2>&1` on
    each line would otherwise hash the error as rules) is never pinned as delivered."""
    body = plug / "skills" / skill / "SKILL.md"
    if not body.is_file():
        return "", True
    out, ok = [], True
    env = {**os.environ, "CLAUDE_PLUGIN_ROOT": str(plug)}
    for m in re.finditer(r"^!`(.+)`\s*$", body.read_text(), re.M):
        cmd = m.group(1).replace("${CLAUDE_PLUGIN_ROOT}", str(plug))
        if not cmd.startswith("mochiko-cli "):
            continue  # only the rule-delivery lines are the instrument's concern
        r = subprocess.run(["sh", "-c", cmd], capture_output=True, text=True, env=env)
        ok = ok and r.returncode == 0
        out.append(r.stdout)
    return "\n".join(out), ok


def pins(skill: str, plug: pathlib.Path | None, old_ref: str | None) -> dict:
    cli = subprocess.run(["claude", "--version"], capture_output=True, text=True).stdout.strip()
    mcli = subprocess.run(["mochiko-cli", "--version"], capture_output=True,
                          text=True).stdout.strip()
    out = {"skill": skill, "cli": cli, "mochiko_cli": mcli, "session_model": SESSION_MODEL,
           "permission_mode": PERMISSION_MODE, "judge_prompt_sha256": JUDGE_PROMPT_SHA,
           "old_ref": old_ref}
    if plug is not None:
        manifest = plug / ".claude-plugin" / "plugin.json"
        out["plugin_version"] = json.loads(manifest.read_text()).get("version")
        body = plug / "skills" / skill / "SKILL.md"
        out["skill_sha256"] = sha(body.read_text()) if body.is_file() else None
        rendered, ok = rendered_rules(skill, plug)
        out["rendered_rules_ok"] = ok
        out["rendered_rules_sha256"] = sha(rendered) if (ok and rendered.strip()) else None
        out["rendered_rules_chars"] = len(rendered) if ok else 0
    return out


# ---------- session ----------

def claude_args(prompt: str, model: str, max_turns: int, plugin: pathlib.Path | None,
                stream: bool = True) -> list:
    # `acceptEdits` is the probe-settled permission mode (R5; `dontAsk` denied writes).
    args = ["claude", "-p", prompt, "--model", model,
            "--permission-mode", PERMISSION_MODE, "--max-turns", str(max_turns),
            # Headless cannot answer a permission prompt: the Skill tool (probe 2026-09-11:
            # `permission_denied` under acceptEdits) and the rule-delivery binary are
            # pre-allowed; everything else stays on acceptEdits' own rules.
            "--allowedTools", ALLOWED_TOOLS,
            # No user/project config: the host carries a user-level mochiko install that
            # would otherwise load beside the provisioned tree. Stored auth survives this.
            "--setting-sources", "",
            "--output-format", "stream-json" if stream else "json"]
    if stream:
        args += ["--verbose"]
    if MODE == "local":
        args.insert(1, "--bare")  # hermetic, needs ANTHROPIC_API_KEY
    if plugin is not None:
        args += ["--plugin-dir", str(plugin)]
    return args


def parse_stream(stdout: str) -> dict:
    init = result = None
    tool_calls, skills_fired, models = [], [], set()
    for line in stdout.splitlines():
        line = line.strip()
        if not line:
            continue
        try:
            ev = json.loads(line)
        except json.JSONDecodeError:
            continue
        if ev.get("parent_tool_use_id"):
            continue  # a subagent's events (an Explore read) never speak for the session
        if ev.get("type") == "system" and ev.get("subtype") == "init":
            init = ev
        elif ev.get("type") == "assistant":
            msg = ev.get("message") or {}
            if isinstance(msg, dict) and msg.get("model"):
                models.add(msg["model"])
            for b in (msg.get("content") or []):
                if isinstance(b, dict) and b.get("type") == "tool_use":
                    tool_calls.append(b.get("name"))
                    if b.get("name") == "Skill":
                        skills_fired.append(str((b.get("input") or {}).get("skill", "")))
        elif ev.get("type") == "result":
            result = ev
    return {"init": init, "result": result, "tool_calls": tool_calls,
            "skills_fired": skills_fired, "models": sorted(models)}


def run_session(prompt: str, *, model: str, plugin: pathlib.Path | None,
                fixture: pathlib.Path | None = None, max_turns: int = MAX_TURNS,
                want_workspace: bool = False) -> dict:
    """One claude -p session. Returns the parsed stream plus `workspace` (kept copy or None)."""
    with tempfile.TemporaryDirectory(prefix="skilleval-") as td:
        ws = pathlib.Path(td) / "ws"
        ws.mkdir()
        if fixture is not None:
            shutil.copytree(fixture, ws, dirs_exist_ok=True)
        proc = subprocess.run(claude_args(prompt, model, max_turns, plugin),
                              cwd=ws, capture_output=True, text=True, timeout=3600)
        if proc.returncode != 0 and not proc.stdout.strip():
            die(f"claude spawn failed (exit {proc.returncode}): {proc.stderr[-2000:]}")
        out = parse_stream(proc.stdout)
        out["stream"] = proc.stdout          # kept per run for forensics
        out["stderr_tail"] = proc.stderr[-800:]
        if want_workspace:
            keep = WORK / f"ws-{uuid.uuid4().hex[:12]}"
            shutil.copytree(ws, keep)
            out["workspace"] = keep
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


def changed_fixture_files(workspace: pathlib.Path, fixture: pathlib.Path | None) -> list:
    """Handed-in fixture files the session edited or deleted (relative paths)."""
    if fixture is None:
        return []
    out = []
    for f in sorted(fixture.rglob("*")):
        if not f.is_file():
            continue
        rel = f.relative_to(fixture)
        w = workspace / rel
        if not w.is_file() or w.read_bytes() != f.read_bytes():
            out.append(str(rel))
    return out


def run_assertions(assertions: list, workspace: pathlib.Path,
                   fixture: pathlib.Path | None = None) -> list:
    out = []
    for a in assertions:
        kind = a["type"]
        ok, detail = False, ""
        target = workspace / a.get("path", "")
        if kind == "fixture_unchanged":
            # Every handed-in file is byte-identical after the run, except the listed paths
            # (`except`: files a golden expects the session to touch).
            changed = [c for c in changed_fixture_files(workspace, fixture)
                       if c not in set(a.get("except", []))]
            ok, detail = not changed, (", ".join(changed) if changed else "")
        elif kind == "file_exists":
            ok = target.is_file()
        elif kind == "file_absent":
            ok = not target.exists()
        elif kind == "contains":
            ok = target.is_file() and re.search(a["pattern"], target.read_text()) is not None
        elif kind == "not_contains":
            ok = target.is_file() and re.search(a["pattern"], target.read_text()) is None
        else:
            detail = f"unknown assertion type {kind}"
        out.append({**a, "passed": ok, "detail": detail})
    return out


# ---------- judges ----------

JUDGE_CHUNK = 15  # rules per judge call — an 82-entry single-shot array was ~9% unparseable


def judge_prompt(chunk: list, artifact_text: str, expected_output: str = "") -> str:
    return (
        "You are grading an artifact against a rule checklist. For EACH of the "
        f"{len(chunk)} rules return a JSON array entry "
        "{\"id\": ..., \"passed\": true|false, \"evidence\": \"<verbatim quote "
        "from the artifact proving the verdict, or empty>\"}. Binary only, no scores. "
        "A rule passes only when the artifact itself shows the rule being followed — a "
        "restatement of the rule as a principle is not evidence. Every rule id must appear "
        "exactly once. Output ONLY the JSON array.\n\nRULES:\n"
        + json.dumps(chunk, indent=1)
        + ("\n\nEXPECTED OUTPUT (golden's intent, context only):\n" + expected_output
           if expected_output else "")
        + "\n\nARTIFACT:\n" + artifact_text[:120_000])


JUDGE_PROMPT_SHA = sha(judge_prompt([], ""))


def judge_checklist(rules: list, artifact_text: str, expected_output: str = "") -> list:
    """One binary per rule, quoted evidence. Advisory. Chunked + one retry per chunk."""
    out = []
    for i in range(0, len(rules), JUDGE_CHUNK):
        chunk = [{k: r[k] for k in ("id", "rule", "class") if k in r} for r in rules[i:i + JUDGE_CHUNK]]
        prompt = judge_prompt(chunk, artifact_text, expected_output)
        byid = {}
        for _ in range(3):  # two retries on parse failure / missing ids (VC baseline: one
            # chunk failed twice and its MISSING read as seven floors lost)
            res = run_session(prompt, model=CHECKLIST_MODEL, plugin=None, max_turns=1)
            verdicts = extract_json((res["result"] or {}).get("result", "") or "")
            if isinstance(verdicts, list):
                for v in verdicts:
                    if isinstance(v, dict) and v.get("id") and v.get("passed") is not None:
                        byid.setdefault(v["id"], v)
            if all(r["id"] in byid for r in chunk):
                break
        out += [byid.get(r["id"], {"id": r["id"], "passed": None, "evidence": "MISSING"})
                for r in chunk]
    return out


def judge_pairwise(golden_id: str, text_a: str, text_b: str) -> dict:
    """Blind A/B with position swap. Advisory, opt-in."""
    def ask(first, second):
        prompt = (
            "Two artifacts answer the same task. Which is better overall? Reply ONLY JSON "
            "{\"winner\": \"1\"|\"2\"|\"tie\", \"reason\": \"<one sentence>\"}.\n\n"
            "ARTIFACT 1:\n" + first[:60_000] + "\n\nARTIFACT 2:\n" + second[:60_000]
        )
        res = run_session(prompt, model=PAIRWISE_MODEL, plugin=None, max_turns=1)
        return extract_json((res["result"] or {}).get("result", "") or "") or {}
    v1 = ask(text_a, text_b)
    v2 = ask(text_b, text_a)  # position swap
    w1, w2 = v1.get("winner"), v2.get("winner")
    agree = (w1 == "1" and w2 == "2") or (w1 == "2" and w2 == "1") or (w1 == w2 == "tie")
    return {"golden": golden_id, "first_order": v1, "swapped": v2, "position_consistent": agree}


def collect_artifact(workspace: pathlib.Path, fixture: pathlib.Path | None) -> str:
    """Every file the session produced or changed — fixture files that came back byte-identical
    are not the artifact."""
    parts = []
    for f in sorted(workspace.rglob("*")):
        if not f.is_file() or f.stat().st_size >= 200_000:
            continue
        rel = f.relative_to(workspace)
        if fixture is not None:
            src = fixture / rel
            if src.is_file() and src.read_bytes() == f.read_bytes():
                continue
        parts.append(f"=== {rel} ===\n{f.read_text(errors='replace')}")
    return "\n\n".join(parts) or "(no files produced)"


# ---------- one graded run ----------

def skill_session(skill: str, golden: dict, arm: str, old_ref: str | None,
                  prompt: str | None = None) -> dict:
    """One session for a golden on an arm; the plugin tree provisioned beside the workspace."""
    if arm not in ARMS:
        die(f"unknown arm {arm} (runnable arms: {ARMS})")
    if arm == "pre" and not old_ref:
        die("the pre arm needs --old-ref <ref>")
    fixture = fixture_dir(skill, golden)
    with tempfile.TemporaryDirectory(prefix="skilleval-plug-") as td:
        plug = None
        if arm != "noskill":
            plug = provision_plugin(pathlib.Path(td), old_ref if arm == "pre" else None)
            if not (plug / "skills" / skill / "SKILL.md").is_file():
                die(f"arm {arm}: {skill} has no SKILL.md in the provisioned tree")
        run_pins = pins(skill, plug, old_ref if arm == "pre" else None)
        out = run_session(prompt or golden["prompt"], model=SESSION_MODEL, plugin=plug,
                          fixture=fixture, want_workspace=True)
    init, result, ws = out["init"], out["result"], out.get("workspace")
    text = (result or {}).get("result") or ""
    loaded = [(p.get("name"), p.get("version")) for p in (init or {}).get("plugins", [])]
    artifact = collect_artifact(ws, fixture) if ws else "(no workspace)"
    fired = any(skill in s for s in out["skills_fired"])
    not_delivered = "rules not delivered" in (text + artifact)
    mochiko_loaded = any(n == "mochiko" for n, _ in loaded)
    asserts = {
        # The control must run bare: no mochiko plugin loaded. The bare model may still
        # attempt the Skill tool on the same prompt (the call errors — no such skill); that
        # attempt is recorded in `skills_fired` and is not a gate failure (grid 2026-09-13:
        # every control session tried it).
        "load_gate": ((not mochiko_loaded) if arm == "noskill"
                      else ("mochiko", run_pins.get("plugin_version")) in loaded),
        "loaded_plugins": loaded,
        "skill_fired": fired if arm != "noskill" else None,
        "rules_delivered": (not not_delivered) if arm != "noskill" else None,
        "model_ok": bool(out["models"]) and all(m.startswith(f"claude-{SESSION_MODEL}")
                                                  for m in out["models"]),
        "models": out["models"],
        "cap_hit": (result or {}).get("num_turns") == MAX_TURNS,
        "auth_failure": "Not logged in" in text,
        "no_result": result is None,
        "is_error": bool((result or {}).get("is_error")),
    }
    if asserts["auth_failure"]:
        die("claude -p reports 'Not logged in' — run /login, then re-run")
    valid = (asserts["load_gate"] and asserts["model_ok"] and not asserts["no_result"]
             and not asserts["is_error"]
             and (arm == "noskill" or (fired and asserts["rules_delivered"]
                                       and run_pins.get("rendered_rules_ok", True))))
    return {"arm": arm, "golden": golden["id"], "pins": run_pins, "asserts": asserts,
            "valid": valid, "tool_calls": out["tool_calls"], "skills_fired": out["skills_fired"],
            "result_text": text, "artifact": artifact, "workspace": ws,
            "stream": out.get("stream", ""),
            "cost_usd": (result or {}).get("total_cost_usd"),
            "num_turns": (result or {}).get("num_turns")}


# ---------- commands ----------

def load_kit(skill: str) -> tuple:
    base = EVALS / skill
    goldens = json.loads((base / "evals.json").read_text())
    rules = json.loads((base / "rules.json").read_text())
    if isinstance(rules, dict):
        rules = rules["rules"]
    ids = {r["id"] for r in rules}
    for g in goldens:
        bad = [t for t in g.get("tempts", []) if t not in ids]
        if bad:
            die(f"golden {g['id']} tempts unknown rule ids: {bad}")
    return base, goldens, rules


def cmd_probe(skill: str, old_ref: str | None, arm: str) -> None:
    """R5: settle invocation mechanics empirically before any priced grid — does the
    provisioned plugin load, does the skill fire on an explicit ask, do its rules render
    (the read-back names a floor count), does a write land under acceptEdits."""
    if not (PLUGIN_SKILLS / skill).is_dir():
        die(f"no such skill: {PLUGIN_SKILLS / skill}")
    golden = {"id": "probe", "prompt": (
        f"Invoke the mochiko:{skill} skill now. Its rules block, once delivered, instructs "
        f"you to state back a floor count and the floor ids before the first procedural "
        f"step. Do exactly that read-back, write it verbatim into `probe.txt` in the current "
        f"directory, and then stop without performing the skill's procedure. If the skill "
        f"halts because its rules were not delivered, write exactly what it surfaced into "
        f"`probe.txt` instead.")}
    r = skill_session(skill, golden, arm, old_ref)
    ws = r["workspace"]
    (WORK / f"probe-{skill}-{arm}.stream.jsonl").write_text(r["stream"])
    print(f"probe findings ({MODE}, arm {arm}):")
    print(f"  stream kept:          {WORK / f'probe-{skill}-{arm}.stream.jsonl'}")
    print(f"  plugin loaded:        {r['asserts']['loaded_plugins']}")
    print(f"  skill fired:          {r['skills_fired']}")
    print(f"  rules delivered:      {r['asserts']['rules_delivered']}")
    print(f"  models:               {r['asserts']['models']}")
    print(f"  write landed:         {ws is not None and (ws / 'probe.txt').is_file()}")
    if ws and (ws / "probe.txt").is_file():
        print("  probe.txt:            " + (ws / "probe.txt").read_text()[:600].replace("\n", "\n                        "))
    print(f"  turns / cost:         {r['num_turns']} / {r['cost_usd']}")
    print(f"  valid:                {r['valid']}")
    print(f"  pins:                 {json.dumps(r['pins'])}")
    if ws:
        shutil.rmtree(ws, ignore_errors=True)


def cmd_grid(skill: str, replicates: int, arms: list, old_ref: str | None,
             out: str | None = None, pairwise: bool = False, budget_usd: float = 25.0) -> None:
    base, goldens, rules = load_kit(skill)
    prereg = base / "preregistration.md"
    if not prereg.is_file():
        die(f"{prereg} missing — the ship bar must be pre-registered before any grid (R6/R9)")
    if len(prereg.read_text().strip()) < 200:
        die(f"{prereg} looks empty — record the ship bar and delivered-chars arithmetic first")
    for arm in arms:
        if arm not in ARMS:
            die(f"arm {arm} is not runnable (runnable: {ARMS}; legacy {LEGACY_ARMS} read only)")
    if "pre" in arms and not old_ref:
        die("the pre arm needs --old-ref <ref>")

    stamp = out or datetime.datetime.now().strftime("%Y%m%d-%H%M%S")
    rundir = base / "runs" / stamp
    rundir.mkdir(parents=True, exist_ok=True)
    results, total_cost, stored_pw = [], 0.0, []
    prior = rundir / "summary.json"
    if prior.is_file():  # resume/append: every stored session is kept; a session already
        # run for (arm, golden, replicate) is skipped. To redo an arm, name a new --out.
        old = json.loads(prior.read_text())
        results = list(old.get("runs", []))
        stored_pw = old.get("pairwise", [])
        old_ref = old_ref or old.get("old_ref")
        total_cost = sum(e.get("cost_usd") or 0.0 for e in results)
        print(f"resuming {stamp}: {len(results)} stored sessions "
              f"({sorted({e['arm'] for e in results})})")

    # Case-major order: for each golden, run every arm before moving on.
    for g in goldens:
        for arm in arms:
            for rep in range(1, replicates + 1):
                label = f"{arm}/{g['id']}/r{rep}"
                art_file = rundir / f"{arm}-{g['id']}-r{rep}.artifact.txt"
                if any(e["arm"] == arm and e["golden"] == g["id"] and e["replicate"] == rep
                       for e in results):
                    print(f"run {label} ... (resumed"
                          f"{'' if art_file.is_file() else ', artifact missing'})", flush=True)
                    continue
                print(f"run {label} ...", flush=True)
                r = skill_session(skill, g, arm, old_ref)
                ws = r.pop("workspace")
                artifact = r.pop("artifact")
                cost = r.get("cost_usd") or 0.0
                total_cost += cost
                entry = {
                    "arm": arm, "golden": g["id"], "replicate": rep,
                    "pins": r["pins"], "asserts": r["asserts"], "valid": r["valid"],
                    "tool_calls": r["tool_calls"], "skills_fired": r["skills_fired"],
                    "assertions": (run_assertions(g.get("assertions", []), ws, fixture_dir(skill, g))
                                   if ws else []),
                    "checklist": (judge_checklist(rules, artifact, g.get("expected_output", ""))
                                  if r["valid"] else []),
                    "cost_usd": cost, "num_turns": r["num_turns"],
                    "artifact_truncated": len(artifact) > 120_000,
                }
                if not r["valid"]:
                    print(f"  INVALID {label}: {json.dumps(r['asserts'])}", flush=True)
                art_file.write_text(artifact)
                (rundir / f"{arm}-{g['id']}-r{rep}.result.md").write_text(r["result_text"])
                (rundir / f"{arm}-{g['id']}-r{rep}.stream.jsonl").write_text(r["stream"])
                results.append(entry)
                # Persist after every session so a killed grid resumes.
                _write_summary(skill, rundir, stamp, replicates, rules, goldens, results,
                               total_cost, old_ref, stored_pw)
                if ws:
                    shutil.rmtree(ws, ignore_errors=True)
                if total_cost > budget_usd:
                    die(f"budget {budget_usd} USD exceeded (sessions {total_cost:.2f}); the grid "
                        f"halts here and resumes with the same --out once the user rules")

    pw = list(stored_pw)
    if pairwise and "pre" in {e["arm"] for e in results}:
        art = lambda arm, gid: (rundir / f"{arm}-{gid}-r1.artifact.txt")
        done = {x.get("golden") for x in pw}
        for g in goldens:
            fa, fb = art("pre", g["id"]), art("post", g["id"])
            if g["id"] not in done and fa.is_file() and fb.is_file():
                pw.append({"arm": "post", **judge_pairwise(g["id"], fa.read_text(), fb.read_text())})
    summary = _write_summary(skill, rundir, stamp, replicates, rules, goldens, results,
                             total_cost, old_ref, pw)
    print(f"\nreport: {rundir / 'report.md'}  (est. spend ${total_cost:.2f} — client-side estimate)")
    # Advisory posture: judged degradation never fails the process. Mechanical `post`
    # assertion failures and invalid runs do.
    if summary["post_assertion_failures"] or summary["invalid_runs"]:
        sys.exit(2)


def _write_summary(skill, rundir, stamp, replicates, rules, goldens, results, total_cost,
                   old_ref, pairwise) -> dict:
    arms = [a for a in ARMS + LEGACY_ARMS if any(e["arm"] == a for e in results)]
    summary = summarize(skill, arms, replicates, rules, goldens, results, total_cost, old_ref)
    summary["pairwise"] = pairwise
    (rundir / "summary.json").write_text(json.dumps({"runs": results, **summary}, indent=1))
    (rundir / "report.md").write_text(render_report(skill, stamp, summary))
    return summary


def cmd_rejudge(skill: str, out: str, only_missing: bool = False) -> None:
    """Re-score stored artifacts with the current judge — no sessions, judges only.
    --only-missing re-scores just the sessions carrying a MISSING verdict."""
    base, goldens, rules = load_kit(skill)
    rundir = base / "runs" / out
    summ = rundir / "summary.json"
    if not summ.is_file():
        die(f"{summ} missing")
    data = json.loads(summ.read_text())
    gmap = {g["id"]: g for g in goldens}
    results = data["runs"]
    for e in results:
        art_file = rundir / f"{e['arm']}-{e['golden']}-r{e['replicate']}.artifact.txt"
        if not art_file.is_file() or not e.get("valid", True):
            print(f"skip {art_file.name}: {'artifact missing' if not art_file.is_file() else 'invalid run'}")
            continue
        if only_missing and not any(v.get("passed") is None for v in e.get("checklist", [])):
            continue
        print(f"rejudge {e['arm']}/{e['golden']}/r{e['replicate']} ...", flush=True)
        e["checklist"] = judge_checklist(rules, art_file.read_text(),
                                         gmap.get(e["golden"], {}).get("expected_output", ""))
        e.setdefault("pins", {})["judge_prompt_sha256"] = JUDGE_PROMPT_SHA
    total_cost = sum(e.get("cost_usd") or 0.0 for e in results)
    _write_summary(skill, rundir, out, data["replicates"], rules, goldens, results, total_cost,
                   data.get("old_ref"), data.get("pairwise", []))
    print(f"rejudged: {rundir / 'report.md'}")


# ---------- summary + report ----------

def summarize(skill, arms, replicates, rules, goldens, results, total_cost, old_ref) -> dict:
    valid = [e for e in results if e.get("valid", True)]
    invalid = [f"{e['arm']}/{e['golden']}/r{e['replicate']}" for e in results
               if not e.get("valid", True)]
    tempts = {g["id"]: set(g.get("tempts", [])) for g in goldens}
    # Invited read (mirrors the persona target, ADR 2026-09-09): when the goldens declare
    # `tempts`, a rule is held only over the goldens that invite it — a golden-specific rule
    # read `false` on a golden that never asked for it is not a loss. A rule no golden invites
    # is `untempted`: disclosed, never read. Without tempts the read is cross-golden.
    any_tempts = any(tempts.values())
    tempted_by = {r["id"]: [g for g, s in tempts.items() if r["id"] in s] for r in rules}
    untempted = [r["id"] for r in rules if any_tempts and not tempted_by[r["id"]]]
    held, flaky, invited_flaky = {}, {}, {}
    for arm in arms:
        for r in rules:
            verdicts = [v.get("passed") for e in valid if e["arm"] == arm
                        and (not any_tempts or e["golden"] in tempted_by[r["id"]])
                        for v in e["checklist"] if v.get("id") == r["id"]]
            held[(arm, r["id"])] = bool(verdicts) and all(v is True for v in verdicts)
    pruned = [r["id"] for r in rules if held.get(("noskill", r["id"]))]  # R3
    live = [r for r in rules if r["id"] not in pruned and r["id"] not in untempted]
    # The band's pairs are the graded set's: live rules only (pruned and untempted rules
    # leave the denominator, as the persona target's model-native claims do).
    for arm in arms:
        n_f = n_if = n_i = n_p = 0
        for r in live:
            for g in goldens:
                vs = [v.get("passed") for e in valid if e["arm"] == arm and e["golden"] == g["id"]
                      for v in e["checklist"] if v.get("id") == r["id"]]
                vs = [v for v in vs if v is not None]
                if len(vs) < 2:
                    continue
                n_p += 1
                disagree = len(set(vs)) > 1
                n_f += disagree
                if r["id"] in tempts[g["id"]]:
                    n_i += 1
                    n_if += disagree
        flaky[arm] = {"flaky": n_f, "pairs": n_p}
        invited_flaky[arm] = {"flaky": n_if, "pairs": n_i}
    ref = "pre" if "pre" in arms else ("baseline" if "baseline" in arms else None)
    lost = {arm: [r["id"] for r in live if held.get((ref, r["id"])) and not held.get((arm, r["id"]))]
            for arm in arms if ref and arm not in ("noskill", ref)}
    floor_ids = {r["id"] for r in rules if r.get("class") == "floor"}
    floor_lost = {arm: [i for i in ids if i in floor_ids] for arm, ids in lost.items()}
    coverage = {arm: sum(1 for r in live if held.get((arm, r["id"]))) for arm in arms
                if arm != "noskill"}
    floor_held = {arm: sum(1 for i in floor_ids if held.get((arm, i))) for arm in arms
                  if arm != "noskill"}
    parse_failures = sum(1 for e in valid for v in e["checklist"] if v.get("passed") is None)
    truncations = sum(1 for e in results if e.get("artifact_truncated"))
    post_fail = [f"{e['golden']}/r{e['replicate']}: {a.get('path', a['type'])}"
                 + (f" ({a['detail']})" if a.get("detail") else "")
                 for e in valid if e["arm"] == "post"
                 for a in e["assertions"] if not a["passed"]]
    return {
        "skill": skill, "arms": arms, "replicates": replicates, "old_ref": old_ref,
        "rules_total": len(rules), "floor_total": len(floor_ids),
        "rules_pruned_by_noskill": pruned, "live_total": len(live),
        "untempted": untempted, "invited_read": any_tempts,
        "coverage_per_arm": coverage, "floor_held_per_arm": floor_held,
        "reference_arm": ref,
        "rules_lost_per_arm": lost, "floor_rules_lost_per_arm": floor_lost,
        "flaky_per_arm": flaky, "invited_flaky_per_arm": invited_flaky,
        "post_assertion_failures": post_fail,
        "invalid_runs": invalid,
        "judge_parse_failures": parse_failures,
        "artifact_truncations": truncations,
        "held": {f"{a}:{rid}": v for (a, rid), v in held.items()},
        "est_total_cost_usd": round(total_cost, 2),
        "pins_per_arm": {arm: next((e["pins"] for e in results if e["arm"] == arm and e.get("pins")), {})
                         for arm in arms},
    }


def render_report(skill, stamp, s) -> str:
    lines = [f"# Eval report — {skill} ({stamp})", "",
             f"Arms: {s['arms']} · replicates {s['replicates']} · old ref {s.get('old_ref')} · "
             f"reference arm {s.get('reference_arm')}", "",
             f"Rules: {s['rules_total']} total ({s.get('floor_total', 0)} floor), "
             f"{len(s['rules_pruned_by_noskill'])} pruned by the no-skill control (they measure "
             f"the model, not the skill), {s.get('live_total', s['rules_total'])} live"
             + (f", {len(s['untempted'])} untempted (disclosed, not read): {s['untempted']}"
                if s.get("untempted") else "")
             + (" · read over invited (golden, rule) pairs" if s.get("invited_read") else
                " · cross-golden read (no tempts declared)") + ".", ""]
    if s.get("invalid_runs"):
        lines += [f"**INVALID runs (excluded from every read): {len(s['invalid_runs'])}** — "
                  + ", ".join(s["invalid_runs"]), ""]
    for arm in s["arms"]:
        if arm == "noskill":
            continue
        cov = s.get("coverage_per_arm", {}).get(arm)
        if cov is None:  # legacy summary: derive from the held map
            cov = sum(1 for k, v in s.get("held", {}).items() if k.startswith(f"{arm}:") and v)
        fh = s.get("floor_held_per_arm", {}).get(arm)
        fl = s.get("invited_flaky_per_arm", {}).get(arm, {})
        al = s.get("flaky_per_arm", {}).get(arm, {})
        line = (f"- **{arm}** — live rules held (pass^k): {cov}/{s.get('live_total', s['rules_total'])}"
                f" · floors held {fh if fh is not None else '?'}/{s.get('floor_total', 0)}")
        if fl.get("pairs"):
            share = 100.0 * fl["flaky"] / fl["pairs"]
            under = fl["pairs"] < 8   # ADR 2026-09-09 clause 3: the band is the cap
            band = 20.0 if under else min(share + 5, 20)
            line += (f" · flaky {fl['flaky']}/{fl['pairs']} invited pairs = {share:.1f} % → band "
                     f"{band:.1f} % (all pairs {al.get('flaky')}/{al.get('pairs')})"
                     + (" UNDER-SAMPLED (< 8 invited pairs)" if under else ""))
        elif al.get("pairs"):
            line += f" · flaky {al['flaky']}/{al['pairs']} pairs (no tempts declared)"
        lines.append(line)
        if arm in s["rules_lost_per_arm"]:
            ids = s["rules_lost_per_arm"][arm]
            floors = s["floor_rules_lost_per_arm"].get(arm, [])
            verdict = "KILLED (floor rule lost)" if floors else (
                f"{len(ids)} rules lost vs {s.get('reference_arm')}" if ids else
                f"no rules lost vs {s.get('reference_arm')}")
            lines.append(f"  - {verdict}" + (f": {', '.join(ids)}" if ids else ""))
    for pv in s.get("pairwise", []):
        w1 = pv["first_order"].get("winner"); w2 = pv["swapped"].get("winner")
        lines.append(f"- pairwise {pv['arm']}/{pv['golden']}: {w1}/{w2} "
                     f"({'position-consistent' if pv['position_consistent'] else 'POSITION-INCONSISTENT'})")
    for arm, p in s.get("pins_per_arm", {}).items():
        if p:
            lines.append(f"- pins {arm}: plugin {p.get('plugin_version')} · skill {p.get('skill_sha256')} · "
                         f"rendered rules {p.get('rendered_rules_sha256')} ({p.get('rendered_rules_chars', 0):,} chars) · "
                         f"judge {p.get('judge_prompt_sha256')} · model {p.get('session_model')}")
    fails = s.get("post_assertion_failures", s.get("baseline_assertion_failures"))
    lines += ["", f"post scripted-assertion failures: {fails or 'none'}",
              f"Judge parse failures: {s.get('judge_parse_failures', 0)} · "
              f"artifact truncations: {s.get('artifact_truncations', 0)}",
              f"Estimated spend: ${s['est_total_cost_usd']} (client-side estimate)", "",
              "Judged results are ADVISORY. The ship decision is the user's ratification "
              "against `preregistration.md`."]
    return "\n".join(lines)


def cmd_report(skill: str, out: str | None) -> None:
    runs = sorted((EVALS / skill / "runs").glob("*/summary.json"))
    if out:
        runs = [EVALS / skill / "runs" / out / "summary.json"]
    if not runs or not runs[-1].is_file():
        die("no runs found")
    data = json.loads(runs[-1].read_text())
    print(render_report(skill, runs[-1].parent.name, {k: data[k] for k in data if k != "runs"}))


def main() -> None:
    global MODE
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--local", action="store_true", help="--bare + ANTHROPIC_API_KEY")
    sub = ap.add_subparsers(dest="cmd", required=True)
    for name in ("probe", "grid", "report", "rejudge"):
        p = sub.add_parser(name)
        p.add_argument("skill")
        if name in ("probe", "grid"):
            p.add_argument("--old-ref", default=None, help="git ref for the pre arm")
        if name == "probe":
            p.add_argument("--arm", default="post", choices=["pre", "post"])
        if name == "rejudge":
            p.add_argument("--out", required=True)
            p.add_argument("--only-missing", action="store_true",
                           help="re-score only sessions carrying a MISSING verdict")
        if name == "report":
            p.add_argument("--out", default=None)
        if name == "grid":
            p.add_argument("--replicates", type=int, default=3)
            p.add_argument("--arms", default="noskill,post")
            p.add_argument("--out", default=None,
                           help="run-dir name under runs/ to append into (staged grids)")
            p.add_argument("--pairwise", action="store_true")
            p.add_argument("--budget-usd", type=float, default=25.0,
                           help="halt the grid when session spend passes this (resumable)")
    args = ap.parse_args()
    MODE = "local" if args.local else "host"
    WORK.mkdir(exist_ok=True)
    if args.cmd == "probe":
        cmd_probe(args.skill, args.old_ref, args.arm)
    elif args.cmd == "grid":
        cmd_grid(args.skill, args.replicates, args.arms.split(","), args.old_ref, args.out,
                 args.pairwise, args.budget_usd)
    elif args.cmd == "rejudge":
        cmd_rejudge(args.skill, args.out, args.only_missing)
    else:
        cmd_report(args.skill, args.out)


if __name__ == "__main__":
    main()
