# Transcript — setup-body-r2

Simulated `/mochiko:setup` for greenfield **Ledgerline**. Variant: `body`. Replicate: r2.
Run-lead plays every seat; the principal is answered only by fresh subagents reading the frozen
persona card (the run-lead never reads the card). Skill bodies loaded from the designated
per-variant paths before each seat's work (see `invocations.json` / `costs.json`).

---

## Seat: interrogation (lead, inline) — `mochiko:analysis-iterative` (body variant)

Skill body loaded: `variants/body/analysis-iterative/SKILL.md`. Agenda:
`plugins/mochiko/skills/authoring-constitution/references/INTERROGATION-AGENDA.md`. One question
per turn, adapted; batched by dimension into principal spawns. Deck:
`references/catalog/` (universal-floor + backend-service).

### Batch 1 — dimensions 1 (identity/intent), 2 (fact profile), 3 (type)

**Q1 (identity/intent):** What are you building, who for, expected lifespan?

**Principal:** Ledgerline is an invoicing and payment-tracking tool for small independent
contractors — plumbers, electricians, freelance designers. They create clients, issue invoices,
track whether they've been paid, send reminders, and get a simple cash-flow view. US market,
small solo operators currently using spreadsheets or nothing. Building it to last — this is my
company, not a side project. Production quality from day one because it holds real people's
financial data.

**Q2 (fact profile):** Industry, data classes, jurisdictions/markets, contractual commitments?

**Principal:** SaaS for small-business/contractor finance — not a regulated space as far as I
know. I store names, emails, addresses, invoice amounts, payment status. No card numbers —
Stripe handles checkout on their hosted pages, so cardholder data never touches my system. US
only, no international at launch. No active contractual obligations — one prospect's procurement
form mentioned SOC 2, but nothing signed or required today.

**Q3 (type):** Frontend / backend / fullstack / service / mobile / desktop?

**Principal:** Fullstack. Python and FastAPI backend, PostgreSQL, React frontend — a contractor
friend helps part-time with the UI. Deploying on a managed host, probably Render.

### Batch 2 — dimensions 4 (risk), 5 (team), 6 (existing practices)

**Q4 (risk surface):** What does failure cost — data loss, money, reputation, trust, legal?

**Principal:** Worst case is I lose or leak a contractor's financial data — their client list,
what they've billed, who owes them. That kills trust instantly; for a solo founder trust is the
whole business. Money moving wrong scares me less — Stripe handles the charge — but if my
payment-status tracking says paid when it isn't, the contractor makes decisions on bad numbers
and blames me. Legal exposure I'm honestly not sure about; it's financial records, so I assume
some liability if I lose them, but I couldn't name a statute. Reputation and trust are what keep
me up.

**Q5 (team reality):** Solo or team, experience mix, review culture?

**Principal:** Basically me. Full-time, six years backend Python and some Node, mid-level, know
REST and migrations but never run a production SaaS by myself. A friend does the React UI ~5
hours a week. No ops person, no one on call but me. No code review culture — no one to review.
Whatever ships has to be operable by one person.

**Q6 (existing practices/tools):** Intended stack/tooling, CI, linters, tests — actual commands?

**Principal:** Python/FastAPI, Postgres, React. Deploy on Render (used it for toy stuff).
Haven't set up tooling yet — it's greenfield. pytest for tests. Linter/formatter — probably
ruff, I've heard it does both. No CI pipeline stood up. What's the normal setup for a stack
like this?

### Batch 3 — dimension 6 close (toolchain recommend-then-arbitrate), 7 (KM), 8 (deploy/release)

**Q6b (lead recommends toolchain, CI-enforced since solo):** ruff (lint+format), mypy, pytest +
pytest-cov; ESLint + Prettier + vitest on the React side; gitleaks (secrets), pip-audit + npm
audit (deps); all blocking in GitHub Actions. Accept or change?

**Principal:** Take it — all of it, blocking merge. No reviewer, so the machine is the reviewer.
One caveat: don't make gates so strict I can't ship — is mypy going to fight me on FastAPI? If
it's a clean setup, fine; otherwise all in.

**Q7 (knowledge management):** Adopt the operating-docs core (default-on)? Electives CHANGELOG /
RUNBOOK?

**Principal:** Yes to core — keeping a long-lived thing coherent in one head is exactly my
problem. CHANGELOG, sure, cheap, add it. RUNBOOK — I don't know what I'd put in it yet; if it's
a low-effort skeleton for later, add it, otherwise punt till after launch.

