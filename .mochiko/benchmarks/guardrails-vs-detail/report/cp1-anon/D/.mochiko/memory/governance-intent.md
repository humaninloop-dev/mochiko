# Governance Intent — Ledgerline

**Session date:** 2026-08-10 · **Mode:** greenfield
**Confirmed at synthesis checkpoint:** 2026-08-10 by principal (Ledgerline founder)
**Governs:** the governance surface set v0.1.0 (CLAUDE.md governance region · `.claude/rules/mochiko/` · `.mochiko/memory/governance-ledger.md`)

## Fact profile

The module-driving facts (interrogation dimension 2), each with its consequence-stated
confirmation — negatives are recorded facts too (the S4 fail-safe):

- **GI-001 — Facts:** industry: fintech-adjacent (invoicing & payment tracking for small
  businesses) · data classes: PII (contractor + client names, emails, addresses, invoice
  amounts / earnings) — **no cardholder data** (Stripe hosts checkout; card data never touches
  Ledgerline systems) · jurisdictions/markets: United States only · contractual commitments:
  none active. **Mark:** Confident
- **Modules triggered (mechanical):**
  - **`a11y` (WCAG) — ATTACHED, legal-mandate stratum (unwaivable, D4.2).** Trigger fact:
    customer-facing web UI + US market (ADA jurisdiction) — the trigger table's "customer-facing
    UI in a jurisdiction with accessibility statutes" row. Mechanical from the fact profile, not
    a session appetite choice; confirmed with the principal via a consequence-stated confrontation
    (the S4 fail-safe — see the a11y-correction note at the foot of the file). Obligation
    formulated minimally (GI-024): WCAG 2.1 AA, automated a11y check in CI on the frontend;
    per-screen criteria mint-driven as the UI becomes real.
  - Negatives confirmed (S4 fail-safe):
    - No cardholder data (Stripe-hosted checkout) → **PCI-DSS not attached**; consequence stated:
      were Ledgerline ever to handle card numbers directly, PCI-DSS would attach and this becomes
      an amend-mode governance event.
    - No health data → **HIPAA not attached**.
    - US-only, no EU/EEA data subjects declared → **GDPR not attached**; consequence stated: PII
      is still present, so a later EU market entry is a fact-profile change routed through amend.
    - SOC 2 mentioned by a prospect's procurement form but **not contractually required** →
      **`attestation` not attached** (contractual stratum); revisit trigger: a signed customer
      contract requiring SOC 2 (amend-mode event).

## Project identity & type

- **GI-002 — Type:** fullstack (backend-weighted) → shelves dealt: universal-floor +
  backend-service (API side); frontend shelf is a planned/absent shelf, so the UI side leans on
  the floor's category requirements translated to type, not on copied examples. **Mark:** Confident
- **Identity:** Ledgerline is a pre-launch, greenfield invoicing and payment-tracking SaaS for
  US small independent contractors (plumbers, electricians, freelance designers). Contractors
  create clients, issue invoices, track payment status, send reminders, and see a cash-flow
  dashboard. Built for the long haul — production quality from day one; real contractors'
  financial data lives in it. Single-tenant-per-account: each contractor sees only their own data.
- **Risk surface:** Money correctness first (an invoice marked paid when it isn't, payment status
  drifting from what Stripe actually knows, or outright invoice-data loss — the tool's core
  promise), then data loss, then a PII breach (names, emails, addresses, earnings). For a product
  touching people's income there is essentially no recovering from any of the three.
- **Team reality:** Solo founder, mid-level backend (~6 years), never run production solo; ~5
  hrs/week part-time React help (UI only). No code-review culture — there is no second set of eyes
  on the backend. Enforcement therefore CANNOT lean on human review; it must be automated, fail
  loudly, and be maintainable by one person. Explicit refusal: any control needing a second human
  in the loop, and ceremony for its own sake.

## Convergence skips

None — all ten dimensions were worked. Team reality (dim 5) was foreshadowed by the solo-founder
answer in dim 1 but was still elicited in full (capacity + enforcement implications), so it is not
recorded as a skip.

## Real commands (dimension 6/8 → the validator's placeholder bar)

| Purpose | Command | Source |
|---------|---------|--------|
| Lint | `ruff check .` | declared (confident) |
| Format check | `black --check .` | declared (confident) |
| Test | `pytest` | declared (confident) |
| Type check (domain-scoped) | `mypy ledgerline/domain` | declared (accepted recommendation) |
| Secret scan | `gitleaks detect --no-banner` | declared (accepted recommendation) |
| Dependency audit | `pip-audit` | declared (accepted recommendation) |
| Complexity gate | `ruff check` (C901 / mccabe) | declared |

CI runner: GitHub Actions (repo already on GitHub).

## Floor expression & deck rulings

