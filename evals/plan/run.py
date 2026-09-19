#!/usr/bin/env python3
# /// script
# dependencies = ["pyyaml"]
# ///
"""Command plan-only eval runner.

Provenance: .mochiko/brainstorms/command-plan-only-eval/record.md (D1-D11, accepted
2026-08-27) with the brainstorm-probe amendments (evals/plan/brainstorm-probe/).
Maintainer-side advisory tooling (GI-019 trace via the harness session); never shipped
(GI-020). Sibling of the skill runner evals/run.py; judge patterns adapted from it.

One run = one headless `claude -p` session in an ephemeral workdir (fixture files +
a provisioned plugins/mochiko tree), invoking the command as the prompt under a pinned
form-only wrapper (D11). The session plans; it never executes (allow-list fence, D7;
user gates described, never awaited, D9). Grading: deterministic asserts (load gate,
rule-delivery gate, name resolution, cap-hit) + a Haiku rule-coverage checklist over the D8
plan-observable subset + a stub-detection axis + a position-swapped Sonnet pairwise read.
Judges are advisory (harness D2): judged degradation never sets a nonzero exit code; only
broken mechanics (missing prereg, load-gate failure, undelivered rules) do.

The rule-delivery gate is the skill runner's, carried over: the command's own
`!`mochiko-cli …`` lines are fired against the provisioned tree before the session, and the
returned plan is scanned for the halt string a command surfaces when its rules did not
arrive. Either limb failing invalidates the run and halts the grid, because a session whose
rules never arrived plans about the halt rather than about the work, and grading that plan
would read a broken instrument as a regression. The gate is scoped to the arms that fire a
command; the `nocmd` control never loads one.

Probe-settled invocation (2026-08-27): NO --bare (it skips stored auth by design);
isolation = --setting-sources "" + neutral cwd; --allowedTools Read,Grep,Glob is a
permission fence (roster stays visible, calls are denied); max-turns 40 with cap-hit
warning.

Vocabulary shared across the eval targets (skills · commands · agents): evals/README.md
(primitive-eval-harness-v2 D1/D10). The persona target lives in agents.py beside this file
and is reached through `evals/run.py agent ...`. Shared mechanics (session ·
provisioning · judge calls · grid math) come from evals/lib/ (D10, second landing act); this
file keeps the command target's rubric, load gate, prompts, and report.

Usage — the converged CLI (D10 act 3), run under `uv run` for PyYAML:
  uv run evals/run.py command <subcommand> <cmd> ...
    partition <cmd> --old-ref <git-ref>       four ID-keyed rubric buckets (D6)
    check-rubric <cmd>                        observable.yaml covers the schema exactly (D8)
    check-fixtures <cmd>                      every path a fixture references exists
    plan-run <cmd> <golden-id> [--arm post|pre|nocmd] [--old-ref REF] [--out DIR]
    grid <cmd> [--replicates 3] [--old-ref REF] [--control] [--out NAME]
    judge <cmd> <run-name>                    coverage + stub + pairwise over stored plans
    report <cmd> <run-name>                   bucket diff, pass^k, noise guard
  uv run evals/run.py agent <subcommand> <persona> ...   (agents.py)
    mint <persona> --old-ref <ref>            mint/re-mint rules.json over pre∪post (v2 D5/I3)
    check <persona> [--old-ref REF]           completeness + partition + temptation (v2 I4/I5)
    plan-run <persona> <golden> [--arm post|pre|nopersona] [--old-ref REF] [--out DIR]
    prune <persona> [--replicates 3]          one-time nopersona pass, untagged ids only (v2 D7/R3)
    grid <persona> [--replicates 3] [--old-ref REF] [--out NAME]   pre/post, persona alone (v2 D6)
    judge <persona> <run-name>                embodiment checklist + pairwise
    report <persona> <run-name>               common regressions · added adoption · removed ghosts
    label-sheet <persona> <run-name> [--size 24]   hand-labelling sheet for judge calibration (v2 I9)
    calibrate <persona> <run-name> --labels <sheet.json>   agreement vs the labels → calibration.json
  Deprecated for one release: `uv run evals/plan/run.py <subcommand> ...` and the persona
  spellings `agent-<subcommand>` (this file's `main` forwards them), and the pre-rename
  `uv run evals/commands/run.py ...` (a shim at the old path).
"""

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

