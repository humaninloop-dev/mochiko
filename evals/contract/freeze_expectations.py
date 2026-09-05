#!/usr/bin/env python3
"""Write `expected-skills.json` — the pre-registered floor sets and baselines for the skills.

Provenance: `.mochiko/brainstorms/cli-schema-delivery/wave5-plan.md` §4.2 and
`wave5-reports/p3-skills-plan.md` §3. This is the tool that produced the committed freeze, kept in
the tree so the file is reproducible rather than a hand-made artifact nobody can re-derive.

**The refusal rule is the point.** Criterion (1) grades delivery against a set fixed before the
wave ran; a set derived after the conversions landed would be a bar read off the thing it grades.
Two guards enforce that, and neither is advisory:

* the script exits without writing if **any** `SKILL.md` already carries a `!` line, so a late
  freeze cannot quietly record post-conversion floor sets or body sizes;
* it exits without writing if the output file already exists, so a second run cannot overwrite a
  freeze that a wave has already been graded against.

`--verify` is how a later reader checks the committed file rather than trusting it: it rebuilds
every derived field from a given plugin root, reuses only the metadata the original run recorded
(the timestamp and the two version strings, which are facts about that run rather than about the
tree), and compares byte for byte.

    python3 evals/contract/freeze_expectations.py
    python3 evals/contract/freeze_expectations.py --verify evals/contract/expected-skills.json \\
        --plugin-root /tmp/plugin-at-7d098b9
"""

import argparse
import datetime
import json
import pathlib
import subprocess
import sys

REPO = pathlib.Path(__file__).resolve().parents[2]
PLUGIN = REPO / "plugins" / "mochiko"
BINARY = REPO / "target" / "release" / "mochiko-cli"
OUT = REPO / "evals" / "contract" / "expected-skills.json"

# The four families of the wave-5 arc, in the order D9 converts them, and the common schema each
# shares. Patterns and the dense five have none, so their baseline is the skill's own schema
# alone — recorded as `common: null` rather than as a zero, so the absence is stated.
FAMILIES = {
    "review": (
        "schemas/skill-review-common.yaml",
        [
            "review-brainstorm", "review-code-minimalism", "review-feasibility",
            "review-governance-intent", "review-plan-artifacts", "review-specifications",
            "review-sufficiency", "validation-constitution",
        ],
    ),
    "authoring": (
        "schemas/skill-authoring-common.yaml",
        [
            "authoring-architecture-store", "authoring-constitution", "authoring-epic",
            "authoring-feature-map", "authoring-prototype", "authoring-requirements",
            "authoring-technical-requirements", "authoring-user-stories",
        ],
    ),
    "patterns": (
        None,
        [
            "patterns-adopt-first", "patterns-architecture-shelves", "patterns-code-minimalism",
            "patterns-map-minimalism", "patterns-model-tiering", "patterns-plan-minimalism",
            "patterns-sound-loop", "patterns-transport-floor", "patterns-vertical-tdd",
        ],
    ),
    "dense-five": (
        None,
        [
            "analysis-codebase", "brownfield-integration", "executing-tdd-cycle",
            "testing-end-user", "testing-gap-finding",
        ],
    ),
}

CONVERTED_MARK = "!`mochiko-cli rules"


def run(args: list) -> subprocess.CompletedProcess:
    return subprocess.run(args, capture_output=True, text=True, timeout=120)


def preamble(binary: str, skill: str, plugin: pathlib.Path) -> str:
    out = run([binary, "rules", skill, "--section", "preamble", "--plugin-root", str(plugin)])
    if out.returncode != 0:
        sys.exit(f"{skill}: the preamble render failed: {out.stderr.strip()[:200]}")
    return out.stdout


def floors_and_pin(text: str, skill: str) -> tuple[list, int]:
    """The `floors:` line's ids and the `class: floor` pin, each read and then cross-checked.

    Reading both and requiring them to agree is what keeps the freeze honest about the render it
    came from: a `floors:` line that had drifted from the pin would be caught here rather than
    frozen into the expectation and graded against for a whole wave.
    """
    ids, pin, inside = None, None, False
    for line in text.splitlines():
        if line.startswith("floors:"):
            value = line[len("floors:"):].strip()
            ids = [] if value == "none" else [i.strip() for i in value.split("·") if i.strip()]
        if line.strip() == "pins":
            inside = True
            continue
        if inside:
            if not line.startswith("- "):
                inside = False
                continue
            if line.startswith("- class: floor · "):
                pin = int(line[len("- class: floor · "):].split()[0])
    if ids is None:
        sys.exit(f"{skill}: no `floors:` line in the preamble render")
    pin = 0 if pin is None else pin
    if len(ids) != pin:
        sys.exit(f"{skill}: the floors line lists {len(ids)} ids but the pin says {pin}")
    return ids, pin


