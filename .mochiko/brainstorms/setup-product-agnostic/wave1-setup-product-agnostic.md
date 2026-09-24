# Wave 1 — setup goes product-agnostic (the build)

**Ruling:** `record.md` D1–D6 as review-amended, accepted 2026-09-24; `DECISIONS.md` row 2026-09-24.
**Build surface:** `record.md` § Build surface, steps 1–7 — one wave.
**Lead:** the session lead. Produces nothing on a governing surface; owns the branch, this plan,
`build-log.md`, sequence allocation, the gate dispatch, the ceremony (CHANGELOG · `plugin.json` ·
`marketplace.json`), and the gates.
**Floors in force:** sound loop (6) · transport floor (11) · model tiering (8) · the primitive-edit
ceremony (`.claude/rules/mochiko/primitive-edits.md`) · the crate rules (`.claude/rules/mochiko/rust-cli.md`).
**Branch:** `setup-product-agnostic` off `main` at `bbe303f` (plugin 0.114.0; installed cache 0.112.0 —
seats resolve the installed personas; `review-seat-plan` and `validation-primitive-edit` exist in both).
**Target version:** plugin `0.115.0` (MINOR: rules struck, agenda changed, templates changed).

## 1. Seats and ownership (strictly disjoint write sets)

| Seat | Persona (tier) | Owns (write set) |
|---|---|---|
| S1 | `tech-lead` (default `opus`) | `plugins/mochiko/migrations/0019-*.yaml` … `0023-*.yaml` (allocated range 19–23, gaps legal) · `plugins/mochiko/commands/setup.md` (goal paragraph only) · `.mochiko/schema-views/**` (regenerated, never hand-edited) · `.mochiko/provenance.yaml` (if the log's anchors are mirrored there — S1 checks) · `.mochiko/strips/setup.md` |
| S2 | `tech-lead` (default `opus`) | the prose references and SKILL.md files listed in § 3 · `.mochiko/strips/<primitive>.md` for each prose primitive it edits |
| S3 | `staff-engineer` (default `opus`) | `evals/plan/setup/observable.yaml` · `evals/contract/run.py` (setup `Expected` block only) + `evals/contract/expected-skills.json` if the freeze touches it · `crates/mochiko-cli/tests/fidelity.rs` (sequence list + census figures) · any crate test fixture the replay count moves |
| G1 (+G2 if context demands) | plain `general-purpose`, `model: opus` | reads only; writes `reports/w1-gate-audit-*.md` |
| P1–P3 | fresh peers of each producer's persona type | plan grades per `mochiko:review-seat-plan`; write nothing |

Order: S1, S2, S3 plan-only in parallel → grades → GO. S1 and S2 execute in parallel. **S3 executes
only after S1's migrations have landed** (its census figures depend on S1's mint/supersede counts).
Gate audit after all three land; fix round by the same grader seat; then the lead's ceremony and gates.

## 2. S1 — schema content (the migration log) + `setup.md`

Grammar: `plugins/mochiko/migrations/README.md`. Every file: `mochiko-cli migrate stamp <file>`;
validate: `mochiko-cli migrate validate --report --plugin-root plugins/mochiko` (0 rejecting);
views: `mochiko-cli views emit --plugin-root plugins/mochiko --out .mochiko/schema-views`.
Anchor format `2026-09-24 setup-product-agnostic D<n>` — one decision segment per migration, so
split by decision. Protected content (floors, fails, anchored rules) leaves only by `supersede-rule`.

**2a — `command/setup` (anchor D5).** Supersede or tombstone the five: `setup.product-truth-leg` ·
`setup.design-truth-write` · `setup.user-map-confirmation` · `setup.feature-map-brownfield` ·
`setup.baselines-bootstrap` (use `supersede-rule` wherever the rule carries an `anchor:` — the 0014
rules do). Reword the seven (ids survive): `setup.user-card-rulings` (drop the product-truth-answer
and design-baseline-ratification clauses) · `setup.interrogation-inputs` (drop `COMPLIANCE-MODULES.md`)
· `setup.feature-map-greenfield` (the empty `FEATURES.md` index scaffold, write-if-absent on every
path; clear its `when:` with `set-rule-field`; brownfield's empty index carries one line naming that
reconstruction is pending and whose it is) · `setup.design-scaffold-unconditional` (the required truth
headings written by the lead as an empty scaffold, no `product-designer` seat) · `setup.fail.no-feature-map`
(narrow to "no feature-map index at close — absent and not scaffolded"; `enforces:` →
`setup.feature-map-greenfield` alone) · `setup.map-never-overwrite` (its instance is the scaffold alone)
· `setup.blind-map-dispatch` ("the setup topic / project name and goal"). Pins after: floors 20 · fail 6;
sections roles 5 · reserved 6 · tools 11 · ways-of-working 6 · boundaries 7 · fail 6 (41 rules).
`setup.md`: the Adaptive Goal Protocol step 2 — scaffolds stay (index if absent, spine stub + `Scope:`,
design truth headings), the baselines and product-truth sentences go; Not-done cites the pin unchanged.

