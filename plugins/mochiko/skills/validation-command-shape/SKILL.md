---
name: validation-command-shape
description: |
  Independently grade a mochiko command's conformance to the codified goal-shaped command
  shape — a deterministic grep floor (the five blocks present per their bindings, forbidden
  phase/flow/Contract headings absent, no ordinal steps inside Constraints, per-block
  ceilings, no seat row grading its own output, references present, no restated
  single-sourced prose, exceptions marked, frontmatter correct) beneath a prose judgment
  ceiling (altitude, parameter completeness over the P1–P16 slot set, Goal/Constraints
  soundness, preserved responsibilities), plus the strip-note audit during minimalism waves
  and the shape-revision audit (a revised command-shape.md graded for ruling fidelity,
  altitude, logging, and re-audit coverage) → binary PASS/FAIL + fix list. Use when grading
  an authored or converted commands/*.md file, auditing shape conformance, closing a strip
  wave, or auditing a shape-home revision.
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

Shape v5 is goal-shaped: the command **is** its contract, in five blocks. The floor below
grades that structure mechanically. The per-check disposition from v4 — what was kept,
dropped, or re-keyed at this revision, and why — is recorded in
`.mochiko/strips/validation-command-shape.md`; read it when auditing the revision itself.

## The deterministic floor (grep-checkable — run first, record results as evidence)

Against the command file:

1. **References present** — the file contains `loop-discipline` AND `agent-dispatch`;
   a team-form file (one containing `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS`) also contains
   `command-shape` — the obligated read of the shape home. The five KM-carrying commands
   (brainstorm · specify · plan · implement · setup) also contain the project-copy
   reference `.mochiko/memory/knowledge-management.md` — never the module template's path.
   A file whose validation model (P6) binds a **sized end-stage review** also contains
   `sized-end-stage-review` — the conditional read for that branch; a file declaring the
   in-loop-critique branch must **not** contain it (loading it there is the sham-read the
   split exists to prevent).
   **Re-keyed by ruling, disposition pending:** the `loop-discipline` member of this set is
   retained by shape v5's transition note, and whether it leaves is ruled at the pilot
   checkpoint (`command-succinctness-strip` D7 + D10 step 3). Until that ruling lands, a
   missing `loop-discipline` reference is a FAIL. Never treat its absence as anticipated.
2. **Frontmatter** — `disable-model-invocation: true` present; `description:` non-empty.
3. **Blocks present per their bindings** — the goal-shaped anatomy, graded on the binding
   and not the heading: `## Goal`, `## Seats & checks`, `## Constraints`, `## Bindings`,
   `## Recovery`. A block whose parameters are all vacuous for this workflow may be
   one-lined or omitted **only with the absence stated in the file** ("no gates", "no resume
   table"). A block that is simply missing — no heading, no stated absence — is a floor
   FAIL, as is a stated absence that contradicts the body (a "no gates" claim in a file
   whose Constraints rule gates).
4. **Forbidden headings** — none of `## Phase`, `## The flow`, `## Contract` appears. The
   contract is the document at v5; a Contract section means the appendix survived, and a
   Phase or flow heading means the procedure did.
5. **No ordinal steps inside Constraints** — within the Constraints block, no line matches
   `^\s*\d+\.\s` and no `step \d` cross-reference appears. Gates are ordered constraints;
   an ordinal list is the ordering narrative returning under a permitted heading.
6. **Per-block ceilings** (words, not lines — line counts are wrap artifacts and
   non-comparable across this surface, per the wave's own correction C2).

   **Count the four terms first, mechanically, and record them as the check's evidence** — the
   ceiling is worthless if the grader and the author count differently:
   - `G` = **gate lines in Constraints** = bullets matching `^- \*\*` that contain
     `evidence:` — the three-part form the shape mandates for P7. **`G` is not "numbered
     gates":** a workflow that numbers nothing still has gates (`brainstorm` carries **G = 4**
     — sizing, survivor rulings, tie-break, acceptance — while the checker map correctly
     records it as having 0 *numbered* gates). Counting numbers instead of gate lines is the
     miscount this clause exists to prevent.
   - `S` = data rows in the Seats & checks table (excluding header and separator).
   - `A` = artifacts P10 binds — the command's own outputs: deliverables + round reports.
     Input and reference paths cited elsewhere in Bindings do **not** count.
   - `R` = data rows in the Recovery table (0 where the block is one-lined).

   Ceilings: preamble ≤ 130 · Goal ≤ 150 · Seats & checks ≤ 100 + 45·S · Constraints ≤
   90·(G+2) · **Bindings ≤ 90 + 12·A, plus 30 where a KM-landing or index-bookkeeping binding
   is present** · Recovery ≤ 60 + 14·R. Over a ceiling is a floor FAIL — name the block, the
   count, the term values, and the bound.

   **Provisional:** calibrated on the v5 floor arithmetic
   (`.mochiko/strips/command-shape.md`), confirmed or revised at the pilot checkpoint. Measured
   headroom at the floor: tightest is `plan`'s Constraints (705 w against 810, 13%); the
   Bindings `+30` term exists because without it `brainstorm`'s measured Bindings floor sat at
   ~0.9% headroom — a ceiling that tight fails on formatting, not on bloat.
7. **No seat row grades its own output** — extract the Seats & checks rows mechanically. No
   row's produces/grades cell claims both authorship and grading of the same artifact, and
   no agent × skill pair appears as both the producer and the grader of one artifact. A row
   that grades must name a different agent *and* a different skill than the row that
   produces what it grades.
8. **No restated shape prose** — the shape home's signature lines do not appear in the
   command (they live in the home; a command references them). Grep for these markers; each
   hit is a restatement unless a marked exception sits on or adjacent to the line:
   - `forbidden transport` — and its older phrasing variant `the forbidden form`, which the
     home does not carry and does not need to: both name the one homed rule, the `name:`
     transport discriminator (home: `command-shape.md` **Layer 2 Seat transport**, relocated
     there at v5). A known phrasing variant of a homed rule is a hit; an unhomed *rule* is not.
   - `load` + `skills:` + `frontmatter` on one line (the teammates-don't-load-skills note —
     home: `command-shape.md` Layer 2)
   - `do not survive` (the recovery preamble — home: the anatomy's Recovery block)
   - a transcription of the four-message cross-exam sequence (`a→b` / `b→a` message-by-
     message lines — home: `review-brainstorm/references/CROSS-EXAM.md`; naming the
     protocol and its count is a reference, transcribing the sequence is restatement)
   - `reads as a malfunction` (the seat-announcement rule — home: `command-shape.md`
     Layer 2)
   - `input, never the gate` (home: Layer 2 Clearing)
   - `Out of rounds = escalate` (home: the anatomy's Constraints spec)
   - `disjoint agents, disjoint skills` / `structural separation` (home: Layer 2 Independence
     by structure, which states both phrasings — the table *shows* independence; prose
     asserting it is restatement)
   - `respawn is cold by design` (home: Layer 2 Independence by structure)

   Every marker above must be text one of the named homes actually carries — a marker whose
   home does not contain it is a broken check, not a strict one, and is fixed by re-keying the
   marker or by homing the content, never by leaving it to fire on prose that has no home to
   be relocated to.
9. **Exceptions marked** — every intentional restatement carries
   `<!-- shape-exception: ... -->` with a non-empty justification; an exception marker
   with no justification is a floor FAIL. At v5 every surviving marker is re-justified
   against the v5 home — a marker whose cited ground is a v4 section that no longer exists
   is a FAIL.
10. **Strip-wave and revision runs** — every strip-note entry touched this wave or
    revision carries a version stamp (a `[v` prefix per entry heading), and every re-add
    entry contains either an evidence link or the literal `override` marker.

## The judgment ceiling (prose — the grade grep cannot give)

11. **Altitude** — every line is unique-to-this-workflow content, a reference, or a marked
    exception. A sentence that would be true of every conformant command is mis-homed:
    name it and where it belongs.
12. **Parameter completeness** — every slot the shape home declares for the command's form
    is bound: **P1** goal line · **P2** probe seat (team-form) · **P3** end state · **P4**
    not-done states · **P5** seat rows · **P6** validation model · **P7** gate lines · **P8**
    bounds · **P9** invariants + survivors · **P10** artifact set · **P11** uncertainty
    carrier · **P12** fact route · **P13** verify-pass owner (sized review only) · **P14**
    clearing unit + checkpoint keying (devolved branch only) · **P15** pause location ·
    **P16** resume rows. An unbound parameter is a gap, not a style choice; a conditional
    slot that does not bind carries its stated absence (check 3's rule).
13. **Goal and Constraints sound** — the Goal's end state is measurable and its not-done
    states are real states of this workflow, not generic FAIL prose; every gate the body
    relies on appears as a Constraints line carrying all three of opening evidence · who
    rules · what it decides; every bound has the lead as its counter; and the Goal names no
    check that no Seats row produces.
14. **Preserved responsibilities (conversions, strips, and rewrites)** — nothing
    workflow-specific was dropped without a strip entry, and every relocation points at a
    home that actually contains the content (Read the home to confirm). At a goal-shape
    rewrite this additionally grades the **structural-prevention claims**: a supersession
    entry claiming a line is "structurally prevented by the new anatomy" is verified against
    the anatomy itself — does the failure that line prevented actually have no path in the
    goal-shaped file? — and is never taken on the author's say-so. Every line traceable to a
    `DECISIONS.md` row or carrying `KEPT:` Tier-2 evidence is either translated into the new
    blocks or superseded by a cited ruling.
15. **Strip-note quality (waves and revisions)** — entries name tier + disposition;
    shared-primitive entries name the consumers assessed; contested keeps carry
    survivor-provenance entries. A revision's note additionally carries its slot map: every
    prior-version parameter maps to a new home or is retired by an explicit supersession
    entry — retirement by omission is a FAIL.

## Shape-revision runs (grading a revised `command-shape.md`)

The graded file is the shape home itself. Read it, the ruling source the revision cites,
and the prior version's text (git) this run. Checks 1–14 do not apply, except check 10's
stamp form and check 15's entry-quality bar, which govern any strip-note entries the
revision writes.

16. **Floor — footer stamped:** the version line is bumped with date + ruling source, and
    the prior version history is preserved.
17. **Floor — rewrites logged:** every line rewritten or removed from the prior version
    carries a version-stamped strip-note entry (check 10's form); pure additions instead
    appear in the revision's decision row.
18. **Ceiling — ruling fidelity:** every cited ruling is encoded, and nothing beyond the
    rulings entered the home — diff against the prior version; each hunk traces to a
    ruling or is named as a gap. A ruling the author reports as un-encodable is graded on
    its reported reason, never waved through.
19. **Ceiling — altitude + re-audit set:** new doctrine is true of every conformant
    command (per-command variance is a `[PARAM]` tag), and the handoff names every
    conformant command the revision affects — an unnamed affected command is a gap. Where a
    ruling defers the re-audit to a named later ceremony, the note's re-audit-coverage line
    names that ceremony and the delta set it must cover.

## Verdict

```
VALIDATE: <graded file — command path, or the shape home in a revision run>
Checklist run:  validation-command-shape (floor 1–10, ceiling 11–15; revision runs 16–19)
Evidence read:  <files Read this run>     # graded file + shape home mandatory (revision runs: home + ruling source + prior version); absent ⇒ FAIL
Floor:          [per check — PASS/FAIL + the grep evidence]
Ceiling:        [per check — PASS/FAIL + one-line evidence]
VERDICT: PASS | FAIL
Issues requiring fix: <item → missing thing → concrete fix>
```

Never edit the file graded; never grade an artifact this context authored.
