# Changelog

All notable changes to the mochiko plugin. One entry per `plugin.json` version bump —
appending here is release gate 4 (`.mochiko/memory/governance-ledger.md`, GI-010/GI-012).
Entries before 0.53.0 predate this file; their history lives in `ROADMAP.md` stamp lines,
`DECISIONS.md`, and git log.

## [0.99.0] — 2026-08-28

**Near-dup convergence — the similar-rule reduction wave** (ruling
`.mochiko/decisions/2026-08-28-near-dup-convergence.md` R1–R6 + wave flags A–E, all as
recommended; `DECISIONS.md` 2026-08-28 row; ruled and delivered the same day). A new
**similar-rule detector** (`scripts/find-similar-rules.py` + 34-case test matrix, GI-019
advisory) scanned the six pairs — 321 rules, 12,203 in-kind pairs, 36 clusters, 11
spanning 3+ commands — and the extraction bar widened narrowly: a **3+-command
near-identical family may converge to one `common.<slug>` block under
strongest-wording-wins** (ontology D8's exact-only limb amended; per-command default and
stub-carried binding unchanged). Every ID survives as an `extends:` stub — no tombstones,
Not-done pins unmoved. `common.yaml` grows 4 → 9 blocks: new `no-acceptance` +
`plan-approval-producers` (exact triples the D8 bar already licensed) · `model-tiering` ·
`author-grader-default-fail` · `tools-referenced-never-restated` (strongest-member
texts); `transport-floor` widened with the desks' four-part enumeration (flag A) and
`acceptance-plain-text` widened to "rulings and acceptance". 24 stubs land across all six
schemas (~24 full texts single-homed); three commands upgrade to stronger wordings;
command-specific content stays local by ruling (arch's drift-probe routing, implement's
named gates and code/verification clauses — flags B/D/E). Keep-distinct survivors are
adjudicated into `scripts/similar-rules-allowlist.yaml` (60 edges with reasons; the
detector now runs silent and re-fires only on new similarity). Ceremony: strips across 8
files incl. new `strips/common.md` (13 floor supersessions) · 3 author≠grader validators,
6/6 pairs PASS (one strip consumers-list omission caught by V3, repaired, CONFIRMED
discharged) · checker 0 findings ×6, all 9 blocks bound · negative matrix 133/133 · cargo
12/12 · pair-audit criterion 11 re-keyed to the widened bar. Evidence: the extends-aware
partition proves only 3 resolved texts changed per instrumented command (all ruled
upgrades), and pre/post plan-only grids on implement + setup (ontology-grid post arm
reused as pre per the D7 precedent; opus judges; prereg noise guard fired → r4 pairs
added both grids) read **no regression attributable to the wave** — every dipped rule
flaky within its own arm, post never trailing at aggregate (implement s1 52/52 tie · s2
post +2 · s3 50/50 tie; setup s2-brownfield clean 31/32 ≥ pre), both widened texts and
all three upgrades graded present against their new wording. Instrument notes for the
eval desk: several opus judge calls returned unparseable arrays (healed by re-judge —
retry belongs in the runner), and the noise falsifier's instrument-side standing is
re-confirmed at k=4 on both grids.

## [0.98.0] — 2026-08-28

**The command schemas gain a typed node grammar — the ontology wave** (record
`.mochiko/brainstorms/command-schema-ontology/record.md` D1–D11 as review-amended, solo
cold review verdict critical-gaps → 18/18 dispositioned → verify CLEAN → user-accepted;
`DECISIONS.md` 2026-08-27 row; built next day as one wave). Every rule block may now carry
**`kind:`** from a nine-kind closed set — `constraint` (the omitted default) · `duty` ·
`gate` · `reservation` · `binding` · `bound` · `routing` · `fail` · `latitude` — a tag
only, per-kind structured fields deferred benefit-keyed (D1/D2). Run-shape branches become
data: each schema declares its **`conditions:`** (dimension · closed value set · resolution
point) and rules gain **`when:`** — conjunction-only, single-homed out of the text (MOVE),
with a DECLARE carve where the condition rides the rule's subject noun (D3 as amended);
**a `class: floor` rule is always read and always delivered — `when:` gates application,
never delivery** (C4). Each schema declares its **`moments:`** anchor vocabulary,
deliberately unordered; the `at:` field does not ship (deferred to the graduation set,
I5). Every `kind: fail` node carries **`enforces:`** — its gate/floor contrapositive
links, empty only with a stated reason under the `D6 empty-with-reason:` marker (D6 under
an argued D16 carve). **`extends: common.<slug>`** enters the grammar narrowly — a
recorded supersession amending command-content-schema D3 (`Contested`): `common.yaml`
ships with exactly four exact-duplicate blocks (`register` · `no-git-mutations` ·
`acceptance-plain-text` · guard-stripped `transport-floor`; 16 stubs across the six
schemas), a stub inheriting `text`/`labels`/`pointer` only with `class:` always local
(C2/C3). The `fail-condition` label retires from the registry — `kind: fail` is the
selector, the six Not-done pins re-keyed. The D9 coverage holes closed at the wave's user
gate: `impl.landing-lane` minted (a product-lane landing adds no map work) and the
eight-site setup amend-mode posture landed (an amend run owns the governance surface set
and nothing else — never-overwrite floors restated instances-not-extent, create/write-if-
absent made explicit, `setup.fail.no-feature-map` gains the surfacing third arm). The F8
provenance citation leaves both setup sites (the `Assumed` hedge stays). The advisory
checker learns the whole grammar (kind vocabulary · `when:`/`moments:` resolution ·
per-dimension coverage reports · in-text ID-citation resolution over all six prefixes ·
`enforces:` + reverse-coverage · `extends:` C3 asserts) with a negative-test matrix grown
79 → 133; `converting-command-to-schema` and the pair-audit criteria re-keyed (new
criterion 11). Build: inventory referent (`conversion-inventory.md`, 320 rules classified,
37 `when:` moves) · 5+4 producer seats · 4 fresh author≠grader validators — six pairs and
three infra units PASS, two infra FAILs fixed same round to CONFIRMED-PASS; checker PASS
0 findings on all six pairs. Evidence: plan-only eval grids on implement (24 plans, noise-
dominated per its own preregistered guard — no stable regression attributable to the edit)
and setup (18 plans — zero regressions in brownfield/amend, coverage up where the wave
changed content); the kind+`when:` partition derivation recovers 82% of the hand-authored
eval rubric. Strips across nine files; `.mochiko/provenance.yaml` untouched.

## [0.97.0] — 2026-08-27

**One canonical command scaffold — the form split is superseded** (record
`.mochiko/brainstorms/command-md-scaffold-standardization/record.md` D1–D7, solo cold
review then user-accepted same day; `DECISIONS.md` 2026-08-27 row). The six command `.md`
files stop varying: all of them now carry **one scaffold** — frontmatter (`description` ·
`disable-model-invocation: true` · **`argument-hint`, newly canonical on all six**) ·
`# <Name> — <epithet>` · a capped `## Identity & Mission` · `## Rules — load the schema
first` · `## Adaptive Goal Protocol` whose three steps are **Entry** (the single home of
`$ARGUMENTS`) · **Goal** · the count-pinned **Not done — default FAIL**, always last. The
ruled charter-form / goal-form split (three commands each) is **superseded — layout and
section vocabulary only**: the per-visit (desk) vs fixed (run) done-condition contracts
survive intact inside the one scaffold (D1 as narrowed at review, C2). `brainstorm`,
`setup`, and `specify` gain newly-authored Identity & Mission sections and lose the bold
`**Goal:**` opener, whose content relocates into protocol steps Entry and Goal; `setup`'s
stray `**You are the lead.**` line absorbs into its identity section. ADR
`2026-08-02-doctrine-purge-wave-1` **decision 4 is superseded by name** (C1) — the command
audit bar moves from "the command's own text" to the command's own **pair**, graded
against the canonical scaffold — with the re-argument recorded: the purge bought cheap
per-command evolution, and the observed price was the drift this wave closes.

**Schema vocabularies unify in the same wave** (D3 `Contested` · D4 · D5). All six schemas
adopt implement's six-set — `<cmd>.sec.roles` · `reserved` · `tools` · `ways-of-working` ·
`boundaries` · `fail-conditions` — with `reserved` first-class everywhere (the desks
extract their user-reserved rulings out of `roles`; the goal-form commands redistribute
`harness` / `bindings`, whose nodes tombstone per D11/D14). Rule IDs and texts are
unchanged and rule-ID-keyed provenance in `.mochiko/provenance.yaml` is untouched;
existing ID prefixes (`impl` · `feat` · `arch` · `spec` · `setup` · `brainstorm`) are
**frozen**, with a recorded derivation rule for future commands (prefix = the command
filename stem, abbreviated only on a recorded collision). **Breadth invariant:** every
schema carries all six sections always, an unpopulated one marked deliberately empty
rather than omitted — which is what makes the `.md` Rules-block enumeration identical in
shape across the library.

**Checker and ceremony.** The D13 advisory checker gains a `.md` scaffold lint — canonical
headings present and in order · the Rules-block enumeration asserted **set-wise** against
the schema's section IDs (a count-vs-count check goes vacuous under the breadth
invariant) · **every** `<cmd>.sec.*` token anywhere in the `.md` resolving to a live node,
not only those inside the Rules block · no surviving rule text referencing a tombstoned or
re-homed node · the Not-done count-pin matching the schema's `fail-condition` count — each
new assertion negative-tested, advisory exit-code only, GI-019 untouched. The audit
ceremony in `.claude/rules/mochiko/primitive-edits.md` collapses its **dual criteria blocks
into one canonical-scaffold block** that branches internally on done-condition class —
desks (`architecture` · `feature`) on the per-visit convergence contract, runs
(`brainstorm` · `implement` · `setup` · `specify`) on the fixed one, `implement` keeping
its entry gating, run-open confirmation, and attempt bounds — with every criterion the two
old blocks graded relocated, none dropped. The exhaustive clause inventory owed before any
supersession entry lands as **Appendix A** of the record: 59 clauses across the five
carriers — 15 superseded-by-D1 · 28 survive · 16 carrying no clause for D1 to reach; four
carriers superseded at least one clause and `charter-ritual-balance` D3 returned
**no-clause-superseded** in full, the first live instance of the record's N4 carve.

**Ripple and framing.** The shipped router `plugins/mochiko/skills/mochiko/SKILL.md` — whose
composition paragraph still stated the two-anatomy split, and still called `implement` the
library's only `.md` + schema pair, stale since the v0.95.0 rollout — re-keys to the
canonical scaffold and takes its own supersession strip citing both rulings. `CLAUDE.md`
(two sites: the author ≠ grader line and the landing-ritual check clause), `README.md`,
`ARCHITECTURE.md` (six sites, hand-maintained legacy per the operating-docs rule), and the
repo-level conversion skill `.claude/skills/converting-command-to-schema/SKILL.md` re-key
with it, so a seventh command cannot reintroduce the superseded pattern. The
"the 1 rules labeled" grammar defect dies in the canonical Not-done line. Framing ruled
(D7): this is a **stage-1.5 tidy** of what stays prose under command-content-schema D2
stage 1 — it neither advances nor retards the absorption trigger, and the three new
identity sections are knowingly strip candidates if that trigger later fires. Delivery
legs stay `Assumed` at n=0: the open first-live-run watch extends to the new scaffold
(schema Read executed fully before first action from position 3 · empty-section runtime
cost · one concrete six-set addressing benefit), with the baseline perturbation named
eyes-open. **Build and audits:** 4 plan-approved producers under the sound-loop + transport
floors (strictly disjoint ownership: six `.md` rewrites · five schema unifications · checker
rework · ceremony + ripple), 3 fresh author≠grader validators — V1 (architecture ·
brainstorm · feature pairs) **PASS round 1**, minors only (a reflow, a strip-clause
clarification; its title-drift finding closed not-a-defect on the producer's six-schema
census); V2 (implement · setup · specify pairs) **PASS round 1**, advisory-only; V3
(checker · ceremony · inventory · ripple) 4/5 round 1 with this entry the one FAIL (a
shipped-surface omission) → fix round → delta-confirm **5/5 PASS**, all 59 inventory
clauses verified against their carriers. Rule conservation proven three ways independently
(producer rebuilder, V1/V2 HEAD-compare, checker): **320 rules byte-identical to HEAD**,
zero ID changes, 6/6 section tombstones, 107 provenance anchors resolve. V1 also discharged
the record's M5 re-inventory — both desks confirmed **under-extracted on `fail-condition`**
(architecture 1 vs 21 other floor rules, feature 1 vs 12), user-ruled deferred to BACKLOG
("Desk FAIL-set widening") as its own ruled pass. Checker: **PASS 0 findings on all six
pairs** (`--all` exit 0); negative matrix **30/30**, mutation-tested for teeth (each
disabled assertion family isolated exactly its own probes); the persisted harness
`scripts/test-check-command-schema.py` is a ruled departure from the prior ephemeral
pattern. Strips landed across eight files including the new
`.mochiko/strips/primitive-edits.md`. Gates 4/5/6: this entry · marketplace 0.97.0 ·
`cargo test` 12/12.

