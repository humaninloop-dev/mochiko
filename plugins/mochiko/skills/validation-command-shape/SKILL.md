---
name: validation-command-shape
description: |
  Independently grade a mochiko command's conformance to the codified command shape — a
  deterministic grep floor (references present, no restated single-sourced prose,
  exceptions marked, frontmatter correct) beneath a prose judgment ceiling (altitude,
  parameter completeness, contract-fill soundness), plus the strip-note audit during
  minimalism waves and the shape-revision audit (a revised command-shape.md graded for
  ruling fidelity, altitude, logging, and re-audit coverage) → binary PASS/FAIL + fix
  list. Use when grading an authored or converted commands/*.md file, auditing shape
  conformance, closing a strip wave, or auditing a shape-home revision.
  MUST BE USED when the task says "grade this command", "audit shape conformance",
  "audit this strip wave", or "audit this shape revision". Run by an independent grader,
  never the author.
---

# Validation: Command Shape

## Overview

Binary PASS/FAIL over a `commands/*.md` file (and, in a strip wave, its cluster's strip
notes; in a shape-revision run, the shape home itself — checks 11–14) against the shape
whose sole authoritative home is
`${CLAUDE_PLUGIN_ROOT}/templates/command-shape.md`. **Read the graded file and the shape
home this run** (a revision run: the shape home, the ruling source, and the prior
version's text via git) — grading from a summary or the author's report is a FAIL by
itself.
Default FAIL; the verdict clears only check by check.

The two-layer design is deliberate: the **deterministic floor** runs first and its results
are recorded as the evidence (it is grep — it cannot be rationalized past); the **judgment
ceiling** does the work grep cannot. A floor failure is a FAIL regardless of how good the
prose reads. (The residual risk that the judgment layer
rationalizes is recorded as accepted — the floor is the backstop.)

## The deterministic floor (grep-checkable — run first, record results as evidence)

Against the command file:

1. **References present** — the file contains `loop-discipline` AND `agent-dispatch`;
   a team-form file (one containing `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS`) also contains
   `command-shape` — the obligated read of the shape home. The five KM-carrying commands
   (brainstorm · specify · plan · implement · setup) also contain the project-copy
   reference `.mochiko/memory/knowledge-management.md` — never the module template's path.
2. **Frontmatter** — `disable-model-invocation: true` present; `description:` non-empty.
3. **No restated shape prose** — the shape home's signature lines do not appear in the
   command (they live in the home; a command references them). Grep for these markers;
   each hit is a restatement unless a marked exception sits on or adjacent to the line:
   - `the forbidden form` / `forbidden transport` (the `name:` transport discriminator —
     home: `agent-dispatch.md` Seat transport)
   - `load` + `skills:` + `frontmatter` on one line (the teammates-don't-load-skills note —
     home: `command-shape.md` Layer 2)
   - `do not survive` (the recovery preamble — home: `command-shape.md` Layer 1)
   - a transcription of the four-message cross-exam sequence (`a→b` / `b→a` message-by-
     message lines — home: `review-brainstorm/references/CROSS-EXAM.md`; naming the
     protocol and its count is a reference, transcribing the sequence is restatement)
   - `reads as a malfunction` (the seat-announcement rule — home: `command-shape.md`
     Layer 2)
4. **Exceptions marked** — every intentional restatement carries
   `<!-- shape-exception: ... -->` with a non-empty justification; an exception marker
   with no justification is a floor FAIL.
5. **Strip-wave and revision runs** — every strip-note entry touched this wave or
   revision carries a version stamp (a `[v` prefix per entry heading), and every re-add
   entry contains either an evidence link or the literal `override` marker.

## The judgment ceiling (prose — the grade grep cannot give)

6. **Altitude** — every line is unique-to-this-workflow content, a reference, or a marked
   exception. A sentence that would be true of every conformant command is mis-homed:
   name it and where it belongs.
7. **Parameter completeness** — every `[PARAM]` the shape home declares for the command's
   form is actually bound: deliverable + ID scheme, seat roster (team-form), sizing-gate
   keying + verify-pass owner (where a sized review exists), fact substrate, bounds with a
   counter-owner, named human gates, recovery mapping. An unbound parameter is a gap, not
   a style choice.
8. **Contract fill sound** — the Contract section's four clauses are concrete and true of
   the body: the done-condition's not-done states are real states of this workflow; the
   named producer/validator agents + skills are disjoint; every bound has an owner; every
   gate the body names appears in the clause.
9. **Preserved responsibilities (conversions and strips)** — nothing workflow-specific was
   dropped without a strip entry, and every relocation points at a home that actually
   contains the content (Read the home to confirm).
10. **Strip-note quality (waves and revisions)** — entries name tier + disposition; shared-primitive
    entries name the consumers assessed; contested keeps carry survivor-provenance
    entries.

## Shape-revision runs (grading a revised `command-shape.md`)

The graded file is the shape home itself. Read it, the ruling source the revision cites,
and the prior version's text (git) this run. Checks 1–10 do not apply, except check 5's
stamp form and check 10's entry-quality bar, which govern any strip-note entries the
revision writes.

11. **Floor — footer stamped:** the version line is bumped with date + ruling source, and
    the prior version history is preserved.
12. **Floor — rewrites logged:** every line rewritten or removed from the prior version
    carries a version-stamped strip-note entry (check 5's form); pure additions instead
    appear in the revision's decision row.
13. **Ceiling — ruling fidelity:** every cited ruling is encoded, and nothing beyond the
    rulings entered the home — diff against the prior version; each hunk traces to a
    ruling or is named as a gap.
14. **Ceiling — altitude + re-audit set:** new doctrine is true of every conformant
    command (per-command variance is a `[PARAM]` tag), and the handoff names every
    conformant command the revision affects — an unnamed affected command is a gap.

## Verdict

```
VALIDATE: <graded file — command path, or the shape home in a revision run>
Checklist run:  validation-command-shape (floor 1–5, ceiling 6–10; revision runs 11–14)
Evidence read:  <files Read this run>     # graded file + shape home mandatory (revision runs: home + ruling source + prior version); absent ⇒ FAIL
Floor:          [per check — PASS/FAIL + the grep evidence]
Ceiling:        [per check — PASS/FAIL + one-line evidence]
VERDICT: PASS | FAIL
Issues requiring fix: <item → missing thing → concrete fix>
```

Never edit the file graded; never grade an artifact this context authored.
