# Strip notes — `skills/patterns-code-minimalism/`

Entry formats: `strips/README.md`. First strip of this skill. Wave context: [v0.64.0] entry —
guardrails-vs-detail **Wave 2** (editorial extension of the D4 cut line to the untested
primitives; design: `.mochiko/brainstorms/validator-scope-and-verbosity/record.md`; build plan:
`.mochiko/benchmarks/guardrails-vs-detail/report/build-plan.md`, Wave 2 sketch).

## [v0.91.0] Two sibling references re-keyed from plan-time to design-time — plan-stage retirement D1

- **Disposition:** superseded → "design-time sibling" and "the design-time discipline"; the
  adopt-first constraint arrives design-committed rather than plan-committed.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` D1; wording ruled by the wave lead
  2026-08-26).
- **Content (superseded fragments, verbatim — two sites):**

  1. Overview, the sibling pointer:

     ```
     this file (`mochiko:review-code-minimalism`). Its plan-time sibling
     `mochiko:patterns-plan-minimalism` runs the same discipline over the plan's design elements
     ```
  2. Rung 5, the adopt-first carve-out:

     ```
        plan-committed adopt-first choice reaches these cards as a binding constraint, not a rung
        to re-open — the plan-time discipline is `mochiko:patterns-adopt-first`.
     ```

- **Kept deliberately:** the two-ladders-one-discipline framing and the build-time/design-time
  altitude split it states; the rung-5 carve-out in full — a committed adopt-first choice reaches
  the cards as a **binding constraint, not a rung to re-open**. That carve-out remains exactly
  true under D1 mechanic (d): mechanic (d) opens a build-time firing site only for a commodity
  need the design phase **never ruled**, which is not a re-opening, and it routes to the user
  rather than to this ladder.
- **Budget:** body 3,794 → **3,795** against the 4,319 budget; description unchanged at 564
  against 705. Both inside.
- **Consumers assessed:** `mochiko:patterns-plan-minimalism` (re-scoped to the design phase this
  wave — the sibling framing now matches on both sides); `mochiko:patterns-adopt-first` (gained
  the build-time gate this wave, which this carve-out is consistent with, as noted above);
  `mochiko:review-code-minimalism` carried the parallel sibling line, re-keyed in the same wave.

## [v0.64.0] RETURNED: existing-code-slimming trigger added to the description
- **Evidence:** the Wave 2 M1 fire-rate probe (14-scenario blind routing spot-check, lead
  dispatch 2026-08-11) found S12 ("slim this function down — grew three abstraction layers")
  a real scope gap PRE-DATING the wave: the MUST clause is scoped to pre-code card
  decomposition, and no description gave an active refactor request a clean home (ambiguous
  against `review-code-minimalism`, which grades a diff). Minimal fix, one clause appended to
  the SHOULD list: "or when slimming existing code that grew unneeded abstraction layers."
  Description 497 → 564 chars (cap 1,536). Not a re-add of cut content — a new trigger the
  probe demanded. User-ruled 2026-08-11.

## [v0.64.0] Guardrails cut — When-to-Use removed, ladder/floor/checklist kept; slim description

- **Disposition:** superseded → Wave 2 editorial guardrails cut (D4 cut line — When-to-Use bullets restate the description).
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail Wave 2, `DECISIONS.md` 2026-08-11 build row Wave 2 residual + user rulings 2026-08-10/11; method warrant: benchmark verdict `.mochiko/benchmarks/guardrails-vs-detail/report/final-verdict.md`).
- **Content (faithfully compressed — section-level inventory; body 3,710 → 3,455 chars, −255, −7%; description 1,023 → 497 chars):**
  - **Removed whole:** `## When to Use` — the three-bullet list ("Decomposing a cycle card into tasks — every prospective task gets a rung" · "Weighing write-new against reuse, stdlib, platform, or an installed dependency" · "Grading a diff's rung claims (the review skill cites this file as its standard)"). Each bullet restates the description's MUST/SHOULD triggers; no obligation lost (the decompose-time firing is stated in Overview + the description; the review-skill grading pointer survives in Overview).
  - Old description verbatim: "This skill MUST be invoked at build-time card decomposition, BEFORE any red-phase test is written — running the pre-code ladder over each prospective task: stop at the first rung that applies (does it need to exist at all · already in the codebase · standard library · native platform feature · installed dependency · one line · only then the minimum that works), with rung choices disclosed in the cycle report. SHOULD also invoke when the work involves \"should this code exist\", \"reuse before build\", \"stdlib first\", \"over-engineering\", \"YAGNI\", \"smallest change\", or when weighing writing new code against reusing, deleting, or skipping it. This is the single source of the ladder: producers apply it (mochiko:executing-tdd-cycle points here at the decompose step), and mochiko:review-code-minimalism grades against it. It governs the DECISION to write code — distinct from the green-phase \"minimum code to pass\" rule, which fires after that decision is made. No rung ever sacrifices a floor obligation or accessibility."
  - Verbatim removed text survives in: git history of the SKILL.md (pre-v0.64.0); archive branch `worktree-brainstorm-validator-scope`.
- **Kept deliberately (the guardrails keep-set):** the epigraph ("The cheapest code is the code never written."), `## Overview` (the decompose-step firing + the review-skill grading pointer), `## When NOT to Use` (the green-phase / topology-sizing / card-scope boundaries), `## Rung zero — read before you rank`, `## The Ladder` (the seven rungs — the core decision content), `## The floor — lazy, not negligent` (the non-waivable floor + accessibility), `## One intensity` (the no-mode-dial rule), `## Quality Checklist`. The description keeps the MUST trigger, the seven-rung gist, the top trigger phrases, and the review-skill / green-phase sibling distinctions.
- **Protected-content reconciliation (MANDATORY):** no prior strip file existed; grep of git history found no `KEPT:` / protected / `DECISIONS.md`-traceable line for this skill. The `## When to Use` list was never a protected survivor. Nothing silently dropped.
- **Consumers assessed:** `commands/implement.md`, `agents/staff-engineer.md`, `skills/executing-tdd-cycle/SKILL.md` (+ `references/CYCLE-REPORT-FORMAT.md`), `skills/review-code-minimalism/SKILL.md`, router `skills/mochiko/SKILL.md` — all reference the skill by name (`mochiko:patterns-code-minimalism`); none links a removed section anchor. The ladder-as-standard contract that `review-code-minimalism` grades against and `executing-tdd-cycle` points to is intact (`## The Ladder`, `## The floor` untouched).
