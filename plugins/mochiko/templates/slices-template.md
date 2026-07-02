# Slices Template

This template defines the canonical structure of `slices.md` — the graduation-slice decomposition
overlay a producer authors over an accepted `spec.md`. It is the single source of the `slices.md`
structure: `authoring-slices` conforms to it, `validation-slices` grades against it, and the
**Graduation contract** section is what downstream slice-scoped runs honor. Square-bracket `[...]`
spans are model-fill targets.

---

```markdown
# Graduation Slices — [feature title]

> Feature: [feature-id]
> Spec stamp: stories [US-1, US-2, … — the FULL story-ID set of spec.md] · spec status [accepted] · stamped [YYYY-MM-DD]
> Slices: [N] ([foundation-slice id] foundation + [N-1] feature)

---

## Slice order

| # | Slice | Type | Stories | Depends on |
|---|-------|------|---------|------------|
| 1 | S1 — [name] | Foundation | [US-#, US-#] | — |
| 2 | S2 — [name] | Feature | [US-#, US-#] | S1 |
| 3 | S3 — [name] | Feature | [US-#] | S1, S2 |

> Pipeline stage is **derived, never stored**: a slice's position is read from which artifacts
> exist under `slices/<id>/` (no plan.md = not designed; plan.md = designed; tasks.md = tasked;
> tasks all `[x]` = shipped).

---

## Slices

### S1: [name]  (Foundation)

> Stories: [US-#, US-#]
> Depends on: none
> Value seam: [the user journey this slice delivers, observable on its own]

- **Rationale**: [why these stories graduate together — and, foundation only: which shared design
  core (entities, contracts, constraints) this slice establishes AND what testable journey it
  still delivers]
- **Extend obligations**: [US-# homed in S_: what this slice must extend when it graduates | none]

### S2: [name]  (Feature)

> Stories: [US-#, US-#]
> Depends on: [S1]
> Value seam: [the user journey this slice delivers]

- **Rationale**: [why these stories graduate together; anything above 4 stories justified here]
- **Extend obligations**: [… | none]

---

## Cross-cutting placements

| Story | Home slice | Obligated slices | Obligation |
|-------|-----------|------------------|------------|
| [US-#] | [S#] (earliest meaningful) | [S#, S#] | [what each later slice must extend] |

*(Omit this section when no story is cross-cutting.)*

---

## Feature-Done  *(declared now — executed at feature-close, when every slice has shipped)*

### SC coverage map

| SC | Verifying slice(s) | How verified |
|----|--------------------|--------------|
| [SC-#] | [S#] | [the observable check that covers this criterion] |

### Cross-slice seams

| Seam | Between | What must hold |
|------|---------|----------------|
| [name] | [S# ↔ S#] | [the cross-slice behavior per-slice verification cannot cover] |

> Feature-close verification executes this section against real infrastructure once all slices
> show shipped. No workflow owns that pass yet — until one does, run it manually (it is
> deliberately executable as written). The feature is **not done** when the last slice ships;
> it is done when this section passes.

---

## Graduation contract  *(how downstream slice-scoped runs honor this file)*

- **Slice-scoped runs** — with this file present, the design → tasking → implementation stages run
  **per slice, in Slice-order**: each stage resolves the current slice (named in its argument, else
  the first slice in order lacking that stage's artifact) and scopes itself to that slice's
  stories **plus its extend obligations** — nothing else.
- **Staleness guard** — before any slice-scoped run: compare the live `spec.md` story-ID set to the
  Spec stamp above. Any mismatch → halt; the decomposition must be redone before any slice
  proceeds.
- **Artifact layout** — shared design artifacts live at the feature root and **accumulate** across
  slices (`requirements.md`, `constraints-and-decisions.md`, `nfrs.md`, `data-model.md`,
  `contracts/`, `quickstart.md`); per-slice artifacts live under `slices/<id>/` (`plan.md`,
  `task-mapping.md`, `tasks.md`, cycle reports, round reports, filled contracts).
- **Extend-mode** — a later slice's design treats the accumulated shared artifacts as brownfield
  input: read first, **extend in place, never re-derive** and never fork per-slice copies.
- **Graded amendment** — an **additive** extension (new entity, attribute, endpoint) is routine
  extend-mode work. A **breaking** change to design an earlier slice already shipped as code is an
  explicit amendment: surfaced as a `[MODIFY]` design change for that run's review — never a
  silent rewrite — with its migration carried as tasks in the *current* slice's breakdown.
  Repeated breaking amendments against the same design are a re-decomposition signal, not routine.
- **Regression safety** — earlier slices' tests live in the repository; every slice's quality
  gates run the full suite, so an amendment that breaks shipped behavior surfaces by construction.
```

---

## Usage Notes

1. **Fill style**: `[...]` spans are model-fill targets; replace every one — no brackets remain in
   a finalized `slices.md`.
2. **The overlay is derivative.** It references `spec.md` content by ID (`US-#`, `SC-#`) and never
   restates, splits, or amends it. The Spec stamp is what keeps the derivation honest — it must
   list the spec's **full** story-ID set, exactly.
3. **No status fields.** Neither the file header nor any slice carries a stage/status column;
   pipeline position is always derived from `slices/<id>/` artifact presence (workspace-as-state).
4. **The Graduation contract travels with the artifact.** Downstream consumers read this file
   anyway; the contract section is the single place the consumption rules live — commands
   reference it rather than restating it.
5. **Feature-Done is authored at decomposition time** so the feature-level done-condition exists
   before any slice graduates. Every `SC-#` in the spec must appear in the coverage map — an
   uncovered SC is a decomposition gap, not a footnote.
6. **Null exit produces no file.** A spec that should run whole-spec gets a recommendation in the
   producer's report and **no** `slices.md` — absence of this file *is* the whole-spec state.
7. **Output location** — `.mochiko/specs/<feature>/slices.md`, alongside the spec it indexes.