def build(plugin: pathlib.Path, binary: str, metadata: dict) -> dict:
    """Every derived field, from the render and the tree. Metadata is passed in, never derived."""
    skills = {}
    for family, (common_rel, members) in FAMILIES.items():
        common = (plugin / common_rel) if common_rel else None
        common_bytes = common.stat().st_size if common else 0
        for name in members:
            directory = plugin / "skills" / name
            schema, body = directory / "schema.yaml", directory / "SKILL.md"
            for path in (schema, body):
                if not path.is_file():
                    sys.exit(f"{name}: no {path}")
            ids, pin = floors_and_pin(preamble(binary, name, plugin), name)
            source = f"wc -c of skills/{name}/schema.yaml"
            source += f" + {common_rel}" if common_rel else " (no family common file)"
            skills[name] = {
                "family": family,
                "floor_ids": sorted(ids),
                "floor_pin": pin,
                "schema_bytes": schema.stat().st_size,
                "common": common_rel,
                "common_bytes": common_bytes if common_rel else None,
                "baseline_bytes": schema.stat().st_size + common_bytes,
                "baseline_source": source,
                "body_bytes_pre": body.stat().st_size,
            }
    return {
        "provenance": (
            "wave5-plan.md §4.2 and wave5-reports/p3-skills-plan.md §3. Frozen from the render "
            "and the working tree before P2 converted any skill and before the wave's first "
            "session. Never edited after; a floor rule added or renamed later breaks the "
            "cross-check in `converted-shape` rather than regrading quietly."
        ),
        "frozen_utc": metadata["frozen_utc"],
        "binary": metadata["binary"],
        "plugin_version": metadata["plugin_version"],
        "labels_registry": {
            "path": "schemas/skill-labels.yaml",
            "bytes": (plugin / "schemas" / "skill-labels.yaml").stat().st_size,
            "note": (
                "recorded beside the baselines and never part of the criterion, the convention "
                "`command-labels.yaml` got at wave 3"
            ),
        },
        "families": {
            family: {"common": common_rel, "members": members}
            for family, (common_rel, members) in FAMILIES.items()
        },
        "skills": {name: skills[name] for name in sorted(skills)},
    }


def resolve_binary() -> str:
    probe = run([str(BINARY), "--version"])
    if probe.returncode != 0:
        sys.exit(f"no runnable binary at {BINARY} — build it with "
                 "`cargo build --release -p mochiko-cli`")
    return str(BINARY)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--plugin-root", type=pathlib.Path, default=PLUGIN)
    parser.add_argument("--out", type=pathlib.Path, default=OUT)
    parser.add_argument("--verify", type=pathlib.Path,
                        help="rebuild against --plugin-root and byte-compare with this file")
    args = parser.parse_args()
    binary = resolve_binary()

    if args.verify:
        existing = json.loads(args.verify.read_text(encoding="utf-8"))
        rebuilt = build(args.plugin_root, binary, existing)
        rendered = json.dumps(rebuilt, indent=2) + "\n"
        original = args.verify.read_text(encoding="utf-8")
        if rendered == original:
            print(f"{args.verify.name}: byte-identical when rebuilt from {args.plugin_root}")
            return 0
        print(f"{args.verify.name}: DIFFERS when rebuilt from {args.plugin_root}",
              file=sys.stderr)
        for old, new in zip(original.splitlines(), rendered.splitlines()):
            if old != new:
                print(f"  committed: {old.strip()[:120]}", file=sys.stderr)
                print(f"  rebuilt  : {new.strip()[:120]}", file=sys.stderr)
                break
        return 1

    # The two guards. Neither is advisory: a freeze taken after the conversions land, or one that
    # overwrites a freeze already graded against, is worse than no freeze at all.
    if args.out.exists():
        sys.exit(f"{args.out} already exists — the freeze happens once and is never re-run; "
                 "use --verify to check it")
    converted = [
        path.parent.name
        for path in sorted((args.plugin_root / "skills").glob("*/SKILL.md"))
        if CONVERTED_MARK in path.read_text(encoding="utf-8")
    ]
    if converted:
        sys.exit(f"skills are already converted ({converted}) — the floor sets and "
                 "`body_bytes_pre` would record the post-conversion state")

    version = run([binary, "--version"]).stdout.strip().splitlines()[0]
    plugin_version = json.loads(
        (args.plugin_root / ".claude-plugin" / "plugin.json").read_text(encoding="utf-8")
    )["version"]
    payload = build(args.plugin_root, binary, {
        "frozen_utc": datetime.datetime.now(datetime.timezone.utc).isoformat(timespec="seconds"),
        "binary": version,
        "plugin_version": plugin_version,
    })
    args.out.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")
    total = sum(len(row["floor_ids"]) for row in payload["skills"].values())
    print(f"froze {len(payload['skills'])} skills, {total} floor ids, into {args.out}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
