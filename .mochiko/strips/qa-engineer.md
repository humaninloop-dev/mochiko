# Strip notes — `agents/qa-engineer`

Entry formats: `strips/README.md`. Wave context: the implement cluster wave (v0.17.0).

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

## [v0.64.0] Frontmatter `description:` examples stripped → prose-only agent description
- **Disposition:** superseded → prose-only agent description (Wave 2 editorial extension of the agents-arm ruling); the `<example>` blocks were removed from the frontmatter `description:` block scalar, the prose framing (routing content) kept verbatim.
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail agents-arm user ruling (b) 2026-08-10, extended to the untested agents by the Wave 2 user ruling; `DECISIONS.md` 2026-08-11 build row Wave 2 residual; `report/final-verdict.md` — 0 route misses over 20+ staffings).
- **Content:** faithfully compressed. **3 `<example>` blocks removed** from the `description:` value:
  1. Implementation complete, run the verification tasks — routes verification / quality-gates / checkpoint work to qa-engineer.
  2. Verify a specific feature against real infrastructure — real-infrastructure testing with evidence capture.
  3. Run lint / build / tests before approval — deterministic quality-gate execution.

  Description parsed-value char delta: **1,403 → 250** (chars of the parsed block-scalar value; block-scalar parse, not `wc -c` bytes). Verbatim removed text survives in git history of `plugins/mochiko/agents/qa-engineer.md` (pre-v0.64.0).
- **Kept deliberately:** the prose framing of the `description:` (the routing content that staffs the agent — "Senior QA engineer who treats verification as an engineering discipline. Executes structured verification tasks, captures evidence, audits produced code's shape against the pre-code ladder (advisory findings), and gates completion on human approval.") — and the entire agent body, byte-for-byte untouched (verified against git HEAD).
- **Consumers assessed:** grep of `plugins/mochiko/commands/` and `plugins/mochiko/skills/` for `qa-engineer`: referenced only by the router `plugins/mochiko/skills/mochiko/SKILL.md`; no command references the agent by name. Routing/staffing contract intact — the agent name and the description's prose framing are unchanged; only the illustrative `<example>` blocks were removed.
- **Standing watch:** an F-X1-class route miss on the untested agents re-opens ruling (b).
- **KEPT reconciliation:** the prior [v0.17.0] KEPT entry protects the "Quality Standards" + "What You Produce" persona *body* enumerations, and the [v0.17.0] strip relocated a "Skills Available" *body* bullet. Neither touches the frontmatter `description:` value or any `<example>` block. No overlap with this edit.

**Consumer status (pre-ruling #1, user-supplied):** qa-engineer is **single-consumer today** (`implement`,
the independent Tier-1 validator) but REGISTRY records a **pending `audit`-cluster affinity** ("qa-engineer
still owes its audit-cluster affinity" — Implement-port follow-ups; "Also serves `audit` (affinity) —
reclaim when that cluster ports"). Per the user's binding pre-ruling, the strip below is ruled only because
it is **safe under a future `audit` consumer** — it removes a restatement of `testing-end-user`'s own
`description:` scope, which is consumer-agnostic (the dedup holds whether qa serves implement, audit, or
both). Nothing here binds anything `audit`-specific; when `audit` ports, re-confirm qa-engineer (and
`testing-end-user`) serve it without a produce+grade leak, per the flagged follow-up.

## [v0.17.0] "Skills Available" scope duplication
- **Disposition:** scope enumeration relocated → the skill itself (`mochiko:testing-end-user`), which
  already single-sources it — its `description:` carries the scope (`**TEST:**` detect/parse/execute
  against real infra + Setup/Action/Assert + evidence capture + quality-gate exit codes + CLI/GUI/SUBJECTIVE
  classification + report/checkpoint), and the skill body already says "consult it there rather than
  restating any of it here"; the skill **name + a one-line reach-for-it hint are kept** (the team-form
  function — teammates ignore `skills:` frontmatter, so the in-body name is how a spawned teammate learns
  its skill)
- **Tier failed:** 1 (a second home for the skill's scope — the bullet restated what `testing-end-user`'s
  `description:` already single-sources, while pointing at that same skill as the single source)
- **Content:** the full scope description in the "Skills Available" bullet — "End-user verification
  testing—parsing `**TEST:**` tasks, executing Setup/Action/Assert steps against real infrastructure,
  capturing evidence, running the quality gates and classifying results by exit code, and generating
  verification reports and checkpoint presentations. This is the single source of truth for the
  parse/execute/classify/report procedure and its formats; consult it there rather than restating any of
  it here."
- **Consumers assessed:** implement only (live) + `audit` (pending affinity — flagged). The dedup is
  consumer-agnostic: it relocates a restatement of the skill's own `description:`, so it holds for any
  consumer of qa-engineer. **Safe under a future audit consumer** (pre-ruling #1) — the strip touches no
  implement-specific or audit-specific content. One instance of the 7-agent library-wide "Skills
  Available" pattern; ruled in-wave under the single-live-consumer allowance and noted so the seven-agent
  escalation lands consistently — but re-verify at audit-port that the compressed persona still reads
  correctly for the audit seat (it should: the skill name + hint carry, and audit reaches the full scope
  in the skill).

## [v0.17.0] KEPT: the "Quality Standards" + "What You Produce" persona enumerations
- **Tier-2 evidence:** the agent↔skill composition axis blesses persona (what the agent cares about) in
  the agent; qa's "Quality Standards" (Evidence-first / Reproducible / Honest / Complete / Conservative)
  and "What You Produce" (verification reports / quality-gate results / checkpoint presentations / evidence
  artifacts) are the QA engineer's *taste and output self-description*, with the "Skills Available" section
  already single-sourcing the parse/execute/classify/report **procedure and its formats** to
  `testing-end-user` ("consult it there rather than restating any of it here"). Persona altitude, not a
  behavior-driving procedure restatement — distinct from the "Skills Available" scope catalog stripped
  above. Matches the `technical-analyst` "Quality Standards" and `task-architect` "What You Produce" keeps.
  Consumer-agnostic taste — holds under a future audit consumer as well.
