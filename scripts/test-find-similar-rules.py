#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.9"
# dependencies = ["pyyaml"]
# ///
"""Test matrix for scripts/find-similar-rules.py.

Two layers, matching what the detector is:

Unit layer — the scoring internals are loaded via importlib and exercised directly:
normalization (vars/command-names/own-prefix collapse), the sequence-vs-token-sort max,
the structural bonus and its cap, the short-text guard, the 1.00 combined cap.
A detector whose arithmetic drifts silently would still "run green" end-to-end, so the
numbers are pinned here.

End-to-end layer — synthetic schemas in a temp directory (via --schemas-dir), each case
asserting one behavior by its visible output: cluster classification tags, the
extend-resolution path, the same-common-block skip, allowlist suppression and its stale-ID
warning, and the --exit-signal contract. Fixtures are synthetic so the matrix stays green
while the real library is mid-wave.

Run:  uv run scripts/test-find-similar-rules.py
"""

import importlib.util
import subprocess
import sys
import tempfile
from pathlib import Path

import yaml

DETECTOR = Path(__file__).resolve().parent / "find-similar-rules.py"

spec = importlib.util.spec_from_file_location("fsr", DETECTOR)
fsr = importlib.util.module_from_spec(spec)
spec.loader.exec_module(fsr)

PASSED = 0
FAILED = []


def check(name: str, cond: bool, detail: str = ""):
    global PASSED
    if cond:
        PASSED += 1
    else:
        FAILED.append(f"{name}{(' — ' + detail) if detail else ''}")


# ---------------------------------------------------------------- unit layer

def test_norm():
    n = fsr.norm_for_sim(
        "Read ${TARGET} before /mochiko:demo fires; demo.sec.tools and demo.read-first bind.",
        "demo")
    check("norm: ${var} collapsed", "«var»" in n and "target" not in n, n)
    check("norm: /mochiko:<cmd> collapsed", "«cmd»" in n and "mochiko" not in n, n)
    check("norm: own-prefix section collapsed", "«self».tools" in n, n)
    # Hyphens strip with the rest of the punctuation, so the slug reads space-joined.
    check("norm: own-prefix rule collapsed", "«self».read first" in n, n)
    n2 = fsr.norm_for_sim("Read ${OTHER} before /mochiko:setup fires; spec.sec.tools stays.", "demo")
    check("norm: foreign prefix kept", "spec.sec.tools" in n2.replace(" ", ""), n2)


def test_text_sim():
    check("sim: identical = 1.0", fsr.text_sim("one two three four", "one two three four", 0.0) == 1.0)
    reordered = fsr.text_sim("alpha beta gamma delta epsilon", "delta epsilon alpha beta gamma", 0.0)
    check("sim: token-sort rescues reorder", reordered > 0.95, f"{reordered:.3f}")
    disjoint = fsr.text_sim("aa bb cc dd", "ww xx yy zz", 0.5)
    check("sim: disjoint under floor = 0.0", disjoint == 0.0, f"{disjoint:.3f}")


def mk(rid, text, schema="demo", prefix="demo", section="tools", kind="constraint",
       cls="must", labels=(), pointer=None, extends=None):
    return fsr.Rule(schema=schema, prefix=prefix, rid=rid, section=section, kind=kind,
                    cls=cls, labels=list(labels), pointer=pointer, extends=extends, text=text)


def test_bonus():
    base = dict(labels=("binding",))
    x = mk("demo.a", "t", pointer="mochiko:x", **base)
    y = mk("demo.b", "t", pointer="mochiko:x", **base)
    check("bonus: pointer+section+labels hits cap", fsr.struct_bonus(x, y) == fsr.BONUS_CAP)
    y2 = mk("demo.c", "t", section="roles", pointer="mochiko:y", labels=("role",))
    check("bonus: nothing shared = 0", fsr.struct_bonus(x, y2) == 0.0)
    y3 = mk("demo.d", "t", section="roles", labels=("binding", "role"))
    check("bonus: labels jaccard 0.5 counts", fsr.struct_bonus(x, y3) == 0.04)


