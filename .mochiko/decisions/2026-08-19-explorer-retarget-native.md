# Explorer retarget — cheap rung moves to native `Explore` + explicit `model: haiku` override

- **Status:** ruled
- **Date:** 2026-08-19
- **Context:** The v0.77.0 model-tiering build (ADR 2026-08-16-model-tiering-build, executing
  model-tiered-seats D4) shipped the cheap rung as a plugin-scoped seat, `mochiko:explorer`
  (`model: haiku` pinned in frontmatter). Dogfood failure, user-reported 2026-08-19: **when an
  agent-team teammate tries to spawn `mochiko:explorer`, the spawn fails** — plugin-scoped
  agent names do not resolve from agent-team teammates. That breaks the build's own primary
  delivery channel: item 5 of the v0.77.0 decision (the uniform `## Delegating Cheap Reads`
  persona section) exists precisely because the persona body is the one channel that reaches
  a seat on the teammate transport — and on that transport the dispatch it prescribes dies.
  The 2026-08-16 ADR's ground for preferring the frontmatter pin over the per-spawn `model`
  parameter ("relies on every dispatcher remembering an argument; the agent-file pin is the
  confirmed, forgettable-proof mechanism") is thereby outweighed: a pin that cannot be
  spawned protects nothing. The per-spawn override mechanism itself was already positively
  evidenced in the same 2026-08-16 session (D6-ii: an opus persona spawned a subagent with a
  haiku override; the subagent self-reported `claude-haiku-4-5-20251001`).
- **Decision:** Retarget the cheap rung; the class key, dispatch ladder, brief obligation,
  and weak-negative watch all stand unchanged. Shipped as v0.78.0:
  1. **The cheap rung is the native `Explore` agent spawned via the Agent tool with an
     explicit `model: haiku` override.** The override is the pin: a bare `Explore` spawn
     inherits the session model (Opus-capped since Claude Code v2.1.198) and fails the
     floor. The native agent resolves from every dispatcher — lead, subagent, and agent-team
     teammate alike — which the plugin-scoped seat does not.
  2. **`mochiko:explorer` is deleted** (agent file + `plugin.json` entry + router agent
     row). No standing replacement seat: the fact-finder constraints its persona carried
     (terse spot-checkable facts with `file:line` provenance, verbatim quotes, method-scoped
     absence, no interpretation) move into the dispatch brief, per the reworked
     `patterns-model-tiering` "Fact-finder brief" clause.
  3. `patterns-model-tiering` (description + body), the six command floor lines, the ten
     persona `## Delegating Cheap Reads` sections, and the router skill row are reworded to
     the native dispatch; each supersession is stripped per primitive.
  This is a recorded supersession of the v0.77.0 mechanism (the 2026-08-16 ADR's items 1 and
  4-in-part, and its "per-spawn `model` parameter as the sole mechanism — rejected as
  primary" alternative ruling). model-tiered-seats D1 (economics), D5 (rostered seats never
  retier), and the doctrine-only enforcement posture stand untouched.
- **Rationale:** The tiering floor's value is that any seat can run it; a rung only the main
  session can dispatch inverts the design (the seats doing the most exploration are exactly
  the teammates that cannot spawn the plugin seat). Native `Explore` + explicit override is
  the only shape that resolves on every transport, and the override mechanism is
  session-evidenced. Doctrine-only enforcement is unchanged — no hooks, nothing kernel-class.
- **Alternatives considered:** keep `mochiko:explorer` for lead-side dispatch and retarget
  only teammate briefs (rejected — two rungs, two doctrines, and the failure mode returns
  whenever a brief line crosses transports); full removal of the tiering floor (rejected —
  the class key and economics are sound; only the dispatch target failed); wait for
  plugin-agent resolution from teammates upstream (rejected — no signal it is coming; the
  floor is broken today).
