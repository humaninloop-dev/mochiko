# Principle Catalog — the tier/type-tagged deck

The catalog is the **seed content** for constitution principles: battle-tested cards the
interrogation session deals, arbitrates, and adapts. It is a starting position, never the final
one — **the interrogation leads, the deck follows.** Principle content comes from three sources,
all recorded in the session synthesis (`.mochiko/memory/governance-intent.md`):

1. **Deck-kept** — a catalog card the user kept (possibly tightened/loosened) during arbitration.
2. **Minted** — a principle written fresh from the user's elicited intent (values dimension). Minted
   content MUST trace to elicited intent, never to shallow prompting.
3. **Floor-preset-with-override** — an Essential Floor card at its tier default, with any
   session-elicited overrides.

## Shelf model

One file per shelf. A project's **type** (declared in the session) selects which shelves are dealt;
its **tier** filters and parameterizes the cards on those shelves.

| Shelf | File | Dealt to | Status |
|-------|------|----------|--------|
| Universal floor | [universal-floor.md](universal-floor.md) | every project, every type | seeded |
| Backend / service | [backend-service.md](backend-service.md) | backend, service, fullstack (API side) | seeded |
| Frontend | `frontend.md` | frontend, fullstack (UI side) | **planned — next authoring pass** |
| CLI | — | cli | **empty by decision — mint-driven** |
| Library | — | library | **empty by decision — mint-driven** |

**Empty shelves are deliberate, not missing.** The CLI and library shelves grow from principles
minted in real sessions (see Graduation seam, below) rather than from another speculative
author-baseline. When a shelf is empty for the declared type, the session leans on minting: the
universal floor still applies, and type principles come from elicited intent.

## Tier taxonomy

Fixed named tiers are the spine — deck filtering and waiver defaults hang off stable tier names —
but the session's risk / values / exclusions dimensions tune the preset per project. **Tier sets
the starting position, never the final one.** Labels are soft and may be renamed by usage; the
structure (an ordered strictness axis with named waiver defaults) is the decision.

| Tier | Meaning | Floor posture | Waiver posture |
|------|---------|---------------|----------------|
| `poc` | Throwaway or proof-of-concept; lifespan measured in weeks | Floor cards dealt at minimum strictness | Any floor category MAY be waived (recorded) |
| `internal` | Internal tool with real users but bounded blast radius | Floor cards at moderate strictness | Floor categories MAY be waived with recorded justification |
| `production` | Real users, real data, real cost of failure | Floor cards at full strictness | Floor categories MUST NOT be waived |
| `regulated` | Compliance obligations on top of production stakes | Full strictness + audit-evidence variants | Floor categories MUST NOT be waived |

A waiver is never silent: it is recorded in the synthesis and in the constitution itself — which
tier waived it and the revisit trigger (tier graduation). Accumulated waivers become the
governance re-entry checklist when the project graduates tiers.

## Card format

Every card carries:

```markdown
### CARD-ID — Card Name
**Type tags:** [which project types this card fits]
**Tier defaults:** poc: [out|offer|default-in] · internal: … · production: … · regulated: …
**Tier parameterization:** [what tightens or loosens by tier — thresholds, enforcement strength]
**Content:** [the principle material — statement skeleton, enforcement/testability/rationale
             source, or a pointer to the canonical definition]
```

- **`default-in`** — dealt as recommended-keep at that tier.
- **`offer`** — dealt as an option, no recommendation weight.
- **`out`** — not dealt at that tier (the user can still ask for it).

Arbitration is a session act: the user keeps, drops, tightens, or re-ranks each dealt card — and
every ruling lands in the synthesis with a trace-ID. A dropped card is a recorded ruling, not an
absence.

## Graduation seam (deferred mechanism, named seam)

Minted principles are **graduation candidates** for this catalog — the deck grows from real
sessions, not one author's baseline. The graduation mechanism itself (curation authority, admission
bar, catalog versioning) is deliberately deferred until real sessions have minted real principles.
The seam that makes deferral safe: every minted principle's provenance is stamped in the synthesis
and the constitution (trace-IDs), so a future graduation pass harvests candidates from real
artifacts. Nothing is lost by waiting.