## [0.96.0] — 2026-08-26

**Runtime-only schemas — the D16 amendment** (record
`.mochiko/brainstorms/command-content-schema/record.md` D16, user-ruled same day;
`DECISIONS.md` 2026-08-26 row). A command schema now carries only what the run consumes:
the inline `ruling:` field leaves the grammar (rule tuple now
`{id, labels, class, text, pointer?}`); all **107 decision anchors** relocate verbatim —
machine-verified lossless, zero value mismatches, every schema body identical after the
strip — to the repo-side sidecar **`.mochiko/provenance.yaml`**
(`kind: command-provenance`, keyed by mint-once rule ID), which is deliberately NOT under
`plugins/` and never ships with the plugin (per-command counts: impl 34 · spec 25 ·
feat 15 · arch 13 · setup 11 · brainstorm 9). Protection semantics unchanged: an anchored
rule still leaves only by recorded supersession-by-ruling; the sidecar joins the
primitive-edits path scope so its own edits take the ceremony. Checker reworked (check 6):
inline `ruling:` is now a finding, a sidecar key naming no rule is a dangling-entry
finding, anchors still format-checked and resolved against live `DECISIONS.md` rows, a
missing sidecar degrades to a warning (plugin-standalone checkouts), foreign-prefix
entries are named in a warning, and stats gain `anchors N` — all negative-tested (inline
reinjection, dangling key, absent file, bad date/slug/format/kind). Grammar headers
updated in all six schemas; conversion-skill step 7 re-pointed; six `[v0.96.0]`
relocation strips. Author≠grader audit **PASS** (0 Critical / 0 Major / 4 Minor — all
repaired or accepted on record). Gates: six checker-PASS runs 0 findings · this entry ·
marketplace 0.96.0 · `cargo test` 12/12.

## [0.95.0] — 2026-08-26

**The D10 rollout: every mochiko command is now a `.md` + schema pair.** User-directed
same-day wave (record `.mochiko/brainstorms/command-content-schema/record.md`, Session
trail "D10 rollout ruling"; `DECISIONS.md` 2026-08-26 row): the five remaining commands
convert structure-only from their frozen v0.94.0 referents — `architecture.yaml` (prefix
`arch`, 47 rules) · `brainstorm.yaml` (29) · `feature.yaml` (`feat`, 49) · `setup.yaml`
(40) · `specify.yaml` (`spec`, 51) — joining `implement.yaml` (104) for a six-pair library
of **320 rules** under the D6/D14/D15 grammar. Conversion ran on the new repo-level skill
`.claude/skills/converting-command-to-schema/SKILL.md` (itself author≠grader validated to
CONFIRMED-PASS in three rounds before use; repo tooling, never shipped — GI-020 intact).
Wave: 5 producers + 5 fresh validators, each pair graded on the three-audit set (pair
coherence · schema fidelity vs the frozen step-0 referent · strip verification) with the
D13 checker as deterministic pre-pass — feature and architecture and brainstorm PASS round
1, specify and setup FAIL round 1 and repaired (headline catches: setup's carve-outs floor
rule had erased the real `governance-surfaces.yaml` referent and narrowed its scope to the
CLAUDE.md region — restored with both carve-out homes named and
`pointer: mochiko:authoring-constitution` adopted on evidence; specify carried one
undisclosed added phrase — struck), all five **CONFIRMED-PASS**. Label registry
`command-labels.yaml` **ten → twelve** by delegated ruling (`binding` · `stewardship`),
swept across the five new schemas. Checker gained the **D14 section-count guard** (the
`.md` "nested in N sections" phrase must match the schema; negative-tested) beside the D15
deixis lint. Pair-form audit criteria generalized in
`.claude/rules/mochiko/primitive-edits.md` (charter-form exception for charter commands,
default coherence block otherwise, both across the pair). Strips: 20 `[v0.95.0]`
supersession entries across five per-command strip files, Content fields machine-verified
verbatim against the referents. Referents frozen under the session's `referents/`. The D10
first-live-run watch in `BACKLOG.md` now spans all six pairs. Gates: six checker-PASS runs
0 findings · CHANGELOG (this entry) · marketplace 0.95.0 · `cargo test` 12/12.

## [0.94.0] — 2026-08-26

Command schemas gain the **referential-closure law** — the D15 post-build amendment (record
`.mochiko/brainstorms/command-content-schema/record.md` D15, user-directed same day;
`DECISIONS.md` 2026-08-26 row). A rule's `text` must be self-contained: every reference
resolves within the block or the schema's addressable namespace (`${var}` · `impl.*` IDs ·
`impl.sec.*` · `class:` values · registry labels · `pointer:` skills · `ruling:` anchors ·
file paths). Deixis — "these rules", "this section", "above"/"below", document-shape
remarks — is a defect: quoted alone, the reference dangles, breaking D12's
independently-citable promise. "this schema" and "the run" stay legal self-reference. The
law is general to every `kind: command` schema; `scripts/check-command-schema.py` carries a
curated **deixis lint** (check 5b, warning-class — heuristic detection never blocks the
advisory pre-pass), verified firing on the defect text and silent on the corpus. The one
live instance in 104 rules reworded: `impl.staffing-latitude` — "the floor these rules
state" becomes "this schema's `class: floor` rules"; the document-shape remark "There is no
Bindings section." dropped without relocation (the schema's existence states it). ID, class,
labels, section unchanged (D11 reword). Grammar header gains the closure block. Strip: 1
supersession entry `[v0.94.0]` in `.mochiko/strips/implement.md`. Author≠grader audit
**PASS** (0 Critical / 0 Major / 2 Minor — gates-owed ordering note; one conforming
wording-tighten candidate deferred to its next touch); checker PASS 0 findings 0 warnings;
gates 4/5/6 (this entry, marketplace 0.94.0, `cargo test` 12/12).

## [0.93.0] — 2026-08-26

`implement.yaml` gains **first-class section nesting** — the D14 post-build amendment (record
`.mochiko/brainstorms/command-content-schema/record.md` D14, user-ruled same day;
`DECISIONS.md` 2026-08-26 row). The flat top-level `rules:` list and its six `#` comment
dividers are superseded by a `sections:` grammar: six nodes `{id, title, intent, rules}` —
`impl.sec.roles` · `impl.sec.reserved` · `impl.sec.tools` · `impl.sec.ways-of-working` ·
`impl.sec.boundaries` · `impl.sec.fail-conditions` — section IDs minted once under the D11
lifecycle, titles verbatim from the divider wording, intents one navigation line each
(sections never grow a second prose surface). **Pure relocation:** all 104 rule IDs and texts
unchanged (+4-space indent only; checker stats identical pre/post — rules 104 · floor 34 ·
must 69 · advisory 1 · fail-condition 15). `commands/implement.md`'s Rules section now
enumerates the six section IDs as the rule map; the Not-done line gains the
`impl.sec.fail-conditions` address, its label-keyed N=15 unchanged. Checker reworked: section
grammar asserted (`<cmd>.sec.<slug>` format, id/title/intent/rules shape, mint-once
uniqueness shared with rule IDs), per-section stats emitted, a surviving flat `rules:` key
now a finding. Charter audit's ID-continuity criterion extended to `impl.sec.*`
(`.claude/rules/mochiko/primitive-edits.md`). Strips: 2 supersession entries `[v0.93.0]` in
`.mochiko/strips/implement.md` (grammar header line + `rules:` key + six dividers; the `.md`
source-of-truth sentence). Author≠grader audit **PASS** (0 Critical / 0 Major / 3 Minor —
intent glosses trimmed on the fix round; the no-git-pre-image provenance note stands until
the wave commits); checker PASS 0 findings; gates 4/5/6 (this entry, marketplace 0.93.0,
`cargo test` 12/12).

## [0.92.0] — 2026-08-26

`/mochiko:implement` becomes a **`.md` + schema pair** — the first command-content schema
(record `.mochiko/brainstorms/command-content-schema/record.md` D1–D13 as review-amended;
`DECISIONS.md` 2026-08-26 row; the simplified-rewrite referent + strip ledger at the session's
`implement-rewrite.md`). The command's rule-shaped content — Roles & Responsibilities, Tools
bindings, Ways of Working, Boundaries, and the 15 FAIL clauses — moves to
`plugins/mochiko/schemas/implement.yaml`: **104 mint-once rules** (34 floor · 69 must · 1
advisory) under the D6 grammar `{id, labels, class, text, ruling?, pointer?}`, dotted-slug IDs
(`impl.<kebab-name>`, FAIL clauses as `impl.fail.*`), a 13-value `vars:` block with `${var}`
substitution, 34 `ruling:` protection anchors, and skill-owned floors carried as `pointer:`
rules. `commands/implement.md` slims 429 → 87 lines: frontmatter + Identity & Mission + the
Adaptive Goal Protocol, an **obligated first-action raw Read** of the schema, and the Not-done
line re-keyed to the count-pinned `fail-condition` set (N=15, out-of-sync halt). New
`plugins/mochiko/schemas/command-labels.yaml` — the ten-label controlled vocabulary all
command-schema rules draw from. New **advisory deterministic checker**
`scripts/check-command-schema.py` (D13, GI-019 advisory carve-out; exit-code signal, never a
gate): ID uniqueness/format, registry labels, `${var}` closure, `ruling:` anchor resolution,
the N=15 pair guard, `kind:` discriminator, tombstone integrity — PASS 0 findings on the
shipped pair, ten-probe negative matrix verified. Charter audit re-keyed in
`.claude/rules/mochiko/primitive-edits.md`: implement graded as the pair from v0.92.0 —
label-keyed FAIL survival, D11 ID continuity, `class: floor` = must-survive, checker output as
deterministic pre-pass. Strips: 17 entries `[v0.92.0]` in `.mochiko/strips/implement.md` (8
supersessions incl. the five whole sections byte-exact, 9 rewrite strips; M4 rule —
shipped-v0.91.0 verbatim throughout, machine-verified) + 1 router supersession in
`.mochiko/strips/mochiko.md`; router and `report-format.md` re-pointed. Wave: 2 plan-approved
producer seats (disjoint ownership schemas+checker vs command+strips+re-key), 2 fresh
author≠grader validators — **both PASS round 1**, Minor-only fix rounds (incl. the
`impl.gates-never-triaged` floor split and the strike-all-block-counts systemic ruling) →
2/2 CONFIRMED-PASS; gates 4/5/6 (this entry, marketplace 0.92.0, `cargo test` 12/12). The
D10 first-live-run watch (delivery probes + benefit observations) stands in `BACKLOG.md`.

## [0.91.0] — 2026-08-26

`/mochiko:plan` retired; `/mochiko:implement` becomes the pipeline's single downstream run
(record `.mochiko/brainstorms/plan-stage-utility/record.md` D1–D7, as amended at the pair cold
review, three verify rounds, and the post-acceptance A1–A6 addendum; `DECISIONS.md` 2026-08-26
row; strip `.mochiko/strips/plan.md` [v0.91.0], carrying the superseded command verbatim plus
an obligation-by-obligation rehome map). Implement's entry now runs a **sufficiency check** —
ten clauses per selected work row, collapsing to a three-clause form per delta card under delta
scope, graded from the spec, the architecture store, and the product baselines by a seat that
authored none of them; binding, with a disputed clause defaulting to gap and routing to the
user. Zero gaps goes straight to card authoring and build; any gap fires an **in-run design
phase scoped to exactly the named gaps**, independently graded and user-signed at a blocking
checkpoint before the first cycle, with card authoring and its own confirm checkpoint
following. Plan's plan-the-plan proposal and package-acceptance gates are replaced by run-open
routing · design sign-off · card confirm. The `plan.md` summary artifact, the mandatory FR→TR
layer, and plan's inline epic mint door die with the command; epic minting survives at
`/mochiko:feature`.