_EVALS_DIR = str(pathlib.Path(__file__).resolve().parent.parent)
if _EVALS_DIR not in sys.path:   # `lib` and this `plan` package live under evals/
    sys.path.insert(0, _EVALS_DIR)
from lib import EVALS, PLUGIN, REPO  # noqa: E402
from lib import judge as J, provision as P, session as S  # noqa: E402
from lib.stats import BAND_CAP, band, flaky, missing, passk  # noqa: E402 — the report's grid math

try:
    import yaml
except ImportError:
    print("error: PyYAML unavailable — run the command and agent targets via "
          "`uv run evals/run.py command|agent ...`", file=sys.stderr)
    sys.exit(2)

CMD_EVALS = EVALS / "plan"
WRAPPER = CMD_EVALS / "wrapper.md"

SESSION_MODEL = "sonnet"    # session under test (skill-harness R7 carried over)
CHECKLIST_MODEL = "haiku"   # coverage + stub judge
PAIRWISE_MODEL = "sonnet"   # pairwise judge
MAX_TURNS = 40              # probe finding 10: 25 hit the cap; headroom + warning
JUDGE_CHUNK = J.JUDGE_CHUNK # lib.judge; skill-harness staged-001 finding: big arrays misparse
ARMS = ["pre", "post", "nocmd"]


die = S.die


# ---------- schema access ----------

# The schema files shipped under plugins/mochiko/schemas/ until v0.107.0; from wave 6 the
# rules live in the migration log and the repo-side derived views at .mochiko/schema-views/
# are their projection (same shape, regenerated never hand-edited). A ref is read from
# whichever of the two it carries — old refs the schema file, current refs the view.
SCHEMA_CANDIDATES = ("plugins/mochiko/schemas/{cmd}.yaml",
                     ".mochiko/schema-views/commands/{cmd}.yaml")
COMMON_CANDIDATES = ("plugins/mochiko/schemas/common.yaml",
                     ".mochiko/schema-views/common/common.yaml")


def read_at_ref(rels: tuple, old_ref: str | None) -> str | None:
    """The first candidate path that exists at the working tree or at `old_ref`."""
    for rel in rels:
        if old_ref is None:
            path = REPO / rel
            if path.is_file():
                return path.read_text()
        else:
            proc = subprocess.run(["git", "-C", str(REPO), "show", f"{old_ref}:{rel}"],
                                  capture_output=True, text=True)
            if proc.returncode == 0:
                return proc.stdout
    return None


def schema_text(cmd: str, old_ref: str | None = None) -> str:
    text = read_at_ref(tuple(r.format(cmd=cmd) for r in SCHEMA_CANDIDATES), old_ref)
    if text is None:
        die(f"no schema or derived view for {cmd}" + (f" at {old_ref}" if old_ref else "")
            + f" — looked at {[r.format(cmd=cmd) for r in SCHEMA_CANDIDATES]}")
    return text


def common_blocks(old_ref: str | None = None) -> dict:
    """{common.<slug>: block} from the common file (schema or derived view), or {} where
    none exists (pre-extends refs). Same resolution semantics as the command .md instructs:
    a stub inherits every field; a locally declared field replaces the inherited one."""
    raw = read_at_ref(COMMON_CANDIDATES, old_ref)
    if raw is None:
        return {}
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
    return P.provision_plugin(dest / "plugins" / "mochiko", old_ref or P.WORKTREE)


def wrapper_text() -> str:
    if not WRAPPER.is_file():
        die(f"{WRAPPER} missing — the pinned wrapper is part of the instrument")
    return WRAPPER.read_text()


