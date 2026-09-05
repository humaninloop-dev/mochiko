#!/usr/bin/env python3
"""The plugin contract suite — the layer that tests what the crate never can.

Provenance: `.mochiko/brainstorms/cli-schema-delivery/record.md` D8 as amended. The suite runs
inside the Docker AI sandbox `claude-mochiko`, through the sandbox helpers `evals/run.py` already
owns; it imports them and never forks them. Maintainer-side, never shipped (GI-020).

What it asserts (D8's deterministic set):

    the `!` line executed · the version-triple line present · the closing end line present ·
    no schema file read anywhere · absence halts · skew halts

The fourth of those went **run-wide at wave 6**, when the last schema file left the plugin. It was
scoped per converted primitive through waves 3 to 5, because a converted command invoking an
unconverted skill legitimately read that skill's file; now nothing may read one, so the assertion
sweeps every JSONL channel each case captured — streams, session transcripts, and the sidechain
transcripts a subagent's turns land in. Its host-side half lives in `render-ceiling`: no rendered
rule may still name a shipped schema file for a model to go and read.

Wave 1 ran two cases, both failure paths against the fixture plugin, because those are the ones
that do not need a converted primitive. Wave 3 added the first converted command, `brainstorm`,
and with it the cases that need one. Wave 4 converts the remaining five and generalizes those
cases into a per-command family:

    hook-input          the two hook scripts, fed real captured stdin, on the host
    converted-shape     a converted `.md`'s `!` lines against the sections its render declares
    render-ceiling      every converted primitive's renders against the inline ceiling, and
                        against the two phrases no rendered rule may still carry
    deliverables        every template, shelf and registry through its CLI form
    absence   [fixture] the binary is off the sandbox PATH -> the run halts, nothing delivered
    skew      [fixture] the log declares a grammar the binary does not read -> the D5 halt
    <cmd>-delivery      the happy path, per converted command: every block its render declares,
                        delivered, plus the read-back metric and the delivered read cost
    <cmd>-absence       the same halt, per converted command, with the plugin's own hooks in play
    brainstorm-skew     the staged plugin's own log is out of range
    brainstorm-hooks-off  hooks disabled: the harness path is the only guard left
    brainstorm-policy   shell execution disabled by policy — recorded, never asserted (D8)

The delivery and absence cases are built from the converted set discovered in the plugin's own
`.md` files, so the case list grows with the conversion waves rather than being written down. The
last three are *mechanism* cases and run against the pilot only: they exercise what happens when
the log, the hooks or the shell is broken, which does not vary with which command fired.

The first four cases need neither a sandbox nor a session and run on the host binary;
`--host-only` runs just those, which is the cheapest gate on the hooks and on delivery shape.

A positive assertion reads only the channels measured to carry delivered text — the session
transcript and the stream's own events. Negative assertions read the wider union that adds the
process streams, because for those breadth is strictness.

**A suite that cannot run says so.** Every prerequisite is checked before any session case, and a
missing one exits 3 with the reason. Exit 0 means every declared case ran and passed — never
"nothing happened". A failed assertion outranks a skip: if a host case fails and the sandbox is
then unreachable, the exit is 1, not 3.

    exit 0  every case ran and passed
    exit 1  a case ran and an assertion failed
    exit 3  the suite could not run (no sandbox, not authenticated, no binary) — SKIPPED

The read-back metric is reported and never gates (D8): it lands in the case's `verdict.json` and
in the summary, and it cannot set a non-zero exit code.

Usage:
    python3 evals/contract/run.py              # run the cases
    python3 evals/contract/run.py --list       # print the case list and exit
    python3 evals/contract/run.py --host-only  # only the cases that need no sandbox
"""

import argparse
import importlib.util
import json
import pathlib
import re
import shlex
import shutil
import subprocess
import sys
import time
import uuid
from typing import NamedTuple

# When this run began, and every directory it staged. Both exist for one check — that the frozen
# skill expectations predate the run they grade — and neither is read by anything else.
RUN_STARTED = time.time()
STAGED_ROOTS: list = []


def _stamp(when: float) -> str:
    return time.strftime("%Y-%m-%d %H:%M:%S", time.localtime(when))

REPO = pathlib.Path(__file__).resolve().parents[2]
CONTRACT = REPO / "evals" / "contract"
FIXTURE = CONTRACT / "fixture" / "probe-plugin"
HOOK_INPUT = CONTRACT / "fixture" / "hook-input"
PLUGIN = REPO / "plugins" / "mochiko"
HOST_BINARY = REPO / "target" / "release" / "mochiko-cli"
WORK = REPO / "evals" / ".work"

EXIT_OK, EXIT_ASSERT, EXIT_SKIP = 0, 1, 3

# The version triple's shape (record D3 as amended): head line, then the body, then the end line.
TRIPLE_HEAD = "mochiko-cli rules "
TRIPLE_MARKERS = ("· binary ", "· grammar ", "· plugin ")
END_LINE = "mochiko-cli rules end"

# `mochiko-cli doc` (record D9 wave 6) wraps a non-rule document the same way `rules` wraps a
# section — same three markers, no `section` field.
DOC_HEAD = "mochiko-cli doc "
DOC_END = "mochiko-cli doc end · "

# `mochiko-cli template` does not, and that is deliberate rather than an omission: its output is
# read as a document, so it opens on the document's own `# Title` and closes on this footer. The
# wave-6 lead ruling keeps that shape and books the wrapping as a follow-up, so the `deliverables`
# case holds each command to what it emits rather than to a shape neither yet shares.
TEMPLATE_FOOTER = "schemas: replayed from "

# What `template` and `doc` are expected to serve, written down beside the discovery that walks
# them. Discovery alone would shrink silently if a document vanished from the log; a written-down
# set alone would go stale when one is added. Compared in both directions, neither can.
TEMPLATE_NAMES = (
    "architecture-store",
    "codebase-analysis",
    "feature-entry",
    "features-index",
    "governance-intent",
    "governance-surfaces",
    "spec",
    "tasks",
)
DOC_NAMES = ("architecture-shelf-backend", "command-labels", "skill-labels")

# What the fixture command prints, so the model's own verdict is readable in the transcript.
PROBE_DELIVERED = "CONTRACT-PROBE: delivered"
PROBE_HALTED = "CONTRACT-PROBE: halted"

# The sandbox's own build tree. NEVER the repository's `target/`: the sandbox mounts the worktree
# at the same path the host uses, so a shared target directory means the Linux sandbox executes
# the host's macOS Mach-O binary and reports `sh: Syntax error: "(" unexpected`.
SANDBOX_TARGET_DIR = "/home/agent/mochiko-target"

# `mochiko-cli --version`, which is also the head of the version triple.
VERSION_LINE = re.compile(r"^mochiko-cli (\d+\.\d+\.\d+) · grammar (\d+)\.\.(\d+)$")

# The wave-3 pilot. It is still the one command the mechanism cases run against (skew, hooks-off
# and policy exercise the delivery mechanism rather than a primitive, so they are not repeated per
# command — wave-4 plan §4), and it is one ordinary row of the expectation table below.
PILOT_COMMAND = "brainstorm"


class Expected(NamedTuple):
    """One command's pre-registered expectations. Written down, never derived.

    `floor_ids` is the read-back bar's subject and `baseline_bytes` is abort criterion (2)'s
    comparand. Both are fixed before any run of the wave that uses them: a bar read off the thing
    it grades is not a bar. What keeps them honest is a cross-check rather than a derivation —
    `converted-shape` compares every set here against the `class: floor` ids the binary actually
    renders and goes red on any difference, so a floor rule added or renamed at a later wave
    breaks a check instead of quietly regrading.

    `baseline_bytes` is `wc -c` of `<cmd>.yaml` plus `common.yaml` — the read the `.md` obligated
    before conversion. Comparisons are bytes to bytes; chars are reported beside them and are
    never the criterion.
    """

    floor_ids: frozenset
    baseline_bytes: int


# Wave 3 pre-registered `brainstorm` (plan §0, at plugin v0.103.0); wave 4 pre-registered the
# other five (wave-4 plan §4, measured 2026-09-04 against binary 0.1.0 before the wave's own
# migration landed). The baselines are deliberately the pre-migration figures: the wave's reworded
# `fail-conditions` intent moves the raw baseline and the delivered figure in the same direction
# by a few bytes, so a constant that moved after the fact would flatter the comparison.
EXPECTED = {
    "architecture": Expected(
        frozenset(
            {
                "arch.dm-health-first",
                "arch.dm-converge-goal",
                "arch.dm-author-baseline",
                "arch.dm-shelf-walk",
                "arch.dm-drift-dispatch",
                "arch.dm-route-triggers",
                "arch.dm-store-integrity-close",
                "arch.dm-km-landing",
                "arch.dm-close-verdict",
                "arch.author-grader-separation",
                "arch.truth-user-ruling",
                "arch.breadth-invariant",
                "arch.floor-precedence",
                "arch.na-handled-elsewhere-pointer",
                "arch.derived-index-never-hand-maintained",
                "arch.drift-empirical",
                "arch.no-depth-dial-coupling",
                "arch.no-delivery-harness",
                "arch.no-silent-store-mutations",
                "arch.sound-loop-floor",
                "arch.transport-floor",
                "arch.fail.no-verdict",
            }
        ),
        23_026,
    ),
    "brainstorm": Expected(
        frozenset(
            {
                "brainstorm.user-record-acceptance",
                "brainstorm.author-grader-default-fail",
                "brainstorm.transport-floor",
                "brainstorm.fail.record-unaccepted",
                "brainstorm.fail.unreviewed-no-waiver",
                "brainstorm.fail.survivor-undispositioned",
                "brainstorm.fail.index-mismatch",
            }
        ),
        12_819,
    ),
    "feature": Expected(
        frozenset(
            {
                "feat.capability-writes-sacred",
                "feat.grooming-door-ceiling",
                "feat.out-of-remit-hosting",
                "feat.growth-door",
                "feat.growth-routes-to-specify",
                "feat.lane-never-widens",
                "feat.no-delivery-harness",
                "feat.no-self-graded-writes",
                "feat.no-silent-map-mutations",
                "feat.sound-loop-floor",
                "feat.transport-floor",
                "feat.stub-parking",
                "feat.fail.no-verdict",
            }
        ),
        21_020,
    ),
    "implement": Expected(
        frozenset(
            {
                "impl.gate-design-checkpoint",
                "impl.gate-card-confirm",
                "impl.gate-final-acceptance",
                "impl.graded-fold",
                "impl.author-grader-default-fail",
                "impl.baselines-never-in-place",
                "impl.deviation-gate",
                "impl.constitution-supremacy",
                "impl.constraint-challenge",
                "impl.attempt-per-grade",
                "impl.attempt-exemption-user-only",
                "impl.no-progress-stop",
                "impl.epic-member-halt",
                "impl.gap-rework-bound",
                "impl.gates-never-triaged",
                "impl.minimalism-advisory",
                "impl.lane-never-widens",
                "impl.sound-loop-floor",
                "impl.transport-floor",
                "impl.fail.sufficiency-unrecorded",
                "impl.fail.design-skipped",
                "impl.fail.card-independence",
                "impl.fail.card-unchecked",
                "impl.fail.quality-gate",
                "impl.fail.no-evidence",
                "impl.fail.regression",
                "impl.fail.baseline-in-place",
                "impl.fail.deviation-unresolved",
                "impl.fail.store-landing-incomplete",
                "impl.fail.ungraded-fold",
                "impl.fail.gap-finding-missing",
                "impl.fail.skip-unstated",
                "impl.fail.spec-gap-unresolved",
                "impl.fail.no-acceptance",
            }
        ),
        44_266,
    ),
    "setup": Expected(
        frozenset(
            {
                "setup.blind-map-dispatch",
                "setup.gate-synthesis-ratification",
                "setup.gate-final-acceptance",
                "setup.author-grader-default-fail",
                "setup.no-git-mutations",
                "setup.acceptance-plain-text",
                "setup.transport-floor",
                "setup.durables-never-deleted",
                "setup.governance-region-ownership",
                "setup.carve-outs-preserved",
                "setup.map-never-overwrite",
                "setup.store-ruled-content-never-here",
                "setup.fail.pre-ratification-authoring",
                "setup.fail.unclosed-trace",
                "setup.fail.author-graded",
                "setup.fail.floor-category-uncovered",
                "setup.fail.no-acceptance",
                "setup.fail.no-feature-map",
            }
        ),
        20_245,
    ),
    "specify": Expected(
        frozenset(
            {
                "spec.pm-recommends-never-selects",
                "spec.gate-selection",
                "spec.gate-acceptance",
                "spec.author-grader-default-fail",
                "spec.transport-floor",
                "spec.staged-derivation",
                "spec.epic-mint-desk-only",
                "spec.fail.blocking-gap",
                "spec.fail.intent-unconfirmed",
                "spec.fail.map-unread",
                "spec.fail.story-unhomed",
                "spec.fail.screens-flows",
                "spec.fail.selection-unruled",
                "spec.fail.premature-map-write",
                "spec.fail.self-graded",
                "spec.fail.no-acceptance",
            }
        ),
        23_434,
    ),
}

# The pilot's own row, under the name the wave-3 suite and its audit use for it.
FLOOR_IDS = EXPECTED[PILOT_COMMAND].floor_ids

# Wave 5 pre-registered the thirty schema-bearing skills the same way, but out of line: thirty
# rows of floor ids run to a few hundred entries, so they live in `expected-skills.json` beside
# this file rather than in it. The file is written from the render and the working tree **before
# any skill is converted and before the wave's first session**, and is never edited afterwards;
# `converted-shape` reports its mtime against the run's own evidence directories so the ordering
# is checkable rather than asserted in prose. Its rows carry two figures this file's `Expected`
# has no room for — the family, and the pre-conversion `SKILL.md` size — because after the
# conversion lands `HEAD` no longer carries the latter.
EXPECTED_SKILLS_FILE = CONTRACT / "expected-skills.json"


def load_expected_skills() -> tuple[dict, dict]:
    """The frozen skill rows, as `Expected` records and as their full selves.

    A missing file is not an error here: the suite has to remain runnable on a tree where the
    freeze has not happened yet, and the cases that need a row say so loudly when they run.
    """
    if not EXPECTED_SKILLS_FILE.is_file():
        return {}, {}
    payload = json.loads(EXPECTED_SKILLS_FILE.read_text(encoding="utf-8"))
    rows = payload.get("skills") or {}
    expected = {
        name: Expected(frozenset(row["floor_ids"]), row["baseline_bytes"])
        for name, row in rows.items()
    }
    return expected, payload


EXPECTED_SKILLS, SKILL_FREEZE = load_expected_skills()


def expectation(name: str) -> Expected | None:
    """One primitive's pre-registered row, whichever table holds it."""
    return EXPECTED.get(name) or EXPECTED_SKILLS.get(name)


def family_of(skill: str) -> str | None:
    return ((SKILL_FREEZE.get("skills") or {}).get(skill) or {}).get("family")

# The read-back metric (D8): N replicates, the bar pre-registered before the first run and
# unchanged after it. One bar for every command, `implement`'s thirty-four ids included — the same
# pre-registration as the pilot. Reported, never gating.
READ_BACK_REPLICATES = 3
READ_BACK_BAR = 3

# The probe topic, in the two-line form the wave-4 diagnostic pre-registered and the wave-5
# re-key adopts for every delivery case. Two things changed from the one-line wave-4 probe, both
# for reasons that run measured this:
#
# 1. **A gate-valid argument comes first.** At wave 4 the instruction text *was* the whole
#    argument, and a model following its primitive's own Entry protocol sometimes refused it as
#    injected text — which is the behaviour the primitives are written to produce. `$ARGUMENTS`
#    takes everything after the name, so the instruction cannot be moved out of the argument in a
#    headless run; what can be done is to put a token the primitive accepts at the front.
# 2. **Two lines, not one.** A model that knows how many floor rules it holds but cannot list
#    them is failing enumeration, not delivery, and one line cannot show that.
#
# Neither line gates anything: criterion (1) is `assert_floor_delivery`, which reads the delivered
# blocks in the transcript rather than the model's account of them.
PROBE_INSTRUCTION = (
    "CONTRACT PROBE — before your first question, and before anything else, reply with exactly "
    "these two lines and then stop:\n"
    "FLOOR-COUNT: <how many class: floor rules you were delivered, as a number>\n"
    "FLOOR: <every class: floor rule id you were delivered, comma-separated>"
)

# The gate-valid argument per primitive, and why its own entry accepts it. Every row was written
# by reading that primitive's opening and first procedural step before the wave's first session —
# the procedure `diagnostic.py` used for the six commands, whose rows are carried here verbatim
# with their justifications. (Carried rather than imported: the diagnostic imports this file, so
# the dependency may not run the other way, and it is a frozen wave-4 artifact rather than a live
# input. The duplication is deliberate and disclosed in the README.)
#
# Two argument shapes appear. A primitive whose entry takes free text gets a one-word subject. A
# primitive whose entry takes an artifact gets a path that does not exist, so its own missing-input
# or routing branch runs instead of the argument becoming the thing under test.
PROBE_ARGUMENTS = {
    # --- commands, from `diagnostic.py`'s pre-registered table -------------------------------
    "architecture": (
        "caching",
        "Entry: `$ARGUMENTS` = the incoming architecture demand or store query. Free text; a "
        "one-word demand is accepted and the health view is surfaced either way.",
    ),
    "brainstorm": (
        "caching",
        "Entry: `$ARGUMENTS` = the topic. Free text; only an empty topic is sent back.",
    ),
    "feature": (
        "caching",
        "Entry: `$ARGUMENTS` = the incoming demand or map query. Free text; health is surfaced "
        "before the request is taken either way.",
    ),
    "implement": (
        ".mochiko/features/FEAT-000/cards/CARD-000.md",
        "Entry gates on a capability entry: a spec's accepted selection, or a desk-confirmed "
        "delta card. A delta-card path that does not exist is neither, so Entry takes its own "
        "routing branch rather than validating a FEAT id or proposing one.",
    ),
    "setup": (
        "caching",
        "Entry: `$ARGUMENTS` = an optional setup request, and empty is explicitly fine, so any "
        "free-text request passes.",
    ),
    "specify": (
        "caching",
        "Entry: `$ARGUMENTS` = the feature description. Free text; only an empty one is sent back.",
    ),
    # --- review family: each grades an artifact the caller supplies --------------------------
    "review-brainstorm": (
        ".mochiko/brainstorms/probe/record.md",
        "Cold reviewer of a frozen `record.md`; the argument is that record. A path that does "
        "not exist takes the skill's own missing-input branch, after the read-back.",
    ),
    "review-code-minimalism": (
        ".mochiko/specs/probe/cycle-report.md",
        "Reads a cycle's diff and its `cycle-report.md`; the argument names the report.",
    ),
    "review-feasibility": (
        ".mochiko/specs/probe/",
        "Grades a design-phase artifact package for cross-artifact feasibility; the argument is "
        "that package's directory.",
    ),
    "review-governance-intent": (
        ".mochiko/memory/governance-intent.md",
        "Cold reviewer of the frozen `governance-intent.md`; the argument is that file, and the "
        "path is the real one so the seat's own scoping step is what runs.",
    ),
    "review-plan-artifacts": (
        ".mochiko/specs/probe/",
        "Completeness grader over the design-phase output package the caller supplies.",
    ),
    "review-specifications": (
        ".mochiko/specs/probe/spec.md",
        "Gap-finder over an already-drafted spec; the argument is that spec.",
    ),
    "review-sufficiency": (
        ".mochiko/features/FEAT-000/",
        "Grades one unit of selected work — the map's own unit of scope — so the argument is a "
        "capability entry directory.",
    ),
    "validation-constitution": (
        ".claude/rules/mochiko/",
        "Grades a drafted governance surface set; the argument names the rules directory that "
        "set lands in.",
    ),
    # --- authoring family: a subject to author for --------------------------------------------
    "authoring-architecture-store": ("caching", "Authors a store write for a subject; free text."),
    "authoring-constitution": (
        ".mochiko/memory/governance-intent.md",
        "Authors the surface set from a ratified synthesis; the argument is that synthesis.",
    ),
    "authoring-epic": ("caching", "Authors an epic for a related batch; free text names it."),
    "authoring-feature-map": ("caching", "Authors a map delta for a capability; free text."),
    "authoring-prototype": ("caching", "Authors a prototype for a feature; free text."),
    "authoring-requirements": ("caching", "Authors the FR/SC layer for a feature; free text."),
    "authoring-technical-requirements": (
        "caching",
        "Authors the constraint layer for a feature; free text.",
    ),
    "authoring-user-stories": ("caching", "Transforms a feature description; free text."),
    # --- patterns family: a technique applied to a subject -------------------------------------
    "patterns-adopt-first": ("caching", "Runs the build-vs-buy ladder over a subject; free text."),
    "patterns-architecture-shelves": ("caching", "Deals a shelf for a surface; free text."),
    "patterns-code-minimalism": ("caching", "Runs the pre-code ladder over a task; free text."),
    "patterns-map-minimalism": ("caching", "Runs the capability tests over a candidate; free text."),
    "patterns-model-tiering": ("caching", "Routes a read by its class key; free text."),
    "patterns-plan-minimalism": ("caching", "Runs the design ladder over an element; free text."),
    "patterns-sound-loop": ("caching", "Wires the floor for a pending write; free text."),
    "patterns-transport-floor": ("caching", "Applies the floor to a composition; free text."),
    "patterns-vertical-tdd": ("caching", "Structures a feature into cycle cards; free text."),
    # --- the dense five -------------------------------------------------------------------------
    "analysis-codebase": (
        ".",
        "Analyses an existing codebase; the argument is the repository root, which exists, so "
        "the skill's own detection step is what runs.",
    ),
    "brownfield-integration": (
        ".mochiko/specs/probe/cards/CARD-000.md",
        "Implements one card that touches existing code; the argument names that card.",
    ),
    "executing-tdd-cycle": (
        ".mochiko/specs/probe/cards/CARD-000.md",
        "Executes one cycle card at runtime; the argument names that card.",
    ),
    "testing-end-user": (
        ".mochiko/specs/probe/tasks.md",
        "Executes a `**TEST:**` task; the argument names the file carrying it.",
    ),
    "testing-gap-finding": (
        "FEAT-000",
        "Runs the blind pass over a selection; the argument is a capability id, its own unit.",
    ),
}