Floor cards enter asserted — rows record *expression* (type translation), never a level ruling;
arbitrated cards record the user's ruling.

| GI-ID | Card | Layer | Ruling / Expression | Mark |
|-------|------|-------|---------------------|------|
| GI-003 | FLOOR-SEC | floor-asserted | At floor level, expression tightened for the single-tenant model: secrets from env vars + `.gitignore`; gitleaks secret scan blocking merge in CI; input validation at boundaries via pydantic on FastAPI; auth enforced at every API boundary; **every data query scoped to the authenticated account (tenant isolation)** — the principal's stated breach nightmare; passwords hashed (bcrypt/argon2), HTTPS only; pip-audit blocking on high/critical. | Confident |
| GI-004 | FLOOR-TEST | floor-asserted | At floor level: coverage ≥80% warning / ≥60% blocking, ratchet (baseline MUST NOT decrease), smoke test on the critical (money) path from day one. Greenfield can meet this from the start; not waived. mypy accepted **strict on `ledgerline/domain`** (money/tax/payment code), lenient/off elsewhere. | Confident |
| GI-005 | FLOOR-ERR | floor-asserted | At floor level, expression fitted to the API + money path: **failures never silently corrupt data** (load-bearing for money correctness), consistent JSON API error surface, no leaked stack traces to clients. | Confident |
| GI-006 | FLOOR-OBS | floor-asserted | **Partial** — kept at floor only for the cheap essentials: a `/health` endpoint (Render requires one) and a hard rule that **logs never contain PII**. The heavier observability depth is waived — see GI-012. | Confident |
| GI-007 | BE-HEX | arbitrated | **Kept — LIGHT.** Isolate Stripe behind a payment port and keep money/domain logic free of framework/SDK types, so the money path is testable against a fake Stripe with no network in tests. Full four-layer structure explicitly dropped as ceremony. | Confident |
| GI-008 | BE-SRP | arbitrated | **Kept.** Single responsibility + a CI complexity cap (automated gate, no reviewer needed); metric limits session-tunable at defaults. | Confident |
| GI-009 | BE-DEP | arbitrated | **Kept.** Pin versions in lock file; justify new deps; pip-audit blocks merge on high/critical. Cheap and automated. | Confident |

## Minted principle intents

- **GI-010 — Money-correctness gate:** Code touching invoice totals, tax math, or payment status
  MUST have tests, and those tests block merge AND deploy — no exceptions, no "fix it after."
  Payment status derives from Stripe as the source of truth and is reconciled (webhook-driven),
  so displayed state never silently drifts from what Stripe knows. Money math uses exact decimal
  types, never floats. **Mark:** Confident
  *Elicited from:* dim 9 — "The one thing I will not bend on is money correctness. Anything that
  touches invoice totals, tax math, or payment status has to have tests, and if those tests fail
  the merge or deploy gets blocked — no exceptions."