def rendered_rules(cmd: str, plugin_dir: pathlib.Path) -> tuple:
    """Fire the command's own `!`mochiko-cli …`` delivery lines against the provisioned
    tree, exactly as the session will, and report whether every one of them exited 0.

    The skill runner carries the same check. A tree whose command file has no delivery
    line — every pre-v0.107.0 ref, where the rules still shipped as a schema file — renders
    nothing and passes: the gate is about a slot that failed, never a slot that does not
    exist yet. A non-zero exit here means the installed `mochiko-cli` cannot serve this
    tree's migration log, so the session under it would plan against undelivered rules.
    """
    body = plugin_dir / "commands" / f"{cmd}.md"
    if not body.is_file():
        return "", True
    out, ok = [], True
    env = {**os.environ, "CLAUDE_PLUGIN_ROOT": str(plugin_dir)}
    for m in re.finditer(r"^!`(.+)`\s*$", body.read_text(), re.M):
        line = m.group(1).replace("${CLAUDE_PLUGIN_ROOT}", str(plugin_dir))
        if not line.startswith("mochiko-cli "):
            continue          # only the rule-delivery lines are the instrument's concern
        r = subprocess.run(["sh", "-c", line], capture_output=True, text=True, env=env)
        ok = ok and r.returncode == 0
        out.append(r.stdout)
    return "\n".join(out), ok


def pins(cmd: str, plugin_dir: pathlib.Path) -> dict:
    """C3: versions pinned into every run's meta and the baseline."""
    mcli = subprocess.run(["mochiko-cli", "--version"], capture_output=True,
                          text=True).stdout.strip()
    rendered, ok = rendered_rules(cmd, plugin_dir)
    return {"plugin_version": P.plugin_version(plugin_dir), "cli": S.claude_version(),
            "mochiko_cli": mcli, "session_model": SESSION_MODEL,
            "wrapper_sha256": hashlib.sha256(wrapper_text().encode()).hexdigest()[:16],
            "rendered_rules_ok": ok,
            "rendered_rules_sha256": (hashlib.sha256(rendered.encode()).hexdigest()[:16]
                                      if ok and rendered.strip() else None),
            "rendered_rules_chars": len(rendered) if ok else 0}


parse_stream = S.parse_stream   # init · result · tool_calls (+ inputs, models, skills fired)


def plan_session(cmd: str, golden: dict, arm: str, old_ref: str | None) -> dict:
    """One plan-only session. Returns plan text + meta + deterministic asserts."""
    with tempfile.TemporaryDirectory(prefix="cmdeval-") as td:
        wd = pathlib.Path(td) / "ws"
        wd.mkdir()
        plug = provision_workdir(wd, fixture_dir(cmd, golden), old_ref)
        run_pins = pins(cmd, plug)
        if arm == "nocmd":
            prompt = golden["control_prompt"]
        else:
            args_part = golden.get("args", "")
            prompt = f"/mochiko:{cmd} {args_part}".strip()
        # --plugin-dir absolute and authoritative (N4); --setting-sources "" isolates and
        # keeps auth (probe f.1); --allowedTools Read,Grep,Glob under dontAsk is the D7
        # permission fence.
        argv = S.claude_argv(prompt, model=SESSION_MODEL, max_turns=MAX_TURNS,
                             plugin_dir=plug, allowed_tools="Read,Grep,Glob",
                             permission_mode="dontAsk", append_system_prompt=wrapper_text())
        proc = S.run_claude(argv, cwd=wd, timeout=1800)
        out = parse_stream(proc.stdout)
        init, result = out["init"], out["result"]
        if result is None:
            die(f"no result event (exit {proc.returncode}): {proc.stderr[-800:]}")
        plan = result.get("result") or ""
        # Blocking load gate (I1): the pair under test visible in the init event.
        loaded = S.loaded_plugins(init)
        load_ok = ("mochiko", run_pins["plugin_version"]) in loaded
        # Delivery gate: a command whose rules did not arrive says so and halts, and the
        # plan is then about the halt rather than about the work. Both limbs are scoped to
        # the arms that fire the command — the `nocmd` control never loads one.
        rules_ok = run_pins["rendered_rules_ok"] if arm != "nocmd" else None
        delivered = ("rules not delivered" not in plan) if arm != "nocmd" else None
        asserts = {
            "load_gate": load_ok,
            "loaded_plugins": loaded,
            "rendered_rules_ok": rules_ok,
            "rules_delivered": delivered,
            "cap_hit": result.get("num_turns") == MAX_TURNS,   # warning, not failure
            "fence_breach": sorted({t for t in out["tool_calls"]
                                    if t not in ("Read", "Grep", "Glob")}),
            "name_resolution": name_resolution(plan) if arm != "nocmd" else [],
            "auth_failure": S.auth_failed(plan),
        }
        if asserts["auth_failure"] or not load_ok:
            die(f"run invalid — load_gate:{load_ok} loaded:{loaded} "
                f"auth_failure:{asserts['auth_failure']} (arm {arm}, {golden['id']})")
        if rules_ok is False or delivered is False:
            die(f"run invalid — mochiko-cli rules not delivered "
                f"(rendered_rules_ok:{rules_ok} plan_reports_delivery:{delivered}, "
                f"arm {arm}, {golden['id']}, plugin {run_pins['plugin_version']}, "
                f"mochiko-cli {run_pins['mochiko_cli'] or 'absent'}) — install a binary "
                f"whose grammar range covers this tree's migration log "
                f"(`cargo install --path crates/mochiko-cli`), then re-run")
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
                "setup", "specify", "mochiko",
                # governance comment markers (`<!-- mochiko:governance -->` and kin) a setup
                # plan quotes — region names, not primitives (false-positive in every setup run)
                "governance", "domain-registry", "output-style"}:
            bad.append(f"mochiko:{m}")
    return sorted(bad)


