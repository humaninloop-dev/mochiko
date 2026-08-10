# Governance Intent — Ledgerline

**Session date:** 2026-08-10 · **Mode:** greenfield
**Confirmed at synthesis checkpoint:** 2026-08-10 by the founder (principal)
**Governs:** the governance surface set v1.0.0 (CLAUDE.md governance region · `.claude/rules/mochiko/` · `.mochiko/memory/governance-ledger.md`)

## Fact profile

The module-driving facts (dimension 2), each with its consequence-stated confirmation — negatives
are recorded facts too (the S4 fail-safe):

- **GI-001 — Facts:** industry: SaaS, small-business/contractor finance (not a regulated
  industry) · data classes: contractor & client PII (names, emails, addresses) + financial
  records (invoice amounts, payment status) — **no cardholder data** (Stripe hosted checkout) ·
  jurisdictions/markets: US only, no international at launch · contractual commitments: none
  binding (a prospect mentioned SOC 2; nothing signed) · **Mark:** Confident
- **Modules triggered (mechanical):**
  - `a11y` (WCAG) — **ATTACHED**, legal-mandate. Trigger fact: a customer-facing React UI served
    to US users → US accessibility statute (ADA). Obligation scoped to the automated-check
    baseline by the user (below, GI-010).
  - Negatives confirmed with consequence:
    - No cardholder data (Stripe hosted checkout) → `pci-dss` will NOT attach.
    - No health/medical data → `hipaa` will NOT attach.
    - US-only, no EU/UK residents → `gdpr` will NOT attach. **Founder-confirmed at cold-review
      ratification (F2):** the *data subjects* are the contractors' own local US clients, not just
      the US contractors — she is not targeting non-US contractors and does not expect EU/UK
      residents' PII at launch. **Revisit trigger:** going international, or onboarding contractors
      who bill EU/UK clients, re-opens `gdpr`.
    - No signed attestation commitment → `attestation` will NOT attach now; **revisit trigger:**
      a customer contract requiring SOC 2 (see GI-018).

## Project identity & type

- **GI-002 — Type:** fullstack → shelves dealt: universal-floor (asserted) + backend-service
  (arbitrated; API side). Frontend shelf floor-examples are backend-flavored (catalog seed note)
  — floor categories translated to the React surface in session. · **Mark:** Confident
- **Identity:** Ledgerline — an invoicing and payment-tracking SaaS for US small independent
  contractors (create clients, issue invoices, track payment, send reminders, cash-flow view).
  Founder-owned, built to last, production quality from day one because it holds real financial
  data.
- **Risk surface:** worst case = leaking or losing contractor financial data → instant trust
  loss → business death (the top-of-mind risk). Payment-status accuracy is the second: a "paid"
  that isn't drives the contractor's decisions on bad numbers. Money movement itself is
  lower-risk (Stripe owns the charge). Legal exposure acknowledged but unquantified by the user.
- **Team reality:** solo founder, full-time, mid-level, 6 years backend Python; never run a
  production SaaS solo. A friend contributes ~5 h/week on React. No ops, no on-call but the
  founder, no code-review culture. **Enforcement must lean on tooling/CI, never on review**, and
  the product must stay operable and shippable by one person.

## Convergence skips

None — all ten dimensions were worked.

## Real commands (dimension 6/8 → the validator's placeholder bar)

| Purpose | Command | Source |
|---------|---------|--------|
| Lint + format (Python) | `ruff check .` · `ruff format --check .` | declared |
| Type-check (Python) | `mypy .` (strictness starts non-strict, ratcheted up over time — F8) | declared/recommended (accepted) |
| Test + coverage (Python) | `pytest --cov --cov-fail-under=60` | declared |
| Floor-enforcing tests (in `pytest`) | cross-account authz-denial test (GI-003, F1) · error-response no-stack-trace test (GI-005, F5) · no-PII-in-logs test (GI-006, F4/F5) | added at cold review (floor obligations need gates, not review) |
| Lint + format (frontend) | `eslint .` · `prettier --check .` | recommended (accepted) |
| Test (frontend) | `vitest run` | recommended (accepted) |
| Secret scan | `gitleaks detect` | recommended (accepted) |
| Dependency scan (Python) | `pip-audit` | declared/recommended (accepted) |
| Dependency scan (JS) | `npm audit --audit-level=high` | declared/recommended (accepted) |
| Accessibility (automated) | axe-based checks in CI (e.g. `@axe-core/cli` / vitest-axe) against WCAG 2.1 AA | recommended (a11y module baseline) |
| Import boundaries | `import-linter` (Python), scoped to the Stripe + DB port seams only | recommended (BE-HEX, tightened) |
| CI | GitHub Actions, all gates blocking merge to `main` | declared/recommended (accepted) |

