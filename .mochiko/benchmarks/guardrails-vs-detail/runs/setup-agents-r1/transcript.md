# Transcript — setup-agents-r1

Simulated `/mochiko:setup` run for the greenfield Ledgerline project. Variant: **agents** (agent
roster staffed from example-free frontmatter descriptions for the 6 variant agents; originals for
the 4 plugin agents; skill bodies + descriptions all originals). Single-agent simulation: the
run-lead plays every seat after staffing it, loading the governing skill body before performing
the seat's work. Principal answers come only from fresh subagent spawns against the frozen card
(the run-lead never read the card).

---

## Phase 0 — Agent roster built (from frontmatter descriptions only)

Roster assembled by reading ONLY the frontmatter `description` (and `skills:`) of each agent
before any seat was staffed:

- Variant (example-free) descriptions: principal-architect, validator, devils-advocate,
  requirements-analyst, product-manager, product-engineer
- Plugin (example-carrying) descriptions: qa-engineer, staff-engineer, system-architect,
  technical-analyst

Setup seats needed and how each was staffed from the descriptions (full log in invocations.json):

| Seat moment | Chosen | Considered | Why (from description) |
|-------------|--------|------------|------------------------|
| Interrogation (ten dimensions) | LEAD, inline | requirements-analyst | Command says run interrogation inline via `analysis-iterative`; not an agent seat. requirements-analyst is spec elicitation, not governance intent. |
| Cold pre-ratification intent review | devils-advocate | validator, principal-architect | "Adversarial reviewer… stress-tests… challenging assumptions… severity-ranked gap report + recommended verdict" + carries `review-governance-intent`. validator is checklist PASS/FAIL of finished authored surfaces; principal-architect's review skill excludes the constitution/intent. |
| Constitution authoring | principal-architect | technical-analyst, staff-engineer | "Establishing governance standards… authors the constitution… greenfield — formulating the client's ratified intent." Others author feature/plan/code artifacts, not governance. |
| Independent validation | validator | devils-advocate, principal-architect | "Grades a finished artifact against an explicit checklist… PASS/FAIL… defaults to FAIL… never grades work it authored." Independence: principal-architect authored, so it cannot self-grade. |
| Feature-map scaffold (greenfield) | product-manager | requirements-analyst, principal-architect | "Owns the product's capability layer… authors the feature map." Greenfield = empty index scaffold. |

No route miss: every setup seat resolved to the correct persona from its example-free
description. (The 4 example-carrying plugin agents staff no setup seat — they are
specify/plan/implement personas.)

---

## Phase 1 — Interrogation (lead, via mochiko:analysis-iterative)

Ten dimensions worked adaptively; principal answered by fresh card-grounded spawns (batched by
dimension group to bound spawn count — see meta.json deviations).

**Batch 1 (dimensions 1-3 — identity, fact profile, type).** Principal: Ledgerline is a
greenfield invoicing/payment-tracking SaaS for solo US contractors; contact PII + financial
records, no card data (Stripe hosted checkout); US-only; no active compliance obligation (SOC 2
only mentioned by a prospect); fullstack — FastAPI/Postgres/React/Stripe, deploy on Render; web
only.

**Batch 2 (dimensions 4-6 — risk, team, practices).** Risk ranked: (1) payment-state/money
correctness ("wrong data that looks right scares me more than no data"), (2) data leak, (3) trust;
downtime least. Team: solo founder full-time (6 yrs backend), friend ~5h/week React; no review
process; never run production SaaS solo, never set up monitoring. Tooling: pytest + GitHub Actions
CI ~settled; Ruff (Black/Ruff) open; timing of pipeline stand-up open.

**Batch 3 (dimensions 7-8 — KM, deployment/release).** KM layer adopted (lightweight, core
whole). CHANGELOG declined (revisit at launch). RUNBOOK undecided pending cost. Deploy Render;
ship-when-ready cadence; green CI is the release bar; wants rollback (asked how — lead answered
Render redeploy + backward-compatible migrations). release-gates adopted.

