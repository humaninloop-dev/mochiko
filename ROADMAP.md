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

- **Guardrails-vs-detail residuals** — Wave 1 DELIVERED 2026-08-11 at v0.63.0 (11 guardrails bodies + slim descriptions, 6 prose-only agents, D7 char-budget cost gate live, four floor lines; 17/17 audits PASS round 1; trail); remaining: first-live-run watches (F-X1 review-evidence · slim-description fire-rate · M1 · M2) + the `analysis-codebase` dangling-pointer repair + Wave 2 editorial extension (17 skills + 4 agents, commands excluded) → [BACKLOG](BACKLOG.md#guardrails-vs-detail-residuals)
- **Feature-sizing & entry-points residuals** — build DELIVERED 2026-08-10 at v0.61.0 (map nesting, `/mochiko:feature`, plan/implement re-key spec→feature, two-altitude design surface with graded folds; breaking change; trail); remaining: the first-live-run watch + setup baseline-bootstrap hardening → [BACKLOG](BACKLOG.md#feature-sizing--entry-points-residuals)
- **Cold-review gap-challenge residuals** — build DELIVERED 2026-08-10 at v0.60.0 (blind angle-map phase, coverage findings, reopen routing, both review clusters; trail); remaining: the first-live-run watch → [BACKLOG](BACKLOG.md#cold-review-gap-challenge-residuals)
- **Feature-map layer residuals** — build DELIVERED 2026-08-10 at v0.57.0–v0.59.0 (slices retired, feature = pipeline unit, `product-manager` agent, FEATURES.md map; trail); remaining: the self-application governance amend (user-gated) + the first-live-run watch (note: first-live-run watch partially overtaken by the sizing/entry-points re-key — watch items re-scope at that build) → [BACKLOG](BACKLOG.md#feature-map-layer-residuals)
- **KM-module redesign dogfood** — fresh-project scaffold + rules-injection probe of the rebuilt module (2026-07-25; partial credit 2026-08-06: in-repo brownfield setup ran dimension-7 core+electives, collision beat vacuous — fresh-scaffold half + injection probe remain) → [BACKLOG](BACKLOG.md#operating-docs--km-module)

## Next

- Cluster-2 oversight-trace ratification wave (2026-07-24) → [BACKLOG](BACKLOG.md#kinako-follow-up-run)
- Token epic: the one-shot OTel probe (2026-07-23; D5 sizing half closed 2026-08-04 — superseded at v8; probe gains TC's three recorded-open questions) → [BACKLOG](BACKLOG.md#token-reduction-epic)
- Defect probes: fresh-session description-delivery (2026-07-25) → [BACKLOG](BACKLOG.md#defects--empirical-checks)
- Re-evaluate the deferred-kernel shortcut entry once the pipeline dogfoods complete (roadmap-v2's surviving next-step; its convention-extraction clause discharged at the 2026-07-25 migration) → [BACKLOG](BACKLOG.md#token-reduction-epic)
- Tier-I builds queued: security-depth (scoped 2026-07-30, SD-D1–D6) · ops & observability (scoped 2026-07-31, OO-D1–D7); then shelf-build scoping → [BACKLOG](BACKLOG.md#production-only-narrowing)
- Architecture-primitive dogfood — first merged-plan run with the architecture stage (built 2026-07-30 at v0.32.0); tie-back chain rides it (built 2026-08-04 at v0.51.0, AT-D1–D6) → [BACKLOG](BACKLOG.md#architecture-primitive-build-items)

## Later *(non-committed)*

- `audit` workflow scoping (the feature-close verification charter)
- Deferred direction: build-room merge · multi-stream implement (slice-fold delivered 2026-08-02 at v0.49.0)
- `/mochiko:graduate` wrapper (on demonstrated shepherding pain)
- Design track (`ui-designer` + its skills)
- Multi-stack / monorepo domain registries (production-only Tier III, PO-D5)
- Distributed-artifact shelf (libraries/SDKs/CLIs) — deferred per PO-D1
- Seat-tiering brainstorm (reliability-gated; untouched by the PO reframe — model-seat sense)
- Token wave-3 candidates (governance/memory layer · brainstorm records)
- Plain-language sweep (internal-jargon leak, e.g. "Layer -2") — the rule's home ruled into the style home 2026-08-01 (D3); the sweep work itself remains — the output-verbosity half graduated to the Now build row

## Standing bets & revisit conditions

- ~~**Hard-require agent teams, no fallback**~~ (`Contested`, 2026-07-04) — **superseded 2026-08-02** by transport-neutral commands (command-architecture-realignment D5): teammates vs subagents is the lead's per-seat call.
- **Code-free until dogfooding** (2026-06-27) — revisit: sequential cycle execution proves too slow on a dependency-rich feature.
- **In-loop mesh as Layer-2 default** (`Contested`, 2026-07-25) — revisit: first dogfood of the rewritten Layer 2.
- **Plan absorbs tasks — start-small surface** (`Contested`, 2026-07-25) — revisit: merged-command dogfood.
- **Scribe/report-writer closed** (2026-07-24/25) — reopen: a genuinely expansion-heavy doc surface (groom-pass watch, CHANGELOG first).
- **GLOSSARY.md in KM core** (`Contested`, 2026-07-25) — revisit: dead-doc evidence at module dogfood.
- ~~**KM pin adopts partial core**~~ (recorded deviation, 2026-07-25) — **discharged 2026-08-06**: the first in-repo setup run ratified the pin into the ruled core (governance v1.0.0, GI-009); residual deviations (GLOSSARY deferral · specify.md landing step) carried in the pin with their own triggers.

---
*Last groomed: 2026-08-06 (backlog groom; same-day touches: specify.md KM-landing defect
closed at v0.54.0 — audit PASS, pin deviation struck, gates 4+5 executed first time,
marketplace synced · kinako waves-1–2 investigation retired by user ruling, its *Now* row
dropped, ADR `2026-08-06-kinako-waves-retired` · 2026-08-07: plugin output-styles session
accepted AND built same day at v0.55.0 — two native output styles + router section, audit
PASS round 1, item straight to trail · 2026-08-07: implement bounds contract booked HIGH ×2
from the mochiko-app S1 run evidence AND closed same day at v0.56.0 — Bounds + Escalation
cadence bullets in `implement.md` Harness, audit FAIL→fix→PASS, both items straight to
trail; 2026-08-10: feature-sizing & entry-points session accepted — D1–D15, Now row added ·
2026-08-10: pm-requirements-stacking session accepted AND built same day at v0.62.0 —
D1–D4 + D2a/D3a, skill supersession + three command additions, audit PASS round 1 all four
surfaces, item straight to trail · 2026-08-10: validator-scope-and-verbosity session
accepted — D1–D8, guardrails-vs-detail benchmark Now row added, skill-succinctness-strip
row method-supersession annotated) —
Now 5 · Next 6 · Later 9 · BACKLOG
baseline 56 open items (per-item ≤15 lines;
figures on BACKLOG's stamp line). Plugin: 6 commands · 10 agents · 28 skills · 14 templates +
modules · 2 output styles, v0.62.0 (marketplace synced). Per-bump build history: `CHANGELOG.md` (from 0.53.0) ·
`DECISIONS.md` · the trail; pre-0.53.0 stamp-line narratives live in git history (this line,
before this groom). Stamp compressed to contract shape this groom — expansion-watch hit
logged in BACKLOG (Operating-docs / KM module section) for the user's report-writer re-open
call.*
