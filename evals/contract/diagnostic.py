#!/usr/bin/env python3
"""The read-back diagnostic — separates enumeration recall from delivery.

Provenance: `.mochiko/brainstorms/cli-schema-delivery/wave4-plan.md` §8, pre-registered
2026-09-04 after the user ruled "land + diagnostic" on the wave-4 trip. Abort criterion (1) tripped
on `implement` 1/3, `setup` 1/3 and `specify` 0/3 while every deterministic assertion passed and
every missed floor rule was present verbatim, with its `class: floor` line, in the transcript the
model read. Two readings fit that evidence — the model was not given the rules, or it was given
them and could not enumerate them back — and the wave-4 probe cannot tell them apart.

This runner is **not** part of the contract suite and gates nothing. It imports `run.py`'s
helpers and modifies nothing in it: the suite is the thing under audit, and a diagnostic that
edited its subject would be worthless. Everything here is recorded.

Two changes from the delivery probe, both fixed before the first session:

1. **A probe argument every Entry gate accepts.** At wave 4 the instruction text *was* the whole
   argument, and one `implement` replicate refused to answer because it read its own argument as
   suspicious. Here the argument *begins* with a neutral, gate-valid token per command, and
   `ARGUMENTS` records why each token passes its command's Entry step.

   Plan §8 asked for the instruction to move "out of `$ARGUMENTS` into the prompt after the
   command". **Two probes on 2026-09-04 found that is not available in a headless run.**
   `$ARGUMENTS` takes everything after the command name, newlines included, so the `after` shape
   still substitutes the instruction into the command body — seven occurrences in `implement`'s
   transcript, at both of its `$ARGUMENTS` sites. The `before` shape keeps it out, but then the
   prompt no longer starts with `/`, the command never expands, and nothing is delivered. What
   change (1) can actually do is put a gate-valid token at the front of the argument, and that is
   what this runner does.

   The `before` shape is kept as a **negative control**, and it earned its place: with zero blocks
   delivered the model answered `FLOOR-COUNT: 0` and named no ids. The read-back line tracks what
   was delivered rather than what the model can guess about a command it knows, which is the
   assumption every reading of this diagnostic rests on.
2. **Two read-back lines.** `FLOOR-COUNT: <N>` first, `FLOOR: <ids>` second. A model that knows
   how many floor rules it holds but cannot list them is failing enumeration, not delivery, and
   one line cannot show that.

Scoring per replicate, four ways: the count matches the preamble's own `class: floor` pin · the id
set is exactly the pre-registered set · the id set is a superset of it (extras tolerated) · which
ids were omitted.

    python3 evals/contract/diagnostic.py --list
    python3 evals/contract/diagnostic.py --probe implement   # one replicate, argument acceptance
    python3 evals/contract/diagnostic.py                     # 6 commands x 3 replicates
"""

import argparse
import importlib.util
import json
import pathlib
import re
import shutil
import sys
import uuid

HERE = pathlib.Path(__file__).resolve().parent


def load_suite():
    """Import `run.py` for its helpers. Never a second copy of them, and never an edit to it."""
    spec = importlib.util.spec_from_file_location("contract_run", HERE / "run.py")
    if spec is None or spec.loader is None:
        raise RuntimeError(f"{HERE / 'run.py'} cannot be imported")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


suite = load_suite()

REPLICATES = 3
MAX_TURNS = 2

# The gate-valid argument per command, and the Entry step that accepts it. Read from each
# command's own Adaptive Goal Protocol step 1 before the first session (plan §8: "verify each
# argument passes its Entry gate by reading the command's Entry step before running").
#
# The point is a run that reaches a natural first question quickly without the argument itself
# being the thing under test. A one-word topic does that everywhere except `implement`, whose
# Entry gates on a capability entry rather than free text; there the argument is a delta-scope
# card path that does not exist, so Entry takes the routing branch it already carries instead of
# validating a feature id.
ARGUMENTS = {
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
        "delta card. `$ARGUMENTS` is otherwise a capability id. A delta-card path that does not "
        "exist is neither, so Entry takes its own routing branch — 'a feature-keyed delta to "
        "/mochiko:feature' — rather than validating a FEAT id or proposing one.",
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
}

INSTRUCTION = (
    "DIAGNOSTIC — before your first question, and before anything else, reply with exactly "
    "these two lines and then stop:\n"
    "FLOOR-COUNT: <how many class: floor rules you were delivered, as a number>\n"
    "FLOOR: <every class: floor rule id you were delivered, comma-separated>"
)

COUNT_LINE = re.compile(r"^\s*\**FLOOR-COUNT:\**\s*(.*)$", re.M)


