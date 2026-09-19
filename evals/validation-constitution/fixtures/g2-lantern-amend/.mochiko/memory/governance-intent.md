# Governance Intent — Lantern

**Session date:** 2026-04-14 · **Mode:** amend (AM-1, 2026-09-10; first ratification brownfield 2026-04-14)
**Confirmed at synthesis checkpoint:** 2026-04-14 by Mara Okonkwo · AM-1 delta confirmed 2026-09-10 by Mara Okonkwo
**Governs:** the governance surface set v1.0.0 (CLAUDE.md governance region · `.claude/rules/mochiko/` · `.mochiko/memory/governance-ledger.md`) — amended at AM-1 (2026-09-10; version per the ledger's amendment log)

## Fact profile

- **GI-001 — Facts:** industry: B2B SaaS analytics (US) · data classes: founder account data (name, e-mail), Stripe customer aggregates (counts and amounts, no card data, no cardholder names) · jurisdictions/markets: US only · contractual commitments: none · **Mark:** Confident
- **Modules triggered (mechanical):** none — negatives confirmed with consequence: no personal data of EU/UK residents → `gdpr` does not attach; no cardholder data (Stripe hosted checkout; PAN never reaches Lantern) → `pci-dss` does not attach; no health data → `hipaa` does not attach; no attestation commitments → `attestation` does not attach; customer-facing UI is a B2B dashboard behind login, ADA Title III applicability to private B2B SaaS not asserted by counsel → `a11y` does not attach, revisit if a public-facing surface ships · **Mark:** Assumed (the a11y negative rests on counsel's informal view)
- **Brownfield cross-check:** consistent — `codebase-analysis.md` detects Stripe hosted checkout + webhooks only, no card fields, no EU data residency.
- *AM-1 (2026-09-10):* fact profile unchanged.

## Project identity & type

- **GI-002 — Type:** backend service + web dashboard → shelves dealt: universal floor · backend-service · **Mark:** Confident
- **Identity:** Lantern turns a founder's Stripe account into MRR, churn, cohort retention, and a weekly e-mail. Solo founder plus one contractor two days a week; 140 paying tenants.
- **Risk surface:** a wrong number in the weekly e-mail is a founder making a hiring decision on bad data; a cross-tenant leak is the end of the company.
- **Team reality:** two people; every PR reviewed by the other; GitHub Actions CI. Enforcement leans on CI and tooling, never on "review will catch it" alone.

## Depth level declaration

- **GI-003 — Declared level:** high · **Declared:** `low` on 2026-04-14 by Mara Okonkwo; **flipped `low`→`high` on 2026-09-10 by Mara Okonkwo (AM-1 flip ceremony)** · **Mark:** Confident
- **Rationale:** at v1.0.0 the founder wanted coverage measured before a blocking threshold was asserted; five months on, coverage sits at 84% and the founder ruled the high row on.
- **Ratchet:** one-way — `high` is terminal.

## Convergence skips

- Dimension 5 (team) — settled by dimension 1: two people, mutual review.

## Real commands

(dimension 6/8 → the validator's placeholder bar)

| Purpose | Command | Source |
|---------|---------|--------|
| Lint | `ruff check .` | detected |
| Types | `mypy src` | detected |
| Test + coverage | `pytest --cov=lantern --cov-report=term-missing` | detected |
| Import rules | `lint-imports` | detected (`importlinter` contract in `pyproject.toml`) |
| Secret scan | `gitleaks detect --source .` | declared (CI job `secrets`, blocking from AM-1) |
| Dependency advisories | `pip-audit` | declared (CI job `audit`, blocking at high/critical from AM-1) |
| Migrations | `alembic upgrade head` | detected |
| Deploy | `fly deploy --app lantern-staging` · `fly deploy --app lantern-prod` | detected (`fly.toml`) |

## Floor expression & deck rulings

| GI-ID | Card | Layer | Ruling / Expression | Mark |
|-------|------|-------|---------------------|------|
| GI-004 | FLOOR-SEC | floor-asserted | Secrets in Fly secrets, `fly.toml` clean (GI-016 fix landed 2026-04-14); Pydantic validation on every route; `require_tenant` on every router. *AM-1, high row:* `gitleaks` blocks merge; `pip-audit` blocks merge at high/critical. | Confident |
| GI-005 | FLOOR-TEST | floor-asserted | v1.0.0 low row: `pytest` in CI (the analysis found it missing — MUST-implement, landed 2026-04-20), coverage reported, ratchet. *AM-1, high row:* coverage thresholds enforced at the card's row — ≥80% warning, ≥60% blocking (`--cov-fail-under=60`); no session override. | Confident |
| GI-006 | FLOOR-ERR | floor-asserted | Low row: no silent corruption of rollups, no swallowed exceptions (the two in `nightly.py` — MUST-fix, landed 2026-04-18), no leaked traces. *AM-1, high row:* RFC 7807 on every error response · `correlation_id` on every error body. | Confident |
| GI-007 | FLOOR-OBS | floor-asserted | v1.0.0: waived structured logging (GI-009). *AM-1, high row, waiver retired:* `structlog` JSON with a `correlation_id` on every line · no personal data in logs (the `auth.py:41` e-mail line removed) · `/healthz` on both apps. | Confident |
| GI-011 | BE-HEX | arbitrated | v1.0.0: dropped ("not yet"). **AM-1: kept** — the analysis's hexagonal layout is real and the `importlinter` contract already exists; the founder ruled the layer rules on, with the domain-dependency registry seeded (`pydantic`, `attrs`). `layer-rules` module adopted at AM-1. | Confident |

## Minted principle intents

- **GI-008 — Tenant-scoped data access:** "nothing is ever queried without a tenant id, and raw SQL lives only in repositories — the two report use cases that build SQL strings get moved." Scope-bound: every layer that can issue or shape a query — the repositories in `infrastructure/`, and the use cases in `application/` that reach the database through the `ReportStore` and `UnitOfWork` ports. · **Mark:** Confident
  *Elicited from:* dimension 9 — the founder's first answer to "what would end the company"; the analysis's high-severity raw-SQL finding named as the thing to close.
- **GI-010 — Repositories per aggregate:** "no SQLAlchemy in the use cases — every aggregate behind a repository protocol, one implementation, so tenant scoping is reviewable in one file." · **Mark:** Confident
  *Elicited from:* dimension 9 — codifying the analysis's Strength 1; the founder's words.
- **GI-015 — Short-lived feature flags (v1.0.0):** "a flag older than 30 days is deleted; the flag service lists age." · **Mark:** Confident *(v1.0.0)*
  *Elicited from:* dimension 9 at v1.0.0.
  *AM-1 (2026-09-10): **superseded — element retired.*** The flag service was decommissioned in August (flags moved to environment variables per tenant cohort); the founder ruled the principle removed with no replacement. Retired, never renumbered.

## Waivers

| GI-ID | Standard | Justification | Revisit trigger | Mark |
|-------|----------|---------------|-----------------|------|
| GI-009 | FLOOR-OBS structured logging + correlation ids (v1.0.0) | plain-text logging today; structlog migration scheduled | the `structlog` migration lands | Confident |

*AM-1 (2026-09-10): GI-009 **un-waived** — the migration landed 2026-08-30; the waiver is retired and the high row of FLOOR-OBS is asserted in full (an un-waive is a governance event).*

## Module selections

| GI-ID | Module | Ruling | Because | Mark |
|-------|--------|--------|---------|------|
| GI-011 | layer-rules | **adopted at AM-1** (declined at v1.0.0) | BE-HEX kept at AM-1; `importlinter` contract exists; registry seeded with `pydantic` (signal level 1) and `attrs` (signal level 2). | Confident |
| GI-012 | release-gates | adopted (v1.0.0) | Fly `lantern-staging` on merge, `lantern-prod` by manual deploy; a 24-hour staging soak and `alembic` reversibility block prod; rollback `fly releases rollback` within 10 minutes. | Confident |
| GI-013 | evolution-notes | adopted (v1.0.0, brownfield — always) | Floor status from the analysis + the session confrontation. | Confident |
| GI-014 | knowledge-management | **declined — durable** (v1.0.0) | "Two people and a Notion page; not now." | Confident |

## Domain-dependency seeds

| GI-ID | Dependency | Signal level | Ruling | Mark |
|-------|------------|--------------|--------|------|
| GI-011 | `pydantic` | 1 — FastAPI's own model layer, live-verified | kept | Confident |
| GI-011 | `attrs` | 2 — widely adopted, maintained | kept | Confident |

## Deliberate exclusions

None recorded.

## Confrontation rulings (brownfield)

- **GI-016 — `DATABASE_URL` with password in `fly.toml`:** the analysis found the connection string committed. Ruling: MUST-fix in the v1.0.0 finalize (moved to Fly secrets, 2026-04-14); folded into GI-004's expression. · **Mark:** Confident

## Review

**2026-04-14 — first ratification**

- **Sizing:** 16 elements · marks Confident except GI-001 Assumed; default pair; **lead sized:** single (solo founder, no contested marks).
- **Review:** solo reviewer; **tally** 3 raised → 1 survivor; recommended status: needs-revision (resolved)
- **Survivor dispositions:**

  | # | Sev | GI element(s) | Finding | Disposition |
  |---|-----|---------------|---------|-------------|
  | S1 | Important | GI-005 | CI did not run `pytest`; asserting a ratchet with no CI figure was hollow. | resolved — MUST-implement, landed 2026-04-20 |

**2026-09-10 — AM-1 (amend)**

- **Sizing:** governance event (depth flip · module attach · un-waive · principle retired); default pair; **lead sized:** pair.
- **Review:** pair (coverage / coherence); **tally** 5 raised → 2 merged survivors; recommended status: needs-revision (resolved)
- **Survivor dispositions:**

  | # | Sev | GI element(s) | Finding | Disposition |
  |---|-----|---------------|---------|-------------|
  | S1 | Critical | GI-003 · GI-011 · GI-015 | Three of the four deltas are MAJOR-class under the ledger's own semver line. | resolved — recorded here; the producer determines the bump from this log |
  | S2 | Important | GI-008 | The scope line had named `infrastructure/` only; the raw-SQL finding lives in `application/reports/`. | resolved — scope widened in the element text above |

## Amendment Log

**AM-1 — 2026-09-10 — depth flip, layer rules, un-waive, one principle retired** *(user-ruled)*

- GI-003 flipped `low`→`high` (flip ceremony).
- GI-011 layer-rules adopted; BE-HEX kept; registry seeded (`pydantic`, `attrs`).
- GI-009 un-waived (structlog migration landed 2026-08-30).
- GI-015 superseded — the flag service was decommissioned; principle removed, no replacement.
- Preserved blocks, values on entry to this amend (to be re-emitted unchanged): the `mochiko:output-style` switch line read **conversation: `lite` · reports: `ultra` · documents: `full`** (Mara set `lite` on 2026-05-02); the Shape-5 output-style rules file carried the user-added line "Spell out currency codes (`USD`, never `$`)".
- Untouched: GI-001, GI-002, GI-004 (expression re-rowed only), GI-006, GI-007, GI-008, GI-010, GI-012, GI-013, GI-014, GI-016.
