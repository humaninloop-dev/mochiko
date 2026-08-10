# Transcript — setup-agents-r2

Simulated `/mochiko:setup` run for the greenfield **Ledgerline** project. Variant: `agents`
(agent roster staffed from frontmatter descriptions; original skill bodies + descriptions).
Run-lead plays every seat after dispatch; principal answered only from fresh subagent spawns
against `fixture/persona-card.md` (run-lead never read the card — D3 fence).

---

## Stage 0 — Roster build (frontmatter descriptions only)

Agent roster assembled by reading ONLY the frontmatter `description`/`skills` of each agent file.
6 named agents from `variants/agents/` (slim descriptions, no `<example>` blocks):
principal-architect, validator, devils-advocate, requirements-analyst, product-manager,
product-engineer. Remainder from `plugins/mochiko/agents/`: qa-engineer, staff-engineer,
system-architect, technical-analyst.

Roster (name — routing-relevant capability from description — skills):
- **principal-architect** — governance judgment; authors/updates the constitution (greenfield &
  brownfield), runs codebase analysis, cross-artifact feasibility — `authoring-constitution,
  analysis-codebase, review-feasibility, authoring-architecture`
- **validator** — skeptical independent grader of a finished artifact against a checklist; binary
  PASS/FAIL; defaults FAIL; never grades own work — `validation-constitution`
- **devils-advocate** — adversarial reviewer; stress-tests, gap-hunts, "what if"; severity-ranked
  verdict — `review-specifications, review-plan-artifacts, review-brainstorm,
  review-governance-intent`
- **requirements-analyst** — vague request → precise spec; user stories — `authoring-requirements,
  authoring-user-stories`
- **product-manager** — capability layer; feature map — `authoring-feature-map`
- **product-engineer** — clickable low-fi prototypes — `authoring-prototype`
- **qa-engineer** — verification discipline — `testing-end-user, review-code-minimalism`
- **staff-engineer** — TDD implementation — `executing-tdd-cycle, brownfield-integration,
  patterns-code-minimalism`
- **system-architect** — topology/architecture — `patterns-system-design,
  patterns-technical-decisions`
- **technical-analyst** — technical requirements + design artifacts — `authoring-technical-requirements,
  patterns-technical-decisions, patterns-entity-modeling, patterns-api-contracts`

Seats this setup needs, staffed from the descriptions alone (route basis recorded in
invocations.json): interrogation (lead, inline), cold intent review, constitution authoring,
independent validation, feature-map scaffold.

---

## Stage 1 — Interrogation (lead, inline; `mochiko:analysis-iterative` engine + INTERROGATION-AGENDA)

Adaptive, one-topic-per-turn, batched per principal spawn (D3 fence). Five principal spawns.

### Batch 1 — Dimension 1 (identity), Dimension 4 (risk)
**Q1** What is Ledgerline, who for, lifespan?
**Principal:** Invoicing + payment-tracking SaaS for US small independent contractors (plumbers,
electricians, freelance designers). Create clients, send invoices with a payment link, track
paid/unpaid, chase, see cash-flow. Built to last years; paying customers ~month four. Backend
Python/FastAPI on Postgres, React front end, Stripe for payments. Solo founder + one part-time UI
helper.
**Q2** What does failure cost?
**Principal:** Worst outcome is corrupting or losing a contractor's financial data — invoice
amounts, payment status, who owes what. Instant loss of trust; kills the product. Wants
production quality from day one.

### Batch 2 — Dimension 2 (fact profile), Dimension 3 (type)
**Q1** Data classes; does it store raw card data or is Stripe hosted? (consequence: no PCI as a
storing entity)
**Principal:** No card numbers touch my servers — Stripe hosted checkout. I store names, emails,
addresses, invoice amounts, payment status. PCI as a storing entity does not apply. Confirmed.
**Q2** Jurisdictions? (consequence: no GDPR)
**Principal:** US-only, no EU/UK users. GDPR shouldn't attach. Confirmed.
**Q3** Health data? (consequence: no HIPAA)
**Principal:** None. HIPAA doesn't apply. Confirmed.
**Q4** Customer-facing US web UI → accessibility (ADA); in scope?
**Principal:** Hadn't thought hard about it, but yes — accessibility in scope. Don't gold-plate;
sensible basics for v1, not a full audit that delays launch. Cheap-to-bake-in, in scope.
**Q5** Contractual commitments (SOC 2 / ISO / addenda)? (consequence: no attestation)
**Principal:** Nothing signed. One prospect's procurement form raised SOC 2, no active
obligation. Attestation doesn't attach. Confirmed.