def test_score_pairs_guards():
    # Short pair: 4 tokens each, similar frame, sim < 0.80 — must not pair.
    s1 = mk("demo.s1", "An unaccepted record here")
    s2 = mk("demo.s2", "An undispositioned review survivor")
    edges, _, _ = fsr.score_pairs([s1, s2], 0.55, set())
    check("guard: short near-frame pair dropped", edges == [], str(edges))
    # Short but exact — pairs.
    s3 = mk("demo.s3", "User acceptance not given")
    s4 = mk("othr.s4", "User acceptance not given", schema="other", prefix="othr")
    edges, _, _ = fsr.score_pairs([s3, s4], 0.55, set())
    check("guard: short exact pair kept", len(edges) == 1)
    # Combined score caps at 1.00 even with full bonus.
    long_t = "the quick brown fox jumps over the lazy dog every single morning"
    x = mk("demo.x", long_t, pointer="mochiko:p", labels=("binding",))
    y = mk("othr.y", long_t, schema="other", prefix="othr", pointer="mochiko:p", labels=("binding",))
    edges, _, _ = fsr.score_pairs([x, y], 0.55, set())
    check("cap: combined never exceeds 1.00", edges and edges[0][0] == 1.0,
          str(edges[0][0] if edges else None))
    # Cross-kind pair never scored.
    k1 = mk("demo.k1", long_t, kind="duty")
    k2 = mk("othr.k2", long_t, schema="other", prefix="othr", kind="fail")
    edges, scored, _ = fsr.score_pairs([k1, k2], 0.55, set())
    check("bucket: cross-kind never scored", edges == [] and scored == 0)
    # Both extending the same common block — skipped. The comparison is on the raw
    # `extends:` value, so the skip is grammar-agnostic: a `review-common.*` skill pair
    # skips exactly like a `common.*` command pair (pinned separately because the skill
    # wave's stubs are the second grammar to lean on it).
    e1 = mk("demo.e1", long_t, extends="common.z")
    e2 = mk("othr.e2", long_t, schema="other", prefix="othr", extends="common.z")
    edges, _, _ = fsr.score_pairs([e1, e2], 0.55, set())
    check("skip: same common block skipped", edges == [])
    s1 = mk("alpha-grader.e1", long_t, schema="alpha-grader", prefix="alpha-grader",
            extends="review-common.z")
    s2 = mk("beta-grader.e2", long_t, schema="beta-grader", prefix="beta-grader",
            extends="review-common.z")
    edges, _, _ = fsr.score_pairs([s1, s2], 0.55, set())
    check("skip: same review-common block skipped (skill grammar)", edges == [])
    # Suppressed pair counted, not emitted.
    edges, _, hits = fsr.score_pairs([x, y], 0.55, {frozenset(("demo.x", "othr.y"))})
    check("allowlist: edge suppressed", edges == [] and hits == 1)


def test_classify():
    def cl(*schemas_extends):
        members = [mk(f"{s}.r{i}", "t", schema=s, prefix=s, extends=e)
                   for i, (s, e) in enumerate(schemas_extends)]
        return fsr.classify({"members": members, "edges": []})

    check("classify: 3 schemas = COMMON-CANDIDATE",
          cl(("a", None), ("b", None), ("c", None))[0] == "COMMON-CANDIDATE")
    check("classify: 2 schemas = CROSS-PAIR", cl(("a", None), ("b", None))[0] == "CROSS-PAIR")
    check("classify: 1 schema = INTRA-SCHEMA", cl(("a", None), ("a", None))[0] == "INTRA-SCHEMA")
    check("classify: mixed extends adds EXTEND-GAP",
          "EXTEND-GAP" in cl(("a", "common.z"), ("b", None)))
    check("classify: all-extends is not a gap",
          "EXTEND-GAP" not in cl(("a", "common.z"), ("b", "common.w")))


# ---------------------------------------------------------- end-to-end layer