## Floor expression & deck rulings

Floor cards enter asserted — rows record *expression* (type translation), never a level ruling;
arbitrated cards record the user's ruling.

| GI-ID | Card | Layer | Ruling / Expression | Mark |
|-------|------|-------|---------------------|------|
| GI-003 | FLOOR-SEC | floor-asserted | At floor level. Expression: secrets out of repo (env vars + `.gitignore`), gitleaks in CI; input validation at FastAPI boundaries (pydantic); authentication enforced at all API boundaries (contractors log in) **and object-level authorization scoping every data read/write to the owning account** — no contractor can reach another account's clients, invoices, or payment records (the single-tenant-per-account guarantee is a security floor obligation, not just a product feature; broken object-level access is the most common vector for the top-of-mind financial-data-leak risk). Dependency vulnerability scanning (pip-audit / npm audit) blocking merge. Financial-data confidentiality is the load-bearing case. **Enforcement (per GI-013, tooling not review):** pytest integration tests asserting cross-account access is denied, on the invoicing critical path. | Confident |
| GI-004 | FLOOR-TEST | floor-asserted | At floor level, session-overridable pre-seed kept: coverage ≥80% warning / ≥60% blocking on new code; ratchet (baseline MUST NOT decrease); a smoke test on the invoicing critical path from day one. Greenfield ratchet baseline starts at first-code coverage. | Confident |
| GI-005 | FLOOR-ERR | floor-asserted | At floor level. Expression: failures never silently corrupt invoice/payment data; consistent API error surface (RFC 7807 problem+json from FastAPI) + React error states; correlation IDs; no stack traces leaked to clients. **Enforcement (per GI-013):** a pytest test asserting error responses carry no stack trace / internal detail. | Confident |
| GI-006 | FLOOR-OBS | floor-asserted | At floor level, right-sized to solo. Expression: structured JSON logs with correlation IDs; **no PII in logs, enforced by a log-scrubbing helper + a test asserting known PII/financial fields never serialize into a log line** (the no-review reality means this floor obligation needs a gate, not discipline — F5); a health-check endpoint (`/health` or `/healthz`) for Render; error tracking via Sentry **configured to scrub PII and financial payloads before egress (`before_send` / no request bodies) — Sentry is a third-party processor and the no-PII-in-logs rule extends to it (F4)**. (Formal SLOs excluded — GI-020; not a floor-category waiver.) | Confident |
| GI-007 | BE-HEX | arbitrated | **KEPT, tightened** — ports around Stripe and the database only; no import-linter ceremony on the web layer or elsewhere. Reason: test invoicing logic without live Stripe/DB, and keep the Render/Stripe seams swappable. import-linter enforcement scoped to those two seams. | Confident |
| GI-008 | BE-SRP | arbitrated | **KEPT** — cyclomatic ≤10 (start there, session-tunable; user may bump for legitimately gnarly code), no "utils" dumping grounds. Enforced by the linter in CI. | Confident |
| GI-009 | BE-DEP | arbitrated | **KEPT**, unchanged — new deps justified, versions pinned/locked, vuln scanning blocks merge on high/critical. | Confident |

## Compliance-module obligations (attached mechanically — Fact profile)

| GI-ID | Module / stratum | Obligation (as ruled) | Mark |
|-------|------------------|-----------------------|------|
| GI-010 | `a11y` (WCAG) / legal-mandate | **Obligation: customer-facing UI meets WCAG 2.1 AA (full — the legal-mandate obligation is unwaivable, D4.2).** Day-one *enforcement* is automated axe-based checks in CI, which catch a known subset of AA failures — they are the gating floor, NOT full verification of AA (automation alone cannot confirm AA; F3). The residual gap between axe coverage and full AA is an **open obligation**, not a closed one: a manual AA audit is a tracked pre-scale item (BACKLOG), not a silent partial waiver. | Confident |

## Minted principle intents

One element per minted intent — traced to the elicited answer, never to prompting:

- **GI-011 — Payment-status integrity:** payment/invoice state MUST be correct and never report a
  false "paid"; the invoicing core is the code the founder "won't ship shaky." Enforce via tests
  on the payment-status state machine and the invoicing critical-path smoke test; the invoicing
  core must be green before a release. · **Mark:** Confident
  *Elicited from:* dimension 9 — "Payment status has to be accurate; if the app says paid and it
  isn't, I'm done" (and dimension 8 — "nothing broken in the invoicing core").
- **GI-012 — Financial-data durability:** contractor financial data MUST NOT be lost — distinct
  from the FLOOR-SEC leak-prevention case. Enforce via automated database backups with a verified
  restore path (backup existence + periodic restore check). · **Mark:** Confident
  *Elicited from:* dimension 9 — "financial data does not leak and does not get lost"; dimension 4
  — losing the data "kills trust instantly."
