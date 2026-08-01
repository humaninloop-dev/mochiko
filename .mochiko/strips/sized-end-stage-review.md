# Strip notes — `templates/sized-end-stage-review.md`

Entry formats: `strips/README.md`. This primitive was **created** at v0.33.0 by a split, not
authored from scratch — its whole body arrived from `command-shape.md` Layer 1 v5. The
departure entry (with the full content record, the tier, and the named skip path) is
`.mochiko/strips/command-shape.md` **[v0.33.0] Sized end-stage review relocated**; this note is
the arrival side, so the relocation is logged at both ends per the transport precedent set the
same version.

---

**Wave context (v0.44.0 — the D7 leakage scrub).** `verbosity-caveman-ops-separation` D7 as
folded at review (S4): **full scrub** of ops leakage from the shipped tree, with no
changelog-worthy detail lost — every removed block is preserved verbatim below. Ruling:
`DECISIONS.md` 2026-08-01 "Output verbosity, caveman & ops separation ruled" row.

**The leak test this wave used, recorded so a future sweep inherits it: *whose artifact does the
pointer name?*** Mochiko's own ops records — `.mochiko/strips/`, `.mochiko/brainstorms/`,
`.mochiko/decisions/`, `.mochiko/archive/` — are leaks: they resolve to nothing in an installed
plugin. Adopter runtime paths (`.mochiko/specs/`, `.mochiko/memory/`) and the KM module's
document contracts are the **user's** artifacts and are untouchable. A prefix-based sweep on
`.mochiko/` would gut the KM module and the brainstorm command; 101 of this tree's 146
`.mochiko/` references were correctly left alone on that test.

## [v0.44.0]  version-history block relocated (class 2, 810 B / 8 lines)
- **Disposition:** superseded → relocated **verbatim** into this note (below). In-file residue: the
  bare stamp plus the live routing (`Governed by / Pairs with`), which is wiring a run consumes, not history.
- **Tier failed:** n/a — supersession by ruling (`verbosity-caveman-ops-separation` D7 + S4; the `DECISIONS.md` 2026-08-01 row above)
- **Content (verbatim, the whole block as it stood at the scrub):**
```
**Version:** v2 (2026-08-01 — `lead-owned-process-flexibility` D6(c) ratified at A4: this
pattern **survives as the stated default** wherever P6 binds it · **U4** — the sizing gate
passes user → lead by recorded supersession · **U1-B** — the verify pass hardens to floor
status and `none` on a lead-penned artifact requires a recorded user waiver · interim note for
unconverted commands added, then **retired at the v0.43.0 wave close** — both its deferrals spent
once the last command converted, the trail home it anticipated now bound at P20; v1 2026-07-30 — command-succinctness-strip, pilot-checkpoint ruling
B: split out of `command-shape.md` Layer 1 v5 as a conditional read; text unchanged in
substance) · **Governed by:** `mochiko:loop-discipline` · **Pairs with:** `command-shape.md`
```
- **Kept deliberately:** the version *number* and its date stay in the file — a consumer still
  learns which revision it is reading; only the per-revision narrative left.

## [v0.44.0] Provenance pointer
- **Disposition:** superseded → deleted; this note is the home it pointed at.
- **Tier failed:** n/a — supersession by ruling (`verbosity-caveman-ops-separation` D7 + S4; the `DECISIONS.md` 2026-08-01 row above)
- **Content (verbatim):**
```
(Provenance:
`.mochiko/strips/sized-end-stage-review.md`.)
```
- **Kept deliberately:** the slot-ownership sentence before it and the stated-default/recorded-
  departure rule after it — both operative.

# v0.43.0 — the v2 interim note retired, both its deferrals spent

**Wave context:** the v7 conversion wave closed with all six commands converted (wave note
`.mochiko/strips/command-shape.md` [v0.43.0 wave close]). Raised by the batch audit as **K1**: the
note's operative clauses describe unconverted commands, and no unconverted command exists. Body
**5,123 → 4,256 B** (words 771 → 630, −18.3%). Still a **conditional** read, so the saving is paid
back only to the two commands whose P6 binds it — it does not reach the shared always-read floor.

