# Feature-map granularity & re-parenting — Decision Record

> **Status:** superseded (2026-08-13, user-ruled at the `pm-role-and-feature-derivation` acceptance) — both threads answered there: granularity balance → D3 (extend-beats-mint + soft cap ~9) and D4 (capability tests); re-parenting → D4b (domain-header moves are navigation-only) and D12 (capability-merge mechanics). See that record.
> **When:** 2026-08-12
> **Topic:** two threads on the feature-map layer: (1) balancing map granularity — not too many
> narrow features vs not too many broad ones; (2) what happens when a new feature is better
> understood as a *parent* of existing `in-flight` or `delivered` features — adoption /
> re-parenting of existing entries under a newly minted parent.
> **Lead:** session lead (questioning inline via `mochiko:analysis-iterative`); decisions are the
> user's rulings.

## Ground facts (lead-read from the repo, 2026-08-12)

- **F1 — Doctrine homes.** The map's method lives in `mochiko:authoring-feature-map` (SKILL.md);
  shape in `templates/feature-entry-template.md` + `templates/features-index-template.md`;
  stewardship + lane intake in `plugins/mochiko/commands/feature.md`. Prior rulings:
  `feature-map-layer` D1–D22 and `feature-sizing-and-entry-points` D1–D15 (both accepted,
  built v0.57.0–v0.61.0).
- **F2 — Nesting is a two-level hard cap.** Parent (capability, roll-up, never built directly) +
  leaf (deliverable, the pipeline unit); a flat entry is a leaf. Red flag verbatim: "This
  capability wants a third level — it doesn't get one; split the parent into two parents."
- **F3 — Sizing bars are per-entry only.** One-breath name at parent/flat; leaf extent ≤ ~3 lines
  ("more than ~3 lines of extent usually means this leaf is two features — or a parent waiting
  to be minted"); red flag at "eight extent lines". Checklist has "single-leaf parents
  deliberate". **No map-level balance guidance exists** — nothing bounds top-level parent count,
  leaf-per-parent spread, or flat-entry proliferation.
- **F4 — Retroactive promotion exists, narrow form.** `feature.md` Goal: "a flat entry
  retroactively promoted to parent — the delivered extent becomes the first child, new work
  lands as sibling children, status never regresses." Promotion on ambiguous cases is reserved
  to the user.
- **F5 — Re-parenting is unruled.** No doctrine covers minting a **new** parent over multiple
  **existing** entries (adopting existing `in-flight`/`delivered` features as children).
  `feature.md` Reserved-to-user line says it out loud: "parent selection semantics — unruled;
  surfaced when it bites, never defaulted here."
- **F6 — Status machinery.** Map owns status (`proposed/in-flight/delivered/retired`); delivered
  is sticky; roll-up: parent in-flight when any child is, delivered when all children were
  delivered at earn time; a delivered parent gaining an in-flight child carries it as a Deltas
  line, never regresses. Parent status contradicting children's roll-up = integrity defect,
  fix-on-sight (invariant 7).
- **F7 — Write timing.** Delivery writes (in-flight flips, graduations, delta folds) land only at
  acceptance landings; `/mochiko:feature` stewardship writes (stub minting, promotion, retire,
  grooming) land directly (invariant 6). Stubs are `unrefined` parking — name + hook only;
  maturation is specify-derivation-only.
- **F8 — No live map in this repo.** mochiko itself carries no `FEATURES.md` /
  `.mochiko/features/` — the map layer ships to products using the plugin; a first-live-run
  watch is open in BACKLOG. This session is doctrine-level.

## Decisions

(none yet)

## Open questions

(populated as the session runs)

## Session trail

- **Q1 — driver.** Observed in a dogfood repo; user held the intuition beforehand — the dedicated
  `/mochiko:feature` stewardship command exists because of it. No specific incident artifact
  cited yet (evidence-honesty marker: observed-directional).
