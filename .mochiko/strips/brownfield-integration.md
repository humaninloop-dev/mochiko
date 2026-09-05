# Strip notes — `skills/brownfield-integration/`

Entry formats: `strips/README.md`. Wave context: skill-succinctness wave 1 (design:
`.mochiko/brainstorms/skill-succinctness-strip/record.md`, batch-ratified 2026-07-25): body
128 → 111 lines, 17 cut = 13% — **under the 30–70 never-stripped band**; per R3 the under-band
second pass generates the survivor-provenance (KEPT) entries below.

<!-- Wave context: wave 5 of the CLI schema-delivery build (v0.106.0) — the dense-five family
converts: each member's rules are rendered at fire by `mochiko-cli` from the migration log the
plugin carries at `plugins/mochiko/migrations/`, and the skill reads no schema file. Ruling for
every [v0.106.0] entry below: `.mochiko/brainstorms/cli-schema-delivery/record.md` D3 as amended
(the skill-side form — `!` runs in `SKILL.md` and at subagent preload), D7 (the `PreToolUse`
`Skill` limb), D9 (families in the arc's order), and D10 clause 6 (the budgeted quantity re-keys
to body + rendered output), with the wave-open rulings in that session's `wave5-plan.md` and the
`DECISIONS.md` 2026-09-04 row. Pre-edit verbatim text:
`git show 7d098b9:plugins/mochiko/skills/brownfield-integration/SKILL.md`. -->

## [v0.106.0] the Rules block — raw schema Read superseded by CLI delivery

- **Disposition:** superseded → `## Rules — delivered by mochiko-cli`: the positive-confirmation
  halt clause plus seven `!` lines, one per rendered block (the preamble and the six sections),
  and the read-back sentence.
- **Tier failed:** n/a — supersession by ruling (`cli-schema-delivery` D3 as amended, the
  skill-side form)
- **Content:** verbatim —

  ```
  ## Rules — load the schema first

  Your first action, before any read or write in the existing file: **Read `schema.yaml`
  (this skill's own directory) raw, in full** — the small families ship no common file, so
  the pair's own schema is the whole first action. The schema is the source of truth for
  this craft's binding rules, nested in six sections, each addressable by its section ID:
  `brownfield-integration.sec.independence` · `brownfield-integration.sec.scope` ·
  `brownfield-integration.sec.inputs` · `brownfield-integration.sec.verdict` ·
  `brownfield-integration.sec.output` · `brownfield-integration.sec.reserved`. Interpret it
  live: a rule's `kind:` names what it is, and an absent `kind:` reads `constraint`; a rule
  of `class: floor` is always read and always delivered; a `pointer:` rule binds you to that
  file's or skill's procedure, referenced never restated; labels come from
  `plugins/mochiko/schemas/skill-labels.yaml`. The floor pin: the 6 rules of `class: floor`
  are non-waivable. Before the first read step, state the floor count back — a skipped or
  partial read leaves that count blank: halt and surface it, and halt likewise if the
  schema's `class: floor` count disagrees with the pin.
  ```

- **Kept deliberately:** everything outside this section, byte for byte — the title, the opening
  paragraph, and every procedural section. The block's substance survives in the render: the six
  section IDs are now the six `--section` arguments, and the interpretation grammar the block
  taught is printed as the preamble's `legend` with every fire.
- **Consumers assessed:** none shared — the block was this skill's own text, and this family
  ships no common file.

## [v0.106.0] the hand-pinned `class: floor` count

- **Disposition:** superseded → the CLI-printed pin. The count is the `- class: floor · N rules`
  line under `pins` in the preamble block, and the `floors:` line beneath it lists the ids; the
  read-back sentence now cites both rather than carrying a number of its own.
