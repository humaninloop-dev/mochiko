# Strip notes — `skills/patterns-model-tiering/SKILL.md`

Entry formats: `strips/README.md`. First entry (new file, created v0.78.0 — the skill was
born v0.77.0 as a pure addition).

## [v0.78.0] Cheap rung retargeted — `mochiko:explorer` superseded by native `Explore` + explicit `model: haiku` override

- **Disposition:** superseded → the reworked skill (same file): cheap rung = native
  `Explore` spawned via the Agent tool with an explicit `model: haiku` override; "The
  override is the pin" replaces "The frontmatter is the pin"; a new "Fact-finder brief"
  dispatch-shape clause carries the constraints the deleted explorer persona used to pin
  (terse `file:line`-provenanced facts, verbatim quotes, method-scoped absence, no
  interpretation).
- **Tier failed:** n/a — supersession by ruling (ADR
  `.mochiko/decisions/2026-08-19-explorer-retarget-native.md`; `DECISIONS.md` 2026-08-19
  row). Dogfood failure: agent-team teammates cannot spawn plugin-scoped agents, so the
  `mochiko:explorer` rung failed on the transport doing most of the exploration.
- **Content:** superseded spans, faithfully compressed —
  - `description:` — "go to the cheap explorer seat (`mochiko:explorer`, model-pinned
    haiku)";
  - Overview — "Since Claude Code v2.1.198 the native `Explore` agent inherits the session
    model (Opus-capped), so 'just use Explore' is no longer cheap — the cheap rung is the
    plugin's own scoped seat: **`mochiko:explorer`**, its `model: haiku` pinned in
    frontmatter (D4)";
  - class-key heading — "dispatch `mochiko:explorer`, disposable per gap";
  - ladder — "Direct tool call → cheap explorer → session-tier read";
  - dispatch shape — "spawn `mochiko:explorer` via the Agent tool" and "**The frontmatter
    is the pin** — dispatching the scoped seat by name is what makes the read cheap; no
    per-spawn model parameter is needed or relied on";
  - brief obligation + checklist — "route ... to `mochiko:explorer`".
- **Kept deliberately:** the class key itself, the session-tier carve-outs (interpretive ·
  absence-driven · completeness-sensitive · producing/reviewing/grading never tiered down,
  D5), the dispatch ladder's three-rung shape, disposable-per-gap / no-librarian (D4/F5),
  the weak-negative watch, the brief obligation, the D1 economics paragraph, and the
  bare-spawn-inherits-session-model fact (restated as the reason the override is
  mandatory).
- **Consumers assessed:** six command floor lines + ten persona sections + the router row
  all point here and were reworded in the same v0.78.0 wave (their strip files carry the
  mirrored entries).
