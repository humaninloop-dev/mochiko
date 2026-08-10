# Strip notes — `skills/grooming-operating-docs/`

Entry formats: `strips/README.md`.

**Wave context (v0.44.0 — the D7 leakage scrub).** `verbosity-caveman-ops-separation` D7 as
folded at review (S4): **full scrub** of ops leakage from the shipped tree, with no
changelog-worthy detail lost — every removed block is preserved verbatim below. Ruling:
`DECISIONS.md` 2026-08-01 "Output verbosity, caveman & ops separation ruled" row.

**The leak test this wave used, recorded so a future sweep inherits it: *whose artifact does the
pointer name?*** Mochiko's own ops records — `.mochiko/strips/`, `.mochiko/brainstorms/`,
`.mochiko/decisions/`, `.mochiko/archive/` — are leaks: they resolve to nothing in an installed
plugin. Adopter runtime paths (`.mochiko/specs/`, `.mochiko/memory/`) and the KM module's
document contracts are the **user's** artifacts and are untouchable. A prefix-based sweep on
`.mochiko/` would gut the KM module and the brainstorm command; 101 of this tree's 146
`.mochiko/` references were correctly left alone on that test.

## [v0.44.0] Delivery-sweep source list: the `.mochiko/strips/` entry
- **Disposition:** superseded → deleted from the shipped file; preserved verbatim here.
- **Tier failed:** n/a — supersession by ruling (`verbosity-caveman-ops-separation` D7 + S4; the `DECISIONS.md` 2026-08-01 row above)
- **Content (verbatim):**
```
trail, `.mochiko/strips/`, the decisions layer, git history, and the primitives on disk.
```
- **Kept deliberately:** the other four sweep sources — trail, decisions layer, git history, primitives on disk — all of which exist in an adopter project. The strips directory never did: it is mochiko-repo-only, so the line sent an adopter's groom run at a path that cannot exist.

## [v0.44.0] Report-writer re-open trigger's session-slug citation
- **Disposition:** superseded → deleted from the shipped file; preserved verbatim here.
- **Tier failed:** n/a — supersession by ruling (`verbosity-caveman-ops-separation` D7 + S4; the `DECISIONS.md` 2026-08-01 row above)
- **Content (verbatim):**
```
(`model-tiered-seats` D3)
```
- **Kept deliberately:** the whole trigger — an expansion-heavy-surface hit is logged as a BACKLOG item for the user and never acted on in the skill.