New skill **`mochiko:review-sufficiency`** — the check's single source, D2 as amended being its
content spec. Ten clauses each carrying an explicit gap form (the A1 locatable/unattachable
split on clauses 2–3, the A4 self-satisfaction exclusion on 4, A3's targets-absent rule on 5,
A5's weighed-alternatives definition on 6, A2's row-keyed status on 9–10), plus the D6
delta-scope collapse, the absent-baseline greenfield seed branch, trips-ride-the-report, and
the report contents. Its fence admits the spec, the store, the baselines, and map entries, and
excludes the code, `tasks.md`, `**TEST:**` cases, cycle reports, and the batch's own
`FEAT-XXX/` run-output directory — with one bounded carve, added at the V3 fix round, for the
in-flight collision clause 10 obliges. Body 6,652 / description 686, unbudgeted at birth,
hard-cap-only (`.mochiko/memory/primitive-cost-budgets.md`).

Retirement mechanics and re-points: `commands/plan.md` and `schemas/plan.yaml` deleted, seven
primitives re-pointed and three crate files re-keyed to the seven remaining pipeline schemas.
P3's library sweep re-pointed 34+ files, including the `review-plan-artifacts` title family
across six sites (slug retained, so every mount stays valid), `authoring-user-stories`,
`authoring-feature-map`, `authoring-architecture-store`, and `architecture-store.yaml`;
`authoring-technical-requirements` re-scopes rather than retires (the TR mandate dies, the
C/D/IP grammar survives), and `testing-gap-finding`'s fence inclusion list re-keys
`requirements.md` to the sufficiency report plus the design-phase deltas. Five persona lines
across three agents shed dead plan-run vocabulary (`tech-lead`, `devils-advocate`,
`technical-analyst`); the pinned KM twin's landing-ritual command list drops `plan/`
(`.mochiko/memory/knowledge-management.md:42`). Root docs `ARCHITECTURE.md` and `README.md`
were re-drawn for the plan-less pipeline and, in the same pass, corrected for pre-existing
v0.81.0 staleness: version stamp v0.48.0 → v0.91.0, skills 34 → 38, templates 14 → 7 +
`constitution-modules/` recording the artifact schemas' v0.76.0 re-home to `schemas/*.yaml`,
the `/mochiko:architecture` desk added to the cluster map and the command table (absent from
both docs since it shipped), `nfrs.md` dropped from the product-baseline list, and `PLAN_STOP`
removed from the kill-switch set.

Pre-wave dry-run (the V8 obligation, run before the build): the check went against kinako work
row R1-2 and **discriminated** — 3 clauses sufficient, 6 gap across four distinct causes, 1
sufficient only under a named confound; neither falsifier arm tripped. Six instrument defects
surfaced and were user-ruled into the clause text as the A1–A6 addendum. This is the first
discrimination datapoint for the D5 watch.

Author≠grader audits: three independent validator seats, **all three FAIL at round 1** (V1 1
blocking · V2 5 blocking + 2 gate gaps · V3 3 blocking) → batched fix rounds across four
producers → **V1, V2, V3 all CONFIRMED-PASS**. V1's B1 was a record-fidelity catch: the
design-phase authoring proposal and the architect's contest brief died at D4 and had been
carried forward regardless (B2/B3 one-liners). V2 closed N1–N4 plus the title family; V3's B1
found the sufficiency fence contradicting its own clause 10. Two declared budget overages both
ruled **HOLDS**: `patterns-vertical-tdd` +294 (D1 mechanic c) and `authoring-feature-map` +562
total / +128 this wave (mechanic e, byte-reconciled). 83 strip entries across 40 strip files,
68 fragments validator-verified. `cargo test` 12/12.

## [0.90.0] — 2026-08-26

`validation-constitution` user-ruled true-deletion body cut (ADR
`.mochiko/decisions/2026-08-26-validation-constitution-true-deletion-cut.md`; `DECISIONS.md`
2026-08-26 row; strip `.mochiko/strips/validation-constitution.md` [v0.90.0]). Body 7,630 →
5,103 chars (−33.1%), five paragraphs; shallow by structure (v0.63.0 benchmark already cut
−44%): yield = the three anti-rationalization table forms into Floors clauses + the
VALIDATION RESULT fenced block compressed field-complete. Every rule survives (69-entry
inventory, 1 restoration); `description:` + both references untouched. Audit PASS round 1,
69/69 homed, VALIDATION RESULT verified field-by-field, reference-to-body pointer sweep
clean (`evals/validation-constitution/audit-v0.90.0.md`). Budget re-seeded 5,103/6,379
(R11). **Compression-series close-out: every `review-*`/`validation-*` skill is now
ruled-cut; further reduction routes to the eval-graded pilot path.** `cargo test` 12/12.

## [0.89.0] — 2026-08-26

`review-governance-intent` user-ruled true-deletion body cut (ADR
`.mochiko/decisions/2026-08-26-review-governance-intent-true-deletion-cut.md`;
`DECISIONS.md` 2026-08-26 row; strip `.mochiko/strips/review-governance-intent.md`
[v0.89.0]). Body 8,150 → 5,562 chars (−31.8%), five paragraphs; shallow by structure (the
v0.63.0 benchmark wave already stripped −46% — the baseline was the keep-set + the v0.65.0
adaptive-depth ruling). Every rule survives (70-entry inventory, 3 restorations:
traceable-contract, lead-introduces + four-message, floor-class qualifier); `description:` +
the three shared reference pointers untouched. Audit PASS round 1, 70/70 homed, CROSS-EXAM
substrate-binding contract verified, stale-pointer sweep clean
(`evals/review-governance-intent/audit-v0.89.0.md`; advisory: the description's delta-pass
clause has had no body home since v0.63.0 — pre-existing, flagged for a future ruling).
Budget re-seeded 5,562/6,953 (R11). `cargo test` 12/12.

## [0.88.0] — 2026-08-26

`review-specifications` user-ruled true-deletion body cut (ADR
`.mochiko/decisions/2026-08-26-review-specifications-true-deletion-cut.md`; `DECISIONS.md`
2026-08-26 row; strip `.mochiko/strips/review-specifications.md` [v0.88.0]). Body 12,184 →
6,187 chars (−49.2%), single file, six paragraphs — the compression series' shallowest
floor, recorded as the boundary case: zero reference files (all 18 ruled checks + the
six-class canonical taxonomy body-only) and twice-compressed already; future candidates
sized by prose-over-rules ratio, not raw chars. Every rule survives as a compressed clause
or behind an explicit single-source pointer; `description:` untouched. Deeper cuts declined
at the gate with the rule deaths named. 81-entry rule inventory non-compressor-authored
pre-gate, 5 clause restorations (R-012/R-015/R-064/R-065/R-066). Author≠grader audit PASS
round 1, no blocking findings, 81/81 rules homed
(`evals/review-specifications/audit-v0.88.0.md`). Budget re-seeded 6,187/7,734 (R11); eval
slot joins the post-cut regression set. `cargo test` 12/12; schemas untouched.

## [0.87.0] — 2026-08-26

`review-plan-artifacts` user-ruled true-deletion body cut (ADR
`.mochiko/decisions/2026-08-26-review-plan-artifacts-true-deletion-cut.md`; `DECISIONS.md`
2026-08-26 row; strip `.mochiko/strips/review-plan-artifacts.md` [v0.87.0]). Body 13,521 →
4,901 chars (−63.8%), single file, six paragraphs: every behavioral rule survives as a
compressed clause or in its untouched single-source reference; `description:`, both
`references/` files (five pointer re-labels only), and the Tier-1 checker untouched. The
user targeted −90% on the `review-brainstorm` v0.83.0 precedent and ruled ship the
rule-complete cut at the ratification gate — deeper cuts declined with the rule deaths
named (−82%: Incremental Mode doubly-KEPT + cycle-card qualifiers + severity floors;
−90%: also the body-only cycle-card set and the consumer-cited adopt-first lens). 113-entry
rule inventory non-compressor-authored pre-gate (`evals/review-plan-artifacts/rules.json`);
author≠grader audit FAIL round 1 (4 blocking — three stale reference→body pointer labels +
the false no-dead-pointers claim) → fix round → delta-verify PASS
(`evals/review-plan-artifacts/audit-v0.87.0.md`; 113/113 rules homed, none lost). Budget
re-seeded 4,901/6,127 (R11); eval slot joins the post-cut regression set. `cargo test`
12/12; schemas untouched.

## [0.86.0] — 2026-08-26

Persona hygiene pass — two primitives, one wave (ADR
`.mochiko/decisions/2026-08-26-persona-hygiene-pass.md`; `DECISIONS.md` 2026-08-26 row).
Driver: the post-v0.84.0 sweep of the other nine personas for validator-class defects.
`requirements-analyst`: the `## Skills Available` section rewritten to the ruled precedent
form (single-source framing + one routing line per mount, the devils-advocate v0.25.0
Tier-1 precedent) — its bullets restated FR/SC and story-format internals the mounted
skills single-source; strip `.mochiko/strips/requirements-analyst.md` [v0.86.0]; audit PASS
round 1. `devils-advocate`: the frontmatter `description:` re-aimed at the full remit
(spec-only wording lagged five mounted review targets); the output tail scoped
"severity-ranked document findings" at the audit fix round (the runtime gap-finding pass
splits findings by kind, never severity); 316 → 384 chars against the unchanged 395 budget;
supersedes the v0.63.0 kept description prose by ruling; strip
`.mochiko/strips/devils-advocate.md` [v0.86.0]; audit FAIL round 1 (2 blocking: over-scoped
output tail · a missed `implement.md:115` consumer in the records) → fix rounds → delta
PASS. `tech-lead` Three-Part-Rule finding declined as carried judgment; six personas clean.

## [0.85.0] — 2026-08-26

`validator` persona: the v0.84.0 closing paragraph's router-registration sentence cut
(Tier-1 altitude strip, user-ruled on a same-day challenge; strip
`.mochiko/strips/validator.md` [v0.85.0]; ADR addendum
`.mochiko/decisions/2026-08-26-validator-router-indexed-checklists.md`; `DECISIONS.md`
2026-08-26 row). The sentence restated the router's "Adding to the library" rule inside a
runtime persona; the maintainer rationale is re-homed to the ADR addendum. The
delivery-not-scope sentence survives as the standalone closing line. `description:` 269/337
unchanged; everything else byte-for-byte. Author≠grader audit PASS round 1 (5/5, deterministic
verbatim + remainder checks).

## [0.84.0] — 2026-08-26

`validator` persona re-indexed to the router (ADR
`.mochiko/decisions/2026-08-26-validator-router-indexed-checklists.md`; `DECISIONS.md`
2026-08-26 row; strip `.mochiko/strips/validator.md` [v0.84.0]). The `## Skills you lean on`
inline one-item checklist list — stale "drafted constitution" wording, restated
`validation-constitution` internals, shape read as scope by the maintainer — superseded by a
three-step selection order: the dispatch brief's named bar wins · otherwise Read the `mochiko`
router (`skills/mochiko/SKILL.md`) for the domain-matching checklist (`validation-*` natively
the validator's; a `review-*` bar lent checklist-only, verdict stays binary, clearing stays
with the dispatching contract) · no fit falls back to the generic method against the handed
bar. `skills: validation-constitution` mount kept, framed delivery-not-scope; new checklist
skills now register in the router only — no persona edit per checklist. `description:` (269
chars, budget 337, unchanged), Iron Law, verdict form, and all other sections byte-for-byte;
v0.45.0/v0.63.0 protected sets reconciled in the ADR. Author≠grader audit PASS round 1 (5/5
items; two non-blocking advisories: ADR lacks labeled Rationale/Alternatives sections; the
repo-relative router path form is a pre-existing library-wide convention question).

## [0.83.0] — 2026-08-26

`review-brainstorm` user-ruled true-deletion body cut (ADR
`.mochiko/decisions/2026-08-26-review-brainstorm-true-deletion-cut.md`; `DECISIONS.md`
2026-08-26 row). Body 11,754 → 2,497 chars (−78.8%), single file: every behavioral rule
survives as a compressed clause, all rationale prose and long-form tables deleted; no
relocation — a breakup into a new `references/REVIEW-PROTOCOL.md` was drafted and rejected by
the user mid-pass as verbosity-shifting. `description:` and all three `references/` files
untouched. Strict −90% declined at the ratification gate with six ruled-rule deaths named.
Supersedes the v0.26.0 whole-body `KEPT:` ruling and the v0.64.0 floor line's verbatim wording
(substance intact); v0.60.0/v0.67.0 protected machinery survives compressed. Author≠grader
audit FAIL round 1 (7 blocking — headline: the lens taxonomy deleted while both gate uses
remained) → fix round → delta PASS. Body budget re-seeded 2,497/3,122. The skill joins the
post-cut regression-check set beside `review-feasibility` ("cut now, eval validates later");
pass artifacts at `evals/review-brainstorm/`.