def sec(prefix, slug, rules):
    return {"id": f"{prefix}.sec.{slug}", "title": slug, "intent": "test", "rules": rules}


def write_fixtures(d: Path):
    """Three tiny schemas: a 3-way near-dup (alpha/beta/gamma), one side already
    extending common (the EXTEND-GAP), and a pair of unrelated rules as controls."""
    common = {"kind": "command-common", "rules": [
        {"id": "common.register", "labels": ["binding"],
         "text": "User-facing prose follows templates/output-style.md."},
    ]}
    alpha = {"kind": "command", "command": "/mochiko:alpha", "sections": [
        sec("alpha", "tools", [
            {"id": "alpha.register", "extends": "common.register", "class": "must",
             "kind": "binding"},
            {"id": "alpha.solo", "labels": ["binding"], "class": "must", "kind": "binding",
             "text": "Completely unrelated obligation about parsing feature maps nightly."},
        ]),
    ]}
    beta = {"kind": "command", "command": "/mochiko:beta", "sections": [
        sec("beta", "tools", [
            {"id": "beta.register", "labels": ["binding"], "class": "must", "kind": "binding",
             "text": "User-facing prose per templates/output-style.md."},
        ]),
    ]}
    gamma = {"kind": "command", "command": "/mochiko:gamma", "sections": [
        sec("gamma", "tools", [
            {"id": "gamma.register", "labels": ["binding"], "class": "floor", "kind": "binding",
             "text": "User-facing prose follows templates/output-style.md."},
            {"id": "gamma.solo", "labels": ["binding"], "class": "must", "kind": "binding",
             "text": "A different unrelated duty naming the governance ledger weekly."},
        ]),
    ]}
    for name, doc in (("common", common), ("alpha", alpha), ("beta", beta), ("gamma", gamma)):
        (d / f"{name}.yaml").write_text(yaml.safe_dump(doc, sort_keys=False), encoding="utf-8")


def run_detector(d: Path, *extra):
    return subprocess.run(
        [sys.executable, str(DETECTOR), "--schemas-dir", str(d), *extra],
        capture_output=True, text=True)


def test_end_to_end():
    with tempfile.TemporaryDirectory() as td:
        d = Path(td)
        write_fixtures(d)
        r = run_detector(d)
        check("e2e: exit 0 by default", r.returncode == 0, r.stderr)
        out = r.stdout
        check("e2e: register cluster found", "beta.register" in out and "gamma.register" in out, out)
        check("e2e: COMMON-CANDIDATE + EXTEND-GAP tagged",
              "COMMON-CANDIDATE + EXTEND-GAP" in out, out)
        check("e2e: extends resolution feeds stub text",
              "alpha.register" in out and "extends common.register" in out, out)
        check("e2e: floor flagged", "⚑floor" in out, out)
        check("e2e: controls stay unclustered", "alpha.solo" not in out and "gamma.solo" not in out, out)

        r = run_detector(d, "--exit-signal")
        check("e2e: --exit-signal exits 1 on clusters", r.returncode == 1)

        r = run_detector(d, "--json")
        check("e2e: --json parses", r.stdout.strip().startswith("["), r.stdout[:80])

        allow = d / "allow.yaml"
        allow.write_text(yaml.safe_dump({"suppressions": [
            {"ids": ["beta.register", "gamma.register"], "reason": "adjudicated"},
            {"ids": ["alpha.register", "gamma.register"], "reason": "adjudicated"},
            {"ids": ["alpha.register", "beta.register"], "reason": "adjudicated"},
            {"ids": ["ghost.rule", "beta.register"], "reason": "stale on purpose"},
        ]}), encoding="utf-8")
        r = run_detector(d, "--allowlist", str(allow))
        check("e2e: fully-suppressed cluster gone",
              "none — no pair clears the threshold" in r.stdout, r.stdout)
        check("e2e: suppression count reported", "allowlist-suppressed edges: 3" in r.stdout, r.stdout)
        check("e2e: stale allowlist ID warned",
              "ghost.rule: not a live rule ID" in r.stdout, r.stdout)

        r = run_detector(d, "--exit-signal", "--allowlist", str(allow))
        check("e2e: --exit-signal exits 0 when suppressed clean", r.returncode == 0)


