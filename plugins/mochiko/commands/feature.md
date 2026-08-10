---
description: Front door to the feature map — steward entries, triage bugs and improvements by the stable-ground test, author the delta card, and dispatch delivery to the re-keyed pipeline.
disable-model-invocation: true
---

# Feature — Map Stewardship & Delivery Lane

**Goal:** resolve `$ARGUMENTS` (a map query, a capability idea to park, a promotion or
retire ask, or a bug/improvement report) through exactly one of the command's remits —
map stewardship, or lane intake ending in a dispatch. Empty → ask the user what they need.

## Goal

The request landed in its remit. **Stewardship:** a map query answered from the actual
files — `FEATURES.md` plus the entries in the territory asked about, never memory of
them · a capability idea parked as a `proposed` stub — name + one-breath hook only,
marked `unrefined`; a stub is parking, never a spec-bypass — selectability stays behind
`/mochiko:specify`'s derivation · a flat entry retroactively promoted to parent — the
delivered extent becomes the first child, new work lands as sibling children, status
never regresses · a retire executed on the user's ruling, entry kept and dated · an
integrity defect fixed on sight. **Lane intake:** the report triaged by the
stable-ground test — the lane writes only surfaces no live run owns — into the feature
lane, the product lane, or filed to the owning run; lane work captured as **one delta
card** — a bug's acceptance is its reproduction-failing test, an improvement carries 1–3
acceptance criteria — plus the minimal enumerated `baseline-delta.md` (appliable
before/after form) when a product-baseline touch is known at intake; the card handed to
the re-keyed pipeline as **delta scope**, where it executes under plan/implement's own
bounds, verification seats, and evidence rules — this command runs no delivery harness.
The map delta the work leaves behind is what the boundary is audited from.

**Not done — default FAIL:** a stub minted with extent or relations filled, or missing
the `unrefined` mark · lane work that mints an entry, promotes to parent, or flips
status — the map-write test failed; it routes to `/mochiko:specify` · a mid-run
outgrowth widened in place instead of aborted and re-routed · a report keyed to an
in-flight feature's surface run as lane work instead of filed to the owning run · a
second live product-lane run · a known baseline touch with no `baseline-delta.md`
authored at intake · any bounds, verification, or evidence discipline restated here
instead of referenced · a retire, or a promotion on an ambiguous case, executed without
the user's ruling.

## Harness

- **You are the lead.** Plan the run and orchestrate it toward the Goal; teammates or
  subagents per seat is your call. Stewardship writes are bookkeeping edits on the live
  map; lane delivery is never yours — it belongs to the dispatched run.
- **Triage — the stable-ground test.** Key the report to its surface, then check the
  ground. The check inputs are **files**: entry status at the feature level; the
  in-flight feature dirs' enumerated baseline deltas at the product level. Keying a raw
  report to its surface is triage judgment, audited from the resulting delta — never
  claimed mechanical. Three branches: single owning feature `delivered` → feature lane,
  delta card on the entry · single owning feature `in-flight` → not lane work; the
  finding files to the owning run, whose verification and regression gates own that
  territory · no single owner → product lane, keyed to the `.mochiko/product/` baselines
  and `ARCHITECTURE.md`, under the same test at the product surface — a baseline surface
  under active delta by an in-flight run files to that run instead. The product lane is
  **single-flight**: one live product-lane run at a time.
- **Lane boundary — the map-write test.** The lane is allowed only when the work needs
  **no new map entry and no status change** — a pure marked delta on an existing
  feature. Anything that would mint, promote, or flip routes to `/mochiko:specify`.
  Mid-run discovery that the work outgrew the lane **aborts and re-routes** — the lane
  never widens in place; the product lane the same — mid-fix discovery that the run
  stands on an in-flight feature's territory files the finding to that run and aborts.
  The boundary is graded from the map delta by the dispatched run's verification seat —
  no new seat here, never a self-declared "small".
- **Dispatch — reference, never restate.** Hand the delta card to `/mochiko:plan` /
  `/mochiko:implement`: the run gates on a feature entry carrying ratified scope — the
  scope source is a spec's accepted Feature Selection or a feature-command delta card.
  That gate, the bounds, the verification seats, and the evidence rules live in those
  commands and the craft skills they bind; this command points at them and adds nothing.
- **Independence:** where a producing seat exists — delta-card or `baseline-delta.md`
  authoring — no output is cleared by its author; any grading reads the files
  themselves, default FAIL. Plan approval: a seat that writes artifacts plans first and
  works only on a plan you approved.
- **Reserved to the user:** retire rulings · promotion on ambiguous cases ·
  lane-vs-specify routing when triage is genuinely borderline · parent selection
  semantics — unruled; surfaced when it bites, never defaulted here.
- Suggest commits; never run git mutations, never push. User rulings are plain blocking
  text, never a timed prompt.

## Bindings

- **Map machinery:** entry shape, parent/leaf nesting, delta grammar, integrity
  invariants, and the `unrefined` mark per `mochiko:authoring-feature-map` and
  `templates/feature-entry-template.md`, never restated. Entry files at
  `.mochiko/features/FEAT-XXX-<slug>.md`; per-feature run artifacts at
  `.mochiko/features/FEAT-XXX/`.
- **Product surface:** baselines at `.mochiko/product/` — `data-model.md` ·
  `contracts/` · `nfrs.md` · `constraints-and-decisions.md` · `quickstart.md` — with
  `ARCHITECTURE.md` at repo root. Product-lane runs at `.mochiko/product/lane-<slug>/`
  (card + reports + `baseline-delta.md`). Across repeat lane runs, cards and reports
  append (dated); delta files overwrite only via the graded fold.
- **Delta card:** one cycle-card-shaped unit per `templates/tasks-template.md`'s card
  shape. `baseline-delta.md` in appliable before/after form; a touch discovered mid-fix
  is authored by the dispatched run, not retro-authored here.
- **Scope types:** `delta scope` — landing is the feature-map delta fold · `selection
  scope` — landing is the graduation batch. The lane dispatches delta scope only;
  landings belong to the dispatched run.
- **Lane liveness:** every `in-flight` status or delta points at an open spec or a live
  lane run — live from dispatch until its acceptance landing; a delta whose lane run
  ended without folding is a defect, fix-on-sight (invariant home:
  `mochiko:authoring-feature-map`).
- **KM relation:** where `.mochiko/memory/knowledge-management.md` exists, `BACKLOG.md`
  is the defect queue — a reported bug is a BACKLOG item until a lane run picks it up —
  and lane acceptance is a landing event, same ritual home as spec and implement
  acceptance. Without KM: no queue — lane runs accept direct requests; that is the
  stated degrade path, never silently assumed.
- **Register:** user-facing prose per `templates/output-style.md`.
- **Next step:** `/mochiko:plan` for a dispatched delta scope (the pipeline scales
  itself); `/mochiko:specify` for anything the map-write test routes out.
