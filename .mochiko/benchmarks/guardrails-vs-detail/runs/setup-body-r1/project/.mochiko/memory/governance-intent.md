# Governance Intent — Ledgerline

**Session date:** 2026-08-10 · **Mode:** greenfield
**Confirmed at synthesis checkpoint:** 2026-08-10 by Priya (founder)
**Governs:** the governance surface set v1.0.0 (CLAUDE.md governance region · `.claude/rules/mochiko/` · `.mochiko/memory/governance-ledger.md`)

## Fact profile

The module-driving facts (interrogation dimension 2), each with its consequence-stated
confirmation — negatives are recorded facts too (the S4 fail-safe):

- **GI-001 — Facts:** industry: SaaS invoicing / payment-tracking for solo contractors ·
  data classes: personal contact info (names, emails, mailing addresses of contractors AND
  their clients) + financial records (invoice line items, amounts, tax rates, payment status) ·
  **no cardholder data** (Stripe hosts checkout; card data never touches Ledgerline servers) ·
  no health data, no government IDs, no SSNs · jurisdictions/markets: **US only** (US
  contractors, US clients, US business; no EU/UK targeting) · contractual commitments: **none
  signed** (one prospect's procurement form mentioned SOC 2; no signed obligation, no active
  commitment) · **Mark:** Confident
- **Modules triggered (mechanical):**
  - **Accessibility (WCAG) — ADOPTED STANDARD, not legal-mandate** *(reclassified by cold-review
    survivor S3, founder ruling)*. Original DECK-1 attachment was `a11y` legal-mandate on the
    trigger fact "customer-facing web UI served to US users." Cold review flagged that US
    web-accessibility obligation for a private SaaS rests on contestable ADA case law / DOJ
    guidance, not a clean statute; the founder reclassified it to a **strong accessibility
    standard the project adopts** — still committed to **WCAG 2.1 AA on the core contractor
    flows** (invoice create/send, payment-status view), but **scopable/waivable** under D4 so a
    solo founder can sequence it, not unwaivable D4.2. Recorded as module GI-025.
  - **Negatives confirmed (consequence stated):**
    - No health data → **`hipaa` will not attach.** Confirmed.
    - No cardholder data on Ledgerline servers (Stripe-hosted checkout) → **`pci-dss` will not
      attach.** Confirmed. **Enforced constraint + watch (cold-review survivor S2, founder
      ruling):** this negative holds ONLY while card data never touches Ledgerline servers
      (Stripe-hosted checkout = PCI SAQ-A eligibility). Recorded as constraint GI-026: **card
      data MUST never touch Ledgerline servers.** Temporal backstop: any feature that would
      collect/proxy card data re-opens the fact and attaches `pci-dss` (governance event) —
      same shape as the SOC 2 watch, so the drift cannot be silent.
    - US-only, no EU/UK residents → **`gdpr` will not attach.** Confirmed. **Reasoned negative
      on US state privacy law (cold-review survivor S1, founder ruling):** "US only" rules out
      GDPR but NOT all privacy regimes — California CCPA/CPRA and VA/CO/CT state privacy laws
      can apply to a nationwide SaaS holding consumer PII once thresholds are crossed. Founder
      ruling: **not governed now** — Ledgerline is below the applicability thresholds at ~200
      contractors, no CCPA/state-privacy module attaches today — but the negative is **recorded
      with a revisit trigger** (scale beyond the state thresholds, OR any new consumer-facing
      data collection), not left as a silent gap.
    - No signed attestation/contractual commitment → **`attestation` will not attach.**
      Confirmed. **Watch item (temporal backstop):** a prospect raised SOC 2; if a SOC 2 or
      similar obligation is ever signed, an amend run re-opens the fact and attaches the module
      (governance event). Recorded so the miss cannot be silent.
- **Brownfield cross-check:** n/a — greenfield, no code to cross-check.

## Project identity & type

- **GI-002 — Type:** fullstack → shelves dealt: universal floor (all) + backend/service (API
  side). Frontend shelf is planned/absent — floor categories translated to the React UI by
  category requirement, not by copying misfitting examples. · **Mark:** Confident
