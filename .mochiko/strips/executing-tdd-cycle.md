# Strip notes — `skills/executing-tdd-cycle/`

Entry formats: `strips/README.md`. Wave context: workflow-token-reduction wave 1 (design:
`.mochiko/brainstorms/workflow-token-reduction/record.md`, D3 + S8 + Q6; rulings ratified
2026-07-23) — reports strip to their verified consumers, machine-first. Skill-succinctness
wave-1 entries atop (batch-ratified 2026-07-25): body 164 → 140 lines, 24 cut = 15% — in the
10–40 previously-stripped band.

Verbosity/caveman wave-1 entries atop (design:
`.mochiko/brainstorms/verbosity-caveman-ops-separation/record.md`, D4 as folded at review
(S2/S13); ruling: `DECISIONS.md` 2026-08-01 "Output verbosity, caveman & ops separation
ruled" row) — the report repair: the format text that forced prose onto passing cycles is
corrected, and the envelope's register and prose-on-clean check are bound where the report is
actually authored.

<!-- Wave context: wave 5 of the CLI schema-delivery build (v0.106.0) — the dense-five family
converts: each member's rules are rendered at fire by `mochiko-cli` from the migration log the
plugin carries at `plugins/mochiko/migrations/`, and the skill reads no schema file. Ruling for
every [v0.106.0] entry below: `.mochiko/brainstorms/cli-schema-delivery/record.md` D3 as amended
(the skill-side form — `!` runs in `SKILL.md` and at subagent preload), D7 (the `PreToolUse`
`Skill` limb), D9 (families in the arc's order), and D10 clause 6 (the budgeted quantity re-keys
to body + rendered output), with the wave-open rulings in that session's `wave5-plan.md` and the
`DECISIONS.md` 2026-09-04 row. Pre-edit verbatim text:
`git show 7d098b9:plugins/mochiko/skills/executing-tdd-cycle/SKILL.md`. -->

<!-- Wave context: wave 6 of the CLI schema-delivery build (v0.107.0) — the end state. No schema
file ships in the plugin: the 20 files under `plugins/mochiko/schemas/` and the 30
`skills/*/schema.yaml` were deleted, and every delivery they served now has a CLI form. Ruling for
the [v0.107.0] entry below: `.mochiko/brainstorms/cli-schema-delivery/record.md` D9 wave 6, with
the `DECISIONS.md` 2026-09-05 row and that session's `wave6-plan.md`. Pre-edit verbatim text:
`git show 62aa99d:plugins/mochiko/skills/executing-tdd-cycle/SKILL.md`. -->

## [v0.107.0] two body lines telling the reader that rules live "in the schema"

- **Disposition:** superseded → "delivered by `mochiko-cli`", section ids unchanged
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/cli-schema-delivery/record.md`
  D9 wave 6; `DECISIONS.md` 2026-09-05)
- **Content:** two lines.
  1. "card reading legally goes, are the schema's `executing-tdd-cycle.sec.inputs` rules."
  2. "is the schema's `executing-tdd-cycle.sec.scope` rules"
- **Kept deliberately:** both section ids, which still resolve, and the substance of both
  sentences — what the card fields are and how far card reading legally goes, and the
  decomposition discipline's full list (task sizing, ordering, extend/modify classification,
  scope, the pre-code ladder walk) with its landing in the cycle report per
  `executing-tdd-cycle.sec.output`. The wording pointed at a file this converted skill is
  forbidden to read, two screens below its own "Never Read a schema file instead" halt clause;
  the wave-5 V2 audit raised the contradiction as a follow-up and this entry discharges it.

## [v0.106.0] the Rules block — raw schema Read superseded by CLI delivery

- **Disposition:** superseded → `## Rules — delivered by mochiko-cli`: the positive-confirmation
  halt clause plus seven `!` lines, one per rendered block (the preamble and the six sections),
  and the read-back sentence.
- **Tier failed:** n/a — supersession by ruling (`cli-schema-delivery` D3 as amended, the
  skill-side form)
- **Content:** verbatim —

  ```
  ## Rules — load the schema first

  Your first action, before reading the card or writing any code: **Read `schema.yaml` (this
  skill's own directory) raw, in full** — the small families ship no common file and no stub
  binds, so the pair's own schema is the whole first action. The schema is the source of
  truth for this skill's binding rules, nested in six sections, each addressable by its
  section ID: `executing-tdd-cycle.sec.independence` · `executing-tdd-cycle.sec.scope` ·
  `executing-tdd-cycle.sec.inputs` · `executing-tdd-cycle.sec.verdict` ·
  `executing-tdd-cycle.sec.output` · `executing-tdd-cycle.sec.reserved`. Interpret it live:
  a rule's `kind:` names what it is, and an absent `kind:` reads `constraint`; a rule of
  `class: floor` is always read and always delivered; a `pointer:` rule binds you to that
  file's or skill's procedure, referenced never restated; labels come from
  `plugins/mochiko/schemas/skill-labels.yaml`. The floor pin: the 10 rules of
  `class: floor` are non-waivable. Before the first card-reading step, state the floor
  count back — a skipped or partial read leaves that count blank: halt and surface it, and
  halt likewise if the schema's `class: floor` count disagrees with the pin.
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
  The floor pin: the 10 rules of
  `class: floor` are non-waivable. Before the first card-reading step, state the floor
  count back — a skipped or partial read leaves that count blank: halt and surface it, and
  halt likewise if the schema's `class: floor` count disagrees with the pin.
  ```

- **Kept deliberately:** the read-back obligation itself (skill-content-schema D6 as amended),
  re-keyed from a hand-pinned number to the printed pin and its id list — the skill still states
  the floor set back before its first procedural step, and a blank or partial read-back still
  halts. The independent second number the `.md` used to hold is booked as a loss, per D3's own
  rationale; the contract suite's `assert_floor_delivery` now carries what it used to check.
- **Consumers assessed:** `.claude/rules/mochiko/primitive-edits.md` skill-pair criterion 3, which
  demanded the hand-pinned count — amended in this same wave to branch on a converted skill.

## [v0.103.0] Converted to the `.md` + schema pair form — rule content relocated to `schema.yaml` (wave 2C, small families)

- **Disposition:** superseded → the pair form: obligation content relocated into
  `plugins/mochiko/skills/executing-tdd-cycle/schema.yaml` (26 rules — 10 floor ·
  16 must · 0 advisory — under the review six-section set reused by ruling,
  `executing-tdd-cycle.sec.verdict` deliberately empty), the `SKILL.md` body keeping the
  letter/spirit epigraph + Overview, the new "Rules — load the schema first" block
  (floor pin 10 + read-back), the Core Process seven-step sequence and the
  Reworking / Fixing subsections as procedure prose, the Red Flags list, the Common
  Mistakes table, and Reference Files. The frontmatter `description:` is byte-untouched
  (498).
- **Tier failed:** n/a — supersession by ruling (`skill-content-schema` D1–D9 as amended
  + the wave-2 small-families door ruling, `DECISIONS.md` 2026-09-01 — the dense five
  convert on the B/C drivers, the review six-set reused with explicit empty markers, no
  common file, zero `extends:`; D8/C4 supersession-transfer). Census referent:
  `.mochiko/brainstorms/skill-content-schema/census-small-families.md` §B ETC rows 1–26
  (26 rules and 10 floors exactly as censused). Lead-ruled at plan approval:
  `sec.verdict` empty with its disclosed marker note (the executor's outcome grammar is
  the report field contract in `sec.output`; the clearing verdict is the lead's — the
  census fit table's own "empty or thin" read). Label note, deliberate: the process
  rules (strict-order · rationalization-stop · the phase-walk duties) carry `ladder`
  (its walk-order clause) — a ruled divergence from wave 2A's `boundary` mapping on
  ATR's STOP rule, chosen so detector runs read it as ruled, not drift. Protection
  transfers to the rule IDs via `.mochiko/provenance.yaml`.
- **Content (superseded body fragments, census-row → rule-ID relocation map; verbatim
  text survives in git history pre-v0.103.0 and verbatim-in-substance in the named
  rules):**
  - When-NOT-to-Use bullet 1 (slicing is `mochiko:patterns-vertical-tdd`'s; "does not
    add, remove, re-scope, or reorder *cycles*") →
    `executing-tdd-cycle.slicing-boundary` (row 1, floor).
  - When-NOT bullet 2 (quality gates + the `**TEST:**` gate are the verifier's,
    `testing-end-user`) → `executing-tdd-cycle.verifier-boundary` (row 2, floor).
  - When-NOT bullet 3 ("the lead Reads the reports and owns that verdict. This skill
    produces its own report; it does not grade one") →
    `executing-tdd-cycle.report-not-verdict` (row 3, floor).
  - When-NOT bullet 4 + the Reworking/Fixing closers ("Whether to rework, how many
    attempts are permitted, and when to stop are the lead's routing decisions — not
    this skill's", the 2026-08-07 pinned line) →
    `executing-tdd-cycle.lead-routing-reservation` (row 4, floor, wording preserved in
    substance).
  - When-NOT bullet 5 (one cycle or one rework, one report; no loop/orchestration
    state) → `executing-tdd-cycle.no-loop-state` (row 5). The When-NOT section leaves
    the body whole — all five bullets are now schema rules.
  - "Execute in strict order. No skipping steps. No reordering." →
    `executing-tdd-cycle.strict-order` (row 6, floor).
  - Step-1 card extraction (fields, ID resolution against the spec/design artifacts,
    first-unchecked-card rule) → `executing-tdd-cycle.card-reading-binding` (row 7).
  - Step-2 decomposition discipline (build time, code in view, task sizing, paths,
    tests-precede, extend/modify classification) →
    `executing-tdd-cycle.decompose-at-build-time` (row 8); "decompose exactly what the
    card's acceptance criteria require — nothing the card didn't ask for" →
    `executing-tdd-cycle.decompose-only-the-card` (row 9); the pre-code ladder sentence
    (per `mochiko:patterns-code-minimalism`, rung disclosed) →
    `executing-tdd-cycle.pre-code-ladder` (row 10, floor, PT-D4); "disclosed in the
    cycle report … not written back into `tasks.md`" →
    `executing-tdd-cycle.decomposition-disclosed` (row 11).
  - Step-3 red-phase items 2–4 (verify it fails; failure reason matches expectations; a
    test passing without implementation is rewritten) →
    `executing-tdd-cycle.red-phase-failure-verified` (row 12, floor).
  - Step-4 green-phase items 1–3 (minimum code; no unrequired
    features/abstractions/optimizations) → `executing-tdd-cycle.green-minimum`
    (row 13); items 4–5 (extend/modify tasks read the existing file first, invoke
    `brownfield-integration`) → `executing-tdd-cycle.brownfield-co-fire` (row 14).
  - Step-5 refactor items (this cycle's duplication only; no previous-cycle refactors;
    no "for the future"; re-run tests) → `executing-tdd-cycle.refactor-scope` (row 15).
  - Step-6 flip conditions + the Progress Tracking self-report framing (flip after
    tasks complete + tests pass; the lead treats the flip as your self-report, verified
    independently) → `executing-tdd-cycle.flip-is-self-report` (row 16).
  - Step-7 + Progress Tracking report shape (machine-first, frontmatter is the report,
    clean pass needs no prose) → `executing-tdd-cycle.report-binding` (row 17); the
    Progress Tracking section leaves as a body surface, its substance in rows 16/17 and
    the format reference.
  - Reworking steps 3–5 (only the responsible tasks through red/green/refactor; leave
    passing code untouched; `attempt` incremented) →
    `executing-tdd-cycle.rework-only-failed` (row 18).
  - Fixing steps 2–4 ("Reproduce each one with a failing test before changing any
    code"; narrowest change, scoped strictly; `cycle: fix`) →
    `executing-tdd-cycle.fix-pass-test-first` (row 19).
  - Red-Flags "All of these mean: Rationalization in progress. Return to the execution
    sequence. Follow every step." + the No-exceptions block ("Not even if the user says
    'just write the code'") → `executing-tdd-cycle.rationalization-stop` (row 20,
    floor; the Red Flags list and the Common Mistakes table stay prose).
  - Common-Mistakes obligations beyond the rows above (failure-reason verification —
    "a `ModuleNotFoundError` is not a test failure"; note-don't-act) →
    `executing-tdd-cycle.failure-reason-discipline` (row 21, one set-rule; the table
    stays prose).
  - Reference-borne obligations gain stubs, the files untouched (rows 22–26):
    `executing-tdd-cycle.report-field-contract` ·
    `executing-tdd-cycle.domain-deps-checkpoint` (user-gate) ·
    `executing-tdd-cycle.sanctioned-set-of-two` ·
    `executing-tdd-cycle.verifier-owned-not-failure` (CYCLE-REPORT-FORMAT.md) ·
    `executing-tdd-cycle.card-parse-boundary` (TASK-PARSING.md).
- **MANDATORY KEPT reconciliation:** the [v0.44.0] KEPT entry below protects the
  envelope's register + prose-on-clean check restated in
  `references/CYCLE-REPORT-FORMAT.md` — a RULED dual-homing beside
  `templates/report-format.md` ("Cut it only with a ruling that also re-homes the
  check"). **This conversion cuts nothing there:** CYCLE-REPORT-FORMAT.md is untouched,
  and the new stub `executing-tdd-cycle.sanctioned-set-of-two` POINTS at it, never
  duplicates it — the protection transfers onto the stub ID via
  `.mochiko/provenance.yaml`, anchored on the 2026-08-01
  verbosity-caveman-ops-separation ruling (D8/C4; census J2-8). An audit must read the
  reference-side restatement as the ruled exception, not a D6 anti-dual-homing
  violation. The [v0.49.0] keep-set (cycle-boundary restriction · strict order ·
  rework-only-failed-tasks · fix-pass scoping · verifier boundary) re-homes to rows
  1/6/18/19/2. The [v0.53.0] keep-set (self-disclosure framing, lead's verdict
  ownership, verifier-grades-independently) re-homes to rows 3/16 and the reserved
  rules. The [v0.64.0] guardrails keep-set: obligations re-home per the map above; the
  When-NOT section it kept leaves the body as ruled MOVE (its five bullets are the
  schema's floors/rules, protection intact on the IDs); the teaching surfaces (epigraph,
  Red Flags, Common Mistakes, Reference Files) stay in the body. The [v0.75.0] and
  [v0.91.0] reference keep-sets (Covers contract, parse-only boundary, card-extraction
  ID resolution) are untouched in TASK-PARSING.md, now stub-covered by rows 7/26. No
  protected line is deleted.
- **Kept deliberately:** the letter/spirit epigraph + its consequence sentence · the
  Overview identity prose · the Core Process seven-step sequence, the Reworking and
  Fixing subsections as procedure frames · the Red Flags list · the Common Mistakes
  table · Reference Files · the `description:` byte-identical at 498.
- **Budget:** re-seed per D8/C1 — delivered-at-invoke payload body 6,464 + schema 12,487
  = **18,951** (was body 9,678 against the 12,095 budget); third seeding path, no
  headroom; the ledger row is the closer seat's write.
- **Consumers assessed:** staff-engineer (mounts it) · implement (binds it) ·
  qa-engineer / review-code-minimalism (read the disclosed decomposition — the
  `decomposition`/`rung` disclosure survives in rows 10/11/17 and the format
  reference) · brownfield-integration (co-fires, row 14) · patterns-vertical-tdd ·
  patterns-code-minimalism (cross-referenced by pointer) · the mochiko router. No
  shipped surface links a removed body section anchor.

## [v0.91.0] Fix round — `references/TASK-PARSING.md` "spec/plan IDs" → "spec/design IDs" (advisory)

- **Disposition:** superseded → "spec/design IDs" at both sites in the reference.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` D1). Raised as an **advisory** by the
  v0.91.0 wave audit: the main pass re-keyed the SKILL.md body's "spec/plan artifacts" but left
  the reference file that defines the `Covers` field still saying "spec/plan IDs" — producer and
  parser would have described the same field in two vocabularies.
- **Content (superseded fragments, verbatim — two sites):**

  1. `Covers` field definition: `- **Covers**: spec/plan IDs this case covers`
  2. `**TEST:**` blocks row: `Each block's `Covers` line cites the spec/plan IDs it verifies.`

- **Kept deliberately:** the `Covers` field itself and its cite-never-re-quote contract; the
  parse-only-to-know-what-the-cycle-must-prove boundary and its hand-off to `testing-end-user`
  for actually running the cases.
- **Budget:** `references/` files are budget-exempt. The SKILL.md body is unchanged by this
  round (9,678 against its 12,095 budget, from the entry below).
- **Consumers assessed:** `mochiko:patterns-vertical-tdd` authors the `Covers` lines this
  reference parses and had its own checklist line re-keyed to "spec/design ID(s)" in the main
  pass — author, parser, and this reference now agree.

## [v0.91.0] Card-extraction ID resolution re-keyed: "spec/plan artifacts" → "spec/design artifacts" — plan-stage retirement D1

- **Disposition:** superseded → cited IDs resolve against the spec and the design artifacts.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` D1; wording ruled by the wave lead
  2026-08-26).
- **Content (superseded fragment, verbatim):**

  ```
  its stories, acceptance criteria (resolve the cited IDs against the spec/plan artifacts),
  ```

- **Kept deliberately:** the whole card-extraction step — stories, acceptance criteria, the
  resolve-the-cited-IDs obligation (IDs are cited on the card and resolved upstream, never
  re-quoted onto it), dependencies, brownfield exposure, and the `**TEST:**` gate.
- **Budget:** body 9,676 → **9,678** against the 12,095 budget; description unchanged at 498
  against 623. Both inside.
- **Consumers assessed:** `mochiko:patterns-vertical-tdd` authors the cards this step reads and
  had its own "spec/plan ID(s)" checklist line re-keyed to "spec/design ID(s)" earlier in this
  wave — card author and card executor now use the same vocabulary. Nothing in this skill's
  build-time decomposition duty changed; the D1 mechanic (d) build-time gate lands on
  `mochiko:patterns-adopt-first`, not here.

## [v0.80.0] CYCLE-REPORT-FORMAT.md — envelope row drops the `slice` key — slice-vocabulary purge

- **Disposition:** superseded → the same field-definition row carrying `report` / `feature` only,
  pointing at `templates/report-format.md` without the slice-scoped clause (the envelope field
  itself was deleted in the same wave — `.mochiko/strips/report-format.md`).
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/decisions/2026-08-19-slice-vocabulary-purge.md`).
- **Content (verbatim, the superseded table row):**

  ```
  | `report` / `feature` / `slice` | envelope | yes | Per `templates/report-format.md` (`slice:` only when slice-scoped) |
  ```

  Replaced by:

  ```
  | `report` / `feature` | envelope | yes | Per `templates/report-format.md` |
  ```

- **Kept deliberately:** the row itself and its envelope pointer — the cycle payload still
  defers to `report-format.md` for the envelope, and the row is what states that. Only the dead
  key and its conditional clause left. Every other row in the field-definition table, and the
  whole conditional-prose section, are untouched.
- **Consumers assessed:** reference file, not a loaded surface — no primitive reads this row
  other than the cycle-report author. Its sibling row in
  `skills/testing-end-user/references/REPORT-TEMPLATES.md` carries the same shape and was edited
  in the same wave.

## [v0.75.0] TASK-PARSING.md — foundation/feature card-type field superseded; test-case-bundle + Covers extraction added

- **Disposition:** superseded → the re-keyed `references/TASK-PARSING.md` Card Pattern and Fields-to-Extract: no card-type annotation, `[P]` derived from dependencies, the card's `**TEST:**` blocks parsed as the named test-case bundle (each with a `Covers` citation line). Execution discipline unchanged.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-16 "Vertical-TDD cycle anchor + QA test-case authorship (D1–D4)"; record `.mochiko/brainstorms/vertical-tdd-complexity-and-qa-role/record.md`, D1/D3 + the D2 acceptance-ID-relocation amendment). Scope is the reference only — the `executing-tdd-cycle` SKILL.md body is untouched.
- **Content:**
  - Card Pattern heading "`### - [ ] Cycle {N}: {title} *({Foundation|Feature})* ` `[P]`?`" → "`### - [ ] Cycle {N}: {title} ` `[P]`?`" (the `*({Foundation|Feature})*` type annotation removed); the pattern gains a `- **Covers**: spec/plan IDs this case covers` line inside the `**TEST:**` block and a note that a card may carry more than one `**TEST:**` block (the bundle) with Cycle 1 of a new path a walking skeleton.
  - Fields-to-Extract row "`Type + [P] | Foundation cards run sequentially, first; [P] marks parallel-eligible feature cards`" → "`[P] | Marks a parallel-eligible card — derived from dependencies, not a card type`".
  - Fields-to-Extract row "`Acceptance criteria | Cited IDs — resolve against the spec/plan artifacts; these bound the decomposition`" removed → merged into a re-keyed "`**TEST:** blocks`" row: "The card's named test-case bundle — the expected behaviour the cycle must ultimately demonstrate green; these bound the decomposition. Each block's `Covers` line cites the spec/plan IDs it verifies. Parse only to know what the cycle must prove; running the cases is `testing-end-user`'s work".
  - Full prior text: git history at v0.74.x.
- **Kept deliberately:** the **Current-Cycle Identification** rule (first unchecked card in file order, all `Depends on` checked; flip at step 6) and the **Quality Gates Pattern** section — execution discipline untouched (D4: the skill consumes the cards, does not re-decide slicing). The Checkbox / Stories / Depends on / Brownfield-exposure extraction rows kept, re-keyed only where they named the type.
- **Consumers assessed:** this reference is loaded by `executing-tdd-cycle/SKILL.md` only (build-time card reader). Upstream owner `patterns-vertical-tdd` re-keyed same wave (card shape); `tasks-template.md` re-keyed same wave (the card shape parsed here); `testing-end-user` runs the `**TEST:**` cases (grammar unchanged, ignores `Covers`). No other consumer reads this file.

## [v0.64.0] Guardrails body + slim description (guardrails-vs-detail Wave 2 editorial cut)
- **Disposition:** superseded → Wave 2 editorial guardrails cut (D4 cut line).
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail Wave 2, `DECISIONS.md`
  2026-08-11 build row Wave 2 residual + user rulings 2026-08-10/11; method warrant: benchmark
  verdict `.mochiko/benchmarks/guardrails-vs-detail/report/final-verdict.md`).
- **Content (faithfully compressed):** body 10,151 → 9,676 chars (−5%); description 1,082 → 498
  chars (−54%). Body cut: the **When to Use** section deleted whole (five bullets restating the
  description's invocation conditions — executing a cycle card through decompose→r/g/r, reworking
  failed tasks, fixing a reported failure, the brownfield-exposure co-fire, writing the
  cycle-report; each obligation survives in the Overview, the Core Process sequence, the
  Reworking / Fixing subsections, and the Green-Phase `brownfield-integration` co-fire steps).
  Description cut: the parenthetical red/green/refactor step spell-out and several SHOULD trigger
  phrases compressed; the MUST clause, core trigger, the decomposition-disclosed obligation, and
  the `patterns-vertical-tdd` design-time/runtime sibling distinction kept. Verbatim homes: git
  history of this file (pre-v0.64.0).
- **Old description (verbatim):**
  > This skill MUST be invoked when executing a cycle card at runtime — turning one card from `.mochiko/specs/<feature>/tasks.md` into working code by decomposing the card into concrete tasks (build-time, code in view), driving each task through the red→green→refactor execution sequence (write the failing test, run it, confirm it fails for the right reason, implement the minimum to pass, refactor only this cycle's code), flipping the card's checkbox, and writing the `cycle-report.md` with the decomposition disclosed. SHOULD also invoke when "execute cycle", "implement the cycle card", or "write the cycle report" is the work at hand; when reworking the specific tasks reported as failing (targeted, test-first rework); when reproducing a reported failure with a failing test before fixing it; or when the card's brownfield exposure names existing code. This is the runtime EXECUTION of cycles — decomposition included. Deciding WHAT the cycles are (the slicing, the cards, the TEST gates) is design-time work owned by `mochiko:patterns-vertical-tdd`, upstream and not this skill.
- **Kept deliberately:** the guardrails keep-set — the Overview + letter/spirit epigraph, When
  NOT to Use (the vertical-tdd / verifier / lead / loop-state boundaries), the entire Core
  Process (Read → Decompose → Red → Green → Refactor → Flip → Write, incl. the pre-code-ladder
  step), Progress Tracking, Reworking Specific Failed Tasks, Fixing a Reported Failure, Red
  Flags, the Common Mistakes table, and the Reference Files pointers.
- **MANDATORY KEPT reconciliation:** the [v0.44.0] KEPT entry protects the envelope's register +
  prose-on-clean check, but that content lives in `references/CYCLE-REPORT-FORMAT.md`, NOT the
  SKILL body — untouched by this body cut. The [v0.49.0] and [v0.53.0] supersessions KEPT the
  cycle-boundary restriction, red/green/refactor strict order, rework-only-failed-tasks, fix-pass
  scoping, the verifier boundary, and the report self-disclosure framing — all live in Core
  Process / When-NOT-to-Use / Progress Tracking, none in the deleted When-to-Use. Progress
  Tracking (the machine-first report obligation) was deliberately kept, not cut. No prior KEPT or
  protected line is touched.
- **Consumers assessed:** staff-engineer (mounts it) · implement (binds it) · qa-engineer /
  review-code-minimalism (read the disclosed decomposition) · brownfield-integration (co-fires) ·
  patterns-vertical-tdd, patterns-code-minimalism (cross-reference) · mochiko router. None links
  the removed When-to-Use bullets or a description clause. Contract intact.

## [v0.53.0] Cycle-report consumer line: lead-only → lead + verification seat
- **Disposition:** superseded → `references/CYCLE-REPORT-FORMAT.md`'s widened consumer line: the verification seat's code-minimalism lens (`mochiko:review-code-minimalism`) now reads the disclosed decomposition and its rung claims alongside the cycle's diff.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-05 "Ponytail code-minimalism ruled (PT-D1–D10)", record `.mochiko/brainstorms/ponytail-concepts-integration/record.md`, D8).
- **Content (verbatim, the superseded consumer statement):**
  ```
  Consumers: the lead's checkpoint
  verdict (the frontmatter) and, on failure, the debugging trail (the failure narrative).
  ```
- **Why the ruling retired it:** D8 — the qa lens grades produced code against the builder's ladder claims; it needs both the diff and the disclosure, and lead-relay would make doctrine out of relay quality. The `decomposition` rows gained a `rung:` note in the same edit (pure addition).
- **Kept deliberately:** the self-disclosure framing ("not a verdict"), the lead's verdict ownership, the verifier-grades-independently line — all verbatim.
- **Consumers assessed:** implement (lens wiring landed same wave) · qa-engineer + `review-code-minimalism` (the new read edge's owner) · staff-engineer (discloses rungs, unaffected as author).

## [v0.49.0] Decomposition restriction removed — builder decomposes the card (step 2)
- **Disposition:** superseded → the same skill's new "Decompose the Card" step (build-time tasks + file paths, code in view, disclosed in `cycle-report.md`'s new `decomposition` field); `references/TASK-PARSING.md` rewritten from `TN.X` task-line parsing to cycle-card reading
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-02 "Task layer de-granularized + slice dissolved into specify (D1–D9)"; record `.mochiko/brainstorms/plan-task-granularity/record.md`, D2.1)
- **Content:** the restriction, both homes — Overview "it does not structure the cycles or decide when they run" (structuring half) and When-NOT-to-Use bullet 1 "Structuring the cycles — identifying the vertical slices, ordering a cycle's tasks test-first, or authoring the `tasks.md` skeleton — is design-time work owned by `patterns-vertical-tdd` … it does not create, split, or reorder tasks." · TASK-PARSING.md's task grammar (`- [ ] **T{N}.{X}**:` pattern, ID-prefix cycle identification, backtick path extraction, `[EXTEND]`/`[MODIFY]` marker table, multi-line sub-bullets, Checkpoint pattern) · "Mark each task `[x]` in `tasks.md` immediately after completing it" (now: flip the card at cycle close). Full text: git history at v0.48.0.
- **Kept deliberately:** the cycle-boundary restriction (does not add/remove/re-scope *cycles*) — decomposition is unlocked, slicing is not · red/green/refactor strict order · rework-only-failed-tasks · fix-pass scoping · verifier boundary (TEST gates + quality gates never this skill's).
- **Consumers assessed:** staff-engineer (persona wording re-keyed) · implement · CYCLE-REPORT-FORMAT.md (decomposition field added same wave) · router.

## [v0.44.0] Failure-narrative trigger: "or any task failed" narrowed to execution failures
- **Disposition:** superseded → `references/CYCLE-REPORT-FORMAT.md`'s corrected trigger, *"or a
  task failed in execution"*, plus a carve-out paragraph under the same section naming the
  verifier-owned case explicitly.
- **Tier failed:** n/a — supersession by ruling (`verbosity-caveman-ops-separation` D4 part 1,
  the F59 clause fix; `DECISIONS.md` 2026-08-01 row above).
- **Content (verbatim, the whole superseded line):**
  ```
  ### Failure narrative *(mandatory when `status` is `fail` or `blocked`, or any task failed)*
  ```
- **Why the ruling retired it:** every cycle report in the `author-navigate` driver run carried a
  non-empty `failed_tasks:` naming one verifier-owned `**TEST:**` gate the producer must not
  execute (record F59). Under *"or any task failed"* the format itself owed a narrative on a
  passing cycle — a producer following the shipped text arrived at prose correctly, and 8/8
  passing reports carried it (F58, 79.9% of report bytes). The text was the defect, not the
  author.
- **Kept deliberately:** the other two trigger conditions (`status` `fail` / `blocked`) verbatim;
  the section's full-detail body paragraph; and the `failed_tasks:` disclosure itself — the
  verifier-owned task is still listed with its one-line reason. Only the narrative obligation
  lifts.
- **Consumers assessed:** `skills/executing-tdd-cycle/SKILL.md` is the reference's only consumer
  (`:74`, `:81`, `:142`) and needs no edit — `:81` already reads *"a `Failure narrative` (full
  detail) whenever the cycle failed or was blocked"*, i.e. the corrected trigger, and `:79`
  already reads *"a clean passing cycle needs no prose"*. The defect was localized to the
  reference. `templates/report-format.md` rule 2 never carried a task-level clause, so it is
  unaffected.

## [v0.44.0] KEPT: the envelope's register + prose-on-clean check, restated in this payload home
- **Tier-2 evidence:** a deliberate exception to the no-restatement rule, recorded here so a
  later minimalism wave does not read it as Tier-1 duplication and relocate it. Ground: record
  F72 — the driver run's report prose was authored against this payload home, one hop *below*
  `templates/report-format.md`, where the stricter frontmatter-only rule already failed to reach
  (F58). D4's S2 fold names the restatement in both payload homes as a host of the check, so the
  binding is ruled, not stylistic. Cut it only with a ruling that also re-homes the check.

## [v0.25.0] Task-extraction preview list (net −7 lines)
- **Disposition:** relocated → `references/TASK-PARSING.md` (the declared parsing home, pointed at two lines above; pointer now names "the per-task fields to extract")
- **Tier failed:** 1 (preview copy of the reference's field list)
- **Content:** ID / description / file-paths / markers / sub-bullets extraction list
- **Consumers assessed:** 6 consumer files checked at wave open; none reference the list

## [v0.25.0] Common Mistakes densified: 4 subsections → 4-row table (net −17 lines)
- **Disposition:** compressed in place (densification, zero deletions — every mistake/failure/fix survives as a row)
- **Tier failed:** n/a — form only
- **Content:** tests-after-implementation, full-cycle re-implementation, refactor scope creep, failure-reason verification
- **Consumers assessed:** none reference the subsection headings

## [v0.25.0] Aphorism consequence-anchored (R4b rider, net 0 lines)
- **Disposition:** reordered in place — the existing consequence sentence ("TDD discipline exists to catch failures before they compound…") moved from mid-Overview to directly under the aphorism
- **Tier failed:** n/a — rider execution, not a strip
- **Content:** unchanged text, relocated within the file
- **Consumers assessed:** n/a

## [v0.22.0] Cycle-report prose sections (What Was Done · Decisions Made · Notes for Next Cycle)
- **Disposition:** deleted from `references/CYCLE-REPORT-FORMAT.md` (What Was Done, Decisions Made) · deleted per the epic's Q6 ruling, **no optional-field resurrection** (Notes for Next Cycle)
- **Tier failed:** consumption evidence (epic F-c): the user never reads cycle reports (Q4); the next cycle never reads the file (15/15 kinako reports authored the section, 0 back-references — the standing seat carries the context); fix/retry consumes a lead-relayed failure list
- **Content:** `### What Was Done` — narrative of what was implemented "in enough detail for the lead and the next cycle" (restated tasks.md); `### Decisions Made` — technology/pattern choices + trade-offs narrative; `### Notes for Next Cycle` — files/interfaces affecting future cycles, patterns established, potential conflicts, improvement opportunities. Replacements: non-obvious decisions + difficulties/blockers → the conditional `## Notes of note` block; deviations → the `deviations:` frontmatter list; failed cycles keep a mandatory `## Failure narrative` (S8). Improvement-opportunity noting (refactor-discipline pressure valve) retargeted to Notes of note (SKILL.md Common Mistakes).
- **Re-add trigger:** a dogfood run where the lead's checkpoint verdict or a fix pass demonstrably starved for the dropped narrative on a *passing* cycle (evidence-gated, marked override).