**2b — templates (anchor D1 / D3 / D4 as each applies; `replace-document`).** `template/governance-intent`:
Fact profile section struck; Project identity & type → type + shelves + team reality only (identity
and risk lines struck); a **Trust vectors** line in dimension 8's slot (what ships where — binary, hooks,
public package, deploy exposure — and their controls); Minted principle intents under the D2 test; a
**Handed off** list under Deliberate exclusions (one line per elicited intent that failed D2, pointer to
the rehoming session); the amend preamble's legacy clause → forward-only (supersede on sight, no carry);
Module selections keeps template modules only (compliance sentence out); the skeleton follows.
`template/governance-surfaces`: `modules:` stamp struck (:41, :121); amend lines (:64, :136, :140) → the
D4 six-event set; legal-mandate line (:127) out; trace vocabulary (:169) drops `module:`.
`template/design-baseline`: overview's writer sentence → the setup path is the lead's empty scaffold, the
`product-designer`'s graded path is the landing fold (and whatever the rehoming session rules); the
Accessibility section's contract/check name no module the project's governance attaches — a pointer to
the standard of record, home to be named by the rehoming session, else the floor line.

**2c — consumer rules (anchor D1 or D4).** `command/specify` `spec.missing-map-surfaced` → "offer
`/mochiko:setup`, which scaffolds the empty index; reconstruction of delivered capabilities is not
setup's" · `skill/authoring-architecture-store`: a rule (mint in its discipline section, or reword
`scope-line-durable-home`) — a `Scope:` write at the desk surfaces "shelf set changed — a setup amend is
owed" at its landing; no watcher · `skill/patterns-design-direction.no-accessibility-content` → the
standard of record named without "the `a11y` compliance module" · `skill/patterns-craft-floor` (view :27)
and `skill/testing-gap-finding.a11y-verification-routing`, `skill/review-design-audit.not-a11y-verification`
— reword only where they name a module the project attaches; otherwise leave and say so.

**2d — constitution skills (anchor D1 / D2 / D4).** `skill/authoring-constitution`: module-attachment
rules (view :103, :288, :349, :410, :444–445, :465) reworded or superseded; a rule carrying D2's test
(the rule stays, the instance leaves; hand-off recorded); the amend path's legacy clause forward-only.
`skill/validation-constitution`: :138 semver text drops "module attach/detach"; mint one check — no
product instance in the set, worked cases = D2's sort (GI-009 stays · GI-008 rule stays/instance leaves ·
GI-028 stays · GI-004 rule stays/list leaves · GI-006 floor stays/corpus wording leaves · GI-036 leaves ·
GI-037 leaves). `skill/review-governance-intent` :168 — the fact-profile angle → the D2 angle (a product
instance restated in the synthesis). `skill/analysis-codebase`: the 0014 design-system detection stays
(a code fact); any feature-map or baselines feed bound to setup is narrowed. Read each rule before
touching it; a rule that already reads right is left alone and named in the plan as left alone.

## 3. S2 — prose primitives + strips