def probe_prompt(name: str) -> str:
    """The prompt one delivery replicate runs: the primitive, its argument, the instruction."""
    argument = PROBE_ARGUMENTS[name][0]
    return f"/mochiko:{name} {argument}\n\n{PROBE_INSTRUCTION}"

# The pilot's baseline, under the names the wave-3 README and verdicts use. The two figures beside
# it are `brainstorm`-only: `wc -m` of the same pair, and `wc -c` counting `command-labels.yaml`.
BASELINE_BYTES = EXPECTED[PILOT_COMMAND].baseline_bytes
BASELINE_CHARS = 12_753
BASELINE_BYTES_WITH_LABELS = 14_349

# Wave-0 probe (e): a `!` block above this arrives as a file-path notice whose preview keeps only
# the first line, which would strand a render without its end line.
INLINE_CEILING = 30_000

# The two-arm wording, and the path it offered. Every rule that carried either was reworded by
# migration `0003` (wave 6, record D9), and no shipped schema file survives for one to point at,
# so a render still carrying one is a rule telling the model to read a file that is not there.
# Checked against the renders in `render-ceiling`; never against the log, which is append-only and
# keeps the original wording in `0001-genesis.yaml` by construction.
DEAD_PHRASES = ("when the binary is absent", "plugins/mochiko/schemas/")

# What marks a primitive as converted. The same test the dependency hook makes, and for the same
# reason: the primitive's own file is the truth, so there is no list to keep in sync with the
# conversion waves.
CONVERTED_MARK = "!`mochiko-cli rules"

# The dependency hook's presence line, keyed to its stable prefix. The trailing noun is `command`
# or `skill` depending on the registration, and `hook-input` asserts the full per-noun line.
HOOK_PRESENT_PREFIX = "mochiko-cli present · rules delivered by the"
INSTALL_LINE = "cargo install mochiko-cli"


class Check(NamedTuple):
    """One assertion's outcome.

    `pending` is a first-class status, not a quiet pass. An assertion whose subject does not exist
    until a later wave is reported as pending every run, so the case summary can never read as
    though it were asserted.
    """

    name: str
    status: str  # "ok" | "fail" | "pending" | "report"
    detail: str = ""


def ok(name: str, problem: str | None) -> Check:
    return Check(name, "ok" if problem is None else "fail", problem or "")


def pending(name: str, why: str) -> Check:
    return Check(name, "pending", why)


def report(name: str, what: str) -> Check:
    """A measured outcome that D8 records rather than asserts.

    The policy case has no gating assertion at all, and a case whose check list is empty prints as
    a clean pass — a suite reporting success for having asserted nothing. A recorded observation
    is a first-class status for the same reason `pending` is: it can never be mistaken for one.
    """
    return Check(name, "report", what)


def load_runner():
    """Import `evals/run.py` — the sandbox helpers, never a second copy of them."""
    path = REPO / "evals" / "run.py"
    spec = importlib.util.spec_from_file_location("mochiko_eval_runner", path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"{path} cannot be imported")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


# ---------------------------------------------------------------------------
# preflight
# ---------------------------------------------------------------------------

def preflight(runner) -> str | None:
    """The reason the suite cannot run, or None."""
    if shutil.which("sbx") is None:
        return "the `sbx` CLI is not on PATH — the sandbox is the suite's substrate (D8)"

    reach = subprocess.run(
        ["sbx", "exec", runner.SANDBOX, "true"],
        capture_output=True,
        text=True,
        timeout=120,
    )
    if reach.returncode != 0:
        return (
            f"the sandbox {runner.SANDBOX!r} is not reachable: "
            f"{(reach.stderr or reach.stdout).strip()[:200]}"
        )

    version = runner.sbx_sh("claude --version", timeout=120)
    if version.returncode != 0:
        return f"`claude` is not runnable in the sandbox: {version.stderr.strip()[:200]}"

    # Authentication is the one that bites silently: an unauthenticated sandbox still starts a
    # session and then returns an error result, which a careless suite reads as a failed
    # assertion rather than as "it never ran". Probe it once, cheaply.
    probe = runner.sbx_sh(
        "cd /tmp && claude -p 'reply with the single word READY' "
        "--model haiku --max-turns 1 --setting-sources '' --output-format json",
        timeout=300,
    )
    if probe.returncode != 0 or not probe.stdout.strip():
        return (
            "the sandbox is not authenticated for headless runs — run `sbx login` "
            f"(claude exited {probe.returncode}: {(probe.stderr or probe.stdout).strip()[:200]})"
        )
    try:
        result = json.loads(probe.stdout[probe.stdout.index("{"):])
    except (ValueError, json.JSONDecodeError):
        return f"the sandbox returned no JSON result: {probe.stdout.strip()[:200]}"
    if result.get("is_error") or result.get("subtype") not in (None, "success"):
        return f"the sandbox session errored: {str(result)[:200]}"

    if not FIXTURE.is_dir():
        return f"the fixture plugin is missing at {FIXTURE}"
    return None


class Sandbox(NamedTuple):
    """What the cases need to know about the substrate they run on."""

    path: str  # the sandbox's own PATH, carrying `claude` and NOT `mochiko-cli`
    binary: str  # the built binary's absolute path in the sandbox
    binary_dir: str  # the directory holding it, for prepending to PATH


def build_binary(runner) -> tuple[str | None, str | None]:
    """Build `mochiko-cli` inside the sandbox and return its path there, or a skip reason.

    Built in the sandbox rather than copied in: the sandbox is Linux and the maintainer's host is
    macOS, so a host build is the wrong architecture. This is the D4 install shape in miniature —
    the binary arrives as a developer tool on PATH, not as part of the plugin.

    Two things this gets right that a first cut did not:

    * **A sandbox-local target directory.** The sandbox mounts the worktree at the same path the
      host uses, so building into the shared `target/` leaves the host's Mach-O binary in place
      and the sandbox executes it — the failure reads `sh: Syntax error: "(" unexpected`, which
      looks like a shell bug rather than an architecture mismatch.
    * **Verification by running it.** `test -x` passes on a binary of the wrong architecture.
      Running `--version` and parsing the line is the only check that proves the thing works, and
      it doubles as a read of the grammar range the D5 assertions depend on.
    """
    cargo = runner.sbx_sh("command -v cargo", timeout=120)
    if cargo.returncode != 0 or not cargo.stdout.strip():
        return None, "no `cargo` in the sandbox, so `mochiko-cli` cannot be built there"

    build = runner.sbx_sh(
        "cargo build --release -p mochiko-cli "
        f"--manifest-path {shlex.quote(str(REPO / 'Cargo.toml'))} "
        f"--target-dir {shlex.quote(SANDBOX_TARGET_DIR)} 2>&1 | tail -5",
        timeout=1800,
    )
    binary = f"{SANDBOX_TARGET_DIR}/release/mochiko-cli"
    version = runner.sbx_sh(f"{shlex.quote(binary)} --version", timeout=120)
    if version.returncode != 0:
        return None, (
            f"the sandbox build produced no runnable binary "
            f"(exit {version.returncode}: {(version.stderr or version.stdout).strip()[:200]}); "
            f"build tail: {build.stdout.strip()[:200]}"
        )
    line = version.stdout.strip().splitlines()[0] if version.stdout.strip() else ""
    if not VERSION_LINE.match(line):
        return None, f"`{binary} --version` printed {line!r}, not the version line"
    return binary, None


# ---------------------------------------------------------------------------
# assertions
# ---------------------------------------------------------------------------

def events_of(stdout: str) -> list:
    out = []
    for line in stdout.splitlines():
        line = line.strip()
        if not line.startswith("{"):
            continue
        try:
            out.append(json.loads(line))
        except json.JSONDecodeError:
            continue
    return out


def transcript_text(events: list) -> str:
    """Everything the model saw or said, as one string."""
    chunks = []
    for event in events:
        chunks.append(json.dumps(event, ensure_ascii=False))
    return "\n".join(chunks)


def tool_uses(events: list) -> list:
    """Every `tool_use` block in the stream.

    Defensive about the event shape rather than trusting it: some rows carry `message` as a plain
    string instead of an object, and the unguarded form raised `AttributeError` on the first
    session that produced one (measured 2026-09-04, a natural-language dispatch). Every case in
    the suite calls this, including the no-Read assertion, so a crash here would take down a run
    rather than fail a check.
    """
    uses = []
    for event in events:
        message = event.get("message")
        if not isinstance(message, dict):
            continue
        content = message.get("content")
        if not isinstance(content, list):
            continue
        for block in content:
            if isinstance(block, dict) and block.get("type") == "tool_use":
                uses.append(block)
    return uses


# The four helpers below belong to the DELIVERY path — the wave-3 per-primitive cases, where the
# `!` line exits 0, the expansion completes, and a model turn actually runs. They are unused by
# the two wave-1 halt cases, which are keyed to the measured harness-level shape instead (see
# `assert_no_model_turn`). Kept rather than deleted: they are four of D8's six assertions, and
# deleting them would leave the set looking smaller than it is.


def assert_bang_ran(text: str) -> str | None:
    """The `!` slot was expanded rather than passed through or denied.

    Delivery path only. In a halt case the block never reaches the model at all, so this would
    report "the probe command never reached the model" for the very run it is meant to pass.
    """
    if "CONTRACT-BLOCK-BEGIN" not in text:
        return "the probe command never reached the model"
    if "!`mochiko-cli rules" in text:
        return "the `!` line was passed through literally — preprocessing did not run"
    if "Permission to use Bash has been denied" in text:
        return "the `!` line was denied — the allowed-tools grant is not in force"
    return None


def assert_version_triple(text: str) -> str | None:
    if TRIPLE_HEAD not in text or not all(marker in text for marker in TRIPLE_MARKERS):
        return "no version-triple line reached the model"
    return None


def assert_no_version_triple(text: str) -> str | None:
    if all(marker in text for marker in TRIPLE_MARKERS):
        return "a version-triple line reached the model, and none should have"
    return None


def assert_end_line(text: str) -> str | None:
    if END_LINE not in text:
        return "no closing `mochiko-cli rules end` line reached the model"
    return None


def is_schema_path(path: str) -> bool:
    """Whether a path names a shipped schema file.

    Broader than "under `plugins/mochiko/schemas/`" on purpose. From wave 3 the plugin under test
    is a staged **copy** at `evals/.work/contract-<case>-<id>/mochiko/`, so a fallback Read of the
    copy's own `schemas/brainstorm.yaml` matches neither the repository path nor the `schema.yaml`
    suffix — the assertion would have passed a run that did exactly the thing no-fallback exists to
    rule out. Any `.yaml` under a `schemas/` directory counts, wherever it was staged.

    Wave 6 widened it once more, for a hole the wave-6 bite proofs found rather than a run did:
    the directory test was `"/schemas/" in path`, so a **relative** `schemas/common.yaml` — the
    form a Read written from the plugin root produces — matched neither limb and passed. The
    leading-segment case is now its own test.
    """
    return path.endswith("schema.yaml") or (
        path.endswith(".yaml")
        and ("/schemas/" in path or path.startswith("schemas/"))
    )


def names_schema_source(path: str) -> bool:
    """Whether a search argument points into the schema corpus — a file, or the directory itself.

    `is_schema_path` answers for a file. A search can be pointed at the directory instead, or at a
    glob that names it, and reading the rules out of `schemas/` a line at a time is the same
    fallback wearing a third hat.
    """
    return (
        is_schema_path(path)
        or "/schemas/" in path
        or path.startswith("schemas/")
        or path.rstrip("/").endswith("schemas")
    )


def schema_reads_in(rows: list) -> tuple[list[str], list[str]]:
    """Every schema-file read, and every mere listing, named by the `tool_use` blocks in `rows`.

    **Structural, never a substring search over the channel's text.** From wave 6 this walks
    session transcripts as well as stream events, and a transcript carries the rendered rules
    themselves, the user's prompt and the model's own prose. A text search for a schema path
    across all that would fire on a run which merely *named* one — which, until migration `0003`
    reworded them, the rules did — and would call a clean run dirty. Only a tool call that would
    hand the file's content back counts.

    The split is by what the call returns. `Read`, `NotebookRead`, a shell read and a content-mode
    `Grep` all return rule text, and are the failure no-fallback exists to rule out. `Glob` and a
    name-mode `Grep` return paths, which delivers no rule to anyone; those come back separately
    and are recorded rather than gated.
    """
    reads, listings = [], []
    for use in tool_uses(rows):
        name, args = use.get("name"), (use.get("input") or {})
        if name in ("Read", "NotebookRead"):
            path = str(args.get("file_path", ""))
            if is_schema_path(path):
                reads.append(f"{name} {path}")
        elif name == "Bash":
            # Routed through `is_schema_path` so the two limbs cannot drift: a separate regex
            # here missed `skills/<name>/schema.yaml`, which the `Read` limb caught by suffix.
            command = str(args.get("command", ""))
            for token in re.findall(r"[\w./-]+\.yaml", command):
                if is_schema_path(token):
                    reads.append(f"Bash {token}")
        elif name == "Grep":
            named = [
                f"{field}={value}"
                for field in ("path", "glob")
                if (value := str(args.get(field, ""))) and names_schema_source(value)
            ]
            if named:
                content = str(args.get("output_mode") or "") == "content"
                (reads if content else listings).append("Grep " + " ".join(named))
        elif name == "Glob":
            pattern = str(args.get("pattern", ""))
            if names_schema_source(pattern):
                listings.append(f"Glob {pattern}")
    return reads, listings


def assert_no_schema_read(events: list) -> str | None:
    """No schema file was read on this case's stream (D8).

    Kept for the two fixture cases, which halt before any model turn and fetch no transcript, so
    the stream is the only channel they have. Every case that reaches a session uses the wider
    evidence sweep below instead.
    """
    reads, _ = schema_reads_in(events)
    return None if not reads else f"a schema file was read: {reads[0]}"


def sweep_evidence(root: pathlib.Path) -> tuple[str | None, list[str]]:
    """Every JSONL channel a case captured, swept for schema reads and schema listings.

    D8 scoped the no-Read assertion per converted primitive through waves 3 to 5, because a
    converted command invoking an unconverted skill legitimately read that skill's file. From
    wave 6 no schema file ships at all, so the assertion is unconditional — and its *reach* has to
    widen with it. Keying it to one case's stream events left a real hole: a subagent's turns are
    in neither the parent's stream nor the parent's transcript, so a fallback read inside the
    `preload` case's spawned agent was invisible to the assertion written to catch exactly that.

    The subject is the case's own evidence directory, which is where every channel has already
    been written by the time a check list is built — `stream*.jsonl` from `run_probe`,
    `transcript*.jsonl` from `fetch_transcript`, `sidechain*-N.jsonl` from
    `fetch_sidechain_transcripts`. Nothing is passed in, so a case that grows a fourth channel is
    covered by having captured it.
    """
    problem, listings, channels = None, [], sorted(root.glob("*.jsonl"))
    for path in channels:
        try:
            rows = events_of(path.read_text(encoding="utf-8", errors="replace"))
        except OSError as failure:
            return f"the captured channel {path.name} could not be read: {failure}", listings
        reads, listed = schema_reads_in(rows)
        listings += [f"{path.name}: {entry}" for entry in listed]
        if reads and problem is None:
            problem = f"a schema file was read on {path.name}: {reads[0]}"
    if not channels:
        # An assertion with nothing to read has proved nothing — the same reason the suite skips
        # rather than reporting a clean sweep of zero cases.
        return f"no JSONL channel was captured under {root.name}; the sweep read nothing", []
    return problem, listings


def no_schema_read_checks(root: pathlib.Path) -> list:
    """The gating sweep, plus the recorded listings when the run produced any."""
    problem, listings = sweep_evidence(root)
    checks = [ok("no schema file was read on any captured channel", problem)]
    if listings:
        checks.append(
            report("schema paths listed but never read", "; ".join(listings[:5]))
        )
    return checks


def assert_halted(text: str) -> str | None:
    """The run reported a delivery failure rather than proceeding."""
    if PROBE_DELIVERED in text:
        return "the run proceeded as if the rules had been delivered"
    return None


def result_event(events: list) -> dict | None:
    for event in reversed(events):
        if event.get("type") == "result":
            return event
    return None


def assert_no_model_turn(events: list) -> str | None:
    """The halt happened before the model ran at all.

    Measured shape: a failing `!` line aborts the expansion, the harness injects the shell's
    stderr as a user message, and the session ends with `num_turns` 0 and an empty `result`.
    `claude` still exits 0 and `is_error` is false, so a suite that read only the exit code would
    call this a clean run.
    """
    result = result_event(events)
    if result is None:
        return "the session produced no result event"
    turns = result.get("num_turns")
    if turns != 0:
        return f"expected 0 model turns, got {turns!r}"
    if (result.get("result") or "").strip():
        return f"the model produced a result: {str(result.get('result'))[:120]!r}"
    return None


def assert_no_assistant_event(events: list) -> str | None:
    if any(event.get("type") == "assistant" for event in events):
        return "an assistant turn ran; this halt is meant to fire before the model"
    return None


def local_command_stderr(events: list) -> list[str]:
    """Every `<local-command-stderr>` block the harness injected as a user message."""
    blocks = []
    for event in events:
        if event.get("type") != "user":
            continue
        content = (event.get("message") or {}).get("content")
        chunks = []
        if isinstance(content, str):
            chunks = [content]
        elif isinstance(content, list):
            for block in content:
                if isinstance(block, str):
                    chunks.append(block)
                elif isinstance(block, dict) and isinstance(block.get("text"), str):
                    chunks.append(block["text"])
        for chunk in chunks:
            blocks += re.findall(
                r"<local-command-stderr>(.*?)</local-command-stderr>", chunk, re.S
            )
    return blocks


def assert_local_command_stderr(events: list, *fragments: str) -> str | None:
    """The harness injected the failing command's stderr, carrying these fragments."""
    blocks = local_command_stderr(events)
    if not blocks:
        return "no `<local-command-stderr>` message was injected"
    joined = "\n".join(blocks)
    missing = [fragment for fragment in fragments if fragment not in joined]
    if missing:
        return f"the injected stderr is missing {missing}: {joined.strip()[:300]!r}"
    return None


def assert_message(text: str, fragment: str) -> str | None:
    if fragment not in text:
        return f"{fragment!r} never reached the model"
    return None


def assert_skew_halt_on_stderr(proc, fragment: str) -> str | None:
    """The binary's own D5 behaviour, read off the process rather than the transcript.

    The halt message goes to stderr, stdout stays empty, and the exit code is 3. Asserting this
    directly keeps the case honest whatever Claude Code does with stderr: the transcript
    assertion says the message reached the model, and this one says the binary wrote it.
    """
    problems = []
    if proc.returncode != 3:
        problems.append(f"expected exit 3 from the skew log, got {proc.returncode}")
    if proc.stdout.strip():
        problems.append(f"stdout was not empty: {proc.stdout.strip()[:120]!r}")
    if fragment not in proc.stderr:
        problems.append(f"{fragment!r} is not on stderr: {proc.stderr.strip()[:200]!r}")
    return "; ".join(problems) if problems else None


# ---------------------------------------------------------------------------
# cases
# ---------------------------------------------------------------------------

