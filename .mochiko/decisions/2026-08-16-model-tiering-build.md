# Model-tiering build — D4 executed, retargeted post-v8

- **Status:** ruled
- **Date:** 2026-08-16
- **Context:** The `model-tiered-seats` record (accepted 2026-07-24) ruled the class-keyed
  cheap-explorer design (D4) but deferred the build to the token-reduction epic's scoping.
  Its build plan is stale: the ~10 "native `Explore`" dispatch sites it cited
  (`loop-discipline/SKILL.md:91`, `plan.md:124/152`, `tasks.md:114/135`, `implement.md:148`,
  and siblings) were all refactored away at the v8 command rebuild — a repo sweep this
  session found zero surviving dispatch lines. The user ruled (2026-08-16): mochiko-wide
  strict guidance to use cheap models for exploration-class operations; rostered teammates
  stay opus. The mechanism was confirmed empirically in-session: a `model: opus` persona
  (principal-architect) spawned a subagent with a haiku override; the subagent self-reported
  `claude-haiku-4-5-20251001` and returned a correct targeted read (tech-lead.md frontmatter).
- **Decision:** Execute D4 retargeted at the current doctrine surface, shipped as v0.77.0:
  1. New agent `mochiko:explorer` (`model: haiku` frontmatter — the pin; disposable per gap,
     locate/enumerate/targeted-read/deterministic checks only, per D4's F5 fold).
  2. New skill `mochiko:patterns-model-tiering` — single source of the class key (D4 + F2
     guide-line), the dispatch ladder, the weak-negative watch, and the brief obligation
     (every seat brief carries the routing rule — spawned teammates never load `skills:`
     frontmatter, so the brief is the only channel that reaches them).
  3. One floor line in each of the six commands (Ways of Working for the charters, Harness
     for the Goal-form commands), referencing the skill, never restating it.
  4. Router registration (skill row + agent row) and `plugin.json` agent entry.
  5. A uniform `## Delegating Cheap Reads` standing section in all ten persona bodies
     (user-directed, 2026-08-16): the persona body is the one delivery channel that reaches
     a seat on both transports — teammates drop `skills:` frontmatter but load the persona —
     so each rostered agent carries the delegate-cheap-reads rule itself and spawns its own
     disposable `mochiko:explorer` subagents (platform: nested spawns allowed to depth 3,
     teammates foreground-only). The section names no command or pipeline stage, keeping
     the decoupling-by-absence keystone intact; `mochiko:explorer` itself is excluded.
  D5 stands untouched: no rostered seat changes tier; all ten personas stay `model: opus`.
  D6's three probe items continue riding the epic's OTel probe, with D6-ii (per-invocation
  `model` on built-in Explore) now positively evidenced by this session's spawn test.
- **Rationale:** The cost channels are D1's — Haiku ~5×/~10× cheaper than Opus/Fable per
  token, plus Opus-cap headroom on subscription seats — and native `Explore` stopped being
  guaranteed-cheap at Claude Code v2.1.198 (Opus-capped inherit). Doctrine-only enforcement
  (MUST skill description + command floor lines + brief injection) keeps the build inside
  the skills-and-agents quality surface; no hooks, no kernel-class tooling, nothing gates or
  dispatches mechanically. Enforcement escalation (e.g. a PreToolUse hook) would cross the
  kernel bright line and is deliberately not taken — revisit only on dogfood evidence that
  seats ignore the floor.
- **Alternatives considered:** `CLAUDE_CODE_SUBAGENT_MODEL=haiku` (rejected — blunt: would
  cheapen validator and reviewer subagents too); `teammateDefaultModel` (rejected — would
  downgrade the rostered opus seats the user ruled to keep); per-spawn `model` parameter as
  the sole mechanism (rejected as primary — relies on every dispatcher remembering an
  argument; the agent-file pin is the confirmed, forgettable-proof mechanism, per the D4
  flow note); blocking PreToolUse hook (rejected — kernel-class gating, needs its own
  recorded ruling and dogfood evidence of doctrine failure first).
