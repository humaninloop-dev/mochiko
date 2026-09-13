# Governance Intent — Halyard

**Session date:** 2026-09-10 · **Mode:** brownfield
**Confirmed at synthesis checkpoint:** 2026-09-10 by Marcus
**Governs:** the governance surface set v1.0.0 (CLAUDE.md governance region · `.claude/rules/mochiko/` · `.mochiko/memory/governance-ledger.md`) — to be authored on ratification

## Fact profile

- **GI-001 — Facts:** industry: healthcare (outpatient physiotherapy, US) · data classes: protected health information (intake forms: presenting condition, medications, prior injuries, clinician notes), patient identity and contact data, appointment history · jurisdictions/markets: US (Oregon, Washington) · contractual commitments: Business Associate Agreements with Cascade Physio Group and Sound Rehab (three-year terms from 2026-01)
- **Modules triggered (mechanical):** `hipaa` (PHI about identifiable people, US market — legal-mandate) · `a11y` (customer-facing booking widget in the US — ADA) · negatives confirmed: no cardholder data (clinics bill outside Halyard) → `pci-dss` does not attach; no EU or UK residents → `gdpr` does not attach; attestation: Cascade's MSA references "a SOC 2 Type II report when available" — read as no present commitment → `attestation` does not attach · **Mark:** Assumed
- **Brownfield cross-check:** consistent — `codebase-analysis.md` detects the `intake_forms` and `patients` tables and no payment processor

## Project identity & type

- **GI-002 — Type:** fullstack (Django + DRF API, React booking front end) → shelves dealt: backend-service (floor categories translated for the widget in session) · **Mark:** Confident
- **Identity:** Halyard schedules appointments and collects intake forms for small physiotherapy clinics; live with six clinics. Lifespan: a twelve-month pilot with the two clinic groups, then a decision on whether to continue.
- **Risk surface:** PHI exposure (breach notification, BAA termination, the end of the business); a missed or double-booked appointment costs a clinic a slot and Halyard its reputation.
- **Team reality:** two engineers (Marcus, Dee); every PR reviewed by the other; no one else touches the code.

## Depth level declaration

- **GI-003 — Declared level:** low · **Declared:** 2026-09-10 by Marcus · **Mark:** Confident
- **Rationale:** setup recommended `high` (brownfield, live PHI, BAAs in force); Marcus ruled `low`: "there are two of us and CI does not even run the tests yet — get the ratchet honest first, flip in Q1."
- **Ratchet:** one-way — `high` is terminal; a later `low`→`high` move happens only through a flip ceremony.

## Convergence skips

- dimension 5 (team reality) settled by dimension 1 — two engineers, named in the identity answer.
- dimension 8 (deployment and release reality) settled by dimension 6 — the analysis shows the Heroku `Procfile` and a `release:` phase, so deployment is already known and the dimension was not asked.

## Real commands

| Purpose | Command | Source |
|---------|---------|--------|
| Lint | `ruff check .` | detected (runs on every PR in CI) |
| Test | `pytest` | detected (runs on every PR in CI) |
| Build (web) | `npm --prefix web run build` | detected |
| Migrate | `python manage.py migrate --noinput` | detected (Heroku release phase) |

## Floor expression & deck rulings

| GI-ID | Card | Layer | Ruling / Expression | Mark |
|-------|------|-------|---------------------|------|
| GI-004 | FLOOR-SEC | floor-asserted | secrets in Heroku config vars; `IsAuthenticated` on every view except the two public ones by design; secret scanning **blocks merge in CI**; `pip-audit` **blocks merge at high/critical**; hosting: Heroku's HIPAA-eligible Shield tier signs a BAA, so the hosting side of the BAA obligation is covered | Confident |
| GI-005 | FLOOR-TEST | floor-asserted | smoke test on the booking flow; coverage measured and reported on every PR; **thresholds enforced — ≥ 80 % warning, ≥ 60 % blocking** | Confident |
| GI-006 | FLOOR-ERR | floor-asserted | low row — booking writes transactional; `DEBUG=False` in production so no stack traces; Sentry on every unhandled error | Confident |
| GI-007 | FLOOR-OBS | floor-asserted | low row — request logs on the booking and intake paths; the PHI redaction filter in `halyard/logging.py` keeps intake fields out of log lines | Confident |
| GI-008 | BE-HEX | arbitrated | kept — as recommended | Confident |
| GI-009 | BE-SRP | arbitrated | kept — as recommended | Confident |
| GI-010 | BE-DEP | arbitrated | dropped — the user overruled the lead's challenge: the lead argued that exact pins without a resolver drift within a quarter and that a two-person team feels supply-chain breakage last (steelman heard); Marcus: "`requirements.txt` is pinned exactly and we bump by hand each month; revisit at the first incident that traces to a dependency" | Contested |

## Minted principle intents

- **GI-011 — PHI never in logs or error reports:** no intake-form field, clinician note, or patient identifier appears in a log line or a Sentry event · **Mark:** Confident
  *Elicited from:* "PHI must never be logged" (dimension 9).
- **GI-012 — Public views are named:** any view without `IsAuthenticated` is listed in `halyard/api/public.py` with a one-line reason, and nothing else is public · **Mark:** Confident
  *Elicited from:* Marcus, dimension 9: "I want to be able to see every public endpoint in one file; that is the list I'd show an auditor."
- **GI-013 — Storybook for every component:** every React component lives in Storybook with a visual-regression snapshot checked in CI · **Mark:** Confident
  *Elicited from:* lead suggestion at dimension 9; Marcus: "sure, if it's not much work."

## Waivers

| GI-ID | Standard | Justification | Revisit trigger (optional) | Mark |
|-------|----------|---------------|---------------------------|------|
| GI-014 | FLOOR-SEC — dependency vulnerability scanning runs | not now | — | Contested |
| GI-015 | `hipaa` — audit logging of authentication events | Heroku router logs already record every request, which covers it | permanent (D4.1 pending) | Confident |

## Module selections

| GI-ID | Module | Ruling | Because | Mark |
|-------|--------|--------|---------|------|
| GI-016 | knowledge-management | adopted (core; CHANGELOG and RUNBOOK electives declined) | as recommended | Confident |
| GI-017 | layer-rules | declined | as recommended | Confident |

## Deliberate exclusions

- **GI-018:** front-end performance — not covered until the frontend shelf exists · **Mark:** Confident

## Review

**2026-09-10 — first ratification**

- **Sizing:** lead stated weight [18 elements · marks 15 Confident / 1 Assumed / 2 Contested · reality-surface load: Django app, Heroku, Twilio, Postmark]; default pair on first ratification; **lead sized:** pair (coverage · coherence)
- **Review:** pending — two cold seats: coverage lens, coherence lens
- **Survivor dispositions:** pending — the seats report; the lead merges and folds
- **Verify pass:** pending

## Amendment Log

(empty — first ratification)