## [0.82.0] — 2026-08-22

Verbosity-envelope enforcement (ADR
`.mochiko/decisions/2026-08-22-verbosity-envelope-enforcement.md`; `DECISIONS.md` 2026-08-22
row). Driver: the kinako EPIC-001 dogfood run wrote ~906KB of design-phase markdown in one
day — the fattest artifact classes had no format home, the strict templates were never
delivered to seats, and reviewers were doctrine-forbidden from grading bloat.
`artifact-format.md` → v3: scope widened to command-minted deliverables (epic proposal,
contest brief, architecture delta, specify's `derivation.md` — envelope-by-default);
undisclosed/unjustified overage past the size defaults now a gradeable **advisory** finding
(brevity stays never-a-finding); new rule 13 — no process self-narration, provenance is one
header line. `report-format.md` → v3: rule 9's mechanical prose-bounce widened to every
lead-collected report class, with the per-class payload-home reading. `plan.md`: new Report
envelope + Proposal & contest shape Tools bullets (pathed template bindings; a
baseline-reconstruction review is a report, never a freehand essay), epic-spine artifacts
bound to the deliverable envelope, seat briefs carry the register.
`feasibility-report-template.md`: `hunt_coverage` frontmatter field — the bounded
proof-of-hunt home. `output-style.md` + `review-specifications`: aligned to the v3 position.
**`review-feasibility` user-ruled 90% cut with breakup** ("cut now, eval validates later"):
body 18,959 → 1,893 chars (−90.0%), floors + a mandatory-load dispatch line;
unique content relocated into `references/FEASIBILITY-LENS.md` (Class 7 section, architecture
pass renumbered A1–A3, gate-fuel fields repaired to `gap/at/impact/fix`, merged reviewer
guardrails); body budget re-seeded 19,058 → 2,367 (R11); the skill-compression eval pilot for
this skill re-purposed as a post-cut regression check. Audit: independent `mochiko:validator`,
FAIL round 1 (4 blocking — headline: the disclosure floor had no envelope-legal home) → fix
round → PASS; 8 strip files touched, KEPT/AD-D7 reconciliation verified; `cargo test` 12/12.

## [0.81.0] — 2026-08-19

Product-architecture store, Stage 1 (`product-architecture-schema` D1–D16, `DECISIONS.md`
2026-08-19 row; Stage 2 — frontend/mobile/desktop shelves — remains queued per D15).
Architecture becomes a first-class living-desk workflow over one schema-backed store at
`.mochiko/product/architecture/` (spine + AX-XXX concern catalog, element lifecycle
`ruled` → in-flight-class → `built`), with repo-root `ARCHITECTURE.md` re-chartered as the
store's derived index. New: `/mochiko:architecture` desk command (charter form) ·
`authoring-architecture-store` + `patterns-architecture-shelves` skills ·
`architecture-store.yaml` (ninth rendered schema) + `architecture-shelf-backend.yaml`
(13 memory-asserted dimensions + 3 topology-spine opinions, floor-bound rows 1/5/13).
Transformed: `patterns-system-design` (altitude + diagram craft now serves store deltas;
protected no-delta and altitude lines carried). Retired: `authoring-architecture` (Duty 1
landing diff inherited by the store skill on the approved-delta-existed trigger, both
directions) and the orphaned `architect-report-template.md`. Re-keyed: plan (omit-architecture
escape dead; metered consult contract; sign-off = the store write gate) · implement (deviation
gate + landing flips + graded `As-built:`/`Drift:`) · setup (unconditional store scaffold with
`Scope:` stub) · feature (growth-door intake) · plan/feature-entry/features-index schemas ·
both plan review skills · router ×10+ rows · `patterns-sound-loop` governing surface ·
`authoring-technical-requirements` (structural D-XXX die into store deltas; `nfrs.md` retired
as an artifact class, NFR-XXX ids re-homed on concern rows) · KM module + pinned invariant
(AT-D6-C In-flight agreement superseded by the orphan rule, store-less carve keyed to ruled
content). 4 plan-approved producer seats (disjoint ownership) + 4 fresh author≠grader
validators under the sound-loop + transport floors; all four clusters FAIL round 1 → fix
rounds → 4/4 unconditional PASS; ~45 supersession strips across 20+ strip files, all quoted
spans byte-verified; ledger swept at the gate (15 measured / 14 matched / 1 drift corrected);
sole budget overage `authoring-feature-map` +434, declared and ruled HOLDS; `cargo test` 12/12.

## [0.80.0] — 2026-08-19

Slice-vocabulary purge (ADR `.mochiko/decisions/2026-08-19-slice-vocabulary-purge.md`;
`DECISIONS.md` 2026-08-19 row). A post-v0.75.0 dogfood run reified "slice" as a unit noun
beside cycles; the unit died at v0.57.0 and the noun outlived it. "Slice" now leaves every
surface where it names a pipeline/TDD unit — replacement language "vertical increment" /
"bundle" / "cycle" — while generic-English and gerund uses ("slicing judgment") and the
`'vertical slice'` routing trigger stay. `patterns-vertical-tdd`:
`references/SLICE-IDENTIFICATION.md` renamed `BUNDLE-IDENTIFICATION.md` (heuristics intact),
description and body re-worded, `TEST-GRAMMAR.md` anti-pattern line re-worded
(vocabulary-only, grammar ownership unchanged per v0.75.0 D4). `schemas/tasks.yaml`: "one
vertical slice" → "one vertical increment" (contract + skeleton). The dead `slice: <s#>`
report-envelope field deleted (`templates/report-format.md` + both reference echoes). Stale
riders retired in the same wave: the router's foundation-vs-feature clause (dead since
v0.75.0 D3), devils-advocate's Delivery-Slices clause (section removed at v0.58.0), the
advocate-template's tasks/slice seat list (seats folded at v0.49.0), and
`authoring-architecture`'s pre-v0.61.0 `.mochiko/specs/<feature>/` paths (now
`.mochiko/features/FEAT-XXX/`). 12 primitives, 10 strip files, 2 producer seats + 2
cross-grading validators; declared +12 body overage ruled HOLDS and the v0.76.0 +86 silent
drift ledgered retroactively; `cargo test` 11/11.

## [0.79.0] — 2026-08-19

QA gap-finding build (`qa-gap-finding-verification` D1–D10 as amended; `DECISIONS.md`
2026-08-19 row). The QA verification lifecycle gains its discovery layer: the deterministic
floor (TEST-gate re-execution + exit-code quality gates) stays, and final validation gains a
**blind, spec-derived gap-finding pass** — selection-scope and epic runs only, delta/lane
runs disclose the skip.

- **New skill `testing-gap-finding`** — the blindness fence (explicit inclusion list:
  `spec.md`, `requirements.md`, Screens & Flows, `data-model.md`, `contracts/`, `nfrs.md`;
  code/cards/`**TEST:**` cases structurally excluded, delegated reads inherit the list),
  two-message blind dispatch, expectation derivation, seven-family probe kit (adversarial ·
  state walks · contract · concurrency · security/abuse · runtime NFR · observability),
  diff-scoped high-depth-only mutation lens (tool-absent/flaky-suite skips disclosed;
  non-kernel per GI-019), finding-kind split with lead adjudication, done condition
  (expectations probed, counts disclosed, zero findings = clean), and the fold-back into
  the new durable gate-set artifact `.mochiko/features/FEAT-XXX/gates.md`.
- **`implement.md`** — final-validation wiring: scope carve + skip disclosure, blind
  dispatch contract, gap-rework whole-run bound (default 2, redeclarable only at run open),
  checkpoint adjudication, out-of-territory routing to `/mochiko:feature`, fold-back at the
  acceptance landing, two new default-FAIL clauses; the "accumulated territory `**TEST:**`
  gates" read re-pointed to its now-named home (union of territory `gates.md` + cards'
  cases). 2 supersession strips (`.mochiko/strips/implement.md` [v0.79.0]).
- **`devils-advocate`** — sits the exploratory seat (D4 reseat): `skills:` +
  `testing-gap-finding`, new runtime-hunting persona section; both never-zero
  finding-count lines scoped to document review (2 supersession strips,
  `.mochiko/strips/devils-advocate.md` [v0.79.0]).
- **`qa-engineer`** — fold-back authoring duty (gates.md cases at the acceptance landing);
  explicitly not the exploratory seat.
- Router row + agent-row sync; ledger birth rows (skill body 10,559 / desc 709, ruled
  HOLDS).
- Audits: 2 producer seats on lead-approved plans (disjoint ownership) + 2 fresh
  author≠grader validators; V1 PASS + 5 advisories producer-applied + seven-span confirm;
  V2 cluster FAIL (F1 second never-zero carrier · F2 fence delegation guard) → fix round →
  CONFIRMED-PASS. Gates: cargo test 11/11, no schema files touched.
- Housekeeping: the v0.78.0 wave's strip-file intro splits repaired across 12
  `.mochiko/strips/` files (pure relocation, line-multiset-verified); stale explorer
  ledger row annotated historical.

## [0.78.0] — 2026-08-19

Explorer retarget — the v0.77.0 cheap rung superseded by recorded ruling (ADR
`2026-08-19-explorer-retarget-native`; `DECISIONS.md` 2026-08-19 row). Dogfood failure:
agent-team teammates cannot spawn plugin-scoped agents, so the `mochiko:explorer` dispatch
the persona delegation channel prescribed failed on exactly the transport it was built for.
The cheap rung is now the **native `Explore` agent spawned with an explicit `model: haiku`
override** — the override is the pin; a bare spawn inherits the session model and fails the
floor. The class key, dispatch ladder, brief obligation, weak-negative watch, D1 economics,
and D5 (rostered seats stay `model: opus`) all stand unchanged.

- **`mochiko:explorer` deleted** — agent file, `plugin.json` agents entry, and the router
  agent-table row. Its fact-finder constraints (terse `file:line`-provenanced facts,
  verbatim quotes, method-scoped absence, no interpretation) move into the dispatch brief
  via `patterns-model-tiering`'s new "Fact-finder brief" clause.
- **`patterns-model-tiering` reworked** — description and body retargeted to native
  `Explore` + override; "The frontmatter is the pin" superseded by "The override is the
  pin".
- **Six command floor lines + ten persona `## Delegating Cheap Reads` sections reworded**
  to the native dispatch; router skill row reworded.
- 20 supersessions recorded in `.mochiko/strips/` (new files: `patterns-model-tiering.md`,
  `explorer.md`, `tech-lead.md`); author≠grader audit per the edit ceremony.

## [0.77.0] — 2026-08-16

Model-tiering dispatch floor — the deferred `model-tiered-seats` D4 build, executed and
retargeted (the July record's ~10 "native Explore" dispatch sites were refactored away at the
v8 command rebuild; the retarget is command-level floor lines plus a brief-injection
obligation, ADR `2026-08-16-model-tiering-build`). Rostered personas stay `model: opus` (D5
untouched); cheap-tier work moves to a scoped disposable seat. Mechanism confirmed empirically
this session: an opus-pinned persona spawning a subagent with a haiku override ran
`claude-haiku-4-5-20251001` and returned a correct targeted read. Pure additions — no strips;
author≠grader audit per the edit ceremony.

- **New agent `mochiko:explorer`** (`agents/explorer.md`, `model: haiku` pinned in
  frontmatter) — disposable-per-gap cheap-tier fact-finder: locate / enumerate /
  targeted-read / deterministic checks; terse spot-checkable returns with file:line
  provenance, method-scoped absence reporting, never interprets, never writes. Registered in
  `plugin.json` and the router agent table.
- **New skill `mochiko:patterns-model-tiering`** — single source of the class-keyed dispatch
  floor: locate/enumerate/targeted-read gaps ride `mochiko:explorer`; interpretive gaps,
  decision-driving absences, and completeness-sensitive enumerations stay session tier (the
  D4 class key + F2 guide-line); the dispatch ladder (direct tool call → cheap explorer →
  session-tier read), disposable-per-gap lifecycle (F5), the weak-negative watch, and the
  brief obligation — every seat brief carries the routing rule, the only channel that reaches
  spawned teammates (they never load `skills:` frontmatter). Third sibling of the sound-loop
  and transport floors.
- **Six command floor lines** — `plan.md`, `implement.md`, `feature.md` (Ways of Working) and
  `specify.md`, `setup.md`, `brainstorm.md` (Harness) each gain one model-tiering line
  referencing the skill, never restating it.