def write_skill_fixtures(d: Path):
    """Two converted-skill directories + their family library (skill-content-schema
    D2/D5/D7-M3 shapes): one member a stub over review-common, its sibling carrying
    near-identical local text (the EXTEND-GAP), plus a rule near-identical to the
    command fixtures' register cluster so a mixed sweep shows a cross-grammar edge
    (the J-5 drift-edge case)."""
    skill_common = {"kind": "skill-common", "rules": [
        {"id": "review-common.default-fail", "labels": ["verdict"],
         "text": "Never default to a clearing verdict — earned only by a completed "
                 "hunt; absence of looking is never evidence."},
    ]}
    delta = {"kind": "skill", "skill": "delta-grader", "sections": [
        sec("delta-grader", "verdict", [
            {"id": "delta-grader.default-fail", "extends": "review-common.default-fail",
             "class": "floor"},
        ]),
    ]}
    epsilon = {"kind": "skill", "skill": "epsilon-grader", "sections": [
        sec("epsilon-grader", "verdict", [
            {"id": "epsilon-grader.never-default", "labels": ["verdict"], "class": "floor",
             "text": "Never default to a clearing verdict — earned by a completed hunt; "
                     "absence of looking is not evidence."},
        ]),
        sec("epsilon-grader", "output", [
            {"id": "epsilon-grader.register", "labels": ["binding"], "class": "must",
             "kind": "binding",
             "text": "User-facing prose follows templates/output-style.md."},
        ]),
    ]}
    (d / "skill-common.yaml").write_text(
        yaml.safe_dump(skill_common, sort_keys=False), encoding="utf-8")
    for name, doc in (("delta-grader", delta), ("epsilon-grader", epsilon)):
        (d / name).mkdir()
        (d / name / "schema.yaml").write_text(
            yaml.safe_dump(doc, sort_keys=False), encoding="utf-8")


def test_skill_end_to_end():
    with tempfile.TemporaryDirectory() as td:
        d = Path(td)
        (d / "skills").mkdir()
        write_skill_fixtures(d / "skills")

        # Skills-only run: in-dir discovery + extends resolution against skill-common.
        r = subprocess.run(
            [sys.executable, str(DETECTOR), "--skills-dir", str(d / "skills")],
            capture_output=True, text=True)
        out = r.stdout
        check("skill e2e: exit 0 by default", r.returncode == 0, r.stderr)
        check("skill e2e: fixture run reads no live allowlist",
              "not a live rule ID" not in out, out)
        check("skill e2e: in-dir schemas discovered and clustered",
              "delta-grader.default-fail" in out and "epsilon-grader.never-default" in out, out)
        check("skill e2e: stub resolves against review-common (EXTEND-GAP)",
              "extends review-common.default-fail" in out and "EXTEND-GAP" in out, out)
        check("skill e2e: floor members flagged", "⚑floor" in out, out)
        check("skill e2e: register control stays unclustered (skills-only run)",
              "epsilon-grader.register" not in out, out)

        # Skill-edge suppression: the allowlist quiets a skill pair like a command pair.
        allow = d / "allow.yaml"
        allow.write_text(yaml.safe_dump({"suppressions": [
            {"ids": ["delta-grader.default-fail", "epsilon-grader.never-default"],
             "reason": "adjudicated keep-distinct"},
        ]}), encoding="utf-8")
        r = subprocess.run(
            [sys.executable, str(DETECTOR), "--skills-dir", str(d / "skills"),
             "--allowlist", str(allow)],
            capture_output=True, text=True)
        check("skill e2e: skill edge suppressed",
              "none — no pair clears the threshold" in r.stdout
              and "allowlist-suppressed edges: 1" in r.stdout, r.stdout)

        # Mixed sweep: command fixtures + skill fixtures in one run — the cross-grammar
        # register cluster spans both, and the scanned count covers both sets.
        write_fixtures(d)
        r = subprocess.run(
            [sys.executable, str(DETECTOR), "--schemas-dir", str(d),
             "--skills-dir", str(d / "skills")],
            capture_output=True, text=True)
        out = r.stdout
        check("mixed e2e: cross-grammar edge surfaces in the register cluster",
              "epsilon-grader.register" in out and "beta.register" in out, out)
        check("mixed e2e: both sets scanned", "rules scanned: 8" in out, out)


