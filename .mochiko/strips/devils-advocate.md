# Strip notes — `agents/devils-advocate.md`

Entry formats: `strips/README.md`. Wave context: the command-waves' ≥3-consumer escalation queue
(D9's guard), ruled library-wide at the skill-succinctness pass's wave-1 open (R4b,
user-approved 2026-07-25; design: `.mochiko/brainstorms/skill-succinctness-strip/record.md`).
This was the sole remaining agent instance — the other five agents' Skills-Available paragraphs
were ruled in their command waves.

## [v0.91.0] Frontmatter `description:` review-target list: "plan packages" → "design packages" — plan-stage retirement D1

- **Disposition:** superseded → the two-word re-key inside the same `description:` value:
  "specifications, **design packages**, brainstorm records, governance intent".
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/brainstorms/plan-stage-utility/record.md` D1 — `/mochiko:plan` retired; the
  package this seat reviews is now the in-run design phase's output, per D5's re-scope of
  `review-plan-artifacts`; `DECISIONS.md` 2026-08-26 row).
- **Content (superseded, verbatim):** `plan packages`
- **Kept deliberately:** every other clause of the v0.86.0 description — the remit list's other
  three members, the blind gap-finding clause, the "what if" sentence, and the audit-round-scoped
  "severity-ranked document findings" tail — is byte-for-byte unchanged. This edit touches two
  words inside one list.
- **Budget:** description-class edit, pre-asserted against `.mochiko/memory/primitive-cost-budgets.md`.
  384 → **386** chars against the unchanged **395** budget (9 chars of headroom remain). No
  overage, so no justification is owed.
- **Wording note (deviation from the build brief, flagged for ruling):** the brief specified
  "design-phase packages" (392/395, also legal). "design packages" was written instead because
  `CLAUDE.md`'s keystone test — *a persona contains no trace of any workflow, decoupling by
  absence* — reads "design-phase" as naming a phase inside `/mochiko:implement`, which is exactly
  the workflow trace the axis forbids in a persona. The neutral form satisfies the ruling's intent
  (the review target is the design output, not a plan package), keeps the persona workflow-free,
  and leaves 3 more chars of budget headroom. Overrule and re-key to "design-phase packages" if
  the brief's literal wording is preferred; it fits the budget either way.
- **Consumers assessed:** frontmatter `description:` only — the delivery-time routing surface.
  No primitive quotes this string. The mounted `skills:` line is untouched, and
  `review-plan-artifacts` keeps its slug through its P3 re-scope, so the mount stays valid.

## [v0.91.0] Skills-Available bullet: "the plan analysis/design sets" → "the design output sets" — plan-stage retirement D1/D5

- **Disposition:** superseded → the re-keyed bullet: "**`mochiko:review-plan-artifacts`** —
  completeness review of the design output sets and the cycle cards (`tasks.md`)."
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/brainstorms/plan-stage-utility/record.md` D1 + D5 — the plan package ceases to
  exist and `review-plan-artifacts` re-scopes onto the in-run design phase's output;
  `DECISIONS.md` 2026-08-26 row).
- **Content (superseded, verbatim):** `completeness review of the plan analysis/design sets and the cycle cards (`tasks.md`).`
- **Kept deliberately:** the `skills:` frontmatter mount `review-plan-artifacts` is **unchanged**
  — the skill keeps its slug through its P3 re-scope, so the mount stays valid and re-keying it
  here would break it. The section's single-source framing line and the other four bullets are
  byte-for-byte untouched.
- **Found-not-assigned note:** this instance was **not** in the build brief's three assigned
  persona lines; it surfaced on a sweep of the same file after the `description:` edit and is
  the same defect from the same ruling. Left unfixed it would have put the persona's own
  Skills-Available bullet in contradiction with the `description:` corrected one entry above.
  Recorded here rather than fixed silently.
- **Consumers assessed:** persona body, one bullet; no primitive quotes it. Body-class edit —
  agent *bodies* carry no budget row in `.mochiko/memory/primitive-cost-budgets.md` (descriptions
  only), so no budget pre-assert applies; the change is net −4 chars regardless.

