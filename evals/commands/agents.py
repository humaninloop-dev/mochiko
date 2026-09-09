"""Persona (agent) target of the plan-only runner.

Provenance: .mochiko/brainstorms/primitive-eval-harness-v2/record.md (D1-D13, accepted
2026-09-08). Maintainer-side advisory tooling (GI-019 trace); never shipped (GI-020).
Vocabulary shared with the other targets: evals/README.md.

One run = one headless `claude -p` session seated AS the persona (`--agent
mochiko:<persona>`, D3) from a fixture workdir, behind a reads-only roster
(`--tools Read,Grep,Glob` — the D3 fence in roster form: write, shell, spawn, and
skill-load tools are absent, not merely denied; `--disallowedTools` left Workflow and
the messaging tools in the roster at CLI 2.1.258), with `--model opus` explicit on
every arm (R5: `--model` overrides the persona's frontmatter pin under `--agent`),
`--permission-mode acceptEdits` (M2: the sibling ruling's probe-settled mode, pinned),
and the pinned form-only wrapper appended to the system prompt (probe 2026-09-08:
`--append-system-prompt` reaches an `--agent` seat). The plugin tree is provisioned
outside the workspace cwd so the persona cannot Read its skills' files through the
fence (smoke finding 2026-09-08). The persona plans; it never executes. Graded
surface: the plan plus the read-trace (tool_use Read/Grep/Glob events in the stream,
with paths).

Rubric (D5): minted from the persona's own body over the union of the `pre` and `post`
refs — every bullet under Quality Standards / What You Reject / What You Embrace /
Your Judgment and every sentence of the persona's craft sections is a unit; each unit
maps to >= 1 claim (compound units split by hand) or is listed in `not_claims`; each
claim carries a stable id (kept across mints by source sha), a partition value
(`plan-observable` | `out-of-instrument` + why), tags (added / removed / common), and
the `model_native` / `untempted` marks. Arms: `pre` and `post`, persona alone (D6);
`nopersona` is a one-time prune pass over untagged ids (D7, R3) — it runs without
`--plugin-dir`, so it also lacks the plugin's SessionStart hook line that `pre`/`post`
carry; the hook is identical at both refs, so D6's one-variable difference holds and the
control differs from the arms in the persona and that one harness line, disclosed here.
"""

import datetime
import hashlib
import json
import pathlib
import re
import os
import shutil
import subprocess
import sys
import tempfile

import run as cmdrun  # the plan-only runner: shared session/judge/report mechanics

REPO = cmdrun.REPO
PLUGIN = cmdrun.PLUGIN
AGENT_EVALS = REPO / "evals" / "agents"
WRAPPER = AGENT_EVALS / "wrapper.md"

ARM_MODEL = "opus"                # R5: explicit on every arm, control included
TOOLS = "Read,Grep,Glob"          # D3 fence, roster form (C1: Skill out)
PERMISSION_MODE = "acceptEdits"   # M2: the sibling ruling's probe-settled mode, recorded in pins
MAX_TURNS = cmdrun.MAX_TURNS
ARMS = ["pre", "post", "nopersona"]
PARTITIONS = ("plan-observable", "out-of-instrument")

# D5: the claim-bearing sections; everything else under `## ` that is not excluded is a
# craft section (sentences are the units). Excluded sections are not unit sources at all.
CLAIM_SECTIONS = {"Quality Standards", "What You Reject", "What You Embrace",
                  "Your Judgment"}
EXCLUDED_SECTIONS = {"Core Identity", "What You Produce", "Skills Available",
                     "Skills you lean on"}

die = cmdrun.die


# ---------- persona body -> units ----------

def persona_path(persona: str) -> pathlib.Path:
    return PLUGIN / "agents" / f"{persona}.md"


def persona_text(persona: str, old_ref: str | None = None) -> str:
    rel = f"plugins/mochiko/agents/{persona}.md"
    if old_ref is None:
        path = REPO / rel
        if not path.is_file():
            die(f"no persona at {path}")
        return path.read_text()
    proc = subprocess.run(["git", "-C", str(REPO), "show", f"{old_ref}:{rel}"],
                          capture_output=True, text=True)
    if proc.returncode != 0:
        die(f"git show {old_ref}:{rel} failed: {proc.stderr.strip()}")
    return proc.stdout


def split_frontmatter(text: str) -> tuple[str, str]:
    m = re.match(r"^---\n(.*?)\n---\n(.*)$", text, re.S)
    if not m:
        die("persona file has no frontmatter fence")
    return m.group(1), m.group(2)


def normalize(s: str) -> str:
    return re.sub(r"\s+", " ", s).strip()


def sha(s: str) -> str:
    return hashlib.sha256(normalize(s).encode()).hexdigest()[:12]


def sections(body: str) -> list[tuple[str, str]]:
    """[(title, text)] for every `## ` section of the body, in order."""
    out, title, buf = [], None, []
    for line in body.splitlines():
        if line.startswith("## "):
            if title is not None:
                out.append((title, "\n".join(buf)))
            title, buf = line[3:].strip(), []
        elif title is not None:
            buf.append(line)
    if title is not None:
        out.append((title, "\n".join(buf)))
    return out


def bullets(text: str) -> list[str]:
    """Top-level `- ` bullets with their continuation lines joined."""
    items, cur = [], None
    for line in text.splitlines():
        if re.match(r"^- ", line):
            if cur is not None:
                items.append(cur)
            cur = line[2:].strip()
        elif cur is not None and line.startswith("  ") and line.strip():
            cur += " " + line.strip()
        elif cur is not None and not line.strip():
            items.append(cur)
            cur = None
    if cur is not None:
        items.append(cur)
    return items


def numbered(text: str) -> list[str]:
    """Top-level `1. ` items with continuation lines joined (review change 4)."""
    items, cur = [], None
    for line in text.splitlines():
        if re.match(r"^\d+\. ", line):
            if cur is not None:
                items.append(cur)
            cur = re.sub(r"^\d+\. ", "", line).strip()
        elif cur is not None and line.startswith("  ") and line.strip():
            cur += " " + line.strip()
        elif cur is not None and not line.strip():
            items.append(cur)
            cur = None
    if cur is not None:
        items.append(cur)
    return items


def fences(text: str) -> tuple[list[str], str]:
    """Fenced code blocks as whole units, and the text with them removed (change 4)."""
    blocks = re.findall(r"```[^\n]*\n.*?```", text, re.S)
    return [normalize(b) for b in blocks], re.sub(r"```[^\n]*\n.*?```", "", text, flags=re.S)