**Q8 (deployment & release reality):** Environments, cadence, release blockers, rollback?

**Principal:** Just prod at first, but staging if it's cheap on Render — catching a bad
migration before it hits real invoices is worth something. Cadence: whatever I'm ready to ship,
several times a week early, no schedule. Blockers: all CI gates green and nothing broken in the
invoicing core — I won't ship that shaky. Rollback: this is where I'm shaky — need to undo a bad
deploy fast, including anything touching the database, but I don't know what good rollback looks
like for Render + Postgres. What should I have?

### Batch 4 — deck arbitration (A), release gates (B), fact confirmations (C), values (D), exclusions (E)

**A — backend-service cards (recommend-then-arbitrate):**

- **(i) BE-HEX (hexagonal):** lead recommends a right-sized version — ports around Stripe and
  the DB only, not ceremony everywhere. **Principal ruling:** KEEP, tightened — ports on Stripe
  and the DB only, no import-linter ceremony around the web layer or anything else. Reason: test
  invoicing logic without a live Stripe or real DB, and I might swap Render or Stripe someday, so
  those two seams are worth it.
- **(ii) BE-SRP (single responsibility, cyclomatic ≤10):** **Principal ruling:** KEEP. Cheap,
  the linter does it, keeps me honest solo. Cyclomatic 10 to start; bump if it blocks something
  legitimately gnarly.
- **(iii) BE-DEP (dependency discipline, block high/critical):** **Principal ruling:** KEEP, no
  change. Right for financial data; pinning and justifying deps is sane.

**B — release gates:** lead recommends staging→prod with a soak, reversible/backward-compatible
(expand-contract) migrations, Render one-click rollback < 15 min, destructive migrations flagged
in PR + explicit self-approval. **Principal:** Mostly yes. Expand-contract migrations — that's
the "fast undo including the DB" I wanted, didn't know the name. Rollback < 15 min — yes.
Destructive-migration flag + my explicit approval — yes, forces me to look twice. **Caveat:**
staging only if cheap on Render; keep the soak SHORT — minutes, smoke-test level — or skippable
for small changes. Don't let the soak become a gate that stops me shipping several times a week.

**C — fact-profile confirmations (consequence-stated):**

- No cardholder data (Stripe hosted) → **PCI-DSS will NOT attach.** — Correct.
- No health/medical data → **HIPAA will NOT attach.** — Correct.
- US-only, no EU/UK residents → **GDPR will NOT attach.** — Correct.
- No signed attestation commitment → **attestation module will NOT attach now**; revisit trigger
  = a customer contract later requiring SOC 2. — Correct; note it, don't build for it.
- Customer-facing React UI served to US users → US accessibility law (ADA) applies →
  **accessibility (WCAG) obligation ATTACHES** as a baseline (WCAG 2.1 AA, automated checks in
  CI). — Fact confirmed (contractors use a React app in the browser). Keep the obligation to the
  automated axe-style CI baseline; NOT manual audits / design ceremony pre-launch. Don't
  gold-plate it.

**D — values & non-negotiables (dimension 9):**

**Principal:** (1) Contractors' financial data does not leak and does not get lost — that's the
whole trust of the product; enforce anything that protects it. (2) Payment status has to be
accurate — if the app says paid and it isn't, I'm done. (3) Secrets never in the repo. (4) The
whole thing stays operable and shippable by one person. **Refuses:** gates so strict/slow I
can't ship several times a week; heavyweight team process (SLO ceremony, NOC-style
incident-response runbooks, enterprise attestation nobody's paying for yet). Right-size
everything to solo.

**E — deliberate exclusions (dimension 10):**

**Principal:** No SOC 2 / attestation work until a contract requires it. No multi-tenancy / teams
governance — single-tenant-per-account at launch, but don't foreclose a bookkeeper seat later.
No formal SLOs or incident-response process — Sentry for errors, revisit ops maturity after
launch. Data retention/deletion — I don't know my obligations (how long invoices must be kept,
what happens on account deletion); don't hard-code a retention policy into governance now — flag
it as an open question to resolve before launch, NOT a gate today.

---

## Convergence note

All ten dimensions were worked; no dimension was skipped by an earlier answer (no convergence
skips). Deck: universal-floor asserted (four floor cards), backend-service shelf arbitrated
(BE-HEX kept-tightened, BE-SRP kept, BE-DEP kept). Frontend shelf: floor-example seed is
backend-flavored (catalog seed-honesty note) — floor categories translated to the React surface
during the session rather than copied.

