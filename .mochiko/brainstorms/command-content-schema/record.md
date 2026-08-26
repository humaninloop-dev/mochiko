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
  (`template:` + `sections:` + `skeleton:`; D8, `schema-based-template-guidance` — data files
  are the source of truth, `mochiko-cli` renders, raw Read the first-class fallback, GI-020)
  **plus one shelf-data schema** (`architecture-shelf-backend.yaml`: `shelf:`/`dimensions:` —
  a third grammar was already on disk before this session).
- **F3 — rules.json precedent** *(corrected at review, M1)*. `evals/*/rules.json` (**6**
  files, from the compression-eval infrastructure): flat rule inventory `{id: R-XXX, rule:
  statement + "Evidence:" clause, class: floor|must|format, source: file:section anchors}` —
  a **derived** inventory pointing back at markdown truth. Built per-eval, not shipped, no
  variables, no clusters.
- **F4 — constraint envelope.** GI-019: kernel-class tooling only by recorded ruling; the
  standing bright line — never gates pipeline progress, never dispatches or sequences agents,
  never holds judgment skills own. D11 admitted the template-schema CLI **for artifact
  templates specifically**; extending the renderer to command content is a new admission
  needing its own recorded ruling. GI-020: plugin install stays markdown-only — commands must
  ship as functioning `.md` whatever the schema does.
- **F5 — audit keying today.** The charter audit (`primitive-edits.md`) grades implement.md
  against prose criteria (floor present + goal contract present, every FAIL clause surviving);
  strips quote content verbatim. Neither is ID-addressable today.

## Problem — why this session (folded at review, C1)

Four failures in the all-prose command form:

1. **No deterministic governance surface.** Prose is auditable only by model judgment. A
   schema's structure is machine-checkable — every ID unique, every label registry-valid,
   every `${var}` bound, every `ruling:` anchor resolving to a live `DECISIONS.md` row —
   deterministic checks the ceremony cannot run today at any cost. (GI-019 carves advisory
   post-hoc checkers out of kernel-class; this is governance-legal by standing ruling.)
2. **Unaddressable content.** Strips, audits, and traceability key on verbatim-quote hunting;
   the audit has already nearly lost a protected line once (v0.34.0). Nothing can be cited by
   name.
3. **No query surface.** Labels give cross-command views; deviation checking becomes a diff of
   same-labeled rules instead of reading everything.
4. **One change regime forced onto two kinds of content.** Rules are atomic and near-immutable
   under ceremony; narrative is voice that should stay freely editable. The schema separates
   the regimes — this is D2's split stated as its real reason, not an aesthetic line.

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
risk; that risk is accepted eyes-open at the C2 disposition (D7 hardening).

## Decisions

### D1 — YAML is the source of truth; the model interprets it at runtime — `Assumed`

*(Re-marked `Assumed` at review, I6: the model-interpreted delivery instrument has never
executed — n=0, D10 — and the house idiom for a never-executed instrument is `Assumed`.)*

**Statement:** the schema data file (`implement.yaml` shape) is the source of truth for the
content it carries. The command `.md` loads it and the **model interprets it live** at command
fire — "render" means runtime interpretation by the agent, not a build-time generation step.
No binary is required on the read path (raw Read stays first-class, GI-020 satisfied);
`mochiko-cli` rendering can be added later as an optional human-facing view.

**Rationale:** the user's stated intent at Q1 clarification — a thin scaffold around the
schema with the ability to effectively interpret. Matches the D8 template precedent's
data-as-source-of-truth posture while dropping its build-time render step.

### D2 — Stage 1 split: rules move to the schema, narrative stays prose; absorption trigger on record — `Confident`

