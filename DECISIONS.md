# Decisions — Index

One line per ruled decision, newest first: date · decision · status · rationale home. The fat
rationale lives in the pointed-at session record, or in a `.mochiko/decisions/` record when no
session record exists — never here. Forward view: [`ROADMAP.md`](ROADMAP.md). At groom, a
superseded row compresses to one line, status preserved. Migrated 2026-07-25 from the
pre-migration fat roadmap's Key Decisions (frozen at `.mochiko/archive/ROADMAP.md` —
historical unit: one row per Key-Decisions row; from `operating-docs-maintenance` on: one row
per ruled decision).

| Date | Decision | Status | Rationale |
|------|----------|--------|-----------|
| 2026-07-30 | PO-D1 — target: customer-facing product applications only (SaaS/web/mobile/desktop; application-vs-building-block axis; libraries/SDKs/CLIs deferred) | ruled | [production-only-focus](.mochiko/brainstorms/production-only-focus/record.md) |
| 2026-07-30 | PO-D2 — governance tier axis retired: one asserted production floor + fact-triggered compliance modules | ruled | [production-only-focus](.mochiko/brainstorms/production-only-focus/record.md) |
| 2026-07-30 | PO-D3 — library owns the standard; setup elicits facts; safety floor + modules asserted, architecture-opinion cards stay arbitrated (S7) | ruled | [production-only-focus](.mochiko/brainstorms/production-only-focus/record.md) |
| 2026-07-30 | PO-D4 — waivers reach everything, recorded + auditable; legal-mandate module obligations unwaivable (D4.2); expiry deferred (D4.1) | ruled (`Contested`; expiry `Deferred`) | [production-only-focus](.mochiko/brainstorms/production-only-focus/record.md) |
| 2026-07-30 | PO-D5 — depth agenda in tiers: security + ops hardening + shelf builds ride the narrowing; IaC staged next; multi-repo later; a11y → compliance modules (S8) | ruled | [production-only-focus](.mochiko/brainstorms/production-only-focus/record.md) |
| 2026-07-30 | PO-D6 — identity written into ROADMAP thesis + CLAUDE.md with the backend-seeded-today qualification (S3) | ruled | [production-only-focus](.mochiko/brainstorms/production-only-focus/record.md) |
| 2026-07-30 | PO-D7 — immature-but-in-scope teams served: full floor + recorded-waiver on-ramp; maturity axis rejected road recorded | ruled | [production-only-focus](.mochiko/brainstorms/production-only-focus/record.md) |
| 2026-07-30 | Layer-2 mesh rewrite executed (D1–D3 + rewrite detail → shape v4; six commands conformed at v0.31.0, all independently audit-PASSed; additions incl. the hand-off-not-a-start-signal hold; loop-discipline pass + agent-dispatch peer-edge field escalated) | ruled | [ADR](.mochiko/decisions/2026-07-30-layer-2-mesh-rewrite-executed.md) |
| 2026-07-30 | Shape-home revision encoded as a keeper-skill mode — `authoring-commands` Job 4 + `validation-command-shape` revision checks 11–14 (v0.30.0; unblocks the Layer-2 mesh rewrite dispatch) | ruled | [ADR](.mochiko/decisions/2026-07-30-shape-home-revision-mode.md) |
| 2026-07-25 | OD-D2/D4 naming correction — the **forward view keeps the `ROADMAP.md` name** (no `DIRECTION.md`); the decision index is `DECISIONS.md`; the fat pre-migration roadmap stays frozen in the archive | ruled (user, at build review — corrects the naming fills) | [ADR](.mochiko/decisions/2026-07-25-roadmap-name-carries-forward-view.md) |
| 2026-07-25 | OD-D1 — BACKLOG fails by shape + missing carrier (fixable in kind); ROADMAP is the wrong document kind | ruled | [operating-docs-maintenance](.mochiko/brainstorms/operating-docs-maintenance/record.md) |
| 2026-07-25 | OD-D2 — `ROADMAP.md`: thesis · Now/Next/Later (caps 5/7/10) · standing bets · nothing else | ruled | [operating-docs-maintenance](.mochiko/brainstorms/operating-docs-maintenance/record.md) |
| 2026-07-25 | OD-D3 — decisions layer = thin `DECISIONS.md` index over records + `.mochiko/decisions/` ADRs | ruled | [operating-docs-maintenance](.mochiko/brainstorms/operating-docs-maintenance/record.md) |
| 2026-07-25 | OD-D4 — the fat roadmap form retires; the thin forward view replaces it (named `ROADMAP.md` per the naming correction above); the decisions layer is a standalone sibling | ruled | [operating-docs-maintenance](.mochiko/brainstorms/operating-docs-maintenance/record.md) |
| 2026-07-25 | OD-D5 — BACKLOG open-items-only; done work → append-only trail (`.mochiko/archive/backlog-trail.md`) | ruled | [operating-docs-maintenance](.mochiko/brainstorms/operating-docs-maintenance/record.md) |
| 2026-07-25 | OD-D6 — enforced floor: boundary invariants + subtractive landing (incl. supersession); groom = fix-on-sight skill; minimal stamps | ruled | [operating-docs-maintenance](.mochiko/brainstorms/operating-docs-maintenance/record.md) |
| 2026-07-25 | OD-D7 — enforcement: command steps primary · `paths`-rules touch-time · CLAUDE.md pointers; invariants project-pinned, upgrades as amend offers | ruled | [operating-docs-maintenance](.mochiko/brainstorms/operating-docs-maintenance/record.md) |
| 2026-07-25 | OD-D8 — REGISTRY retires to `.mochiko/archive/`; 13 open rows dispositioned (`authoring-roadmap` abandoned) | ruled | [operating-docs-maintenance](.mochiko/brainstorms/operating-docs-maintenance/record.md) |
| 2026-07-25 | OD-D9 — scope: the redesigned knowledge-management module; core adopted whole + enforcement surfaces; mochiko first dogfood | ruled | [operating-docs-maintenance](.mochiko/brainstorms/operating-docs-maintenance/record.md) |
| 2026-07-25 | OD-D10 — admission rule (read-job + writer moment + carrier); ARCHITECTURE.md + GLOSSARY.md join core; CHANGELOG/RUNBOOK elective | ruled (glossary-core `Contested`) | [operating-docs-maintenance](.mochiko/brainstorms/operating-docs-maintenance/record.md) |
| 2026-07-25 | OD-D11 — two new skills (`grooming-operating-docs`, `authoring-architecture`); zero new commands/agents | ruled | [operating-docs-maintenance](.mochiko/brainstorms/operating-docs-maintenance/record.md) |
| 2026-07-25 | OD-D12 — report-writer held per model-tiered-seats; re-open watch rides the groom pass | ruled | [operating-docs-maintenance](.mochiko/brainstorms/operating-docs-maintenance/record.md) |
| 2026-07-25 | Team-method contrast (D1–D5): in-loop mesh as Layer-2 default · cold review a stage property · devolved clean-cycle verdicts · plan absorbs tasks · one package acceptance | ruled (D1/D4 `Contested`) | [team-method-vs-command-shape](.mochiko/brainstorms/team-method-vs-command-shape/record.md) |
| 2026-07-25 | Succinctness bands by authorship era | ruled | [ADR](.mochiko/decisions/2026-07-25-succinctness-bands-by-era.md) |
| 2026-07-25 | Skill-succinctness strip pass (D1–D5; executed v0.24.0–v0.28.0, −23.7% across 27 skills) | ruled | [skill-succinctness-strip](.mochiko/brainstorms/skill-succinctness-strip/record.md) |
| 2026-07-24 | Model-tiered seats (D1–D6): class-keyed cheap explorer; scribe a non-avenue; seat-tiering reliability-gated | ruled | [model-tiered-seats](.mochiko/brainstorms/model-tiered-seats/record.md) |
| 2026-07-24 | Token-reduction wave-2 build rulings (artifact layer, v0.23.0) | ruled | [ADR](.mochiko/decisions/2026-07-24-token-reduction-wave-2.md) |
| 2026-07-23 | Standing-seat lifecycle (D1–D4): conditioned checkpoint recycling; transport-vs-lifecycle Layer-2 reframe (lands v4+) | ruled | [standing-seat-lifecycle](.mochiko/brainstorms/standing-seat-lifecycle/record.md) |
| 2026-07-23 | Token-reduction wave-1 build rulings (report layer, v0.22.0) | ruled | [ADR](.mochiko/decisions/2026-07-23-token-reduction-wave-1.md) |
| 2026-07-23 | Workflow token reduction — the efficiency epic (D1–D6, pure-waste-first) | ruled | [workflow-token-reduction](.mochiko/brainstorms/workflow-token-reduction/record.md) |
| 2026-07-21 | Domain-dependency allowlist (D1–D5; built v0.18.0) | ruled | [domain-dependency-allowlist](.mochiko/brainstorms/domain-dependency-allowlist/record.md) |
| 2026-07-19 | Governance-injection probe suite (v0.12.0) | ruled | [ADR](.mochiko/decisions/2026-07-19-governance-injection-probe.md) |
| 2026-07-18 | Pattern codification + library minimalism (D1–D9: shape home `command-shape.md`, keeper pair, strip notes, cluster waves) | ruled | [pattern-codification-and-minimalism](.mochiko/brainstorms/pattern-codification-and-minimalism/record.md) |
| 2026-07-18 | Setup G3 adversarial review + the `validation-*`/`review-*` split (D1–D7) | ruled | [setup-adversarial-review](.mochiko/brainstorms/setup-adversarial-review/record.md) |
| 2026-07-18 | Team-transport legibility (`agent-dispatch.md` v3 + addressability check; no-fallback stands) | ruled | [setup-v3-team-defect](.mochiko/brainstorms/setup-v3-team-defect/record.md) |
| 2026-07-18 | Transformer cluster retired | ruled | [ADR](.mochiko/decisions/2026-07-18-transformer-cluster-retired.md) |
| 2026-07-18 | Setup-v3 build ruling — brainstorm team idiom adopted | ruled | [ADR](.mochiko/decisions/2026-07-18-setup-v3-team-idiom.md) |
| 2026-07-18 | Constitution dissolution — governance on native surfaces (D1–D8; no `constitution.md`) | ruled | [constitution-native-surfaces](.mochiko/brainstorms/constitution-native-surfaces/record.md) |
| 2026-07-17 | Knowledge-management module v1 (elective four-part bundle, D1–D7) | superseded by OD-D9/D10 (2026-07-25) | [setup-operating-docs-scaffolding](.mochiko/brainstorms/setup-operating-docs-scaffolding/record.md) |
| 2026-07-16 | Setup v2 — constitution from interrogated intent (D1–D9: interrogation → ratified synthesis → traceable authoring) | ruled | [setup-constitution-flexibility](.mochiko/brainstorms/setup-constitution-flexibility/record.md) |
| 2026-07-16 | Setup-v2 build rulings (brainstorm command shape + team-form substrate) | ruled | [ADR](.mochiko/decisions/2026-07-16-setup-v2-build-rulings.md) |
| 2026-07-16 | Brainstorm v2.2 — the sized lens-split review | ruled | [brainstorm-v2-2-revision](.mochiko/brainstorms/brainstorm-v2-2-revision/record.md) |
| 2026-07-05 | Brainstorm v2.1 — the end-stage review pair | ruled | [brainstorm-v2-revision](.mochiko/brainstorms/brainstorm-v2-revision/record.md) |
| 2026-07-05 | Fact-checker seat kept + renamed (né grounder) | ruled | [fact-checker-seat](.mochiko/brainstorms/fact-checker-seat/record.md) |
| 2026-07-04 | Brainstorm v2 — the agent-team pilot (hard-require teams, no fallback, `Contested`) | ruled (engagement model revised → v2.1/v2.2) | [brainstorm-command-rewrite](.mochiko/brainstorms/brainstorm-command-rewrite/record.md) |
| 2026-07-02 | Brainstorm v1 design | superseded by v2 (2026-07-04) | [brainstorm-command](.mochiko/brainstorms/brainstorm-command/synthesis.md) |
| 2026-07-02 | Slice workflow / vertical-graduation substrate (`slices.md` overlay + slice-scoped entry variants) | ruled | [vertical-graduation](.mochiko/brainstorms/vertical-graduation/synthesis.md) |
| 2026-06-30 | Command altitude / single-sourcing — a command references doctrine, never restates it | ruled | [command-altitude](.mochiko/brainstorms/command-altitude/synthesis.md) |
| 2026-06-27 | Agent↔workflow decoupling by absence (keystone test; caller-side context in `agent-dispatch.md`) | ruled | [agent-decoupling](.mochiko/brainstorms/agent-decoupling/synthesis.md) |
| 2026-06-27 | Roadmap-v2 re-baseline: human-validator thesis · kernel deferred · loop-discipline skill+template · producer↔validator pairing · workflow-first order · gates/memory/gap-routing · sequential implement | ruled (migration-tracking clause superseded 2026-07-25) | [ADR](.mochiko/decisions/2026-06-27-roadmap-v2-rebaseline.md) |