# ---------- judges (mechanics in lib.judge; the prompts pinned here; advisory) ----------

judge_session = J.judge_session
extract_json = J.extract_json


def scrub_rule_ids(plan: str, rule_ids) -> str:
    """Smoke finding: sessions cite rule IDs despite the wrapper's register line.
    Scrub every known ID before judging so the coverage judge cannot lexically match
    and must grade embodiment (D3). Mechanical and content-neutral."""
    for rid in sorted(rule_ids, key=len, reverse=True):
        plan = plan.replace(rid, "[rule]")
    # extends stubs: sessions may cite the common.<slug> source ID instead
    plan = re.sub(r"\bcommon\.[a-z][a-z0-9.-]*\b", "[rule]", plan)
    return plan


def coverage_prompt(chunk: list, plan: str) -> str:
    return (
        "You are grading a command's PLANNED-run action plan against rules the "
        "command is bound to. The plan speaks in concrete actions and may never "
        "cite rule IDs — grade EMBODIMENT: does the plan's described behavior "
        "enact the rule? For EACH rule return a JSON array entry "
        "{\"id\": ..., \"verdict\": \"reflected\"|\"absent\"|\"contradicted\", "
        "\"evidence\": \"<verbatim quote from the plan proving the verdict, or "
        "empty for absent>\"}. Every rule id exactly once. Output ONLY the JSON "
        "array.\n\nRULES:\n" + json.dumps(chunk, indent=1)
        + "\n\nPLAN:\n" + plan[:120_000])


def judge_coverage(rule_items: list, plan: str, model: str = CHECKLIST_MODEL) -> list:
    """One binary per observable rule: does the PLAN's content EMBODY the rule —
    reflected in its actions, absent, or contradicted (D2/D3). Quoted plan-line
    evidence. Chunked + two retries (skill-harness staged-001 finding; the second
    retry is the command target's, added after a chunk came back unparseable)."""
    return J.judge_chunked(rule_items, lambda chunk: coverage_prompt(chunk, plan), model,
                           key="verdict", attempts=3, accept=lambda v: bool(v.get("verdict")),
                           chunk_size=JUDGE_CHUNK)


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


def pairwise_prompt(first: str, second: str) -> str:
    return ("Two action plans answer the same command invocation. Which is the "
            "better plan overall — more faithful, more concretely actionable? "
            "Reply ONLY JSON {\"winner\": \"1\"|\"2\"|\"tie\", \"reason\": "
            "\"<one sentence>\"}.\n\nPLAN 1:\n" + first[:60_000]
            + "\n\nPLAN 2:\n" + second[:60_000])


