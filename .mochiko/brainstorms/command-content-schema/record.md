# Command-content schema — decision record

**Topic:** YAML schema for command content — each section's content tagged with stable IDs,
higher-altitude clusters as metadata, and variable injection inside content blocks (e.g. the
design-seats staffing rule with injectable agent/path values). Start with `implement.md`, roll
out to the other commands after. Shape kin: the eval `rules.json` inventories, in YAML like the
shipped template schemas.

**Status:** accepted (2026-08-26)
**Opened:** 2026-08-26
**Lead:** session lead (brainstorm charter)

---

## Ground facts

- **F1 — substrate state** *(corrected at verify, B3/N1/N2)*. `commands/implement.md` is
  charter-form (six sections) at v0.91.0, 429 lines. A full-file plain-language rewrite (419
  lines / 371 non-blank vs the shipped 408 non-blank — modestly shorter by line, much plainer
  in language) was drafted in this same conversation: sections 1–4 explicitly approved, the
  final pass (Ways of Working + Boundaries) presented without objection — its **formal
  confirmation is owed at the build wave's user gate**. Not landed to `plugins/` — no strips,
  no audit; the drafted text exists as this session's step-0 artifact
  `implement-rewrite.md`. This session's schema idea arrived on top of that draft.
- **F2 — template-schema precedent** *(corrected at review, M2)*. Nine schema data files ship
  at `plugins/mochiko/schemas/*.yaml`: **eight** pipeline artifact-template schemas
  (`template:` + `sections:` + `skeleton:`; `schema-based-template-guidance` D8-schema-data-files — data files
  are the source of truth, `mochiko-cli` renders, raw Read the first-class fallback, GI-020-plugin-install-model)
  **plus one shelf-data schema** (`architecture-shelf-backend.yaml`: `shelf:`/`dimensions:` —
  a third grammar was already on disk before this session).
- **F3 — rules.json precedent** *(corrected at review, M1)*. `evals/*/rules.json` (**6**
  files, from the compression-eval infrastructure): flat rule inventory `{id: R-XXX, rule:
  statement + "Evidence:" clause, class: floor|must|format, source: file:section anchors}` —
  a **derived** inventory pointing back at markdown truth. Built per-eval, not shipped, no
  variables, no clusters.
- **F4 — constraint envelope.** GI-019-kernel-tooling-admission: kernel-class tooling only by recorded ruling; the
  standing bright line — never gates pipeline progress, never dispatches or sequences agents,
  never holds judgment skills own. `schema-based-template-guidance` D11-kernel-position-softened admitted the template-schema CLI **for artifact
  templates specifically**; extending the renderer to command content is a new admission
  needing its own recorded ruling. GI-020-plugin-install-model: plugin install stays markdown-only — commands must
  ship as functioning `.md` whatever the schema does.
- **F5 — audit keying today.** The charter audit (`primitive-edits.md`) grades implement.md
  against prose criteria (floor present + goal contract present, every FAIL clause surviving);
  strips quote content verbatim. Neither is ID-addressable today.

## Problem — why this session (folded at review, C1)

Four failures in the all-prose command form:

1. **No deterministic governance surface.** Prose is auditable only by model judgment. A
   schema's structure is machine-checkable — every ID unique, every label registry-valid,
   every `${var}` bound, every `ruling:` anchor resolving to a live `DECISIONS.md` row —
   deterministic checks the ceremony cannot run today at any cost. (GI-019-kernel-tooling-admission carves advisory
   post-hoc checkers out of kernel-class; this is governance-legal by standing ruling.)
2. **Unaddressable content.** Strips, audits, and traceability key on verbatim-quote hunting;
   the audit has already nearly lost a protected line once (v0.34.0). Nothing can be cited by
   name.
3. **No query surface.** Labels give cross-command views; deviation checking becomes a diff of
   same-labeled rules instead of reading everything.
4. **One change regime forced onto two kinds of content.** Rules are atomic and near-immutable
   under ceremony; narrative is voice that should stay freely editable. The schema separates
   the regimes — this is D2-rules-prose-split's split stated as its real reason, not an aesthetic line.

**Null road (steelmanned, rejected).** Keep `.md` as-is: the ceremony demonstrably works
today, at zero delivery risk and no new format — its cost is exactly the four failures above,
paid at every edit forever. Rejected by the user's ruling: addressability, checkability, and
linkage are wanted as the foundation for rollout.

**Rejected road — derive-the-inventory (folded at review, I1).** Generate the ID'd, labeled
rule inventory *from* `implement.md`, keeping the `.md` as truth — the literal shape of the
`rules.json` kin, and the lower-delivery-risk path. Considered and rejected on the user's
ruling, three reasons: (1) the Q1 clarification wanted the schema **as source** with runtime
interpretation — a derived inventory is the opposite posture; (2) deterministic checks are
worth most against the source of truth — checking a derived shadow certifies the projection,
not the thing; (3) derivation keeps rules and prose in one file under one change regime — the
exact mixing driver 4 names. Cost stated honestly: the rejected road carried less delivery
risk; that risk is accepted eyes-open at the C2 disposition (D7-stage-one-scope hardening).