SHAPES = ("after", "before")


def prompt_for(command: str, shape: str = "after") -> str:
    """The command, its gate-valid argument, and where the instruction sits relative to them.

    **Measured, not assumed.** `$ARGUMENTS` takes everything after the command name, newlines
    included — the probe of 2026-09-04 found the whole instruction substituted into `implement`'s
    Entry prose at both of its `$ARGUMENTS` sites. So `after` does not put the instruction outside
    the argument; what it does change, and what plan §8 actually needs, is that the argument now
    *begins* with a token Entry accepts, instead of being nothing but instruction text.

    `before` is the only other shape available in a headless `claude -p` run, and it trades the
    substitution for a prompt that no longer starts with `/`. Whether the command still expands
    then is exactly the sort of thing this suite measures rather than reasons about.
    """
    argument = ARGUMENTS[command][0]
    if shape == "before":
        return f"{INSTRUCTION}\n\n/mochiko:{command} {argument}"
    return f"/mochiko:{command} {argument}\n\n{INSTRUCTION}"


def floor_pin(binary: str, command: str, plugin: pathlib.Path) -> int | None:
    """The `class: floor` count the preamble itself pins, which is what the count line is graded
    against. Read from the render rather than from `len(EXPECTED[...])` so the two sides of the
    comparison cannot be the same number by construction."""
    out = suite.render(binary, command, "preamble", plugin)
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


def score(text: str, command: str, pin: int | None) -> dict:
    """One replicate's two lines, graded four ways. Nothing here gates anything."""
    expected = set(suite.EXPECTED[command].floor_ids)

    count_match = COUNT_LINE.search(text)
    raw_count = count_match.group(1).strip() if count_match else ""
    digits = re.search(r"\d+", raw_count)
    named_count = int(digits.group()) if digits else None

    tokens, ids_exact = suite.score_read_back(text, command)
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


def stage(command: str):
    """One case's directory: the staged plugin, and where its evidence lands.

    Mirrors the suite's staging, under a `diagnostic-` prefix so this runner's evidence never
    mixes with `contract-*` directories a suite run or its audit reads.
    """
    root = suite.WORK / f"diagnostic-{command}-{uuid.uuid4().hex[:8]}"
    root.mkdir(parents=True, exist_ok=True)
    plugin = root / suite.PLUGIN.name
    shutil.copytree(suite.PLUGIN, plugin)
    return suite.Staged(root, plugin)


def replicate(runner, sandbox, staged, command: str, pin: int, index: int,
              shape: str = "after") -> dict:
    """One session, its evidence on disk, and its two lines graded."""
    tag = f"-{index}"
    probed = suite.run_probe(
        runner,
        staged,
        path_env=f"{sandbox.binary_dir}:{sandbox.path}",
        log_dir=None,
        prompt=prompt_for(command, shape),
        max_turns=MAX_TURNS,
        tag=tag,
    )
    session = suite.session_id_of(probed.events)
    seen, transcript_path = suite.fetch_transcript(runner, staged, session, tag=tag)
    final = suite.final_assistant_text(probed.events)
    result = suite.result_event(probed.events)
    blocks = suite.delivered_blocks(seen, command)

    row = score(final, command, pin)
    row.update(
        {
            "index": index,
            "session_id": session,
            "transcript_path": transcript_path,
            "num_turns": (result or {}).get("num_turns"),
            "final_text": final[:600],
            # Delivery, measured beside recall on the same session — the whole point of the
            # diagnostic is that these two are separable, so both are recorded per replicate.
            "blocks_delivered": len(blocks),
            "delivered_bytes": sum(len(b.encode("utf-8")) for b in blocks.values()),
            "omitted_ids_in_transcript": {
                rule_id: (rule_id in seen) for rule_id in row["omitted"]
            },
            # `$ARGUMENTS` substitution shows up as repeats of the instruction inside the expanded
            # command body; one occurrence means the instruction stayed out of the argument.
            "instruction_occurrences_in_transcript": seen.count("FLOOR-COUNT:"),
        }
    )
    return row


def run_command(runner, sandbox, command: str, replicates: int,
                shape: str = "after") -> tuple[dict, pathlib.Path]:
    staged = stage(command)
    binary, reason = suite.host_binary()
    pin = None if reason else floor_pin(binary, command, staged.plugin)

    rows = [replicate(runner, sandbox, staged, command, pin, i + 1, shape)
            for i in range(replicates)]
    summary = {
        "command": command,
        "argument": ARGUMENTS[command][0],
        "entry_gate": ARGUMENTS[command][1],
        "shape": shape,
        "prompt": prompt_for(command, shape),
        "replicates": len(rows),
        "floor_pin": pin,
        "pre_registered_ids": len(suite.EXPECTED[command].floor_ids),
        "count_exact": sum(1 for r in rows if r["count_exact"]),
        "ids_exact": sum(1 for r in rows if r["ids_exact"]),
        "ids_superset": sum(1 for r in rows if r["ids_superset"]),
        "gating": False,
        "per_replicate": rows,
    }
    suite.write_evidence(
        staged.root, "diagnostic.json", json.dumps(summary, indent=2) + "\n"
    )
    return summary, staged.root


