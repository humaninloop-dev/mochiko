# Blind second list — where feature's ruled set lives

Seat: one fresh persona-less `general-purpose` seat, spawned `model: opus`, given the session's
frame as the opening and the question only — never the lead's options, the conversation or the
ruled set. Returned 2026-10-15 in its message; transcribed here by the lead, unedited in substance.
Seat disclosure: its haiku `Explore` locate dispatch returned no report, so it ran the locate greps
itself on the seat tier; no files written outside scratch.

The question as dispatched: when a `/mochiko:feature` visit cuts a card (a bug, an improvement, or
a growth row) and the user first rules what should change — in their own words, plus one "I'd know
it works if…" sentence or `open` — where should that ruled set be saved, so that (1) the seat that
then writes the card stays inside it, (2) the `/mochiko:implement` run the card is dispatched to can
read it later, possibly days later, and (3) the feature map stays free of orphans?

## Ground facts (seat's)

- Where the card itself lives is not settled yet. `feat.stable-ground-triage` says "card on the
  entry", but the feature-entry template has no card section. The feature home declares
  `tasks.md`. The lane home `.mochiko/product/<slug>/` declares no `tasks.md` (`mochiko-cli home
  .mochiko/product/lane-x`). Every road has to name the card's home or cite it.
- Engine slot: `analysis-iterative.caller-slots` ("where the ruled set is saved") ·
  `analysis-iterative.contract-pen` (the lead writes the set down before the next writer opens it)
  · `analysis-iterative.contract-inside` (floor).
- Precedent: `arch.feature-delta-visit` closes in a `visit.md` keyed by its slug (the joined FEAT
  id, lower-cased), with no link on the entry. `impl.design-inputs` reads only
  `.mochiko/architecture/desk/`. The design-stage template header already has a "Handed in: a desk
  visit file's path" line.
- Map: `authoring-feature-map.integrity-fix-on-sight` (every row names its run · a live row points
  at an open run) · `authoring-feature-map.pre-acceptance-workspace` (unratified thought stays off
  the live map).
- `impl.design-stage-inputs` is read only when a design stage opens. A delta run with zero gaps
  never opens one.

## Roads

- **R1 (cheapest)** — write the set into the desk sitting's existing `derivation.md`
  (`.mochiko/features/desk/<date-slug>/`, no template, 300 lines). Touch: a new feat
  thinking-stage rule (fills the slots, contract on); `impl.design-inputs` widened to
  `.mochiko/features/desk/`; no home migration. Read by: the card seat when the pen passes · the
  card review leg (`feat.sound-loop-floor`) · implement's design stage as a handed-in file.
  Weakness: no shape, so nothing checks direction, seed and provenance; "derivation" is specify's
  word (`templates/artifact-format.md:8`); keyed by slug convention only.
- **R2** — a typed feature-desk visit file in the same home. New template, sibling of
  `architecture-visit`: goal and done condition · keyed to a FEAT or a `lane-<slug>` · ruled set
  per card (direction, seed or `open`, provenance) · which card was cut, with a pointer · verdict.
  The directory slug carries the key; the card carries one pointer line back. Touch: a migration
  adding the deliverable and template to the feature-desk home · `feat.artifact-home` · a new feat
  thinking-stage rule · `feat.delta-cards` (the review leg grades the card against the file for
  same meaning) · `impl.design-inputs` and `impl.design-stage-inputs`. Read by: the same seats as
  R1, plus the implement landing table. Weakness: the heaviest viable road; one key per sitting
  breaks when a visit cuts cards on two capabilities.
- **R3** — the desk pre-seeds `design-stage.md` in `.mochiko/features/<FEAT-ID>/`. Touch:
  design-stage template header · `impl.design-stage-file` · `feat.artifact-home` (a third desk
  home). Read by: implement's design stage, which resumes from the file. Weakness: mixes desk
  intake with the gap-keyed design agenda (the gap list does not exist yet at desk time); one file
  per home, so the next run overwrites it; a zero-gap delta run never reads it.
- **R4** — a new deliverable in the feature home (`card-stage.md`), one dated entry appended per
  card id. Touch: feature-home migration (plus the lane home for cards with no owning capability) ·
  `feat.artifact-home` · a thinking-stage rule · implement's input rules. Read by: the card seat ·
  the review leg · implement in any scope. Weakness: puts pre-dispatch thought under a FEAT, so an
  undispatched card leaves orphan-shaped residue; the 300-line whole-file bound fills up on a
  long-lived capability.
- **R5** — put the set on the card itself. The lead writes Direction, "I'd know it works if" and
  provenance as card fields before the pen passes (`###` content is free under the tasks
  conformance check). Touch: tasks template card fields · `feat.delta-cards`. Read by: implement
  directly (the card is its delta entry, `commands/implement.md:60`). Weakness: growth rows enter
  selection scope, where implement's card-authoring rewrites `tasks.md` and the set is
  overwritten; the writer shares a file with the user's words; lane cards have no `tasks.md`.
- **R6** — on the entry's work-row or Obligations line. Weakness: the entry is capability truth at
  one line each, with no room for provenance; it puts pre-dispatch thought on the live map.

## Rejected by the seat

- **R7**, conversation plus brief only: fails need (2) and breaks contract-pen.
- **R8**, the run folder `.mochiko/runs/`: ephemeral, deleted at acceptance, and the run id does not
  exist yet at desk time.
- **R9**, `BACKLOG.md`: depends on the KM module (`feat.km-relation`), is an operating doc rather
  than a product record, and does not cover growth rows.
- **R10**, a typed report in the sitting's `reports/`: the report envelope's ultra register mangles
  the user's own words.

## Seat's pick and the case against it

Pick **R2**. It keeps unratified thought off the map, matching pre-acceptance-workspace and the
no-entry-link rule in `arch.feature-delta-visit`, so it leaves no orphan. The typed shape gives the
card review leg something to grade against. Implement already reads "a desk visit file keyed to the
feature" and has the "Handed in" line, so the read path needs one widened clause. The card's
pointer back turns the slug convention into a checkable link (a dead pointer is a KM defect).

Case against R2: it is the heaviest viable road (migration, template, edits to two commands). Slug
keying is a convention, so a two-capability visit or a lane card needs a per-card or `lane-<slug>`
key. A zero-gap delta run never opens a design stage, so the sufficiency check or the card-vs-entry
check must be told to read the file, which is one more rule touch. And R5 gives bug and
improvement cards the same thing far cheaper; it fails only on growth rows.
