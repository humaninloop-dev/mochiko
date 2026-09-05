# Strip notes — `skills/review-code-minimalism`

Entry formats: `strips/README.md`. First strip for this skill. Wave context: guardrails-vs-detail
Wave 2 (editorial extension of the D4 cut line to the untested primitives). This skill was authored
recently and terse (the Ponytail code-minimalism build, PT-D1–D10); the honest D4 yield is small —
its body is almost entirely non-waivable floors + output contract + the `patterns-code-minimalism`
standard binding, none of which the cut line removes.

<!-- Wave context: wave 5 of the CLI schema-delivery build (v0.106.0) — the review family
converts: each member's rules are rendered at fire by `mochiko-cli` from the migration log the
plugin carries at `plugins/mochiko/migrations/`, and the skill reads no schema file. Ruling for
every [v0.106.0] entry below: `.mochiko/brainstorms/cli-schema-delivery/record.md` D3 as amended
(the skill-side form — `!` runs in `SKILL.md` and at subagent preload), D7 (the `PreToolUse`
`Skill` limb), D9 (families in the arc's order), and D10 clause 6 (the budgeted quantity re-keys
to body + rendered output), with the wave-open rulings in that session's `wave5-plan.md` and the
`DECISIONS.md` 2026-09-04 row. Pre-edit verbatim text:
`git show 7d098b9:plugins/mochiko/skills/review-code-minimalism/SKILL.md`. -->

## [v0.106.0] the Rules block — raw schema Read superseded by CLI delivery

- **Disposition:** superseded → `## Rules — delivered by mochiko-cli`: the positive-confirmation
  halt clause plus seven `!` lines, one per rendered block (the preamble and the six sections),
  and the read-back sentence.
- **Tier failed:** n/a — supersession by ruling (`cli-schema-delivery` D3 as amended, the
  skill-side form)
- **Content:** verbatim —

  ```
  ## Rules — load the schema first

  Your first action, before any procedure step: **Read `schema.yaml` (this skill's own
  directory) and `../../schemas/skill-review-common.yaml` raw, in full, in the same first
  action.** The schema is the source of truth for this skill's binding rules, nested in six
  sections, each addressable by its section ID: `review-code-minimalism.sec.independence` ·
  `review-code-minimalism.sec.scope` · `review-code-minimalism.sec.inputs` ·
  `review-code-minimalism.sec.verdict` · `review-code-minimalism.sec.output` ·
  `review-code-minimalism.sec.reserved`. Interpret it live: a rule's `kind:` names what it
  is, and an absent `kind:` reads `constraint`; a `class: floor` rule is always read and
  always delivered; a `pointer:` rule binds you to that skill's procedure, referenced never
  restated; labels come from `plugins/mochiko/schemas/skill-labels.yaml`. A rule carrying
  `extends: review-common.<slug>` inherits text/labels/pointer from
  `skill-review-common.yaml` only — `class` and every absence-meaningful field are local —
  and the stub's `review-code-minimalism.*` ID stays the citable ID. The floor pin:
  the 3 rules of `class: floor` are non-waivable. Before the first procedure step, state the floor
  count back — a skipped or partial read leaves that count blank: halt and surface it, and
  halt likewise if the schema's `class: floor` count disagrees with the pin.
  ```

- **Kept deliberately:** everything outside this section, byte for byte — the title, the opening
  paragraph, and every procedural section. The block's substance survives in the render: the six
  section IDs are now the six `--section` arguments, and the interpretation grammar the block
  taught is printed as the preamble's `legend` with every fire. The `extends:` stub resolution
  and the family common-file co-Read are discharged by the render, which resolves every stub
  before the model sees it.
- **Consumers assessed:** the family common file
  `plugins/mochiko/schemas/skill-review-common.yaml` is unchanged and still bound by every
  unconverted consumer; nothing shared leaves. The block was this skill's own text.

## [v0.106.0] the hand-pinned `class: floor` count

- **Disposition:** superseded → the CLI-printed pin. The count is the `- class: floor · N rules`
  line under `pins` in the preamble block, and the `floors:` line beneath it lists the ids; the
  read-back sentence now cites both rather than carrying a number of its own.