def sandbox_path(runner) -> tuple[str | None, str | None]:
    """The sandbox's own `PATH`, and whether `mochiko-cli` is absent from it.

    The absence case needs a PATH that carries `claude` but not `mochiko-cli`. Hand-writing one
    (`/usr/bin:/bin`) removes `claude` too — it lives in `~/.local/bin` — and the run dies with
    `env: 'claude': No such file or directory` before a session ever starts, which is a broken
    harness rather than the halt the case is about. So the PATH is read from the sandbox, and the
    binary's absence from it is verified rather than assumed.
    """
    probe = runner.sbx_sh(
        'printf %s "$PATH"; echo; command -v mochiko-cli || true', timeout=120
    )
    if probe.returncode != 0 or not probe.stdout.strip():
        return None, f"the sandbox PATH could not be read: {probe.stderr.strip()[:200]}"
    lines = probe.stdout.splitlines()
    value = lines[0].strip()
    found = "\n".join(lines[1:]).strip()
    if not value:
        return None, "the sandbox reported an empty PATH"
    if found:
        return None, (
            f"`mochiko-cli` is already on the sandbox PATH at {found!r}; the absence case "
            "cannot be run against it"
        )
    return value, None


class Staged(NamedTuple):
    """One case's working directory: the staged plugin, and where its evidence lands."""

    root: pathlib.Path
    plugin: pathlib.Path


def stage(case: str, source: pathlib.Path = FIXTURE) -> Staged:
    """Copy a plugin into `evals/.work/`, where the sandbox sees the same path.

    `source` is the fixture probe plugin for the wave-1 cases and the real `plugins/mochiko/` for
    the wave-3 ones — the whole plugin, its migration log and its hooks included, which is what
    makes the staged copy a faithful subject: the `!` lines resolve their log through
    `${CLAUDE_PLUGIN_ROOT}/migrations`, so a case that wants to perturb the log perturbs this copy
    and never the repository.

    The directory is also the case's evidence directory. D8 wants the transcript on disk, not a
    pass/fail line: a case that fails has to be readable afterwards without re-running it, and a
    case that passes has to be auditable by someone who was not here.
    """
    root = WORK / f"contract-{case}-{uuid.uuid4().hex[:8]}"
    root.mkdir(parents=True, exist_ok=True)
    plugin = root / source.name
    shutil.copytree(source, plugin)
    STAGED_ROOTS.append(root)
    return Staged(root, plugin)


def write_evidence(root: pathlib.Path, name: str, text: str) -> None:
    (root / name).write_text(text, encoding="utf-8")


def write_verdict(root: pathlib.Path, case: str, checks: list, extra: dict) -> None:
    """The case's machine-readable outcome, beside its transcript."""
    payload = {
        "case": case,
        "ran": True,
        "failed": [c.name for c in checks if c.status == "fail"],
        "pending": [c.name for c in checks if c.status == "pending"],
        "reported": [c.name for c in checks if c.status == "report"],
        "checks": [
            {"name": c.name, "status": c.status, "detail": c.detail} for c in checks
        ],
        **extra,
    }
    write_evidence(root, "verdict.json", json.dumps(payload, indent=2) + "\n")


def write_skew_log(root: pathlib.Path) -> pathlib.Path:
    """A migration log the binary declares itself unable to read (record D5)."""
    log = root / "migrations"
    log.mkdir(parents=True, exist_ok=True)
    (log / "0001-skew.yaml").write_text(
        "grammar: 99\n"
        "id: 0001-skew\n"
        "sequence: 1\n"
        "intent: A log from a grammar this binary does not read.\n"
        'hash: "sha256:0000000000000000000000000000000000000000000000000000000000000000"\n'
        "changes: []\n",
        encoding="utf-8",
    )
    return log


class Probed(NamedTuple):
    events: list
    proc: subprocess.CompletedProcess


def run_probe(
    runner,
    staged: Staged,
    *,
    path_env: str,
    log_dir: str | None,
    prompt: str = "/mochiko-contract-probe:rules-probe",
    max_turns: int = 3,
    settings: dict | None = None,
    tag: str = "",
) -> Probed:
    """One headless run of a staged plugin's command, with its evidence written to disk.

    Wrapped in `sh -c` through `runner.sbx_sh` rather than passed as argv: `sbx exec` rejects an
    empty argv element, and `claude_args` carries one (`--setting-sources ''`, which is what
    keeps the sandbox's user-level plugin install out of the run).

    `settings` is appended as `--settings '<json>'` rather than written into the workspace,
    because the CLI applies `--settings` even under an empty `--setting-sources` — which is what
    lets the policy and hooks-off cases set one flag without dragging the sandbox's own user
    configuration back into the run. `claude_args` is used, never forked; the flag is appended to
    what it returns.

    `tag` distinguishes the evidence files of several runs sharing one case directory, which is
    what the three delivery replicates need.
    """
    args = runner.claude_args(prompt, "sonnet", max_turns, True, staged.plugin)
    if settings is not None:
        args += ["--settings", json.dumps(settings, separators=(",", ":"))]
    env = [f"PATH={shlex.quote(path_env)}"]
    if log_dir is not None:
        env.append(f"MOCHIKO_MIGRATIONS={shlex.quote(log_dir)}")
    workspace = f"/tmp/contract-{uuid.uuid4().hex[:8]}"
    script = (
        f"mkdir -p {workspace} && cd {workspace} && "
        f"env {' '.join(env)} {shlex.join(args)}"
    )
    proc = runner.sbx_sh(script)
    runner.sbx_sh(f"rm -rf {workspace}", timeout=60)

    write_evidence(staged.root, f"argv{tag}.txt", "\n".join(args) + "\n")
    write_evidence(staged.root, f"script{tag}.sh", script + "\n")
    write_evidence(staged.root, f"stream{tag}.jsonl", proc.stdout)
    write_evidence(staged.root, f"stderr{tag}.txt", proc.stderr)
    return Probed(events_of(proc.stdout), proc)


def case_absence(runner, sandbox: "Sandbox") -> tuple[list, pathlib.Path]:
    """The binary is off PATH: nothing can be delivered, and the run must halt.

    The measured shape (sandbox `claude-mochiko`, this wave): the `!` line fails, the harness
    aborts the expansion and injects the shell's stderr as a user message, and **no model turn
    happens at all** — `num_turns` 0, empty `result`, `is_error` false, `claude` exit 0. The
    `.md`'s halt clause never executes, because the model never runs. The halt is real and it is
    earlier than the clause: the assertions below are keyed to where it actually fires.

    Wave 1 carried a `pending` assertion here — "the install line reaches the model" — waiting on
    hooks that did not exist. They exist now, but they ship in `plugins/mochiko/` and this case
    loads the fixture plugin, which has none. The assertion therefore resolved into
    `brainstorm-absence`, which stages the real plugin, rather than into this case.
    """
    staged = stage("absence")
    probed = run_probe(runner, staged, path_env=sandbox.path, log_dir=None)
    events, text = probed.events, transcript_text(probed.events)
    checks = [
        ok("no model turn ran", assert_no_model_turn(events)),
        ok("no assistant event", assert_no_assistant_event(events)),
        ok(
            "the harness injected the shell's stderr, naming the missing binary",
            assert_local_command_stderr(events, "mochiko-cli", "command not found"),
        ),
        ok("no version triple reached the model", assert_no_version_triple(text)),
        ok("no schema file was Read", assert_no_schema_read(events)),
        ok("nothing was delivered", assert_halted(text)),
    ]
    write_verdict(
        staged.root,
        "absence",
        checks,
        {
            "shape": "harness-level halt before any model turn",
            "claude_exit": probed.proc.returncode,
            "result_event": result_event(events),
            "local_command_stderr": local_command_stderr(events),
        },
    )
    return checks, staged.root


def case_skew(runner, sandbox: "Sandbox") -> tuple[list, pathlib.Path]:
    """The log is out of the binary's grammar range: the D5 halt, not a partial read.

    Two assertions of the same halt, from opposite sides. The direct run says the binary wrote
    it — exit 3, the message on stderr, stdout empty. The probe run says the harness carried it:
    the `!` line exits non-zero, so the expansion aborts exactly as in the absence case, and the
    injected `<local-command-stderr>` carries the D5 wording verbatim.
    """
    staged = stage("skew")
    log = write_skew_log(staged.root)

    # The binary's own behaviour first, on the channel it actually writes to.
    direct = runner.sbx_sh(
        f"env PATH={shlex.quote(sandbox.binary_dir + ':' + sandbox.path)} "
        f"MOCHIKO_MIGRATIONS={shlex.quote(str(log))} "
        f"mochiko-cli rules brainstorm --section preamble",
        timeout=120,
    )
    write_evidence(
        staged.root,
        "direct-binary.txt",
        f"exit: {direct.returncode}\n--- stdout ---\n{direct.stdout}"
        f"--- stderr ---\n{direct.stderr}",
    )

    probed = run_probe(
        runner,
        staged,
        path_env=f"{sandbox.binary_dir}:{sandbox.path}",
        log_dir=str(log),
    )
    events, text = probed.events, transcript_text(probed.events)

    checks = [
        ok(
            "the binary halts on stderr with exit 3",
            assert_skew_halt_on_stderr(direct, "cargo install mochiko-cli"),
        ),
        ok("no model turn ran", assert_no_model_turn(events)),
        ok("no assistant event", assert_no_assistant_event(events)),
        ok(
            "the harness injected the D5 halt message",
            assert_local_command_stderr(
                events, "cargo install mochiko-cli", "grammar 99"
            ),
        ),
        ok("no version triple reached the model", assert_no_version_triple(text)),
        ok("no schema file was Read", assert_no_schema_read(events)),
        ok("nothing was delivered", assert_halted(text)),
    ]
    write_verdict(
        staged.root,
        "skew",
        checks,
        {
            "shape": "harness-level halt before any model turn",
            "claude_exit": probed.proc.returncode,
            "direct_binary_exit": direct.returncode,
            "direct_binary_stderr": direct.stderr.strip(),
            "result_event": result_event(events),
            "local_command_stderr": local_command_stderr(events),
        },
    )
    return checks, staged.root


# ---------------------------------------------------------------------------
# wave 3 — the converted primitive
# ---------------------------------------------------------------------------

MINIMAL_PATH = "/usr/bin:/bin:/usr/sbin:/sbin"


def host_sh(script: str, *, env: dict | None = None, stdin: str | None = None,
            timeout: int = 120) -> subprocess.CompletedProcess:
    """One shell command on the host. The hook scripts are POSIX `sh`, so `sh -c` is the subject."""
    return subprocess.run(
        ["sh", "-c", script],
        capture_output=True,
        text=True,
        timeout=timeout,
        env=env,
        input=stdin,
    )


def host_binary() -> tuple[str | None, str | None]:
    """The host's `mochiko-cli`, verified by running it, or the reason there isn't one.

    Same lesson as the sandbox build: `test -x` passes on a binary that cannot run, so the check
    is `--version` and a parse of the line it prints.
    """
    candidates = [str(HOST_BINARY)] if HOST_BINARY.is_file() else []
    found = shutil.which("mochiko-cli")
    if found:
        candidates.append(found)
    for candidate in candidates:
        probe = host_sh(f"{shlex.quote(candidate)} --version")
        line = probe.stdout.strip().splitlines()[0] if probe.stdout.strip() else ""
        if probe.returncode == 0 and VERSION_LINE.match(line):
            return candidate, None
    return None, (
        f"no runnable `mochiko-cli` on the host — build one with "
        f"`cargo build --release -p mochiko-cli` (looked at {HOST_BINARY} and PATH)"
    )


def render(binary: str, primitive: str, section: str, plugin_root: pathlib.Path
           ) -> subprocess.CompletedProcess:
    """One section render, direct from the binary, against a plugin root's own log."""
    return host_sh(
        f"{shlex.quote(binary)} rules {shlex.quote(primitive)} "
        f"--section {shlex.quote(section)} --plugin-root {shlex.quote(str(plugin_root))}"
    )


class Section(NamedTuple):
    id: str
    title: str
    rules: int


def parse_preamble(text: str) -> list[Section]:
    """The section list the preamble prints, which is the render's own statement of what follows.

    Read rather than hard-coded: the expected block set and the expected end-line counts then come
    from the same source the delivery does, so a schema change breaks a check instead of quietly
    rewriting the expectation.
    """
    sections, inside = [], False
    for line in text.splitlines():
        if line.strip() == "sections":
            inside = True
            continue
        if inside:
            if not line.startswith("- "):
                break
            parts = [part.strip() for part in line[2:].split(" · ")]
            if len(parts) < 3:
                continue
            count = re.match(r"(\d+) rules?$", parts[-1])
            if count:
                sections.append(Section(parts[0], " · ".join(parts[1:-1]), int(count.group(1))))
    return sections


def rendered_floor_ids(binary: str, primitive: str, plugin_root: pathlib.Path) -> set[str]:
    """Every `class: floor` rule id the binary renders for a primitive.

    Each command's read-back bar names a fixed id set, pre-registered before that wave's first
    session. `EXPECTED` holds those sets as written-down constants — a bar derived from the thing
    it grades is not a bar. What they need is a cross-check: a floor rule added or renamed at a
    later wave would otherwise leave a bar quietly grading the wrong set, and the metric would keep
    reporting a clean 3/3 while asking the wrong question. `converted-shape` runs this function for
    every command in `EXPECTED` and goes red on any difference, in either direction.

    Shape read off the render: an id line `### <id>`, then an attribute line carrying `class:`.
    """
    preamble = render(binary, primitive, "preamble", plugin_root)
    if preamble.returncode != 0:
        return set()
    found, pending_id = set(), None
    for section in [s.id for s in parse_preamble(preamble.stdout)]:
        out = render(binary, primitive, section, plugin_root)
        if out.returncode != 0:
            continue
        for line in out.stdout.splitlines():
            if line.startswith("### "):
                pending_id = line[4:].strip()
            elif pending_id and line.startswith("[") and "class:" in line:
                if "class: floor" in line:
                    found.add(pending_id)
                pending_id = None
    return found


FLOORS_LINE = re.compile(r"^floors:[ \t]*(.*)$", re.M)


def floors_from_render(binary: str, primitive: str, plugin_root: pathlib.Path
                       ) -> tuple[set[str] | None, set[str], str | None]:
    """The primitive's `class: floor` ids, read two ways, and any disagreement between them.

    The preamble carries a `floors:` index line (wave-5 plan §2) listing every floor id in render
    order, and `floors: none` when there are none. That line is the cheap read and the one the
    `.md`'s read-back sentence cites. Walking the section renders is the expensive read and the
    one that cannot be wrong by construction, because it is the rule bodies themselves.

    Both are computed and returned. Their disagreement is a failing check wherever this is called,
    which makes the index line's own correctness a thing the suite tests rather than trusts. The
    first element is `None` when the render carries no `floors:` line at all — the shape before
    P1's change, kept so this file stays runnable against an older binary.
    """
    preamble = render(binary, primitive, "preamble", plugin_root)
    if preamble.returncode != 0:
        return None, set(), f"the preamble render failed: {preamble.stderr.strip()[:200]!r}"
    match = FLOORS_LINE.search(preamble.stdout)
    listed = None
    if match:
        value = match.group(1).strip()
        listed = set() if value == "none" else {
            token.strip() for token in value.split("·") if token.strip()
        }
    walked = rendered_floor_ids(binary, primitive, plugin_root)
    problem = None
    if listed is not None and listed != walked:
        problem = (
            f"the `floors:` line and the section renders disagree — only on the line: "
            f"{sorted(listed - walked)}; only in the sections: {sorted(walked - listed)}"
        )
    return listed, walked, problem


def transcript_floor_ids(text: str) -> set[str]:
    """Every rule id delivered into a session *as a floor rule*, read off the transcript.

    The render's rule shape is an id heading followed by an attribute line, and this pairs them
    the same way `rendered_floor_ids` pairs them on the binary's own output. Two properties make
    it a delivery assertion rather than a recall one:

    * The pair cannot come from the model. A read-back names ids comma-separated on a `FLOOR:`
      line; the converted `.md` body carries no rule ids at all after the re-point. Only a
      rendered block puts `### <id>` immediately above a `class: floor` attribute line.
    * It is read from the transcript, which is where the delivered blocks actually are — the
      stream carries no row containing the expanded prompt.
    """
    found, pending_id = set(), None
    for line in text.splitlines():
        stripped = line.strip()
        if stripped.startswith("### "):
            pending_id = stripped[4:].strip()
        elif pending_id and stripped.startswith("[") and "class:" in stripped:
            if "class: floor" in stripped:
                found.add(pending_id)
            pending_id = None
    return found


def assert_floor_delivery(text: str, expected: set[str]) -> str | None:
    """**Criterion (1).** Every `class: floor` rule of the primitive reached the model.

    This is the wave-5 re-key (record wave-4 section; the user's ruling at the wave open). The
    wave-4 criterion was the model's own read-back, which measured recall of a long list and
    tripped on three commands whose every missed id was verifiably present in the transcript they
    read. This asks the question that was always meant: was it delivered. It is deterministic,
    it gates, and the read-back stays beside it as a recorded measurement.

    A superset test on purpose. Extra floor ids in a transcript cannot buy a false pass — they
    would have to come from a rendered block, and delivering the wrong primitive's blocks still
    fails the head-line and end-line assertions beside this one.
    """
    if not expected:
        return None
    delivered = transcript_floor_ids(text)
    missing = sorted(expected - delivered)
    if missing:
        return (
            f"{len(missing)} of {len(expected)} floor rules never reached the model as a "
            f"`### <id>` heading with a `class: floor` line: {missing}"
        )
    return None


def floor_pin(binary: str, primitive: str, plugin_root: pathlib.Path) -> int | None:
    """The `class: floor` count the preamble itself pins.

    The count line is graded against this rather than against `len(...)` of the pre-registered
    set, so the two sides of that comparison cannot be the same number by construction. Same
    function `diagnostic.py` carries, and for the same reason.
    """
    out = render(binary, primitive, "preamble", plugin_root)
    if out.returncode != 0:
        return None
    inside = False
    for line in out.stdout.splitlines():
        if line.strip() == "pins":
            inside = True
            continue
        if inside:
            if not line.startswith("- "):
                break
            found = re.match(r"- class: floor · (\d+) rules?$", line)
            if found:
                return int(found.group(1))
    return None


def converted_primitives(plugin_root: pathlib.Path) -> list[tuple[str, str, pathlib.Path]]:
    """Every primitive whose rules come from the binary, as `(kind, name, file)`.

    The same test the dependency hook makes — the primitive's own file carries the `!` line — so
    the suite and the hook can never disagree about what is converted.
    """
    out = []
    for path in sorted((plugin_root / "commands").glob("*.md")):
        if CONVERTED_MARK in path.read_text(encoding="utf-8"):
            out.append(("command", path.stem, path))
    for path in sorted((plugin_root / "skills").glob("*/SKILL.md")):
        if CONVERTED_MARK in path.read_text(encoding="utf-8"):
            out.append(("skill", path.parent.name, path))
    return out


def converted_commands(plugin_root: pathlib.Path) -> list[str]:
    """Every converted command, in schema order, with the pilot first.

    The case matrix is built from this rather than from a written-down list, for the same reason
    the hook is: the primitive's own file says whether it is converted, so the suite grows with the
    conversion waves and can never disagree with what actually ships. The pilot leads because the
    three mechanism cases hang off it, and keeping its five cases adjacent preserves the wave-3
    reading order.
    """
    names = [name for kind, name, _ in converted_primitives(plugin_root) if kind == "command"]
    lead = [name for name in names if name == PILOT_COMMAND]
    return lead + [name for name in names if name != PILOT_COMMAND]


def converted_skills(plugin_root: pathlib.Path) -> list[str]:
    """Every converted skill, in the arc's own family order where the freeze knows it.

    Discovered exactly as the commands are — the skill's own `SKILL.md` carries the `!` line — so
    the case matrix grows family by family through wave 5 and a skill P2 has not re-pointed
    contributes no case. Ordered by family (review · authoring · patterns · dense five, the
    conversion order) and then by name, so a partial run's case list reads in the order the
    families landed; a converted skill the freeze does not know sorts last and its cases fail
    loudly on the missing row rather than being skipped.
    """
    names = [name for kind, name, _ in converted_primitives(plugin_root) if kind == "skill"]
    order = list((SKILL_FREEZE.get("families") or {}))
    return sorted(
        names,
        key=lambda name: (
            order.index(family_of(name)) if family_of(name) in order else len(order),
            name,
        ),
    )


def unconverted_primitive(plugin_root: pathlib.Path, kind: str) -> str | None:
    """One primitive the hook must leave alone, because it has no rules to deliver.

    Through wave 5 the subject of this was the transition clause: a primitive still reading a
    shipped schema file was never gated. The clause expired at wave 6 and no schema file ships,
    but the limb it exercised outlives it and the subject is still real. Eight shipped skills
    carry no schema and never did — the router and the prose skills — so the hook must leave them
    alone for the simpler reason that there is nothing to halt for.
    """
    if kind == "command":
        for path in sorted((plugin_root / "commands").glob("*.md")):
            if CONVERTED_MARK not in path.read_text(encoding="utf-8"):
                return path.stem
        return None
    for path in sorted((plugin_root / "skills").glob("*/SKILL.md")):
        if CONVERTED_MARK not in path.read_text(encoding="utf-8"):
            return path.parent.name
    return None