## [v0.86.0] Frontmatter `description:` re-aimed at the full remit — spec-only wording superseded

- **Disposition:** superseded → the rewritten `description:` value: "Adversarial reviewer who
  stress-tests finished artifacts — specifications, plan packages, brainstorm records,
  governance intent — and probes built systems in the blind gap-finding pass. Asks the hard
  "what if" questions that surface costly problems while they are still cheap to fix, and
  returns severity-ranked document findings with clarifying questions and a recommended
  verdict." (The `document` qualifier scopes the output-shape tail to document reviews — the
  runtime gap-finding pass splits findings by kind, never by severity, per
  `testing-gap-finding`; audit fix round 1.)
- **Tier failed:** n/a — supersession by ruling (user ruling 2026-08-26, persona hygiene
  pass; ADR `.mochiko/decisions/2026-08-26-persona-hygiene-pass.md`; `DECISIONS.md`
  2026-08-26 row). The old wording was made wrong by remit growth: the description named
  only specifications while the seat's mounts grew to five review targets
  (`review-brainstorm`, `review-governance-intent`, `testing-gap-finding` all arrived after
  it was written); the description is the routing/staffing surface and lagged the router's
  own cross-workflow row.
- **Content:** verbatim superseded value — "Adversarial reviewer who stress-tests
  specifications by finding gaps, challenging assumptions, and identifying edge cases. Asks
  the hard "what if" questions that surface costly problems while they are still cheap to
  fix, and returns a severity-ranked gap report with clarifying questions and a recommended
  verdict."
- **Kept deliberately:** the description's core framing — adversarial posture, the what-if
  sentence near-verbatim, severity-ranked output, clarifying questions, recommended verdict —
  so the v0.63.0 benchmark's routing result is widened, not re-aimed. The entire agent body
  byte-for-byte. This entry supersedes, by ruling, the description-prose portion of the
  [v0.63.0] entry's `Kept deliberately` set.
- **Consumers assessed:** `plugins/mochiko/commands/implement.md:115` names the agent (the
  gap-finding seat: "a fresh `devils-advocate`, dispatched blind per run") — it names the
  agent, never quotes the description, so the staffing contract is untouched and the new
  description now covers exactly that duty; no other command names it. Router agents-table
  row describes the cross-workflow remit (the gap-finding duty detailed at the
  implement-cluster `testing-gap-finding` row); the description now agrees instead of
  lagging. Parsed value 384 chars against the 395 budget — a re-measurement, budget
  unchanged, ledger row annotated.


## [v0.80.0] Skills-Available row drops the stale "Delivery Slices section" clause — slice-vocabulary purge