Write set: `skills/authoring-constitution/references/INTERROGATION-AGENDA.md` (seven dimensions + depth
level; dimension 8 gains the trust vectors and the D2 clause on SLO numbers; dimension 9 phrasing under D2;
steps 1/5 lose module attachment; depth-per-mode Amend paragraph forward-only; a struck-dimension clause
on the no-pruning note) · `references/COMPLIANCE-MODULES.md` (header: retired from setup's inputs
2026-09-24, content kept for the rehoming session; nothing else) · `references/ESSENTIAL-FLOOR.md` :13 ·
`references/catalog/README.md` :50 and the "two-row production floor" module bullet ·
`references/catalog/universal-floor.md` :16 · `references/catalog/backend-service.md` :110 ·
`skills/authoring-constitution/SKILL.md` :20, :94 · `skills/validation-constitution/references/QUALITY-CHECKLIST.md`
:26, :35, :39, :40, :47, :52 (module checks retired; the no-product-instance check added as a checklist
line mirroring S1's rule) · `skills/review-governance-intent/SKILL.md` :9 · `skills/analysis-codebase/SKILL.md`
(only if a sentence binds it to setup's map/baselines feed) · `skills/patterns-vertical-tdd/references/TEST-GRAMMAR.md`
:137 · `templates/constitution-modules/release-gates.md` :39 · `skills/mochiko/SKILL.md` (router rows
for setup / authoring-constitution only if their text names modules or the product-truth leg).
Strips: one entry per removed or superseded line, in `.mochiko/strips/<primitive>.md`, stamped
`[v0.115.0]`, supersession-by-ruling form citing the `DECISIONS.md` row 2026-09-24 and the D. Budgets:
`.mochiko/memory/primitive-cost-budgets.md` — `references/` exempt; SKILL.md body and `description:`
measured as parsed characters; an overage named in the audit brief with its justification.

## 4. S3 — evals and crate

`evals/plan/setup/observable.yaml`: drop the three struck ids it cites (`user-map-confirmation` ·
`feature-map-brownfield` · `baselines-bootstrap`); re-key the `why` of the amended ids; comments at
:78–:83 re-read. `evals/contract/run.py` setup `Expected`: the floor id set is unchanged (20 floors
stay); its `baseline_bytes` figure is a historical constant (`evals/contract/README.md:265–266`)
and is never re-frozen — the block is untouched unless the floor id set moves *(the original
"re-freeze via `freeze_expectations.py`" sentence withdrawn by lead ruling at the S3 plan grade,
2026-09-24: the script writes skills only and refuses on today's tree)*. Also in S3's set by
lead ruling at plan time: `crates/mochiko-cli/tests/validate.rs` and `matrix_similar.rs` (census
pins), `evals/contract/expected-skills.json` + `README.md:273` (field-scoped, only if a skill floor
moves), `evals/plan/setup/evals.json` (the s1 and s2 goldens grade struck behaviour — § 7 build
defects), and three pre-existing uncovered ids added to `observable.yaml` as observable.
`crates/mochiko-cli/tests/fidelity.rs`: the sequence list gains S1's migrations; the census
figures move by S1's exact supersede/mint counts (read the landed migrations, never assume) — a
comment paragraph in the file's own style records the wave. Run `cargo test --all`, `cargo fmt --all
--check`, `cargo clippy --all-targets -- -D warnings`; run `python3 evals/contract/run.py` in the sandbox
and report the tally verbatim. No crate source file outside `tests/` is touched.

## 5. Gate audit

One plain seat (`general-purpose`, `model: opus`; split into two only if the files exceed its context,
said in the lines), contract = `mochiko:validation-primitive-edit`'s render pasted verbatim. Units:
command pairs `setup`, `specify` · schema content (each migration + its view diff) · skill pairs
`authoring-constitution`, `validation-constitution`, `review-governance-intent`, `analysis-codebase`,
`patterns-design-direction`, `authoring-architecture-store`, plus any other skill a migration touched ·
templates (schema content) · prose: `release-gates.md`, the router · crate + evals (the independent
non-author code review, `rust-cli.md`). Pre-pass by the grader: `migrate validate --report`,
budgets. Outcome lines into `build-log.md`. Bound: `common.gate-loop-bound` — one fix + one re-audit
by the same seat; a second FAIL halts to the user.

## 6. Ceremony and gates (lead)

`CHANGELOG.md` entry · `plugins/mochiko/.claude-plugin/plugin.json` → `0.115.0` · `.claude-plugin/marketplace.json`
synced · views ≡ replay · `cargo test` green · contract suite green (a SKIPPED suite is not green) ·
`build-log.md` closed with the disclosure line `floor: tripped · seats: S1/S2/S3 / P1–P3, G1 ·
plans: …` · BACKLOG build item → trail on the bump. Commits suggested, never run by the lead.

## 7. Stops (halt to the user)

A second gate FAIL on any unit · a migration that cannot express a ruled change under grammar 1 · a
budget overage with no justification · the contract suite SKIPPED or red · any consumer of a struck id
found outside the record's closed list (a build defect: fix and disclose, no new ruling).