**Statement:** stage 1 moves the **rule-like content** of `implement.md` into the schema
(ID-tagged blocks); the charter narrative (Identity & Mission, protocol prose voice) stays in
the `.md`. An **absorption trigger** is on record *(benefit-keyed at review, C1)*:
stage 2 — narrative absorbs into the schema, `.md` thins to scaffold — is pre-authorized when
the first live `/mochiko:implement` run under the schema shows **(a) delivery** — the
schema-carried rules read fully, before first action, no miss attributable to YAML carriage —
**and (b) at least one concrete benefit** — an `impl.*` ID cited by a strip, audit finding, or
`DECISIONS.md` row · a `vars:` change replacing what would have been a multi-site edit · the
advisory checker (D13) catching a real defect. Stage 2 then lands as an ordinary build citing
this record. *(Retreat branch, I4:)* contrary evidence does not merely hold the split — where
the evidence shows the **split itself** harming (drift between the two homes, floors missed on
the schema side), retreat to all-`.md` is a named option **reserved to the user**; the build's
strips (item 4) keep both directions reconstructible per GI-006.

**Rationale:** the lead recommended whole-move (A) on drift/boundary/injection-reach/audit
grounds; the user weighed the steering risk — a YAML-carried mission is unproven (n=0) — and
ruled the staged middle path the lead offered: B's graceful degradation without B becoming
permanent. House idiom: `not-now + trigger` stance rows, sound-loop rules-file first-miss
deferral.

### D3 — No shared rule library: per-command rules + a common label vocabulary — `Contested`

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

### D4 — Label job: navigation now, edit-time drift check as the goal — `Confident`

**Statement:** stage 1 ships labels as **query/navigation only** — cross-command views by
label, no ceremony obligation. The goal state is the **edit-time drift check** (editing a
labeled rule surfaces same-labeled rules in other command schemas; the editor states aligned
or diverged-on-purpose, folded into the primitive-edit ceremony). Graduation is
benefit-keyed *(amended at review, C1/I3)*: the ceremony hook lands citing this record when
the vocabulary has survived — without label churn — **either** the implement build plus the
first rollout command, **or** three implement-touching edits (the within-implement path keeps
the criterion reachable while D10 schedules no second command), **and** a label query was
actually used at least once in real work.

**Rationale:** user's ruling — "B for now, but then goal is to be A … an effective B gives
confidence for A." Same staged-trigger idiom as D2.

### D5 — Variables: schema-local `vars:` block + `${var}` substitution at read — `Confident`

**Statement:** each command schema carries a `vars:` block (seat names, paths, bounds); rule
text carries `${var}` placeholders; the model substitutes at read time. One place to change a
value; text stays readable as text. Runtime repo-state resolution declined for now; a value
class can graduate later by its own ruling. *(Sigil amended at review, I2 — verified:
`{{...}}` already ships across four template schemas and four `templates/*.md` meaning "blank
the agent fills with authored content"; same directory, opposite semantics. Var substitution
uses `${var}` to keep the two conventions unconfusable.)*

**Rationale:** the user's original ask, literally; ruled as recommended.

### D6 — Rule-block grammar adopted — `Confident`

**Statement:** a rule block is `{id, labels: [...], class: floor|must|advisory, text,
ruling?: <DECISIONS.md anchor>, pointer?: <skill>}` under a top-level `vars:` block. ID format
*(amended post-review at the user's ask)*: **dotted slug** — `impl.<kebab-name>`, e.g.
`impl.design-seats-staffing`, `impl.attempt-exemption-user-only`, with `fail-condition` rules
under `impl.fail.<name>`; the slug is a **name, never a summary**, frozen at mint per D11, and
the D13 checker enforces uniqueness and format. `ruling:` machine-tags protected content — the
audit's preserved-responsibilities check and the strip ceremony gain an addressable anchor.
`pointer:` carries the skill fence: where a skill owns the floor, the rule holds the pointer,
never the procedure. `class` reuses the eval taxonomy shape, and has a named consumer
*(M3 fold)*: the charter audit grades `floor`-class rules as must-survive; `advisory`-class
rules may change without supersession ceremony.

**Rationale:** adopted as drafted from the preview; ceremony wiring from day one preferred
over minimal-then-grow.

### D7 — Stage-1 scope: all rule-shaped content moves, FAIL list included; source = the simplified rewrite — `Confident`

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
hard-codes the `fail-condition` rule **count** — a stale count trips the D13 checker, and a
lead that never read the schema cannot fake the clauses · the D10 first-live-run watch gains
delivery probes — schema read? read fully? before first action? · the user's stated
compensator: "having deterministic script check gives better ability." The reviewer's
alternative — hold `fail-condition` rules inline for stage 1 — was offered and declined.)*

