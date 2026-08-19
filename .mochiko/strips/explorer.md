# Strip notes — `agents/explorer.md` (deleted v0.78.0)

Entry formats: `strips/README.md`. First and final entry — the agent was born v0.77.0
(model-tiered-seats D4 executed, ADR 2026-08-16-model-tiering-build) and deleted whole one
version later.

## [v0.78.0] Agent deleted — cheap rung retargeted to native `Explore` + explicit `model: haiku` override

- **Disposition:** superseded → native `Explore` spawned with `model: haiku`, per the
  reworked `patterns-model-tiering` skill; the persona's fact-finder constraints move into
  the dispatch brief (the skill's new "Fact-finder brief" clause). `plugin.json` agents
  entry and the router agent-table row deleted with it.
- **Tier failed:** n/a — supersession by ruling (ADR
  `.mochiko/decisions/2026-08-19-explorer-retarget-native.md`; `DECISIONS.md` 2026-08-19
  row). Dogfood failure: agent-team teammates cannot spawn plugin-scoped agents — the seat
  was unreachable from exactly the dispatchers the v0.77.0 persona-delegation channel
  targeted.
- **Content:** the whole agent, faithfully compressed — `model: haiku` frontmatter pin;
  scope: locate / enumerate (bounded, spot-checkable) / targeted read (verbatim, never
  paraphrase) / deterministic checks; return style: terse facts with `file:line`
  provenance, shortest decisive span, no raw dumps; absence method-scoped ("not found by
  `<method>` over `<scope>`"), never a nonexistence verdict; uncertain match returns
  candidates; refusals: interpretation ("interpretive — session tier"), writes, grading.
  Verbatim text survives in git history (`plugins/mochiko/agents/explorer.md` at v0.77.0,
  commit 7536b95).
- **Kept deliberately:** every behavioral constraint listed above — relocated, not dropped:
  the skill's "Fact-finder brief" clause now obliges the dispatch brief to carry them,
  since the native agent has no persona pinning it.
- **Consumers assessed:** dispatch sites were doctrine lines only (six commands, ten
  personas, the tiering skill, the router) — all reworded to the native dispatch in the
  same v0.78.0 wave. Nothing else names the agent.
