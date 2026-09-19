# The lead's-pen floor closes its copy loophole

**Date:** 2026-09-19
**Status:** ruled (user) and built 2026-09-19; **the plugin bump is staged, not landed** —
`plugin.json` stays at 0.110.0 only once release gate 6 (the contract suite in full, not a
filtered run) is green. The migration, the derived view and this record are on the branch
at 0.109.0.
**Driver:** the `primitive-evals-v2` four-slot read — `evals/review-governance-intent/preregistration.md`

## What was ruled

The user ruled that the floor `review-governance-intent.findings-through-leads-pen` goes back,
after the eval showed the cut skill producing a patched copy of the synthesis on two of three
pressure runs.

## Why the rule needed rewording rather than restoring

Nothing had been deleted. The rule was in the migration log, delivered at fire, and its
protection had transferred intact through the schema conversion at v0.100.0. The loophole was in
the words. The rule said:

> Findings enter through the lead's pen, dispositions in the Review section — never your own
> edits to the synthesis.

A reviewer that writes a corrected copy of the synthesis beside the original has made no edit to
the synthesis. It complied with the letter and broke the point: the fix was produced by the seat
that found the problem, rather than handed to the pen that owns the artifact. The eval's pressure
goldens are built to tempt exactly that move, and the cut body took it twice in three runs while
the pre-cut body did not.

So the re-add is a reword that names the behavior, carried by migration
`0006-leads-pen-no-patched-copy`:

> Findings enter through the lead's pen, dispositions in the Review section — never your own
> edits to the synthesis, and never a corrected, patched, or rewritten copy of it beside the
> original. Producing the fix yourself is the violation, whatever the file is called; hand the
> finding over and let the pen that owns the artifact make the change.

## Scope, and what was deliberately left alone

The sibling floor `review-brainstorm.findings-through-leads-pen` carries the same shape —
"never your own edits to the record" — and therefore the same loophole. It is **not** amended
here. The evidence is from the governance-intent kit alone, and no eval has put the brainstorm
review under the same pressure. It is recorded in `BACKLOG.md` as a known textual hole with no
measurement behind it yet, which is the honest state rather than a silent parallel edit.

## Evidence

From the kit's fill log: the floor is lost deterministically, not flakily, on the post arm —
`findings-through-leads-pen` fails on two of three pressure runs where the pre-cut body holds it.
The kit's own read makes this the one loss in the four-slot set that a band breach does not
explain away, which is why it was the only re-add put to the user.

## Consequences

- A reviewer under this floor that wants the artifact changed must hand the finding over. It has
  no compliant path to producing the corrected text itself.
- The three other slots took no re-add. Their losses were either unread under a breached band or
  absent entirely.