def swap_plugin_log(plugin: pathlib.Path) -> pathlib.Path:
    """Replace a staged plugin's own migration log with one out of the binary's grammar range.

    This — not `MOCHIKO_MIGRATIONS` — is the lever for a converted command, because its `!` lines
    pass `--plugin-root "${CLAUDE_PLUGIN_ROOT}"`, and the resolution order is `--log-dir` ›
    `--plugin-root <root>/migrations` › `MOCHIKO_MIGRATIONS` › `./migrations`. The environment
    variable loses to the flag the command actually passes, so a case that set it would leave the
    real log in play and quietly test nothing.
    """
    log = plugin / "migrations"
    if log.exists():
        shutil.rmtree(log)
    log.mkdir(parents=True)
    (log / "0001-skew.yaml").write_text(
        "grammar: 99\n"
        "id: 0001-skew\n"
        "sequence: 1\n"
        "intent: A log from a grammar this binary does not read.\n"
        'hash: "sha256:0000000000000000000000000000000000000000000000000000000000000000"\n'
        "changes: []\n",
        encoding="utf-8",
    )
    return log


# ---------------------------------------------------------------------------
# what the model saw
# ---------------------------------------------------------------------------

def session_id_of(events: list) -> str | None:
    init = init_event(events)
    return (init or {}).get("session_id")


def json_strings(node, out: list) -> None:
    if isinstance(node, str):
        out.append(node)
    elif isinstance(node, dict):
        for value in node.values():
            json_strings(value, out)
    elif isinstance(node, list):
        for value in node:
            json_strings(value, out)


def transcript_plaintext(raw: str) -> str:
    """Every string value in a session transcript JSONL, concatenated.

    Parsed rather than grepped: the transcript is JSON, so a raw substring search would have to
    reason about escaping, and the rendered blocks are full of `·` separators and newlines. Walking
    the values and joining the strings gives back exactly the text as the session held it.
    """
    chunks = []
    for line in raw.splitlines():
        line = line.strip()
        if not line.startswith("{"):
            continue
        try:
            json_strings(json.loads(line), chunks)
        except json.JSONDecodeError:
            continue
    return "\n".join(chunks)


def fetch_transcript(runner, staged: Staged, session_id: str, tag: str = "") -> tuple[str, str]:
    """Copy a session's transcript JSONL out of the sandbox, and return its text and its path.

    **This is where the delivered rules actually are.** `--output-format stream-json` does not
    carry the expanded prompt: the seven rendered blocks appear in no stream row at all (measured
    on a host fire of the real plugin, 2026-09-04). They are in the session transcript, which the
    hook stdin names as `transcript_path` and which lives under the session user's
    `~/.claude/projects/`. Every assertion about what the model was given therefore reads this
    file, and a copy of it lands in the evidence directory so the claim is auditable later.

    The file is found by globbing the session id rather than by rebuilding the project-directory
    name from the working directory, which is a lossy munge of path separators and dots.
    """
    if not session_id:
        return "", ""
    found = runner.sbx_sh(
        f"ls ~/.claude/projects/*/{shlex.quote(session_id)}.jsonl 2>/dev/null | head -n 1",
        timeout=120,
    )
    path = found.stdout.strip()
    if not path:
        return "", ""
    got = runner.sbx_sh(f"cat {shlex.quote(path)}", timeout=300)
    if got.returncode != 0:
        return "", path
    write_evidence(staged.root, f"transcript{tag}.jsonl", got.stdout)
    return transcript_plaintext(got.stdout), path


def user_text(events: list) -> str:
    """Every user-role message, concatenated — where an expanded prompt arrives."""
    chunks = []
    for event in events:
        if event.get("type") != "user":
            continue
        content = (event.get("message") or {}).get("content")
        if isinstance(content, str):
            chunks.append(content)
        elif isinstance(content, list):
            for block in content:
                if isinstance(block, str):
                    chunks.append(block)
                elif isinstance(block, dict) and isinstance(block.get("text"), str):
                    chunks.append(block["text"])
    return "\n".join(chunks)


def final_assistant_text(events: list) -> str:
    for event in reversed(events):
        if event.get("type") != "assistant":
            continue
        content = ((event.get("message") or {}).get("content")) or []
        parts = [
            block["text"]
            for block in content
            if isinstance(block, dict) and isinstance(block.get("text"), str)
        ]
        if parts:
            return "\n".join(parts)
    result = result_event(events)
    return str((result or {}).get("result") or "")


def asserted_output(probed: Probed, transcript: str) -> str:
    """The two channels measured to carry what the session actually delivered.

    **This is what a positive assertion may read, and nothing wider.** Across every recorded run
    the two channels that carry delivered text are the session transcript and the stream's own
    events; the process streams never do. Including `proc.stderr` in an assertion would mean a
    regression where a hook's message stops reaching the session but still lands on the binary's
    stderr keeps a check named "the install line reached the session" green — the false pass the
    keying discipline exists to rule out. The process streams stay in `channels_of` as evidence,
    where they cost nothing and prove something.

    Negative assertions read `session_output_with` instead: for those, breadth is strictness.
    """
    return "\n".join([transcript_text(probed.events), transcript])


def session_output(probed: Probed) -> str:
    """Everything the run produced, whatever channel carried it.

    The widest text available, and therefore the right subject for a **negative** assertion —
    "this string appears nowhere" is a stronger claim the more places it looks. Positive
    assertions must use `asserted_output`, which is narrowed to the channels measured to carry
    delivered content.
    """
    return "\n".join(
        [transcript_text(probed.events), probed.proc.stdout or "", probed.proc.stderr or ""]
    )


def session_output_with(probed: Probed, transcript: str) -> str:
    """The widest union, for negative assertions only."""
    return "\n".join([session_output(probed), transcript])


def channels_of(probed: Probed, fragment: str, transcript: str = "") -> list[str]:
    """Which channels carried a fragment. Reported, never asserted.

    The channel names are the ones that turned out to matter: the harness's own injection, the
    session transcript (which carries what the stream does not), and the raw process streams.
    """
    found = []
    if any(fragment in block for block in local_command_stderr(probed.events)):
        found.append("<local-command-stderr>")
    if fragment in user_text(probed.events):
        found.append("stream-user-message")
    if transcript and fragment in transcript:
        found.append("session-transcript")
    if fragment in transcript_text(probed.events):
        found.append("stream-events")
    if fragment in (probed.proc.stderr or ""):
        found.append("process-stderr")
    return found


def assert_in_session(probed: Probed, fragment: str, transcript: str = "") -> str | None:
    """A positive assertion, keyed to the measured channels only (see `asserted_output`)."""
    if fragment not in asserted_output(probed, transcript):
        return (
            f"{fragment!r} is in neither the session transcript nor the stream events"
            + (
                f" (it is on the process streams: {sorted(set(channels_of(probed, fragment)))})"
                if fragment in session_output(probed)
                else ""
            )
        )
    return None


# The measured shape of a hook-blocked expansion (sandbox `claude-mochiko`, 2026-09-04). The
# harness puts its own notice in the `result` field, prefixed like this and followed by the
# blocking script's path and the message the script wrote to stderr.
HOOK_BLOCK_PREFIX = "UserPromptExpansion operation blocked by hook:"


def assert_halt_before_model(events: list) -> str | None:
    """Nothing was delivered and no model turn ran, whichever limb fired first.

    Two halt shapes exist, both measured rather than assumed, and the difference is where the
    `result` field comes from.

    * **Wave 1, the failing `!` line:** the expansion aborts, the harness injects the shell's
      stderr as a user message, and `result` is empty.
    * **Wave 3, the dependency hook:** the hook exits 2 *before* expansion, no
      `<local-command-stderr>` block is injected at all, and `result` carries the harness's own
      notice — `UserPromptExpansion operation blocked by hook: [<script>]: <message>`.

    Both are `num_turns: 0`, no assistant event, `is_error: false`, `claude` exit 0. So a
    non-empty `result` is only acceptable when it is that notice: anything else in that field is
    the model having spoken, which is the failure this assertion exists to catch.
    """
    if any(event.get("type") == "assistant" for event in events):
        return "an assistant turn ran; this halt is meant to fire before the model"
    result = result_event(events)
    if result is None:
        return None
    turns = result.get("num_turns")
    if turns not in (0, None):
        return f"expected 0 model turns, got {turns!r}"
    text = (result.get("result") or "").strip()
    if text and not text.startswith(HOOK_BLOCK_PREFIX):
        return f"the model produced a result: {text[:160]!r}"
    return None


def assert_hook_block(probed: Probed) -> str | None:
    """The halt was the dependency hook blocking before expansion, and nothing else.

    Two halt shapes exist for a converted primitive with no binary, and only one of them is
    correct once the hook covers the path: the hook exits 2 *before* expansion, so the harness's
    own notice rides the result event and **no `<local-command-stderr>` is injected**. The other
    shape — the `!` line failing during expansion, its stderr injected — means the hook did not
    fire, which is the regression this asserts against.

    Both shapes halt safely, which is exactly why the weaker claim is not good enough here: a
    check that accepted either would stay green through the hook silently losing its skill limb.
    That is the tolerant-union failure the wave-3 audit ruled against.
    """
    result = result_event(probed.events)
    if result is None:
        return "the session produced no result event"
    text = str(result.get("result") or "")
    injected = local_command_stderr(probed.events)
    if not text.startswith(HOOK_BLOCK_PREFIX):
        return (
            "the result event carries no hook-block notice, so the hook did not gate this run"
            + (f"; the harness injected its own stderr instead: {injected[0].strip()[:160]!r}"
               if injected
               else f"; the result reads {text.strip()[:160]!r}")
        )
    if injected:
        return (
            "the hook blocked, but the `!` line also ran and its stderr was injected: "
            f"{injected[0].strip()[:160]!r}"
        )
    return None


def halt_shape(probed: Probed) -> dict:
    """The measured shape of a halt, recorded per case so the assertion can be keyed to it."""
    result = result_event(probed.events)
    return {
        "event_types": sorted({event.get("type") for event in probed.events if event.get("type")}),
        "claude_exit": probed.proc.returncode,
        "num_turns": (result or {}).get("num_turns"),
        "is_error": (result or {}).get("is_error"),
        "subtype": (result or {}).get("subtype"),
        "result_text": str((result or {}).get("result") or "")[:300],
        "local_command_stderr": local_command_stderr(probed.events),
        "process_stderr": (probed.proc.stderr or "").strip()[:600],
    }


def init_event(events: list) -> dict | None:
    for event in events:
        if event.get("type") == "system" and event.get("subtype") == "init":
            return event
    return None


def assert_slash_commands(events: list, expected: list[str]) -> str | None:
    """The plugin's commands registered as commands — the wave-0 manifest quirk, re-verified.

    Wave 0 saw a `commands` directory-string manifest register a probe plugin's command files as
    one skill under `--plugin-dir`, while the identical form worked for the real plugin. This is
    the re-verification on the real plugin the record asked for.
    """
    init = init_event(events)
    if init is None:
        return "the session produced no init event"
    listed = set(init.get("slash_commands") or [])
    missing = [name for name in expected if name not in listed]
    if missing:
        return f"the init event's slash_commands is missing {missing}"
    return None


# ---------------------------------------------------------------------------
# the read-back metric — reported, never gating
# ---------------------------------------------------------------------------

FLOOR_LINE = re.compile(r"^\s*\**FLOOR:\**\s*(.*)$", re.M)


def score_read_back(text: str, command: str) -> tuple[list[str], bool]:
    """One replicate's floor read-back: the token list it named, and whether it is exactly right.

    An id counts bare or wrapped in backticks; every other decoration is a miss (lead ruling,
    2026-09-04). Set equality against that command's pre-registered set in `EXPECTED` — every id
    present, nothing else, no partial credit — and a missing `FLOOR:` line is a failed replicate
    rather than a harness error.
    """
    floor_ids = expectation(command).floor_ids
    match = FLOOR_LINE.search(text)
    if not match:
        return [], False
    tokens = []
    for raw in match.group(1).split(","):
        token = raw.strip()
        if token.startswith("`") and token.endswith("`") and len(token) > 1:
            token = token[1:-1].strip()
        if token:
            tokens.append(token)
    return tokens, set(tokens) == set(floor_ids)


COUNT_LINE = re.compile(r"^\s*\**FLOOR-COUNT:\**\s*(.*)$", re.M)


def score_two_line(text: str, primitive: str, pin: int | None) -> dict:
    """One replicate's read-back, graded the diagnostic's four ways. Gates nothing.

    Carried over from `diagnostic.py`'s `score()`, which the wave-4 delta audit recomputed across
    all eighteen replicates with zero mismatches. The count leg is graded against the preamble's
    own pin and the ids leg against the pre-registered set, so a model that holds the right number
    but cannot list them is visibly failing enumeration rather than delivery.
    """
    expected = set(expectation(primitive).floor_ids)
    count_match = COUNT_LINE.search(text)
    raw_count = count_match.group(1).strip() if count_match else ""
    digits = re.search(r"\d+", raw_count)
    named_count = int(digits.group()) if digits else None
    tokens, ids_exact = score_read_back(text, primitive)
    got = set(tokens)
    return {
        "count_line": raw_count or None,
        "named_count": named_count,
        "pin": pin,
        "count_exact": named_count is not None and pin is not None and named_count == pin,
        "ids_named": len(got),
        "ids_exact": ids_exact,
        "ids_superset": expected.issubset(got),
        "omitted": sorted(expected - got),
        "extra": sorted(got - expected),
        "tokens": tokens,
    }


def delivered_blocks(text: str, primitive: str) -> dict[str, str]:
    """Each delivered block, head line through end line, keyed by section id.

    This is the read-cost measurement's subject: what actually arrived, not what the binary would
    print if asked again.
    """
    pattern = re.compile(
        r"(mochiko-cli rules " + re.escape(primitive) + r" · section (\S+) · binary .*?"
        r"mochiko-cli rules end · " + re.escape(primitive) + r" · \2 · \d+ rules)",
        re.S,
    )
    return {match.group(2): match.group(1) for match in pattern.finditer(text)}


def end_line_counts(text: str, primitive: str) -> dict[str, int]:
    pattern = re.compile(
        r"mochiko-cli rules end · " + re.escape(primitive) + r" · (\S+) · (\d+) rules"
    )
    return {match.group(1): int(match.group(2)) for match in pattern.finditer(text)}


def head_line_sections(text: str, primitive: str) -> set[str]:
    """The sections whose version-triple head line is present.

    Placeholder captures are dropped. The command's own halt clause quotes the head line's shape —
    `mochiko-cli rules brainstorm · section <id> · binary <v> · grammar <g> · plugin <p>` — and
    the clause travels with the expanded prompt, so a naive match counts the instructions as a
    delivered block. It never affects the delivery assertions, which ask whether each *expected*
    id is present and `<id>` is not one of them, but it does inflate a count, and a count that
    reads `1 of 7` when nothing was delivered is worse than no count at all.
    """
    pattern = re.compile(
        r"mochiko-cli rules " + re.escape(primitive) + r" · section (\S+) · binary \S+ "
        r"· grammar \S+ · plugin \S+"
    )
    return {
        match.group(1)
        for match in pattern.finditer(text)
        if "<" not in match.group(1) and ">" not in match.group(1)
    }


# ---------------------------------------------------------------------------
# host cases — no sandbox, no session
# ---------------------------------------------------------------------------

def load_captures() -> tuple[dict, list[str]]:
    """The committed hook-stdin captures, indexed by event, plus any note about their provenance.

    The captures are real hook input, sanitized, and they are the *shape* source: each row below
    substitutes only the one field that names the primitive, so the field set stays whatever the
    platform actually sends and the case matrix stays this file's business.
    """
    captures, notes = {}, []
    for path in sorted(HOOK_INPUT.glob("*.json")) if HOOK_INPUT.is_dir() else []:
        try:
            payload = json.loads(path.read_text(encoding="utf-8"))
        except (OSError, json.JSONDecodeError) as err:
            notes.append(f"{path.name} is not readable JSON: {err}")
            continue
        event = payload.get("hook_event_name")
        if event:
            captures.setdefault(event, payload)
    if "SessionStart" not in captures:
        # Approved fallback (lead ruling, 2026-09-04): a labelled synthesis from the documented
        # field set. `session-start.sh` reads only `cwd` from stdin, so the two SessionStart rows
        # would otherwise go untested entirely.
        captures["SessionStart"] = {
            "session_id": "00000000-0000-0000-0000-000000000000",
            "hook_event_name": "SessionStart",
            "cwd": "/tmp/contract-session-start",
            "permission_mode": "default",
            "source": "startup",
        }
        notes.append(
            "no captured SessionStart stdin was committed; this row used a synthesized payload "
            "built from the documented field set (record F14)"
        )
    return captures, notes


def run_hook(script: pathlib.Path, payload: dict, *, path_env: str,
             plugin_root: pathlib.Path, home: pathlib.Path) -> subprocess.CompletedProcess:
    env = {"PATH": path_env, "CLAUDE_PLUGIN_ROOT": str(plugin_root), "HOME": str(home)}
    return host_sh(shlex.quote(str(script)), env=env, stdin=json.dumps(payload))


