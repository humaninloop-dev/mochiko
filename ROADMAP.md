# Roadmap

## Thesis

Mochiko is the kernel-free successor to human-in-loop: engineering discipline lives in the
quality of the skill library — native agent teams and Workflows orchestrate, and the human is
the framework's primary external validator, present in every loop. Current bet: a small
team-form command surface over a stripped, single-sourced primitive library, hardened by
dogfood evidence rather than speculation. **Target: customer-facing product applications
only — SaaS, web, mobile, desktop — under one asserted production floor, no tier ladder
(production-only, PO-D1–D7, 2026-07-30); backend/service standards are seeded today,
frontend/mobile/desktop shelves are Tier-I roadmap work.**

## Now

- **KM-module redesign dogfood** — fresh-project scaffold + rules-injection probe of the rebuilt module (2026-07-25) → [BACKLOG](BACKLOG.md#operating-docs--km-module)
- **Kinako follow-up run** — waves-1–2 didn't land: investigate, then the UI fix wave through journey gates (2026-07-24) → [BACKLOG](BACKLOG.md#kinako-follow-up-run)

## Next

- Team-form confirm-or-revert — instrumented run or an explicit accept-on-weak-evidence ruling (2026-07-24) → [BACKLOG](BACKLOG.md#pipeline-dogfood--confirm-or-revert)
- Cluster-2 oversight-trace ratification wave (2026-07-24) → [BACKLOG](BACKLOG.md#kinako-follow-up-run)
- Token epic: D5 sizing-gate generalization + the one-shot OTel probe (2026-07-23; probe gains TC's three recorded-open questions) → [BACKLOG](BACKLOG.md#token-reduction-epic)
- Defect probes: fresh-session description-delivery (2026-07-25) → [BACKLOG](BACKLOG.md#defects--empirical-checks)
- Re-evaluate the deferred-kernel shortcut entry once the pipeline dogfoods complete (roadmap-v2's surviving next-step; its convention-extraction clause discharged at the 2026-07-25 migration) → [BACKLOG](BACKLOG.md#token-reduction-epic)
- Tier-I builds queued: security-depth (scoped 2026-07-30, SD-D1–D6) · ops & observability (scoped 2026-07-31, OO-D1–D7); then shelf-build scoping → [BACKLOG](BACKLOG.md#production-only-narrowing)
- Architecture-primitive dogfood — first merged-plan run with the architecture stage (built 2026-07-30 at v0.32.0) → [BACKLOG](BACKLOG.md#architecture-primitive-build-items)

## Later *(non-committed)*

- `audit` workflow scoping (the feature-close verification charter)
- Deferred direction: build-room merge · slice-fold · multi-stream implement
- `/mochiko:graduate` wrapper (on demonstrated shepherding pain)
- Design track (`ui-designer` + its skills)
- Slice spec-amend mode
- Multi-stack / monorepo domain registries (production-only Tier III, PO-D5)
- Distributed-artifact shelf (libraries/SDKs/CLIs) — deferred per PO-D1
- Seat-tiering brainstorm (reliability-gated; untouched by the PO reframe — model-seat sense)
- Token wave-3 candidates (governance/memory layer · brainstorm records)
- Ergonomics sweep: output verbosity (caveman) · plain-language / internal-jargon leak (e.g. "Layer -2") — validator worktree isolation built 2026-08-01 (v0.42.0, trail)

## Standing bets & revisit conditions

- **Hard-require agent teams, no fallback** (`Contested`, 2026-07-04) — revisit: distribution beyond the author's machines.
- **Code-free until dogfooding** (2026-06-27) — revisit: sequential cycle execution proves too slow on a dependency-rich feature.
- **In-loop mesh as Layer-2 default** (`Contested`, 2026-07-25) — revisit: first dogfood of the rewritten Layer 2.
- **Plan absorbs tasks — start-small surface** (`Contested`, 2026-07-25) — revisit: merged-command dogfood.
- **Scribe/report-writer closed** (2026-07-24/25) — reopen: a genuinely expansion-heavy doc surface (groom-pass watch, CHANGELOG first).
- **GLOSSARY.md in KM core** (`Contested`, 2026-07-25) — revisit: dead-doc evidence at module dogfood.
- **KM pin adopts partial core** (recorded deviation, 2026-07-25; narrowed 2026-07-31 — rules file + plugin-scope `ARCHITECTURE.md` exist, GLOSSARY still deferred) — revisit: first in-repo setup/amend run or the module dogfood.

---
*Last groomed: 2026-08-01 — Now 2 · Next 7 · Later 10; open-item figures on BACKLOG's stamp line. Three DONE build rows swept from Now to the trail (residuals in BACKLOG). Shape-v7 full-surface conversion landed v0.43.0 (all six commands; marker retired; trail).*