def sentences(text: str) -> list[str]:
    """Sentences of the non-bullet, non-numbered, non-fenced prose in a craft section."""
    _, text = fences(text)
    prose = "\n".join(l for l in text.splitlines()
                      if not re.match(r"^- ", l) and not re.match(r"^\d+\. ", l)
                      and not l.startswith("  "))
    prose = normalize(prose)
    if not prose:
        return []
    parts = re.split(r"(?<=[.!?])\s+(?=[A-Z`*(\"“])", prose)
    return [p.strip() for p in parts if len(p.strip()) > 1]


def units(body: str) -> list[dict]:
    """The D5 unit list: {section, kind, text, sha}. Excluded sections yield nothing."""
    out = []
    for title, text in sections(body):
        if title in EXCLUDED_SECTIONS:
            continue
        for b in bullets(text):
            out.append({"section": title, "kind": "bullet", "text": normalize(b),
                        "sha": sha(b)})
        if title not in CLAIM_SECTIONS:            # craft section: prose is a unit source
            for n in numbered(text):
                out.append({"section": title, "kind": "numbered", "text": normalize(n),
                            "sha": sha(n)})
            blocks, _ = fences(text)
            for f in blocks:
                out.append({"section": title, "kind": "fence", "text": f, "sha": sha(f)})
            for s in sentences(text):
                out.append({"section": title, "kind": "sentence", "text": s,
                            "sha": sha(s)})
    return out


# ---------- rules.json (the rubric) ----------

def rules_path(persona: str) -> pathlib.Path:
    return AGENT_EVALS / persona / "rules.json"


def load_rules(persona: str) -> dict:
    path = rules_path(persona)
    if not path.is_file():
        die(f"{path} missing — mint it first: agent-mint {persona} --old-ref <ref>")
    return json.loads(path.read_text())


def slug(s: str, words: int = 4) -> str:
    toks = re.findall(r"[a-z0-9]+", s.lower())
    stop = {"the", "a", "an", "and", "or", "to", "of", "is", "are", "you", "your",
            "that", "it", "in", "on", "for", "so", "as", "with", "never", "always"}
    keep = [t for t in toks if t not in stop] or toks
    return "-".join(keep[:words]) or "unit"


def mint(persona: str, old_ref: str) -> dict:
    """Mint or re-mint rules.json over the union of pre/post units (I3), keeping every
    known claim's id, partition, why, and marks by source sha; new units become drafts."""
    pre_units = units(split_frontmatter(persona_text(persona, old_ref))[1])
    post_units = units(split_frontmatter(persona_text(persona))[1])
    pre_shas = {u["sha"] for u in pre_units}
    post_shas = {u["sha"] for u in post_units}
    by_sha = {u["sha"]: u for u in pre_units + post_units}

    path = rules_path(persona)
    existing = json.loads(path.read_text()) if path.is_file() else {}
    old_claims = existing.get("claims") or []
    not_claims = set(existing.get("not_claims") or [])
    known = {}
    for c in (existing.get("retired") or []):      # a retired unit that returns keeps its id
        known.setdefault(c["source"]["sha"], []).append(c)
    for c in old_claims:
        known.setdefault(c["source"]["sha"], []).append(c)
    used_ids = {c["id"] for c in old_claims}

    claims = []
    retired = [c for s, cs in known.items() if s not in by_sha for c in cs]
    for s, u in by_sha.items():
        tag = ("common" if s in pre_shas and s in post_shas
               else "added" if s in post_shas else "removed")
        if s in known:
            for c in known[s]:
                c = dict(c)
                c["tag"] = tag
                c["text"] = u["text"] if len(known[s]) == 1 else c["text"]
                claims.append(c)
            continue
        if s in not_claims:
            continue
        base = f"{persona}.{slug(u['section'], 3)}.{slug(u['text'])}"
        cid, n = base, 2
        while cid in used_ids:
            cid, n = f"{base}-{n}", n + 1
        used_ids.add(cid)
        claims.append({
            "id": cid, "text": u["text"],
            "source": {"section": u["section"], "kind": u["kind"], "sha": s},
            "tag": tag, "partition": "unpartitioned", "why": "",
            "model_native": False, "untempted": False, "draft": True,
        })
    claims.sort(key=lambda c: (c["source"]["section"], c["id"]))
    doc = {
        "persona": persona,
        "provenance": "primitive-eval-harness-v2 D5 (+ I3 union-of-refs, I4 completeness, "
                      "D7/R3 model_native carry-forward, I5 untempted)",
        "minted_at": datetime.datetime.now(datetime.timezone.utc).isoformat(timespec="seconds"),
        "refs": {"pre": old_ref, "post": "working-tree"},
        "unit_counts": {"pre": len(pre_units), "post": len(post_units),
                        "union": len(by_sha)},
        "not_claims": sorted(not_claims),
        "claims": claims,
        "retired": retired + [c for c in (existing.get("retired") or [])
                              if c["source"]["sha"] not in by_sha],
    }
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(doc, indent=1, ensure_ascii=False) + "\n")
    return doc


