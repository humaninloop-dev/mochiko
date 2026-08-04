# Architecture tie-back — per-slice/feature `architecture.md` ↔ repo `ARCHITECTURE.md`

**Status:** accepted · bare session (direct `mochiko:analysis-iterative` invocation, un-reviewed;
rulings user-accepted inline; Q4 recommendation contested by the user, lead pushed back once,
user composed the surviving A+C hybrid)
**Date:** 2026-08-04
**Driver:** BACKLOG item "Per-slice `architecture.md` ↔ repo `ARCHITECTURE.md` tie-back"
(2026-08-01; mochiko-app dogfood observation — the design-time `architecture.md` landed in a
spec *slice* subfolder, and no shipped text said how it folds up to the repo-level doc).

---

## Ground facts (grep-verified at session open, post-v0.50.0 surface)

- **F1 — slice placement is already ruled.** `plan.md` Bindings (lines 68–72): slice-scoped
  runs land `plan.md`, `architecture.md`, `tasks.md` under `slices/<slice>/`; "the architecture
  delta seeds from the accumulated feature-root `architecture.md` / `ARCHITECTURE.md`, never
  per-slice from scratch." The dogfood's per-slice nesting is **intended granularity, not a
  placement anomaly** — that half of the backlog item's question was answered by shipped text
  (the item pre-dates the v0.48–0.50 rebuild that shipped it).
- **F2 — the middle level is referenced but unowned.** `plan.md` seeds from an "accumulated
  feature-root `architecture.md`", but no shipped text obliges anyone to *write or accumulate*
  that file: a slice-scoped run writes only under `slices/<slice>/`. The accumulation the seed
  chain depends on had no producer.
- **F3 — the fold seam.** `mochiko:authoring-architecture` carries two duties at a landing
  (landing diff on approved-delta-existed; `ARCHITECTURE.md` fold on built-structural-change).
  Implement lands per slice, so both duties fire per slice landing. The fold targets repo
  `ARCHITECTURE.md` directly; the feature-root file sat between, unowned (F2).
- **F4 — branch locality.** Folds happen in the working tree where implement runs; on a
  feature branch the updated `ARCHITECTURE.md` travels with the code and main's copy stays
  correct for main until merge. "Staleness" is per-branch and self-resolving at merge; the
  real gap is *main-branch visibility of in-flight structural change* (Q3′).

## Decisions

### AT-D1 — feature-root `architecture.md` is a real, accumulated artifact `Confident`

Each slice landing folds its slice's approved delta into the feature-root `architecture.md`
(same in-place-update discipline as the `ARCHITECTURE.md` fold). This gives `plan.md`'s
existing "accumulated feature-root" seed wording (F1) the producer it lacked (F2).
Alternatives rejected: killing the middle level (late slices re-derive state from N prior
deltas); first-slice-file-is-the-feature-root (conflates one slice's artifact with the
feature target).

### AT-D2 — both folds fire at slice landing `Confident`

A slice landing folds the built delta into **both** the feature-root `architecture.md` and
repo `ARCHITECTURE.md`. Repo doc stays current-state-honest per branch (F4); the second fold
is mechanical (same delta). Division of meaning: repo `ARCHITECTURE.md` = current shipped
state; feature-root = accumulated feature *target* (includes approved-but-not-yet-built
later-slice structure once each slice's plan signs off). Rejected: repo fold at feature close
only (violates the current-state contract for the whole multi-slice duration).

### AT-D3 — per-slice nesting confirmed intended `Confident` *(ground-fact confirmation)*

Not a new ruling — F1's shipped text already rules it. Recorded so the backlog item's
"anomaly or intended?" question has an explicit answer: **intended granularity**.

### AT-D4 — `ARCHITECTURE.md` gains a thin "In flight" pointer list `Confident`

User-originated dimension (main-branch reader should see in-flight structural change).
Ruled as a pointer block, not content sections: one line per active feature — feature name +
link to its feature-root `architecture.md` — **added at plan's architecture sign-off, removed
at feature close**. Topology never duplicated into the repo doc; present-tense/current-state
contract intact; merge-conflict surface is one line, not a section; removal rides the same
subtractive-landing discipline as ROADMAP Now. Rejected: full in-progress sections in
`ARCHITECTURE.md` (future-tense state, concurrent-feature merge conflicts, zombie-section
rot); git-only (real visibility gap stands).

### AT-D5 — feature-close diff ruled now, executed later `Confident` — parked

At feature close, before the In-flight pointer is removed, diff shipped code against the
accumulated feature-root target. Catches the descoped/partially-built-slice hole per-slice
diffs cannot see (each slice's diff reads clean or never ran; nothing else checks the *whole*
feature target — worked example: slice 3's approved Avatar Worker dropped, feature declared
done). No owning workflow exists today (feature-close verification is unowned; `audit` is
unscoped), so the obligation **parks as a named line on the audit-scoping backlog item**; the
pointer-removal step (AT-D4) is its natural hook. Until audit lands, the Delivery Slices
section's slice status is the only live check. Steelman for skipping it entirely (slice
bookkeeping suffices) rejected on the mid-run silent-skip case.