(Synthesis assembled at `project/.mochiko/memory/governance-intent.md`; cold intent review,
ratification, authoring, validation, and finalize recorded below and in the run's project tree.)

---

## Cold intent review (blind-map dispatch, before ratification)

**Sizing:** default is a pair; lead sized to solo (departure trail in the synthesis Review
section) — greenfield, no codebase reality-surface for a second lens; compact synthesis.

**Dispatch (two messages):** message 1 gave the reviewer topic/identity/goal only; it built a
blind angle map covering all ten dimensions (name-signal flag: "Ledgerline" reads financial →
watch the fact profile). Message 2 sent the frozen synthesis path; the reviewer diffed against
its map and ran the five hunt classes.

**Reviewer result:** 11 raised → 8 survived. Recommended status: **needs-revision**. Blind-map
coverage: all ten dimensions were worked — survivors are depth/consistency/mechanical-attachment
gaps, not missed dimensions.

- **F1 (Important):** cross-tenant / object-level authz absent — FLOOR-SEC covered authN only.
- **F2 (Important):** GDPR declined on "US only", conflating US users with the client data subjects.
- **F3 (Important):** a11y automation-only axe checks can't verify AA — reads as a hidden partial waiver.
- **F4 (Important):** Sentry vs no-PII-in-logs — PII/financial payloads egress to a third party.
- **F5 (Important):** floor obligations left to solo discipline, unenforced by any listed gate.
- **F6/F7/F8 (Minor):** retention-vs-backups tension; no-IR vs day-one financial framing; toolchain/mypy strictness unspecified.

**Lead disposition:** F1/F3/F4/F5 folded as floor-expression strengthenings; F6/F7/F8 recorded
as notes; F2 routed to the principal as a fact question. (Full dispositions in the synthesis
Review section.)

---

## Synthesis ratification (principal spawn — F2 + revised synthesis)

**Interviewer:** [F2] Do you expect contractors to bill EU/UK clients, or store EU/UK residents'
PII at launch? [B] Accept/amend/reject the revised synthesis (four floor-strengthening folds +
three minor notes)?

**Principal:** [F2] No — my contractors are US operators billing their own local US clients; not
targeting non-US at launch, don't expect EU/UK data. Don't attach GDPR now; record a revisit
trigger for if I go international. [B] Accept, all of it — no amendments. (Condition noted: the
WCAG manual-audit gap stays backlog-tracked, not a launch gate.)

Synthesis frozen for authoring.

---

## Authoring (authoring-constitution, greenfield)

Produced the governance surface set under `project/`: CLAUDE.md governance region (v1.0.0 stamp,
12 principle/index lines, tech stack, quality gates with real commands, governance operations
incl. output-style switch line + standing new-file read line), `.claude/rules/mochiko/`
(layers.md two-seam + pydantic domain-registry, operating-docs.md, output-style.md), the
governance ledger (Three-Part records GI-003..017, waivers "None.", amendment policy,
release-gates detail, domain-dependency policy, trace summary), and the project-pinned
`.mochiko/memory/knowledge-management.md`.

---

## Independent validation (validation-constitution, real subagent, default FAIL)

**Round 1 — FAIL.** Two blocking issues: (1) region ratified-stamp module list did not match the
ledger floor header (only a11y listed); (2) layers.md missing a `## Project Structure` tree.
Two non-blocking advisories (new-file read-line path precision; the 60% coverage override, noted
as the founder's recorded choice).

**Fix round.** Stamp extended to enumerate all attached modules; Project Structure tree added;
read-line paths corrected to only those with matching rules files.

**Round 2 — PASS.** Both fixes verified. One advisory noted for the ratifier (coverage 60%
blocking is a recorded session override).

---

## Finalize — KM scaffolding + acceptance

Scaffolded the operating-docs set on the never-overwrite floor: ROADMAP, BACKLOG (carrying the
two pre-launch open items — data-retention/deletion, WCAG manual-audit gap — + mypy ratchet),
DECISIONS + `.mochiko/decisions/2026-08-10-governance-ratified.md`, ARCHITECTURE (planned
topology), GLOSSARY, CHANGELOG, RUNBOOK (skeleton + data-at-risk stub), FEATURES (empty index),
brainstorms/specs indexes, backlog-trail.

**Governance-injection probe:** offered at finalize; founder DECLINED (fresh empty repo).

**Final acceptance (principal spawn):** presented the trace summary + validation PASS + the doc
set. **Principal:** "Accept. This is v1.0.0. … Skip the injection probe. … Ship it."

Setup complete. (Per setup.md: commits suggested to the user, no git mutations run.)