def check(persona: str, old_ref: str | None = None) -> int:
    """I4 completeness + partition validity + I5 temptation coverage. Returns the
    number of failures (0 = OK). Mechanical, no LLM."""
    doc = load_rules(persona)
    old_ref = old_ref or doc["refs"]["pre"]
    pre_units = units(split_frontmatter(persona_text(persona, old_ref))[1])
    post_units = units(split_frontmatter(persona_text(persona))[1])
    live = {u["sha"]: u for u in pre_units + post_units}
    not_claims = set(doc.get("not_claims") or [])
    claims = doc["claims"]
    bad = 0

    def fail(msg):
        nonlocal bad
        bad += 1
        print(f"FAIL {msg}")

    ids = [c["id"] for c in claims]
    for dup in {i for i in ids if ids.count(i) > 1}:
        fail(f"duplicate claim id {dup}")
    covered = {c["source"]["sha"] for c in claims}
    for s, u in live.items():
        if s not in covered and s not in not_claims:
            fail(f"unit unmapped ({u['section']} · {u['kind']}): {u['text'][:80]!r}")
    for c in claims:
        if c["source"]["sha"] not in live:
            fail(f"{c['id']}: source unit no longer exists in either ref")
        if c.get("partition") not in PARTITIONS:
            fail(f"{c['id']}: partition {c.get('partition')!r} not in {PARTITIONS}")
        elif c["partition"] == "out-of-instrument" and not c.get("why"):
            fail(f"{c['id']}: out-of-instrument without a why")
        if c.get("draft"):
            fail(f"{c['id']}: still a draft — split or confirm, then drop `draft`")
    for s in not_claims & covered:
        fail(f"sha {s} is both a claim source and in not_claims")
    # I5: every graded claim is tempted by >= 1 golden or tagged untempted
    goldens = load_goldens(persona) if (AGENT_EVALS / persona / "evals.json").is_file() else []
    tempted = {t for g in goldens for t in g.get("tempts", [])}
    for t in tempted - set(ids):
        fail(f"golden tempts unknown claim id {t}")
    graded = [c for c in claims if c.get("partition") == "plan-observable"
              and not c.get("model_native") and c.get("tag") != "removed"]
    for c in graded:
        if c["id"] not in tempted and not c.get("untempted"):
            fail(f"{c['id']}: plan-observable, not tempted by any golden, not tagged untempted")
    obs = sum(1 for c in claims if c.get("partition") == "plan-observable")
    print(f"rubric {persona}: {len(claims)} claims · {obs} plan-observable · "
          f"{len(claims) - obs} out-of-instrument · {sum(1 for c in claims if c.get('model_native'))} "
          f"model-native · tags "
          f"{ {t: sum(1 for c in claims if c.get('tag') == t) for t in ('common', 'added', 'removed')} } · "
          f"{'OK' if bad == 0 else f'{bad} failure(s)'}")
    return bad


# ---------- goldens & fixtures ----------

def load_goldens(persona: str) -> list:
    path = AGENT_EVALS / persona / "evals.json"
    if not path.is_file():
        die(f"{path} missing")
    return json.loads(path.read_text())


def fixture_dir(persona: str, golden: dict) -> pathlib.Path:
    d = AGENT_EVALS / persona / "fixtures" / golden["fixture"]
    if not d.is_dir():
        die(f"fixture dir missing: {d}")
    return d


# ---------- session ----------

def wrapper_text() -> str:
    if not WRAPPER.is_file():
        die(f"{WRAPPER} missing — the pinned wrapper is part of the instrument")
    return WRAPPER.read_text()


def pins(persona: str, plugin_dir: pathlib.Path | None, old_ref: str | None) -> dict:
    body = persona_text(persona, old_ref)
    cli = subprocess.run(["claude", "--version"], capture_output=True,
                         text=True).stdout.strip()
    out = {"persona": persona, "persona_sha256": hashlib.sha256(body.encode()).hexdigest()[:16],
           "cli": cli, "arm_model": ARM_MODEL, "tools": TOOLS,
           "permission_mode": PERMISSION_MODE,
           "wrapper_sha256": hashlib.sha256(wrapper_text().encode()).hexdigest()[:16]}
    if plugin_dir is not None:
        manifest = plugin_dir / ".claude-plugin" / "plugin.json"
        out["plugin_version"] = json.loads(manifest.read_text()).get("version")
    return out


def parse_stream(stdout: str) -> dict:
    """The command runner's parser plus the read-trace (paths) and the models seen."""
    base = cmdrun.parse_stream(stdout)
    reads, models = [], set()
    for line in stdout.splitlines():
        try:
            ev = json.loads(line.strip())
        except (json.JSONDecodeError, AttributeError):
            continue
        if ev.get("type") != "assistant":
            continue
        msg = ev.get("message") or {}
        if isinstance(msg, dict) and msg.get("model"):
            models.add(msg["model"])
        for b in msg.get("content") or []:
            if isinstance(b, dict) and b.get("type") == "tool_use":
                inp = b.get("input") or {}
                reads.append({"tool": b.get("name"),
                              "target": inp.get("file_path") or inp.get("pattern")
                              or inp.get("path"),
                              "path": inp.get("path")})   # Glob/Grep search root
    base["reads"] = reads
    base["models"] = sorted(models)
    return base


def inside(target: str, root: pathlib.Path) -> bool:
    """Is `target` under `root`? Real paths, path-component-safe (`ws2` is not inside
    `ws`); a relative target resolves against the root. macOS reports the temp
    workspace through the `/private/var` symlink while tempfile hands out `/var/...`."""
    real_root = os.path.realpath(str(root))
    real_target = os.path.realpath(target if os.path.isabs(target)
                                   else os.path.join(real_root, target))
    try:
        return os.path.commonpath([real_root, real_target]) == real_root
    except ValueError:
        return False


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


def plan_session(persona: str, golden: dict, arm: str, old_ref: str | None) -> dict:
    """One plan-only persona session (or the bare-model `nopersona` control)."""
    if arm not in ARMS:
        die(f"unknown arm {arm}")
    with tempfile.TemporaryDirectory(prefix="agenteval-") as td:
        wd = pathlib.Path(td) / "ws"
        wd.mkdir()
        shutil.copytree(fixture_dir(persona, golden), wd, dirs_exist_ok=True)
        plug = None
        if arm != "nopersona":
            # The plugin tree is provisioned OUTSIDE the workspace cwd (smoke finding
            # 2026-09-08: seated inside it, the persona Read its own skills' files through
            # the fence — `Glob **/*` from cwd reached plugins/mochiko/skills). A sibling
            # directory keeps D6's "persona alone" true for reads as well as tools.
            plug = provision_plugin(pathlib.Path(td) / "plugin",
                                    old_ref if arm == "pre" else None)
        run_pins = pins(persona, plug, old_ref if arm == "pre" else None)
        argv = ["claude", "-p", golden["card"]]
        if plug is not None:
            argv += ["--agent", f"mochiko:{persona}", "--plugin-dir", str(plug)]
        argv += ["--setting-sources", "",
                 "--tools", TOOLS,
                 "--permission-mode", PERMISSION_MODE,
                 "--max-turns", str(MAX_TURNS),
                 "--model", ARM_MODEL,
                 "--output-format", "stream-json", "--verbose",
                 "--append-system-prompt", wrapper_text()]
        proc = subprocess.run(argv, cwd=wd, capture_output=True, text=True, timeout=1800)
        out = parse_stream(proc.stdout)
        init, result = out["init"], out["result"]
        if result is None:
            die(f"no result event (exit {proc.returncode}): {proc.stderr[-800:]}")
        plan = result.get("result") or ""
        agents = (init or {}).get("agents") or []
        loaded = [(p.get("name"), p.get("version")) for p in (init or {}).get("plugins", [])]
        load_ok = (arm == "nopersona") or (
            f"mochiko:{persona}" in agents
            and ("mochiko", run_pins.get("plugin_version")) in loaded)
        roster = set((init or {}).get("tools") or [])
        asserts = {
            "load_gate": load_ok,
            "loaded_plugins": loaded,
            "roster_extra": sorted(roster - {"Read", "Grep", "Glob"}),   # fence as roster
            "fence_breach": sorted({t for t in out["tool_calls"] if t not in ("Read", "Grep", "Glob")}),
            "model_ok": all(m.startswith(f"claude-{ARM_MODEL}") for m in out["models"]) and bool(out["models"]),
            "models": out["models"],
            "cap_hit": result.get("num_turns") == MAX_TURNS,
            "auth_failure": "Not logged in" in plan,
            "files_read_line": bool(re.search(r"^FILES-READ:", plan, re.M)),
            # D6 watch: any Read whose absolute target lies outside the workspace — the
            # persona reaching for its plugin/skill files, or anything else off-fixture.
            "reads_outside_ws": sorted(
                {x for r in out["reads"]
                 for x in ((r["target"],) if r["tool"] == "Read" else (r.get("path"),))
                 if x and not inside(x, wd)}),
        }
        if asserts["auth_failure"] or not load_ok or not asserts["model_ok"]:
            die(f"run invalid — load_gate:{load_ok} model_ok:{asserts['model_ok']} "
                f"models:{out['models']} auth_failure:{asserts['auth_failure']} "
                f"(arm {arm}, {golden['id']})")
        return {"golden": golden["id"], "arm": arm, "plan": plan, "pins": run_pins,
                "asserts": asserts, "reads": out["reads"],
                "cost_usd": result.get("total_cost_usd"),
                "num_turns": result.get("num_turns"),
                "duration_ms": result.get("duration_ms")}