- **Identity:** Ledgerline — a SaaS invoicing and payment-tracking tool for US solo
  independent contractors (plumbers, electricians, freelance designers) who lose money by not
  sending or chasing invoices. Contractors create clients, issue invoices, track payment
  status, send reminders, and get a cash-flow view; Stripe collects payment. Pre-launch,
  greenfield, built for production from day one because real financial data lives in it. First
  paying customer ~4 months out; target ~200 paying contractors in year one. A real product
  intended to run and grow.
- **Risk surface:** worst realistic outcomes — (1) a breach or data-loss exposing contractors'
  and clients' financial + contact info → trust gone instantly, business dead; (2) payment
  state drifting from reality (invoice shows "paid" when unpaid, or a reminder for a settled
  invoice) → the contractor looks bad to their own customer. Direct money-loss risk is lower
  because Stripe, not Ledgerline, moves the money.
- **Team reality:** solo founder (6 years backend, strong on FastAPI/Postgres/migrations,
  self-described weak on security beyond bcrypt/HTTPS) writing all backend code; one part-time
  UI helper (~5 hrs/week React, takes tickets, owns nothing); no ops. **No code-review
  culture — the founder merges their own PRs.** Governance MUST be enforceable by one person
  who is also writing all the code: **CI is the reviewer.** A rule that needs a second human
  reviewer to function will not run here.

## Convergence skips

- Dimension 5 (team reality) largely settled by dimension 1's "solo founder + part-time
  helper" — probed only for review culture, which confirmed no reviewer. Bookkeeping, not a
  scope ruling.
- Dimension 6 (existing practices) has no detected stack (greenfield) — collected as declared
  intended tooling instead of detected commands.

## Real commands (dimension 6/8 → the validator's placeholder bar)

| Purpose | Command | Source |
|---------|---------|--------|
| Lint (backend) | `ruff check .` | declared |
| Format check (backend) | `ruff format --check .` | declared (ruff format supersedes black) |
| Type check (backend) | `mypy .` | declared — CI-enforced per founder request |
| Test (backend) | `pytest` | declared |
| Lint (frontend) | `npm run lint` (ESLint) | declared |
| Test (frontend) | `npm test` (Vitest) | declared |
| Secret scan (CI) | `gitleaks detect` (or `trivy fs --scanners secret .`) | recommended |
| Dependency audit | `pip-audit` · `npm audit` | declared |
| Import-linter | `lint-imports` (Python `import-linter`) · ESLint `import/no-restricted-paths` (TS) | declared |
| CI provider | GitHub Actions | declared |
| Deploy target | Render (lean, not locked) | declared |

## Floor expression & deck rulings

Floor cards enter asserted (level not negotiated); their rows record *expression* (type
translation). Arbitrated cards record the user's ruling; dropped cards would be rulings too.

| GI-ID | Card | Layer | Ruling / Expression | Mark |
|-------|------|-------|---------------------|------|
| GI-003 | FLOOR-SEC | floor-asserted | At floor level, expressed for FastAPI/React: secrets out of repo (env vars + `.gitignore`), secret scanning blocks CI, input validation at all API boundaries (pydantic) and UI boundaries (zod), auth enforced at every API boundary, dependency vuln scanning blocks merge. Tenant isolation (GI-012) and secret-commit blocking (GI-010/GI-013 kin) are the founder's emphasized expressions of this floor. | Confident |
| GI-004 | FLOOR-TEST | floor-asserted | Category kept, expression per waiver GI-014: numeric coverage gate waived; **required critical-path tests** (payment state machine, auth, tenant isolation) enforced in CI; coverage ratchet (MUST NOT decrease) retained; day-one smoke test on the critical path. | Confident |
| GI-005 | FLOOR-ERR | floor-asserted | At floor level, expressed as an API error schema (RFC 7807 Problem Details) for FastAPI + UI error states for React; failures never silently corrupt data (**especially payment state — GI-010**); correlation/trace IDs on error responses; no stack traces leaked to users; **no PII or invoice amounts in error payloads** (ties GI-013). | Confident |
| GI-006 | FLOOR-OBS | floor-asserted | At floor level, expressed for the stack: structured JSON logs with standard fields, correlation IDs, a `/health` endpoint (FastAPI), Sentry for error tracking. **No PII (names/emails/addresses) or invoice amounts in logs or Sentry payloads** (GI-013). Observability *beyond* this — SLOs, formal targets — is excluded for now (GI-020). | Confident |
| GI-007 | BE-HEX | arbitrated | **KEPT (pragmatic).** Stripe and PostgreSQL behind port interfaces; invoice/payment domain logic isolated so payment-state behavior is testable without calling real Stripe; enforced by an import-linter in CI. Founder's stated reason: "the one place I'll pay the upfront-structure tax" — it buys tested payment correctness. Selects the `layer-rules` module (GI-015). | Confident |
| GI-008 | BE-SRP | arbitrated | **KEPT (scoped).** Cyclomatic-complexity gate (`ruff` C901, ≤10) blocks CI; "no utils/helpers dumping ground" rule kept. File-length and parameter-count limits are **advisory only** (no numeric CI gate) — founder refuses bikeshedding with no reviewer to sanity-check. | Confident |
| GI-009 | BE-DEP | arbitrated | **KEPT.** `pip-audit` / `npm audit` in CI block merge on high/critical vulnerabilities; lock files committed; external calls via ports (pairs with GI-007). Founder: "exactly the security corner I'm weak on — I want the pipeline doing this for me." | Confident |

