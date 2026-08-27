# Preregistration — setup plan-only eval

Committed BEFORE the first grid (record build 2; skill-harness R6 carried over). The
runner refuses a grid without this file. Amending it after results exist is a recorded,
deliberate act — never a quiet retro-fit.

Second command on the instrument, after the `implement` pilot (D5). The grid under
preregistration here evaluates the command-schema-ontology wave's edit to the
`/mochiko:setup` pair.

## Read rule (D6 as amended — tolerance band, verify V4)

- Comparison substrate: the **in-grid pre-edit arm** (never a committed baseline file,
  which is only a pinned historical record).
- **Unchanged bucket:** a rule regresses when it is `reflected` under pass^k in the pre
  arm and not in the post arm. Tolerance: **0 regressed rules** is the pass reading;
  1–2 regressed rules = "investigate — read the evidence quotes before any verdict";
  ≥3 = "regression reading, present to the maintainer". Coverage-count drift without a
  named regressed rule is noise, not signal.
- **Removed bucket:** any removed rule still surfacing (pass^k in post) = "edit did not
  take" finding.
- **Added bucket:** an added observable rule not reflected in any post replicate =
  DEAD-TEXT finding.
- **Changed bucket:** graded against the NEW text; pre-versus-post comparison advisory
  only — stability cannot be demanded of an obligation that itself moved.

## Arms and the edit under evaluation

- **Pre arm:** the pre-ontology pair as committed at **`99d219e`** — 40 rules, no
  `conditions:` / `moments:` blocks, no `extends` stubs (`schemas/common.yaml` does not
  exist at that ref, and the runner resolves an absent common file to an empty block set).
- **Post arm:** the working tree's pair — the ontology grammar (`kind:` · `when:` ·
  `conditions:` · `moments:` · `enforces:` · `extends`) plus this wave's amend-mode and
  store-scaffold edits.
- Pin the ref explicitly: `--old-ref 99d219e`. `HEAD` is the same commit only while the
  ontology edit is uncommitted; once it lands, `HEAD` is the post pair and the pre arm
  must name the SHA.

## Bucket shape at authoring time (recompute before reading the grid)

`uv run evals/commands/run.py partition setup --old-ref 99d219e`, crossed with the D8
observable subset, gave **unchanged 29 (22 observable) · changed-text 11 (10 observable) ·
removed 0 · added 0**. Two consequences the reader must hold:

- The regression denominator is the **22 unchanged observable rules**, not all 32. The
  tolerance band above applies to that set.
- **Removed and added are empty**, so two of D6's three answers are vacuous for this
  grid — the instrument reports unchanged-bucket regression plus the advisory changed-text
  read, and nothing about removal or adoption. If the pair changes again before the grid
  runs, recompute: the buckets move with every edit.

## Noise guard (F2's guard, verbatim discipline)

Same-variant replicate spread exceeding the variant gap = noise; run one more replicate
pair before any verdict. Operationally here: if the count of flaky rules (replicate
disagreement within one arm) exceeds the count of pass^k differences between arms, the
grid is noise-dominated — add one replicate per arm and re-judge before reading anything.

## Grid shape

3 goldens (s1-greenfield · s2-brownfield · s3-amend) × 3 replicates × 2 arms (pre + post)
= 18 sessions. The three scenarios plant all three values of the schema's `mode`
dimension, so every mode-gated rule is graded in the scenario that activates it. Judges:
Haiku coverage checklist over the 32-rule observable subset + the stub axis; Sonnet
pairwise, position-swapped. All judges advisory (harness D2) — the runner exits 0 on
judged degradation.

No no-command control arm is planned for this grid: the dead-zone read was taken once at
the `implement` pilot (I5). `--control` remains available if the maintainer wants a
per-command dead-zone reading, at +9 sessions.

## Fixture-echo caveat (s3 only)

An amend fixture's honest repo state is the previous run's own output, so s3 necessarily
carries governance-artifact vocabulary a greenfield workspace does not: a synthesis header
naming its checkpoint confirmation, a ledger with an amendment policy, the region's amend
route line. The fixtures were scanned for echoes of the pair's rule language and the one
hit found — a line naming the pre-ratification stress-test and its cold seat — was removed
from the synthesis. What remains is artifact identity, not rule text, but s3 coverage on
the artifact-binding rules should still be read as slightly easier than s1 and s2 coverage
of the same rules. A no-command control arm on s3 is what would measure the residue.

## Ship bar (advisory instrument — informs, never gates)

The grid is useful if it (a) localizes at least one true behavioral difference between the
pre-ontology and post-ontology pairs to named rule IDs, and (b) keeps its flaky-rule set
under 20% of the observable subset (under 7 of 32). Failing (b) triggers the record's
noise falsifier (open question 3): the substrate bet weakens and the session premise is
revisited.

A second, substrate-level question this grid answers for free: whether the mode branch is
legible to the instrument at all — whether s1, s2, and s3 produce visibly different
coverage profiles on the mode-gated rules. Identical profiles across the three scenarios
would mean the fixtures do not force the branch, which is a fixture finding, not a
command finding.