# ---------- judge (embodiment; recitation reads as absent) ----------

def judge_items(doc: dict, *, for_prune: bool = False, include_removed: bool = False) -> list:
    """The claims a judge grades. Grid judging: plan-observable, not model-native,
    removed claims included when asked (I3: their disappearance is read). Prune: only
    plan-observable claims never yet judged by a control pass (R3: `control` stamp)."""
    items = []
    for c in doc["claims"]:
        if c.get("partition") != "plan-observable":
            continue
        if for_prune:
            if c.get("control") or c.get("tag") == "removed":
                continue
        else:
            if c.get("model_native") or c.get("untempted"):   # graded set only (I5)
                continue
            if c.get("tag") == "removed" and not include_removed:
                continue
        item = {"id": c["id"], "section": c["source"]["section"], "text": c["text"]}
        if c.get("discriminator"):
            item["must_name"] = c["discriminator"]
        items.append(item)
    return items


JUDGE_COST = {"usd": 0.0, "calls": 0}   # accumulated per process; written into summary.json


def judge_session_costed(prompt: str, model: str) -> str:
    """The command runner's judge call, with the session's own `total_cost_usd` kept
    (validator fix 4: the pre-registration's spend bound must be measurable)."""
    with tempfile.TemporaryDirectory(prefix="agentjudge-") as td:
        proc = subprocess.run(
            ["claude", "-p", prompt, "--model", model, "--max-turns", "1",
             "--setting-sources", "", "--output-format", "json"],
            cwd=td, capture_output=True, text=True, timeout=600)
        try:
            doc = json.loads(proc.stdout)
        except json.JSONDecodeError:
            return ""
        JUDGE_COST["usd"] += doc.get("total_cost_usd") or 0.0
        JUDGE_COST["calls"] += 1
        return doc.get("result", "") or ""


def judge_pairwise(text_a: str, text_b: str, model: str = cmdrun.PAIRWISE_MODEL) -> dict:
    """Blind A/B with position swap — the command runner's mechanics, cost-accounted and
    with the persona-target wording ("the same card for the same workspace"). A deliberate
    fork of `run.judge_pairwise`: two prompts to keep in step until the shared core lands
    (v2 D10, promotion step 2)."""
    def ask(first, second):
        prompt = ("Two action plans answer the same card for the same workspace. Which is the "
                  "better plan overall — more faithful to the task, more concretely actionable? "
                  "Reply ONLY JSON {\"winner\": \"1\"|\"2\"|\"tie\", \"reason\": "
                  "\"<one sentence>\"}.\n\nPLAN 1:\n" + first[:60_000]
                  + "\n\nPLAN 2:\n" + second[:60_000])
        return cmdrun.extract_json(judge_session_costed(prompt, model)) or {}
    v1, v2 = ask(text_a, text_b), ask(text_b, text_a)
    w1, w2 = v1.get("winner"), v2.get("winner")
    agree = (w1 == "1" and w2 == "2") or (w1 == "2" and w2 == "1") or (w1 == w2 == "tie")
    return {"first_order": v1, "swapped": v2, "position_consistent": agree}


def judge_coverage(items: list, plan: str, model: str = cmdrun.CHECKLIST_MODEL) -> list:
    out = []
    for i in range(0, len(items), cmdrun.JUDGE_CHUNK):
        chunk = items[i:i + cmdrun.JUDGE_CHUNK]
        prompt = (
            "You are grading an engineer's PLANNED action plan for one task against the "
            "professional standards that engineer holds themself to. The plan speaks in "
            "concrete actions for THIS task. Grade EMBODIMENT only: a standard is "
            "\"reflected\" when a concrete planned action enacts it; a plan that merely "
            "restates or paraphrases the standard as a principle, without a task-specific "
            "action, is \"absent\" (recitation is not embodiment); a planned action that "
            "violates the standard is \"contradicted\". Polarity: a standard from a section "
            "named 'What You Reject' describes behaviour to AVOID — it is \"reflected\" when a "
            "concrete action visibly avoids or refuses that behaviour, \"contradicted\" when "
            "the plan does it. Conditional standards (ones that apply only when a path is "
            "taken, e.g. delegating work) are \"reflected\" only if the plan actually takes "
            "that path with a concrete enacting action; declining the path, or describing it "
            "hypothetically, is \"absent\". A dispatch of a read-only explorer or fact-finder "
            "(e.g. an `Explore` subagent that only locates or enumerates) is NOT a delegation of "
            "implementation or authoring work and never satisfies a standard about handing such "
            "WORK down; a standard about handing a READ down is satisfied by exactly such a read "
            "dispatch. Where a standard carries `must_name`, it is \"reflected\" only if the "
            "planned action names every listed term (the dispatch target or model), and your "
            "evidence quote must contain them. For EACH standard return a JSON "
            "array entry {\"id\": ..., \"verdict\": \"reflected\"|\"absent\"|\"contradicted\", "
            "\"evidence\": \"<verbatim quote of the planned ACTION proving the verdict, or "
            "empty for absent>\"}. Every id exactly once. Output ONLY the JSON array.\n\n"
            "STANDARDS:\n" + json.dumps(chunk, indent=1) + "\n\nPLAN:\n" + plan[:120_000])
        byid = {}
        for _ in range(2):
            verdicts = cmdrun.extract_json(judge_session_costed(prompt, model))
            if isinstance(verdicts, list):
                for v in verdicts:
                    if isinstance(v, dict) and v.get("id") and v.get("verdict"):
                        byid.setdefault(v["id"], v)
            if all(r["id"] in byid for r in chunk):
                break
        out += [byid.get(r["id"], {"id": r["id"], "verdict": None, "evidence": "MISSING"})
                for r in chunk]
    return out