- **Tier failed:** n/a — supersession by ruling (`cli-schema-delivery` D3, "the counts are computed
  and printed by the CLI, never hand-pinned"; the wave-4 re-key ruling adding the `floors:` index)
- **Content:** verbatim —

  ```
  The floor pin: the 6 rules of `class: floor`
  are non-waivable. Before the first read step, state the floor count back — a skipped or
  partial read leaves that count blank: halt and surface it, and halt likewise if the
  schema's `class: floor` count disagrees with the pin.
  ```

- **Kept deliberately:** the read-back obligation itself (skill-content-schema D6 as amended),
  re-keyed from a hand-pinned number to the printed pin and its id list — the skill still states
  the floor set back before its first procedural step, and a blank or partial read-back still
  halts. The independent second number the `.md` used to hold is booked as a loss, per D3's own
  rationale; the contract suite's `assert_floor_delivery` now carries what it used to check.
- **Consumers assessed:** `.claude/rules/mochiko/primitive-edits.md` skill-pair criterion 3, which
  demanded the hand-pinned count — amended in this same wave to branch on a converted skill.

## [v0.103.0] Schema conversion — census-row → minted-ID map (skill-content-schema wave 2C, small families)

Ruling for every entry below: skill-content-schema D3 (obligations move, procedure stays
prose) / D8/C4 (protected transfers), `DECISIONS.md` 2026-09-01 rows (Skill-content schema
ruled · Skill-schema wave-2 family doors ruled — the small-families door); census:
`.mochiko/brainstorms/skill-content-schema/census-small-families.md` §A (BI) + §B (BI rows
1–11). Schema home: `plugins/mochiko/skills/brownfield-integration/schema.yaml`. Minted IDs
carry the `brownfield-integration.` prefix (omitted below). Map — census §B row → minted ID:
1 `consumes-classification-never-decides` · 2 `extend-interface-impact` ·
3 `modify-interface-impact` · 4 `extend-never-silently-modify` · 5 `read-full-file-first` ·
6 `follow-file-conventions` (set-rule, checklist steps 2–5) · 7 `interface-preservation` ·
8 `conflict-detection-set` (set-rule, the four checks) · 9 `flag-blockers-into-cycle-report`
(the five When-to-Flag blockers ride the rule text) · 10 `refactoring-out-of-scope` ·
11 `letter-is-spirit`. 11 rules, no splits.
**Section distribution (review six-set reused by the door ruling):** scope {1, 2, 3, 7,
10} · inputs {5, 6, 8} · verdict {11} · output {9} · reserved {4} · independence — the one
explicit empty marker. **Fit-table deviation, lead-confirmed:** census §B predicted 2 empty
markers (independence + verdict); the letter-IS-spirit epigraph (row 11, a body floor) homes
in `sec.verdict` per the wave-1 RPA/VC precedent, so verdict is non-empty and only
independence carries the marker.
**Floor count 6 (rows 2 · 3 · 4 · 5 · 7 · 11), lead-ruled at plan approval:** the row-grain
enumeration marks 5; the §A [v0.49.0] keep-set's "(floors)" disposition names the entire
consumption discipline — read-before-write, **interface preservation**,
EXTEND-never-silently-becomes-MODIFY, conflict escalation — which is row-grain
protected-set evidence promoting row 7 (`interface-preservation`) to floor (the 2B
vertical-tdd precedent: a protected-set naming beats a class-mix cell). The census tally's
"BI 6" was right; its path ran through the wrong cell — correction queued for the census §K
landing appendix. Row 9 stays must (the §A "protected escalation seam" carries no floor
claim). No `conditions:` block — EXTEND/MODIFY is the rules' own subject (content-derived;
census §B's live-`when:` dimension list omits BI); the load-first block legally omits the
`when:` grammar sentence (wave-1 RCM-4 wave-wide ruling).
Accounting (seat-measured snapshot; the closer re-measures at the gate): body 6,342 → 3,903
(obligations out + the load-first Rules block in) + schema 6,674 = **payload 10,577**
(census §F estimate ~10,100, ×1.67 vs est ×1.6 — inside the ±30% band); the delta over the
pre-conversion body is structural overhead — no content growth claimed. The old 7,928 body
budget is superseded by the conversion re-seed (ledger's third seeding path, no headroom —
the wave closer executes the ledger row). Description byte-untouched at 491.

## [v0.103.0] EXTEND/MODIFY table, checklist, conflict detection, When-to-Flag — protection transfers ([v0.25.0] KEPT set)

- **Disposition:** superseded — protection transfers per D8/C4 onto the schema rules; the
  body sections leave (a floor kept inline beside its schema rule would be the D6
  anti-dual-homing violation): the EXTEND/MODIFY consumption table's cells →
  `brownfield-integration.extend-interface-impact` (floor) +
  `brownfield-integration.modify-interface-impact` (floor); the Read-Before-Write
  checklist → `brownfield-integration.read-full-file-first` (floor, step 1) +
  `brownfield-integration.follow-file-conventions` (must set-rule, steps 2–5); Interface
  Preservation → `brownfield-integration.interface-preservation` (floor, promoted — map
  entry above); Conflict Detection → `brownfield-integration.conflict-detection-set`
  (must set-rule, all four checks); When to Flag →
  `brownfield-integration.flag-blockers-into-cycle-report` (must, routing — all five
  blockers and the cycle-report seam ride the rule text, pointer
  `mochiko:executing-tdd-cycle`).
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01
  skill-content-schema rows; protecting lineage the [v0.25.0] KEPT entry below —
  batch-2 ratification, `DECISIONS.md` 2026-07-25 skill-succinctness-strip — and the
  [v0.49.0] keep-set, `DECISIONS.md` 2026-08-02 plan-task-granularity D2.1).
