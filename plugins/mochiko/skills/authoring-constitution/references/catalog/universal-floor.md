# Shelf: Universal Floor

Dealt to **every project, every type**. These four cards are the Essential Floor — the categories
whose absence degrades a constitution into a preferences document. The floor *concept* is
invariant: no session emits a floor-less constitution. The floor *level* is the asserted
production level below — single, non-negotiable in level (nothing can lower it); a deviation is
only ever a **recorded waiver** (D4: justification in the governance ledger, permanent pending
the D4.1 revisit), never a loosened card. Absence is always deliberate and auditable, never
silent. Audit-evidence variants (the retired `regulated` rows) live in
[../COMPLIANCE-MODULES.md](../COMPLIANCE-MODULES.md) and attach via the fact profile.

The canonical **category definitions** (what each category must address) live in
[../ESSENTIAL-FLOOR.md](../ESSENTIAL-FLOOR.md) — cards here carry the asserted level, never a
second definition. Worked example principles also live there.

> **Seed honesty note:** the current worked examples are backend/service-flavored (RFC 7807 error
> bodies, `/health` endpoints). Frontend-, mobile-, and desktop-appropriate floor examples ship
> with their shelves (planned — Tier-I roadmap work). Until then, translate the *category
> requirements* to the declared type during the session rather than copying misfitting examples.

---

### FLOOR-SEC — Security by Default

**Type tags:** all
**Layer:** floor-asserted
**Asserted level:** secrets out of the repo (env vars + `.gitignore`) · secret scanning in CI ·
input validation at boundaries · auth enforced at all boundaries · dependency vulnerability
scanning blocking merge.
**Waiver posture:** D4 — recorded justification in the ledger; prefer narrowing over waiving
(e.g. "no auth — single-user local companion" as a *tightened scope*, not a dropped category).
**Content:** category definition + example principle in [../ESSENTIAL-FLOOR.md](../ESSENTIAL-FLOOR.md) (Security).

---

### FLOOR-TEST — Testing Discipline

**Type tags:** all
**Layer:** floor-asserted
**Asserted level:** coverage pre-seed (session-overridable): ≥80% warning, ≥60% blocking ·
ratchet rule (baseline MUST NOT decrease) · a smoke test on the critical path exists from day
one.
**Waiver posture:** D4 — recorded justification in the ledger (the young-team on-ramp, PO-D7: a
recorded waiver with the ratchet's starting point set from reality beats a silently ignored
threshold).
**Content:** category definition + example principle in [../ESSENTIAL-FLOOR.md](../ESSENTIAL-FLOOR.md) (Testing).

---

### FLOOR-ERR — Error Handling Standards

**Type tags:** all
**Layer:** floor-asserted
**Asserted level:** failures never silently corrupt data · consistent error surface in the form
that fits the type (API error schema, UI error states, mobile/desktop failure surfaces) ·
correlation IDs · no leaked stack traces.
**Waiver posture:** D4 — recorded justification in the ledger.
**Content:** category definition + example principle in [../ESSENTIAL-FLOOR.md](../ESSENTIAL-FLOOR.md) (Error Handling).

---

### FLOOR-OBS — Observability Requirements

**Type tags:** all
**Layer:** floor-asserted
**Asserted level:** structured logs · correlation IDs · health checks (in the form that fits the
type) · no PII in logs.
**Waiver posture:** D4 — recorded justification in the ledger (historically the most-waived
category on immature stacks — the recorded waiver, not a silent gap, is the honest state).
**Content:** category definition + example principle in [../ESSENTIAL-FLOOR.md](../ESSENTIAL-FLOOR.md) (Observability).