def case_hook_input(runner, sandbox) -> tuple[list, pathlib.Path]:
    """The two hook scripts, fed real captured stdin, on the host.

    The cheapest gate there is on the hooks: no sandbox, no session, no metered tokens, and it
    catches the failure that would otherwise only show up as a confusing session result — a `sed`
    extraction that misses a field, a converted-check that gates the wrong primitive, a block that
    exits with the wrong code or writes to the wrong channel.

    Every row is keyed to a shape the scripts actually produce, and the two rows that matter most
    are the negative ones: a primitive with no rules of its own must be left completely alone,
    because there is nothing for the binary's absence to have cost it.
    """
    staged = stage("hook-input", PLUGIN)
    checks: list[Check] = []
    dependency = staged.plugin / "hooks" / "scripts" / "dependency-halt.sh"
    session_start = staged.plugin / "hooks" / "scripts" / "session-start.sh"

    for script in (dependency, session_start):
        if not script.is_file():
            checks.append(ok(f"{script.name} exists", f"no script at {script}"))
    binary, reason = host_binary()
    if reason:
        checks.append(ok("a runnable host binary for the present-binary rows", reason))
    if any(check.status == "fail" for check in checks):
        write_verdict(staged.root, "hook-input", checks, {"shape": "host, no session"})
        return checks, staged.root

    captures, notes = load_captures()
    absent_path = MINIMAL_PATH
    probe = host_sh("command -v mochiko-cli", env={"PATH": absent_path})
    checks.append(
        ok(
            "the absent-binary rows really have no `mochiko-cli`",
            None if probe.returncode != 0 else f"found one at {probe.stdout.strip()!r}",
        )
    )
    bin_dir = staged.root / "bin"
    bin_dir.mkdir(exist_ok=True)
    (bin_dir / "mochiko-cli").symlink_to(binary)
    present_path = f"{bin_dir}:{absent_path}"

    # The `PreToolUse` limb's subjects. From wave 5 these are real: every schema-bearing skill is
    # converted, so the rows iterate the shipped set exactly as the command rows do, and the hook
    # is tested against the names it will actually see. Before that landing there is nothing
    # converted to point at, so the case writes a stub into its own staged copy rather than
    # dropping the limb — the wave-3 arrangement, kept for exactly as long as it is needed.
    skill_subjects = converted_skills(staged.plugin)
    if not skill_subjects:
        stub = staged.plugin / "skills" / "contract-stub"
        stub.mkdir(parents=True, exist_ok=True)
        (stub / "SKILL.md").write_text(
            "---\nname: contract-stub\ndescription: staged-only stub for the hook-input case\n"
            "---\n\n!`mochiko-cli rules contract-stub --section preamble`\n",
            encoding="utf-8",
        )
        skill_subjects = ["contract-stub"]
        notes.append(
            "no skill is converted yet, so the `PreToolUse` rows used a staged-only stub "
            "`skills/contract-stub/SKILL.md` (wave-3 arrangement)"
        )

    unconverted_command = unconverted_primitive(staged.plugin, "command")
    unconverted_skill = unconverted_primitive(staged.plugin, "skill")
    if unconverted_command is None:
        # From wave 4 every shipped command is converted, so this row loses its subject and the
        # case would fail on a check about the hook rather than about the wave. The row still has
        # to run: eight shipped skills carry no rules and never will, so the limb it exercises —
        # leave a primitive with nothing to deliver completely alone — stays live on the skill
        # side. So the subject is staged, exactly as the converted-skill row above stages its own,
        # and nothing in `plugins/mochiko/` is touched.
        unconverted_command = "contract-unconverted"
        (staged.plugin / "commands" / f"{unconverted_command}.md").write_text(
            "---\ndescription: staged-only stub for the hook-input leave-alone row\n---\n\n"
            "This command carries no `!` rules line, so the dependency hook must leave it alone.\n",
            encoding="utf-8",
        )
        notes.append(
            "every shipped command is converted, so the unconverted-command row used a "
            "staged-only stub `commands/contract-unconverted.md` (wave-4 plan §4)"
        )
    checks.append(
        ok(
            "the leave-alone rows have a rule-less command and skill to leave alone",
            None
            if unconverted_command and unconverted_skill
            else "no skill without rules remains; that leave-alone row cannot be run",
        )
    )
    if any(check.status == "fail" for check in checks):
        write_verdict(staged.root, "hook-input", checks, {"shape": "host, no session"})
        return checks, staged.root

    upe = captures.get("UserPromptExpansion")
    pre = captures.get("PreToolUse")
    if upe is None or pre is None:
        checks.append(
            ok(
                "captured UserPromptExpansion and PreToolUse stdin are committed",
                f"missing under {HOOK_INPUT}: "
                + ", ".join(
                    name
                    for name, value in (("UserPromptExpansion", upe), ("PreToolUse", pre))
                    if value is None
                ),
            )
        )
        write_verdict(staged.root, "hook-input", checks, {"shape": "host, no session"})
        return checks, staged.root

    def upe_with(name: str) -> dict:
        return {**upe, "command_name": name, "prompt": f"/{name} probe"}

    def skill_with(name: str) -> dict:
        return {**pre, "tool_input": {**(pre.get("tool_input") or {}), "skill": name}}

    def silent(proc) -> str | None:
        problems = []
        if proc.returncode != 0:
            problems.append(f"exit {proc.returncode}, expected 0")
        if proc.stdout.strip():
            problems.append(f"stdout {proc.stdout.strip()[:120]!r}")
        if proc.stderr.strip():
            problems.append(f"stderr {proc.stderr.strip()[:120]!r}")
        return "; ".join(problems) if problems else None

    def json_field(proc, *path: str) -> tuple[object, str | None]:
        if proc.returncode != 0:
            return None, f"exit {proc.returncode}, expected 0: {proc.stderr.strip()[:160]!r}"
        try:
            payload = json.loads(proc.stdout)
        except json.JSONDecodeError as err:
            return None, f"stdout is not JSON ({err}): {proc.stdout.strip()[:160]!r}"
        node = payload
        for key in path:
            if not isinstance(node, dict) or key not in node:
                return None, f"no {'.'.join(path)} in {proc.stdout.strip()[:160]!r}"
            node = node[key]
        return node, None

    rows = []

    def hook(script, payload, path_env):
        proc = run_hook(script, payload, path_env=path_env, plugin_root=staged.plugin,
                        home=staged.root)
        rows.append(
            {
                "script": script.name,
                "path": "present" if path_env == present_path else "absent",
                "exit": proc.returncode,
                "stdout": proc.stdout.strip()[:400],
                "stderr": proc.stderr.strip()[:400],
            }
        )
        return proc

    # --- a primitive with no rules of its own is never gated -----------------------------
    proc = hook(dependency, upe_with(f"mochiko:{unconverted_command}"), absent_path)
    checks.append(ok(f"rule-less command `{unconverted_command}` is left alone", silent(proc)))

    proc = hook(dependency, skill_with(f"mochiko:{unconverted_skill}"), absent_path)
    checks.append(ok(f"rule-less skill `{unconverted_skill}` is left alone", silent(proc)))

    proc = hook(dependency, upe_with("other:thing"), absent_path)
    checks.append(ok("a command outside the mochiko namespace is left alone", silent(proc)))

    # --- absence: the block, on the channel each registration uses -----------------------
    #
    # Every converted command, not just the pilot. The hook extracts the command name from its
    # own stdin and puts it back in the message, so a per-command row is what proves the user is
    # told which command halted rather than being handed a generic notice.
    for command in converted_commands(staged.plugin):
        proc = hook(dependency, upe_with(f"mochiko:{command}"), absent_path)
        problems = []
        if proc.returncode != 2:
            problems.append(f"exit {proc.returncode}, expected 2")
        for fragment in (INSTALL_LINE, f"/mochiko:{command}"):
            if fragment not in proc.stderr:
                problems.append(f"stderr is missing {fragment!r}")
        if proc.stdout.strip():
            problems.append(f"stdout was not empty: {proc.stdout.strip()[:120]!r}")
        checks.append(
            ok(
                f"converted command `{command}` + no binary: exit 2 and the install line on stderr",
                "; ".join(problems) if problems else None,
            )
        )

    # Every converted skill, for the same reason the command rows iterate: the hook puts the
    # primitive's own name back in the deny reason, so a per-skill row is what proves the user is
    # told which skill halted.
    for skill in skill_subjects:
        proc = hook(dependency, skill_with(f"mochiko:{skill}"), absent_path)
        decision, problem = json_field(proc, "hookSpecificOutput", "permissionDecision")
        if problem is None and decision != "deny":
            problem = f"permissionDecision was {decision!r}, expected 'deny'"
        if problem is None:
            reason_text, problem = json_field(
                proc, "hookSpecificOutput", "permissionDecisionReason"
            )
            if problem is None and INSTALL_LINE not in str(reason_text):
                problem = f"the deny reason is missing {INSTALL_LINE!r}: {reason_text!r}"
            if problem is None and f"/mochiko:{skill}" not in str(reason_text):
                problem = f"the deny reason never names `/mochiko:{skill}`: {reason_text!r}"
        checks.append(
            ok(f"converted skill `{skill}` + no binary: a JSON deny carrying the install line",
               problem)
        )

    # --- the prompt-expansion path for a skill -------------------------------------------
    #
    # A `/mochiko:<skill>` prompt line raises `UserPromptExpansion`, not `PreToolUse`: measured at
    # the wave-5 shape probes, which found no `Skill` tool call happens at all. The hook resolves
    # the name against `commands/<bare>.md` first and falls back to `skills/<bare>/SKILL.md`, so
    # one primitive name reaches it down two different limbs and both have to be right. These rows
    # exist because the fallback did not: the wave-5 run measured a converted skill arriving with
    # no hook gating it, and this is the row that would have caught that.
    for skill in skill_subjects:
        proc = hook(dependency, upe_with(f"mochiko:{skill}"), absent_path)
        problems = []
        if proc.returncode != 2:
            problems.append(f"exit {proc.returncode}, expected 2")
        for fragment in (INSTALL_LINE, f"/mochiko:{skill}"):
            if fragment not in proc.stderr:
                problems.append(f"stderr is missing {fragment!r}")
        if proc.stdout.strip():
            problems.append(f"stdout was not empty: {proc.stdout.strip()[:120]!r}")
        checks.append(
            ok(
                f"skill `{skill}` down the prompt-expansion limb + no binary: exit 2 and the "
                "install line",
                "; ".join(problems) if problems else None,
            )
        )

    # --- presence: one confirmation line, per registration, and never the rules ----------
    payloads = [(upe_with(f"mochiko:{command}"), "command", command)
                for command in converted_commands(staged.plugin)]
    payloads += [(skill_with(f"mochiko:{skill}"), "skill", skill) for skill in skill_subjects]
    # The same skills again, down the prompt-expansion limb: the noun must still be `skill`,
    # because the confirmation names what was actually resolved and not which event carried it.
    payloads += [(upe_with(f"mochiko:{skill}"), "skill", skill) for skill in skill_subjects]
    for payload, noun, subject in payloads:
        proc = hook(dependency, payload, present_path)
        context, problem = json_field(proc, "hookSpecificOutput", "additionalContext")
        expected = f"{HOOK_PRESENT_PREFIX} {noun}'s own render"
        if problem is None and str(context) != expected:
            problem = f"additionalContext was {context!r}, expected {expected!r}"
        if problem is None and TRIPLE_HEAD in str(context):
            problem = "the hook injected rules; branch B confirms presence and delivers nothing"
        checks.append(
            ok(f"converted {noun} `{subject}` + binary present: the presence line only", problem)
        )

    # --- skew: the hook's only other gate, and the only one that needs a broken log ------
    skew_root = staged.root / "mochiko-skew"
    shutil.copytree(staged.plugin, skew_root)
    swap_plugin_log(skew_root)
    skew_proc = run_hook(
        dependency,
        upe_with(f"mochiko:{PILOT_COMMAND}"),
        path_env=present_path,
        plugin_root=skew_root,
        home=staged.root,
    )
    rows.append(
        {
            "script": dependency.name,
            "path": "present, skew log",
            "exit": skew_proc.returncode,
            "stdout": skew_proc.stdout.strip()[:400],
            "stderr": skew_proc.stderr.strip()[:400],
        }
    )
    problems = []
    if skew_proc.returncode != 2:
        problems.append(f"exit {skew_proc.returncode}, expected 2")
    for fragment in ("grammar 99", INSTALL_LINE):
        if fragment not in skew_proc.stderr:
            problems.append(f"stderr is missing {fragment!r}")
    checks.append(
        ok(
            "converted command + out-of-range log: exit 2 and the binary's own D5 message",
            "; ".join(problems) if problems else None,
        )
    )

    # --- SessionStart: loud, never blocking ---------------------------------------------
    proc = hook(session_start, captures["SessionStart"], present_path)
    problems = []
    if proc.returncode != 0:
        problems.append(f"exit {proc.returncode}, expected 0")
    line = proc.stdout.strip().splitlines()[0] if proc.stdout.strip() else ""
    if not re.match(r"^mochiko-cli \d+\.\d+\.\d+ · grammar \d+\.\.\d+ · plugin ", line):
        problems.append(f"the first line is {line!r}")
    if "· in range" not in proc.stdout:
        problems.append("no `· in range` in the output")
    checks.append(
        ok("SessionStart + binary present: the version and in-range line",
           "; ".join(problems) if problems else None)
    )

    proc = hook(session_start, captures["SessionStart"], absent_path)
    problems = []
    if proc.returncode != 0:
        problems.append(f"exit {proc.returncode}, expected 0 — SessionStart never blocks")
    if INSTALL_LINE not in proc.stdout:
        problems.append(f"stdout is missing {INSTALL_LINE!r}: {proc.stdout.strip()[:160]!r}")
    checks.append(
        ok("SessionStart + no binary: the install line, still exit 0",
           "; ".join(problems) if problems else None)
    )

    # The unsupported-environment notice. GI-020 declares a policy that blocks inline execution
    # unsupported, and this is the only place a user is told so before their first fire.
    policy_home = staged.root / "policy-home"
    (policy_home / ".claude").mkdir(parents=True, exist_ok=True)
    (policy_home / ".claude" / "settings.json").write_text(
        '{"disableSkillShellExecution": true}\n', encoding="utf-8"
    )
    policy_proc = run_hook(
        session_start,
        captures["SessionStart"],
        path_env=present_path,
        plugin_root=staged.plugin,
        home=policy_home,
    )
    rows.append(
        {
            "script": session_start.name,
            "path": "present, policy set",
            "exit": policy_proc.returncode,
            "stdout": policy_proc.stdout.strip()[:400],
            "stderr": policy_proc.stderr.strip()[:400],
        }
    )
    problems = []
    if policy_proc.returncode != 0:
        problems.append(f"exit {policy_proc.returncode}, expected 0")
    if "unsupported" not in policy_proc.stdout:
        problems.append(f"no unsupported-environment line: {policy_proc.stdout.strip()[:200]!r}")
    checks.append(
        ok("SessionStart names an environment that disables shell execution",
           "; ".join(problems) if problems else None)
    )

    # Where a row's subject came from, when it was not simply the shipped plugin: a synthesized
    # capture, or a staged stub standing in for a primitive the conversion waves have used up.
    for note in notes:
        checks.append(report("row provenance", note))

    write_verdict(
        staged.root,
        "hook-input",
        checks,
        {"shape": "host, no session", "rows": rows, "provenance_notes": notes},
    )
    return checks, staged.root


def case_render_ceiling(runner, sandbox) -> tuple[list, pathlib.Path]:
    """Every render of every converted primitive, against the inline ceiling and the dead phrases.

    Wave-0 probe (e) measured the ceiling at roughly 30,000 characters: above it a `!` block
    arrives as a file-path notice whose preview keeps only the first line, which would strand a
    render without its end line — delivered, apparently fine, and silently truncated. This is the
    one assertion that catches that before a user does, and it needs no session at all.

    Wave 6 folds a second sweep into the same renders, at no extra cost because they are already
    in hand: no rendered rule may still carry `when the binary is absent` or a
    `plugins/mochiko/schemas/` path. That is the host-side half of the run-wide no-fallback
    posture — the sessions prove nothing *read* a schema file, and this proves nothing *told* the
    model to. Its subject is the **render**, never the log: the log is append-only, so
    `0001-genesis.yaml` still carries the two-arm wording by construction and always will. Only
    the replayed state has to be clean, and only the render shows it.
    """
    staged = stage("render-ceiling", PLUGIN)
    checks: list[Check] = []
    binary, reason = host_binary()
    if reason:
        checks.append(ok("a runnable host binary", reason))
        write_verdict(staged.root, "render-ceiling", checks, {"shape": "direct binary"})
        return checks, staged.root

    if sandbox is not None:
        host_version = host_sh(f"{shlex.quote(binary)} --version").stdout.strip()
        sandbox_version = runner.sbx_sh(f"{shlex.quote(sandbox.binary)} --version").stdout.strip()
        checks.append(
            ok(
                "the host binary matches the sandbox build",
                None
                if host_version == sandbox_version
                else f"host {host_version!r} vs sandbox {sandbox_version!r} — "
                "the host binary is stale; rebuild it before trusting these figures",
            )
        )

    primitives = converted_primitives(staged.plugin)
    checks.append(
        ok(
            "at least one primitive is converted",
            None if primitives else "no primitive carries a `!` rules line; nothing to measure",
        )
    )
    measurements, largest, dead = [], None, []
    for kind, name, _ in primitives:
        preamble = render(binary, name, "preamble", staged.plugin)
        if preamble.returncode != 0:
            checks.append(
                ok(f"{kind} `{name}` renders its preamble",
                   f"exit {preamble.returncode}: {preamble.stderr.strip()[:200]!r}")
            )
            continue
        for section in ["preamble"] + [s.id for s in parse_preamble(preamble.stdout)]:
            out = (
                preamble
                if section == "preamble"
                else render(binary, name, section, staged.plugin)
            )
            if out.returncode != 0:
                checks.append(
                    ok(f"{kind} `{name}` renders `{section}`",
                       f"exit {out.returncode}: {out.stderr.strip()[:200]!r}")
                )
                continue
            entry = {
                "primitive": name,
                "section": section,
                "chars": len(out.stdout),
                "bytes": len(out.stdout.encode("utf-8")),
            }
            measurements.append(entry)
            if largest is None or entry["chars"] > largest["chars"]:
                largest = entry
            for line in out.stdout.splitlines():
                for phrase in DEAD_PHRASES:
                    if phrase in line:
                        dead.append(
                            {
                                "primitive": name,
                                "section": section,
                                "phrase": phrase,
                                "line": line.strip()[:200],
                            }
                        )
    over = [m for m in measurements if m["chars"] >= INLINE_CEILING]
    checks.append(
        ok(
            f"every converted render is under the {INLINE_CEILING:,}-char inline ceiling",
            None if not over else f"over the ceiling: {over}",
        )
    )
    checks.append(
        ok(
            "no rendered rule still names a shipped schema file "
            f"({' · '.join(DEAD_PHRASES)})",
            None
            if not dead
            else "; ".join(
                f"{d['primitive']} · {d['section']}: {d['phrase']!r} in {d['line']!r}"
                for d in dead[:5]
            )
            + (f" (+{len(dead) - 5} more)" if len(dead) > 5 else ""),
        )
    )
    if largest is not None:
        checks.append(
            report(
                "largest render",
                f"{largest['primitive']} · {largest['section']} — {largest['chars']:,} chars / "
                f"{largest['bytes']:,} bytes, {largest['chars'] / INLINE_CEILING:.1%} of the ceiling",
            )
        )
    write_verdict(
        staged.root,
        "render-ceiling",
        checks,
        {
            "shape": "direct binary",
            "converted": [f"{kind}:{name}" for kind, name, _ in primitives],
            "measurements": measurements,
            "largest": largest,
            "dead_phrases": {"looked_for": list(DEAD_PHRASES), "found": dead},
        },
    )
    return checks, staged.root


# ---------------------------------------------------------------------------
# sandbox cases — the converted command in a real session
# ---------------------------------------------------------------------------

def primitive_expectations(
    plugin: pathlib.Path, command: str
) -> tuple[list[str], dict[str, int], str | None]:
    """What the delivery must carry, read from the binary rather than written down here.

    Returns the section ids in the order the command's `!` lines fire, the rule count each end
    line must report, and a reason if the expectation could not be built.

    Deliberately *not* pre-registered, unlike the floor sets in `EXPECTED`: the section list and
    its per-section counts are the render's own statement of what follows, so reading them here
    means a schema change breaks a check rather than silently rewriting the expectation. The floor
    sets are the opposite case — they are the bar the model is graded against, so they are written
    down and cross-checked instead.
    """
    binary, reason = host_binary()
    if reason:
        return [], {}, reason
    preamble = render(binary, command, "preamble", plugin)
    if preamble.returncode != 0:
        return [], {}, f"the preamble render failed: {preamble.stderr.strip()[:200]!r}"
    sections = parse_preamble(preamble.stdout)
    if not sections:
        return [], {}, f"no section list in the preamble render: {preamble.stdout[:200]!r}"
    ids = ["preamble"] + [section.id for section in sections]
    counts = {"preamble": 0, **{section.id: section.rules for section in sections}}
    return ids, counts, None


def assert_delivery(
    text: str, command: str, ids: list[str], counts: dict[str, int]
) -> list[Check]:
    heads = head_line_sections(text, command)
    ends = end_line_counts(text, command)
    missing_heads = [i for i in ids if i not in heads]
    missing_ends = [i for i in ids if i not in ends]
    wrong = {i: ends[i] for i in ids if i in ends and ends[i] != counts[i]}
    return [
        ok(
            f"all {len(ids)} version-triple head lines reached the model",
            None if not missing_heads else f"missing {missing_heads}",
        ),
        ok(
            f"all {len(ids)} closing end lines reached the model",
            None if not missing_ends else f"missing {missing_ends}",
        ),
        ok(
            "every end-line count matches the preamble's own section list",
            None if not wrong else f"disagreements (delivered vs pinned): {wrong} vs {counts}",
        ),
        ok("the `!` lines were expanded, not passed through", assert_bang_ran_converted(text)),
        ok("no Bash denial", assert_no_denial(text)),
    ]


def assert_bang_ran_converted(text: str) -> str | None:
    if CONVERTED_MARK in text:
        return "a `!` line was passed through literally — preprocessing did not run"
    return None


def assert_no_denial(text: str) -> str | None:
    if "Permission to use Bash has been denied" in text:
        return "a `!` line was denied — the allowed-tools grant is not in force"
    return None


def expected_slash_commands() -> list[str]:
    return [f"mochiko:{path.stem}" for path in sorted((PLUGIN / "commands").glob("*.md"))]


LATENCY_RUNS = 10


def measure_latency(
    runner, sandbox: "Sandbox", staged: Staged, command: str, sections: list[str]
) -> dict:
    """Per-section render latency in the sandbox, written to the evidence directory.

    Timed inside the sandbox in a single shell so the figure is the binary and not the `sbx exec`
    transport. **These numbers are load-dependent** — two independent passes on the same machine
    differed by roughly a factor of two — so the artifact records every individual run, not just
    the summary, and a reader can see the spread rather than trusting a mean.
    """
    quoted = " ".join(shlex.quote(section) for section in sections)
    script = f"""
B={shlex.quote(sandbox.binary)}
R={shlex.quote(str(staged.plugin))}
for sec in {quoted}; do
  i=0
  while [ $i -lt {LATENCY_RUNS} ]; do
    t0=$(date +%s%N)
    $B rules {shlex.quote(command)} --section "$sec" --plugin-root "$R" >/dev/null 2>&1
    t1=$(date +%s%N)
    echo "RUN $sec $(( (t1 - t0) / 1000000 ))"
    i=$(( i + 1 ))
  done
done
t0=$(date +%s%N)
for sec in {quoted}; do
  $B rules {shlex.quote(command)} --section "$sec" --plugin-root "$R" >/dev/null 2>&1
done
t1=$(date +%s%N)
echo "FIRE $(( (t1 - t0) / 1000000 ))"
"""
    out = runner.sbx_sh(script, timeout=900)
    runs: dict[str, list[int]] = {section: [] for section in sections}
    whole_fire = None
    for line in out.stdout.splitlines():
        parts = line.split()
        if len(parts) == 3 and parts[0] == "RUN" and parts[1] in runs:
            runs[parts[1]].append(int(parts[2]))
        elif len(parts) == 2 and parts[0] == "FIRE":
            whole_fire = int(parts[1])
    per_section = {
        section: {
            "runs_ms": values,
            "mean_ms": round(sum(values) / len(values)) if values else None,
            "max_ms": max(values) if values else None,
        }
        for section, values in runs.items()
    }
    means = [v["mean_ms"] for v in per_section.values() if v["mean_ms"] is not None]
    maxes = [v["max_ms"] for v in per_section.values() if v["max_ms"] is not None]
    payload = {
        "runs_per_section": LATENCY_RUNS,
        "note": "load-dependent; timed inside the sandbox, not across `sbx exec`",
        "per_section": per_section,
        "mean_band_ms": [min(means), max(means)] if means else None,
        "worst_single_run_ms": max(maxes) if maxes else None,
        "whole_fire_ms": whole_fire,
    }
    write_evidence(staged.root, "latency.json", json.dumps(payload, indent=2) + "\n")
    return payload