- **Content:** the five `## Core Process` subsections whole — table cells, five checklist
  steps, three preservation bullets, four conflict checks, five flag bullets — wording
  preserved verbatim in substance in the rule texts; verbatim originals in git history
  (pre-v0.103.0).
- **Kept deliberately:** the Rationalizations table (whole, per its census
  body-stays-prose disposition), the Common Mistakes table, and the Red Flags thought
  bullets stay body prose; the v0.64.0 reconciliation's finding — "The When-to-Use cut
  removes NONE of these" — now reads: every protected element survives as a schema rule,
  protection transferred, none deleted.
- **Consumers assessed:** `mochiko:staff-engineer` mounts the skill; `executing-tdd-cycle`
  co-fires and owns the cycle report the flag rule points at (seam unchanged);
  `patterns-vertical-tdd` owns the marker vocabulary (referenced, never restated);
  none links the removed section anchors.

## [v0.103.0] Boundary, epigraph, refactoring carve — protection transfers + body slims

- **Disposition:** superseded — protection transfers per D8/C4:
  the Overview's classification provenance and boundary sentences →
  `brownfield-integration.consumes-classification-never-decides` (must, pointer
  `mochiko:patterns-vertical-tdd`); the letter-IS-spirit epigraph + the Red-Flags
  `**No exceptions:**` block → `brownfield-integration.letter-is-spirit` (floor,
  `sec.verdict`); the extend-never-silently-modify warning →
  `brownfield-integration.extend-never-silently-modify` (floor, `sec.reserved` — surfaced,
  never seat-resolved); the When-NOT refactoring bullet's obligation clause →
  `brownfield-integration.refactoring-out-of-scope` (must), the bullet itself slimming to
  its category name.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 rows;
  protecting lineage `DECISIONS.md` 2026-08-05 ponytail-concepts-integration PT-D7
  (rung-zero thin-form pointers) + 2026-08-02 plan-task-granularity D2.1 for the boundary;
  the epigraph's home ruling is the wave-1 RPA/VC verdict-section precedent).
