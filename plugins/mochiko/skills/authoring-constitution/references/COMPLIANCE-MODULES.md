# Compliance Modules — the fact-triggered additive layer

The production floor's companion (PO-D2): regulated and compliance needs are not a tier — they
are **additive elective modules attached by project facts** (industry, data classes,
jurisdiction, contractual commitments). A module only ever ADDS obligations on top of the
asserted floor — never subtracts, never loosens. Attachment is **mechanical from the fact
profile** (interrogation dimension 2): a confirmed fact matches a trigger → the module attaches
and the ruling is recorded in the synthesis. No rigor negotiation occurs at attachment — the
user rules the *facts*; a module's applicability follows from a confirmed fact, never from
appetite.

## Trigger table (seed)

| Trigger fact (dimension 2) | Module | Stratum | Status |
|---|---|---|---|
| Health/medical data about identifiable people (US market) | `hipaa` | legal-mandate | seed |
| Cardholder / payment-card data handled or stored | `pci-dss` | legal-mandate | seed |
| Personal data of EU/UK residents | `gdpr` | legal-mandate | seed |
| Customer-facing UI in a jurisdiction with accessibility statutes (ADA / EAA / EN 301 549) | `a11y` (WCAG) | legal-mandate | seed — routed here per PO-D5's S8 fold |
| Contractual attestation commitments (SOC 2, ISO 27001, customer security addenda) | `attestation` | contractual | seed |

## Strata — waiver posture (ruled 2026-07-30, `po-narrowing-build-scope`)

- **legal-mandate** — obligations entering via a legally-mandated module are **unwaivable**
  (PO-D4.2): a recorded permanent waiver of a legal control is documented evidence of a knowing
  violation, not an escape valve.
- **contractual / non-legal** — waivable under the D4 model: a recorded, auditable
  justification in the governance ledger, permanent pending the D4.1 expiry revisit.
- Both strata are **additive-only** over the floor (PO-D2), in all cases.

## Seed obligations (relocated from the retired `regulated` tier)

The retired tier ladder's `regulated` rows survive here as the **audit-evidence baseline** —
the common obligation pool an attached module draws on at attachment (the session records which
apply). Full per-regime obligation sets (HIPAA safeguards, PCI DSS requirements, WCAG level
targets) are **mint-driven**: authored from real sessions via the catalog's graduation seam,
never speculatively.

- **Security** *(ex FLOOR-SEC `regulated`)*: audit logging of auth events · documented
  key-rotation policy · compliance-mapped controls.
- **Testing** *(ex FLOOR-TEST `regulated`)*: coverage ≥90% warning / ≥80% blocking · coverage
  evidence retained for audit.
- **Error handling** *(ex FLOOR-ERR `regulated`)*: error-event retention and traceability
  requirements.
- **Observability** *(ex FLOOR-OBS `regulated`)*: log retention policy · access-controlled log
  storage · audit-grade traceability.
- **Dependencies** *(ex BE-DEP `regulated`)*: license compliance · documented supply-chain
  review · vulnerability-blocking severity tightened (high/critical → medium+).

## The fact-validation fail-safe (PO record, S4 fold)

A wrong module-driving fact is a **silent under-scoping surface**: "no regulated data" answered
casually while the app stores health metrics means the HIPAA module never fires and nothing
records the miss. Module-driving facts are therefore never merely elicited:

1. **Named elicitation** — dimension 2 asks them explicitly (industry · data classes ·
   jurisdictions/markets · contractual commitments), never infers them silently.
2. **Consequence-stated confirmation** — each negative is confirmed with its consequence in
   view ("no health data confirmed — the HIPAA module will not attach"), and recorded in the
   synthesis with a confidence mark either way.
3. **Brownfield cross-check** — declared facts are checked against `codebase-analysis.md` and,
   where present, data-model DS-XXX annotations and detected integrations; a conflict
   (declared "no payment data", Stripe integration detected) is **confronted in the open**,
   never silently resolved.
4. **Temporal backstop** — a later-discovered contradiction (a data model or integration that
   belies a recorded negative fact) re-opens the fact via an amend run as a governance event,
   never as a quiet local fix.

## Amend events

A fact-profile change (new data class, new market/jurisdiction, new contractual commitment) is
a **governance event**: the amend takes the full review default (pair), the module
attaches/detaches with a synthesis ruling and a ledger entry, and the region's semver bumps
MAJOR (module attach/detach) per the surfaces template.
