# Principle Catalog — the type-shelved deck

The catalog is the **seed content** for constitution principles: battle-tested cards the
interrogation session deals, arbitrates, and adapts. It is a starting position, never the final
one — **the interrogation leads, the deck follows.** Principle content comes from three sources,
all recorded in the session synthesis (`.mochiko/memory/governance-intent.md`):

1. **Floor-asserted** — an Essential Floor card at the asserted production level, its expression
   shaped by type facts during the session; deviations only ever through recorded waivers
   (never a loosened card).
2. **Deck-kept** — an arbitrated catalog card the user kept (possibly tightened) during
   arbitration.
3. **Minted** — a principle written fresh from the user's elicited intent (values dimension).
   Minted content MUST trace to elicited intent, never to shallow prompting.

## Shelf model

One file per shelf. A project's **type** (declared in the session) selects which shelves are
dealt. Mochiko's target is customer-facing product applications (PO-D1): SaaS, web, mobile,
desktop.

| Shelf | File | Dealt to | Status |
|-------|------|----------|--------|
| Universal floor | [universal-floor.md](universal-floor.md) | every project, every type | seeded |
| Backend / service | [backend-service.md](backend-service.md) | backend, service, fullstack (API side) | seeded |
| Frontend | `frontend.md` | frontend, fullstack (UI side) | **planned — Tier-I roadmap work** |
| Mobile | `mobile.md` | mobile | **planned — Tier-I roadmap work** |
| Desktop | `desktop.md` | desktop | **planned — Tier-I roadmap work** |

*(The former CLI and library shelves retired with their types under PO-D1's deferral of
building-block software — libraries, SDKs, CLIs are out of scope, deferred not rejected.)*

**Planned shelves are honest gaps, not silent ones** (the identity docs carry the same
qualification): only backend/service has seeded type content today. When a shelf is planned or
absent for the declared type, the session leans on minting and on adapting the universal floor's
category requirements to the type — never on copying misfitting examples.

## The asserted production floor

There is exactly one standard level — the production floor (PO-D2). The retired
`poc → internal → production → regulated` ladder is gone: no card carries per-tier defaults or
strictness ladders, and no session negotiates the floor's level. What varies per project:

- **Expression** — type facts translate each floor category into its correct form (an API error
  schema vs UI error states; a web health check vs a desktop crash reporter).
- **Modules** — compliance obligations attach additively from the fact profile per
  [../COMPLIANCE-MODULES.md](../COMPLIANCE-MODULES.md) (the retired `regulated` rows live there
  as seed content).
- **Waivers** — any asserted standard can be waived with a recorded, auditable justification in
  the governance ledger (D4; permanent pending the D4.1 revisit) — except legal-mandate module
  obligations (D4.2). A waiver is never silent: recorded in the synthesis and the ledger, it is
  the honest staged-adoption on-ramp for early-stage teams (PO-D7). Accumulated waivers are the
  governance re-entry checklist as the team matures.

## Card format

Every card carries:

```markdown
### CARD-ID — Card Name
**Type tags:** [which project types this card fits]
**Layer:** [floor-asserted | arbitrated]
**Asserted level / Recommended form:** [the single production-level content — thresholds,
             enforcement strength; floor-asserted cards state the asserted level, arbitrated
             cards the recommended form]
**Content:** [the principle material — statement skeleton, enforcement/testability/rationale
             source, or a pointer to the canonical definition]
```

- **floor-asserted** — enters every session at the asserted level; not arbitrated; expression
  shaped by type; loosening only via recorded waiver.
- **arbitrated** — dealt recommend-then-arbitrate (the S7 carve-out layer: architecture-opinion
  and other per-project-judgment cards); the user keeps / tightens / drops / re-ranks.

Arbitration is a session act on the arbitrated layer — and every ruling lands in the synthesis
with a trace-ID. A dropped card is a recorded ruling, not an absence. Floor cards are not
kept/dropped: their per-project record is the expression shaping and any waivers.

## Graduation seam (deferred mechanism, named seam)

Minted principles are **graduation candidates** for this catalog — the deck grows from real
sessions, not one author's baseline. The graduation mechanism itself (curation authority, admission
bar, catalog versioning) is deliberately deferred until real sessions have minted real principles.
The seam that makes deferral safe: every minted principle's provenance is stamped in the synthesis
and the constitution (trace-IDs), so a future graduation pass harvests candidates from real
artifacts. Nothing is lost by waiting.