- **Content:** "Brownfield tasks arrive tagged `[EXTEND]` or `[MODIFY]` by the builder's
  own decomposition — classified from the cycle card's **brownfield exposure** line, which
  `patterns-vertical-tdd` declares at design time. This skill does not decide the
  classification…" · "**Violating the letter of the rules is violating the spirit of the
  rules.** Every shortcut in read-before-write discipline is a broken consumer waiting to
  surface." · the five No-exceptions bullets · "**Never treat an `[EXTEND]` task as a
  `[MODIFY]`.** If you believe the existing code cannot support the extension without
  changing its interface, surface it as a blocker — do not silently rewrite." ·
  "Refactoring work — out of scope for an extend/modify task; note the opportunity, do not
  act on it."
- **Kept deliberately:** the Overview's craft summary and the consequence aphorism ("The
  existing code is not wrong until proven otherwise…"). Two conversion-collateral re-keys,
  disclosed (the v0.24.0 dangling-reference class): the Common Mistakes fix cell "(checklist
  step 3)" and the Rationalizations cell "Step 1 exists because…" referenced the deleted
  checklist section — re-keyed to "exactly" and "The read-first floor exists because…";
  meaning unchanged, referent re-homed.
- **Consumers assessed:** staff-engineer · executing-tdd-cycle (co-fires, cross-links) ·
  patterns-vertical-tdd · the router row — none references the moved sentences by anchor;
  the consumes-classification + co-fire framing the [v0.49.0] ruling re-keyed survives in
  the untouched description.

## [v0.64.0] Guardrails body + slim description (guardrails-vs-detail Wave 2 editorial cut)
- **Disposition:** superseded → Wave 2 editorial guardrails cut (D4 cut line).
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail Wave 2, `DECISIONS.md`
  2026-08-11 build row Wave 2 residual + user rulings 2026-08-10/11; method warrant: benchmark
  verdict `.mochiko/benchmarks/guardrails-vs-detail/report/final-verdict.md`).