# ---------- commands ----------

def rundir(persona: str, name: str) -> pathlib.Path:
    return AGENT_EVALS / persona / "runs" / name


def cmd_grid(persona: str, replicates: int, old_ref: str | None, out: str | None,
             arms: list | None = None) -> None:
    prereg = AGENT_EVALS / persona / "preregistration.md"
    if not prereg.is_file():
        die(f"{prereg} missing — no grid without a pre-registration (D11)")
    doc = load_rules(persona)
    if old_ref and old_ref != doc["refs"]["pre"]:
        die(f"--old-ref {old_ref} differs from the minted pre ref {doc['refs']['pre']}; re-mint")
    old_ref = old_ref or doc["refs"]["pre"]
    if check(persona, old_ref):
        die("rubric check failed")
    goldens = load_goldens(persona)
    if len(goldens) < 3:
        die(f"{len(goldens)} golden(s) — D8 sets a floor of three per persona before any grid")
    arms = arms or ["pre", "post"]          # `--arms pre` = the pre-registration's probe run
    if any(a not in ("pre", "post") for a in arms):
        die(f"grid arms are pre/post only (nopersona is agent-prune): {arms}")
    name = out or datetime.datetime.now().strftime("%Y%m%d-%H%M%S")
    rd = rundir(persona, name)
    rd.mkdir(parents=True, exist_ok=True)
    runs, total = [], 0.0
    for g in goldens:
        for arm in arms:
            for r in range(1, replicates + 1):
                print(f"run {g['id']}/{arm}/r{r} ...", flush=True)
                e = plan_session(persona, g, arm, old_ref if arm == "pre" else None)
                e["replicate"] = r
                (rd / f"{g['id']}-{arm}-r{r}.plan.md").write_text(e.pop("plan"))
                runs.append(e)
                total += e.get("cost_usd") or 0.0
                if e["asserts"]["cap_hit"]:
                    print(f"  WARN cap-hit at {MAX_TURNS} turns")
                if e["asserts"]["roster_extra"]:
                    print(f"  WARN roster carries more than the fence: {e['asserts']['roster_extra']}")
                if e["asserts"]["reads_outside_ws"]:
                    print(f"  WARN reads outside the workspace: {e['asserts']['reads_outside_ws']}")
    meta = {"persona": persona, "old_ref": old_ref, "replicates": replicates, "arms": arms,
            "total_cost_usd": round(total, 4),
            "rubric_snapshot": {c["id"]: {"tag": c["tag"], "partition": c["partition"],
                                          "model_native": c.get("model_native", False),
                                          "untempted": c.get("untempted", False),
                                          "discriminator": c.get("discriminator")}
                                for c in doc["claims"]},
            "runs": runs}
    (rd / "summary.json").write_text(json.dumps(meta, indent=1))
    print(f"grid done: {rd}  (${total:.2f})")


def cmd_prune(persona: str, replicates: int, out: str | None) -> None:
    """D7 / R3: the one-time nopersona pass. Judges only plan-observable claims never yet
    judged by a control pass (the `control` stamp), and each only on the goldens that
    tempt it (a conditional claim cannot be native on a golden that never invites its
    path). Stamps every judged claim with the pass, its verdicts, and the arm model."""
    doc = load_rules(persona)
    items = judge_items(doc, for_prune=True)
    goldens = load_goldens(persona)
    tempted_by = {i["id"]: [g for g in goldens if i["id"] in g.get("tempts", [])] for i in items}
    skipped = [i["id"] for i in items if not tempted_by[i["id"]]]
    if skipped:
        print(f"skipping {len(skipped)} untempted claim(s) — no golden invites them, so no control "
              f"read is possible; tempt them or keep `untempted`: {skipped}")
    items = [i for i in items if tempted_by[i["id"]]]
    if not items:
        print("nothing to prune: every tempted plan-observable claim already carries a control stamp")
        return
    name = (out or datetime.datetime.now().strftime("%Y%m%d-%H%M%S")) + "-prune"
    rd = rundir(persona, name)
    rd.mkdir(parents=True, exist_ok=True)
    verdicts, total, runs = {i["id"]: [] for i in items}, 0.0, []
    for g in goldens:
        g_items = [i for i in items if g in tempted_by[i["id"]]]
        if not g_items:
            continue
        for r in range(1, replicates + 1):
            print(f"prune {g['id']}/nopersona/r{r} ({len(g_items)} claims) ...", flush=True)
            e = plan_session(persona, g, "nopersona", None)
            plan = e.pop("plan")
            (rd / f"{g['id']}-nopersona-r{r}.plan.md").write_text(plan)
            e["coverage"] = judge_coverage(g_items, plan)
            for v in e["coverage"]:
                verdicts[v["id"]].append(v["verdict"])
            total += e.get("cost_usd") or 0.0
            runs.append(e)
    native = sorted(i for i, vs in verdicts.items() if vs and all(v == "reflected" for v in vs))
    for c in doc["claims"]:
        if c["id"] in verdicts:
            c["control"] = {"pass": name, "arm_model": ARM_MODEL, "replicates": replicates,
                            "goldens": [g["id"] for g in tempted_by[c["id"]]],
                            "verdicts": verdicts[c["id"]]}
            c["model_native"] = c["id"] in native
    rules_path(persona).write_text(json.dumps(doc, indent=1, ensure_ascii=False) + "\n")
    (rd / "summary.json").write_text(json.dumps(
        {"persona": persona, "pass": "nopersona-prune", "arm_model": ARM_MODEL,
         "replicates": replicates, "judged": [i["id"] for i in items], "model_native": native,
         "total_cost_usd": round(total, 4), "judge_cost_usd": round(JUDGE_COST["usd"], 4),
         "judge_calls": JUDGE_COST["calls"], "runs": runs}, indent=1))
    print(f"prune done: {len(native)}/{len(items)} judged claims tagged model_native  (${total:.2f})  {rd}")