- **Disposition:** superseded → the same bullet ending at the artifact it names; the review's
  actual scope stays where the persona already says it lives — in
  `mochiko:review-specifications`, whose description carries the feature layer and the Screens
  & Flows of a UX-bearing spec.
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/decisions/2026-08-19-slice-vocabulary-purge.md`).
- **Content (verbatim, the superseded bullet):**

  ```
  - **`mochiko:review-specifications`** — gap review of a drafted spec (`spec.md`), its Delivery Slices section included.
  ```

  Replaced by:

  ```
  - **`mochiko:review-specifications`** — gap review of a drafted spec (`spec.md`).
  ```

- **Kept deliberately:** the bullet itself and the four sibling skill bullets — the
  Skills-Available list is the persona's routing surface. The clause was dropped rather than
  re-keyed to `Screens & Flows` (the current UX section per `schemas/spec.yaml`) because the
  section's own preamble states the rule: "its scope lives in the skill, not a copy here." A
  replacement clause would re-create exactly the copy that went stale.
- **Consumers assessed:** persona file, no downstream reader. The clause named a spec section
  created at v0.49.0 (the task-de-granularization build, `DECISIONS.md` 2026-08-02 — `slices.md`
  became a co-accepted spec Delivery-Slices section) and **removed at v0.58.0** under
  `feature-map-layer` D4, "graduation slices retire — the feature is the pipeline unit"
  (`DECISIONS.md` 2026-08-10). Verified by grep against the current schema: `schemas/spec.yaml`
  carries `Screens & Flows` as its UX section and no `Delivery Slices` section exists. The
  router's own `devils-advocate` row (`skills/mochiko/SKILL.md`) describes the seat as
  "spec-gap critic (feature-layer grade included)" and never named Delivery Slices, so it
  needed no matching edit.

## [v0.79.0] Never-zero calibration scoped to document review

- **Disposition:** superseded → the same calibration bullet, scoped: "**Never approve a
  document review with zero findings** — If a spec, record, or artifact review surfaces
  nothing, you missed something; go back and look harder. The runtime gap-finding pass is the
  exception: there, zero findings with full disclosure is a clean pass, and
  `mochiko:testing-gap-finding` owns that done condition".
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-19 QA gap-finding row;
  record `.mochiko/brainstorms/qa-gap-finding-verification/record.md` D8 as amended at review
  by I10 — the pass's done condition is "every derived expectation probed or explicitly marked
  unprobeable, counts disclosed", with **zero findings = a clean pass, no never-zero rule**;
  disclosure, not volume, is the honesty mechanism).
- **Content:** verbatim superseded text (`## Adversarial Calibration`, first bullet): "**Never
  approve with zero findings** — If a review surfaces nothing, you missed something; go back
  and look harder"
- **Kept deliberately:** the calibration posture itself for every document review — unchanged
  in force, only bounded to the artifact classes it was written for; and the three sibling
  calibration bullets (never downgrade severity, challenge the looks-good instinct, require
  evidence for approval) byte-for-byte, all of which hold on both surfaces. **Amended at the
  V2 fix round (F1):** a fourth sibling did NOT hold on both surfaces — the Quality Standards
  bullet "Thorough over fast" carried the same never-zero premise in weaker words ("every
  review surfaces at least one non-obvious finding") and was scoped in the same wave rather
  than left standing; see the sibling entry below. Scoping one carrier and leaving the other
  is what V2 caught, correctly: the persona would have kept the padding pressure through the
  back door.
- **Why the collision is real, not pedantic:** the review that produced D8 killed a
  zero-findings objection on the premise that the seat was `qa-engineer`, which carries no
  never-zero rule. That premise died in the same review when I1 reseated the pass to
  `devils-advocate` — which does carry one. Left unscoped, this bullet would push the seat to
  pad findings on a clean pass, the exact failure I10's disclosure mechanism exists to prevent.
  The D8 ruling governs the pass; this bullet governs document review.
- **Consumers assessed:** the bullet is persona-local — no command, skill, or router row
  quotes it (grepped). `mochiko:review-specifications`, `review-plan-artifacts`,
  `review-brainstorm`, and `review-governance-intent` are the document-review skills the
  surviving scope covers and are untouched; `mochiko:testing-gap-finding` (new, same wave)
  carries the exception's done condition as its own. **Widened at the V2 fix round (F1):**
  the original sweep checked external consumers only and missed the in-file ones. The whole
  persona body was re-read for other carriers of the never-zero premise; the sweep now covers
  in-file siblings, and it found exactly one — "Thorough over fast" (Quality Standards),
  scoped in the same wave. The remaining sections (Core Identity, What You Produce, What You
  Hunt For, What You Reject, What You Embrace, Delegating Cheap Reads) carry no
  finding-count floor and stand unchanged.

## [v0.79.0] "Thorough over fast" scoped to document review — the never-zero premise's second carrier

- **Disposition:** superseded → the same Quality Standards bullet, scoped: "**Thorough over
  fast** — Every document review surfaces at least one non-obvious finding; shallow \"looks
  good\" is never acceptable. On the runtime gap-finding pass, disclosure — not finding count —
  is the standard".
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-19 QA gap-finding row;
  record `.mochiko/brainstorms/qa-gap-finding-verification/record.md` D8 as amended at review
  by I10 — done condition is every derived expectation probed or explicitly marked unprobeable
  with counts disclosed, **zero findings = a clean pass**; and D4 as amended by I1, the reseat
  that put this persona on the pass and so put its finding-count floors in scope at all).
