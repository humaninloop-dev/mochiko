# Strip notes — `agents/qa-engineer`

Entry formats: `strips/README.md`. Wave context: the implement cluster wave (v0.17.0).

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