## Decisions

### D1-yaml-source-truth — YAML is the source of truth; the model interprets it at runtime — `Assumed`

*(Re-marked `Assumed` at review, I6: the model-interpreted delivery instrument has never
executed — n=0, D10-per-command-rollout — and the house idiom for a never-executed instrument is `Assumed`.)*

**Statement:** the schema data file (`implement.yaml` shape) is the source of truth for the
content it carries. The command `.md` loads it and the **model interprets it live** at command
fire — "render" means runtime interpretation by the agent, not a build-time generation step.
No binary is required on the read path (raw Read stays first-class, GI-020-plugin-install-model satisfied);
`mochiko-cli` rendering can be added later as an optional human-facing view.

**Rationale:** the user's stated intent at Q1 clarification — a thin scaffold around the
schema with the ability to effectively interpret. Matches the `schema-based-template-guidance` D8-schema-data-files template precedent's
data-as-source-of-truth posture while dropping its build-time render step.

### D2-rules-prose-split — Stage 1 split: rules move to the schema, narrative stays prose; absorption trigger on record — `Confident`

**Statement:** stage 1 moves the **rule-like content** of `implement.md` into the schema
(ID-tagged blocks); the charter narrative (Identity & Mission, protocol prose voice) stays in
the `.md`. An **absorption trigger** is on record *(benefit-keyed at review, C1)*:
stage 2 — narrative absorbs into the schema, `.md` thins to scaffold — is pre-authorized when
the first live `/mochiko:implement` run under the schema shows **(a) delivery** — the
schema-carried rules read fully, before first action, no miss attributable to YAML carriage —
**and (b) at least one concrete benefit** — an `impl.*` ID cited by a strip, audit finding, or
`DECISIONS.md` row · a `vars:` change replacing what would have been a multi-site edit · the
advisory checker (D13-advisory-schema-checker) catching a real defect. Stage 2 then lands as an ordinary build citing
this record. *(Retreat branch, I4:)* contrary evidence does not merely hold the split — where
the evidence shows the **split itself** harming (drift between the two homes, floors missed on
the schema side), retreat to all-`.md` is a named option **reserved to the user**; the build's
strips (item 4) keep both directions reconstructible per GI-006-primitive-edit-traceability.

**Rationale:** the lead recommended whole-move (A) on drift/boundary/injection-reach/audit
grounds; the user weighed the steering risk — a YAML-carried mission is unproven (n=0) — and
ruled the staged middle path the lead offered: B's graceful degradation without B becoming
permanent. House idiom: `not-now + trigger` stance rows, sound-loop rules-file first-miss
deferral.

### D3-no-shared-library — No shared rule library: per-command rules + a common label vocabulary — `Contested`

**Statement:** rule definitions live inline in each command's schema — no shared
`charter-rules.yaml`, no library-of-rules with binding files. The cross-command connective
tissue is a **common label vocabulary**: one controlled set of labels (the "altitude clusters")
applied to rule blocks across all command schemas; the same label on rules in different
commands constitutes the link. Duplication between commands is accepted and made *visible and
addressable* rather than extracted away. The skill fence holds regardless: where a skill owns
a floor (`patterns-sound-loop`, `patterns-transport-floor`, …), the rule block carries the
pointer, never the procedure.

**Rationale:** the lead recommended a shared library + per-command bindings (single-sources
the real charter-boilerplate duplication); the user ruled against extraction — the want is a
common vocab that creates command links, "sort of like label", keeping every command readable
from its own file.

### D4-label-job-scope — Label job: navigation now, edit-time drift check as the goal — `Confident`

**Statement:** stage 1 ships labels as **query/navigation only** — cross-command views by
label, no ceremony obligation. The goal state is the **edit-time drift check** (editing a
labeled rule surfaces same-labeled rules in other command schemas; the editor states aligned
or diverged-on-purpose, folded into the primitive-edit ceremony). Graduation is
benefit-keyed *(amended at review, C1/I3)*: the ceremony hook lands citing this record when
the vocabulary has survived — without label churn — **either** the implement build plus the
first rollout command, **or** three implement-touching edits (the within-implement path keeps
the criterion reachable while D10-per-command-rollout schedules no second command), **and** a label query was
actually used at least once in real work.

**Rationale:** user's ruling — "B for now, but then goal is to be A … an effective B gives
confidence for A." Same staged-trigger idiom as D2-rules-prose-split.

### D5-schema-local-variables — Variables: schema-local `vars:` block + `${var}` substitution at read — `Confident`

**Statement:** each command schema carries a `vars:` block (seat names, paths, bounds); rule
text carries `${var}` placeholders; the model substitutes at read time. One place to change a
value; text stays readable as text. Runtime repo-state resolution declined for now; a value
class can graduate later by its own ruling. *(Sigil amended at review, I2 — verified:
`{{...}}` already ships across four template schemas and four `templates/*.md` meaning "blank
the agent fills with authored content"; same directory, opposite semantics. Var substitution
uses `${var}` to keep the two conventions unconfusable.)*