- **Ten persona standing sections** — every rostered agent body gains a uniform
  `## Delegating Cheap Reads` section (explorer excluded): spawn a disposable
  `mochiko:explorer` subagent for locate/enumerate/targeted-read/deterministic-check gaps,
  one gap per spawn; interpretive, absence-driven, and completeness-sensitive reads stay the
  seat's own. Persona body is the one channel reaching seats on both transports (teammates
  drop `skills:` frontmatter but load the persona); names no command or pipeline stage, so
  decoupling-by-absence holds. Platform basis: teammates and subagents may spawn nested
  subagents (depth cap 3; teammates foreground-only).
- **Router updates** — `patterns-model-tiering` skill row + `explorer` agent row in
  `skills/mochiko/SKILL.md`.

## [0.76.0] — 2026-08-16

Template-schema CLI build — the accepted `schema-based-template-guidance` record (D1–D11 as
amended at review) landed as one wave under the sound-loop and transport floors: six producer
seats with strictly disjoint file ownership (schemas · crate + CI · template deletions + strips ·
command re-points · skill re-points + D7 re-key) and three fresh author≠grader validator seats.
Pipeline artifact template guidance moves from static `.md` exemplars to schema data files
rendered by mochiko's first Rust crate; the plugin still installs and functions markdown-only
(GI-020), reading the schema data raw when the binary is absent (D8). Audit tally: V1 crate PASS
plus delta-confirm, V2 8/8 schema fidelity plus 8/8 strips, V3 13/13 re-points, the
`authoring-feature-map` +450-char budget overage ruled HOLDS; one fix round (CI `cargo audit`
raised to `--deny warnings`).

