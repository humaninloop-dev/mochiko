# Strip notes — `templates/governance-intent-template.md`

Entry formats: `strips/README.md`. First entry created at **v0.43.0** — this primitive had no strip
note before the shape-v7 conversion wave, so there is no prior-wave history here and no protected
set recorded in this file; the protections that govern its content are the `DECISIONS.md` rows
traced in the entry below and the *Kept deliberately* fields of `.mochiko/strips/setup.md`, its only
command consumer.

**No version footer.** Unlike `command-shape.md` and `sized-end-stage-review.md`, this template
carries no version line (checked at this edit), so no footer stamp was owed and none was invented.

---

# v0.43.0 — the v6→v7 conversion wave

**Wave context:** the shape-v7 conversion wave (`lead-owned-process-flexibility`,
`.mochiko/brainstorms/lead-owned-process-flexibility/record.md`; `DECISIONS.md` 2026-08-01), all six
commands converting after the user widened the wave on 2026-08-01. This template is not a command
and did not convert; it is edited here because **U4**'s sizing-ownership flip, executed in
`commands/setup.md` at its conversion touch, left this scaffold printing the superseded ruler.
Surfaced by the setup conversion and routed to a scope extension by the wave lead.

## [v0.43.0] The Review-section sizing scaffold flips user → lead (U4), and its waiver moves to the weight card (U1-B)

- **Disposition:** superseded → rewritten in place. The Review section, its recovery keying, and
  every fill slot survive; the *ruler* named by the sizing sub-line and the *gate* named by the
  waiver sub-line changed.
- **Tier failed:** n/a — supersession by ruling (**U4**, 2026-08-01: *"review sizing passes to the
  lead by recorded supersession of the brainstorm-v2-2 ruling"*, ratified with the set at **A4**;
  and **U1-B**: *"a lead-penned deliverable ships with zero cold reads only by recorded user waiver
  at the weight card"*). Doctrine home already flipped:
  `templates/sized-end-stage-review.md` **v2** — *"The sizing gate is the lead's"* and *"On a
  **lead-penned** artifact `none` is not the lead's to take"*. This edit brings the scaffold into
  line with the home it serves.
- **Content (verbatim, the two bullets that were rewritten):**
  ```
  - **Sizing:** lead stated weight [element count · mark mix · reality-surface load]; recommended
    [pair | single | none] ([pair default on first ratification | event-scaled on amend:
    governance event → pair · lighter substantive delta → single · wording-level →
    none-with-recorded-waiver]); **user ruled:** [pair | single | none]
  - *(if none)* **Waiver:** waived by [user] at the sizing gate — [stated reason]. The synthesis
    proceeds to G3 unreviewed; this record is the audit trail.
  ```
- **Replaced by (verbatim):**
  ```
  - **Sizing:** lead stated weight [element count · mark mix · reality-surface load]; the default
    [pair on first ratification | event-scaled on amend: governance event → pair · lighter
    substantive delta → single · wording-level → none-with-recorded-waiver]; **lead sized:**
    [pair | single | none], stated on the user's run-start weight card — below the default also
    takes a departure-trail line.
  - *(if none)* **Waiver:** this record is lead-penned, so `none` is not the lead's to take —
    waived by [user] at the **weight card**, [stated reason]. The synthesis proceeds to G3
    unreviewed; this record is the audit trail.
  ```
- **The waiver half is a correction, not a rename — and it is why this residue was worth fixing
  rather than tolerating.** The old line let the waiver be recorded *"by [user] at the sizing
  gate"*. After U4 the lead owns that gate, so the scaffold as written would have invited a lead to
  size to `none` and record the waiver at a gate it now rules — the lead waiving its own cold
  grade. That is exactly the meta-self-grading hole **R3** identified and **U1-B** closes. Moving
  the waiver to the **weight card** (the user-ruled stop, U1-A) restores the separation, and states
  the lead-penned ground so the constraint reads as ruled rather than arbitrary.
- **Kept deliberately:**
  - **The Review section itself, whole, and its recovery keying** — `DECISIONS.md`-traceable to
    adversarial **D7.7** (*"the synthesis gains a Review section; recovery keys off its state"*).
    A sizing ruling is still recorded, only by a different ruler, so the section's own comment
    (*"no sizing ruling → open the gate"*) and setup's three Review-section Recovery rows still key
    exactly as before — **verified against `commands/setup.md` this run** (rows at *"Review section
    empty (no sizing ruling)"*, *"sizing ruled, survivors undispositioned"*, *"Review section
    verified or waived"*).
  - **The default keying, verbatim in content** — pair on first ratification; event-scaled on
    amend with all three limbs (governance event → pair · lighter substantive delta → single ·
    wording-level → none-with-recorded-waiver). U4 moved who rules the sizing, never what the
    default is.
  - **The weight-statement inputs** — element count · mark mix · reality-surface load, unchanged.
  - **The waiver's audit-trail purpose and both fill slots** — `[user]`, `[stated reason]`, and
    *"The synthesis proceeds to G3 unreviewed; this record is the audit trail"* verbatim.
  - **Every other slot in the block** — the Review line with its lens/tally/status fills, the
    survivor-disposition table, the verify pass, and the G3-edit delta-pass: untouched.
- **Added (not a supersession):** *"below the default also takes a departure-trail line"* — P20,
  making the flipped doctrine's departure obligation visible where the sizing is recorded.
- **Consumers assessed** — grep-verified across `plugins/` this run, four consumers, one
  disposition each:
  - **`plugins/mochiko/commands/setup.md`** — the **only command** consumer
    (`grep -rln 'governance-intent-template' plugins/mochiko/commands/` returns setup alone).
    Flipped in this same wave under `.mochiko/strips/setup.md` [v0.43.0]; this scaffold edit is the
    consumer-side completion of that flip. Recovery keying re-verified, above.
  - **`plugins/mochiko/skills/authoring-constitution/references/INTERROGATION-AGENDA.md`** — a
    second consumer in `plugins/`, found by the same grep and named here rather than left silent.
    It cites this template **only as a pointer** to where the synthesis output lives (*"Its output
    is the session synthesis at `.mochiko/memory/governance-intent.md` (see
    `templates/governance-intent-template.md`)"*); it restates neither the Review section nor the
    sizing line. **Unaffected, not edited.**
  - **`commands/brainstorm.md`** — **the wave lead's claim, verified before writing it and it
    holds.** brainstorm's `record.md` has **no equivalent scaffold**: there is no record template in
    `templates/` at all, and the only `user ruled` sizing scaffold anywhere in `plugins/` was the
    line superseded here. brainstorm's Review-section shape is described in the command body, and
    its own sizing construction is already flipped in this wave (*"Review sizing is the one
    lead-ruled gate here"*). Nothing to edit.
  - **`templates/sized-end-stage-review.md`** — the doctrine home, already flipped at **v2** in the
    v0.40.0 doctrine wave. Not edited; it is what this scaffold was brought into line with.