## Minted principle intents

One element per minted intent — traced to the elicited answer, never to prompting:

- **GI-010 — Payment-state integrity:** Stripe is the single source of truth for payment
  status; the app's idea of "paid" MUST reconcile to Stripe and MUST NOT flip an invoice to
  paid via a manual/guessed code path. Stripe webhook handling MUST be idempotent — a
  duplicate Stripe event MUST NOT double-count a payment or double-send a "you're paid"
  notification. Manual mark-as-paid is a real state transition logged with actor + timestamp,
  never a silent field flip. **CI MUST block** any change touching invoice status or payment
  amounts that lacks a test proving the state machine behaves. **Periodic reconciliation
  against Stripe (cold-review survivor S4, founder ruling):** idempotent handlers alone do not
  catch a webhook Stripe delays or drops; a scheduled reconciliation against Stripe (the source
  of truth) MUST recover missed events so an invoice cannot sit "unpaid" while Stripe shows it
  paid. The reconciliation job's design is a plan-time detail; the requirement is in scope now.
  · **Mark:** Confident
  *Elicited from:* dim 9 — "I never want the app's idea of 'paid' to drift from Stripe's… if a
  change touches invoice status or payment amounts and there's no test proving the state
  machine behaves, CI should block it. I'd rather ship late than ship a payment bug." (S4
  reconciliation folded from cold review — "a dropped webhook leaving an invoice stuck 'unpaid'
  is exactly the failure I'm trying to avoid.")
- **GI-011 — Currency exactness:** money MUST be represented as integer cents; floating-point
  types MUST NOT be used for currency amounts or arithmetic. · **Mark:** Confident
  *Elicited from:* dim 9 — "money math is exact, no floats for currency. Cents as integers."
- **GI-012 — Tenant isolation:** every query for a contractor's data MUST be scoped to that
  contractor; one account MUST NOT be able to read or affect another's invoices or clients.
  Data-access scoping is non-negotiable and MUST be tested (a required critical path, GI-014).
  · **Mark:** Confident
  *Elicited from:* dim 9 — "every query for a contractor's data is scoped to that contractor;
  one account seeing another's invoices is the nightmare, so tenant isolation on data access is
  non-negotiable."
- **GI-013 — No customer data in logs or telemetry:** PII (names, emails, mailing addresses)
  and invoice amounts MUST NOT appear in log lines or in error payloads sent to Sentry.
  Strengthens FLOOR-OBS/FLOOR-ERR expression; minted because the founder named it a hard rule.
  · **Mark:** Confident
  *Elicited from:* dim 9 — "no PII or invoice data in logs… stay out of log lines and out of
  error payloads that go to Sentry."
- **GI-024 — Data durability & recoverability** *(folded from cold-review survivor S5, founder
  ruling):* the database MUST have automated daily backups with point-in-time recovery enabled,
  and a periodic restore check MUST verify the backups actually restore. The retention window
  is a plan-time detail; the obligation lands now. Rationale: data loss of contractors'
  financial records is the founder's #1 worst-case, and managed Postgres (Render) provides
  backups + PITR at near-zero cost. · **Mark:** Confident
  *Elicited from:* cold-review routing — "losing contractors' financial records is my #1 worst
  case and nothing governed backups… require it now — daily backups, PITR on, and a periodic
  restore check so I know the backups actually restore."

