# Shelf: Universal Floor

Dealt to **every project, every type**. These four cards are the Essential Floor — the categories
whose absence degrades a constitution into a preferences document. The floor *concept* is
invariant: no session emits a floor-less constitution. Floor *content and strictness* are
tier-parameterized below, and at low tiers a category MAY be waived — explicitly, recorded in the
constitution with the waiving tier and a revisit trigger. Absence is always deliberate and
auditable, never silent.

The canonical **category definitions** (what each category must address) live in
[../ESSENTIAL-FLOOR.md](../ESSENTIAL-FLOOR.md) — cards here carry tier posture and strictness,
never a second definition. Worked example principles also live there.

> **Seed honesty note:** the current worked examples are backend/service-flavored (RFC 7807 error
> bodies, `/health` endpoints). Frontend-appropriate floor examples ship with the frontend shelf
> (planned next authoring pass). Until then, adapt the *category requirements* to the declared
> type during the session rather than copying misfitting examples.

---

### FLOOR-SEC — Security by Default

**Type tags:** all
**Tier defaults:** poc: default-in (minimum) · internal: default-in · production: default-in · regulated: default-in
**Waiver posture:** waivable at `poc`/`internal` only — and even there, prefer narrowing over waiving (e.g. "no auth — single-user local tool" as a *tightened scope*, not a dropped category)
**Tier parameterization:**

| Tier | Strictness |
|------|-----------|
| poc | Secrets out of the repo (env vars + `.gitignore`); no other mandate |
| internal | + secret scanning in CI, input validation at boundaries |
| production | + auth enforced at all boundaries, dependency vulnerability scanning blocking merge |
| regulated | + audit logging of auth events, documented key-rotation policy, compliance-mapped controls |

**Content:** category definition + example principle in [../ESSENTIAL-FLOOR.md](../ESSENTIAL-FLOOR.md) (Security).

---

### FLOOR-TEST — Testing Discipline

**Type tags:** all
**Tier defaults:** poc: offer · internal: default-in · production: default-in · regulated: default-in
**Waiver posture:** waivable at `poc`/`internal` (a POC that will be thrown away in two weeks may record a testing waiver with graduation as the revisit trigger)
**Tier parameterization:**

| Tier | Coverage pre-seed (session-overridable) |
|------|------------------------------------------|
| poc | No threshold; smoke test on the critical path SHOULD exist |
| internal | ≥60% warning, no blocking gate; ratchet rule optional |
| production | ≥80% warning, ≥60% blocking; ratchet rule (baseline MUST NOT decrease) |
| regulated | ≥90% warning, ≥80% blocking; ratchet rule + coverage evidence retained for audit |

**Content:** category definition + example principle in [../ESSENTIAL-FLOOR.md](../ESSENTIAL-FLOOR.md) (Testing).

---

### FLOOR-ERR — Error Handling Standards

**Type tags:** all
**Tier defaults:** poc: offer · internal: default-in · production: default-in · regulated: default-in
**Waiver posture:** waivable at `poc`/`internal`
**Tier parameterization:**

| Tier | Strictness |
|------|-----------|
| poc | Failures must not silently corrupt data; nothing more |
| internal | + consistent error surface (format fits the type: API error schema, UI error states, CLI exit codes) |
| production | + full category requirements (consistent format, correlation IDs, no leaked stack traces) |
| regulated | + error-event retention and traceability requirements |

**Content:** category definition + example principle in [../ESSENTIAL-FLOOR.md](../ESSENTIAL-FLOOR.md) (Error Handling).

---

### FLOOR-OBS — Observability Requirements

**Type tags:** all
**Tier defaults:** poc: out · internal: offer · production: default-in · regulated: default-in
**Waiver posture:** waivable at `poc`/`internal` (the most commonly waived category at low tiers — record it, with tier graduation as the revisit trigger)
**Tier parameterization:**

| Tier | Strictness |
|------|-----------|
| poc | Not dealt by default |
| internal | Structured logging SHOULD exist; no APM mandate |
| production | Full category requirements (structured logs, correlation IDs, health checks, no PII in logs) |
| regulated | + retention policy, access-controlled log storage, audit-grade traceability |

**Content:** category definition + example principle in [../ESSENTIAL-FLOOR.md](../ESSENTIAL-FLOOR.md) (Observability).