- **Tier failed:** n/a — supersession by ruling (`cli-schema-delivery` D3, "the counts are computed
  and printed by the CLI, never hand-pinned"; the wave-4 re-key ruling adding the `floors:` index)
- **Content:** verbatim —

  ```
  The floor pin:
  the 3 rules of `class: floor` are non-waivable. Before the first procedure step, state the floor
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

## [v0.100.0] Schema conversion — census-row → minted-ID map (skill-content-schema wave)

Ruling for every entry below: skill-content-schema D3 (boundary) / D8/C4 (protected
transfers), `DECISIONS.md` 2026-09-01 row; census:
`.mochiko/brainstorms/skill-content-schema/census.md` §B (RCM). Schema home:
`plugins/mochiko/skills/review-code-minimalism/schema.yaml`. Minted IDs carry the
`review-code-minimalism.` prefix (omitted below). Map — census row → minted ID:
1 `standard-binding` · 2 `per-cycle-lens-only` · 3 `not-general-code-review` ·
4 `not-test-gates` · 5 `not-design-time` · 6 `diff-and-report-both-read` ·
7 `missing-rung-note-is-finding` · 8 `codebase-read-rungs-2-3-5` · 9 `floor-line-check` ·
10 `minimalism-entries-form` · 11 `advisory-never-cycle-failing` ·
12 `dispute-at-checkpoint-only` · C3 mint (no census §B row; census §C ×8)
`author-grader` — no body clause leaves for it (the independence wording lives in the
kept Overview identity and the untouched `description:`), so it carries no relocation
entry; deviation reported to the wave lead · C4 mint (no census §B row; census §C ×5)
`verdict-is-input` — its body clause is the entry below. The v0.64.0 recorded floor-line
SKIP stands as the C1 keep-distinct allowlist edge (RCM binds no
`review-common.evidence-floor` stub; its persistence obligation lives in
`minimalism-entries-form`). The Quality Checklist block is removed with this conversion —
every line restated a schema rule; the twins are quoted in the row entries below.
Accounting (V1 fix round, post-fix measurements): body 3,884 → 2,971 (−913) + schema
5,478 = payload 8,449; the delta over the pre-conversion body is structural overhead
(IDs, keys, section scaffolding, reading grammar) — no content growth claimed (the
restored diff-location mechanic and the carried rework-now-or-carry limb are
pre-conversion content, relocated not grown).

## [v0.100.0] Inputs section framing — the two items without obligations of their own (V1 fix round, RCM-1)
- **Disposition:** superseded → item 1's diff-location mechanic RESTORED to the body (Procedure intro line: "Locate the diff first: files created/modified per the cycle report locate it; `git diff` over those paths"); item 1's and item 3's framing otherwise carried by schema rules `review-code-minimalism.diff-and-report-both-read` and `review-code-minimalism.codebase-read-rungs-2-3-5`.
- **Tier failed:** n/a — supersession by ruling (skill-content-schema D3; `DECISIONS.md` 2026-09-01 row; V1 audit finding RCM-1).
- **Content (verbatim, the two Inputs items):** "1. **The cycle's git diff** — what was actually built (files created/modified per the cycle report locate it; `git diff` over those paths)." · "3. **The codebase around the diff** — obligated for rungs 2/3/5 (below)."
- **Kept deliberately:** the diff-location mechanic, now in the Procedure intro — teaching prose per D3.
- **Consumers assessed:** none (skill-local framing).

## [v0.100.0] Codebase-read floor — protection transfers (census RCM-8; PT-D1–D10 protected)
- **Disposition:** superseded — protection transfers to schema rule `review-code-minimalism.codebase-read-rungs-2-3-5` (class: floor), per skill-content-schema D8/C4; provenance sidecar carries the protected status. The per-rung mechanics (greps · stdlib · manifest) stay in Procedure step 3 per D3.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 skill-content-schema row).
- **Content:** "**3. Never take reuse claims on trust — the codebase-read obligation.** For rungs 2 (reuse), 3 (stdlib), and 5 (installed dependency), diff + disclosure alone cannot verify \"should have reused\"" + checklist twin "Rung-2/3/5 claims verified by codebase read (greps / stdlib / manifest), not trusted"
- **Kept deliberately:** Procedure step 3's three verification mechanics, body prose.
- **Consumers assessed:** `agents/qa-engineer.md` mounts the skill — obligation survives at the same strength, composition intact.

## [v0.100.0] Advisory posture floor — protection transfers (census RCM-11; PT-D1–D10 protected)
- **Disposition:** superseded — protection transfers to `review-code-minimalism.advisory-never-cycle-failing` (class: floor), per D8/C4.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 row).
- **Content:** "A `minimalism:` finding **never fails a cycle** the way a `**TEST:**` gate does." + checklist twin "Findings emitted as advisory `minimalism:` entries — no cycle failed on this lens alone"
- **Consumers assessed:** `commands/implement.md` dispatches the lens — advisory posture survives verbatim in substance.

## [v0.100.0] Design-phase carve-out — protection transfers (census RCM-5; v0.91.0 ruled wording)
- **Disposition:** superseded — protection transfers to `review-code-minimalism.not-design-time` (kind: routing), per D8/C4; the v0.91.0 ruled wording carried verbatim in substance.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 row).
- **Content:** "**Design-time artifact review** — the design-phase and spec reviewers own those surfaces; the design-time sibling of this lens is the rung-honesty grade in `mochiko:review-plan-artifacts` against `mochiko:patterns-plan-minimalism` (same posture, design-time altitude)"
- **Kept deliberately:** both skill slugs unchanged; pointers resolve.
- **Consumers assessed:** `mochiko:review-plan-artifacts` carries the mirror line — untouched by this seat (P3's member).

## [v0.100.0] Standard binding relocated (census RCM-1)
- **Disposition:** relocated → `plugins/mochiko/skills/review-code-minimalism/schema.yaml` `review-code-minimalism.standard-binding` (pointer: `mochiko:patterns-code-minimalism`)
- **Tier failed:** n/a — supersession by ruling (D3; `DECISIONS.md` 2026-09-01 row).
- **Content:** "The grading standard is `mochiko:patterns-code-minimalism` — this skill carries the *grading procedure*, never a copy of the ladder."

## [v0.100.0] Per-cycle placement relocated (census RCM-2)
- **Disposition:** relocated → schema.yaml `review-code-minimalism.per-cycle-lens-only`
- **Tier failed:** n/a — supersession by ruling (D3; `DECISIONS.md` 2026-09-01 row).
- **Content:** "It runs inside the per-cycle verification the verification seat already performs; no separate stage, no final-pass sweep"

## [v0.100.0] General-code-review carve-out relocated (census RCM-3)
- **Disposition:** relocated → schema.yaml `review-code-minimalism.not-general-code-review`
- **Tier failed:** n/a — supersession by ruling (D3; `DECISIONS.md` 2026-09-01 row).
- **Content:** "**General code review** — naming, patterns, framework choices, correctness beyond tests: out of scope; this lens grades ladder discipline only" + checklist twin "No general-code-review findings smuggled in (naming, style, patterns)"

## [v0.100.0] TEST-gate carve-out relocated (census RCM-4)
- **Disposition:** relocated → schema.yaml `review-code-minimalism.not-test-gates` (pointer: `mochiko:testing-end-user`)
- **Tier failed:** n/a — supersession by ruling (D3; `DECISIONS.md` 2026-09-01 row).
- **Content:** "**Executing `**TEST:**` gates or quality gates** — `mochiko:testing-end-user`, the same seat's other craft"

## [v0.100.0] Both-inputs-read rule relocated (census RCM-6)
- **Disposition:** relocated → schema.yaml `review-code-minimalism.diff-and-report-both-read`
- **Tier failed:** n/a — supersession by ruling (D3; `DECISIONS.md` 2026-09-01 row).
- **Content:** "Read the report file itself, never a relay of it." + checklist twin "Diff AND cycle report both read — never one without the other"

## [v0.100.0] Missing-rung-note rule relocated (census RCM-7)
- **Disposition:** relocated → schema.yaml `review-code-minimalism.missing-rung-note-is-finding`
- **Tier failed:** n/a — supersession by ruling (D3; `DECISIONS.md` 2026-09-01 row).
- **Content:** "A missing rung note is itself a finding (the disclosure surface exists so this lens can grade it …)"

## [v0.100.0] Floor-line check relocated (census RCM-9)
- **Disposition:** relocated → schema.yaml `review-code-minimalism.floor-line-check`
- **Tier failed:** n/a — supersession by ruling (D3; `DECISIONS.md` 2026-09-01 row).
- **Content:** "**4. Check the floor line.** Code cut to reach a cheaper rung that a floor obligation or accessibility required is a finding — the standard's floor line is part of the standard."

## [v0.100.0] Output contract relocated (census RCM-10)
- **Disposition:** relocated → schema.yaml `review-code-minimalism.minimalism-entries-form`
- **Tier failed:** n/a — supersession by ruling (D3; `DECISIONS.md` 2026-09-01 row).
- **Content:** "One `minimalism:` entry per finding in the verification report (format: `mochiko:testing-end-user`'s report templates): task ID, claimed rung, observed rung, one-line evidence (the grep hit, the stdlib call, the manifest entry). No findings → an empty block; never narrate a clean grade." + checklist twin "Every finding cites task ID + evidence, one line each"

## [v0.100.0] Dispute reservation relocated (census RCM-12)
- **Disposition:** relocated → schema.yaml `review-code-minimalism.dispute-at-checkpoint-only` (kind: reservation)
- **Tier failed:** n/a — supersession by ruling (D3; `DECISIONS.md` 2026-09-01 row).
- **Content:** "A builder-vs-reviewer rung dispute escalates to the user only at the checkpoint, never as a mid-cycle stop."

## [v0.100.0] Findings-ride-to-checkpoint COMPRESSED into C4 stub (no census §B row — mint reported; relabeled at the V1 fix round, RCM-3)
- **Disposition:** superseded → schema.yaml `review-code-minimalism.verdict-is-input` (`extends: review-common.verdict-is-input`, class: must, kind: reservation) carries the input-not-clearing core; the dropped limb "the lead decides rework-now or carry" is CARRIED into `review-code-minimalism.dispute-at-checkpoint-only`'s text (V1 fix round). Census §B carried no RCM row; minted per census §C (C4 ×5), deviation reported to the wave lead, never silent.
- **Tier failed:** n/a — supersession by ruling (D3/D5 + near-dup R2; `DECISIONS.md` 2026-09-01 row).
- **Content:** "Findings ride the verification report to the lead's checkpoint verdict; the lead decides rework-now or carry."
- **Kept deliberately:** the rework-now-or-carry limb, in `dispute-at-checkpoint-only`.
- **Consumers assessed:** none (skill-local clause).

## [v0.91.0] When-NOT-to-Use carve-out re-keyed: "plan/spec reviewers", "the plan-time sibling" → design — plan-stage retirement D1/D5

- **Disposition:** superseded → "the design-phase and spec reviewers own those surfaces; the
  design-time sibling of this lens…".
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` D1 and D5; wording ruled by the wave
  lead 2026-08-26).