def judge_pairwise(text_a: str, text_b: str, model: str = PAIRWISE_MODEL) -> dict:
    """Blind A/B with position swap (lib.judge mechanics; this target's prompt)."""
    return J.judge_pairwise(text_a, text_b, model, pairwise_prompt)


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
              pairwise_model: str = PAIRWISE_MODEL, pairwise: bool = False) -> None:
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
    # Pairwise is opt-in (the Sonnet A/B read chose position 2 in 23 of 24 calls across the
    # persona pilots — ADR 2026-09-09-persona-pilot-2-validator-read; same read here).
    if pairwise and "pre" in meta["arms"]:
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


# pass^k · flaky · missing: lib.stats (imported above).


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
    # Noise band (primitive-eval-harness-v2 D11: measured band + stopping rule, applied to
    # the command target at its next grid). Commands' goldens declare no tempts, so the
    # band is the all-pairs replicate-disagreement share per arm over the observable rules,
    # plus five points, capped at 20 %; fewer than eight pairs = UNDER-SAMPLED (band = cap).
    lines.append("## Band input — all goldens (all (golden, rule) pairs; v2 D11)")
    for arm in [a for a in meta["arms"] if a != "nocmd"]:
        n_f = n_p = 0
        for g in goldens:
            entries = [e for e in meta["runs"] if e["golden"] == g and e["arm"] == arm]
            if len(entries) < 2:
                continue
            for rid in obs:
                if missing(entries, rid) == len(entries):
                    continue
                n_p += 1
                n_f += flaky(entries, rid)
        if n_p:
            # One band arithmetic for every target (lib.stats); an under-sampled arm reads the
            # cap itself as its band, per the skill target's ADR 2026-09-09 clause 3.
            share, band_pct, under = band(n_f, n_p)
            if under:
                band_pct = BAND_CAP
            lines.append(f"- {arm}: flaky {n_f}/{n_p} pairs = {share:.1f} % → "
                         f"band {band_pct:.1f} %"
                         + (" UNDER-SAMPLED (< 8 pairs)" if under else "")
                         + " — an arm above its band is noise-dominated: no pre/post difference is"
                           " read; one extra replicate per arm, once; two re-keys without a"
                           " detectable control return the target to the user")
    lines.append("")
    for p in meta.get("pairwise", []):
        w = p["first_order"].get("winner"), p["swapped"].get("winner")
        lines.append(f"- pairwise {p['golden']}/r{p['replicate']}: {w} "
                     f"(position_consistent={p['position_consistent']})")
    (rd / "report.md").write_text("\n".join(lines) + "\n")
    print(f"report: {rd / 'report.md'}")


def add_subcommands(sub) -> None:
    """The command target's subcommands under `evals/run.py command <subcommand> <cmd> ...`."""
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
            p.add_argument("--pairwise", action="store_true",
                           help="opt-in Sonnet A/B read (position-biased in both pilots)")


def dispatch(a) -> None:
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
        cmd_judge(a.command, a.run_name, a.judge_model, a.pairwise_model, a.pairwise)
    elif a.cmd == "report":
        cmd_report(a.command, a.run_name)


def main() -> None:
    """Deprecated entrypoint, kept one release (D10 act 3): `uv run evals/plan/run.py <sub> ...`
    forwards to `evals/run.py command <sub> ...`, and `agent-<sub> ...` to
    `evals/run.py agent <sub> ...`; the converged CLI is the one code path."""
    argv = sys.argv[1:]
    first = next((i for i, a in enumerate(argv) if not a.startswith("-")), None)
    if first is not None and argv[first].startswith("agent-"):
        mapped = ["agent", argv[first][len("agent-"):], *argv[:first], *argv[first + 1:]]
    else:
        mapped = ["command", *argv]
    old_sub = argv[first] if first is not None else "<subcommand>"
    new_sub = " ".join(mapped[:2]) if first is not None else "command <subcommand>"
    print(f"deprecated: `evals/plan/run.py {old_sub}` is now `evals/run.py {new_sub}`; "
          "this entrypoint goes away next release", file=sys.stderr)
    os.execv(sys.executable, [sys.executable, str(EVALS / "run.py"), *mapped])


if __name__ == "__main__":
    main()