## [v0.43.0] The v2 interim note retired whole — a note whose population is now zero
- **Disposition:** superseded → deleted. Both of its clauses were addressed to "a command not yet
  converted to shape v7"; that set is empty.
- **Tier failed:** n/a — supersession by ruling (the shape's marker-retirement trigger, met when
  the last command converted; `DECISIONS.md` 2026-08-01 wave-close ratifications row. Raised as
  **K1** at the batch audit).
- **Content (v2, verbatim):**
  ```
  > **Interim note (v2).** Sizing ownership moved user → lead by recorded supersession (U4 —
  > `.mochiko/strips/sized-end-stage-review.md`). A command not yet converted to shape v7 still
  > says the user rules on sizing; **that stands, as written, in those commands** until their
  > conversion touch (`command-shape.md`, the v7 interim note). Where the two disagree, **the
  > unconverted command wins for its own run** — it is the file that binds. What does not change
  > either way is the protection: the floor's second invariant gives a lead-penned artifact its
  > cold grade whoever named the size. **The trail line above has a home before P20 exists:** in
  > an unconverted command a size below the default is recorded where that command already records
  > review outcomes — the artifact's **Review section**, beside the waiver record the `none` path
  > writes there. P20 names that home explicitly at the conversion touch; until then the
  > deliverable is it, by the same logic that lets the unconverted command win.
  ```