- **Content (superseded text, verbatim):**

  ```
  - **Design-time artifact review** — plan/spec reviewers own those surfaces; the plan-time sibling of this lens is the rung-honesty grade in `mochiko:review-plan-artifacts` against `mochiko:patterns-plan-minimalism` (same posture, design-time altitude)
  ```

- **Kept deliberately:** the carve-out's whole point — this lens does **not** review design-time
  artifacts, those surfaces belong to their own reviewers, and the sibling relationship runs to
  the rung-honesty grade in `mochiko:review-plan-artifacts`. Both skill slugs are unchanged, so
  both pointers still resolve. Note the bullet's own label was already "**Design-time artifact
  review**" and its closing parenthetical already read "same posture, design-time altitude" — the
  ruled wording brings the body of the bullet into line with a heading that was correct all along.
- **Budget:** body 3,866 → **3,884** against the 4,612 budget; description unchanged at 492
  against 615. Both inside.
- **Consumers assessed:** `mochiko:patterns-code-minimalism` carried the parallel sibling line,
  re-keyed identically in the same wave; `mochiko:review-plan-artifacts` was re-scoped to the
  design-phase package this wave, so the pointer's target now matches its description.

## [v0.64.0] Guardrails Wave 2 — slim description + "When to Use" deleted (floor line skipped — equivalent present)
- **Disposition:** superseded → the guardrails-vs-detail Wave 2 editorial cut (D4 cut line): the "When to Use" list restating the description is deleted; description slimmed. No other body change.
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail Wave 2, `DECISIONS.md` 2026-08-11 build row [its Wave 2 residual authorization] + user rulings 2026-08-10/11; method warrant: benchmark verdict `.mochiko/benchmarks/guardrails-vs-detail/report/final-verdict.md` — guardrails held across all four skill natures).
- **Content (faithfully compressed).** Description 1,102 → 492 chars (−55%). Body 3,879 → 3,689 chars (−190, −5%; no additions). Section removed:
  - **## When to Use** (two bullets — "Grading a cycle's diff + disclosed decomposition during per-cycle verification" · "Auditing a rung claim … against the actual codebase") — restated the description's MUST trigger and the Procedure's rung-2/3/5 codebase-read obligation. Both survive in the slim description and in Procedure steps 2–3.
  - Description cut: the rung glosses ("2 (reuse), 3 (stdlib), 5 (installed dependency)"), the explicit grep/manifest mechanics, the "over-engineering in this diff / should this have been reused / rung claim / code-shape audit / minimalism findings" SHOULD-trigger phrases, and the mounted-on-the-verification-seat-never-the-producer clause compressed. Kept in the slim description: the MUST trigger, the pre-code-ladder + code-minimalism-lens + verification-seat framing, read-diff-AND-`cycle-report.md`, grade-against-`mochiko:patterns-code-minimalism`-the-standard-never-restated-here, the rungs-2/3/5 codebase-read obligation, the ADVISORY-never-a-cycle-failing-gate posture, and the minimalism-lens-ONLY scope.
  - **Old description (verbatim):** "This skill MUST be invoked when independently grading a cycle's produced code against the pre-code ladder — the per-cycle code-minimalism lens run by the verification seat during the verification it already performs: read the cycle's git diff AND cycle-report.md (the disclosed decomposition and its rung claims), grade each claim against mochiko:patterns-code-minimalism (the standard — never restated here), and emit a minimalism: findings block in the verification report. Rungs 2 (reuse), 3 (stdlib), and 5 (installed dependency) carry a codebase-read obligation — targeted greps for existing helpers and a dependency-manifest check, never diff + disclosure alone. SHOULD also invoke when the work involves \"over-engineering in this diff\", \"should this have been reused\", \"rung claim\", \"code-shape audit\", or \"minimalism findings\". Findings are ADVISORY to the lead's checkpoint verdict — never a cycle-failing gate. Mounted on the verification seat only, never on the producer. Scope is the minimalism lens ONLY: general code review (naming, patterns, correctness beyond tests) stays out of scope."
  - Verbatim homes for the removed text: git history of this SKILL.md (pre-v0.64.0); archive branch `worktree-brainstorm-validator-scope`.