def print_row(s: dict) -> None:
    n = s["replicates"]
    omitted = sorted({i for r in s["per_replicate"] for i in r["omitted"]})
    print(
        f"  {s['command']:14s} count {s['count_exact']}/{n}   ids {s['ids_exact']}/{n}   "
        f"superset {s['ids_superset']}/{n}   pin {s['floor_pin']}   "
        f"omitted {omitted if omitted else 'none'}"
    )


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--list", action="store_true", help="print the plan and exit")
    parser.add_argument("--probe", metavar="COMMAND",
                        help="one replicate of COMMAND, to check argument acceptance first")
    parser.add_argument("--shape", choices=SHAPES, default="after",
                        help="where the read-back instruction sits relative to the command")
    args = parser.parse_args()

    commands = suite.converted_commands(suite.PLUGIN)
    unknown = [c for c in commands if c not in ARGUMENTS]
    if unknown:
        print(f"no pre-registered Entry argument for {unknown}", file=sys.stderr)
        return 2

    print("read-back diagnostic · wave-4 plan §8 · recorded, never gating")
    for command in commands:
        arg, why = ARGUMENTS[command]
        print(f"  {command:14s} argument {arg!r}")
        if args.list:
            print(f"                 {why}")
    if args.list:
        print(f"\n{len(commands)} commands x {REPLICATES} replicates = "
              f"{len(commands) * REPLICATES} sessions")
        return 0

    if args.probe and args.probe not in ARGUMENTS:
        print(f"unknown command {args.probe!r}", file=sys.stderr)
        return 2

    suite.WORK.mkdir(parents=True, exist_ok=True)
    runner = suite.load_runner()
    reason = suite.preflight(runner)
    if reason:
        print(f"\nSKIPPED: {reason}")
        return 3
    binary_path, reason = suite.build_binary(runner)
    if reason:
        print(f"\nSKIPPED: {reason}")
        return 3
    path_value, reason = suite.sandbox_path(runner)
    if reason:
        print(f"\nSKIPPED: {reason}")
        return 3
    sandbox = suite.Sandbox(
        path=path_value,
        binary=binary_path,
        binary_dir=str(pathlib.PurePosixPath(binary_path).parent),
    )

    if args.probe:
        print(f"\nprobe: one replicate of `{args.probe}`, shape {args.shape!r}")
        summary, evidence = run_command(runner, sandbox, args.probe, 1, args.shape)
        row = summary["per_replicate"][0]
        print_row(summary)
        print(f"\n  argument            {summary['argument']!r}")
        print(f"  instruction seen    {row['instruction_occurrences_in_transcript']}x in the "
              f"transcript (1 = it stayed out of $ARGUMENTS)")
        print(f"  blocks delivered    {row['blocks_delivered']}")
        print(f"  model turns         {row['num_turns']}")
        print(f"  count line          {row['count_line']!r}")
        print(f"  ids named           {row['ids_named']} of "
              f"{summary['pre_registered_ids']}")
        print(f"  final text          {row['final_text'][:300]!r}")
        print(f"  evidence            {evidence.relative_to(suite.REPO)}")
        return 0

    print(f"\n{len(commands)} commands x {REPLICATES} replicates")
    summaries, evidence_dirs = [], {}
    for command in commands:
        summary, evidence = run_command(runner, sandbox, command, REPLICATES, args.shape)
        summaries.append(summary)
        evidence_dirs[command] = str(evidence.relative_to(suite.REPO))
        print_row(summary)

    root = suite.WORK / f"diagnostic-summary-{uuid.uuid4().hex[:8]}"
    root.mkdir(parents=True, exist_ok=True)
    suite.write_evidence(
        root,
        "diagnostic.json",
        json.dumps(
            {
                "provenance": "wave4-plan.md §8; recorded, never gating",
                "replicates": REPLICATES,
                "max_turns": MAX_TURNS,
                "model": "sonnet",
                "shape": args.shape,
                "evidence": evidence_dirs,
                "commands": summaries,
            },
            indent=2,
        )
        + "\n",
    )
    print(f"\nsummary: {(root / 'diagnostic.json').relative_to(suite.REPO)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