- **GI-013 — Solo-operable, ship-frequently enforcement:** governance enforcement MUST fit a
  one-person shop — automated (CI/hooks/tooling), never dependent on a second reviewer — and gates
  MUST be right-sized so they do not block shipping several times a week (e.g. the staging soak is
  smoke-test-level and skippable for small changes; complexity limits are tunable). · **Mark:**
  Confident
  *Elicited from:* dimension 9 — "operable and shippable by one person … I'd refuse gates so
  strict or slow I can't ship" (and dimension 5/6b — "the machine is the reviewer").

## Waivers

| GI-ID | Standard | Justification | Revisit trigger | Mark |
|-------|----------|---------------|-----------------|------|
| — | None. | All four floor categories are principled; no floor standard was waived. | — | — |

## Module selections

**Template modules** (compliance modules attach mechanically in the Fact profile above, not here):

| GI-ID | Module | Ruling | Because | Mark |
|-------|--------|--------|---------|------|
| GI-014 | knowledge-management | adopted (core + electives: CHANGELOG **adopted**, RUNBOOK **adopted as skeleton**) | Offered default-on at dimension 7; user: "keeping a long-lived thing coherent in one head is exactly my problem." CHANGELOG "cheap, add it." RUNBOOK skeleton adopted (low-effort scaffold under the never-overwrite floor; deployed service operated solo — a runbook fits the one-operator case), filled post-launch. **Even at skeleton, it carries a minimal data-incident stub** (backup-restore steps for GI-012, Sentry error triage) so a day-one financial product is not left with zero playbook while formal IR stays deferred (GI-020, F7). | Confident |
| GI-015 | release-gates | adopted | Deployed/operated target class (always offered); user supplied content — staging→prod on Render, expand-contract migrations, rollback < 15 min, destructive-migration flag + explicit approval. Soak kept smoke-test-level / skippable (GI-013). | Confident |
| GI-016 | layer-rules | adopted | BE-HEX kept (GI-007). Scoped to the tightened two-seam expression: import rules around the Stripe and DB port boundaries, not full multi-layer ceremony. | Confident |

## Domain-dependency seeds (layer-rules adopted)

Domain-relevance filtered, trust-ranked; the tightened two-seam scope keeps the registry minimal:

| GI-ID | Dependency | Signal level | Ruling | Mark |
|-------|------------|--------------|--------|------|
| GI-017 | `pydantic` | 1 — ecosystem standard (FastAPI-native; domain modeling/validation without I/O; >80% adoption) | kept | Confident |

*(Registry deliberately minimal per the user's "no ceremony" tightening of BE-HEX; grows at
implement time via the domain-dependency add-process under human ruling.)*

## Deliberate exclusions (dimension 10)

- **GI-018:** No SOC 2 / attestation work until a customer contract requires it. (Not a floor
  category; ties to the attestation-module revisit trigger in GI-001.) · **Mark:** Confident
- **GI-019:** No multi-tenancy / teams governance — single-tenant-per-account at launch; a
  bookkeeper seat is deliberately not foreclosed for later. · **Mark:** Confident
- **GI-020:** No formal SLOs or incident-response process for now — error tracking via Sentry;
  ops-maturity revisited after launch. (FLOOR-OBS is met at its asserted level, GI-006; this
  excludes only beyond-floor SLO/IR ceremony, not an observability-category waiver.) · **Mark:**
  Confident
- **GI-021:** Data retention / deletion policy is **not** hard-coded into governance now — the
  user does not yet know the legal obligations (invoice retention duration, account-deletion
  behavior). Recorded as an **open question to resolve before launch**, NOT a gate today, and NOT
  a floor waiver. **The pre-launch resolution MUST cover the interaction with GI-012 durability
  backups** (F6): backups retain data the deletion policy may later require purging, so
  "delete my account" must reconcile with backup retention windows — decided together, not
  separately. · **Mark:** Deferred

## Review

<!-- Durable record of the sized pre-G3 intent review. -->

**2026-08-10 — first ratification**

- **Sizing:** lead stated weight — 21 GI elements, mark mix mostly Confident with one Deferred
  (GI-021), reality-surface load low (greenfield, no codebase to cross-check; user-declared facts
  dominate). Default on first ratification is a **pair**; **lead sized: solo** (departure below).
  - *Departure trail:* sized down from the default pair to a **single** cold reviewer. Rationale:
    greenfield with no codebase means no reality-surface fact substrate for a second lens to work;
    the synthesis is compact and the highest-risk stretch (card arbitration) produced explicit,
    reasoned user rulings rather than passive acceptances. A single coherence+coverage reviewer
    covers the surface; the user retains ratification.