def cmd_judge(persona: str, name: str, judge_model: str = cmdrun.CHECKLIST_MODEL,
              pairwise_model: str = cmdrun.PAIRWISE_MODEL) -> None:
    rd = rundir(persona, name)
    meta = json.loads((rd / "summary.json").read_text())
    doc = load_rules(persona)
    items = judge_items(doc, include_removed=True)   # I3: removed claims are read, not silent
    meta["judge_models"] = {"coverage": judge_model, "pairwise": pairwise_model}
    plans = {}
    for e in meta["runs"]:
        key = (e["golden"], e["arm"], e["replicate"])
        plans[key] = (rd / f"{e['golden']}-{e['arm']}-r{e['replicate']}.plan.md").read_text()
        print(f"judge {key} ...", flush=True)
        e["coverage"] = judge_coverage(items, plans[key], judge_model)
    meta["pairwise"] = []
    for g in {e["golden"] for e in meta["runs"]}:
        for r in range(1, meta["replicates"] + 1):
            a, b = plans.get((g, "pre", r)), plans.get((g, "post", r))
            if a and b:
                print(f"pairwise {g}/r{r} ...", flush=True)
                meta["pairwise"].append({"golden": g, "replicate": r,
                                         **judge_pairwise(a, b, pairwise_model)})
    # Accumulate across re-judges (the noise guard prescribes one): never overwrite.
    meta["judge_cost_usd"] = round((meta.get("judge_cost_usd") or 0.0) + JUDGE_COST["usd"], 4)
    meta["judge_calls"] = (meta.get("judge_calls") or 0) + JUDGE_COST["calls"]
    (rd / "summary.json").write_text(json.dumps(meta, indent=1))
    print(f"judged: {rd / 'summary.json'}  (judges ${JUDGE_COST['usd']:.2f} over {JUDGE_COST['calls']} calls)")


def cmd_report(persona: str, name: str) -> None:
    rd = rundir(persona, name)
    meta = json.loads((rd / "summary.json").read_text())
    snap = meta["rubric_snapshot"]
    goldens = {g["id"]: g for g in load_goldens(persona)}
    # The graded set: plan-observable, not model-native, not removed, and TEMPTED by some
    # golden — an `untempted` claim is a disclosed gap, never read as absent (I5).
    graded = sorted(i for i, s in snap.items()
                    if s["partition"] == "plan-observable" and not s["model_native"]
                    and s["tag"] != "removed" and not s.get("untempted"))
    untempted = sorted(i for i, s in snap.items() if s.get("untempted"))
    discriminators = {i: s["discriminator"] for i, s in snap.items() if s.get("discriminator")}

    def term_present(text: str, term: str) -> bool:
        """Pre-registered matching convention: an all-lowercase term matches case-
        insensitively on word boundaries (`haiku` ≈ `Haiku`); a term with capitals or
        punctuation matches exactly on word boundaries (`Explore`, `mochiko:explorer`)."""
        if term == term.lower() and re.fullmatch(r"[a-z0-9-]+", term):
            return re.search(rf"(?<![A-Za-z0-9]){re.escape(term)}(?![A-Za-z0-9])", text, re.I) is not None
        return re.search(rf"(?<![A-Za-z0-9:]){re.escape(term)}(?![A-Za-z0-9])", text) is not None

    def names_all(text: str, terms: list) -> bool:
        return all(term_present(text, term) for term in terms)
    removed = sorted(i for i, s in snap.items() if s["tag"] == "removed")
    lines = [f"# Persona plan-only eval report — {persona} / {name}", "",
             f"Arms: {meta['arms']} · replicates {meta['replicates']} · pre ref {meta['old_ref']} · "
             f"cost ${meta['total_cost_usd']}", "",
             "Advisory (harness D2): nothing below sets an exit code. Read against the persona's "
             "preregistration.md — the positive control and the noise band live there.", "",
             f"Graded claims: {len(graded)} · untempted (disclosed, not read): {len(untempted)}"
             + (f" {untempted}" if untempted else ""), ""]
    for g in sorted({e["golden"] for e in meta["runs"]}):
        lines.append(f"## {g}")
        pre = [e for e in meta["runs"] if e["golden"] == g and e["arm"] == "pre"]
        post = [e for e in meta["runs"] if e["golden"] == g and e["arm"] == "post"]
        both = bool(pre) and bool(post)      # a single-arm run (the probe) reads no diff
        regressions, adoptions, ghosts, flaky_ids = [], [], [], []
        flaky_pre = [rid for rid in graded if cmdrun.flaky(pre, rid)]
        flaky_post = [rid for rid in graded if cmdrun.flaky(post, rid)]
        for rid in graded:
            if cmdrun.flaky(post, rid) or cmdrun.flaky(pre, rid):
                flaky_ids.append(rid)
            if both and snap[rid]["tag"] == "common" and cmdrun.passk(pre, rid) \
                    and not cmdrun.passk(post, rid):
                regressions.append(rid)
            if both and snap[rid]["tag"] == "added":
                adoptions.append((rid, cmdrun.passk(pre, rid), cmdrun.passk(post, rid)))
        removed_read = []
        for rid in removed:
            judged = any(v["id"] == rid for e in post for v in e.get("coverage", []))
            if not judged or not both:
                continue
            a, b = cmdrun.passk(pre, rid), cmdrun.passk(post, rid)
            removed_read.append((rid, a, b))
            if b:
                ghosts.append(rid)
        for arm_name, arm_runs in (("pre", pre), ("post", post)):
            if arm_runs:
                lines.append(f"- {arm_name} coverage (pass^k): "
                             f"{sum(1 for r in graded if cmdrun.passk(arm_runs, r))}/{len(graded)}")
        if both:
            lines.append(f"- **common-claim regressions:** {regressions or 'none'}")
        else:
            lines.append("- single-arm run: no pre/post read (band and calibration inputs only)")
        if adoptions:
            lines.append("- added-claim adoption (pre → post): " + ", ".join(
                f"{r}={'absent' if not a else 'PRESENT-IN-PRE'}→{'LANDED' if b else 'DEAD-TEXT'}"
                for r, a, b in adoptions))
        # Deterministic discriminator check (pre-registered for control claims whose texts are
        # near-identical): does every replicate's plan text name the claim's terms?
        g_tempts = set(goldens.get(g, {}).get("tempts", []))
        if discriminators:
            for rid, terms in sorted(discriminators.items()):
                if rid not in g_tempts:
                    continue          # the control runs only where the golden tempts it
                cells = []
                for arm_name, arm_runs in (("pre", pre), ("post", post)):
                    if not arm_runs:
                        continue
                    hits = [names_all((rd / f"{e['golden']}-{e['arm']}-r{e['replicate']}.plan.md").read_text(), terms)
                            for e in arm_runs]
                    cells.append(f"{arm_name} {sum(hits)}/{len(hits)}")
                lines.append(f"- discriminator {rid.split('.')[-1]} names {terms}: " + " · ".join(cells))
        tempted = goldens.get(g, {}).get("tempts", [])
        if tempted:
            def mark(r):
                parts = []
                if pre:
                    parts.append(f"pre{'✓' if cmdrun.passk(pre, r) else '✗'}")
                if post:
                    parts.append(f"post{'✓' if cmdrun.passk(post, r) else '✗'}")
                return f"{r}={'/'.join(parts)}"
            lines.append("- tempted claims (this golden's expectations): "
                         + ", ".join(mark(r) for r in tempted))
        if removed_read:
            lines.append("- removed-claim read (pre → post): " + ", ".join(
                f"{r}={'present' if a else 'absent'}→{'GHOST' if b else 'gone'}"
                for r, a, b in removed_read))
        if ghosts:
            lines.append(f"- **removed claims still surfacing:** {ghosts}")
        lines.append(f"- flaky claims (replicate disagreement — noise-guard input): {len(flaky_ids)}"
                     + (f" {flaky_ids}" if flaky_ids else ""))
        lines.append("- flaky share per arm (band input): "
                     + " · ".join(f"{a} {len(f)}/{len(graded)}"
                                  for a, f, runs_ in (("pre", flaky_pre, pre), ("post", flaky_post, post))
                                  if runs_))
        for e in pre + post:
            if e["asserts"]["cap_hit"]:
                lines.append(f"- WARN cap-hit {e['arm']}/r{e['replicate']}")
            if e["asserts"]["fence_breach"]:
                lines.append(f"- WARN fence breach {e['arm']}/r{e['replicate']}: {e['asserts']['fence_breach']}")
            if e["asserts"].get("reads_outside_ws"):
                lines.append(f"- WARN reads outside the workspace {e['arm']}/r{e['replicate']}: "
                             f"{e['asserts']['reads_outside_ws']}")
        reads = {(e["arm"], e["replicate"]): [r["target"] for r in e.get("reads", []) if r["tool"] == "Read"]
                 for e in pre + post}
        lines.append("- read-trace (Read targets per arm/replicate): "
                     + "; ".join(f"{a}/r{r}: {len(t)}" for (a, r), t in sorted(reads.items())))
        lines.append("")
    for p in meta.get("pairwise", []):
        lines.append(f"- pairwise {p['golden']}/r{p['replicate']}: "
                     f"{(p['first_order'].get('winner'), p['swapped'].get('winner'))} "
                     f"(position_consistent={p['position_consistent']})")
    if meta.get("judge_cost_usd") is not None:
        lines.append(f"- judge spend: ${meta['judge_cost_usd']} over {meta.get('judge_calls')} calls "
                     f"(plan sessions ${meta['total_cost_usd']})")
    (rd / "report.md").write_text("\n".join(lines) + "\n")
    print(f"report: {rd / 'report.md'}")