## Waivers

Any asserted standard may be waived with a recorded justification (D4) — except legal-mandate
module obligations (D4.2). Waivers are permanent pending the D4.1 revisit:

| GI-ID | Standard | Justification | Revisit trigger (optional) | Mark |
|-------|----------|---------------|---------------------------|------|
| GI-014 | FLOOR-TEST numeric coverage-percentage gate (≥80% warning / ≥60% blocking) | Solo founder refuses coverage-percentage theater; risk-targeted testing preferred over a vanity number. **The category is not dropped** — it is replaced by a REQUIRED critical-path test rule: the payment state machine, auth, and tenant-isolation paths MUST have tests enforced in CI. The ratchet (coverage MUST NOT decrease) and a day-one smoke test are retained. | When the team grows past solo, or a SOC 2 / attestation obligation is signed | Confident |

## Module selections

Template modules ruled in session; compliance modules attach mechanically in the Fact profile
above (a11y), never here. Declines are rulings too (durable; amend re-offers only unruled
modules).

| GI-ID | Module | Ruling | Because | Mark |
|-------|--------|--------|---------|------|
| GI-015 | layer-rules | adopted | BE-HEX kept (GI-007) | Confident |
| GI-016 | knowledge-management (core) | adopted | Offered default-on at dim 7; founder: "adopt it — decisions log and backlog are the parts I actually need; it's just me, if it's not written down it's gone." Architecture doc + brainstorm records kept but low-touch. | Confident |
| GI-016a | knowledge-management → CHANGELOG (elective) | adopted | "Cheap, and I'll want release notes once there are customers." | Confident |
| GI-016b | knowledge-management → RUNBOOK (elective) | **declined (deferred)** | "I don't know what goes in one for a one-person shop — hold off until I've deployed and know what breaks." Re-offer at the amend run after first deploy. Also memorialized in dim 10 exclusions. | Confident |
| GI-017 | release-gates | adopted | Offered default-on at dim 8 (deployed product). Formalizes the founder's "pipeline is the reviewer" bar: a release is blocked unless tests pass, lint passes, and type checks pass. | Confident |
| GI-025 | accessibility (WCAG) standard | adopted (scopable, **not** legal-mandate) | Reclassified from the DECK-1 `a11y` legal-mandate by cold-review survivor S3 + founder ruling: commit to WCAG 2.1 AA on core contractor flows (invoice create/send, payment-status view), scopable/sequenceable by a solo founder, waivable under D4 (not D4.2). The accessibility work stands; only the unwaivable classification was removed. | Confident |

### Recorded constraints (not modules)

| GI-ID | Constraint | Enforcement / watch | Mark |
|-------|-----------|---------------------|------|
| GI-026 | Card data MUST never touch Ledgerline servers (preserve PCI SAQ-A eligibility) | Stripe-hosted checkout only; any feature that would collect or proxy card data is a governance event that re-opens the PCI fact and attaches `pci-dss`. Cold-review survivor S2 + founder ruling. | Confident |

## Domain-dependency seeds (layer-rules adopted → GI-015)

Session-arbitrated registry seeds — domain-relevance filtered, trust-ranked, level-cited:

| GI-ID | Dependency | Signal level | Ruling | Mark |
|-------|------------|--------------|--------|------|
| GI-018 | `pydantic` (Python — validation / value objects) | 4 — quantitative (ecosystem-standard adoption, verified publisher on PyPI), live-verified at seed time | kept — "going in anyway" | Confident |
| GI-019 | `zod` (TypeScript — runtime validation / value objects) | 4 — quantitative (ecosystem-standard adoption), live-verified at seed time | kept — "going in anyway" | Confident |

*Money path: integer cents is the ruled representation (GI-011); Python's stdlib `Decimal`
needs no registry entry and may be used where helpful, but cents-as-integer is the rule. No
third-party decimal library is seeded.*

## Deliberate exclusions (dimension 10)