def write_authoring_fixtures(d: Path):
    """The authoring-family library + two producer directories (wave 2A): one member a
    stub over authoring-common, its sibling carrying near-identical local text (the
    EXTEND-GAP proves the stub's text resolved and re-entered scoring), plus a third
    member whose stub names a prefix no loaded library carries — it must warn, never
    cluster."""
    authoring_common = {"kind": "skill-common", "rules": [
        {"id": "authoring-common.envelope-binding", "labels": ["binding"],
         "text": "The produced artifact follows the deliverable envelope in "
                 "templates/artifact-format.md — referenced, never restated."},
    ]}
    zeta = {"kind": "skill", "skill": "zeta-producer", "sections": [
        sec("zeta-producer", "artifact", [
            {"id": "zeta-producer.envelope", "extends": "authoring-common.envelope-binding",
             "class": "must", "kind": "binding"},
        ]),
    ]}
    eta = {"kind": "skill", "skill": "eta-producer", "sections": [
        sec("eta-producer", "artifact", [
            {"id": "eta-producer.envelope-local", "labels": ["binding"], "class": "must",
             "kind": "binding",
             "text": "The produced artifact follows the deliverable envelope in "
                     "templates/artifact-format.md — referenced never restated."},
        ]),
    ]}
    theta = {"kind": "skill", "skill": "theta-producer", "sections": [
        sec("theta-producer", "artifact", [
            {"id": "theta-producer.ghost-stub", "extends": "patterns-common.ghost",
             "class": "must", "kind": "binding"},
        ]),
    ]}
    (d / "skill-authoring-common.yaml").write_text(
        yaml.safe_dump(authoring_common, sort_keys=False), encoding="utf-8")
    for name, doc in (("zeta-producer", zeta), ("eta-producer", eta),
                      ("theta-producer", theta)):
        (d / name).mkdir()
        (d / name / "schema.yaml").write_text(
            yaml.safe_dump(doc, sort_keys=False), encoding="utf-8")


def test_authoring_end_to_end():
    with tempfile.TemporaryDirectory() as td:
        d = Path(td)
        (d / "skills").mkdir()
        write_skill_fixtures(d / "skills")
        write_authoring_fixtures(d / "skills")

        r = subprocess.run(
            [sys.executable, str(DETECTOR), "--skills-dir", str(d / "skills")],
            capture_output=True, text=True)
        out = r.stdout
        check("authoring e2e: exit 0 by default", r.returncode == 0, r.stderr)
        check("authoring e2e: stub resolves against authoring-common (EXTEND-GAP)",
              "extends authoring-common.envelope-binding" in out
              and "eta-producer.envelope-local" in out and "EXTEND-GAP" in out, out)
        check("authoring e2e: both families resolve in one run",
              "extends review-common.default-fail" in out
              and "delta-grader.default-fail" in out, out)
        check("authoring e2e: unknown-prefix stub warns, never clusters",
              "theta-producer.ghost-stub: empty resolved text, skipped" in out
              and "theta-producer.ghost-stub  (" not in out, out)


def main() -> int:
    for t in (test_norm, test_text_sim, test_bonus, test_score_pairs_guards,
              test_classify, test_end_to_end, test_skill_end_to_end,
              test_authoring_end_to_end):
        t()
    print(f"passed: {PASSED} · failed: {len(FAILED)}")
    for f in FAILED:
        print(f"FAIL: {f}")
    return 1 if FAILED else 0


if __name__ == "__main__":
    sys.exit(main())