# ---------- judge calibration (D11 fold I9) ----------

def cmd_label_sheet(persona: str, name: str, size: int, seed: int) -> None:
    """Emit a hand-labelling sheet: `size` (golden, arm, replicate, claim) pairs sampled
    from a judged run, judge verdicts hidden, for the lead to label. The filled sheet
    feeds `agent-calibrate`."""
    import random
    rd = rundir(persona, name)
    meta = json.loads((rd / "summary.json").read_text())
    pairs = [{"golden": e["golden"], "arm": e["arm"], "replicate": e["replicate"],
              "id": v["id"], "label": ""}
             for e in meta["runs"] for v in e.get("coverage", []) if v.get("verdict")]
    if not pairs:
        die("run carries no judged coverage — run agent-judge first")
    random.Random(seed).shuffle(pairs)
    pairs = pairs[:size]
    # Arm-blind (validator fix 12): the labeller sees an opaque plan key and an anonymized
    # copy of the plan, never the arm; the key → (golden, arm, replicate) map is written to a
    # separate file the labeller must not open, and agent-calibrate joins through it.
    cal = rd / "calibration"
    cal.mkdir(exist_ok=True)
    keys, mapping = {}, {}
    for pr in pairs:
        src = (pr["golden"], pr["arm"], pr["replicate"])
        if src not in keys:
            k = hashlib.sha256(f"{seed}:{src}".encode()).hexdigest()[:8]
            keys[src] = k
            mapping[k] = {"golden": pr["golden"], "arm": pr["arm"], "replicate": pr["replicate"]}
            shutil.copy(rd / f"{pr['golden']}-{pr['arm']}-r{pr['replicate']}.plan.md",
                        cal / f"plan-{k}.md")
        pr["plan"] = keys[src]
        for f in ("golden", "arm", "replicate"):
            pr.pop(f)
    sheet = {"persona": persona, "run": name, "seed": seed,
             "instructions": "Label each pair reflected | absent | contradicted by reading "
                             "calibration/plan-<plan>.md against the claim text in rules.json: a "
                             "restated principle without a task-specific action = absent; a "
                             "declined or hypothetical conditional path = absent; a Reject-section "
                             "standard is reflected when a concrete action avoids the behaviour. Do "
                             "not open calibration-map.json and do not look at the judge's verdicts.",
             "pairs": pairs}
    out = rd / "calibration-sheet.json"
    out.write_text(json.dumps(sheet, indent=1))
    (rd / "calibration-map.json").write_text(json.dumps(mapping, indent=1))
    print(f"sheet: {out}  ({len(pairs)} pairs to label, {len(mapping)} anonymized plans in {cal})")