# What the delivery cases measured, in the order they ran. The lead reads both abort criteria off
# the summary block this feeds; nothing here can change an exit code.
MEASURED: list[dict] = []


def case_delivery(kind: str, name: str):
    """Build the happy-path case for one converted primitive, command or skill.

    The wave-3 pilot case, parameterized twice: at wave 4 across the six commands, and here
    across both kinds. Every command-side assertion is the one `brainstorm-delivery` carried at
    wave 3, so the keying that audit established survives both generalizations; the skill side
    adds the four assertions its own delivery path has and the commands do not.

    What changed at wave 5 is which check gates. Criterion (1) is now `assert_floor_delivery` —
    the floor rules present in the transcript as rendered blocks — and the model's read-back
    became a recorded two-line measurement beside it. The wave-4 evidence is why: three commands
    tripped the read-back bar while every missed id was verifiably delivered, so the bar was
    grading recall of a long list and calling it delivery.

    Three replicates, and every delivery assertion runs on all three rather than on the first: if
    delivery were flaky a single-replicate check would find it only by luck, and flakiness is
    exactly the failure this suite exists for.
    """
    case_name = f"{name}-delivery"

    def run(runner, sandbox: "Sandbox") -> tuple[list, pathlib.Path]:
        staged = stage(case_name, PLUGIN)
        row = expectation(name)
        if row is None or name not in PROBE_ARGUMENTS:
            checks = [
                ok(
                    f"`{name}` has a pre-registered floor set, baseline and probe argument",
                    f"missing for `{name}`: "
                    + ", ".join(
                        what
                        for what, present in (
                            ("a frozen expectation row", row is not None),
                            ("a probe argument", name in PROBE_ARGUMENTS),
                        )
                        if not present
                    )
                    + " — a converted primitive nobody pre-registered cannot be measured",
                )
            ]
            write_verdict(staged.root, case_name, checks, {"shape": "delivery", "kind": kind})
            return checks, staged.root
        ids, counts, reason = primitive_expectations(staged.plugin, name)
        if reason:
            checks = [ok("the delivery expectation could be built from the binary", reason)]
            write_verdict(staged.root, case_name, checks, {"shape": "delivery", "kind": kind})
            return checks, staged.root

        # The floor set this case grades against, read from the render at case time and
        # cross-checked against the freeze rather than taken from it. Both reads of the render are
        # kept: the `floors:` index line, and the walk over the section renders.
        binary, binary_reason = host_binary()
        pin = None if binary_reason else floor_pin(binary, name, staged.plugin)
        listed, walked, disagreement = (
            (None, set(), binary_reason)
            if binary_reason
            else floors_from_render(binary, name, staged.plugin)
        )
        floor_expected = walked if listed is None else listed
        frozen = set(row.floor_ids)
        missing_frozen = sorted(frozen - floor_expected)
        extra_frozen = sorted(floor_expected - frozen)

        path_env = f"{sandbox.binary_dir}:{sandbox.path}"
        # A skill fires through the platform's `Skill` tool, so the model needs one turn to call
        # it before the turn that answers. A command's blocks arrive in the expanded prompt and
        # need no turn of their own. Measured on the wave-5 pre-flight probe, not assumed.
        max_turns = 3 if kind == "skill" else 2
        replicates = []
        for index in range(READ_BACK_REPLICATES):
            probed = run_probe(
                runner,
                staged,
                path_env=path_env,
                log_dir=None,
                prompt=probe_prompt(name),
                max_turns=max_turns,
                tag=f"-{index + 1}",
            )
            session = session_id_of(probed.events)
            seen, transcript_path = fetch_transcript(runner, staged, session, tag=f"-{index + 1}")
            final = final_assistant_text(probed.events)
            scores = score_two_line(final, name, pin)
            blocks = delivered_blocks(seen, name)
            result = result_event(probed.events)
            replicates.append(
                {
                    "index": index + 1,
                    "probed": probed,
                    "seen": seen,
                    "transcript_path": transcript_path,
                    "session_id": session,
                    "scores": scores,
                    "tokens": scores["tokens"],
                    "read_back_passed": scores["ids_exact"],
                    "delivered_chars": sum(len(b) for b in blocks.values()),
                    "delivered_bytes": sum(len(b.encode("utf-8")) for b in blocks.values()),
                    "blocks": sorted(blocks),
                    "floor_ids_delivered": len(transcript_floor_ids(seen) & frozen),
                    "num_turns": (result or {}).get("num_turns"),
                    "final_text": final[:400],
                }
            )

        checks: list[Check] = []
        for check_name, problems in _aggregate(replicates, kind, name, ids, counts):
            checks.append(ok(check_name, problems))

        # --- the no-fallback posture, swept run-wide ------------------------------------------
        #
        # One sweep for the whole case rather than one assertion per replicate: the subject is
        # every channel the three replicates wrote, so a read on replicate 2's transcript fails
        # the case even though replicate 1's stream is clean.
        checks += no_schema_read_checks(staged.root)

        # --- criterion (1), re-keyed: gating -------------------------------------------------
        checks.append(
            ok(
                f"the `floors:` line and the section renders agree ({len(floor_expected)} ids)",
                disagreement,
            )
        )
        checks.append(
            ok(
                "the case-time floor set matches the frozen expectation",
                None
                if not missing_frozen and not extra_frozen
                else f"frozen but not rendered: {missing_frozen}; rendered but not frozen: "
                f"{extra_frozen} — the freeze predates this run and cannot be edited to match it",
            )
        )
        floor_failures = [
            f"replicate {entry['index']}: {problem}"
            for entry in replicates
            if (problem := assert_floor_delivery(entry["seen"], floor_expected))
        ]
        checks.append(
            ok(
                f"every one of the {len(floor_expected)} floor rules was delivered "
                f"(criterion (1))",
                "; ".join(floor_failures) if floor_failures else None,
            )
        )

        # --- how the skill actually arrived, recorded ------------------------------------------
        #
        # Measured at the wave-5 shape probe and recorded per run rather than asserted, because
        # both facts are about the platform's routing rather than about this plugin's contract.
        # A `/mochiko:<skill>` prompt line takes the **prompt-expansion** path: the blocks arrive
        # in the expanded prompt exactly as a command's do, no `Skill` tool call happens, and so
        # neither dependency-hook limb fires — `UserPromptExpansion` looks the name up under
        # `commands/`, and `PreToolUse`/`Skill` is never reached. Wave-0 probe (c) saw the Skill
        # tool fire for a skill, so the routing is prompt-shape-dependent; recording it here means
        # a change in either direction shows up in the evidence instead of silently altering what
        # the suite is testing.
        if kind == "skill":
            paths = {
                "Skill tool" if assert_skill_tool_use(r["probed"].events, name) is None
                else "prompt expansion"
                for r in replicates
            }
            limbs = set()
            for r in replicates:
                for noun in ("skill", "command"):
                    if f"{HOOK_PRESENT_PREFIX} {noun}'s own render" in r["seen"]:
                        limbs.add(noun)
            checks += [
                report("invocation path", ", ".join(sorted(paths))),
                report(
                    "dependency-hook limb that spoke",
                    ", ".join(sorted(limbs))
                    if limbs
                    else "none — no hook gates this path; the halt on absence is the harness's "
                    "(see the absence case)",
                ),
            ]

        # --- the read-back, recorded ----------------------------------------------------------
        scored = sum(1 for r in replicates if r["scores"]["ids_exact"])
        counted = sum(1 for r in replicates if r["scores"]["count_exact"])
        superset = sum(1 for r in replicates if r["scores"]["ids_superset"])
        omitted = sorted({i for r in replicates for i in r["scores"]["omitted"]})
        checks.append(
            report(
                "read-back, two-line form (recorded, never gating)",
                f"count {counted}/{len(replicates)} against the preamble pin {pin} · "
                f"ids {scored}/{len(replicates)} exact · {superset}/{len(replicates)} superset · "
                f"omitted {omitted if omitted else 'none'}",
            )
        )
        baseline = row.baseline_bytes
        delivered = replicates[0]["delivered_bytes"]
        # Wave 3's three replicates agreed to the byte. Across six commands that is worth stating
        # rather than assuming, so a disagreement is named instead of hidden behind the first.
        spread = sorted({r["delivered_bytes"] for r in replicates})
        checks.append(
            report(
                "delivered read cost",
                f"{delivered:,} bytes / {replicates[0]['delivered_chars']:,} chars against the "
                f"{baseline:,}-byte baseline — {(delivered - baseline) / baseline:+.1%} bytes"
                + ("" if len(spread) == 1 else f"; replicates disagree: {spread}"),
            )
        )
        latency = measure_latency(runner, sandbox, staged, name, ids)
        band = latency.get("mean_band_ms")
        checks.append(
            report(
                "store latency (load-dependent)",
                f"per-section mean {band[0]}–{band[1]} ms, worst single run "
                f"{latency['worst_single_run_ms']} ms, whole fire "
                f"{latency['whole_fire_ms']} ms — {LATENCY_RUNS} runs per section, in latency.json"
                if band
                else "not measured",
            )
        )

        frozen_row = (SKILL_FREEZE.get("skills") or {}).get(name) or {}
        read_cost = {
            "kind": kind,
            "family": family_of(name),
            "baseline_bytes": baseline,
            "baseline_source": frozen_row.get(
                "baseline_source", "pre-registered; `wc -c` of <cmd>.yaml + common.yaml"
            ),
            "source": "the session transcript, not the stream",
            "per_replicate": [
                {
                    "index": r["index"],
                    "bytes": r["delivered_bytes"],
                    "chars": r["delivered_chars"],
                    "blocks": r["blocks"],
                    "transcript_path": r["transcript_path"],
                }
                for r in replicates
            ],
        }
        if name == PILOT_COMMAND:
            # The two `brainstorm`-only figures the wave-3 README quotes beside the baseline.
            read_cost["baseline_chars"] = BASELINE_CHARS
            read_cost["baseline_bytes_with_labels"] = BASELINE_BYTES_WITH_LABELS
        if kind == "skill":
            # Delivered-at-invoke on both sides, which is what criterion (2) reads per family
            # (record F3: body + schema). The pre-conversion body comes from the freeze because
            # after the conversion lands `HEAD` no longer carries it; the new body is measured
            # from the staged copy the session actually loaded.
            body_now = (staged.plugin / "skills" / name / "SKILL.md").stat().st_size
            body_pre = frozen_row.get("body_bytes_pre")
            read_cost.update(
                {
                    "body_bytes_new": body_now,
                    "body_bytes_pre": body_pre,
                    "delivered_at_invoke_new": body_now + delivered,
                    "delivered_at_invoke_old": (body_pre + baseline) if body_pre else None,
                }
            )
        MEASURED.append(
            {
                "kind": kind,
                "command": name,
                "family": family_of(name),
                "read_back_scored": scored,
                "read_back_counted": counted,
                "read_back_replicates": READ_BACK_REPLICATES,
                "floor_ids": len(frozen),
                "floor_delivered": min(r["floor_ids_delivered"] for r in replicates),
                "delivered_bytes": delivered,
                "delivered_chars": replicates[0]["delivered_chars"],
                "baseline_bytes": baseline,
                "body_bytes_new": read_cost.get("body_bytes_new"),
                "body_bytes_pre": read_cost.get("body_bytes_pre"),
                "invoke_new": read_cost.get("delivered_at_invoke_new"),
                "invoke_old": read_cost.get("delivered_at_invoke_old"),
            }
        )

        write_verdict(
            staged.root,
            case_name,
            checks,
            {
                "shape": "delivery",
                "kind": kind,
                "command": name,
                "expected_sections": ids,
                "expected_counts": counts,
                "floor_set": {
                    "frozen": sorted(frozen),
                    "floors_line": None if listed is None else sorted(listed),
                    "section_walk": sorted(walked),
                    "preamble_pin": pin,
                    "gating": True,
                },
                "read_back": {
                    "form": "two-line: FLOOR-COUNT then FLOOR",
                    "replicates": READ_BACK_REPLICATES,
                    "ids_exact": scored,
                    "count_exact": counted,
                    "ids_superset": superset,
                    "gating": False,
                    "pre_registered_floor_ids": sorted(frozen),
                    "probe_argument": PROBE_ARGUMENTS[name][0],
                    "probe_argument_why": PROBE_ARGUMENTS[name][1],
                    "per_replicate": [
                        {
                            "index": r["index"],
                            "num_turns": r["num_turns"],
                            "final_text": r["final_text"],
                            **r["scores"],
                        }
                        for r in replicates
                    ],
                },
                "latency": latency,
                "read_cost": read_cost,
            },
        )
        return checks, staged.root

    return run


def _aggregate(replicates: list, kind: str, name: str, ids: list[str],
               counts: dict[str, int]) -> list:
    """Run each delivery assertion over every replicate, reporting which ones failed.

    Two different sources, deliberately. What the model was *given* — the seven blocks and the
    dependency hook's presence line — is read from the session transcript, because the stream
    carries neither: the expanded prompt appears in no stream row, and the `UserPromptExpansion`
    hook produces no stream row at all even when it fires. What the model *did* — tool uses, the
    init event's registries — is read from the stream, which is where those live.

    The no-Read assertion used to sit here, once per replicate, keyed to that replicate's stream.
    From wave 6 it is a single case-level sweep over every channel the case captured, all three
    replicates' streams and transcripts included, so it moved to the caller.

    The skill rows are the ones the command path has no equivalent of. A command's blocks arrive
    in the expanded prompt before any turn; a skill's arrive because the model called the `Skill`
    tool, and the dependency hook meets it on a different limb with a different noun. Both facts
    are asserted rather than assumed, so a skill that silently stopped going through that path —
    or a hook limb that stopped firing — is a failure and not a quiet pass.
    """
    per_replicate = []
    for entry in replicates:
        checks = [
            ok(
                "the session transcript was recovered",
                None
                if entry["seen"]
                else f"no transcript for session {entry['session_id']!r} under "
                "~/.claude/projects/ — every delivery assertion below reads it",
            )
        ]
        checks += assert_delivery(entry["seen"], name, ids, counts) + [
            ok(
                "the SessionStart hook reported the binary",
                _session_start_line(entry["probed"], entry["seen"]),
            ),
            ok(
                "the plugin's six commands registered as slash commands",
                assert_slash_commands(entry["probed"].events, expected_slash_commands()),
            ),
        ]
        # Both kinds, and the history is worth keeping. A converted skill invoked as
        # `/mochiko:<skill>` takes the prompt-expansion path (measured, wave-5 shape probes), and
        # for the length of the wave-5 full run the hook resolved that name against
        # `commands/<name>.md` only — so no limb fired and a converted skill ran ungated. The
        # hook now falls back to the skill file, which is what makes this assertion meaningful
        # for skills; before that fix it could only have been a recorded observation. The noun is
        # asserted either way, so a skill confirmed as a command still fails.
        checks.append(
            ok(
                "the dependency hook confirmed presence in the transcript",
                _hook_presence_line(entry["seen"], kind),
            )
        )
        if kind == "skill":
            checks.append(
                ok(
                    f"the init event registered `mochiko:{name}`",
                    assert_primitive_registered(entry["probed"].events, name),
                )
            )
        per_replicate.append(checks)
    merged = []
    for position, template in enumerate(per_replicate[0]):
        failures = [
            f"replicate {index + 1}: {group[position].detail}"
            for index, group in enumerate(per_replicate)
            if group[position].status == "fail"
        ]
        merged.append((template.name, "; ".join(failures) if failures else None))
    return merged


def _hook_presence_line(transcript: str, kind: str) -> str | None:
    """The dependency hook's confirmation, on the limb this primitive's registration uses.

    The hook writes `… delivered by the <noun>'s own render`, and the noun is the registration:
    `command` on `UserPromptExpansion`, `skill` on `PreToolUse`/`Skill`. Asserting the noun rather
    than the prefix is what makes a skill case fail when the skill somehow arrived down the
    command limb, or when the `PreToolUse` limb stopped firing and only a stale command-side line
    is in the transcript.
    """
    expected = f"{HOOK_PRESENT_PREFIX} {kind}'s own render"
    if expected in transcript:
        return None
    if HOOK_PRESENT_PREFIX in transcript:
        return f"the hook spoke, but not with {expected!r} — the wrong limb confirmed presence"
    return f"{expected!r} is not in the session transcript"


def assert_skill_tool_use(events: list, skill: str) -> str | None:
    """The skill was invoked through the platform's `Skill` tool.

    Wave-0 probe (c) recorded the shape this asserts: a `Skill` tool call carrying
    `tool_input.skill` namespaced as `<plugin>:<skill>`, which is the path a `/mochiko:<skill>`
    prompt line takes and the path the `PreToolUse` hook limb sits on. Asserting it keeps the case
    honest about *how* delivery happened: if the platform ever routed the prompt some other way,
    the blocks might still arrive and the hook limb under test would never have fired.
    """
    wanted = f"mochiko:{skill}"
    for use in tool_uses(events):
        if use.get("name") != "Skill":
            continue
        named = str((use.get("input") or {}).get("skill", ""))
        if named == wanted or named == skill:
            return None
    called = sorted(
        {
            str((use.get("input") or {}).get("skill", ""))
            for use in tool_uses(events)
            if use.get("name") == "Skill"
        }
    )
    return f"no `Skill` tool use named {wanted!r}; the Skill calls in the stream were {called}"


def assert_primitive_registered(events: list, skill: str) -> str | None:
    """The init event lists the skill the case is about.

    The command side has a dedicated field (`slash_commands`) and its own assertion. For skills
    the registry field is whatever the platform reports, so this looks for the namespaced name
    anywhere in the init event and the verdict records the whole event — measured first, then
    narrowed. The wave-0 manifest quirk is the reason a registration check exists at all: a
    directory-string manifest once registered a probe plugin's commands as a single skill.
    """
    init = init_event(events)
    if init is None:
        return "the session produced no init event"
    if f"mochiko:{skill}" in json.dumps(init, ensure_ascii=False):
        return None
    return f"`mochiko:{skill}` appears nowhere in the init event's registries"


def _session_start_line(probed: Probed, transcript: str = "") -> str | None:
    """The SessionStart hook's own line, whichever form it took.

    Three outcomes are all legitimate presence reports — the in-range line, the D5 line, and the
    both-lines fallback — so the assertion is that the hook spoke and named the binary, not that
    it chose one particular branch. Read from the measured channels only: this hook's output does
    reach the stream as `hook_started`/`hook_response` rows, and finding it on the process streams
    instead would not mean the session ever saw it.
    """
    if re.search(
        r"mochiko-cli \d+\.\d+\.\d+ · grammar \d+\.\.\d+", asserted_output(probed, transcript)
    ):
        return None
    return "no SessionStart version line in the session transcript or the stream events"


def case_command_absence(command: str):
    """Build the binary-off-PATH case for one converted command, hooks in play.

    The wave-3 pilot case, parameterized: every assertion is the one `brainstorm-absence` carried,
    with the command name substituted and nothing else changed.

    This is where wave 1's pending assertion resolves. Two limbs can halt this run — the
    `UserPromptExpansion` hook's exit-2 block, which fires *before* expansion, and the `!` line's
    own failure, which fires during it. The hook should win, and the install line should reach the
    user either way; which channel carried it is recorded rather than assumed, because the stream
    shape of a blocked expansion is not measured anywhere yet.
    """
    case_name = f"{command}-absence"

    def run(runner, sandbox: "Sandbox") -> tuple[list, pathlib.Path]:
        staged = stage(case_name, PLUGIN)
        probed = run_probe(
            runner,
            staged,
            path_env=sandbox.path,
            log_dir=None,
            prompt=probe_prompt(command),
            max_turns=2,
        )
        seen, transcript_path = fetch_transcript(runner, staged, session_id_of(probed.events))
        union = session_output_with(probed, seen)
        channels = channels_of(probed, INSTALL_LINE, seen)
        checks = [
            ok("no model turn ran", assert_halt_before_model(probed.events)),
            ok(
                "the install line reached the session",
                assert_in_session(probed, INSTALL_LINE, seen),
            ),
            ok("no version triple was delivered", assert_no_version_triple(union)),
            report("install-line channel", ", ".join(channels) or "none"),
            report(
                "which limb halted first",
                "the dependency hook — the result event carries its block notice and no "
                "`<local-command-stderr>` was injected"
                if str((result_event(probed.events) or {}).get("result") or "").startswith(
                    HOOK_BLOCK_PREFIX
                )
                else "the `!` line — the harness injected its stderr",
            ),
        ]
        checks += no_schema_read_checks(staged.root)
        write_verdict(
            staged.root,
            case_name,
            checks,
            {
                "shape": halt_shape(probed),
                "command": command,
                "channels": channels,
                "transcript_path": transcript_path,
            },
        )
        return checks, staged.root

    return run