**Batch 4 (dimensions 9-10 — values, exclusions).** Non-negotiables: payment-state correctness
with CI teeth (incl. manual mark-as-paid), secrets never in repo (block merge), HTTPS/hashed
passwords/tenant isolation (wanted the last as a check). Refuses heavyweight team ceremony (no
mandatory two-reviewer approvals). RUNBOOK on (stub + grow). Exclusions: SLO numbers, data
retention/deletion, SOC 2.

**Deck arbitration (recommend-then-arbitrate).** BE-HEX kept, tightened to lite (ports around
Stripe+DB tested with fakes; strict import-linter dropped). BE-SRP kept, automated complexity gate
only (size limits advisory). BE-DEP kept with an unpatchable-CVE escape hatch. Ranked 1-3-2; no
drops. → layer-rules module declined (import-linter enforcement was what the founder refused).

## Phase 2 — Synthesis + cold intent review

Synthesis authored (lead's pen) at `.mochiko/memory/governance-intent.md` — GI-001…GI-019, zero
floor waivers.

Cold intent review staffed to **devils-advocate** running `review-governance-intent`, **sized
down to solo** (departure trail recorded: greenfield, small element count, clean waiver ledger;
single cold read retained for the two load-bearing intents). Blind-map diff + five hunt classes →
**7 survivors, none Critical, recommended needs-revision**. Routed to the user:

1. Money/decimal precision (Important) → rule inline: **mint GI-020** (exact decimal, never float).
2. Webhook idempotency (Important) → **explore now**; re-elicited and **minted GI-021**
   (idempotent transitions, CI-blocking duplicate test; mechanism left to implementation).
3. Payment-state audit trail (Minor) → rule inline: **mint GI-022**.
4. Reminder email failure (Minor) → **defer** (GI-023).
5. Fullstack but backend-only floor (Important) → rule inline: **firm two UI-side security clauses
   into GI-003** (no secrets in bundle; input validation/XSS on API-touching UI), accept the rest
   as known-thin **GI-024**.
6. CCPA/CPRA (Minor) → **defer** (GI-025; GI-001 negative added).
7. Day-one 60% blocking coverage vs timing hesitancy (Minor) → **confirm accept** (ratchet-from-
   zero framing dissolves it).

Folds applied; solo verify pass PASS.

## Phase 3 — Ratification

Synthesis-confirmation checkpoint: user **edited then confirmed** — GI-010 scoped so manual
mark-as-paid (checks/cash) stays a first-class close path (not a violation of the Stripe-
confirmation rule), carrying GI-021 idempotency + GI-022 audit trail. Bounded G3-edit delta-pass
over the single edited element: PASS. Header stamped **confirmed 2026-08-10**.

## Phase 4 — Constitution authoring (principal-architect, mochiko:authoring-constitution)

Greenfield surface set authored (plan-approved by lead): CLAUDE.md governance region (v0.1.0,
principle index, floor imperatives, tech stack, quality gates, KM + release-gates pointers,
output-style switch line, new-file read line); scope-bound rules files (financial-correctness,
api-security, frontend-security, error-handling, architecture, operating-docs, output-style);
governance ledger (Three-Part records GI-003…GI-022, release-gates section, unpatchable-CVE
exception policy, trace summary, amendment log); KM scaffold (project-pinned copy + ROADMAP,
BACKLOG, DECISIONS, ARCHITECTURE, GLOSSARY, RUNBOOK, brainstorms/specs indexes, backlog-trail);
greenfield empty FEATURES.md.

## Phase 5 — Independent validation (validator, mochiko:validation-constitution)

Graded from the files (author≠grader; principal-architect authored, validator graded), default
FAIL. Deterministic scans: markers unique, all index pointers resolve, no placeholders (the lone
`<YYYY-MM-DD>` hit is literal schema-format text in the KM copy, matching the source template),
4/4 floor categories principled, three-part records complete, trace closes both ways, zero waivers
to reconcile, no anti-patterns.

**FAIL (1 issue):** ledger Governance Floor header mixed template modules (KM, release-gates) into
the `Modules:` field the template reserves for compliance modules and which must mirror the region
stamp (`modules: none`).

**Fix round:** header corrected to `Modules (compliance): none`; template modules moved to a note
+ the module record. **Re-validation: PASS.**

## Phase 6 — Acceptance

User accepted the surface set at **v0.1.0** with the trace summary in hand; no flagged proposals.
Setup complete.