### D8 — Label vocabulary: controlled registry file + ten-label seed — `Confident`

**Statement:** the vocabulary ships as `plugins/mochiko/schemas/command-labels.yaml` — one
line of meaning per label; every rule's `labels:` values must come from it; new labels enter
by registry edit first (normal shipped-primitive ceremony). Seed set (10): `independence` ·
`user-gate` · `fail-condition` · `attempt-economy` · `landing` · `evidence` · `scope-entry` ·
`seats` · `floor-pointer` · `reporting`.

**Rationale:** rules stay per-command (D3), but the vocab is the one deliberately shared
surface — a registry under ceremony is what keeps it common instead of drifting per command.
Ruled as recommended, full seed.

### D9 — Governance envelope: no new kernel admission; schemas are shipped primitives; audit re-keys to the pair — `Confident`

**Statement:** model-interpreted command schemas need **no new kernel-class admission** — the
schema is data, the interpreter is the model; nothing executable gates pipeline progress or
dispatches agents, so GI-019 is untouched. A future `mochiko-cli` render/`--check` over
command schemas would extend the admitted CLI and takes its own ruling note at that time.
`implement.yaml` and `command-labels.yaml` are shipped primitives under the full strip +
author≠grader ceremony (the v0.76.0 schema precedent). The charter audit re-keys to grade the
**`.md` + schema pair** — floor present + goal contract present across both surfaces, the
FAIL-clause-survival criterion keyed to the `fail-condition` label set — via a
`primitive-edits.md` edit riding the build wave. GI-020 holds: install ships markdown + data
files, nothing heavier.

**Rationale:** proposed whole by the lead; adopted as stated.

### D10 — Rollout by per-command ruling; first-live-run watch is the shared trigger evidence — `Confident`

**Statement:** implement converts first (this build). Each further command (`feature.md`,
`architecture.md`, the v8 trio) converts **by its own ruling** — door-open idiom, as the
charter ADR did. Evidence honesty: n=0 for model-interpreted schema delivery; a
**first-live-run watch** on schema-carried rule delivery is owed in `BACKLOG.md`; its outcome
is the trigger evidence for D2 (narrative absorption) and for D4's first path, while D4's
within-implement path (three implement-touching edits) accumulates as edit observations in
the **same watch item** *(N3 repair)*.

**Rationale:** proposed whole by the lead; adopted as stated.

### D11 — ID lifecycle: mint-once, frozen, tombstoned — `Confident` *(review fold, C3)*

**Statement:** an `impl.*` ID is minted once and never reused. A reword preserves the ID; a
split mints children and records the parent; a merge retires the losers with a tombstone entry
(ID + disposition) so no anchor ever dangles. Slugs are names, not summaries — wording drift
never renames. Enforcement: the D13 checker verifies format, uniqueness, and tombstone
integrity deterministically; the charter audit verifies continuity (no ID vanished without a
tombstone) as part of preserved-responsibilities. Compression waves obey the same rule — a
pass that rewrites text must carry IDs through unchanged. *(N4 repair:)* a D2 retreat retires
the whole `impl.*` namespace via **one namespace-level tombstone** carried by the recorded
retreat ruling — never per-ID entries.

**Rationale:** three decisions (D6, D7, D9) bind to ID/label persistence; without a stability
rule, protected-content tracking floats and GI-006 reconstructibility breaks.

### D12 — Rule grain: one block per independently-citable obligation — `Confident` *(review fold, I5)*

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

### D13 — Stage 1 ships an advisory deterministic checker — `Confident` *(review fold, C1; user-ruled "yes ship the checker")*