- **Floor line — SKIPPED (equivalent already present).** The Wave 2 review-evidence floor line ("The independent review leaves its verdict and per-finding dispositions in the reviewed artifacts themselves — review evidence that lives only in conversation is a floor violation") was NOT added: this skill produces no verdict (its findings are advisory) and already carries the equivalent persistence obligation — Procedure step 5 ("One `minimalism:` entry per finding in the verification report … never narrate a clean grade") plus the Quality Checklist item "Findings emitted as advisory `minimalism:` entries". Review evidence living only in conversation is already precluded. Per the Wave 2 instruction, the skip is recorded here.
- **Kept deliberately (the guardrails keep-set):** the epigraph floor ("Diff shows what was written … neither alone shows what should never have been written"); When NOT to Use (scope boundary — general code review out · TEST/quality gates to `testing-end-user` · design-time to plan reviewers); Inputs (the grading substrate contract, incl. "read the report file itself, never a relay"); the full Procedure (steps 1–5 — the missing-rung-note-is-a-finding rule, the grade-against-the-standard binding, the **rung-2/3/5 codebase-read obligation** [the skill's central non-waivable floor], the floor-line/accessibility check, the `minimalism:` output contract); Verdict semantics — advisory; Quality Checklist.
- **MANDATORY KEPT reconciliation:** no prior strip file existed and no `KEPT:` / protected / `DECISIONS.md`-traceable line is in scope of this cut. The Ponytail-ruled codebase-read obligation and advisory-only posture (PT-D1–D10) are the skill's core and are fully retained.
- **Consumers assessed:** command — `plugins/mochiko/commands/implement.md` references it (the implement command dispatches the verification seat's minimalism lens); the retained MUST trigger, codebase-read obligation, and advisory posture leave that dispatch intact. Agent — `plugins/mochiko/agents/qa-engineer.md` mounts it (the verification seat); the kept Procedure, Inputs, and Verdict-semantics leave that composition intact. Contract intact.
