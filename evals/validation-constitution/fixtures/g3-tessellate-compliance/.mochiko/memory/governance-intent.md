# Governance Intent — Tessellate

**Session date:** 2026-09-09 · **Mode:** greenfield
**Confirmed at synthesis checkpoint:** 2026-09-09 by Inês Carvalho (founder)
**Governs:** the governance surface set v1.0.0 (CLAUDE.md governance region · `.claude/rules/mochiko/` · `.mochiko/memory/governance-ledger.md`)

## Fact profile

- **GI-001 — Facts:** industry: consumer e-commerce (a two-sided marketplace) · data classes: personal data of customers and makers (names, addresses, e-mail, order history, makers' bank details for payouts); cardholder data — the primary account number passes through Tessellate's own checkout handler (`src/checkout/`) before the Adyen tokeniser in `src/payments/` · jurisdictions/markets: EU and UK · contractual commitments: none · **Mark:** Confident
- **Modules triggered (mechanical):** `gdpr` — personal data of EU and UK residents · `pci-dss` — cardholder data handled (SAQ D scope; the PAN touches Tessellate's servers) · `a11y` — customer-facing UI in the EU (European Accessibility Act, in force since 2025-06-28). Negatives confirmed with consequence: no health data → `hipaa` does not attach; no attestation commitments → `attestation` does not attach.

## Project identity & type

- **GI-002 — Type:** fullstack web (Remix server + browser UI, PostgreSQL) → shelves dealt: universal floor · backend-service · **Mark:** Confident
- **Identity:** a marketplace where independent ceramicists list pieces and customers across the EU and UK buy them; Tessellate takes payment, prints labels, and handles VAT. Founder plus six engineers; launch in two weeks.
- **Risk surface:** card data in scope; a leaked PAN or a cross-maker payout error is existential. Personal data of buyers and makers under two GDPR regimes.
- **Team reality:** seven people, PR review on every change, GitHub Actions CI, one on-call engineer. Enforcement leans on CI, hooks, and a non-author reviewer.

## Depth level declaration

- **GI-003 — Declared level:** high · **Declared:** 2026-09-09 by Inês Carvalho · **Mark:** Confident
- **Rationale:** card data in scope and a consumer launch in two weeks; the founder ruled the high row on from day one. Setup recommended `high`; the user ruled `high`.
- **Ratchet:** one-way — `high` is terminal.

## Convergence skips

None.

## Real commands

(dimension 6/8 → the validator's placeholder bar)

| Purpose | Command | Source |
|---------|---------|--------|
| Lint | `npm run lint` | detected |
| Test | `npm test` | detected |
| Coverage | `npm run test:cov` | detected (`vitest --coverage`; `--coverage.thresholds.lines` set in CI) |
| End-to-end | `npm run e2e` | detected (Playwright) |
| Accessibility | `npm run a11y` | detected (axe-core over every route) |
| Performance | `npm run lhci` | detected (Lighthouse CI, product and checkout pages) |
| Secret scan | `npm run scan:secrets` (gitleaks) | declared (pre-commit + CI job `secrets`, blocking) |
| Dependency advisories | `npm audit --audit-level=high` | declared (CI job `audit`, blocking) |
| Build | `npm run build` | detected |
| Deploy | GitHub Actions → ECS (`staging` on merge, `prod` by manual promotion) | declared |

## Floor expression & deck rulings

| GI-ID | Card | Layer | Ruling / Expression | Mark |
|-------|------|-------|---------------------|------|
| GI-004 | FLOOR-SEC | floor-asserted | High row: secrets in AWS Secrets Manager, `.env*` gitignored · Zod validation on every input · auth at every HTTP boundary (`requireSession` / `requireMaker`; public routes listed with a reason) · `gitleaks` blocks merge · `npm audit --audit-level=high` blocks merge at high/critical. Universal — every layer, every route. | Confident |
| GI-005 | FLOOR-TEST | floor-asserted | High row at floor level: coverage thresholds at the card's row — ≥80% warning, ≥60% blocking — **no session override**; ratchet; `npm test` + `npm run e2e` on the checkout path from day one. | Confident |
| GI-006 | FLOOR-ERR | floor-asserted | High row: no silent corruption of orders, payments, payouts (every write transactional; payouts computed from settled amounts) · RFC 7807 on every error response with `correlation_id` · no leaked stack traces. | Confident |
| GI-007 | FLOOR-OBS | floor-asserted | High row: structured JSON logs (`pino`) · `correlation_id` on every line · no personal data and no card-number fragment in logs (redaction list + a PAN-pattern log test) · `/healthz` on every service. | Confident |
| — | BE-HEX | arbitrated | **dropped** — "Remix's route modules are our layers; we are not adding a hexagon around them" (Inês). No `layer-rules` module. | Confident |
| — | BE-API-ERR | arbitrated | **folded into GI-006** — RFC 7807 is the FLOOR-ERR high-row expression here; no separate principle. | Confident |

## Compliance module obligations (from GI-001)

- **GI-008 — gdpr obligations (legal-mandate, unwaivable):** lawful-basis record and retention class per data class · subject access, portability, and erasure within 30 days · erasure propagates to replicas and backups within 90 days · breach reported to the lead supervisory authority (CNPD) within 72 hours · **Mark:** Confident
- **GI-009 — pci-dss obligations (legal-mandate, unwaivable):** PAN never logged, persisted, cached, or queued; in memory only between the checkout handler and the tokeniser · PAN-handling functions named and unexported · card data confined to the checkout handler and the tokeniser — **scope: `src/checkout/` and `src/payments/`, the two modules the PAN transits** · quarterly ASV scans and an annual SAQ D filed under `compliance/pci/` · **Mark:** Confident
- **GI-010 — a11y obligations (legal-mandate, unwaivable):** every customer-facing page meets WCAG 2.2 AA · `npm run a11y` blocks merge on serious or critical axe findings · an accessibility statement is published before launch · **Mark:** Confident

## Minted principle intents

- **GI-011 — Fast product pages:** "Largest Contentful Paint at or under 2.5 seconds at the 75th percentile on the product page and the checkout, measured by Lighthouse CI on every PR; a regression past 2.5 s blocks merge." · **Mark:** Confident
  *Elicited from:* dimension 9 — the founder's own number, from the previous marketplace's bounce data.

## Waivers

| GI-ID | Standard | Justification | Revisit trigger | Mark |
|-------|----------|---------------|-----------------|------|
| GI-012 | pci-dss quarterly ASV scan (Requirement 11.3.2) | Too expensive pre-launch; the first paying customer funds it | first paying customer | Contested — the lead recorded the D4.2 unwaivability challenge; the founder overruled with the steelman in view: "we will fix it at the gate if we have to" |

## Module selections

| GI-ID | Module | Ruling | Because | Mark |
|-------|--------|--------|---------|------|
| GI-013 | release-gates | adopted | Deployment dimension: ECS `staging` on merge, `prod` promoted manually after a four-hour soak; Playwright e2e against `staging` and the axe run block promotion; rollback is an ECS task-definition rollback within 15 minutes. Region carries one summary line; detail in the ledger. | Confident |
| GI-014 | knowledge-management | adopted — core, whole; elective `CHANGELOG.md` adopted; `RUNBOOK.md` declined | Offered default-on at dimension 7; the team wants the operating-docs layer from launch. `CHANGELOG.md` exists already and is codified into the elective role. Runbooks live in PagerDuty. | Confident |
| GI-015 | layer-rules | declined | BE-HEX dropped; no layered intent minted. | Confident |
| — | evolution-notes | n/a | Greenfield. | — |

## Domain-dependency seeds

Not applicable — `layer-rules` not adopted.

## Deliberate exclusions

- **GI-016 — Documentation language and style are not governed:** the team writes in whatever English it has; no principle governs spelling, register, or the language of internal documents. · **Mark:** Confident
  *Elicited from:* dimension 10 — "we have Portuguese, Scottish, and Bristol English on the team and nobody is going to police it" (Inês).

## Review

**2026-09-09 — first ratification**

- **Sizing:** 16 elements · marks Confident except GI-012 Contested · reality-surface load high (three legal-mandate modules, launch in two weeks); default pair; **lead sized:** pair.
- **Review:** pair (coverage / coherence); **tally** 6 raised → 3 merged survivors; recommended status: needs-revision
- **Survivor dispositions:**

  | # | Sev | GI element(s) | Finding | Disposition |
  |---|-----|---------------|---------|-------------|
  | S1 | Critical | GI-012 | A legal-mandate obligation cannot be waived; the row is documented evidence of a knowing violation. | user-ruled — overruled → GI-012 marked `Contested`; recorded-open |
  | S2 | Important | GI-009 | The PCI scope named `src/payments/` only; the PAN transits `src/checkout/` first. | resolved — scope widened in the element text above |
  | S3 | Minor | GI-011 | "Fast" had no number when first drafted. | resolved — 2.5 s LCP at p75, Lighthouse CI, recorded in the element |