**Statement:** the build ships a minimal advisory checker — a script, exit-code-only signal,
never a required CI gate, never gating pipeline progress (inside GI-019's advisory-checker
carve-out). Checks, all deterministic: ID uniqueness + slug format (D6/D11) · every label ∈
`command-labels.yaml` (D8) · every `${var}` bound in `vars:`, no orphan placeholders (D5) ·
every `ruling:` anchor resolving to a live `DECISIONS.md` row (D6) · tombstone integrity
(D11) · the `.md` Not-done line's `fail-condition` count matching the schema (C2 guard) · a
`kind:` discriminator present *(closing the open question — decided yes at M2: three grammars
now coexist under `schemas/`, top-level shape no longer discriminates)*. Its output is cited
in the audit brief as a deterministic pre-pass — the char-budget pre-assert idiom. Crate
extension stays reserved per D9; the script is standalone.

**Rationale:** the user's core driver — "the ability to enforce deterministic checks on schema
… opens up a lot better governance"; checkability without a checker would be a recorded hope.

*(Mark split at verify, N5: the decision to ship is `Confident`; the efficacy claim — the
checker catching real defects — is `Assumed`, n=0, and a caught defect is itself one of D2's
benefit observations.)*

### D14 — Nested section grammar: sections are first-class nodes — `Confident` *(post-build amendment, 2026-08-26, user-ruled)*

**Statement:** amends D6. The schema's top level is a `sections:` list, each section
`{id, title, intent, rules}`; rule blocks nest under their section, their own grammar
unchanged. Section IDs use the `<cmd>.sec.<slug>` segment (`impl.sec.roles` ·
`impl.sec.reserved` · `impl.sec.tools` · `impl.sec.ways-of-working` · `impl.sec.boundaries` ·
`impl.sec.fail-conditions`), minted once and tombstoned under the same D11 lifecycle as rule
IDs. Section metadata stays thin — `title` verbatim from the charter group, `intent` one
navigation line; sections never grow a second prose surface (narrative stays in the `.md`).
The `.md` points at section IDs; the checker asserts the section grammar and prints
per-section stats; a top-level flat `rules:` key is a checker finding. The v0.92.0 flat form's
comment dividers are superseded — they were invisible to the checker and unpointable from the
`.md`.

**Rationale:** the user, reviewing the shipped v0.92.0 file: the flat list "is not utilizing
nesting" — the `#` divider groups should be real sections with metadata the `.md` can point
to. Membership becomes data (checkable, per-section stats) instead of comment convention.
All 104 rule IDs and texts carried unchanged — pure relocation, D11 continuity trivial.

### D15 — Referential closure: rule texts are self-contained — `Confident` *(post-build amendment, 2026-08-26, user-directed)*