### Batch 3 — Dimension 5 (team), Dimension 6 (practices/stack), Dimension 8 (deployment)
**Q1** Review culture, or CI/tooling-led enforcement?
**Principal:** No review culture — no one to review. Discipline comes from tooling and CI, not a
second human on every PR.
**Q2** Real commands/tools?
**Principal:** Python: ruff (lint+format), pytest. JS: ESLint + Prettier. Front-end tests: maybe
Vitest / React Testing Library for important bits, not chasing UI coverage early. CI: GitHub
Actions, lint + tests on push. Cheap and automatic.
**Q3** Deploy target, environments, cadence, rollback?
**Principal:** Render (leaning, not committed). Staging separate from prod. Ship-when-ready, no
fixed schedule. Rollback: shaky — wants fast return to last good version, hasn't set it up, asks
for a recommendation, don't over-build.

### Batch 4 — Dimension 7 (KM + electives), Dimension 9 (values), Dimension 10 (exclusions)
**Q1** Adopt operating-docs layer (default-on)?
**Principal:** Adopt it — solo, won't remember decisions in three months. Keep it lightweight.
**Q2** Electives CHANGELOG / RUNBOOK?
**Principal:** Both. Paid product with real releases → CHANGELOG; one-person deployed service →
RUNBOOK (fill in as I learn).
**Q3** Values — insist / refuse?
**Principal:** INSIST: CI blocks on linters + tests before prod (ruff, pytest, ESLint/Prettier),
no merging red; no secrets in repo (catch a committed API/Stripe key). REFUSE: coverage-%
gates; mandatory code review to merge — "no ceremony that assumes a team I don't have."
**Q4** Deliberate exclusions?
**Principal:** Observability/incident-response depth (SLOs, on-call) — hold off, don't know what
they should be pre-launch; Sentry probably, figure the rest out at real traffic. SOC 2 /
formal-compliance shaped — no active obligation, don't build governance around it. Product
features recurring/multi-currency/estimates/client-portal — out of product scope, govern
nothing there.

### Batch 5 — Floor assertion + deck arbitration (recommend-then-arbitrate) + rollback recommendation
**A) Asserted floor** — Security list kept verbatim; Testing: coverage-% gate WAIVED, replaced by
tests-must-pass-in-CI + smoke test on invoice-create & payment-status from day one (approved);
Error handling kept every word (#1 fear — money corruption); Observability minimal baseline (JSON
logs, no PII/keys, /health, Sentry) kept, heavy SLO/on-call layer stays excluded.
**B) Backend cards** — Layered: keep PRAGMATIC form (money math isolated from FastAPI + DB behind
service+repository, import-linter in CI), full four-layer ports/adapters dropped as over-build.
Complexity: keep CI complexity limit (ruff C901 ≤10) only; review-enforced metrics (param count,
file length, nesting) dropped — no reviewer. Dependency discipline: kept (justify new deps, pin
versions, vuln scan blocks high/critical).
**C) Rollback** — recommendation accepted: Render redeploy last good image (≤15 min), Alembic
reversible migrations, destructive migrations flagged for explicit self-approval; no blue/green,
no canary.

Passive-acceptance watch: principal elaborated substantively on every turn (multiple
tighten/drop rulings, not an adoption streak) — no ratification-streak flag raised.

Interrogation converged. Synthesis assembled → `.mochiko/memory/governance-intent.md`.

---

## Stage 2 — Cold intent review (blind-map dispatch)  [see Stage-2 section appended below]
## Stage 3 — Synthesis ratification (principal spawn)
## Stage 4 — Constitution authoring (principal-architect / authoring-constitution)
## Stage 5 — Independent validation (validator / validation-constitution) + fix round
## Stage 6 — KM scaffolding + greenfield feature map