def case_skill_absence(skill: str):
    """Build the binary-off-PATH case for one converted skill.

    **Keyed to the measured shape, which changed twice under this case.** The plan expected the
    `PreToolUse`/`Skill` limb to deny the call, leaving a model turn and an error tool result. The
    wave-5 shape probes measured something else: `/mochiko:<skill>` takes the prompt-expansion
    path, and with the hook as it then stood no limb fired at all, so the halt was the wave-1
    harness shape — the `!` line exits non-zero, the harness aborts the expansion and injects the
    shell's stderr. The hook then gained its skill fallback, and the halt moved again: the hook
    now blocks before expansion, so **no `<local-command-stderr>` is injected** and the notice
    rides the result event instead.

    Both shapes are legitimate halts and the case asserts what is true of both — no model turn,
    the install line in the session, no block delivered, no version triple, no schema read. Which
    limb halted is recorded rather than asserted, because that is the part that moved. An
    assertion pinned to the injected stderr went red the moment the hook was fixed, which is the
    keying discipline working rather than failing.
    """
    case_name = f"{skill}-absence"

    def run(runner, sandbox: "Sandbox") -> tuple[list, pathlib.Path]:
        staged = stage(case_name, PLUGIN)
        probed = run_probe(
            runner,
            staged,
            path_env=sandbox.path,
            log_dir=None,
            prompt=probe_prompt(skill),
            max_turns=3,
        )
        seen, transcript_path = fetch_transcript(runner, staged, session_id_of(probed.events))
        union = session_output_with(probed, seen)
        channels = channels_of(probed, INSTALL_LINE, seen)
        delivered = head_line_sections(seen, skill)
        denied = [
            use
            for use in tool_uses(probed.events)
            if use.get("name") == "Skill"
        ]
        checks = [
            ok("no model turn ran", assert_halt_before_model(probed.events)),
            ok(
                "the install line reached the session",
                assert_in_session(probed, INSTALL_LINE, seen),
            ),
            # The halt has to say *which* primitive stopped rather than hand over a generic
            # notice; the hook writes the name into its own message, so this is the session-level
            # twin of the per-skill `hook-input` rows. It only became assertable when the hook
            # gained its skill fallback: before that no limb fired here at all, and the harness's
            # own stderr names the failing shell command rather than the skill.
            ok(
                f"the halt names `/mochiko:{skill}`",
                assert_in_session(probed, f"/mochiko:{skill}", seen),
            ),
            ok("no version triple was delivered", assert_no_version_triple(union)),
            ok(
                "no rendered block reached the model",
                None if not delivered else f"blocks arrived with no binary: {sorted(delivered)}",
            ),
            # Deliberately *not* `assert_halted`: that reads the fixture's own
            # `CONTRACT-PROBE: delivered` marker, which a real skill never prints, so the check
            # would pass without looking at anything.
            #
            # The halt shape itself is asserted, not merely recorded (lead ruling, 2026-09-04,
            # applying the wave-3 V3 ruling on tolerant unions). With the hook's skill fallback in
            # place there is exactly one correct outcome here — the hook blocks before expansion —
            # so accepting the harness shape beside it would be a union that passes on a
            # regression. The pre-fix harness halt is the record on disk, not a second accepted
            # outcome.
            ok("the halt was the dependency hook's block", assert_hook_block(probed)),
            report("install-line channel", ", ".join(channels) or "none"),
            report(
                "which limb carried it",
                "`PreToolUse`/Skill — a Skill call was attempted and refused"
                if denied
                else "`UserPromptExpansion` — the prompt-expansion limb",
            ),
            report("model turns", str((result_event(probed.events) or {}).get("num_turns"))),
        ]
        checks += no_schema_read_checks(staged.root)
        write_verdict(
            staged.root,
            case_name,
            checks,
            {
                "shape": halt_shape(probed),
                "kind": "skill",
                "skill": skill,
                "channels": channels,
                "skill_tool_calls": len(denied),
                "transcript_path": transcript_path,
            },
        )
        return checks, staged.root

    return run


# The preload subject: a plugin agent whose `skills:` frontmatter carries a converted skill, so
# the platform renders that skill into the subagent at spawn (wave-0 probe (d): `!` runs at
# subagent skill preload). `review-specifications` converts in the first family, which is what
# makes this pair runnable from the wave's first landing.
PRELOAD_AGENT = "devils-advocate"
PRELOAD_SKILL = "review-specifications"
PRELOAD_PROMPT = (
    "Dispatch the `mochiko:devils-advocate` subagent with this brief and then stop: "
    "«CONTRACT PROBE — before anything else, reply with exactly one line "
    "`FLOOR-COUNT: <how many class: floor rules you were delivered, as a number>` and stop.» "
    "Report the subagent's reply verbatim."
)


def fetch_sidechain_transcripts(runner, staged: Staged, since: str, tag: str = ""
                                ) -> tuple[str, list[str]]:
    """Every session transcript the sandbox wrote during this case, concatenated.

    A subagent's turns do not live in the parent's stream, and depending on how the platform
    stores them they may not live in the parent's transcript file either. Rather than guess which
    file holds them, this copies out every transcript newer than a marker file created just before
    the run, and the assertion reads the union. Each file lands in the evidence directory, so a
    later reader can see exactly which one carried the delivered blocks.

    `tag` state-tags the filenames. Without it the absent run's fetch overwrites the present run's
    files, and a reader replaying the case from disk reconstructs neither union — the evidence
    silently destroys half of itself (V3-1).
    """
    listing = runner.sbx_sh(
        f"find ~/.claude/projects -name '*.jsonl' -newer {shlex.quote(since)} 2>/dev/null",
        timeout=120,
    )
    paths = [line.strip() for line in listing.stdout.splitlines() if line.strip()]
    chunks, kept = [], []
    for index, path in enumerate(sorted(paths)):
        got = runner.sbx_sh(f"cat {shlex.quote(path)}", timeout=300)
        if got.returncode != 0:
            continue
        write_evidence(staged.root, f"sidechain{tag}-{index + 1}.jsonl", got.stdout)
        chunks.append(transcript_plaintext(got.stdout))
        kept.append(path)
    return "\n".join(chunks), kept


def case_preload(runner, sandbox: "Sandbox") -> tuple[list, pathlib.Path]:
    """The subagent preload path, in both binary states — one case, two sessions.

    D3's skill form has two delivery channels, and the `<skill>-delivery` cases only exercise one
    of them. The other is the preload: a plugin agent declaring `skills:` gets that skill rendered
    into it at spawn, `!` lines and all (wave-0 probe (d)), with no `Skill` tool call and no
    `PreToolUse` limb in the way. If that channel silently stopped delivering, every assertion in
    this suite would still pass while a whole class of runs got no rules.

    The absent half is the fail-closed claim, and it is **measured before it is asserted**. Wave-0
    measured a *denied* `!` line failing the spawn outright; a `command not found` line is a
    different failure and could plausibly leave the spawn alive with an empty block. So the
    gating assertion here is the one that holds either way — nothing was delivered and nothing was
    read as a fallback — and the spawn's own fate is recorded for the wave-5 report to key a
    tighter assertion to.
    """
    staged = stage("preload", PLUGIN)
    checks: list[Check] = []
    agent = staged.plugin / "agents" / f"{PRELOAD_AGENT}.md"
    converted = PRELOAD_SKILL in converted_skills(staged.plugin)
    declared = agent.is_file() and PRELOAD_SKILL in agent.read_text(encoding="utf-8")
    if not declared or not converted:
        checks.append(
            ok(
                f"`{PRELOAD_AGENT}` preloads the converted skill `{PRELOAD_SKILL}`",
                f"agent declares it: {declared}; skill converted: {converted} — the preload case "
                "needs both",
            )
        )
        write_verdict(staged.root, "preload", checks, {"shape": "preload"})
        return checks, staged.root

    binary, reason = host_binary()
    floor_expected = set() if reason else floors_from_render(binary, PRELOAD_SKILL,
                                                             staged.plugin)[1]
    ids, _, _ = primitive_expectations(staged.plugin, PRELOAD_SKILL)
    outcomes = {}
    for state, path_env in (
        ("present", f"{sandbox.binary_dir}:{sandbox.path}"),
        ("absent", sandbox.path),
    ):
        marker = f"/tmp/preload-marker-{uuid.uuid4().hex[:8]}"
        runner.sbx_sh(f"touch {marker}", timeout=60)
        probed = run_probe(
            runner,
            staged,
            path_env=path_env,
            log_dir=None,
            prompt=PRELOAD_PROMPT,
            max_turns=6,
            tag=f"-{state}",
        )
        parent, _ = fetch_transcript(runner, staged, session_id_of(probed.events), tag=f"-{state}")
        side, files = fetch_sidechain_transcripts(runner, staged, marker, tag=f"-{state}")
        runner.sbx_sh(f"rm -f {marker}", timeout=60)
        union = "\n".join([parent, side])
        outcomes[state] = {
            "probed": probed,
            "seen": union,
            "sidechain_files": files,
            "blocks": sorted(head_line_sections(union, PRELOAD_SKILL)),
            "result_text": str((result_event(probed.events) or {}).get("result") or "")[:400],
            "num_turns": (result_event(probed.events) or {}).get("num_turns"),
        }

    present, absent = outcomes["present"], outcomes["absent"]
    checks += [
        ok(
            f"binary present: the subagent received all {len(ids)} blocks of `{PRELOAD_SKILL}`",
            None
            if set(present["blocks"]) >= set(ids)
            else f"delivered {present['blocks']}, expected {ids}",
        ),
        ok(
            f"binary present: every floor rule reached the subagent (criterion (1), "
            f"{len(floor_expected)} ids)",
            assert_floor_delivery(present["seen"], floor_expected),
        ),
        ok(
            "binary absent: nothing was delivered",
            None
            if not absent["blocks"]
            else f"blocks arrived with no binary: {absent['blocks']}",
        ),
        ok(
            "binary absent: no version triple reached either transcript",
            assert_no_version_triple(
                session_output_with(absent["probed"], absent["seen"])
            ),
        ),
        report(
            "binary absent: what became of the spawn",
            f"turns {absent['num_turns']}; result {absent['result_text'][:200]!r}",
        ),
        report(
            "sidechain transcripts fetched",
            f"present {len(present['sidechain_files'])}, absent "
            f"{len(absent['sidechain_files'])} — copied into the evidence directory",
        ),
    ]
    # Both states in one sweep, and this is the case the sweep was widened for: the subagent's
    # turns are in the sidechain files and in neither `probed.events` nor the parent transcript,
    # so the per-state stream assertions this replaces could not have seen a fallback read there.
    checks += no_schema_read_checks(staged.root)
    write_verdict(
        staged.root,
        "preload",
        checks,
        {
            "shape": "preload, two sessions",
            "agent": PRELOAD_AGENT,
            "skill": PRELOAD_SKILL,
            "expected_sections": ids,
            "floor_ids": sorted(floor_expected),
            "present": {k: v for k, v in present.items() if k not in ("probed", "seen")},
            "absent": {k: v for k, v in absent.items() if k not in ("probed", "seen")},
        },
    )
    return checks, staged.root


def case_brainstorm_skew(runner, sandbox: "Sandbox") -> tuple[list, pathlib.Path]:
    """The staged plugin's own log is out of the binary's grammar range.

    The lever is the plugin's own `migrations/`, not `MOCHIKO_MIGRATIONS`: the converted command's
    `!` lines pass `--plugin-root "${CLAUDE_PLUGIN_ROOT}"`, which beats the environment variable
    in the resolution order, so the wave-1 case's lever cannot reach them. The direct run below
    proves the swap took — that the halt is about the log this case built, and not about some
    other log that happened to be broken.
    """
    staged = stage("brainstorm-skew", PLUGIN)
    swap_plugin_log(staged.plugin)
    path_env = f"{sandbox.binary_dir}:{sandbox.path}"

    direct = runner.sbx_sh(
        f"env PATH={shlex.quote(path_env)} mochiko-cli rules {PILOT_COMMAND} "
        f"--section preamble --plugin-root {shlex.quote(str(staged.plugin))}",
        timeout=120,
    )
    write_evidence(
        staged.root,
        "direct-binary.txt",
        f"exit: {direct.returncode}\n--- stdout ---\n{direct.stdout}"
        f"--- stderr ---\n{direct.stderr}",
    )

    probed = run_probe(
        runner,
        staged,
        path_env=path_env,
        log_dir=None,
        prompt=probe_prompt(PILOT_COMMAND),
        max_turns=2,
    )
    seen, transcript_path = fetch_transcript(runner, staged, session_id_of(probed.events))
    text = session_output_with(probed, seen)
    channels = channels_of(probed, "grammar 99", seen)
    checks = [
        ok(
            "the swapped log is what the command's own root resolves to",
            assert_skew_halt_on_stderr(direct, INSTALL_LINE),
        ),
        ok("no model turn ran", assert_halt_before_model(probed.events)),
        ok(
            "the D5 grammar message reached the session",
            assert_in_session(probed, "grammar 99", seen),
        ),
        ok("the install line reached the session", assert_in_session(probed, INSTALL_LINE, seen)),
        ok("no version triple was delivered", assert_no_version_triple(text)),
        report("halt channel", ", ".join(channels) or "none"),
        report(
            "which limb halted first",
            "the dependency hook's range check — the result event carries its block notice and "
            "no `<local-command-stderr>` was injected"
            if str((result_event(probed.events) or {}).get("result") or "").startswith(
                HOOK_BLOCK_PREFIX
            )
            else "the `!` line — the harness injected its stderr",
        ),
    ]
    checks += no_schema_read_checks(staged.root)
    write_verdict(
        staged.root,
        "brainstorm-skew",
        checks,
        {
            "shape": halt_shape(probed),
            "direct_binary_exit": direct.returncode,
            "direct_binary_stderr": direct.stderr.strip(),
            "channels": channels,
            "transcript_path": transcript_path,
        },
    )
    return checks, staged.root


def case_brainstorm_hooks_off(runner, sandbox: "Sandbox") -> tuple[list, pathlib.Path]:
    """Binary absent and hooks disabled: the harness path is the only guard left.

    D7's floor is that a hook which cannot run never blocks anything — fail-open. That is the
    right default and it means the hooks cannot be the only thing standing between a missing
    binary and a run that improvises. With every hook switched off, the wave-1 shape must come
    back: the `!` line fails, the harness injects its stderr, and no model turn happens.
    """
    staged = stage("brainstorm-hooks-off", PLUGIN)
    probed = run_probe(
        runner,
        staged,
        path_env=sandbox.path,
        log_dir=None,
        prompt=probe_prompt(PILOT_COMMAND),
        max_turns=2,
        settings={"disableAllHooks": True},
    )
    # The transcript is fetched even though the halt check is the decisive one. Without it, "no
    # version triple was delivered" would read a text that excludes the single channel measured to
    # carry delivered rules, and so could not fail — a vacuous assertion. It also makes this the
    # last brainstorm case whose evidence a later reader can re-derive from disk.
    seen, transcript_path = fetch_transcript(runner, staged, session_id_of(probed.events))
    union = session_output_with(probed, seen)
    checks = [
        ok("no model turn ran", assert_halt_before_model(probed.events)),
        ok(
            "the harness injected the shell's stderr, naming the missing binary",
            assert_local_command_stderr(probed.events, "mochiko-cli", "command not found"),
        ),
        ok("no version triple was delivered", assert_no_version_triple(union)),
        report(
            "the hooks really were off",
            "absent"
            if HOOK_PRESENT_PREFIX not in union
            else "a hook still spoke — the setting did not take",
        ),
    ]
    checks += no_schema_read_checks(staged.root)
    write_verdict(
        staged.root,
        "brainstorm-hooks-off",
        checks,
        {"shape": halt_shape(probed), "transcript_path": transcript_path},
    )
    return checks, staged.root


def case_brainstorm_policy(runner, sandbox: "Sandbox") -> tuple[list, pathlib.Path]:
    """Shell execution disabled by policy, hooks on. Recorded, never asserted (D8).

    GI-020 declares this environment unsupported, so there is no contract to hold it to and
    nothing here gates. What it is worth knowing is what actually happens: under branch B the hook
    delivers only a presence line, so the placeholder should reach the model and the `.md`'s prose
    halt clause should be the thing that stops the run — the one path in the whole suite where
    that clause is load-bearing. Whether the model obeys it is a fact about the model, which is
    exactly why it is recorded and not asserted.
    """
    staged = stage("brainstorm-policy", PLUGIN)
    probed = run_probe(
        runner,
        staged,
        path_env=f"{sandbox.binary_dir}:{sandbox.path}",
        log_dir=None,
        prompt=probe_prompt(PILOT_COMMAND),
        max_turns=2,
        settings={"disableSkillShellExecution": True},
    )
    seen, transcript_path = fetch_transcript(runner, staged, session_id_of(probed.events))
    text = session_output_with(probed, seen)
    final = final_assistant_text(probed.events)
    placeholder = "[shell command execution disabled by policy]"

    # The command's own halt clause quotes both `[shell command execution disabled by policy]` and
    # `mochiko-cli rules not delivered`, and the clause is part of the expanded prompt. Searching
    # the whole transcript for either phrase would find the instructions rather than the outcome.
    # So: the placeholder is detected by the *absence of rendered blocks* — with shell execution
    # off, no `!` line can produce a version-triple head — and the prose halt is looked for only in
    # what the model itself wrote.
    delivered = head_line_sections(seen, PILOT_COMMAND)
    expected_ids, _, _ = primitive_expectations(staged.plugin, PILOT_COMMAND)
    checks = [
        report("rendered blocks delivered", f"{len(delivered)} of {len(expected_ids)} expected"),
        report(
            "the placeholder text appears in the transcript",
            f"{seen.count(placeholder)} occurrence(s); 1 is the halt clause quoting it",
        ),
        report(
            "the run halted on the prose clause",
            "yes — the model surfaced the not-delivered line"
            if "mochiko-cli rules not delivered" in final
            else ("no model text at all" if not final.strip()
                  else f"no — the model replied {final.strip()[:200]!r}"),
        ),
        report("the hook spoke", "yes" if HOOK_PRESENT_PREFIX in text else "no"),
        report("model turns", str((result_event(probed.events) or {}).get("num_turns"))),
        # Recorded, never asserted, like everything else this case measures — but read through
        # the same run-wide sweep the gating cases use, so what is recorded here is the same fact
        # they assert rather than a narrower one.
        report(
            "a schema file was read as a fallback",
            sweep_evidence(staged.root)[0] or "no",
        ),
    ]
    write_verdict(
        staged.root,
        "brainstorm-policy",
        checks,
        {
            "shape": halt_shape(probed),
            "final_text": final[:600],
            "delivered_sections": sorted(delivered),
            "placeholder_occurrences": seen.count(placeholder),
            "transcript_path": transcript_path,
        },
    )
    return checks, staged.root


BANG_LINE = re.compile(
    r"^!`mochiko-cli rules (\S+) --section (\S+) --plugin-root \"\$\{CLAUDE_PLUGIN_ROOT\}\" 2>&1`$",
    re.M,
)
GRANT = "allowed-tools: Bash(mochiko-cli *)"