**Rationale:** the user's original ask, literally; ruled as recommended.

### D6-rule-block-grammar — Rule-block grammar adopted — `Confident`

**Statement:** a rule block is `{id, labels: [...], class: floor|must|advisory, text,
ruling?: <DECISIONS.md anchor>, pointer?: <skill>}` under a top-level `vars:` block. ID format
*(amended post-review at the user's ask)*: **dotted slug** — `impl.<kebab-name>`, e.g.
`impl.design-seats-staffing`, `impl.attempt-exemption-user-only`, with `fail-condition` rules
under `impl.fail.<name>`; the slug is a **name, never a summary**, frozen at mint per D11-rule-id-lifecycle, and
the D13-advisory-schema-checker checker enforces uniqueness and format. `ruling:` machine-tags protected content — the
audit's preserved-responsibilities check and the strip ceremony gain an addressable anchor.
`pointer:` carries the skill fence: where a skill owns the floor, the rule holds the pointer,
never the procedure. `class` reuses the eval taxonomy shape, and has a named consumer
*(M3 fold)*: the charter audit grades `floor`-class rules as must-survive; `advisory`-class
rules may change without supersession ceremony.

**Rationale:** adopted as drafted from the preview; ceremony wiring from day one preferred
over minimal-then-grow.

### D7-stage-one-scope — Stage-1 scope: all rule-shaped content moves, FAIL list included; source = the simplified rewrite — `Confident`

**Statement:** `implement.yaml` rules = R&R seat wiring + reserved-to-user items + Ways of
Working + Boundaries + Tools bindings + the FAIL clauses (labeled `fail-condition`; the `.md`
protocol's Not-done line becomes "every rule labeled `fail-condition`"). Narrative staying in
`implement.md` = Identity & Mission + Adaptive Goal Protocol prose. Source text is the
**simplified rewrite drafted earlier this session** (never the shipped v0.91.0 wording;
approval status per F1 — the Ways of Working + Boundaries confirmation is an explicit item at
the build wave's user gate, B3); rewrite + schema land as **one build wave**. The charter audit criterion "every prior FAIL
clause surviving" re-keys to the `fail-condition` label set.

**Rationale:** FAIL clauses are the most rule-shaped content in the file; the audit re-key is
owed by the build wave anyway. Ruled as recommended.

*(Hardened at review, C2 — user-ruled, the risk accepted eyes-open: the `.md` is guaranteed
in context at command fire while the schema is an instructed-not-forced Read, and what moves
out is precisely the FAIL list and the non-waivable floors. Guards: the `.md`'s Not-done line
hard-codes the `fail-condition` rule **count** — a stale count trips the D13-advisory-schema-checker checker, and a
lead that never read the schema cannot fake the clauses · the D10-per-command-rollout first-live-run watch gains
delivery probes — schema read? read fully? before first action? · the user's stated
compensator: "having deterministic script check gives better ability." The reviewer's
alternative — hold `fail-condition` rules inline for stage 1 — was offered and declined.)*

### D8-label-vocabulary-registry — Label vocabulary: controlled registry file + ten-label seed — `Confident`

**Statement:** the vocabulary ships as `plugins/mochiko/schemas/command-labels.yaml` — one
line of meaning per label; every rule's `labels:` values must come from it; new labels enter
by registry edit first (normal shipped-primitive ceremony). Seed set (10): `independence` ·
`user-gate` · `fail-condition` · `attempt-economy` · `landing` · `evidence` · `scope-entry` ·
`seats` · `floor-pointer` · `reporting`.

**Rationale:** rules stay per-command (D3-no-shared-library), but the vocab is the one deliberately shared
surface — a registry under ceremony is what keeps it common instead of drifting per command.
Ruled as recommended, full seed.

### D9-schema-governance-envelope — Governance envelope: no new kernel admission; schemas are shipped primitives; audit re-keys to the pair — `Confident`

**Statement:** model-interpreted command schemas need **no new kernel-class admission** — the
schema is data, the interpreter is the model; nothing executable gates pipeline progress or
dispatches agents, so GI-019-kernel-tooling-admission is untouched. A future `mochiko-cli` render/`--check` over
command schemas would extend the admitted CLI and takes its own ruling note at that time.
`implement.yaml` and `command-labels.yaml` are shipped primitives under the full strip +
author≠grader ceremony (the v0.76.0 schema precedent). The charter audit re-keys to grade the
**`.md` + schema pair** — floor present + goal contract present across both surfaces, the
FAIL-clause-survival criterion keyed to the `fail-condition` label set — via a
`primitive-edits.md` edit riding the build wave. GI-020-plugin-install-model holds: install ships markdown + data
files, nothing heavier.

**Rationale:** proposed whole by the lead; adopted as stated.

### D10-per-command-rollout — Rollout by per-command ruling; first-live-run watch is the shared trigger evidence — `Confident`

**Statement:** implement converts first (this build). Each further command (`feature.md`,
`architecture.md`, the v8 trio) converts **by its own ruling** — door-open idiom, as the
charter ADR did. Evidence honesty: n=0 for model-interpreted schema delivery; a
**first-live-run watch** on schema-carried rule delivery is owed in `BACKLOG.md`; its outcome
is the trigger evidence for D2-rules-prose-split (narrative absorption) and for D4-label-job-scope's first path, while D4-label-job-scope's
within-implement path (three implement-touching edits) accumulates as edit observations in
the **same watch item** *(N3 repair)*.

**Rationale:** proposed whole by the lead; adopted as stated.

### D11-rule-id-lifecycle — ID lifecycle: mint-once, frozen, tombstoned — `Confident` *(review fold, C3)*

**Statement:** an `impl.*` ID is minted once and never reused. A reword preserves the ID; a
split mints children and records the parent; a merge retires the losers with a tombstone entry
(ID + disposition) so no anchor ever dangles. Slugs are names, not summaries — wording drift
never renames. Enforcement: the D13-advisory-schema-checker checker verifies format, uniqueness, and tombstone
integrity deterministically; the charter audit verifies continuity (no ID vanished without a
tombstone) as part of preserved-responsibilities. Compression waves obey the same rule — a
pass that rewrites text must carry IDs through unchanged. *(N4 repair:)* a D2-rules-prose-split retreat retires
the whole `impl.*` namespace via **one namespace-level tombstone** carried by the recorded
retreat ruling — never per-ID entries.

**Rationale:** three decisions (D6-rule-block-grammar, D7-stage-one-scope, D9-schema-governance-envelope) bind to ID/label persistence; without a stability
rule, protected-content tracking floats and GI-006-primitive-edit-traceability reconstructibility breaks.

### D12-rule-block-grain — Rule grain: one block per independently-citable obligation — `Confident` *(review fold, I5)*

**Statement:** a rule block carries exactly one independently-citable obligation — the unit a
strip, audit finding, or `DECISIONS.md` row would cite alone. Worked example *(corrected at
verify, B2)*: the Boundaries attempt-economy bullet yields **five** blocks —
`impl.attempt-per-grade` (an attempt is consumed per verification grade, default 3, run-open
redeclaration) · `impl.attempt-exemption-user-only` · `impl.no-progress-stop` (two unchanged
rounds halt) · `impl.epic-member-halt` (member-scoped halt, disposition user-reserved) ·
`impl.gap-rework-bound` (the run-scale analogue: default 2 rounds, its own run-open
redeclaration point, a localizing finding charges that cycle's remaining attempts,
exhaustion disposition user-reserved).

**Rationale:** grain decides whether labels are navigable and `ruling:` anchors precise; left
to the build it would be invented ungoverned.

### D13-advisory-schema-checker — Stage 1 ships an advisory deterministic checker — `Confident` *(review fold, C1; user-ruled "yes ship the checker")*

**Statement:** the build ships a minimal advisory checker — a script, exit-code-only signal,
never a required CI gate, never gating pipeline progress (inside GI-019-kernel-tooling-admission's advisory-checker
carve-out). Checks, all deterministic: ID uniqueness + slug format (D6-rule-block-grammar/D11-rule-id-lifecycle) · every label ∈
`command-labels.yaml` (D8-label-vocabulary-registry) · every `${var}` bound in `vars:`, no orphan placeholders (D5-schema-local-variables) ·
every `ruling:` anchor resolving to a live `DECISIONS.md` row (D6-rule-block-grammar) · tombstone integrity
(D11-rule-id-lifecycle) · the `.md` Not-done line's `fail-condition` count matching the schema (C2 guard) · a
`kind:` discriminator present *(closing the open question — decided yes at M2: three grammars
now coexist under `schemas/`, top-level shape no longer discriminates)*. Its output is cited
in the audit brief as a deterministic pre-pass — the char-budget pre-assert idiom. Crate
extension stays reserved per D9-schema-governance-envelope; the script is standalone.

**Rationale:** the user's core driver — "the ability to enforce deterministic checks on schema
… opens up a lot better governance"; checkability without a checker would be a recorded hope.

*(Mark split at verify, N5: the decision to ship is `Confident`; the efficacy claim — the
checker catching real defects — is `Assumed`, n=0, and a caught defect is itself one of D2-rules-prose-split's
benefit observations.)*

### D14-nested-section-grammar — Nested section grammar: sections are first-class nodes — `Confident` *(post-build amendment, 2026-08-26, user-ruled)*

**Statement:** amends D6-rule-block-grammar. The schema's top level is a `sections:` list, each section
`{id, title, intent, rules}`; rule blocks nest under their section, their own grammar
unchanged. Section IDs use the `<cmd>.sec.<slug>` segment (`impl.sec.roles` ·
`impl.sec.reserved` · `impl.sec.tools` · `impl.sec.ways-of-working` · `impl.sec.boundaries` ·
`impl.sec.fail-conditions`), minted once and tombstoned under the same D11-rule-id-lifecycle lifecycle as rule
IDs. Section metadata stays thin — `title` verbatim from the charter group, `intent` one
navigation line; sections never grow a second prose surface (narrative stays in the `.md`).
The `.md` points at section IDs; the checker asserts the section grammar and prints
per-section stats; a top-level flat `rules:` key is a checker finding. The v0.92.0 flat form's
comment dividers are superseded — they were invisible to the checker and unpointable from the
`.md`.

**Rationale:** the user, reviewing the shipped v0.92.0 file: the flat list "is not utilizing
nesting" — the `#` divider groups should be real sections with metadata the `.md` can point
to. Membership becomes data (checkable, per-section stats) instead of comment convention.
All 104 rule IDs and texts carried unchanged — pure relocation, D11-rule-id-lifecycle continuity trivial.

### D15-rule-text-closure — Referential closure: rule texts are self-contained — `Confident` *(post-build amendment, 2026-08-26, user-directed)*

**Statement:** amends D6-rule-block-grammar. A rule's `text` must be **referentially closed**: every reference in
it resolves within the block itself or the schema's addressable namespace — `${var}` names,
`impl.*` rule IDs, `impl.sec.*` section IDs, `class:` values, registry labels, `pointer:`
skills, `ruling:` anchors, and literal file paths. **Deixis is a defect:** a pointing word
whose referent lives outside the block ("these rules", "this section", "above", "below",
document-shape remarks like "There is no X section") breaks the D12-rule-block-grain promise that a block is
independently citable — quoted alone, the reference dangles. Corpus-level deixis is worse than
a named dependency, not better: it couples the block to everything and addresses nothing. The
law is general to every `kind: command` schema; the checker carries a curated deixis lint
(warning-class — heuristic detection never blocks the advisory pre-pass; the list grows only
on observed recurrence). Legal self-reference: "this schema" (the file being read) and "the
run" (every rule's subject) — resolvable at read time, excluded from the lint.

**Rationale:** the user, reading `impl.staffing-latitude` — "what does 'these' refer to? is it
creating dependency on other rule" — then directed the general form over the spot fix. Root
cause: extraction from `.md` prose preserved wording whose referents were the surrounding
document; atomization moved the referents outside the block. One instance in 104 rules at
audit (the lint's first catch); the law prevents the class at D10-per-command-rollout rollout.

### D16-provenance-leaves-schemas — Runtime-only schemas: provenance moves to a sidecar — `Confident` *(post-rollout amendment, 2026-08-26, user-ruled)*

**Statement:** amends D6-rule-block-grammar. A command schema carries **only what the run consumes**: `text`,
`class` (floor waivability), `labels` (the `fail-condition` set is operative; the rest are
query-usable in-run), `pointer:` (binds procedure), `vars:`, and the section grammar. The
`ruling:` field leaves the schemas: every id → anchor pair relocates verbatim to one
maintainer-side sidecar, **`.mochiko/provenance.yaml`** (`kind: command-provenance`), keyed
by the mint-once rule ID — the addressability the ID namespace exists to provide. The
sidecar lives repo-side, NOT under `plugins/` — the user's refinement mid-execution:
maintainer metadata is never delivered with the plugin (the GI-020-plugin-install-model posture applied to
data). A plugin-standalone checkout simply lacks it; the checker degrades to a warning. Protection semantics unchanged: an anchored rule still leaves only by
recorded supersession-by-ruling; the checker still validates every anchor (format + live
`DECISIONS.md` resolution) and now also flags any inline `ruling:` as superseded grammar and
any sidecar key naming a nonexistent rule ID as dangling. Edit-time visibility is carried by
the path-scoped ceremony reminder on schema Read plus the mandatory pre-bump audit (GI-004-primitive-audit-ratchet),
not by inline placement.

**Rationale:** the user, reading the shipped anchors: a session "need[s not] know the trace
of what decision made the change, it is information that is decorative, rather than helping
… runtime is the only thing that matters; for traces of change, we have strips." The run
never acts on provenance — 107 anchor lines across the library were pure read-time overhead.
Placement was wrong, not existence.

## Session trail

- **Q1 — source of truth** (structured fork A/B/C): user rejected the framing and clarified —
  thin `.md` scaffold around `implement.yaml`, model interprets at runtime. → D1-yaml-source-truth.
- **Q2 — how thin** (whole charter moves vs rules-only vs overlay; lead recommended
  whole-move): user picked rules-only, had second thoughts, asked for the A-vs-B case; lead
  gave drift/boundary/reach/audit for A, steering-risk + ratchet steelman for B, offered the
  staged middle path; user asked what an absorption trigger is; ruled **B with absorption
  trigger**. → D2-rules-prose-split.
- **Q3 — reuse shape** (shared library recommended / per-command inline / library-of-bindings):
  user ruled against extraction — per-command rules + a common label vocab as the command
  link. → D3-no-shared-library (`Contested`).
- **Q4 — label job** (drift-check ceremony recommended / navigation-only / staged): user ruled
  "B for now, goal is A; effective B gives confidence for A". → D4-label-job-scope.
- **Q5 — variables** (vars block + placeholders recommended / typed fields / runtime
  resolution): ruled as recommended. → D5-schema-local-variables.
- **Q6 — rule-block grammar** (adopt drafted grammar / trim / adjust; preview shown): adopted
  as drafted. → D6-rule-block-grammar.
- **Q7 — stage-1 scope** (all rule-shaped content incl. FAIL list recommended / FAIL stays /
  adjust; simplified-rewrite premise stated): ruled as recommended. → D7-stage-one-scope.
- **Q8 — vocab home** (registry + ten-label seed recommended / trimmed seed / no registry):
  adopted with full seed. → D8-label-vocabulary-registry.
- **Q9/Q10 — wrap batch** (governance envelope + rollout/probes proposed whole): adopted as
  stated. → D9-schema-governance-envelope, D10-per-command-rollout.
- **Post-review trail:** sizing gate — solo recommended, "as recommended" · C1 presented with
  a drafted problem statement; the user supplied the deeper driver set (deterministic checks ·
  label queries · deviation checking · atomic-rules-vs-prose separation) — redraft adopted,
  and the follow-up "ship the checker now?" ruled **yes** → Problem section + D13-advisory-schema-checker · C2
  presented as a two-door fork; user ruled door A (D7-stage-one-scope stands, hardened) with the risk
  explicitly accepted: "i understand the risk not having all in .md poses but having
  determinstic script check gives better ability" · I1 ruled recorded-as-rejected with the
  three principled reasons · post-review user ask — "i want the id to be easier to understand"
  — four-option fork (dotted slug / numeric+name / section-keyed / label-keyed), ruled
  **dotted slug** → D6-rule-block-grammar amendment.
- **Post-build amendment (2026-08-26, v0.93.0):** user, reading the shipped flat schema —
  "i dont think it is utilizing nesting … create sections and some metadata of section, and
  in the implement command md point to those section rules." Three-option fork (nested
  sections with IDs recommended / flat + `section:` key / nesting without IDs), ruled
  **adopt as drafted** → D14-nested-section-grammar.
- **Post-build amendment 2 (2026-08-26, v0.94.0):** user, probing `impl.staffing-latitude`'s
  "these rules" — "what does 'these' refer to? is it creating dependency on other rule" — and
  directing the general form: "think at a higher level, build a way that be implemented
  generally." → D15-rule-text-closure (referential closure + checker deixis lint); the one live instance
  reworded, ID kept.
- **D10-per-command-rollout rollout ruling (2026-08-26, user-directed):** conversion tooling first — the user
  directed a reusable repo-level skill ("launch another subagent to build a skill that i can
  reuse to convert other commands … at the repo level, not to be distributed") →
  `.claude/skills/converting-command-to-schema/SKILL.md`, producer-authored on the lead's
  approved outline, author≠grader validated (round 1 FAIL 3 Major/3 Minor → fix round →
  round 2 one residual Major + Minor + nit, lead-repaired → round 3 confirm). Then the
  rollout itself: "have multiple separate agent teammates convert the remaining commands into
  the format. have validation too for each" — the D10-per-command-rollout per-command ruling exercised in one
  wave over the five remaining commands: `architecture.md` (prefix `arch`) · `brainstorm.md`
  (prefix `brainstorm`) · `feature.md` (prefix `feat`) · `setup.md` (prefix `setup`) ·
  `specify.md` (prefix `spec`). Scope: structure-only extraction — the shipped v0.94.0 text
  is each command's step-0 referent (frozen under this session's `referents/`), meaning
  survives, no simplification pass. One producer + one validator per command, disjoint file
  ownership; registry, checker, `primitive-edits.md`, and operating docs stay lead-owned.
  Mid-wave user delegation: "i am happy for labels to expand based on your judgement" — D8-label-vocabulary-registry's
  amendment-by-ruling is delegated to the lead for this wave; additions admitted only as
  genuinely cross-command concepts, recorded here at collection. **Collection ruling:** two
  labels admitted — `binding` (paths/schemas/workspaces/surfaces; gap flagged independently by
  the architecture and specify producers, and every non-charter command carries a Bindings
  group) and `stewardship` (living-artifact care at the standing desks; feature map +
  architecture store). Declined: a sequencing label (one rule, one command — below the
  cross-command bar; nearest fit stands). Registry ten → twelve. `implement.yaml` keeps its
  labels as-is; a `binding` sweep of `impl.sec.tools` is a next-touch candidate, not owed now.
  Two follow-on label rulings at collection: `brainstorm.synthesis-on-request` carries
  `binding` alone (the obligation is the artifact's surface, not the reporting register —
  conscious ruling on the brainstorm validator's observation); the three feature label swaps
  the confirm round flagged as narrowed links were restored to dual labels lead-side
  (`feat.dm-epic-stewardship` +`landing` · `feat.delta-cards` +`evidence` ·
  `feat.no-silent-map-mutations` +`evidence`), checker re-run PASS.
- **D10-per-command-rollout rollout outcome (2026-08-26, v0.95.0):** all five conversions DELIVERED — five
  producers + five fresh author≠grader validators, each pair graded on the three-audit set
  (pair coherence · schema fidelity vs the frozen step-0 referent · strip verification) with
  the checker as deterministic pre-pass. Verdicts: brainstorm PASS round 1 (2 Minor) ·
  feature PASS round 1 (zero findings) · architecture PASS round 1 (3 Minor) · specify FAIL
  round 1 (1 Major: an undisclosed added phrase — struck by ruling) · setup FAIL round 1
  (1 Critical: the carve-outs reword had erased the real `governance-surfaces.yaml` referent
  and narrowed a floor rule's scope to the CLAUDE.md region — restored with both carve-out
  homes named, `pointer: mochiko:authoring-constitution` adopted on evidence, the strip's
  false rationale rewritten; 1 Major: registry reconciliation) — all five **CONFIRMED-PASS**
  on fix/confirm rounds. Checker grew two guards mid-wave (lead-side): the D14-nested-section-grammar section-count
  guard ("nested in N sections" phrase vs schema, negative-tested) beside the D15-rule-text-closure deixis
  lint. Library state at v0.95.0: six `.md`+schema pairs, 320 rules
  (impl 104 · feat 49 · spec 51 · arch 47 · setup 40 · brainstorm 29), twelve labels, six
  checker-PASS runs 0 findings. The conversion skill
  (`.claude/skills/converting-command-to-schema/SKILL.md`) validated to CONFIRMED-PASS in
  its own three rounds before the wave used it.
- **Post-rollout amendment (2026-08-26, v0.96.0):** user, probing the shipped `ruling:`
  field — "why a session need to know the trace of what decision made the change, it is
  information that is decorative" — then ruled: "runtime is the only thing that matters, for
  traces of change we have strips. make the change." → D16-provenance-leaves-schemas (provenance sidecar); all
  anchors extracted from the six schemas, lossless (107 anchors: impl 34 · spec 25 ·
  feat 15 · arch 13 · setup 11 · brainstorm 9), checker-reworked with negative-tested
  inline-ruling and dangling-key findings. Mid-execution refinement, user-ruled: "can we
  not put it somewhere that is not delivered by plugin?" — sidecar home moved from
  `plugins/mochiko/schemas/` to **`.mochiko/provenance.yaml`**, beside the strips. Audit
  PASS (0 Critical / 0 Major / 4 Minor — count corrected to 107; the sidecar added to the
  primitive-edits path scope; the checker warns on skipped foreign-prefix entries). Two
  bounds accepted eyes-open on the audit's naming: schema→sidecar coverage is
  one-directional (an anchor removed from the sidecar is caught by the sidecar's own
  ceremony — it now sits in the primitive-edits path scope — not by a checker finding), and
  the schemas' three-line header pointer at the repo-side path stays (edit-time visibility
  worth three comment lines; the header states "not shipped").

## Review + disposition trail

**Sizing:** solo cold review, user-ruled "as recommended" at the named gate.
**Dispatch:** blind two-message per the charter — message 1 topic + goal only (fence held:
nothing under this session's directory read); 30-angle Phase 0 map returned before the record
path was sent.
**Verdict:** `critical-gaps` — 4 Critical · 6 Important · 5 Minor survived the reviewer's own
cross-examination; 14 further findings raised and killed by the reviewer (kill list in the
review output, incl. GI-019-kernel-tooling-admission correctly passed, frontmatter fidelity, dual-maintenance vs D8-label-vocabulary-registry,
harness-load-path).
**Dispositions:** C1 → Problem section + benefit-keyed D2/D4 triggers + D13-advisory-schema-checker checker (ruled
individually) · C2 → D7-stage-one-scope hardened, door A, risk accepted eyes-open (ruled individually) · I1 →
rejected-road entry with reasons (ruled individually) · C3 → D11-rule-id-lifecycle · C4 → build step 0 · I2 →
D5-schema-local-variables `${var}` · I3 → D4-label-job-scope within-implement path + seed validation · I4 → D2-rules-prose-split retreat branch · I5 →
D12-rule-block-grain · I6 → D1-yaml-source-truth re-marked `Assumed` · M1/M2 → F3/F2 corrected, `kind:` decided (D13-advisory-schema-checker) · M3 →
`class:` consumer named (D6-rule-block-grammar) · M4 → strip-verbatim rule (build item 4) · M5 → cost paragraph
(build surface) — the batch user-ruled "as recommended".

**Verify trail:** round 1 NOT CLEAN — 3 blocking (B1 stale `{{placeholder}}` in D5-schema-local-variables's heading ·
B2 D12-rule-block-grain's worked example under-counting its own referent, four blocks where the grain yields
five · B3 the record over-claiming approval on the Ways of Working + Boundaries text — the
floor text — against the referent's own honest provenance header) + 7 non-blocking (N1/N2 F1
now-false "no file write" and wrong line count · N3 D10-per-command-rollout not amended alongside D4-label-job-scope · N4 retreat
vs tombstone gap · N5 D13-advisory-schema-checker mark split · N6 pin N=15 · N7 landing residue, deferred to
acceptance by design). All lead-repaired same round except N7. Round 2: bounded re-verify of
the repairs — **CLEAN**: 9/9 landed, no new contradiction; B2's five-block example verified
complete against the referent (exactly five obligations, no sixth; `impl.gap-rework-bound`
correctly outside the `impl.fail.*` prefix); F1's four counts independently re-measured
exact; B3's wording agreeing across F1, D7-stage-one-scope, step 0, and the artifact header; N=15
independently re-counted. Acceptance followed the CLEAN verify, with the Ways of Working +
Boundaries build-gate confirmation restated to the user at the acceptance gate.

## Build surface (cold-buildable)

One wave, landing the earlier-approved simplified rewrite and the schema together:

0. **Durable referent first (C4):** the simplified rewrite lands at
   `.mochiko/brainstorms/command-content-schema/research/implement-rewrite.md` as a session artifact —
   the build's source text and the fidelity audit's referent (done in-session). Without this
   step the wave is not cold-buildable. **The build wave's user gate explicitly confirms the
   Ways of Working + Boundaries text** — the two sections whose approval rides that gate per
   F1 (B3).
1. `plugins/mochiko/schemas/command-labels.yaml` — the D8-label-vocabulary-registry registry, ten-label seed. The build
   **validates the seed against the real rule inventory** (I3): every rule labelable, no
   single-member or catch-all labels; a mismatch takes the revise-the-seed branch (registry
   edit in the same wave, noted in the audit brief).
2. `plugins/mochiko/schemas/implement.yaml` — `kind: command` discriminator (D13-advisory-schema-checker) + `vars:`
   block + rules per D6-rule-block-grammar grammar at D12-rule-block-grain grain; IDs dotted-slug per the D6-rule-block-grammar amendment; `${var}`
   placeholders (D5-schema-local-variables); content = the rule-shaped inventory of D7-stage-one-scope, text from step 0's artifact;
   FAIL clauses as `impl.fail.*`; skill-owned floors as `pointer:` rules; protected lines
   carrying `ruling:` anchors.
3. `commands/implement.md` — simplified narrative (Identity & Mission + Adaptive Goal
   Protocol prose), a load-and-follow instruction naming the schema, the Not-done line
   re-keyed to "the 15 rules labeled `fail-condition` in `implement.yaml`" — **N=15 pinned
   from the referent** (`implement-rewrite.md` carries 15 FAIL clauses; C2 guard, N6).
4. Strips: the rewrite's deletion/relocation ledger (rationale cuts, R&R-restatement cuts,
   deviation-grammar single-homing) **plus** supersession entries for every block moving from
   `.md` to schema. **Verbatim rule (M4):** every entry's content field carries the shipped
   v0.91.0 text (what actually left the file — the GI-006-primitive-edit-traceability referent); the rewrite delta is
   recorded separately, never co-mingled.
5. The advisory checker (D13-advisory-schema-checker) — standalone script, exit-code signal; its checks per D13-advisory-schema-checker; its
   output cited in the audit brief as the deterministic pre-pass. Home and language are the
   build's judgment (crate extension reserved, D9-schema-governance-envelope).
6. `.claude/rules/mochiko/primitive-edits.md` — the D9-schema-governance-envelope audit re-key (pair grading;
   label-keyed FAIL criterion; D11-rule-id-lifecycle ID-continuity check; `class: floor` = must-survive, M3).
7. Audits: author≠grader validators — command-pair coherence + schema fidelity against step
   0's artifact + strip verification, each brief citing the checker's pre-pass output.
8. Gates 4/5/6: `CHANGELOG.md` · `marketplace.json` sync · `cargo test` (binary untouched;
   command schemas are explicitly outside `mochiko-cli`'s template set per D9-schema-governance-envelope).
9. `BACKLOG.md`: first-live-run watch (D10-per-command-rollout) — now carrying the C2 **delivery probes** (schema
   read? read fully? before first action?) and the D2-rules-prose-split **benefit observations**; the shared
   trigger evidence for D2-rules-prose-split and D4-label-job-scope.

**Cost line (M5).** Priced and accepted: a standing two-file read at command fire (`.md` +
schema) with YAML structural overhead on the pipeline's single downstream run; YAML block
scalars carrying prose dense in `:`/`**`/backticks (checker catches parse breaks); Edit
exact-match authoring against block-scalar indentation. The compensating asset is the
deterministic check surface (Problem, driver 1).

## Open questions

- ~~Two schema grammars coexist; `kind:` discriminator left to the build~~ — **closed at
  review (M2/D13):** three grammars coexist (template · shelf · command); `kind:` is decided
  yes, carried by the D13-advisory-schema-checker checker.