(Each stage's dialogue and rulings appended in order below as the run proceeds.)

---

## Stage 2 — Cold intent review (blind-map dispatch) — devils-advocate, solo

Seat staffed: **devils-advocate** (`review-governance-intent`). Two-message blind-map dispatch:
message 1 (topic/identity/goal only, no synthesis path) → reviewer built a 17-angle blind map;
message 2 delivered the frozen synthesis path → Phase 1 cold read.

**Result: 10 raised, 8 survived** (2 fell: US-state-privacy CCPA/CPRA at the blind-map diff;
availability/SLO as saw-and-ruled). Surviving: 4 Critical, 2 Important, 2 Minor. Recommended:
**needs-revision**, with escalation-to-critical-gaps only if the data-class re-elicitation confirmed
stored tax-ID/bank data.

Survivors (full dispositions in `governance-intent.md` Review section):
- C1 (data classes — tax IDs + Stripe Connect payout/KYC never consequence-confirmed)
- C2 (data-loss/backup half of the top risk ungoverned)
- C3 (no financial audit trail / invoice-edit immutability)
- C4 (auth-not-authz — cross-tenant isolation ungoverned)
- I1 (GDPR negative reasoned about users, not the client population)
- I2 (financial-path test only checked create+status existence)
- M1 (email deliverability/anti-spoofing + link integrity)
- M2 (thin domain-dependency seed)

Contamination correction (deviation, see meta.json): the first synthesis draft carried a
pre-filled Review section; a first reviewer read it before the fence was caught. The Review section
was reset to a genuine pre-review state and a fresh reviewer re-ran Phase 1 against the clean
synthesis — the report above is that clean run.

## Stage 2b — Coverage-survivor routing (user rules the path) — principal spawn (review follow-up)

Each survivor questions setup scope → presented to the user as a candidate topic; the user ruled:
- C1 → **rule inline**: no tax IDs stored (no 1099 v1); NOT Stripe Connect (contractor connects own
  Stripe, paid directly; no bank/KYC data on Ledgerline). Both recorded as consequence-stated
  negatives → **no critical-gaps escalation**.
- C2 → **mint** GI-012: automated DB backups + a restore tested before launch and periodically.
- C3 → **mint** GI-013: immutable audit log of invoice-amount + payment-status changes (scoped).
- C4 → **mint** GI-014: per-tenant query scoping + a cross-tenant 403/404 test.
- I1 → **defer with consequence** GI-024: clients treated US v1, no GDPR machinery; EU push = event.
- I2 → **fold into GI-004**: financial-path test widened to amount/tax/rounding correctness + assert
  the Stripe webhook flips payment status.
- M1 → **mint** GI-015: SPF/DKIM/DMARC + payment-link integrity.
- M2 → **accept**: no extra money helper for v1 (`decimal` is stdlib); GI-020 stays `Assumed`.

Reopen-born intents (GI-012/013/014/015) landed in the GI namespace and rode the verify pass
(internal consistency + provenance). Verify pass: PASS.

## Stage 3 — Synthesis ratification — principal spawn

Founder CONFIRMED the assembled synthesis ("Confirmed … go author the surfaces"), reasserting the
financial-path test and the firm v1 scope. Synthesis confirmed at checkpoint 2026-08-10.

## Stage 4 — Constitution authoring — principal-architect (`authoring-constitution`, greenfield)

Authored the surface set from the ratified synthesis:
- `CLAUDE.md` governance region (ratified stamp, 13-principle index + universal lines, tech stack,
  quality gates with real commands, governance operations incl. release-gate + KM + output-style).
- 6 rules files under `.claude/rules/mochiko/`: layer-boundaries.md (layer-rules module + domain
  registry: pydantic), tenant-isolation.md, financial-audit.md, accessibility.md, operating-docs.md
  (KM), output-style.md (Shape 5).
- Governance ledger: Three-Part records for all 13 principles, waiver (GI-019), amendment policy,
  exception registry, domain-dependency policy, release-gates content.
- Trace summary manifest (13 principle rows, closure both ways).

## Stage 5 — Independent validation — validator (`validation-constitution`) + fix round

Round 1: **FAIL** — (1) GI-008 missing index line (index→home closure broken); (2) ratified stamp
under-reported the three attached template modules. Fix round applied both (index line added; stamp
+ ledger header enumerate all attachments). Round 2 re-grade: **PASS** (62/62 checklist, trace
closure 13/13, no anti-patterns, v1.0.0). Full evidence in
`project/.mochiko/memory/validation-report.md`.

## Stage 6 — KM scaffolding + greenfield feature map + acceptance

- KM module scaffolded: project-pinned `.mochiko/memory/knowledge-management.md` + core docs
  (ROADMAP.md, BACKLOG.md, DECISIONS.md, ARCHITECTURE.md, GLOSSARY.md, brainstorms/index.md,
  specs/index.md, archive/backlog-trail.md) + electives (CHANGELOG.md, RUNBOOK.md).
- Greenfield feature map: empty FEATURES.md index scaffold + `.mochiko/features/` (product-manager
  seat; body not loaded — mechanical scaffold).
- **Final acceptance** (principal spawn): founder ACCEPTED the validated surface set ("accept it …
  Ship it"). No flagged proposals.

Setup complete. Next step: `/mochiko:specify` (+ `/mochiko:brainstorm`, KM adopted).
