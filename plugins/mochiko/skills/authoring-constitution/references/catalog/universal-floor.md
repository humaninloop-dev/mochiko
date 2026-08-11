# Shelf: Universal Floor

Dealt to **every project, every type**. These four cards are the Essential Floor — the categories
whose absence degrades a constitution into a preferences document. The floor *concept* is
invariant: no session emits a floor-less constitution, and every category below is present at
**both** depth levels (breadth invariant — a `high`-only obligation means that obligation is
absent at `low`, never that the category is). The floor *depth* is one project-wide dial with two
rows — **low** and **high** — declared by the user at setup and moved only by an explicit user
re-declaration, one-way `low`→`high` (D1/D2); nothing derives or auto-advances it, and no watcher
flips it. The `low` row is drawn on the retrofit-cost cut line (D5): obligations expensive to
retrofit hold identically at both levels, while addable rigor (merge-blocking gates, coverage
thresholds) may relax at `low`. A deviation from either row is only ever a **recorded waiver**
(D4: a per-check *fit* exception justified in the governance ledger, available at both levels,
permanent pending the D4.1 revisit), never a loosened card; staged adoption is the `low` level,
not a waiver (PO-D7 superseded). Absence is always deliberate and auditable, never silent. Audit-evidence variants (the retired `regulated` rows) live in
[../COMPLIANCE-MODULES.md](../COMPLIANCE-MODULES.md) and attach via the fact profile.

The canonical **category definitions** (what each category must address) live in
[../ESSENTIAL-FLOOR.md](../ESSENTIAL-FLOOR.md) — cards here carry the two-row `low`/`high` depth,
never a second definition. Worked example principles also live there.

> **Seed honesty note:** the current worked examples are backend/service-flavored (RFC 7807 error
> bodies, `/health` endpoints). Frontend-, mobile-, and desktop-appropriate floor examples ship
> with their shelves (planned — Tier-I roadmap work). Until then, translate the *category
> requirements* to the declared type during the session rather than copying misfitting examples.

---

### FLOOR-SEC — Security by Default

**Type tags:** all
**Layer:** floor-asserted
| Level | Asserted level |
|-------|----------------|
| **low** | secrets out of the repo (env vars + `.gitignore`) · input validation at boundaries · auth enforced at all boundaries · secret scanning runs (pre-commit or CI) · dependency vulnerability scanning runs |
| **high** | low, and: secret scanning **blocks merge** in CI · dependency vulnerability scanning **blocks merge** at high/critical severity |

**Waiver posture:** D4 — recorded justification in the ledger, available at either level as a per-check *fit* exception; prefer narrowing over waiving (e.g. "no auth — single-user local companion" as a *tightened scope*, not a dropped category). Staged adoption is the **low** level, not a waiver (PO-D7 superseded).
**Content:** category definition + example principle in [../ESSENTIAL-FLOOR.md](../ESSENTIAL-FLOOR.md) (Security).

---

### FLOOR-TEST — Testing Discipline

**Type tags:** all
**Layer:** floor-asserted
| Level | Asserted level |
|-------|----------------|
| **low** | a smoke test on the critical path exists from day one · ratchet rule (baseline MUST NOT decrease) · coverage measured and reported on every PR |
| **high** | low, and: coverage thresholds enforced — ≥80% warning, ≥60% blocking (session-overridable) |

**Waiver posture:** D4 — recorded justification in the ledger, available at either level as a per-check *fit* exception. The young-team on-ramp is the **low** level, not a waiver (PO-D7 superseded): coverage is measured and ratcheted from reality before the blocking threshold is asserted at `high`.
**Content:** category definition + example principle in [../ESSENTIAL-FLOOR.md](../ESSENTIAL-FLOOR.md) (Testing).

---

### FLOOR-ERR — Error Handling Standards

**Type tags:** all
**Layer:** floor-asserted
| Level | Asserted level |
|-------|----------------|
| **low** | failures never silently corrupt data · no leaked stack traces · errors are surfaced, never swallowed |
| **high** | low, and: consistent error surface in the form that fits the type (API error schema, UI error states, mobile/desktop failure surfaces) · correlation IDs |

**Waiver posture:** D4 — recorded justification in the ledger, available at either level as a per-check *fit* exception.
**Content:** category definition + example principle in [../ESSENTIAL-FLOOR.md](../ESSENTIAL-FLOOR.md) (Error Handling).

---

### FLOOR-OBS — Observability Requirements

**Type tags:** all
**Layer:** floor-asserted
| Level | Asserted level |
|-------|----------------|
| **low** | logs exist on the critical path · no PII in logs |
| **high** | low, and: structured logs · correlation IDs · health checks (in the form that fits the type) |

**Waiver posture:** D4 — recorded justification in the ledger, available at either level as a per-check *fit* exception (historically the most-waived category on immature stacks — the recorded waiver, not a silent gap, is the honest state). Staged adoption is the **low** level, not a waiver (PO-D7 superseded).
**Content:** category definition + example principle in [../ESSENTIAL-FLOOR.md](../ESSENTIAL-FLOOR.md) (Observability).