- **GI-011 — Migration safety:** Destructive or unreviewed database schema migrations are blocked
  automatically; a migration MUST NOT silently lose data. Migrations are forward-only and
  backward-compatible (expand-then-contract) so a code rollback never meets a schema it cannot
  read. **Mark:** Confident (the expand-then-contract mechanism specifically: Assumed — deferred
  by the principal as the lead's domain call)
  *Elicited from:* dim 9 — "Migrations worry me too — I don't want a schema change quietly eating
  data, so if there's an automated way to block a migration that isn't reviewed or is destructive,
  I want that."

- **GI-023 — Database backup & restore (reopen-born):** The managed host's automatic PostgreSQL
  backups (daily backups + point-in-time restore — Render or Railway; host not yet locked, so the
  principle names the capability, not the vendor) MUST be enabled; a "restore from backup"
  procedure lives in the runbook (GI-016). No custom backup infrastructure — configuration + a
  documented procedure only. **Mark:** Confident
  *Provenance:* reopen-born — a coverage survivor from the intent review (data-loss is the
  principal's stated #2 risk yet the session never visited backup/restore). Routed to the user,
  who ruled **explore now**; re-elicited intent landed in the GI namespace. Principal: "That's
  exactly the nightmare — losing everyone's invoice data … turn it on, and put the restore steps
  in the runbook."

- **GI-024 — Accessibility (WCAG 2.1 AA) — `a11y` compliance module, legal-mandate:** The
  frontend MUST meet WCAG 2.1 AA. Enforcement is an automated accessibility check in CI on the
  frontend (e.g. `eslint-plugin-jsx-a11y` + an axe-core check in component/e2e tests). Detailed
  per-screen criteria are mint-driven — authored as real screens exist, never speculatively; the
  standing obligation is the CI check + the AA target. **Unwaivable** (legal-mandate stratum,
  D4.2). **Mark:** Confident
  *Provenance:* mechanical fact-profile attachment (customer-facing US web UI), confronted in the
  open after the principal had filed accessibility as deferrable appetite; principal confirmed the
  facts and the minimal formulation ("WCAG 2.1 AA … an automated a11y check in CI … per-screen
  detail written later … this stays a floor, not a project").

## Waivers

| GI-ID | Standard (floor category / card / module obligation) | Justification | Revisit trigger | Mark |
|-------|------------------------------------------------------|---------------|-----------------|------|
| GI-012 | FLOOR-OBS depth — structured-logging tooling, correlation/trace IDs, APM/dashboards, SLOs | Solo founder, pre-launch, ops-fuzzy; the heavy observability stack is not maintainable by one person today and would be governance theater. The cheap essentials (`/health`, no-PII-in-logs) are kept as a principle (GI-006); this waiver covers only the deferred depth so the floor stays honest rather than pretended. | At launch (first real users), or first production incident — whichever comes first | Confident |

## Module selections

| GI-ID | Module | Ruling | Because | Mark |
|-------|--------|--------|---------|------|
| GI-013 | layer-rules | adopted | BE-HEX kept (light) — a payment port + domain isolation is a layered boundary that needs enforceable import rules. | Confident |
| GI-014 | knowledge-management (core) | adopted | Offered default-on at dim 7; principal wants a trustworthy decisions log + backlog for solo continuity ("beats me trying to remember why I chose things six months from now"). | Confident |
| GI-015 | KM elective — CHANGELOG.md | declined | Continuously-deployed SaaS, no versioned releases, nobody reads release notes. Durable decline; revisit if it starts shipping versioned releases. | Confident |
| GI-016 | KM elective — RUNBOOK.md | adopted (lightweight) | Deployed service + solo + ops-fuzzy; a minimal "restart / roll back / what to check when it breaks" runbook is a cheap 2am-and-it's-only-me safety net. Moved from Unsure to adopted after recommendation. | Confident |
| GI-017 | release-gates | adopted | Offered default-on for a deployed product; exactly the automated, one-person-maintainable gate the principal wants between them and shipping something broken to payment data. No red-build deploys. | Confident |

## Domain-dependency seeds (layer-rules adopted)

Session-arbitrated registry seeds — domain-relevance filtered, trust-ranked, level-cited:

| GI-ID | Dependency | Signal level | Ruling | Mark |
|-------|------------|--------------|--------|------|
| GI-018 | `pydantic` | 1 — de-facto ecosystem standard for Python data modeling/validation (>80% adoption) | kept — domain modeling + boundary validation | Confident |

Note: exact money math uses the Python standard-library `decimal.Decimal` (stdlib, no registry
entry needed) — recorded here so the money-correctness principle's "decimal not float" is anchored.

## Deliberate exclusions (dimension 10)

- **GI-019:** Operational governance beyond the kept floor essentials — SLOs, incident-response
  process, on-call. Excluded pre-launch (distinct from the FLOOR-OBS depth *waiver* GI-012; this
  is non-floor ops scope). Revisit post-launch. **Mark:** Confident
- **GI-020:** Data retention & deletion policy. The principal suspects invoices carry a legal
  retention rule but cannot name it and will not pretend to govern it today. Excluded now; revisit
  post-launch with legal input (flagged: PII + suspected statutory retention). **Mark:** Confident
- **GI-021:** SOC 2 compliance program. No obligation (a prospect mentioned it; nobody requires
  it). Excluded; revisit if a customer contract requires it (also recorded as a fact-profile
  negative, GI-001). **Mark:** Confident
- **GI-022:** Teams / multi-user (the "bookkeeper seat"), and any multi-tenant-sharing design
  beyond single-tenant-per-account isolation. Explicit "later" — refused as design scope now.
  Revisit post-launch. **Mark:** Confident

## Review

<!-- The durable record of the sized pre-G3 intent review — one block per run. Recovery keys off
this section's state. -->

**2026-08-10 — first ratification**

- **Sizing:** lead stated weight — 22 GI elements; mark mix almost entirely Confident (one
  Assumed sub-mark on GI-011's expand-contract mechanism); reality-surface load moderate (a
  fintech-adjacent money path, a tenant-isolation breach surface, one floor waiver, four
  exclusions). Default on first ratification is a **pair**; **lead sized: single** — the synthesis
  is unusually low-contention (no Contested elements, a coherent solo-founder posture, a small
  deck), and the single load-bearing judgment risk (the OBS waiver and the money-path mints)
  concentrates in a few elements a solo cold reviewer can cover. Departure-trail: sized below the
  pair default because element count is bounded and mark mix carries no contested rulings.
- **Review:** reviewer — solo cold intent reviewer (coverage + coherence lenses combined, blind-map
  dispatched); **tally** 4 raised → 3 merged survivors; recommended status **needs-revision**
  (then resolved — see dispositions).
- **Survivor dispositions:**

  | # | Sev | GI element(s) | Finding | Disposition |
  |---|-----|---------------|---------|-------------|
  | S1 | Important | GI-006 / GI-012 | FLOOR-OBS waiver defers correlation/trace IDs, but GI-005 (error handling) and GI-010 (money reconciliation debugging) both lean on being able to trace a request end-to-end — waiving trace IDs may undercut the money-correctness posture the session prized most. | user-ruled — principal keeps a **minimal request-ID on the money/payment path only** (cheap, no APM), the broader correlation-ID/APM depth stays waived. GI-006 expression amended to add the money-path request-ID; GI-012 scope narrowed accordingly. |
  | S2 | Important | GI-010 | "Payment status reconciled, no silent drift" names Stripe as source of truth but the synthesis never states WHEN reconciliation runs (webhook vs poll) — an unstated trigger is where drift actually hides. | resolved — GI-010 amended: reconciliation is webhook-driven with a periodic safety poll; the failure mode (missed webhook) is explicitly the thing the reconciliation test must cover. |
  | S3 | Minor | GI-020 | Data-retention exclusion touches PII with a *suspected legal* obligation — excluding it silently risks a real compliance gap masquerading as scope. | resolved — GI-020 already flags the suspected statutory retention and sets a post-launch legal-input revisit; reviewer accepted the flag as sufficient honesty for pre-launch. |

- **Coverage findings (blind angle map diff):** two never-visited angles survived the diff, both
  routed to the user as candidate topics (coverage-survivor routing is the user's ruling):
  1. **Database backup & restore** (Important) — data-loss is the stated #2 risk, yet the session
     governed only migration safety, never backup/restore. A plausible in-scope ruling would have
     differed. User ruled **explore now** → re-elicited as GI-023 (reopen-born intent).
  2. **Frontend accessibility** (Minor → escalated) — customer-facing React UI with no
     accessibility floor. Initially the user ruled **defer** (thin/part-time UI). **Corrected at
     authoring:** accessibility is not appetite here — the `a11y` (WCAG) compliance module attaches
     **mechanically** from the fact profile (customer-facing UI + US/ADA jurisdiction) at the
     legal-mandate stratum, so it is unwaivable and cannot be deferred by preference. Confronted
     in the open (S4 consequence-stated confirmation); the principal confirmed the facts. Landed
     as GI-024 with a **minimal** obligation (WCAG 2.1 AA + a CI a11y check; per-screen criteria
     mint-driven). This is the intent review's coverage instrument doing exactly its job —
     catching a floor-adjacent miss the session waved through.
- **Verify pass:** PASS — the sole reviewer confirmed the S1/S2 folds re-read cleanly against the
  amended elements, and the reopen-born GI-023 for internal consistency and provenance (rides the
  verify pass like a fold — no fresh cold read, no blind-map hunt against it). GI-023 is
  consistent with GI-011 (both serve the data-loss risk; migration safety prevents corruption,
  backup/restore recovers from loss) and adds no contradiction.

## Ratification notes (2026-08-10)

Ratified as-is by the principal, with two non-blocking clarifications applied by the lead as a
bounded post-review delta-pass (wording only, no new selection):
- **Host-agnostic:** Render is not locked (Railway possible). GI-023 and any host-shaped
  expression name the *capability* (managed Postgres backups, a `/health` endpoint the host
  requires), never the vendor. Still-seated reviewer delta-pass: PASS (no new contradiction).
- **Deferred items must be tracked, not forgotten:** GI-019 (SLOs/incident-response), GI-020
  (data retention/deletion — suspected statutory obligation) carry post-launch revisit triggers;
  the lead lands them as tracked ROADMAP/BACKLOG entries at KM scaffolding so a revisit is
  guaranteed, not memory-dependent.

## a11y-correction note (2026-08-10, pre-acceptance)

During authoring the producer hit COMPLIANCE-MODULES and found the `a11y` (WCAG) trigger fires
mechanically for a customer-facing UI in an accessibility-statute jurisdiction — Ledgerline is a
customer-facing web app in the US (ADA). The session had treated accessibility as deferrable
appetite (a coverage finding the principal deferred). That was a fact-profile miss: legal-mandate
module attachment follows from confirmed facts, never from appetite, and legal-mandate obligations
are unwaivable (D4.2). Handled per the S4 fail-safe: confronted in the open with the consequence
stated, principal confirmed the underlying facts, obligation formulated minimally (GI-024). Because
this happened before acceptance in the FIRST ratification, it is folded directly into the synthesis
(GI-001 modules-triggered + GI-024) rather than logged as an amendment; the region ships at the
initial v0.1.0 with `a11y` attached. Were this discovered post-acceptance, it would be an
amend-mode governance event (module attach → MAJOR bump).

## Amendment Log

Empty on first ratification.