### AT-D6 — carriers: A+C hybrid `Contested` — lead recommended commands-first (A), user chose KM-only (C), lead pushed back once, user composed the hybrid

**A carries the executable, event-time obligations (primary):**
- `authoring-architecture` — Duty 2 extension: on a slice-scoped landing the fold targets
  feature-root `architecture.md` **and** repo `ARCHITECTURE.md`; procedure single-sourced here.
- `plan.md` Bindings — one line: In-flight pointer written at architecture sign-off.
- `implement.md` Bindings — one line: feature-root fold at slice landing.
- BACKLOG — the AT-D5 parked line on the audit-scoping item.

**C adds a KM backstop (secondary, between-events):** the knowledge-management module's
`ARCHITECTURE.md` invariants gain one line — *In-flight pointer list: entries added at plan
sign-off, removed at feature close; a pointer to a closed feature, or a closed feature still
pointed at, is a defect — fix on sight* (same shape as the brainstorm-index status-agreement
invariant). Lands in the module template + mochiko's own pinned copy
(`.mochiko/memory/knowledge-management.md`).

No split-brain: the KM line is a consistency check, never a second procedure home. Pure-C
rejected on: (a) non-KM-module projects would lose the fold entirely, breaking F1's seed
chain; (b) a lead composing implement reads command Bindings, not KM invariants, at fire
time; (c) it would split one event's two folds across two governance homes.

## Build surface

1. `patterns-system-design` — no edit expected (delta/seeding language already correct);
   confirm at build.
2. `authoring-architecture` — Duty 2 slice-scoped extension (AT-D6-A). Shipped-primitive
   edit → strip entry + independent author≠grader audit.
3. `plan.md` — one Bindings line (In-flight pointer at sign-off). Same ceremony.
4. `implement.md` — one Bindings line (feature-root fold at landing). Same ceremony.
5. KM module template (`constitution-modules/knowledge-management.md`) + mochiko's pinned
   `.mochiko/memory/knowledge-management.md` — one invariant line each (AT-D6-C).
6. Repo `ARCHITECTURE.md` — no In-flight entries exist yet; the list section is created
   lazily by the first pointer write, not scaffolded empty.
7. BACKLOG — AT-D5 parked line on the `audit` workflow-scoping item.

Token discipline: command edits are 1-line pointers to the single-sourced skill procedure,
per the token-justified-additions rule.

## Build (2026-08-04, same day, plugin v0.51.0)

Landed per the build surface, two corrections at build:

- **All edits were pure additions** — per `primitive-edits.md`, pure additions ride the
  decision row: **no strip entries owed** (the build surface above assumed three; corrected).
- **Auditor-caught defect, fixed round 2:** the dual-target paragraph as first written sat
  under Duty 2 and inherited its built-change gate — a descoped slice's approved delta would
  silently never accumulate into the feature-root target. Rewritten per the auditor's fix
  (matching AT-D2's ruling): feature-root fires on **approved-delta-existed** (Duty 1's
  trigger shape) independent of built structure; "no structural change → no update" scoped
  to repo `ARCHITECTURE.md` only.

Surface: `authoring-architecture` (dual-target fold + In-flight pointer-list section +
description, 1,523/1,536 chars) · `plan.md` + `implement.md` one Bindings line each ·
KM module template + pinned copy invariant line (template's validator-checklist enumeration
and vacuous-at-zero note aligned at audit round 1's FAIL) · `plugin.json` 0.51.0 ·
confirmed-no-edit: `patterns-system-design`, router. Audits author≠grader ×4 (validator
seats, session-model override — persona's pinned `model: opus` unavailable on this key):
plan/implement PASS round 1 · KM template FAIL→fix→PASS · authoring-architecture
FAIL→fix→PASS. Non-blocking advisories recorded in audit outputs (frontmatter
strict-YAML portability — pre-existing repo-wide pattern).

## Open threads

1. **Feature-close diff execution** — parked until `audit` (or whatever owns feature-close)
   exists; the line rides the audit-scoping item (AT-D5).
2. **Concurrent-features watch** — the In-flight list's one-line-per-feature conflict surface
   is believed negligible; confirm at the first project running two features concurrently.
3. **Dogfood** — the whole chain (slice fold → feature-root accumulation → In-flight pointer
   → repo fold) rides the existing architecture-primitive dogfood watch; no new watch item.