- **Review:** COMPLETE — one cold reviewer, blind-map dispatch (topic-only angle map built before
  the synthesis path was sent; the diff yielded the coverage read). Recommended status:
  **needs-revision** — 11 raised, 8 survived the reviewer's own cross-check. Blind-map coverage
  result: all ten interrogation dimensions were worked (no wholly missed dimension); survivors are
  depth / consistency / mechanical-attachment gaps, not missed dimensions. Reviewer stayed in
  jurisdiction — did not challenge floor LEVEL or authored-principle formulation; user-declared
  facts were routed as confirmation questions, not arguments.

  **Survivors and dispositions** (author's disposition; the revised synthesis then goes to the
  founder for ratification — F2 additionally carries a fact question only she can answer):

  | # | Sev | Finding | Disposition |
  |---|-----|---------|-------------|
  | F1 | Important | AuthZ / cross-tenant isolation absent — FLOOR-SEC covered authN only; the #1 named risk (financial-data leak) via its most common vector (broken object-level authz / IDOR). GI-019 excludes only multi-user-per-account, not cross-account isolation. | **ACCEPTED** — folded into GI-003 as object-level authorization scoping every access to the owning account, with a pytest cross-account-denial gate. Floor expression, not new scope (the card's per-account isolation is a security obligation). |
  | F2 | Important | GDPR declined on a market fact ("US only") that conflates *users* (US contractors) with *data subjects* — the client PII a contractor stores (names/emails/addresses) may include EU/UK residents. | **ROUTED TO PRINCIPAL** — a genuine fact question, not the author's to decide: does Ledgerline store PII of clients who may be EU/UK residents? Her ruling below determines whether the `gdpr` negative stands (with a data-subject-residency revisit trigger) or the module attaches. |
  | F3 | Important | a11y legal-mandate scoped to automation-only axe checks, which cannot *verify* WCAG 2.1 AA — reads as a hidden partial waiver of an unwaivable (D4.2) obligation. | **ACCEPTED** — GI-010 reworded: obligation stays full AA; axe checks are the day-one enforcement floor (a known subset), the residual AA gap is an open tracked obligation (BACKLOG), not a silent waiver. Honesty fix, no level change. |
  | F4 | Important | Sentry vs "no PII in logs" + financial-data confidentiality — error payloads egress PII/invoice data to a third-party processor. | **ACCEPTED** — GI-006 now requires Sentry PII/financial scrubbing before egress (`before_send`, no request bodies); the no-PII rule extends to the third-party processor. |
  | F5 | Important | "Never on review / the machine is the reviewer" leaves floor obligations (auth-at-boundaries, no-PII-in-logs, no stack traces) that no listed gate enforces — silently falls to solo-founder discipline, contradicting GI-013. | **ACCEPTED** — added the three floor-enforcing pytest gates to the real-commands table and to GI-003/005/006 expression, so each floor obligation has a tooling gate, not discipline. |
  | F6 | Minor | Retention-deferral (GI-021) vs durability-backups (GI-012) tension — backups retain data a future deletion policy may need to purge. | **NOTED** — GI-021's pre-launch resolution now must reconcile with backup retention windows. No gate today. |
  | F7 | Minor | No-IR / empty-runbook (GI-020) vs day-one financial framing — zero incident playbook for a product holding financial data. | **NOTED** — GI-014 RUNBOOK skeleton now carries a minimal data-incident stub (backup-restore, Sentry triage); formal IR stays deferred. |
  | F8 | Minor | Wholesale toolchain acceptance / unspecified mypy strictness — accepted-on-recommendation without depth; strictness changes the gate's teeth. | **NOTED** — mypy strictness recorded as start-non-strict, ratchet up. |

- **F2 principal fact-ruling:** RESOLVED — founder confirmed the data subjects are the
  contractors' own local US clients, not EU/UK residents; no non-US targeting at launch. `gdpr`
  negative stands, now with a data-subject-residency revisit trigger recorded in GI-001. The
  reviewer's conflation concern is answered by the fact, not by attaching the module.
- **Verify pass:** solo reviewer covers it — the four Important folds are floor-expression
  strengthenings traceable to the finding that prompted them; no new dimension opened, no floor
  level moved. Verify deferred to the independent validation seat (author ≠ grader) which reads
  the authored surfaces against the checklist.
- **Ratification:** ACCEPTED — founder ratified the revised synthesis in full, no amendments
  (all four Important folds and three Minor notes accepted; F2 answered as above). One acceptance
  condition recorded: the WCAG manual-audit gap (F3) is backlog-tracked, **not** a launch gate.
- **Status after fold:** needs-revision findings dispositioned and folded; synthesis revised and
  **ratified by the founder 2026-08-10.** Synthesis is frozen for authoring.

## Amendment Log

[Empty on first ratification.]