**Statement:** amends D6. A rule's `text` must be **referentially closed**: every reference in
it resolves within the block itself or the schema's addressable namespace — `${var}` names,
`impl.*` rule IDs, `impl.sec.*` section IDs, `class:` values, registry labels, `pointer:`
skills, `ruling:` anchors, and literal file paths. **Deixis is a defect:** a pointing word
whose referent lives outside the block ("these rules", "this section", "above", "below",
document-shape remarks like "There is no X section") breaks the D12 promise that a block is
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
audit (the lint's first catch); the law prevents the class at D10 rollout.

## Session trail

- **Q1 — source of truth** (structured fork A/B/C): user rejected the framing and clarified —
  thin `.md` scaffold around `implement.yaml`, model interprets at runtime. → D1.
- **Q2 — how thin** (whole charter moves vs rules-only vs overlay; lead recommended
  whole-move): user picked rules-only, had second thoughts, asked for the A-vs-B case; lead
  gave drift/boundary/reach/audit for A, steering-risk + ratchet steelman for B, offered the
  staged middle path; user asked what an absorption trigger is; ruled **B with absorption
  trigger**. → D2.
- **Q3 — reuse shape** (shared library recommended / per-command inline / library-of-bindings):
  user ruled against extraction — per-command rules + a common label vocab as the command
  link. → D3 (`Contested`).
- **Q4 — label job** (drift-check ceremony recommended / navigation-only / staged): user ruled
  "B for now, goal is A; effective B gives confidence for A". → D4.
- **Q5 — variables** (vars block + placeholders recommended / typed fields / runtime
  resolution): ruled as recommended. → D5.
- **Q6 — rule-block grammar** (adopt drafted grammar / trim / adjust; preview shown): adopted
  as drafted. → D6.
- **Q7 — stage-1 scope** (all rule-shaped content incl. FAIL list recommended / FAIL stays /
  adjust; simplified-rewrite premise stated): ruled as recommended. → D7.
- **Q8 — vocab home** (registry + ten-label seed recommended / trimmed seed / no registry):
  adopted with full seed. → D8.
- **Q9/Q10 — wrap batch** (governance envelope + rollout/probes proposed whole): adopted as
  stated. → D9, D10.
- **Post-review trail:** sizing gate — solo recommended, "as recommended" · C1 presented with
  a drafted problem statement; the user supplied the deeper driver set (deterministic checks ·
  label queries · deviation checking · atomic-rules-vs-prose separation) — redraft adopted,
  and the follow-up "ship the checker now?" ruled **yes** → Problem section + D13 · C2
  presented as a two-door fork; user ruled door A (D7 stands, hardened) with the risk
  explicitly accepted: "i understand the risk not having all in .md poses but having
  determinstic script check gives better ability" · I1 ruled recorded-as-rejected with the
  three principled reasons · post-review user ask — "i want the id to be easier to understand"
  — four-option fork (dotted slug / numeric+name / section-keyed / label-keyed), ruled
  **dotted slug** → D6 amendment.
- **Post-build amendment (2026-08-26, v0.93.0):** user, reading the shipped flat schema —
  "i dont think it is utilizing nesting … create sections and some metadata of section, and
  in the implement command md point to those section rules." Three-option fork (nested
  sections with IDs recommended / flat + `section:` key / nesting without IDs), ruled
  **adopt as drafted** → D14.
- **Post-build amendment 2 (2026-08-26, v0.94.0):** user, probing `impl.staffing-latitude`'s
  "these rules" — "what does 'these' refer to? is it creating dependency on other rule" — and
  directing the general form: "think at a higher level, build a way that be implemented
  generally." → D15 (referential closure + checker deixis lint); the one live instance
  reworded, ID kept.

## Review + disposition trail