- **GI-020:** Observability *beyond* error tracking + structured logs + `/health` — SLOs and
  formal observability targets are not governed now. The founder cannot yet say what SLOs look
  like for a one-person company and won't have a gate demand them. (Floor-level OBS expression
  in GI-006 is unaffected — this excludes only the beyond-floor layer.) · **Mark:** Confident
- **GI-021:** Formal incident-response process — not governed now (beyond floor; one-person
  shop). · **Mark:** Confident
- **GI-022:** Data retention & deletion policy — **not authored; flagged as a real open
  question, NOT a launch blocker.** The founder suspects a legal record-retention requirement
  exists (US financial records) but cannot source it and refuses to fake a policy to satisfy a
  checklist. Recorded as an open thread for the founder to resolve before it bites; a governance
  gate is deliberately withheld until the requirement is sourced. **Revisit trigger:** before
  scaling, or as soon as the legal retention requirement is sourced. · **Mark:** Deferred
- **GI-023 (RUNBOOK decline, memorialized):** operational runbook deferred until after first
  deploy (see GI-016b). · **Mark:** Confident

## Review

**2026-08-10 — first ratification**

- **Sizing:** lead stated weight — 23 GI elements, mark mix almost entirely `Confident` (one
  `Deferred`: GI-022), reality-surface load moderate (one legal-mandate module attached by
  inference from a fact, one floor waiver with a substitute, several minted payment/security
  principles). The default on first ratification is a **pair**; **lead sized: pair** —
  composed in the run plan because the a11y-by-inference attachment and the FLOOR-TEST waiver
  are the two highest-leverage, most-challengeable rulings and warrant a coverage lens plus a
  coherence lens.
- **Review:** cold reviewer, blind-map dispatch, coverage+coherence lenses in one seat (single
  reviewer in this simulation — see deviation in `meta.json`; the default pair was reduced to
  solo because the run is a single-agent simulation). Blind angle map built from the topic
  alone before the synthesis path was sent; the map diff drove the coverage findings.
  **Tally: 7 raised → 5 survived** (2 fell: CAN-SPAM/email-deliverability for transactional
  reminder emails — largely exempt, low-regret to defer; and a BE-HEX adoption-streak /
  "pragmatic-hexagonal under-specified" note — formulation quality, out of jurisdiction per
  design record D1). **Recommended status: needs-revision** — every survivor was resolvable by
  the session, none were critical-gaps.
- **Survivor dispositions** (every survivor carries one):

  | # | Sev | GI element(s) | Finding | Disposition |
  |---|-----|---------------|---------|-------------|
  | S1 | Important | GI-001 | "US only" treated as clearing ALL privacy regimes; CCPA/CPRA + VA/CO/CT state privacy laws unexamined | user-ruled → recorded reasoned negative (below thresholds, not governed now, revisit trigger = scale / new consumer-facing data collection) |
  | S2 | Important | GI-001 → GI-026 | PCI-avoidance recorded as a point-in-time fact, not a rule that must stay true | user-ruled → resolved: recorded as enforced constraint GI-026 + temporal-backstop watch |
  | S3 | Important | GI-001 → GI-025 | Accessibility attached as unwaivable legal-mandate on a contestable ADA legal claim | user-ruled → resolved: reclassified to adopted scopable WCAG 2.1 AA standard (GI-025), commitment retained |
  | S4 | Minor | GI-010 | Idempotent webhooks don't catch a dropped/delayed Stripe webhook (invoice stuck "unpaid") | user-ruled → resolved: periodic Stripe reconciliation folded into GI-010 |
  | S5 | Minor | GI-024 | Data loss (#1 worst-case) ungoverned — no backup/PITR/restore-check | user-ruled → resolved: minted GI-024 (daily backups, PITR, periodic restore check) |

- **Verify pass:** PASS — the five folds were re-read against the survivor rulings by the
  (solo) reviewer seat; each disposition matches the founder's ruling, and no fold introduced a
  cross-element contradiction. S1's reasoned negative carries its revisit trigger; S2/S3/S4/S5
  landed as new/edited GI elements with marks. Synthesis is internally consistent post-fold.
- **Ratification:** the founder ratified the synthesis as amended by S1–S5 at the
  synthesis-confirmation checkpoint (2026-08-10).

## Amendment Log

[Empty on first ratification.]