- **Both deferrals spent, each verified against the files rather than assumed:**
  1. **The sizing conflict.** The note existed because U4 moved sizing user → lead while the two
     binding commands still said the user rules. **Both binders flipped at their conversion touch
     this wave** — `brainstorm` and `setup`, each logged at [v0.43.0] in its own strip note. There
     is no disagreement left for "the unconverted command wins" to arbitrate.
  2. **The trail-line home.** The note supplied an interim home (the artifact's Review section)
     *"before P20 exists"*. **P20 now exists and is bound in both binders** — verified by grep:
     both carry the `**Departure trail:**` binding. The clause's own stated condition retires it.
- **Kept deliberately:** the **whole body above the note**, untouched — the lead-owned sizing gate
  with its pair/single/none outcomes and waiver record, the cold-and-mutually-withheld reviewers,
  the one-shot cross-exam reference, the per-reviewer tally with the lead-owned merge, survivor
  routing by answer-owner, and **the verify pass as floor** (U1-B). Two protections the note
  restated survive at their real homes and were **not** dropped with it: the floor's second
  invariant giving a lead-penned artifact its cold grade *whoever named the size* (home:
  `command-shape.md` Layer 1, *The floor*, invariant 2 — the note was echoing it, not owning it),
  and the below-default trail line, which the header's stated-default sentence still requires and
  P20 now homes. The version footer records the retirement rather than dropping the note's history.
- **Consumers assessed** — the two commands whose P6 binds this file, both flipped this wave:
  - `commands/brainstorm.md` — **converted at [v0.43.0]**, sizing flipped to the lead per U4, P20
    bound. Its strip entry cites *this note's deferral* as the ground for the flip.
  - `commands/setup.md` — **converted at [v0.43.0]**, same flip, P20 bound, same citation.
  - **Those citations stay valid as history and must not be read as dangling.** Each entry records
    why its command flipped *at the moment it flipped*, when the note was live and was the
    authority. A strip entry is a record of a landing, not a live pointer; retiring the note it
    cites does not falsify it, and the two entries were deliberately left unedited (this
    directory's entries are frozen once stamped).
  - No third consumer: no other command references `sized-end-stage-review` (grep across all six —
    the four in-loop-critique commands must *not* contain it, per `validation-command-shape`
    check 1's negative direction, and do not).

---

# v0.40.0 — v1 → v2, the sizing gate changes hands and the verify pass becomes floor

**Wave context:** `lead-owned-process-flexibility`
(`.mochiko/brainstorms/lead-owned-process-flexibility/record.md`), **D6(c)** ratified with the
set at acceptance **A4**. Wave note: `.mochiko/strips/command-shape.md` [v0.40.0]. Body
**2,992 → 5,123 B** (words 423 → 771), measured after the repair round. This file is a
**conditional** read, so the growth is
paid only by the two commands whose P6 binds it (`brainstorm`, `setup`), never on the shared
floor.

**The headline is what did *not* happen.** D6(c)'s earlier form — "demotes to optional
playbook" — was **withdrawn** at the folds, because it created R3's self-grading hole and R7's
silent supersession. The pattern **survives as the stated default** wherever P6 binds it, and
departing from it is a recorded departure like any other. Two ruled changes only, both below.

**Additions this revision** — recorded for the decision row, not as strips: the header's
stated-default sentence · the interim note for unconverted commands, including the clause naming
where a below-default sizing's trail line lands **before P20 exists** (the artifact's Review
section, beside the waiver record the `none` path already writes there — added at the repair
round, the audit having observed that the sizing bullet obliges a trail line the two binding
v6-form commands have no slot for).

## [v0.40.0] Sizing-gate ownership: the user's → the lead's — supersession by ruling
- **Disposition:** superseded → rewritten in place, same bullet, same position. The gate itself,
  its inputs, its three outcomes and its waiver record all survive; only who rules it moves.
- **Tier failed:** n/a — supersession by ruling (**U4**, user card 2026-08-01: *"review sizing
  passes to the lead **by recorded supersession** of the brainstorm-v2-2 ruling; the supersession
  is logged at the landing under the primitive-edit ceremony"*; raised as **R7** — *"a user-owned
  gate moves to lead discretion with no recorded supersession"* — and this entry is the discharge
  of that finding. `DECISIONS.md` 2026-08-01 row.)
- **What is being superseded, named at its source:** the sizing gate was **born** user-owned at
  `brainstorm-v2-2-revision` (index `:149` — the token-efficiency revision that measured ≈654k
  out on a review pair triple-reading an already-mapped reality surface, and answered it with
  "review sizing becomes a named human gate (pair / single / none with waiver)"). That ruling is
  what U4 supersedes; it is not being quietly outgrown.
- **Content (v1, verbatim):** "**The sizing gate is the user's.** At convergence the lead states
  the artifact's weight (element count, confidence-mark mix, reality-surface load) and the
  estimated review cost, recommends **pair / single / none** against the declared default (P7
  carries that keying), and the user rules."
- **Kept deliberately:** the lead's weight statement and its three inputs · the cost estimate ·
  the three sizes and the declared-default keying at P7 · the waiver record in the artifact's
  Review section, with who waived, at which gate, why · "the validator seat passes to the user
  alone, deliberately and auditably", verbatim.
- **What replaces the user's ruling, so the transfer is not a straight loss of a human stop:**
  two floor stops, neither the lead's to waive. The **run-start weight card** is user-ruled
  (U1-A), so the lead sizes *under* it rather than around it; and the estimated cost is now a
  **declared bound** (A3), counted, rising only at a user checkpoint. A size below the default is
  a departure and takes one trail line. On a **lead-penned** artifact `none` is not the lead's to
  take at all — floor invariant 2 (U1-B) gives that artifact one cold-seat grade unless the user
  waived it on the record.
- **Consumers assessed:** the two binding commands. `brainstorm.md:48` and `setup.md:76–82` both
  carry `rules: the user` on their sizing gate — **unedited this wave and correct as written**
  for a v6-form file (D4, convert-on-touch), which is why v2 carries an interim note saying so
  in the template rather than letting the two surfaces silently contradict each other. The
  template's `none` path was the only one that could have diverged dangerously, and it cannot:
  `brainstorm.md:83–84`'s "derived, unchecked" stamp and the floor's invariant 2 both survive.
  `review-brainstorm` / `review-governance-intent` own the reviewer-side protocol and take no
  position on who sizes — unaffected, verified. `CROSS-EXAM.md` untouched.

## [v0.40.0] The verify pass hardened from a bound to a floor obligation
- **Disposition:** superseded → the verify-pass bullet gains a floor clause; the sentence's
  mechanics are unchanged.
- **Tier failed:** n/a — supersession by ruling (**U1-B**, closing **R3**: the lead's folds and
  any lead-penned record get one cold-seat grade **non-discretionarily** wherever a review ran;
  zero-cold-read shipping only by recorded user waiver at the weight card).
- **Content (v1, the status that changed — the mechanics stand):** the verify pass sat inside
  "**Review + verify is the bound**", i.e. as one of the review's caps, and was therefore
  reachable by the same lead judgment that sizes the review. R3's finding: that makes the check
  on the lead's own folds lead-discretionary, and D2.2's conditional binding vacuously
  satisfiable — meta-self-grading.
- **Kept deliberately:** P13's owner, the solo-reviewer rule (a solo reviewer verifies the lead's
  folds, grading the repairs and never its own findings), the quote-the-evidence obligation, and
  **"Review + verify is the bound"** with its escalation clause — the pass is now floor *and*
  still bounded, which is the point: one pass, non-discretionary.
- **Consumers assessed, and one residual named rather than papered over:** `brainstorm.md` and
  `setup.md` both bind a verify-pass owner (P13) and neither states the pass as optional — no
  interim contradiction. **`review-brainstorm/SKILL.md:58` and
  `review-governance-intent/SKILL.md:108` each phrase the trigger as "*when the lead assigns
  it*"** — reviewer-side craft
  describing when the reviewer acts, and it authorizes nothing on the lead's side; but under
  U1-B it no longer states the whole truth, because where a review ran the assignment is not the
  lead's to withhold. **Left unedited deliberately:** D6(c) names exactly two changes to this
  branch (U4's ownership transfer, U1-B's hardening), both to *this* template; rewriting two
  reviewer skills is neither ruled here nor reachable by inference from the record. Raised to
  the lead as a residual for the next touch of those two skills, and logged here so a later
  auditor finds the seam named rather than missed.

## [v0.33.0] Created by split from `command-shape.md` Layer 1 (conditional read)
- **Disposition:** relocated **in** ← `templates/command-shape.md` Layer 1 (shape v5).
- **Tier failed (on the departure side):** 1 (altitude — wrong load class, not wrong content):
  correct shape doctrine paid on every run of every command while binding in only two of six.
- **Ruling source:** `.mochiko/brainstorms/command-succinctness-strip/record.md` — raised by the
  shape-v5 floor arithmetic as an unruled remedy, declined at authoring time per Job 4 step 1,
  then **adopted at the pilot checkpoint (ruling B, user, 2026-07-30)**, craft left to the
  author. Landed inside the same v0.33.0 revision — one shape version (v5), one bump.
- **Content:** unchanged in substance from v5's Layer 1 block — the user-ruled sizing gate and
  its waiver, cold mutually-withheld reviewers, the frozen artifact, the one-shot cross-exam
  pointer, per-reviewer survivors and tallies with the merge reserved to the lead, survivor
  routing by answer-owner, one disposition per survivor, the verify pass, review+verify as the
  bound. Editorial only: a two-line standalone header (what this is, when it is read) and a
  version footer, which the block did not need while it sat inside another file.
- **The skip path** (D4's condition for reduction credit): the command's **P6 validation model**
  decides the load. Loaded by `brainstorm` and `setup`; never loaded by `specify`, `slice`,
  `plan`, `implement`, which declare the bounded in-loop-critique branch. Enforced in both
  directions by `validation-command-shape` check 1.
- **Declares no parameters.** The four slots cited in the text (P5 · P7 · P12 · P13) are declared
  in `command-shape.md`'s anatomy; nothing about the P1–P16 set or the v4→v5 slot map changed
  with this move.
- **Consumers assessed:** `brainstorm` and `setup` gain the conditional read when re-authored
  (pilot + wave); they are v4 files this step and still receive the text inline via the shape, so
  the interim is correct, not broken. `authoring-commands` points at the shape for sizing-gate
  parameters, not at this block — unaffected. `review-brainstorm` /
  `review-governance-intent` own the reviewer-side protocol — unaffected;
  `review-brainstorm/references/CROSS-EXAM.md` stays the cross-exam's home and is now cited from
  here. `loop-discipline` untouched (no shared-skill edit this step).
