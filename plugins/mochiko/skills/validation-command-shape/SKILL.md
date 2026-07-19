---
name: validation-command-shape
description: |
  Independently grade a mochiko command's conformance to the codified command shape — a
  deterministic grep floor (references present, no restated single-sourced prose,
  exceptions marked, frontmatter correct) beneath a prose judgment ceiling (altitude,
  parameter completeness, contract-fill soundness), plus the strip-note audit during
  minimalism waves → binary PASS/FAIL + fix list. Use when grading an authored or
  converted commands/*.md file, auditing shape conformance, or closing a strip wave.
  MUST BE USED when the task says "grade this command", "audit shape conformance", or
  "audit this strip wave". Run by an independent grader, never the author.
---

# Validation: Command Shape

## Overview

Binary PASS/FAIL over a `commands/*.md` file (and, in a strip wave, its cluster's strip
notes) against the shape whose sole authoritative home is
`${CLAUDE_PLUGIN_ROOT}/templates/command-shape.md`. **Read the command file and the shape
home this run** — grading from a summary or the author's report is a FAIL by itself.
Default FAIL; the verdict clears only check by check.

The two-layer design is deliberate: the **deterministic floor** runs first and its results
are recorded as the evidence (it is grep — it cannot be rationalized past); the **judgment
ceiling** does the work grep cannot. A floor failure is a FAIL regardless of how good the
prose reads. (The floor revives the retired `verify-output` altitude gate's floor+ceiling
design; the residual risk that the judgment layer rationalizes is recorded as accepted —
the floor is the backstop.)

## The deterministic floor (grep-checkable — run first, record results as evidence)

Against the command file:

1. **References present** — the file contains `loop-discipline` AND `agent-dispatch`;
   a team-form file (one containing `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS`) also contains
   `command-shape` — the obligated read of the shape home.
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
5. **Strip-wave runs only** — every strip-note entry touched this wave carries a version
   stamp (a `[v` prefix per entry heading), and every re-add entry contains either an
   evidence link or the literal `override` marker.

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
10. **Strip-note quality (waves)** — entries name tier + disposition; shared-primitive
    entries name the consumers assessed; contested keeps carry survivor-provenance
    entries.

## Verdict

```
VALIDATE: <command path>
Checklist run:  validation-command-shape (floor 1–5, ceiling 6–10)
Evidence read:  <files Read this run>     # command + shape home mandatory; absent ⇒ FAIL
Floor:          [per check — PASS/FAIL + the grep evidence]
Ceiling:        [per check — PASS/FAIL + one-line evidence]
VERDICT: PASS | FAIL
Issues requiring fix: <item → missing thing → concrete fix>
```

Never edit the file graded; never grade a command this context authored.