def case_converted_shape(runner, sandbox) -> tuple[list, pathlib.Path]:
    """A converted primitive's `!` lines against the section list its own render declares.

    Cheap, and it catches the failure that is most expensive to find any other way: a `.md` that
    enumerates six sections when the schema has seven delivers six blocks, every one of them
    correctly formed, and the only symptom is a rule the model was never given. Nothing in the
    session assertions would call that a failure — they check that what arrived is well-formed,
    not that everything was asked for. The `.md` and the render have to be compared directly, and
    on the host, before a metered run is spent on it.
    """
    staged = stage("converted-shape", PLUGIN)
    checks: list[Check] = []
    binary, reason = host_binary()
    if reason:
        checks.append(ok("a runnable host binary", reason))
        write_verdict(staged.root, "converted-shape", checks, {"shape": "static + binary"})
        return checks, staged.root

    primitives = converted_primitives(staged.plugin)
    checks.append(
        ok(
            "at least one primitive is converted",
            None if primitives else "no primitive carries a `!` rules line; nothing to compare",
        )
    )
    details = []
    for kind, name, path in primitives:
        body = path.read_text(encoding="utf-8")
        asked = [(m.group(1), m.group(2)) for m in BANG_LINE.finditer(body)]
        wrong_primitive = sorted({p for p, _ in asked if p != name})
        checks.append(
            ok(
                f"{kind} `{name}`: every `!` line renders its own primitive",
                None if not wrong_primitive else f"lines ask for {wrong_primitive}",
            )
        )
        preamble = render(binary, name, "preamble", staged.plugin)
        if preamble.returncode != 0:
            checks.append(
                ok(f"{kind} `{name}` renders its preamble",
                   f"exit {preamble.returncode}: {preamble.stderr.strip()[:200]!r}")
            )
            continue
        expected = ["preamble"] + [s.id for s in parse_preamble(preamble.stdout)]
        requested = [section for _, section in asked]
        checks.append(
            ok(
                f"{kind} `{name}`: the `!` lines enumerate every section, in the render's order",
                None
                if requested == expected
                else f"the `.md` asks for {requested}, the render declares {expected}",
            )
        )
        checks.append(
            ok(
                f"{kind} `{name}`: the Bash grant the `!` lines need is in the frontmatter",
                None if GRANT in body else f"no {GRANT!r} in {path.name}",
            )
        )
        details.append({"primitive": name, "kind": kind, "requested": requested,
                        "declared": expected})

    # Every read-back bar's pre-registered floor set against what the binary actually renders.
    #
    # Run for each command in `EXPECTED`, not only the converted ones: the render comes from the
    # migration log rather than from the `.md`, so a command's bar can be checked before its
    # conversion lands — which is what lets a wave validate its constants before spending a
    # metered session on them.
    unregistered = sorted(
        (set(converted_commands(staged.plugin)) | set(converted_skills(staged.plugin)))
        - set(EXPECTED)
        - set(EXPECTED_SKILLS)
    )
    checks.append(
        ok(
            "every converted primitive has a pre-registered floor set and baseline",
            None
            if not unregistered
            else f"converted with no frozen row: {unregistered} — their delivery cases have "
            "nothing to grade criterion (1) against",
        )
    )
    unargued = sorted(
        (set(converted_commands(staged.plugin)) | set(converted_skills(staged.plugin)))
        - set(PROBE_ARGUMENTS)
    )
    checks.append(
        ok(
            "every converted primitive has a pre-registered probe argument",
            None
            if not unargued
            else f"converted with no PROBE_ARGUMENTS row: {unargued}",
        )
    )
    # Both tables, and both reads of the render. The floor sets are the subject of criterion (1),
    # so a set that drifted from what the binary renders would leave the gating assertion grading
    # a stale list; the `floors:` line is the cheap read the `.md` cites, so its agreement with
    # the rule bodies is checked here rather than trusted.
    frozen_all = {**{k: v for k, v in EXPECTED.items()}, **EXPECTED_SKILLS}
    floor_report = {}
    for primitive in sorted(frozen_all):
        listed, walked, disagreement = floors_from_render(binary, primitive, staged.plugin)
        pre_registered = frozen_all[primitive].floor_ids
        missing = sorted(set(pre_registered) - walked)
        extra = sorted(walked - set(pre_registered))
        checks.append(
            ok(
                f"the pre-registered floor set matches the {primitive} render "
                f"({len(pre_registered)} ids)",
                None
                if not missing and not extra
                else f"pre-registered but not rendered: {missing}; rendered but not "
                f"pre-registered: {extra} — criterion (1) would grade the wrong set",
            )
        )
        checks.append(
            ok(
                f"{primitive}: the `floors:` line agrees with the section renders",
                disagreement
                if disagreement
                else (
                    None
                    if listed is not None
                    else "the preamble carries no `floors:` line — the binary predates the "
                    "wave-5 render change"
                ),
            )
        )
        floor_report[primitive] = {
            "pre_registered": sorted(pre_registered),
            "rendered": sorted(walked),
            "floors_line": None if listed is None else sorted(listed),
        }
    # The freeze has to predate the run it grades. This reports the two mtimes rather than
    # asserting an ordering the filesystem can only weakly attest; the durable proof is the commit
    # that added the file preceding every session in git history, which an auditor reads there.
    if EXPECTED_SKILLS_FILE.is_file():
        # Against *this run's* start, not against the oldest directory in `evals/.work/`: that
        # directory accumulates across waves, so the earliest thing in it is a wave-4 leftover and
        # comparing to it would report a freeze that predates this run as out of order. The
        # earliest directory this run staged is named beside it for an auditor to check.
        frozen_at = EXPECTED_SKILLS_FILE.stat().st_mtime
        mine = sorted(STAGED_ROOTS, key=lambda p: p.stat().st_mtime)
        checks.append(
            report(
                "the skill freeze predates this run",
                f"frozen {_stamp(frozen_at)}, run started {_stamp(RUN_STARTED)} — "
                f"{'ordered' if frozen_at < RUN_STARTED else 'NOT ordered'}"
                + (f"; this run's earliest evidence directory is {mine[0].name}" if mine else "")
                + ". The durable proof is the commit adding the freeze preceding every session "
                "in git history; this is the cheap check beside it.",
            )
        )
    write_verdict(
        staged.root,
        "converted-shape",
        checks,
        {
            "shape": "static + binary",
            "primitives": details,
            "floor_ids": floor_report,
        },
    )
    return checks, staged.root


def render_shape_problems(proc: subprocess.CompletedProcess) -> list[str]:
    """What every rendered deliverable owes, whichever command produced it."""
    problems = []
    if proc.returncode != 0:
        problems.append(f"exit {proc.returncode}")
    if proc.stderr.strip():
        problems.append(f"stderr was not empty: {proc.stderr.strip()[:120]!r}")
    if not proc.stdout.strip():
        problems.append("stdout was empty")
    return problems


def set_delta(found: list, expected: tuple) -> str | None:
    missing = sorted(set(expected) - set(found))
    extra = sorted(set(found) - set(expected))
    if not missing and not extra:
        return None
    return f"written down but not emitted: {missing}; emitted but not written down: {extra}"


def case_deliverables(runner, sandbox) -> tuple[list, pathlib.Path]:
    """Every non-rule document the deleted schema files used to serve, through its CLI form.

    Wave 6's done condition is that no schema file ships, which is only half a contract: the other
    half is that everything those files delivered still has a way of being delivered. Rules had
    one from wave 3. The artifact templates got `mochiko-cli template` at the template-schema
    wave. The shelf document and the two label registries had no CLI form at all until
    `mochiko-cli doc` (record D9 wave 6), and a primitive citing one of those was, until this
    case, pointing at a file about to be deleted with nothing checking that the replacement
    worked. This is the check that the replacement works, on the host, for nothing.

    **The subjects are discovered, not written down.** The binary's own view emitter names every
    document in the replayed state, so the case walks what the log actually holds rather than a
    list that would quietly go stale — the doctrine `converted_commands()` already follows for the
    session cases. A written-down set is then compared against it in both directions, for the
    opposite reason: discovery alone would shrink silently if a document vanished from the log,
    and a case that gets smaller when the thing it guards disappears is not a guard.

    **The two commands are asserted to their own shapes, not to a shared one.** `doc` wraps its
    document in the version triple, head line and end line, exactly as `rules` does. `template`
    does not and did not: it opens on the document's own title and closes on the provenance
    footer, because its output is read as a document rather than as a delivery envelope. Wrapping
    it would be a change to the render output shape and a `mochiko-cli` release concern (GI-012),
    not something for a test to presume; the lead booked it as a follow-up at the wave-6 approval.
    So each command is held to what it emits.
    """
    staged = stage("deliverables", PLUGIN)
    checks: list[Check] = []
    binary, reason = host_binary()
    if reason:
        checks.append(ok("a runnable host binary", reason))
        write_verdict(staged.root, "deliverables", checks, {"shape": "direct binary"})
        return checks, staged.root

    views = staged.root / "views"
    emitted = host_sh(
        f"{shlex.quote(binary)} views emit --out {shlex.quote(str(views))} "
        f"--plugin-root {shlex.quote(str(staged.plugin))}"
    )
    checks.append(
        ok(
            "the view emitter wrote a tree to discover the documents from",
            None
            if emitted.returncode == 0
            else f"exit {emitted.returncode}: {emitted.stderr.strip()[:200]!r}",
        )
    )
    if emitted.returncode != 0:
        write_verdict(staged.root, "deliverables", checks, {"shape": "direct binary"})
        return checks, staged.root

    found = {
        kind: sorted(path.stem for path in (views / kind).glob("*.yaml"))
        for kind in ("templates", "shelves", "labels")
    }
    templates = found["templates"]
    documents = sorted(found["shelves"] + found["labels"])
    checks += [
        ok("every template in the log is one this case walks", set_delta(templates, TEMPLATE_NAMES)),
        ok(
            "every shelf and registry in the log is one this case walks",
            set_delta(documents, DOC_NAMES),
        ),
    ]

    rendered = []
    for name in templates:
        for flag, view in (("", "producer"), ("--check", "checklist")):
            proc = host_sh(
                f"{shlex.quote(binary)} template {shlex.quote(name)} {flag} "
                f"--plugin-root {shlex.quote(str(staged.plugin))}"
            )
            problems = render_shape_problems(proc)
            lines = [line for line in proc.stdout.splitlines() if line.strip()]
            if lines and not lines[0].startswith("# "):
                problems.append(f"the first line is not the document's title: {lines[0][:80]!r}")
            if lines and not lines[-1].startswith(TEMPLATE_FOOTER):
                problems.append(f"the last line is not the provenance footer: {lines[-1][:80]!r}")
            checks.append(
                ok(f"template `{name}` renders its {view} view", "; ".join(problems) or None)
            )
            rendered.append({"command": "template", "name": name, "view": view,
                             "exit": proc.returncode, "bytes": len(proc.stdout.encode("utf-8"))})

    for name in documents:
        proc = host_sh(
            f"{shlex.quote(binary)} doc {shlex.quote(name)} "
            f"--plugin-root {shlex.quote(str(staged.plugin))}"
        )
        problems = render_shape_problems(proc)
        lines = [line for line in proc.stdout.splitlines() if line.strip()]
        head = lines[0] if lines else ""
        if not head.startswith(f"{DOC_HEAD}{name} ·") or not all(
            marker in head for marker in TRIPLE_MARKERS
        ):
            problems.append(f"the head line is not the version triple: {head[:120]!r}")
        if not lines or lines[-1] != f"{DOC_END}{name}":
            problems.append(
                f"the end line is not {DOC_END + name!r}: {(lines[-1] if lines else '')[:120]!r}"
            )
        checks.append(
            ok(f"document `{name}` renders with both lines", "; ".join(problems) or None)
        )
        rendered.append({"command": "doc", "name": name, "view": "document",
                         "exit": proc.returncode, "bytes": len(proc.stdout.encode("utf-8"))})

    checks.append(
        report(
            "deliverables rendered",
            f"{len(templates)} templates × 2 views through `template`, {len(documents)} "
            f"through `doc` — {len(rendered)} invocations, no session",
        )
    )
    write_verdict(
        staged.root,
        "deliverables",
        checks,
        {
            "shape": "direct binary",
            "discovered": found,
            "written_down": {"templates": list(TEMPLATE_NAMES), "documents": list(DOC_NAMES)},
            "rendered": rendered,
        },
    )
    return checks, staged.root


HOST_CASES = [
    ("hook-input", "the hook scripts, fed captured stdin — no sandbox, no session", case_hook_input),
    ("converted-shape", "a converted `.md`'s `!` lines against its own render", case_converted_shape),
    ("render-ceiling", "every converted render against the inline ceiling", case_render_ceiling),
    ("deliverables", "every template, shelf and registry through its CLI form", case_deliverables),
]

def build_sandbox_cases() -> list:
    """The sandbox case list, built from the converted set rather than written down.

    Two fixture cases, then a delivery and an absence case per converted command. The pilot also
    carries the three mechanism cases — skew, hooks-off and policy — which are *not* repeated per
    command: they exercise how delivery behaves when the log, the hooks or the shell is broken,
    and none of that varies with which command fired. Repeating them would buy metered sessions
    and no new fact (wave-4 plan §4).

    A command with no row in `EXPECTED` is listed with its cases anyway and fails loudly at run
    time rather than being silently skipped: a converted command nobody pre-registered a bar for
    is a gap in the wave, not a case to drop.
    """
    cases = [
        ("absence", "[fixture] the binary is off PATH — the run halts, nothing delivered",
         case_absence),
        ("skew", "[fixture] the log's grammar is out of range — the D5 halt fires", case_skew),
    ]
    for command in converted_commands(PLUGIN):
        cases.append(
            (f"{command}-delivery",
             f"`/mochiko:{command}` delivers every block its render declares",
             case_delivery("command", command))
        )
        cases.append(
            (f"{command}-absence", "no binary, hooks on — the install line reaches the user",
             case_command_absence(command))
        )
        if command == PILOT_COMMAND:
            cases += [
                (f"{command}-skew", "the staged plugin's own log is out of range",
                 case_brainstorm_skew),
                (f"{command}-hooks-off", "no binary, hooks off — the harness is the only guard",
                 case_brainstorm_hooks_off),
                (f"{command}-policy", "shell execution disabled by policy — recorded, not asserted",
                 case_brainstorm_policy),
            ]
    # The skill family, in the conversion order the freeze records. Same two cases per primitive
    # as the commands and for the same reason — what it delivers, and what it does when delivery
    # is impossible — with the mechanism cases still run once against the pilot. The preload case
    # is the one that is neither: it covers the *other* skill delivery channel, once, because the
    # channel does not vary with which skill rides it.
    skills = converted_skills(PLUGIN)
    for skill in skills:
        cases.append(
            (f"{skill}-delivery",
             f"`/mochiko:{skill}` delivers every block its render declares",
             case_delivery("skill", skill))
        )
        cases.append(
            (f"{skill}-absence", "no binary, hooks on — the Skill call is denied with the "
             "install line", case_skill_absence(skill))
        )
    if PRELOAD_SKILL in skills:
        cases.append(
            ("preload", f"`{PRELOAD_AGENT}` preloads `{PRELOAD_SKILL}` — both binary states",
             case_preload)
        )
    return cases


SANDBOX_CASES = build_sandbox_cases()

CASES = HOST_CASES + SANDBOX_CASES


# ---------------------------------------------------------------------------
# main
# ---------------------------------------------------------------------------

def run_cases(cases: list, runner, sandbox) -> tuple[int, int, int]:
    """Run a case list, printing each check. Returns (failures, pendings, reports)."""
    failures = pendings = reports = 0
    for name, _, case in cases:
        checks, evidence = case(runner, sandbox)
        failed = [c for c in checks if c.status == "fail"]
        pendings += sum(1 for c in checks if c.status == "pending")
        reports += sum(1 for c in checks if c.status == "report")
        print(f"{'FAIL' if failed else 'ok  '}  {name}")
        for check in checks:
            mark = {"ok": "ok", "fail": "FAIL", "pending": "pend", "report": "rec "}[check.status]
            detail = f" — {check.detail}" if check.detail else ""
            print(f"        {mark:4}  {check.name}{detail}")
        print(f"        evidence: {evidence.relative_to(REPO)}")
        if failed:
            failures += 1
    return failures, pendings, reports


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--list", action="store_true", help="print the case list and exit")
    parser.add_argument(
        "--host-only",
        action="store_true",
        help="run only the cases that need no sandbox and no session",
    )
    args = parser.parse_args()

    declared = HOST_CASES if args.host_only else CASES
    scope = "host cases only" if args.host_only else "all cases"
    print(f"mochiko contract suite · declared cases ({scope}):")
    for name, description, _ in declared:
        print(f"  {name:22s} {description}")
    if args.list:
        return EXIT_OK

    # Exit 0 means every declared case ran. A suite with nothing to run has proved nothing, so
    # it skips rather than reporting a clean sweep of zero.
    if not declared:
        print("\nSKIPPED: the suite declares no cases")
        print("exit 3 — the suite did not run, so nothing here is evidence of anything.")
        return EXIT_SKIP

    WORK.mkdir(parents=True, exist_ok=True)

    if args.host_only:
        print()
        failures, pendings, reports = run_cases(HOST_CASES, None, None)
        summarize(len(HOST_CASES), failures, pendings, reports)
        return EXIT_ASSERT if failures else EXIT_OK

    # The host cases run first: they are free, they need nothing built, and a broken hook script
    # or an undeliverable template should be visible before twenty minutes of sandbox build and a
    # hundred and fifty-one metered sessions.
    print()
    failures, pendings, reports = run_cases(HOST_CASES, None, None)

    runner = load_runner()

    def skipped(reason: str) -> int:
        # A failed assertion outranks a skip. The host cases really ran, and what they found is
        # evidence whether or not the sandbox is reachable.
        print(f"\nSKIPPED (sandbox cases): {reason}")
        summarize(len(HOST_CASES), failures, pendings, reports)
        if failures:
            print("exit 1 — a host case failed; the sandbox cases did not run.")
            return EXIT_ASSERT
        print("exit 3 — the sandbox cases did not run, so they are evidence of nothing.")
        return EXIT_SKIP

    reason = preflight(runner)
    if reason:
        return skipped(reason)
    binary_path, reason = build_binary(runner)
    if reason:
        return skipped(reason)
    path_value, reason = sandbox_path(runner)
    if reason:
        return skipped(reason)

    sandbox = Sandbox(
        path=path_value,
        binary=binary_path,
        binary_dir=str(pathlib.PurePosixPath(binary_path).parent),
    )
    more = run_cases(SANDBOX_CASES, runner, sandbox)
    failures, pendings, reports = (a + b for a, b in zip((failures, pendings, reports), more))
    summarize(len(CASES), failures, pendings, reports)
    return EXIT_ASSERT if failures else EXIT_OK


def summarize(ran: int, failures: int, pendings: int, reports: int) -> None:
    print(f"\ncontract suite: {ran - failures}/{ran} cases passed, {ran} ran", end="")
    if pendings:
        print(f", {pendings} assertion(s) pending a later wave", end="")
    if reports:
        print(f", {reports} measurement(s) recorded and not asserted", end="")
    print()
    print_measured()


def print_measured() -> None:
    """The per-primitive block, and the per-family aggregate criterion (2) is read at.

    What is printed here changed at wave 5, and the change is the point. Criterion (1) is no
    longer in this table: it became `assert_floor_delivery`, a gating check inside each delivery
    case, so a floor rule that failed to arrive fails the run rather than printing a number the
    lead has to notice. The read-back stays as a recorded measurement of a different thing —
    whether the model can enumerate what it was given — and names no criterion.

    Criterion (2) is read per family for the skills, as the wave open ruled, so the aggregate rows
    are what the lead reads: delivered-at-invoke against the pre-conversion figure, in bytes.
    Nothing here can change an exit code; every figure reached a check list through `report()`.
    """
    if not MEASURED:
        return
    print("\nper-primitive measurements (reported, never gating):")
    print(f"  {'primitive':32s} {'read-back':>16s}  {'floors':>7s}  {'delivered':>10s}  "
          f"{'baseline':>9s}  {'delta':>7s}")
    for row in MEASURED:
        delivered, baseline = row["delivered_bytes"], row["baseline_bytes"]
        delta = (delivered - baseline) / baseline if baseline else 0.0
        n = row["read_back_replicates"]
        print(
            f"  {row['command']:32s} "
            f"{row['read_back_counted']}/{n} ct {row['read_back_scored']}/{n} id  "
            f"{row['floor_delivered']:>3d}/{row['floor_ids']:<3d}  "
            f"{delivered:>10,}  {baseline:>9,}  {delta:>+7.1%}"
        )

    families = {}
    for row in MEASURED:
        if row["kind"] != "skill" or not row["invoke_old"]:
            continue
        entry = families.setdefault(row["family"], {"n": 0, "new": 0, "old": 0, "chars": 0})
        entry["n"] += 1
        entry["new"] += row["invoke_new"]
        entry["old"] += row["invoke_old"]
        entry["chars"] += row["delivered_chars"] + (row["body_bytes_new"] or 0)
    if not families:
        return
    print("\nskill families — delivered-at-invoke, body + render (criterion (2), per family):")
    print(f"  {'family':12s} {'skills':>6s}  {'converted':>10s}  {'pre-conversion':>14s}  "
          f"{'delta':>7s}")
    for family, entry in families.items():
        delta = (entry["new"] - entry["old"]) / entry["old"] if entry["old"] else 0.0
        print(
            f"  {family:12s} {entry['n']:>6d}  {entry['new']:>10,}  {entry['old']:>14,}  "
            f"{delta:>+7.1%}"
        )
    print(
        "  bytes throughout; the record's F3 family figures are chars and are compared "
        "separately in the wave report."
    )


if __name__ == "__main__":
    sys.exit(main())