- **Content:** verbatim superseded text (`## Quality Standards`, first bullet): "**Thorough
  over fast** — Every review surfaces at least one non-obvious finding; shallow \"looks good\"
  is never acceptable"
- **Kept deliberately:** the thoroughness standard itself, unchanged in force for every
  document review, and its anti-rubber-stamp clause verbatim; the three sibling Quality
  Standards bullets (Actionable over abstract, Calibrated severity, Product-framed)
  byte-for-byte — none carries a finding-count floor, so none needed scoping.
- **Why it needed the same treatment:** "at least one non-obvious finding" is the never-zero
  rule restated as a quality standard rather than a calibration. Scoping the calibration
  bullet alone (the first v0.79.0 entry above) would have left the padding pressure intact in
  weaker words — V2 caught this as F1 and the finding is correct. Both carriers now name the
  runtime pass as the exception and route its done condition to
  `mochiko:testing-gap-finding`.
- **Consumers assessed:** persona-local, same as its sibling — no command, skill, router row,
  or template quotes the bullet (grepped). Whole-body re-read for further carriers of the
  premise: none remain.

## [v0.78.0] Delegating Cheap Reads retargeted — `mochiko:explorer` dispatch superseded by native `Explore` + `model: haiku` override

- **Disposition:** superseded → the reworded `## Delegating Cheap Reads` sentence: "spawn a
  disposable native `Explore` subagent with an explicit `model: haiku` override (the
  override makes the read cheap; a bare spawn inherits the session tier)".
- **Tier failed:** n/a — supersession by ruling (ADR
  `.mochiko/decisions/2026-08-19-explorer-retarget-native.md`; `DECISIONS.md` 2026-08-19
  row). Dogfood failure: agent-team teammates cannot spawn plugin-scoped agents, so the
  `mochiko:explorer` dispatch this section prescribed failed on exactly the transport the
  section was built for.
- **Content:** verbatim superseded span (identical across all ten personas): "spawn a
  disposable `mochiko:explorer` subagent (its `model: haiku` frontmatter makes the read
  cheap)".
- **Kept deliberately:** the rest of the `## Delegating Cheap Reads` section byte-for-byte —
  the class-key summary (locate/enumerate/targeted-read cheap; interpretive, absence-driven,
  completeness-sensitive kept), one-gap-per-spawn, the bulk-read-stays-out rule, and the
  closing pointer to `mochiko:patterns-model-tiering`.
- **Consumers assessed:** the section wording is shared across the ten personas; all ten
  edited in the same v0.78.0 wave (this entry mirrored in each persona's strip file). No
  command or skill names the section.

## [v0.63.0] Frontmatter `description:` examples stripped → prose-only agent description
- **Disposition:** superseded → prose-only agent description (variant at `.mochiko/benchmarks/guardrails-vs-detail/variants/agents/devils-advocate.md`); the `<example>` blocks were removed from the frontmatter `description:` block scalar, the prose framing (routing content) kept.
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail benchmark, agents-arm user ruling (b) 2026-08-10 — `DECISIONS.md` benchmark-verdict row 2026-08-10; `.mochiko/brainstorms/validator-scope-and-verbosity/record.md` Benchmark execution; `report/final-verdict.md`).
- **Content:** faithfully compressed. **2 `<example>` blocks removed** from the `description:` value:
  1. Context: a drafted feature specification needs an adversarial gap review — commentary claimed the example demonstrated that a spec-review request triggers adversarial review of requirements completeness with a verdict.
  2. Context: a reviewer is needed to pressure-test requirements and produce an evidence-backed verdict — commentary claimed it demonstrated that a readiness question triggers a structured adversarial review that returns a verdict, never a rubber-stamp.

  Description parsed-value char delta: **1,301 → 315** (chars of the parsed block-scalar value; regex/block-scalar parse, not `wc -c` bytes). Verbatim removed text survives in three homes: (a) git history of `plugins/mochiko/agents/devils-advocate.md`; (b) the pre-edit original state in this tree plus the after-state variant at `.mochiko/benchmarks/guardrails-vs-detail/variants/agents/devils-advocate.md`; (c) archive branch `worktree-brainstorm-validator-scope`.