**Sizing:** solo cold review, user-ruled "as recommended" at the named gate.
**Dispatch:** blind two-message per the charter — message 1 topic + goal only (fence held:
nothing under this session's directory read); 30-angle Phase 0 map returned before the record
path was sent.
**Verdict:** `critical-gaps` — 4 Critical · 6 Important · 5 Minor survived the reviewer's own
cross-examination; 14 further findings raised and killed by the reviewer (kill list in the
review output, incl. GI-019 correctly passed, frontmatter fidelity, dual-maintenance vs D8,
harness-load-path).
**Dispositions:** C1 → Problem section + benefit-keyed D2/D4 triggers + D13 checker (ruled
individually) · C2 → D7 hardened, door A, risk accepted eyes-open (ruled individually) · I1 →
rejected-road entry with reasons (ruled individually) · C3 → D11 · C4 → build step 0 · I2 →
D5 `${var}` · I3 → D4 within-implement path + seed validation · I4 → D2 retreat branch · I5 →
D12 · I6 → D1 re-marked `Assumed` · M1/M2 → F3/F2 corrected, `kind:` decided (D13) · M3 →
`class:` consumer named (D6) · M4 → strip-verbatim rule (build item 4) · M5 → cost paragraph
(build surface) — the batch user-ruled "as recommended".

**Verify trail:** round 1 NOT CLEAN — 3 blocking (B1 stale `{{placeholder}}` in D5's heading ·
B2 D12's worked example under-counting its own referent, four blocks where the grain yields
five · B3 the record over-claiming approval on the Ways of Working + Boundaries text — the
floor text — against the referent's own honest provenance header) + 7 non-blocking (N1/N2 F1
now-false "no file write" and wrong line count · N3 D10 not amended alongside D4 · N4 retreat
vs tombstone gap · N5 D13 mark split · N6 pin N=15 · N7 landing residue, deferred to
acceptance by design). All lead-repaired same round except N7. Round 2: bounded re-verify of
the repairs — **CLEAN**: 9/9 landed, no new contradiction; B2's five-block example verified
complete against the referent (exactly five obligations, no sixth; `impl.gap-rework-bound`
correctly outside the `impl.fail.*` prefix); F1's four counts independently re-measured
exact; B3's wording agreeing across F1, D7, step 0, and the artifact header; N=15
independently re-counted. Acceptance followed the CLEAN verify, with the Ways of Working +
Boundaries build-gate confirmation restated to the user at the acceptance gate.

## Build surface (cold-buildable)

One wave, landing the earlier-approved simplified rewrite and the schema together:

0. **Durable referent first (C4):** the simplified rewrite lands at
   `.mochiko/brainstorms/command-content-schema/implement-rewrite.md` as a session artifact —
   the build's source text and the fidelity audit's referent (done in-session). Without this
   step the wave is not cold-buildable. **The build wave's user gate explicitly confirms the
   Ways of Working + Boundaries text** — the two sections whose approval rides that gate per
   F1 (B3).
1. `plugins/mochiko/schemas/command-labels.yaml` — the D8 registry, ten-label seed. The build
   **validates the seed against the real rule inventory** (I3): every rule labelable, no
   single-member or catch-all labels; a mismatch takes the revise-the-seed branch (registry
   edit in the same wave, noted in the audit brief).
2. `plugins/mochiko/schemas/implement.yaml` — `kind: command` discriminator (D13) + `vars:`
   block + rules per D6 grammar at D12 grain; IDs dotted-slug per the D6 amendment; `${var}`
   placeholders (D5); content = the rule-shaped inventory of D7, text from step 0's artifact;
   FAIL clauses as `impl.fail.*`; skill-owned floors as `pointer:` rules; protected lines
   carrying `ruling:` anchors.
3. `commands/implement.md` — simplified narrative (Identity & Mission + Adaptive Goal
   Protocol prose), a load-and-follow instruction naming the schema, the Not-done line
   re-keyed to "the 15 rules labeled `fail-condition` in `implement.yaml`" — **N=15 pinned
   from the referent** (`implement-rewrite.md` carries 15 FAIL clauses; C2 guard, N6).
4. Strips: the rewrite's deletion/relocation ledger (rationale cuts, R&R-restatement cuts,
   deviation-grammar single-homing) **plus** supersession entries for every block moving from
   `.md` to schema. **Verbatim rule (M4):** every entry's content field carries the shipped
   v0.91.0 text (what actually left the file — the GI-006 referent); the rewrite delta is
   recorded separately, never co-mingled.
5. The advisory checker (D13) — standalone script, exit-code signal; its checks per D13; its
   output cited in the audit brief as the deterministic pre-pass. Home and language are the
   build's judgment (crate extension reserved, D9).
6. `.claude/rules/mochiko/primitive-edits.md` — the D9 audit re-key (pair grading;
   label-keyed FAIL criterion; D11 ID-continuity check; `class: floor` = must-survive, M3).
7. Audits: author≠grader validators — command-pair coherence + schema fidelity against step
   0's artifact + strip verification, each brief citing the checker's pre-pass output.
8. Gates 4/5/6: `CHANGELOG.md` · `marketplace.json` sync · `cargo test` (binary untouched;
   command schemas are explicitly outside `mochiko-cli`'s template set per D9).
9. `BACKLOG.md`: first-live-run watch (D10) — now carrying the C2 **delivery probes** (schema
   read? read fully? before first action?) and the D2 **benefit observations**; the shared
   trigger evidence for D2 and D4.

**Cost line (M5).** Priced and accepted: a standing two-file read at command fire (`.md` +
schema) with YAML structural overhead on the pipeline's single downstream run; YAML block
scalars carrying prose dense in `:`/`**`/backticks (checker catches parse breaks); Edit
exact-match authoring against block-scalar indentation. The compensating asset is the
deterministic check surface (Problem, driver 1).

## Open questions

- ~~Two schema grammars coexist; `kind:` discriminator left to the build~~ — **closed at
  review (M2/D13):** three grammars coexist (template · shelf · command); `kind:` is decided
  yes, carried by the D13 checker.