- **Content (faithfully compressed):** body 6,657 → 6,342 chars (−5%); description 913 → 491
  chars (−46%). Body cut: the **When to Use** section deleted whole (four bullets restating the
  description's invocation conditions — `[EXTEND]` marker, `[MODIFY]` marker, a task referencing a
  file on disk, following prior patterns; each obligation survives in the EXTEND/MODIFY
  consumption table, the Read-Before-Write checklist, and Interface Preservation). Description
  cut: the "detecting conflicts before adding code" clause and the design-time-declaration
  provenance ("the builder assigns at decomposition time … declared at design time by
  patterns-vertical-tdd") compressed; the MUST clause, core triggers, and the
  `executing-tdd-cycle` co-fire sibling distinction kept. Verbatim homes: git history of this
  file (pre-v0.64.0).
- **Old description (verbatim):**
  > This skill MUST be invoked when implementing a task that touches existing code — safely making an `[EXTEND]` or `[MODIFY]` change to a file already on disk: reading the whole file before writing, following its established patterns, preserving its interface, and detecting conflicts before adding code. SHOULD also invoke when extending an existing file, modifying existing behavior, integrating against an established interface, or following patterns set by prior work in the codebase. Consumes the extend/modify classification the builder assigns at decomposition time (from the cycle card's brownfield exposure, declared at design time by patterns-vertical-tdd); this is the implement-time, read-before-write craft of making that one modification safely — NOT the execution of the cycle the task belongs to (that is executing-tdd-cycle, which co-fires on the same brownfield task and drives red/green/refactor).
- **Kept deliberately:** the guardrails keep-set — the Overview + consequence aphorism, When NOT
  to Use, the EXTEND/MODIFY consumption table, the Read-Before-Write checklist, Interface
  Preservation, Conflict Detection, When to Flag, the Common Mistakes table, the Common
  Rationalizations table, and the Red Flags section.
- **MANDATORY KEPT reconciliation:** this file's [v0.25.0] KEPT entry protects the EXTEND/MODIFY
  consumption table, the Read-Before-Write checklist, Conflict Detection, When to Flag, and the
  Rationalizations table; the [v0.49.0] supersession KEPT "the entire consumption discipline
  (read-before-write, interface preservation, EXTEND-never-silently-becomes-MODIFY, conflict
  escalation)." **The When-to-Use cut removes NONE of these** — every protected element is a
  distinct surviving section; only the invocation-condition bullets (in no KEPT set) were
  deleted. The slim description preserves the consumes-classification + co-fire framing the
  [v0.49.0] ruling re-keyed. No prior KEPT or protected line is touched.
- **Consumers assessed:** staff-engineer (mounts it) · executing-tdd-cycle (co-fires, cross-links)
  · patterns-vertical-tdd, patterns-code-minimalism (cross-reference) · implement · mochiko
  router. None links the removed When-to-Use bullets or a description clause. Contract intact.

## [v0.49.0] Marker source re-keyed — builder classifies at decomposition
- **Disposition:** superseded → tasks arrive tagged by the builder's own decomposition, classified from the cycle card's brownfield-exposure line (declared by patterns-vertical-tdd at design time)
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-02 "Task layer de-granularized + slice dissolved into specify (D1–D9)"; record `.mochiko/brainstorms/plan-task-granularity/record.md`, D2.1)
- **Content:** "the marker **vocabulary** is defined by `patterns-vertical-tdd`, which stamps those markers onto tasks at design time" and the parallel description/interface-table clauses.
- **Kept deliberately:** the entire consumption discipline (read-before-write, interface preservation, EXTEND-never-silently-becomes-MODIFY, conflict escalation) — untouched.
- **Consumers assessed:** staff-engineer · executing-tdd-cycle (co-fires).

## [v0.25.0] Common Mistakes densified: 4 subsections → 4-row table (net −17 lines)
- **Disposition:** compressed in place (densification, zero deletions)
- **Tier failed:** n/a — form only
- **Content:** not-reading-full-file, silent-rewrite-on-extend, ignored error handling, "better" patterns
- **Consumers assessed:** 4 consumer files checked at wave open; none reference the subsection headings

## [v0.25.0] Red Flags inline rebuttals trimmed to thought-only bullets
- **Disposition:** relocated → the Common Rationalizations table (in-file — the rebuttals duplicated its rows: "better pattern", "refactor to make it work", "clean it up"); the one flag without a table row ("existing tests don't cover this") keeps its inline rebuttal
- **Tier failed:** 1 (second encoding of the table's rebuttals; the flags name the thoughts, the table rebuts them — the validation-constitution pattern)
- **Content:** per-bullet rebuttal clauses
- **Consumers assessed:** none reference the section

## [v0.25.0] Interface Preservation — 3 bullets restating the EXTEND/MODIFY table
- **Disposition:** relocated → the EXTEND/MODIFY consumption table (in-file; the section header now points at it)
- **Tier failed:** 1 (the signature / export-surface / public-API MUST-NOTs are the table's cells verbatim)
- **Content:** the three restating DO-NOT bullets; rename-prohibition + the two DO bullets kept
- **Consumers assessed:** none reference the bullets

## [v0.25.0] Aphorism consequence-anchored (R4b rider, +1 line)
- **Disposition:** consequence attached to the previously bare copy: "Every shortcut in read-before-write discipline is a broken consumer waiting to surface."
- **Tier failed:** n/a — rider execution (bare copy → Tier-2-qualifying in place)
- **Content:** one added consequence line
- **Consumers assessed:** n/a

## [v0.25.0] KEPT: EXTEND/MODIFY consumption table, Read-Before-Write checklist, Conflict Detection, When to Flag, Rationalizations table
- **Tier-2 evidence:** contested at the under-band pass and kept — each names a concrete failure this craft exists to prevent (interface breaks, unseen conventions, name/import collisions, silent workarounds) and the flag-routing seam into `executing-tdd-cycle`'s cycle report; the marker vocabulary stays with `patterns-vertical-tdd` (grammar seam respected, not restated). Session ruling: batch-2 ratification 2026-07-25.