- **First Rust crate `crates/mochiko-cli`** — mochiko's first non-markdown code and the owned
  foundation seed for future native tooling (D6, Tauri-bound). Two dependencies (`serde` derive,
  `serde_norway` for YAML — `serde_yml` void per RUSTSEC-2025-0068), no `clap`/`anyhow`. Surface:
  `mochiko-cli template <name>` (producer view: schema + example + good/bad guidance) and
  `mochiko-cli template <name> --check` (checklist view — a view, not a linter; takes no artifact
  input and stays provably advisory under the D11 bright line). Runtime schema resolution reads
  the shipped `plugins/mochiko/schemas/*.yaml` as the source of truth, falling back to the
  compile-time embedded copy only for run-from-anywhere. 11 tests. CI at
  `.github/workflows/ci.yml` (the repo's first executable gate) runs `cargo test`, `cargo fmt
  --check`, `cargo clippy -- -D warnings`, `cargo audit --deny warnings`, and a secret-scan step,
  on push/PR touching `crates/**` or `plugins/mochiko/schemas/**`.
- **Eight schema data files at `plugins/mochiko/schemas/`** (D8: data = source of truth, binary =
  renderer, raw Read the first-class degraded path) — `spec.yaml`, `plan.yaml`, `tasks.yaml`,
  `feature-entry.yaml`, `features-index.yaml`, `codebase-analysis.yaml`, `governance-intent.yaml`,
  `governance-surfaces.yaml`. YAML chosen for raw-Read legibility on prose-heavy templates. One
  source renders both the producer view and the `--check` checklist view; the per-section `check`
  lines are net-new authored guidance under D7's authority, graded by V2/V3.
- **Eight template supersessions with byte-exact strips** — the eight pipeline artifact templates
  (`spec-template.md`, `plan-template.md`, `tasks-template.md`, `feature-entry-template.md`,
  `features-index-template.md`, `codebase-analysis-template.md`, `governance-intent-template.md`,
  `governance-surfaces-template.md`) are superseded-by-ruling into their schema equivalents, each
  with a verbatim strip entry (`.mochiko/strips/<template>.md`). The two doctrine-dense schemas
  (`governance-intent`, `governance-surfaces`) preserve every operative line verbatim, audited by
  `validation-constitution`.
- **Fourteen primitive surfaces re-pointed** — every read-pointer to an in-scope template swaps to
  the two-arm form (`invoke mochiko-cli template <name>; if absent, Read
  plugins/mochiko/schemas/<name>.yaml` — the fallback is D8-first-class). Thirteen are two-arm
  re-points across `commands/{specify,plan,feature,setup}.md` and the skill/reference surfaces
  (`authoring-prototype`, `authoring-feature-map`, `analysis-codebase` + `CONTEXT-GATHERING.md`,
  `patterns-vertical-tdd`, `authoring-constitution` + `INTERROGATION-AGENDA.md`,
  `validation-constitution/references/QUALITY-CHECKLIST.md`, `templates/output-style.md`), each
  with a supersession-by-ruling strip entry. The `mochiko/SKILL.md` router rows (spec/tasks/plan)
  are re-typed `(schema)` rather than swapped, keeping discoverability intact (I4 carve-out).
- **Thin D7 re-key** — only the in-scope-template checklists cite the `--check` view: the
  tasks cycle-card criteria (`review-plan-artifacts`) and the governance-surfaces structure
  (`validation-constitution/references/QUALITY-CHECKLIST.md`). Out-of-scope artifact checklists
  are left untouched (D3 boundary).
- **Governance activation, PATCH 2.0.0 → 2.0.1** — pre-authorized at AM-1, activating the dormant
  crate-gate clauses now that the crate has landed (no fresh `/mochiko:setup` amend): GI-012 gains
  `cargo test` PASS as release gate 6 and schema-data/binary consistency on the marketplace-sync
  gate; GI-002 tech-stack line re-expressed (Rust crate, compiled binary, CI present); GI-003
  un-narrowed (CI secret-scan now exists); GI-004/GI-007 re-expressed (the crate carries a real
  `cargo test` suite coexisting with the prose audit ratchet). One amendment-log activation row in
  the ledger; the `.claude/rules/mochiko/primitive-edits.md` path scope now covers
  `plugins/mochiko/schemas/**` (schema files are shipped primitives).

### Changed

- Plugin identity reworded from "Kernel-free agent-skill framework" to "Skills-first agent
  framework" (`plugin.json` + `marketplace.json` descriptions; the `kernel-free` keyword →
  `skills-first`) — the "kernel-free" claim went stale when the D11 softening admitted the Rust
  crate (governance v2.0.0, GI-019); user-ruled rider at this landing. Pre-existing manifest
  description divergences left as-is. Strip: `.mochiko/strips/plugin-manifest.md`.

### Removed

- `plugins/mochiko/templates/spec-template.md` — superseded by `plugins/mochiko/schemas/spec.yaml` + `mochiko-cli`
- `plugins/mochiko/templates/plan-template.md` — superseded by `plugins/mochiko/schemas/plan.yaml` + `mochiko-cli`
- `plugins/mochiko/templates/tasks-template.md` — superseded by `plugins/mochiko/schemas/tasks.yaml` + `mochiko-cli`
- `plugins/mochiko/templates/feature-entry-template.md` — superseded by `plugins/mochiko/schemas/feature-entry.yaml` + `mochiko-cli`
- `plugins/mochiko/templates/features-index-template.md` — superseded by `plugins/mochiko/schemas/features-index.yaml` + `mochiko-cli`
- `plugins/mochiko/templates/codebase-analysis-template.md` — superseded by `plugins/mochiko/schemas/codebase-analysis.yaml` + `mochiko-cli`
- `plugins/mochiko/templates/governance-intent-template.md` — superseded by `plugins/mochiko/schemas/governance-intent.yaml` + `mochiko-cli`
- `plugins/mochiko/templates/governance-surfaces-template.md` — superseded by `plugins/mochiko/schemas/governance-surfaces.yaml` + `mochiko-cli`

## [0.75.0] — 2026-08-16

Vertical-TDD test-case-anchor build — the accepted `vertical-tdd-complexity-and-qa-role`
record (D1–D4 as amended at review) landed as one wave under the sound-loop and transport
floors: two producer seats on lead-approved plans with disjoint file ownership (TDD-skill
surfaces vs roles/wiring), all messaging through the lead, two fresh author≠grader validator
seats cross-grading the quiesced tree — 12/12 surfaces PASS round 1, zero fix rounds.
Version 0.74.0 is reserved by the concurrent template-schema CLI wave on `mochiko-cli`;
this build stamps 0.75.0 to avoid strip renumbering at merge (four files overlap — reconcile
there, per the record's coordination note).

- **`patterns-vertical-tdd` re-keyed to the test-case-bundle anchor (D1)** — a cycle is a
  coherent bundle of named test cases (expected behaviour, in the `**TEST:**` grammar)
  demonstrated green to the user on real infrastructure; walking-skeleton standing
  first-cycle rule with the greenfield/new-path carve; time-based sizing (1–3 sessions /
  1–3 hours / <30-min merge bar) superseded by "worth demonstrating"; the proliferating
  worked examples (CRUD=4 / Search=6 / Auth=7 foundation stacks) replaced by one
  skeleton-first example.
- **Foundation/feature card type dies (D3)** — the skeleton absorbs sequencing; IP-XXX
  platform infrastructure homes in the skeleton cycle (skeleton-path) or the first bundle
  needing it; infra-only cards never minted; `[P]` derives from dependencies, not a type
  column. `tasks-template.md` and `TASK-PARSING.md` re-keyed to match; acceptance-ID trace
  relocated into each test case as a card-level `**Covers**:` line (`TEST-GRAMMAR.md`
  untouched per D4 — `Covers` is a trace convention, not a grammar field).
- **qa-engineer gains its design-time seat (D2)** — the persona authors the cycle cards'
  acceptance test-case content in the grammar it later executes; the slicing judgment
  (bundles, Simple/Split/Merge, walking-skeleton, dependencies) stays with the design seat
  (I7 split); `plan.md` wires the QA producing seat; no `skills:` change — persona judgment
  plus `testing-end-user`'s grammar carriage resolve the record's seat-mechanics open
  question minimally.
- **`review-plan-artifacts` re-keyed (D1/D2/D3)** — sizing and foundation-sequenced clauses
  retired; new oracle-semantics check: each card's Asserts graded semantically against the
  acceptance scenario/criteria they cite, not merely for presence and grammar;
  `ARTIFACT-CHECKLISTS.md` aligned.
- 6 supersession strips across 5 strip files, all v0.75.0-stamped, protected
  foundation/feature lineage reconciled in the owner strip. First-live-run watch owed (n=0;
  directional expectation: kinako-s1-shaped work well under 12 cycles).

## [0.73.0] — 2026-08-15

Adopt-first build — the accepted `build-vs-off-the-shelf` record (D1–D6 as amended at review)
landed as one wave under the sound-loop and transport floors: two producer seats on
lead-approved plans with disjoint file ownership (new skill + router + ledger vs the nine
pointer touches + strip), mesh-hold briefs, fan-in confirmation on every deliverable, two
fresh author≠grader validator seats on the quiesced tree — 13/13 artifacts PASS round 1, zero
fix rounds (two advisory alignments producer-applied post-audit).

- **New skill `patterns-adopt-first`** — single source for the build-vs-off-the-shelf
  discipline, born from the kinako FEAT-006 SQLite miss: every commodity-category decision
  ("is this problem older than this product?" — storage, locking, serialization, queueing,
  caching, auth, search; author framing never gates the check) names at least one real
  off-the-shelf candidate or an explicit "no shelf candidate exists" line (absence itself a
  review finding), and custom wins only over the named candidate in the written rationale.
  Two-sided (a named candidate may lose on merits; BE-DEP's <100-lines red flag a legitimate
  custom-wins rationale below the gate) · named candidates are external claims
  (`verified:`/`memory-asserted`, verified at review) · retrofit-cost gate — custom-over-shelf
  is user-ruled when expensive to walk back (persisted formats, storage engines,
  locking/concurrency, migration-bearing shapes) · constraint-challenge route-back — a shelf
  check colliding with a ratified constraint files a three-part finding routed to the user,
  only the colliding decision pauses · scope bound: in-process/self-hostable only, SaaS buy
  routes to IP-XXX + PM/user. Desc 497 / body 6,493, unbudgeted at birth (ledger note).
- **`patterns-plan-minimalism`** — rung 3 widened: "a baseline, the current system, an
  installed dependency, or an adoptable proven component (per `mochiko:patterns-adopt-first`)
  carries it: extend, reference, or adopt — never re-design" (supersession strip, new strip
  file).
- **Eight pure-addition pointer touches** — `patterns-technical-decisions` (alternative-set
  obligation + checklist line) · `patterns-code-minimalism` (rung-5 plan-commitment line) ·
  `review-feasibility` (class 7 also fires on needed-but-should-have-been-adopted) ·
  `review-plan-artifacts` (blocking adopt-first disclosure check as a named sibling lane at
  conformance strength + checklist line) · `authoring-requirements` / `review-specifications`
  (constraints state capabilities, never storage/implementation postures) ·
  `principal-architect` / `tech-lead` (cheaper-boxes persona lens, keystone test held) ·
  `plan.md` (Boundaries route-back bullet).
- Router row (Plan cluster) · budgets-ledger birth note · audit-added BACKLOG watch on the
  rung-3 read-obligation seam (checked at the kinako re-plan probe).

## [0.72.0] — 2026-08-14

Epic build — the accepted `multi-feature-plan-implement` record (D1–D13 as amended at review)
landed as one wave under the sound-loop and transport floors it inherits: two producer seats
on lead-approved plans with disjoint file ownership (skills vs commands), mesh-hold
approvals, fan-in confirmation on every deliverable, two fresh author≠grader validator seats
on the quiesced tree — 8/8 artifacts PASS round 1, zero fix rounds.

- **New skill `authoring-epic`** — single source for the epic, the transient first-class
  multi-feature delivery unit (`EPIC-XXX`): manifest + spine shape at
  `.mochiko/epics/EPIC-XXX/` (joint proposal · joint architecture/seam design with
  design-time cross-member seam owners · ordering · shared-baseline joint deltas authored
  once under a single pen-holder, transport-steer disclosure), the mint-once/overlap guard
  (desk mints · plan may mint inline via declare-and-contest · specify proposes, never
  mints · one open epic per feature's pending rows), selection-scope-only membership, and
  close semantics (each member's graduation batch + epic close; markers vanish, manifest
  stamped, dir persists as record). Desc 497 / body 7,503, unbudgeted at birth (ledger note).
- **`authoring-feature-map`** — two ruling-mandated additions: the `[EPIC-XXX]` row-marker
  grammar (vanishes with the row at its graduation fold) and the within-epic design-time
  seam-owner rule. Body 15,661 vs its 15,413 budget — the 248-char overage was declared and
  ruled HOLDS at audit (genuine new obligation, record D8/D13); ledger caution updated.
- **`plan.md`** — epic entry (`EPIC-XXX` lookup, or bare multi-feature list minting inline
  through declare-and-contest with the overlap guard), one-unit gates (one proposal · one
  contest · one joint architecture sign-off with per-seam owners · whole-package acceptance,
  per-feature verdicts as in-run amendment only, no partial planned exit + FAIL clause),
  in-epic dependency unblocking, Epic-spine Tools bullet (shared-baseline delta once under a
  single pen-holder). **1 supersession strip:** the "One run per capability-batch" charter
  line amended to admit epic composition — single-capability runs unchanged.
- **`implement.md`** — the epic run whole: entry gates on the accepted package + spine; one
  merged sequential cycle sequence with feature-tagged cards and per-feature reports; one
  cold-snapshot final validation with union-territory gates; one landing = each member's
  graduation batch + epic close, shared baselines folding once from the spine; member-scoped
  halts with the carve-out/hold disposition reserved to the user; multi-spec compositional
  closure. Pure additions.
- **`feature.md`** — desk epic stewardship (mint, membership, status, close; mint-once,
  overlap-guarded) beside — never inside — the sacred capability writes. Pure additions.
- **`specify.md`** — selection-stage epic proposal: the PM seat may propose, never mint.
  Pure addition.
- **Router** — one `authoring-epic` discoverability row.
- Ripple: `pm-role-and-feature-derivation` D7's capability-batch pipeline-key clause
  annotated amended on its `DECISIONS.md` row; budget-ledger notes updated; `ARCHITECTURE.md`
  skill counts corrected to 33.

## [0.71.0] — 2026-08-14

Teammate-transport floor build — the accepted `teammate-message-races` record (D1–D7 as
amended at review) landed as one wave run under the floor it ships: two producer seats on
lead-approved plans with disjoint file ownership (no shared write surface by construction),
mesh-hold approvals, fan-in confirmation on every deliverable, two fresh author≠grader
validator seats on the quiesced tree.

- **New skill `patterns-transport-floor`** — `patterns-sound-loop`'s sibling on the transport
  axis: the split trigger (message legs 3/4/6/7 on any multi-seat run with cross-seat or
  lead-relayed messaging, shared writes or not; topology legs 1/2/5 on a shared write
  surface; each lane non-waivable once fired), the seven legs (composition steer — binding
  per D4: worktree-isolated writers or a single pen-holder · single writer per surface ·
  mesh hold · content-pinned supersession · quiesce before cold grade · no ritual
  sends/never re-send · fan-in confirmation), and the platform floor (agent-teams
  ≥ v2.1.224 — below it, sends reported success on failed mailbox writes; teammate delivery
  documented-automatic, ordering undocumented; the official ownership-split line as doc
  anchor). Desc 450 / body 5,398, unbudgeted at birth (ledger note).
- **`patterns-sound-loop`** — the Overview neutrality line narrowed: transport *choice* stays
  neutral (realignment D5), transport *use* now carries its own floor. 1 supersession strip
  (new strip file, the replaced sentence verbatim).
- **Commands** — one transport-floor pointer each, pure additions: v8 Harness bullet in
  `specify.md` / `brainstorm.md` / `setup.md`; charter Boundaries bullet beside the
  sound-loop pointer in `feature.md` / `plan.md` / `implement.md`.
- **Review clusters** — `review-brainstorm/references/CROSS-EXAM.md` gains the message-legs
  note on the paired exchange (references/, budget-exempt; SKILL.md untouched at 2,630 chars
  of body headroom).
- **Ripple**: router row · cost-budgets ledger notes (transport-floor at birth; sound-loop
  re-measured 5,849) · DECISIONS realignment-D5 row annotated (choice neutral *except where
  writes collide*).
- Driver on record: the kinako FEAT-006 specify run's five race classes; the session
  self-demonstrated the class four times live while ruling on it.
- Audits: 2 fresh author≠grader validator seats (skills cluster · 7-file commands cluster),
  2/2 PASS round 1, zero fix rounds.

## [0.70.0] — 2026-08-13

Sound-loop floor build — the accepted `charter-ritual-balance` record (D1–D7 as amended at
review) landed as one wave; the wave itself ran under the floor it ships: two producer seats
on lead-approved plans, two fresh author≠grader validator seats, the user's word opening and
closing it.

- **New skill `patterns-sound-loop`** — fourth discipline sibling: the kind-keyed trigger
  (judgment-authored × governing surface, no size threshold), the per-member regime table
  (four surfaces satisfied by construction; net-new bite: capability map · product baselines ·
  `ARCHITECTURE.md` folds outside landings), the three legs (a seat produces on a lead-approved
  plan, never the lead · independent non-author review — the user's ruling alone never
  substitutes · user gate), three exemptions (mechanical execution · transcription ·
  fix-on-sight) with the explicit no-delta-card-exemption rule (review leg before dispatch),
  the import-rituals adaptation for out-of-remit hosting, role-claim honesty, the default
  seat-wiring table (map: `product-manager` × `devils-advocate` with the spec-less desk
  procedure · architecture/baseline: `principal-architect` × `tech-lead` · desk delta cards:
  `devils-advocate`), and the pinned disclosure grammar (`floor: tripped|clear · seats: …`).
  Desc 500 / body 5,729, unbudgeted at birth (ledger note).
- **`feature.md`** — Boundaries sound-loop floor bullet · Tools delta-cards review-leg rider ·
  the capability-writes bullet gains the D6 grooming-door ceiling (merge · retire · status ·
  extent-tidy of **existing** entries; wholesale re-derivation routes to specify; explicit-user
  out-of-remit hosting names the crossing and imports the home rituals; mint enumerated out of
  the door) · staffing freedom narrowed "below the sound-loop floor". 2 supersession strips.
- **`plan.md` / `implement.md`** — one Boundaries floor pointer bullet each; their run shapes
  already satisfy the floor — the pointer makes it doctrine, not habit. Pure additions.
- **Ripple**: router row (Specify cluster, beside `patterns-map-minimalism`) · charter audit
  re-key extended in `primitive-edits.md` (sound-loop pointer a graded floor check, all three
  charters; rulings list cites the record) · cost-budgets ledger note.
- **Deferred by ruling (D5, `Contested`)**: the path-injected rules-file leg stays unbuilt —
  the first observed floor miss in live use builds it immediately (BACKLOG carries the trigger).
- Audits: 2 fresh author≠grader validator seats (skill · 6-file cluster), 2/2 PASS round 1.

## [0.69.0] — 2026-08-13

Charter anatomy extended to the pipeline pair — ADR `2026-08-13-charter-plan-implement`
(direct ruling, no session): `plan.md` and `implement.md` join `feature.md` as six-section
charters; D10's "this command only" clause superseded; setup/specify/brainstorm stay v8, door
open for each on its own ruling. Behavior-preserving throughout — every prior obligation
re-homed, nothing added or dropped.

- **`plan.md` re-formats**: chartered **Delivery Manager of the goal** (Identity & Mission);
  the Adaptive Goal Protocol maps the existing gates — a literal **Entry** step, proposal
  approval as the convergence (the approved list = the run's done condition + default-FAIL
  floor; delta scope keeps its collapse), the done condition closing at package acceptance
  with the prior FAIL list verbatim; Bindings dissolve into Tools (referenced, never
  restated); the non-waivable floor (baselines never edited in place · architecture before
  detail · `[MODIFY]`-only breaks of delivered features · constitution never overruled)
  lands in Boundaries. Full prior text + re-home map in the strip.
- **`implement.md` re-formats**: same anatomy; run-open confirmation as the convergence
  (batch, scope type, attempt bound at its one redeclaration point, fixed done condition —
  no negotiation exchange); the v8 Bounds bullet splits by kind — attempt economy →
  Boundaries, delta re-verification method → Ways of Working, rounds/seats transparency →
  the DM floor; gates-never-severity-triaged and the-lane-never-widens land in Boundaries.
  Full prior text + re-home map in the strip.
- **Audit re-key broadened** (`.claude/rules/mochiko/primitive-edits.md`): the charter
  exception now covers both forms — per-visit goal contract (the desk) and per-run goal
  contract (the pipeline pair; four-point definition incl. the labeled Entry step and the
  surviving default-FAIL list). Both charters keep the literal **Entry** step so
  `feature.md`'s "name at their Entry" cross-reference holds with `feature.md` untouched.
- Ripple: router universal-anatomy lines re-worded to the two-anatomy reality (4 sites, own
  strip; repairs the v0.68.0-stale "Every command is goal + harness" claim) ·
  `ARCHITECTURE.md` Commands row / Command form / Feature section re-worded · the BACKLOG
  ARCHITECTURE-staleness item's command-anatomy clause resolved.
- Audits: 3 fresh author≠grader validator seats (plan · implement · router), 3/3 PASS round
  1; the implement strip fence byte-verified identical to the prior text; commands carry no
  char budget (hard-cap fallback), router `description:` unchanged.

## [0.68.0] — 2026-08-13

Capability-map rebuild — the accepted `pm-role-and-feature-derivation` record (D1–D12 as
amended at review, plus the user-ratified D8 build rider) landed as one four-cluster wave;
four fresh author≠grader audits + an audited lead ripple.

- **The map re-types** (`authoring-feature-map` + both feature templates): durable
  **capabilities** + transient **work rows** (pending|live) replace parent/leaf nesting —
  "feature" is reserved for capabilities; delivered rows fold into the capability's extent at
  acceptance landings and vanish, pending rows persist as the completeness view; invariants
  re-keyed (row-level dependency closure · capability status vs row state ·
  delivered-with-live-rows stickiness · capability-write sacredness · work-row integrity);
  retired entries may carry a merged-into pointer. 14 supersession strips.
- **New skill `patterns-map-minimalism`** — the map-minimalism discipline (third sibling):
  ranked capability tests (system's-language · durability · new-kind govern; noun+verbs a
  heuristic), extend-beats-mint, the ~9 soft cap with grooming trigger, capability-merge
  mechanics, and the dormant-domain paragraph (cap-trip only; PM proposes + architect
  co-signs — machinery deferred to the first real cap-trip).
- **PM moves to specify's front** (`specify.md`): the intent stage carries the PM's
  **capability frame** — a nouns-and-verbs hypothesis stories may overturn; after stories the
  PM confirms the frame, cuts work rows grouped per capability, and runs the filter; selection
  re-keys to work rows with the per-capability completeness view. `product-manager.md` gains
  the framing-first posture (persona stays workflow-trace-free).
- **`feature.md` becomes the product desk — the library's one charter-form command** (v8
  anatomy superseded for this command only; audit re-keyed to floor + per-visit goal
  contract, `.claude/rules/mochiko/primitive-edits.md`): Delivery-Manager lead, health report
  first, each visit converging to a done condition; advisory front door; the **growth door**
  cuts extend-verdict work rows under the capability-write test (mint/merge/retire/status stay
  sacred to specify or a user grooming ruling). Growth rows dispatch in selection scope,
  bug/improvement delta cards in delta scope (the D8 build rider).
- **Pipeline re-keys to the capability-batch** (`plan.md` / `implement.md`): one plan run per
  capability's selected rows, ordered by row dependency closure; implement's landing executes
  the row folds; the `.mochiko/product/` baseline machinery is byte-untouched (verified at
  audit).
- Ripple: router rows re-keyed + `patterns-map-minimalism` row (strips), `ARCHITECTURE.md`
  specify/plan/implement/data-flow sections re-keyed + a new Feature-desk section, cost-budget
  ledger notes (new skill unbudgeted; `authoring-feature-map` 23-char headroom caution).
- Audits: map cluster 3/3 PASS · new skill PASS · commands 3/3 PASS (baseline blocks verified
  byte-identical) · desk cluster PASS after one blocking provenance fix — the scope-split
  rider had no recorded home; user-ratified onto record D8, strips repointed.

## [0.67.0] — 2026-08-13

Combined build wave — two accepted records landed together (one strip ceremony per file, one
audit round): `plan-structure-yagni` (D1–D7) + `architect-role-pushback-and-abstraction` (D1–D7).

- **Plan re-keyed to the plan-the-plan proposal** (`plan.md`): the lead is a delivery manager
  of the goal; the artifact set demotes from fixed goal-conditions to the approved proposal's
  call (proposed-missing = FAIL, unproposed-absent = correct); architecture
  conditional-but-reserved with the quiet tie-back; baseline seeding untouched. `implement.md`
  entry + design inputs proposal-conditional; both templates re-keyed off the fixed set.
- **New skill `patterns-plan-minimalism`** — the five-rung simplest-execution ladder (single
  source), firing at the proposal, each producing seat's plan, and the proposal contest;
  pointer lines in entity-modeling / api-contracts / technical-requirements /
  patterns-system-design; cross-pointers with `patterns-code-minimalism`.
- **`review-plan-artifacts` re-keyed**: conformance to the approved proposal BLOCKING
  (material divergence auto-FAILs), rung-honesty advisory, completeness within scope.
- **Architect rotation**: `system-architect` retired (strips carry AD-D7 + SD-D1 lineage;
  its report template re-keyed and renamed `architect-report-template.md`);
  `principal-architect` rechartered as the architecture + altitude seat
  (topology craft + `authoring-architecture` + altitude judgment, keystone-clean); new
  `tech-lead` persona (governance authoring, codebase analysis, feasibility review).
- **Remove-shaped review posture library-wide**: `review-feasibility` hunt class 7
  (unjustified structure / wrong altitude — blocking-capable, calibration clause,
  interrogatory round) + tailored excess classes in `review-specifications`,
  `review-brainstorm`, `review-governance-intent`, `validation-constitution` (each with the
  calibration clause); `patterns-system-design` altitude bar hardened (container vs
  C4-level-3 check, override must assert altitude).
- Ceremony: 15 supersession strip entries across 13 strip files; author≠grader audits 4/4
  clusters PASS (agents cluster FAIL→fix→delta-PASS on the orphaned report template); char
  pre-asserts all inside budgets; ledger rotated (tech-lead seeded, principal-architect
  re-measured, system-architect retired); router re-indexed; marketplace synced.

## [0.66.0] — 2026-08-12

- Baseline-seed enforcement — dogfood defect close (ADR
  `.mochiko/decisions/2026-08-11-plan-baseline-seed-enforced.md`): the first plan run never
  seeded `.mochiko/product/` because the greenfield bootstrap obligation lived only in the
  ruling record and `setup.md`, never in `plan.md`. `plan.md` gains the Goal seed sentence,
  a **Baseline-seed** binding (no delivered code: empty scaffolds stating so; delivered
  code: reconstructed and confirmed with the user), and the Not-done "absent at close"
  clause; `implement.md`'s acceptance fold gains the empty-pre-fold clause with the absence
  surfaced as a seeding gap. Pure additions (no strips owed), wording token-tightened at
  landing by user ruling. Audits: two fresh author≠grader validator seats, 2/2 PASS round 1.

## [0.65.0] — 2026-08-11

- Adaptive-depth floor build (ruling: `production-floor-adaptive-depth` D1–D8, accepted
  2026-08-11; PO-D2 amended, PO-D7 superseded). The asserted production floor gains a
  **user-declared low/high depth level** — breadth invariant at both levels, one-way ratchet
  (high terminal), moved only by an explicit declaration recorded in the governance ledger,
  never fact-derived, never watched. Three plan-approved producer clusters, three independent
  author≠grader audits, **all three PASS round 1** (D7 char pre-asserts clean:
  `authoring-constitution` 17,686/21,550 · `validation-constitution` 7,009/8,418 ·
  `review-governance-intent` 7,592/8,862).
  - **Catalog/floor doctrine:** all four FLOOR cards (`catalog/universal-floor.md`) converted
    to two-row `| Level | Asserted level |` low/high tables under the D5 retrofit-cost cut
    line (expensive-to-retrofit rules — secrets-out, input validation, auth boundaries,
    no-silent-corruption, no-PII-in-logs — identical at both levels; merge-blocking gates and
    threshold formalization are the high-row rigor); `catalog/README.md` doctrine rewritten
    (two rows, never a revived tier ladder); `ESSENTIAL-FLOOR.md` light touch (worked examples
    framed as the high row, NON-NEGOTIABLE = category presence); `COMPLIANCE-MODULES.md` gains
    the D7 level-blind clause; `DOMAIN-DEPENDENCIES.md` carrier line level-blind per D8.
    Arch-opinion cards (`backend-service.md`) untouched — outside the dial.
  - **Setup/templates:** `governance-intent-template.md` Depth level declaration section +
    interim transition-delta waiver case + legacy default-high clause;
    `governance-surfaces-template.md` stamp/ledger depth state + MAJOR flip event;
    `INTERROGATION-AGENDA.md` step 0 (declare the level, recommend-then-arbitrate, low for
    greenfield) + the flip-ceremony section (high-mode rerun, adherence delta, no watcher) +
    agenda-test/no-pruning guards (dimension-2 and the pruning license stay dead);
    `authoring-constitution/SKILL.md` authors the declared level's row. "level" terminology
    disambiguated — coverage senses renamed to "coverage threshold".
  - **Graders:** `validation-constitution` accounting verifies declaration + declared-level
    rows (flip = MAJOR); `review-governance-intent` mistake row flipped — the declaration's
    process fidelity is reviewable, level-vs-reality is not (D6 no-watcher held under hostile
    audit read); missing declaration = critical-gap; open transition waivers on a fresh high
    declaration are not a defect.
  - Ceremony: 17 supersession-by-ruling strips across 4 strip files (incl. the protected
    `review-governance-intent` mistake-row lineage and two audit-advisory landing rewords);
    repo ledger set `Depth level: high` (legacy default, #7 fold); ROADMAP/CLAUDE.md identity
    sentences rewritten; SD/OO Tier-I build items carry the two-row obligation.

## [0.64.0] — 2026-08-11

- Guardrails-vs-detail build, Wave 2 (user ruling 2026-08-10: two waves, commands excluded;
  editorial extension of the D4 cut line to the untested primitives, warranted by the Wave 1
  benchmark verdict — no per-primitive benchmark, deeper audit scrutiny instead). **16 skills**
  cut editorially: every `description:` slimmed to ≤500 chars (from up to 1,514 — retiring the
  M1 near-cap risk on `patterns-system-design`, `review-feasibility`, `authoring-architecture`,
  `review-brainstorm`, `review-plan-artifacts`), bodies lose description-restating When-to-Use
  sections and homed walkthrough steps (−3% to −10% where cuttable; `authoring-architecture`,
  `grooming-operating-docs`, and `review-brainstorm` are audited body no-ops — all content
  owned/protected); the `mochiko` router body deliberately excluded (the index IS the
  contract). **4 agents** shipped prose-only (`qa-engineer`, `staff-engineer`,
  `system-architect`, `technical-analyst` — 13 `<example>` blocks removed, descriptions
  78–86% smaller). **Review-evidence floor line** added to `review-brainstorm`,
  `review-feasibility`, `review-plan-artifacts` (skipped in `review-code-minimalism` —
  audited equivalent obligation already present). **M1 fire-rate probe:** 14-scenario blind
  routing spot-check on the slim descriptions — 14/14 hits incl. both sibling traps and the
  non-fire control; its two findings landed as RETURNED clauses (probe-evidenced, user-ruled):
  current-state-baseline cue restored to `patterns-system-design`, existing-code-slimming
  trigger added to `patterns-code-minimalism` (a pre-existing gap, not cut content) — both
  re-audited PASS. **Ledger seeded:** 18 skill-body + 18 skill-description + 4 agent budgets
  added to `.mochiko/memory/primitive-cost-budgets.md` from the audited cut results +25%;
  every skill and agent is now budgeted (only commands and the router body remain
  hard-cap-only). Ceremony: 20 supersession-by-ruling strips (2 with RETURNED entries), full
  prior-KEPT reconciliation (`review-brainstorm`'s v0.60.0 blind-map machinery survives whole);
  independent author≠grader audits by 4 fresh validator seats + 1 bounded re-audit — 20/20
  PASS round 1, zero fix rounds.

## [0.63.0] — 2026-08-11

- Guardrails-vs-detail build, Wave 1 (ruling: `validator-scope-and-verbosity` D1–D8 +
  benchmark verdict, `DECISIONS.md` 2026-08-10;
  `.mochiko/benchmarks/guardrails-vs-detail/report/final-verdict.md`). Shipped the benchmark's
  winning variants across the setup+specify substrate: **11 skills** re-authored as guardrails
  bodies + slim descriptions (`analysis-iterative`, `analysis-codebase`,
  `authoring-constitution`, `authoring-feature-map`, `review-governance-intent`,
  `validation-constitution`, `testing-governance-injection`, `authoring-requirements`,
  `authoring-user-stories`, `authoring-prototype`, `review-specifications`) — body chars −6%
  to −54%, every description cut to ≤500 chars (from up to 1,517); **6 agents** shipped
  prose-only (`<example>` blocks removed from the frontmatter description:
  `principal-architect`, `validator`, `devils-advocate`, `requirements-analyst`,
  `product-manager`, `product-engineer` — descriptions 69–81% smaller, benchmark showed 0
  route misses over 20+ staffings). **D7 cost gate** landed: per-class char budgets
  (winning-variant chars +25%, parsed-value chars never `wc -c` bytes) as a deterministic
  pre-assert in the primitive-edit ceremony (`.claude/rules/mochiko/primitive-edits.md` +
  `.mochiko/memory/primitive-cost-budgets.md`), justified-exemption path, `references/`
  exempt, untested primitives on hard-cap fallback only. **Four floor lines** replace returned
  prose (cross-cutting finding 1): "surface every elicited unknown as an open question"
  (`analysis-iterative`) and "independent review leaves verdict + dispositions in the
  artifacts" (`review-specifications`, `review-governance-intent`, `validation-constitution` —
  the F-X1 mitigation under agents ruling (b)). Ceremony: 17 supersession-by-ruling strips
  (prior `KEPT:` content reconciled — every removal recorded, no silent drops) + independent
  author≠grader audits by 4 fresh validator seats, 17/17 PASS. Known accepted defect:
  `analysis-codebase` dangling "indicators below" pointer (comprehension-only, byte-faithful
  to the ruled variant; follow-up edit in BACKLOG residuals). Commands and the remaining 17
  skills / 4 agents deferred to Wave 2. Residual watches (F-X1 review-evidence,
  slim-description fire-rate, M1 near-cap untested skills, M2 audit-substrate shrink) in
  `BACKLOG.md`.

## [0.62.0] — 2026-08-10

- PM requirements-stacking build (ruling: `pm-requirements-stacking` record D1–D4 + D2a/D3a
  as amended at review). `authoring-feature-map` skill: three-phasing-forms paragraph — an
  extensive feature's phases reuse shipped machinery (within-run = vertical-slice cycles,
  oversize-at-derivation = parent-minting), across-selection-round phasing = leaves under one
  parent with the independently-useful phase-leaf bar (D1, F-6); capability-stub section
  superseded to two-seat minting — specify's derivation may park uncertain remainder as
  `unrefined` stubs with story-trace provenance, selectability and maturation stay
  specify-derivation-only, `/mochiko:feature` stewards-never-matures (D2/D2a; strip
  [v0.62.0]); confidence-keyed cut paragraph (D2); per-parent completeness ledger on the
  selection card + two-site re-surfacing obligation (territory-touching specs +
  stewardship touches, D3/D3a); dependency-triggered escalation split by carrier — leaf via
  technically asserted map relation, shapeless stub via flagged-unverified PM judgment, both
  recommendations for the user's ruling, never a PM-forced cut (D3/D4, F-4/F-7); checklist +
  red-flag reinforcement. Command line-edits (pure additions): `specify.md` selection card
  carries the ledger · `feature.md` stewardship touches re-surface the touched parent's
  parked stubs and undelivered leaves · `plan.md` architecture stage asserts dependency
  relations onto the entry with provenance (technical seat asserts, PM consumes).
  Author≠grader audit: all four surfaces PASS round 1.

## [0.61.0] — 2026-08-10

- Feature-sizing & entry-points build — **breaking change, no migration** (ruling:
  `feature-sizing-and-entry-points` record D1–D15 as amended; D10: the v0.57–v0.59
  spec-folder layout is not read). Plan/implement re-key from spec to feature (D9): entry
  gates on a feature entry carrying ratified scope (a spec's accepted Feature Selection or
  a feature-command delta card); per-feature artifacts re-home to `.mochiko/features/FEAT-XXX/`
  incl. per-feature `requirements.md`; two-altitude design surface — product baselines at
  `.mochiko/product/` (`data-model.md` · `contracts/` · `nfrs.md` ·
  `constraints-and-decisions.md` · `quickstart.md`, `ARCHITECTURE.md` at repo root) + appliable
  before/after per-feature deltas, graded three-way-diff folds at the acceptance landing
  checked by the existing verification seat (D15); cross-spec reach and extend-mode-at-spec-root
  die; spec becomes a pure delivery-event record. New `/mochiko:feature` command (D5–D8 as
  amended): map steward (view/query, `unrefined` stub parking per D12, retroactive promotion,
  retire, integrity grooming) + stable-ground lane triage (D14: feature lane on delivered
  entries, product lane single-flight for cross-cutting defects, in-flight findings file to
  the owning run) + delta-card authoring + dispatch to the re-keyed pipeline — discipline
  floor bound by reference, never restated; lane boundary is the map-write test with
  abort-and-reroute (D7). Map gains two-level nesting (D2–D4): parent capability + leaf
  deliverable, leaf = pipeline unit, hard two-level cap, sticky-delivered roll-up, parents
  minted both directions + retroactive promotion; R5 invariant re-worded (open spec or live
  lane run). Setup bootstraps product baselines (`Assumed`, open thread 4); router + KM
  module updated (lane acceptance is a landing event, D13). Supersessions recorded in strips
  ([v0.61.0]: plan · implement · authoring-feature-map · feature-entry-template ·
  features-index-template · mochiko router), covering feature-map D10/D17/D18/D19/R5 clauses
  + this record's D8 inline-harness architecture (never shipped, no strip owed). Author≠grader
  audits: five commands PASS round 1; map cluster FAIL round 1 (one uncovered reword in the
  skill's strip) → fix → bounded re-audit PASS.

## [0.60.0] — 2026-08-10

- Cold-review gap-challenge build — blind angle map + coverage findings + reopen routing,
  both review clusters (ruling: `cold-review-gap-challenge` record D1–D10, D2/D4 as amended,
  D8 + I4 rider). `review-brainstorm`: new Phase 0 — the reviewer maps expected coverage
  from the topic + free repo grounding (session artifacts excluded) before ever seeing the
  record; map-vs-record diff mints coverage findings, first-class beside the hunt classes,
  admitted by a materiality argument (exempt from the concrete-failure-scenario bar), severity
  by plausibility of ruling change, rejected-roads-checked; hunt class 2 narrowed to
  intra-decision scope (supersession strip `[v0.60.0]` in `.mochiko/strips/review-brainstorm.md`);
  verify pass extends to reopen-born decisions (lighter-review trade-off ruled, one-level
  recursion stop); verdict table admits Critical coverage gaps; description re-fitted at
  1,531/1,536. `brainstorm.md`: two-message reviewer dispatch (topic-only spawn, map back,
  then the record path — blindness lead-enforced), coverage survivors presented per gap with
  the user ruling explore-now / rule-inline / defer (reopens re-enter `analysis-iterative`,
  same D-namespace), reopen-born verify jurisdiction. `review-governance-intent` + `setup.md`:
  same doctrine adapted — the ten-dimension agenda-diff stays primary, the blind map augments
  beyond the agenda, agenda governs on overlap; re-elicited intents land in GI-XXX and ride
  the verify/delta pass. Author≠grader audits: both clusters FAIL round 1 (evidence-bar
  carve-out missing on the setup skill; description 9 over cap; lead-subject wording on the
  reopen ruling in both commands) → fix round → both PASS. First-live-run watch on both
  carriers in BACKLOG (D9).

## [0.59.0] — 2026-08-10

- Feature-map layer wave 4 — brownfield bootstrap + KM wiring (ruling: `feature-map-layer`
  record D11/D12/D16-as-amended/R7/R14/R15; all edits purely additive, no strip notes
  owed). `setup.md`: brownfield runs reconstruct the initial feature map from code (routes,
  UI surfaces, services), confirmed by the user entry by entry, landing as `FEATURES.md` +
  `.mochiko/features/` entries with `delivered` status and the reconstructed-from-code
  mark (first-touch re-verify obligation carried by `authoring-feature-map`); greenfield
  runs scaffold the empty index; the never-overwrite floor covers both writes. KM module
  template: specs-index agreement invariant (open/close contract, rows never contradict
  the map) + FEATURES.md joins the top-level living-doc set marked pipeline-core (never
  scaffolded or declined with the module; map-integrity invariants stay pipeline-side per
  R7) + the R15 boundary line (capabilities on the map, defects/tooling/process in
  BACKLOG). `analysis-codebase`: capability signals seed the reconstruction (one pointer).
  Lead-dispatched author≠grader audit PASS round 1, all three artifacts (the producer's
  self-dispatched audit was not accepted as the ceremony audit). Completes the
  feature-map-layer build: D1–D22 all carried.

## [0.58.0] — 2026-08-10

- Feature-map layer waves 2+3 — the slices→features conversion (ruling: `feature-map-layer`
  record D1–D22). `specify.md` rebuilt: the feature map is an obligated intent-stage read
  (missing map surfaced, never tolerated); after stories, the product-manager seat derives
  features and runs the story filter (rejections recorded, never silent); a user-ruled
  **Feature Selection** replaces the Delivery Slices section (deferred SCs visible at the
  moment of choice); the spec workspace restructures (specs `index.md`, `stories/US-*.md`
  files, map owns status); map writes land only at spec acceptance as one atomic batch;
  migration stance: existing slice-form specs frozen valid, new runs new-form.
  `spec-template.md`: Feature Selection section in, Delivery Slices out, stories section
  becomes an index, header re-keyed to spec vocabulary (`{{spec_title}}`/`{{spec_id}}`).
  `authoring-slices` skill deleted — supersession by ruling, full verbatim preservation,
  per-invariant re-key mapping into `authoring-feature-map`. `plan.md` re-keyed: one run
  per selected feature (FEAT-XXX) in dependency order; the Graduation contract re-keys
  verbatim (shared artifacts extend-in-place at spec root, `[MODIFY]` breaking amendments,
  cross-spec extend reach via owning-spec provenance); in-flight features are readable
  inputs (three-fork resolution, no locks). `implement.md`: the acceptance landing absorbs
  map bookkeeping (status→delivered, delta folds, index touches, in-flight pointer clear,
  derived spec-close) — no separate feature-close stage; regression scope adds accumulated
  delivered-feature gates + later-landing seam ownership; v0.56.0 bounds and snapshot
  isolation preserved byte-identical (audit-verified). `review-specifications`: 10-check
  feature-layer table replaces the Delivery-Slices grade. `authoring-prototype`: FEAT-tag
  re-tag pass at derivation, rejected screens kept greyed. D15 boundary notes on
  `authoring-user-stories`/`authoring-requirements`. Router + `artifact-format.md`
  re-keyed. Strip notes `[v0.58.0]` ×10. Audits: skills PASS round 1 · plan/implement PASS
  round 1 · specify cluster FAIL (unrecorded clause drop, two clobbered prior strip
  headings, template header vocabulary) → fix → delta re-audit PASS.

## [0.57.0] — 2026-08-10

- Feature-map layer wave 1 (pure additions; ruling: `feature-map-layer` record D1–D22,
  DECISIONS.md 2026-08-10). New `product-manager` agent — the product-layer producer: owns
  *which* (feature derivation, the story filter, map writes, selection advice); the
  requirements-analyst owns *how well* under the PM's frame; selection is always the user's
  ruling (D14/D15). New `authoring-feature-map` skill — one living repo-level feature map:
  intent-stage map-read agenda, stories-first derivation with recorded filter rejections,
  FEAT-XXX entry authoring, D8 delta grammar, acceptance-time atomic map writes including
  the specs-index row, map-integrity invariants (incl. in-flight-agreement), foundation as
  an ordering role. New templates `features-index-template.md` (repo-root FEATURES.md — a
  succinct index) and `feature-entry-template.md` (full D6-as-amended entry shape:
  capability, extent, relations, architecture link, story trace, obligations, deltas;
  statuses proposed/in-flight/delivered/retired). Router: skills and agents tables gain
  both rows. plugin.json: agents 9→10. Author≠grader audits: PASS round 1, all four
  artifacts (two advisories logged: specify's wave-2 brief routes spec-index stewardship;
  the skill's split-Process form is record-driven per D5, noted against future form
  audits). Slice machinery untouched this wave — retirement lands with the wave-2 specify
  cluster rebuild.

## [0.56.0] — 2026-08-07

- `implement.md` gains an enforceable bounds contract (pure additions to Harness). New
  **Bounds** bullet: every grading round consumes an attempt whatever its label (default 3
  per cycle, redeclarable at run open); exempting a round is reserved to the user; two
  unchanged-findings rounds is a no-progress stop; test-/records-only changes get a
  delta-grade (no gate re-sweep, prior gate evidence stands); the graded object is the code
  tree, so records-only commits don't move the graded head; round/seat cost surfaced per
  checkpoint. New **Escalation cadence** bullet: reserved-to-user questions batch at the
  cycle checkpoint (build-blockers excepted); Minor advisory findings default to a
  `BACKLOG.md` booking, Important-or-above advisory findings block; gate failures are never
  severity-triaged; `minimalism:` findings stay advisory. Provenance: the mochiko-app
  author-navigate S1 run (R27/R28/R31/R32 "no attempt is consumed" reclassifications) +
  the 2026-08-07 command-text audit. Author≠grader audit FAIL→fix→PASS.

## [0.55.0] — 2026-08-07

- Two native output styles shipped in `output-styles/` (new plugin surface): **Caveman**
  (terse register, baked full level) and **Caveman BLUF** (answer-first BLUF structure +
  caveman diction, BLUF-wins conflict rule). Both `keep-coding-instructions: true`, no
  `force-for-plugin` — user-selectable via `/config` → Output style; main conversation only,
  pipeline reports/artifacts untouched (`templates/output-style.md` still governs those).
- Router skill gains an Output-styles discoverability section (pure addition).
- Design: `.mochiko/brainstorms/plugin-output-styles-delivery/record.md` (D1–D6,
  solo-cold-reviewed 9/9 dispositioned). Author≠grader audit PASS round 1, all three
  artifacts.

## [0.54.0] — 2026-08-06

- `specify.md` gains its missing KM landing Bindings line (governance v1.0.0 validator
  finding — the pin named specify landings but the command carried no reference; pure
  addition, author≠grader audit PASS round 1; pin deviation line struck).
- `marketplace.json` synced 0.10.0 → 0.54.0 — first execution of release gate 5 (GI-016).
- Governance surface set v1.0.0 ratified via first in-repo `/mochiko:setup` run (brownfield):
  CLAUDE.md governance region · governance ledger · KM pin ratified into ruled core ·
  release gates adopted · CHANGELOG elective adopted (this file) ·
  `.claude/settings.local.json` gitignored (GI-015 fix).

## [0.53.0] — 2026-08-05

- Code-minimalism ladder + review lens: `patterns-code-minimalism` + `review-code-minimalism`
  skills minted (26→28); staff/qa personas widened; implement lens wiring. (Pre-CHANGELOG
  entry, reconstructed from the ROADMAP stamp line.)