def cmd_calibrate(persona: str, name: str, labels_path: str) -> None:
    """Agreement between the hand labels and the judge's verdicts on the same pairs;
    writes calibration.json beside the run. The bars live in preregistration.md."""
    rd = rundir(persona, name)
    meta = json.loads((rd / "summary.json").read_text())
    sheet = json.loads(pathlib.Path(labels_path).read_text())
    judged = {(e["golden"], e["arm"], e["replicate"], v["id"]): v["verdict"]
              for e in meta["runs"] for v in e.get("coverage", [])}
    mapping = json.loads((rd / "calibration-map.json").read_text())
    rows, agree, contra_total, contra_agree = [], 0, 0, 0
    confusion = {}
    for pr in sheet["pairs"]:
        lab = (pr.get("label") or "").strip()
        if lab not in ("reflected", "absent", "contradicted"):
            die(f"unlabelled or invalid pair {pr}")
        src = mapping.get(pr["plan"])
        if src is None:
            die(f"sheet row names plan key {pr['plan']!r} absent from calibration-map.json — "
                "the sheet and map were generated with different seeds; regenerate both together")
        pr = {**pr, **src}
        jv = judged.get((src["golden"], src["arm"], src["replicate"], pr["id"]))
        rows.append({**pr, "judge": jv, "agree": jv == lab})
        agree += jv == lab
        confusion[(lab, jv)] = confusion.get((lab, jv), 0) + 1
        if lab == "contradicted":
            contra_total += 1
            contra_agree += jv == lab
    n = len(rows)
    result = {"persona": persona, "run": name, "pairs": n,
              "agreement": round(agree / n, 3) if n else None,
              "contradicted_pairs": contra_total,
              "contradicted_agreement": (round(contra_agree / contra_total, 3)
                                         if contra_total else None),
              "confusion": {f"label={a} judge={b}": c for (a, b), c in sorted(confusion.items(),
                                                                                key=str)},
              "rows": rows}
    (rd / "calibration.json").write_text(json.dumps(result, indent=1))
    print(f"calibration: agreement {result['agreement']} over {n} pairs · contradicted "
          f"{contra_agree}/{contra_total} · {rd / 'calibration.json'}")


def add_subcommands(sub) -> None:
    for name in ("agent-mint", "agent-check", "agent-plan-run", "agent-grid",
                 "agent-prune", "agent-judge", "agent-report", "agent-label-sheet",
                 "agent-calibrate"):
        p = sub.add_parser(name)
        p.add_argument("persona")
        if name == "agent-mint":
            p.add_argument("--old-ref", required=True)
        if name == "agent-check":
            p.add_argument("--old-ref")
        if name == "agent-plan-run":
            p.add_argument("golden")
            p.add_argument("--arm", default="post", choices=ARMS)
            p.add_argument("--old-ref")
            p.add_argument("--out")
        if name in ("agent-grid", "agent-prune"):
            p.add_argument("--replicates", type=int, default=3)
            p.add_argument("--out")
        if name == "agent-grid":
            p.add_argument("--old-ref")
            p.add_argument("--arms", default="pre,post",
                           help="comma list; `pre` alone is the pre-registration probe run")
        if name in ("agent-judge", "agent-report", "agent-label-sheet", "agent-calibrate"):
            p.add_argument("run_name")
        if name == "agent-label-sheet":
            p.add_argument("--size", type=int, default=24)
            p.add_argument("--seed", type=int, default=7)
        if name == "agent-calibrate":
            p.add_argument("--labels", required=True)
        if name == "agent-judge":
            p.add_argument("--judge-model", default=cmdrun.CHECKLIST_MODEL)
            p.add_argument("--pairwise-model", default=cmdrun.PAIRWISE_MODEL)


def dispatch(a) -> bool:
    """Handle an agent-* subcommand; False when `a.cmd` is not one of ours."""
    if not a.cmd.startswith("agent-"):
        return False
    if a.cmd == "agent-mint":
        doc = mint(a.persona, a.old_ref)
        drafts = sum(1 for c in doc["claims"] if c.get("draft"))
        print(f"minted {len(doc['claims'])} claims ({drafts} drafts to partition) → "
              f"{rules_path(a.persona)}  units pre/post/union "
              f"{doc['unit_counts']['pre']}/{doc['unit_counts']['post']}/{doc['unit_counts']['union']}")
    elif a.cmd == "agent-check":
        sys.exit(1 if check(a.persona, a.old_ref) else 0)
    elif a.cmd == "agent-plan-run":
        goldens = {g["id"]: g for g in load_goldens(a.persona)}
        if a.golden not in goldens:
            die(f"unknown golden {a.golden}; have {sorted(goldens)}")
        old_ref = a.old_ref or (load_rules(a.persona)["refs"]["pre"]
                                if rules_path(a.persona).is_file() else None)
        if a.arm == "pre" and not old_ref:
            die("--arm pre needs --old-ref (or a minted rules.json)")
        e = plan_session(a.persona, goldens[a.golden], a.arm, old_ref if a.arm == "pre" else None)
        out = pathlib.Path(a.out) if a.out else rundir(a.persona, "adhoc")
        out.mkdir(parents=True, exist_ok=True)
        (out / f"{a.golden}-{a.arm}.plan.md").write_text(e.pop("plan"))
        (out / f"{a.golden}-{a.arm}.meta.json").write_text(json.dumps(e, indent=1))
        print(json.dumps(e["asserts"], indent=1))
        print(f"reads: {[r['target'] for r in e['reads']]}")
        print(f"saved: {out}  (${e.get('cost_usd')}, {e.get('num_turns')} turns)")
    elif a.cmd == "agent-grid":
        cmd_grid(a.persona, a.replicates, a.old_ref, a.out,
                 [x.strip() for x in a.arms.split(",") if x.strip()])
    elif a.cmd == "agent-prune":
        cmd_prune(a.persona, a.replicates, a.out)
    elif a.cmd == "agent-judge":
        cmd_judge(a.persona, a.run_name, a.judge_model, a.pairwise_model)
    elif a.cmd == "agent-report":
        cmd_report(a.persona, a.run_name)
    elif a.cmd == "agent-label-sheet":
        cmd_label_sheet(a.persona, a.run_name, a.size, a.seed)
    elif a.cmd == "agent-calibrate":
        cmd_calibrate(a.persona, a.run_name, a.labels)
    return True