- **Kept deliberately:** the prose framing of the `description:` (the routing content that staffs the agent — "Adversarial reviewer who stress-tests specifications by finding gaps, challenging assumptions … returns a severity-ranked gap report with clarifying questions and a recommended verdict.") — and the entire agent body, byte-for-byte untouched.
- **Consumers assessed:** grep of `plugins/mochiko/commands/` and `plugins/mochiko/skills/` for `devils-advocate`: `skills/*/SKILL.md` reference(s) only; no command references the agent by name. Routing/staffing contract intact — the agent name and the description's prose framing are unchanged; only the illustrative `<example>` blocks were removed (benchmark: 0 route misses over 20+ staffings).
- **Standing watch:** an F-X1-class review-evidence omission at the first live runs re-opens ruling (b).
- **Protected-content reconciliation:** the prior entries touch the `skills:` frontmatter list and body sections only — [v0.49.0] roster drop (frontmatter `skills:` + Skills-Available bullets), [v0.25.0] "Skills Available" paragraphs (body), [v0.25.0] "What You Hunt For" catalog bullets (body). None touches the frontmatter `description:` value or any `<example>` block. No overlap.

## [v0.49.0] Roster drops review-task-artifacts + review-slices
- **Disposition:** superseded → the two absorbing skills already on the roster (review-plan-artifacts, review-specifications)
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-02 "Task layer de-granularized + slice dissolved into specify (D1–D9)"; record `.mochiko/brainstorms/plan-task-granularity/record.md`, D4+D9)
- **Content:** frontmatter `skills:` entries `review-task-artifacts`, `review-slices` + their two Skills-Available bullets ("completeness review of `task-mapping.md` / `tasks.md`", "completeness review of the `slices.md` decomposition overlay").
- **Consumers assessed:** router agent row (re-keyed same wave) · both dispatching commands.

## [v0.25.0] Six per-mount "Skills Available" paragraphs → one-liners (ruled precedent form)
- **Disposition:** relocated → each mounted skill's own `description:` (the declared single source); the agent keeps the ruled precedent form (task-architect et al.): a single-source framing + one routing line per mount
- **Tier failed:** 1 (each paragraph restated its mounted skill's description — the review-brainstorm and review-governance-intent instances at near-full length)
- **Content:** six paragraphs (~30 lines) summarizing review-specifications / review-plan-artifacts / review-task-artifacts / review-brainstorm / review-slices / review-governance-intent scope, severity classification, and verdict/status formats
- **Consumers assessed:** all six mounted skills' delivered descriptions verified live same day (R1 measurement pass); the agent file is the only consuming surface

## [v0.25.0] "What You Hunt For" catalog bullets → category names + the existing pointer
- **Disposition:** relocated → `review-specifications`' Gap Categories section. **Audit catch (wave-1 audit, 2026-07-25):** the home initially held only the question-framing taxonomy, not the five defect classes — the surviving pointer line had been dishonest since before this wave; the five-class table landed in Gap Categories at fix time, making the relocation (and the pointer) true
- **Tier failed:** 1 (persona keeps the five hunt-category names — what the agent cares about; the per-category bullets were the skill's catalog copied, contradicting the file's own single-source line)
- **Content:** 3–4 example bullets under each of Missing Requirements / Ambiguities / Edge Cases / Assumption Gaps / Contradictions and Conflicts (~20 lines)
- **Consumers assessed:** `review-specifications` untouched; the agent file is the only consuming surface
