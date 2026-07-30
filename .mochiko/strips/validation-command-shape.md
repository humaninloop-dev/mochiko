# Strip notes — `skills/validation-command-shape/`

Entry formats: `strips/README.md`. Wave context: skill-succinctness wave 1 (design:
`.mochiko/brainstorms/skill-succinctness-strip/record.md`, batch-ratified 2026-07-25): body
80 → 79 lines, 1 cut = 1% — **deeply under the 30–70 never-stripped band by ruling** (born under
current conventions; D1 forbids cutting to reach a band).

---

# v0.33.0 — the grader revised alongside shape v5 (CS-D9)

**Wave context:** command goal-shape rebuild, **step 1 of 4** (design:
`.mochiko/brainstorms/command-succinctness-strip/record.md`, **CS-D9** in its reworked,
user-ratified form; `DECISIONS.md` 2026-07-30). D9's ground: the v4 floor keys on the v4 anatomy
and "would FAIL every conformant goal-shaped file", so the grader lands *with* the shape and
*before* any command is re-authored. Body 118 → **208 lines, 1,033 → 2,160 words, 7,179 →
14,341 B (+109% w)** — measured after the pilot-checkpoint delta and the audit fix pass; the
figures first recorded here (180 / 1,861 / 12,441) were pre-delta and are superseded.
**This is a growth revision, and deliberately so** — five new deterministic floor
checks replace judgment that v4 could only ask for in prose, and the audit fix pass added the
term definitions that make one of them reproducible. It is not a strip pass and claims no
reduction. Author ≠ grader intact: `command-architect` authored both this and shape v5;
`mochiko:validator` × these checks (16–19) grades them.

**`description:` measured first** (skill-library axis 3, 1,536-char delivery truncation):
840 → 1,092 chars = **71.1% of the boundary**, up from 54.7%. Not truncated. The added text is
trigger surface for the new floor (five-block/anatomy/ordinal/ceiling/slot vocabulary) — the
phrases a caller uses when asking for a v5 grade.

## [v0.35.0] Ceremony polish — three residual auditor fixes to the check-6 term definitions

- **Disposition:** corrected in place (correction class — no content left or entered the
  primitive; three definitions were made unambiguous)
- **Tier failed:** n/a — auditor-found defects at the wave ceremony, not a strip
- **Cause:** two of the three are the **stale-summary failure mode** this build has now hit four
  times (`.mochiko/strips/plan.md`'s standing-habit block quote): a definition written against an
  earlier draft of the surface it measures. The third is a plain ambiguity nobody had needed to
  resolve until two commands contested it.
- **Content:**
  1. **`G` re-keyed to the complete three-part form.** Was "bullets matching `^- \*\*` that
     contain `evidence:`"; now requires **all three** of `evidence:` · `rules:` · `decides:`.
     Keying on `evidence:` alone over-counted by one wherever a non-gate bullet cites evidence —
     **two live instances**, `brainstorm`'s Invariants and `specify`'s Enrichment — and each
     false hit loosened that command's Constraints ceiling by 90 w. Measured under the corrected
     rule and recorded in the check as its own evidence: **G = 4 brainstorm · 4 specify · 4 slice
     · 10 setup · 8 implement · 7 plan** (37 total). No command's verdict changes: brainstorm
     tightens 630 → 540 against a measured 511 (5.4% headroom), specify likewise stays under.
  2. **`A` gains the KM-fold exclusion.** A doc the command folds *into* rather than produces
     (`ARCHITECTURE.md`, `GLOSSARY.md`, `DECISIONS.md`, a session index) is not one of its own
     outputs — the KM binding already carries the `+30` term, so counting a fold target paid for
     the same content twice and inflated Bindings by 12 w each. **Contested twice — at `plan` and
     at `implement`** — before being written down, which is the signal that it belonged in the
     definition rather than in each grader's head.
  3. **Heading-vs-ceiling ambiguity resolved: the `## Heading` line does not count.** A 2-word
     ambiguity, resolved on the calibration's own recorded evidence rather than by preference —
     `brainstorm`'s measured Bindings floor of 113 w sits at ~0.9% headroom under the
     un-augmented 114 (90 + 12·2), which is arithmetic only with the heading excluded. Counting
     it would put the declared floor case 1 w *over* its own ceiling and make the `+30` term's
     stated ground incoherent.
- **Kept deliberately:** every ceiling formula, including `90·(G+2)` — the tightened `G` changes
  what the formula is fed, never the formula.
- **Regression check run, and one consequence handed off.** Constraints was re-measured for all six
  under the tightened `G`: every command still passes (brainstorm 511/540 · specify 441/540 · slice
  470/540 · setup 1,051/1,080 · implement 796/900 · plan 789/810). Bindings is the surface fix 2
  moves, and **`implement`'s verdict is now sensitive to how `A` is counted**: its Bindings block
  measures **178 w**, against a ceiling of 192 at `A = 6` (7.3% headroom), **180 at `A = 5`** (1.1%
  — the "fails on formatting, not on bloat" zone), and **168 at `A = 4`, which is a FAIL by 10 w**.
  Reading its Bindings under the corrected definition — working code + `cycle-report.md` + the
  per-cycle verification report + the final-validation report + the built-vs-approved diff report,
  with the design inputs excluded as references and `ARCHITECTURE.md` excluded as a fold target —
  gives `A = 5`, so it passes narrowly; the floor table's pre-wave fitted `A = 4` would fail it.
  **`A` is a judgment count the grader owns, so this is a handoff, not a verdict:** the bounded
  confirm of this edit should settle `implement`'s `A` explicitly rather than inherit either
  number. `plan`, the other contested case, is unaffected with room (231 w against 276 at `A = 13`
  once its two fold targets come out). `specify` is unaffected (145 w against 156 at `A = 3`).

## [v0.33.0] Per-check disposition table — all ten v4 command-grading checks, plus 11–14

D9 requires this table and makes **silence on any check a FAIL** in the audit of the revision.
Nothing is dropped: **0 dropped · 3 kept verbatim · 5 re-keyed · 2 carried-with-an-added-clause**,
and **5 floor checks are new**. v5 renumbers (floor 1–10 · ceiling 11–15 · revision 16–19), so
every row names both numbers.

| v4 check | v5 | disposition | why |
|---|---|---|---|
| **1** References present | **1** | **re-keyed by ruling — disposition pending** | D7's read-drop changes this check's reference set. Per D9 its disposition "lands with the D10 checkpoint, never silently": v5 states that until that ruling lands, a missing `loop-discipline` reference is a FAIL and its absence is never read as anticipated. The KM-path clause is **held as-is** — shape v5 now homes the *generic* landing ritual, which would let a v5 command stop naming the project copy, but no ruling in CS-D1–D10 removed the reference, so the guard stands and the tension is flagged (below). |
| **2** Frontmatter | **2** | **kept** (verbatim) | Named kept in D9. Unaffected by the anatomy. |
| **3** No restated shape prose | **8** | **re-keyed** | D9: "3's signature list re-keyed to the new forbidden markers." All five v4 markers survive; one **home changed** (`the forbidden form` / `forbidden transport` → `command-shape.md` Layer 2, per D6's absorption). Six markers added from the checker map's measured 4–6/6 cross-command repetition set: `input, never the gate` · `Out of rounds = escalate` · `disjoint agents, disjoint skills` / `structurally separated` · `unsized by design` · `respawn is cold by design`. Each is doctrine the home states with zero parameter content. **Guarded against over-reach:** naming *which* validation branch a workflow runs is P6's binding and explicitly not a hit — only explaining the branch is. |
| **4** Exceptions marked | **9** | **re-keyed** | D9: "4 re-keyed — both live `shape-exception` markers re-justified against v5." Added rule: a marker whose cited ground is a v4 section that no longer exists is a FAIL. The two live markers are `plan.md:227` (AD-D8/R5, the un-rendered-diagram degrade-with-record) and `setup.md:100–101` (the falsified routing=independence correction) — both are on v4 files this step and are re-justified at the pilot/wave, not here. |
| **5** Version stamps | **10** | **kept** (verbatim) | Named kept in D9. Governs strip-wave and revision runs unchanged. |
| **6** Altitude | **11** | **carried forward unchanged** | D9: "6 (altitude) … carried forward unchanged." Text unchanged. |
| **7** Parameter completeness | **12** | **re-keyed** | D9: "7 re-keyed to the v5 slot list." Now enumerates **P1–P16** with the three conditional slots marked (P2 team-form · P13 sized review · P14 devolved branch), and requires a non-binding conditional slot to carry its stated absence. |
| **8** Contract fill sound | **13** | **re-keyed** | D9: "8 re-keyed (Contract-section fill → Goal/Constraints soundness)." The four-clause Contract check becomes: measurable end state, workflow-real not-done states, every gate line carrying all three of evidence/who-rules/what-it-decides, every bound owned by the lead, and no Goal check that no Seats row produces. |
| **9** Preserved responsibilities | **14** | **carried + new clause** | D9: "9 … carried forward unchanged, with 9 gaining a clause **grading D8's structural-prevention claims**." Added: a "structurally prevented by the new anatomy" supersession claim is verified against the anatomy and never taken on the author's say-so; plus D8's user-ruled extension (U4) — every line traceable to a `DECISIONS.md` row, not only Tier-2-evidenced lines, must be translated or superseded by a cited ruling. |
| **10** Strip-note quality | **15** | **kept + new clause** | D9: "10 kept" (restored at the verify pass's precision fix). Added: a revision's note must carry its slot map, and **retirement by omission is a FAIL** (D2′'s not-done state). |
| **11** Footer stamped | **16** | **kept** | Revision-run floor; D9 silent, unaffected by the anatomy. Needed intact because the step-4 read-drop edit is graded under 16–19. |
| **12** Rewrites logged | **17** | **kept** | As above. |
| **13** Ruling fidelity | **18** | **kept + new clause** | Added: a ruling the author *reports* as un-encodable is graded on its reported reason, never waved through — closing the gap that an author's "couldn't encode this" would otherwise exit the check. |
| **14** Altitude + re-audit set | **19** | **kept + new clause** | Added: where a ruling defers the re-audit to a named later ceremony (CS-D10's pilot + wave), the note's re-audit-coverage line must name that ceremony **and** the delta set it covers — the case this very revision is in. |

**New floor checks (v5 3–7)** — all five are D9-mandated and all are mechanical:

| v5 | check | D9 ground |
|---|---|---|
| **3** | **Block presence per binding, not per heading** — a vacuous block may be one-lined or omitted only with the absence *stated*; a silently missing block, or a stated absence the body contradicts, is a FAIL | D9 "block presence per binding (not per heading — D5 fold (c))" |
| **4** | **Forbidden headings** — `## Phase`, `## The flow`, **and `## Contract`** | D9 names Phase/The flow; `## Contract` added on **D5**'s "the Contract section disappears as a section" — without it the retired appendix could return under its own heading unchecked |
| **5** | **No ordinal steps inside Constraints** — no `^\s*\d+\.\s` line, no `step \d` cross-reference | D9 "a forbidden ordinal-step pattern inside Constraints"; M-I7's teeth ("ordering narrative cannot return under a permitted heading") |
| **6** | **Per-block ceilings keyed to gate/seat/artifact/row counts** | D9 "a per-block line ceiling keyed to gate count" — **denominator changed to words, see below** |
| **7** | **No seat row grades its own output** — mechanical row extraction; no row claims authorship and grading of one artifact, no agent × skill pair is both producer and grader of one artifact | D9 "the Seats & checks table's mechanical no-row-grades-its-own-output check" |

### Denominator deviation on check 6 — reported, not silently taken

D9's text says "a per-block **line** ceiling keyed to gate count". v5 encodes a per-block **word**
ceiling. Ground: the same session's correction **C2** rules that "line counts are kept as raw fact
only" on this surface because five commands are hard-wrapped at 64–86 chars/line while
`brainstorm.md` is unwrapped at 203, and a past rewrap changed `specify.md` by **+106% lines at
+1.0% chars** — so a line ceiling would grade wrap convention, and a re-wrap could fail a
conformant file or pass a bloated one. The ruling's *function* (a hard ceiling with teeth, keyed to
gate count) is encoded; only its unit differs. **Flagged for the audit** as a deliberate deviation
from a ruling's letter in service of the same ruling's session's own measurement correction — the
auditor's call, not the author's, and reversible to lines if ruled.

The ceilings — preamble ≤ 130 · Goal ≤ 150 · Seats & checks ≤ 100 + 45·seats · Constraints ≤
90·(gates+2) · Bindings ≤ 90 + 12·artifacts **+ 30 with a KM/index binding** · Recovery ≤
60 + 14·rows — are calibrated on the v5
floor arithmetic (`.mochiko/strips/command-shape.md`). Headroom over the tightest **measured**
floor is 13% (`plan`'s Constraints, 705 w against 810) — see the audit fix pass below, which
corrected both this figure and the Bindings formula that produced it. They are marked **provisional in the
skill body**, confirmed or revised at the pilot checkpoint — the same treatment the sibling wave's
R3 bands got. Note the ceilings are keyed to counts a command *declares*, so a command cannot buy
headroom without adding a gate, seat, artifact or resume row that the other checks then grade.

### Post-checkpoint delta (same revision, ruling B) — two checks re-pointed

The pilot checkpoint adopted the sized-review split (`.mochiko/strips/command-shape.md`,
[v0.33.0] Sized end-stage review relocated). Two checks moved with it; nothing else in this
revision changed, and the disposition table above still holds row for row.

- **Check 1 (references present)** gains a **bidirectional** clause: a file whose P6 binds a
  sized end-stage review must contain `sized-end-stage-review`; a file declaring the
  in-loop-critique branch must **not** — loading the conditional read where it does not bind is
  the sham read the split exists to prevent, and it is exactly the failure D4's skip-path rule
  is written against. This is the check that makes the relocation's reduction credit auditable
  rather than asserted.
- **Check 8 (no restated shape prose)**: the `unsized by design` marker's home moves from
  "Layer 1's sized-review branch" to `templates/sized-end-stage-review.md`. The marker itself is
  unchanged, as is the guard that naming which branch a workflow runs is P6's binding and not a
  hit.

Unchanged by the split, confirmed: **check 12 (parameter completeness)** — the four slots the
relocated text cites (P5 · P7 · P12 · P13) are declared in the shape's anatomy section, not in
the moved block, so the P1–P16 set is intact. **Check 6 (per-block ceilings)** keys on a
*command's* blocks, not the shape's. **Check 13**'s P13 clause (verify-pass owner, sized review
only) still resolves.

### Audit fix pass (v0.33.0, same revision) — checks 6 and 8 repaired

The independent audit returned FAIL with two blocking defects in this skill. Both were real: a
check that could not be run reproducibly, and markers pointing at homes that did not contain
their content.

**Check 8 — three markers had no home** (a marker that fires on prose with nowhere to be
relocated *to* is a broken check, not a strict one; it would FAIL a command for a line the author
cannot legally move). Fixed, and a standing clause added so the defect class is auditable in
future revisions: "a marker whose home does not contain it is a broken check … fixed by re-keying
the marker or by homing the content, never by leaving it to fire."

| marker | defect | fix |
|---|---|---|
| `unsized by design` | Absent from both the shape and the new reference file, **and** check 8's own guard exempts the only way commands actually use it (inside the P6 validation-model declaration, where it is a binding). Self-contradicting: forbidden by the list, exempted by the guard, homed nowhere. | **Marker dropped.** No home carries it, and the legitimate use is a binding. Dropping it is the honest fix; forbidding it would have failed all four in-loop commands for a correct P6 line. |
| `structurally separated` | Zero `structural*` hits anywhere in v5 — the phrase left the home when the v4 Contract clause ("different agents, different skills, structural separation") was retired into the anatomy, and nobody noticed the marker still pointed at it. | **Re-keyed to `structural separation`, and the home now carries it:** Layer 2 Independence by structure states that structural separation *is* what independence-by-seat-assignment means. Content, not new doctrine. |
| `respawn is cold by design` | Same defect, undetected by the audit: no home stated it, though 5 of 6 commands say it. | **Home added:** Layer 2 Independence now derives it from the cold-arrival property — a seat refilled at its own stage is cold by the same definition, so recovery never costs independence. |
| `the forbidden form` | A phrasing variant the home does not carry (the home carries `forbidden transport`). | **Kept, with the relationship made explicit:** both name one homed rule, and a known phrasing variant of a homed rule is a hit. An unhomed *rule* is not. |

Also aligned: the anatomy's bounds sentence now reads "out of rounds = escalate, never done", so
the `Out of rounds = escalate` marker matches text the home actually carries rather than a
paraphrase of it.

**Check 6 — not deterministic, and mis-calibrated.** Two defects:

1. **`G` had no defined source.** `brainstorm` reads **4** by the floor arithmetic (sizing ·
   survivor rulings · tie-break · acceptance) and **0** by the checker map's "0 numbered gates" —
   so author and grader could compute different ceilings from the same file, which is
   disqualifying for a floor check. **Fixed at both ends:** the shape now mandates a countable
   gate-line form for P7 (`- **<label>** — evidence: … · rules: … · decides: …`), and the check
   defines `G` as bullets matching `^- \*\*` containing `evidence:`, with an explicit warning that
   `G` is *not* numbered gates and that counting numbers is the miscount the clause exists to
   prevent. `S`, `A` and `R` likewise got mechanical definitions (`A` = the command's own outputs
   only — deliverables + round reports — not input paths cited in Bindings), and the grader must
   **record the term values as the check's evidence**. Side benefit: the mandated form also makes
   check 13's "all three parts present" test greppable rather than a read.
2. **The Bindings ceiling omitted the KM term the floor carries.** The floor model is
   `65 + 9·A + 30` where a KM-landing or index-bookkeeping binding applies; the published ceiling
   was `90 + 12·A` with no such term, leaving `brainstorm`'s measured Bindings floor at **~0.9%
   headroom** — a ceiling that fails on formatting rather than bloat. **Fixed:** `Bindings ≤
   90 + 12·A, plus 30 where a KM-landing or index-bookkeeping binding is present` (brainstorm
   → 144 against a 106 w floor, 26% headroom). The recorded "tightest headroom" claim is
   corrected with it: the tightest **measured** block is `plan`'s Constraints at 705/810 (13%),
   not setup's Constraints at 780/900 — that 13.3% figure was fitted, not measured, and the note
   said "measured".

## [v0.34.0] Check 8's `out of rounds = escalate` marker dropped — third homeless-marker instance
- **Disposition:** deleted from check 8's marker list.
- **Tier failed:** n/a — supersession by ruling (**pilot-checkpoint ruling**, user, 2026-07-30;
  ADR `.mochiko/decisions/2026-07-30-goal-shape-pilot-checkpoint.md`).
- **The defect:** the marker was **mis-specified, not merely homeless.** Grepped
  case-insensitively it fires on a conformant command's ruled bounds line — and **D5 fold (a)'s
  graded exemplar mandates that exact phrase in Constraints**: "Bounds (once, for every loop, in
  Constraints): round cap lead-counted · no-progress exit · kill-switch · **out of rounds =
  escalate, never done**." A check cannot forbid text the ruled preservation standard requires. Same
  class as `unsized by design` (dropped at the v0.33.0 fix pass): a marker keyed to a phrase that
  is a *binding* in the command, not a restatement of doctrine the command could point at instead.
- **Content (the removed marker, verbatim):** "`out of rounds = escalate` (home: the anatomy's
  Constraints spec, which carries it in lowercase mid-sentence — grep case-insensitively; commands
  capitalise it sentence-initially)".
- **Kept deliberately:** the anatomy's own bounds sentence in `command-shape.md` still reads "out of
  rounds = escalate, never done" — the *doctrine* is homed and unchanged; only the grep that
  punished commands for honoring it is gone. Also kept: the standing clause added at v0.33.0 that a
  marker whose home does not contain it is a broken check — it is what caught this one, and it now
  needs a companion reading, recorded here: **a marker must also not fire on text the exemplar
  mandates.**
- **Pattern, third instance:** `unsized by design` (author-detected at the v0.33.0 rework),
  `structurally separated` + `respawn is cold by design` (audit-detected, same pass), and this one
  (audit-detected at the pilot). All four share one cause — **markers written from the *commands'*
  repeated phrasing (the checker map's 4–6/6 repetition set) rather than from the *home's* text**,
  which silently inverts the check: it grades what commands say instead of what the home owns. The
  surviving markers were re-derived from home text at v0.33.0; this was the last one written the
  old way. Carried into the step-4 wave briefing.

### Open tension carried to the audit

Check 1's KM clause requires the five KM-carrying commands to contain
`.mochiko/memory/knowledge-management.md`. Shape v5's Goal block now homes the generic landing
ritual and instructs commands to state "only its own landing additions" — so a conformant v5
command whose landing carries no additions might legitimately not contain that string, and check 1
would FAIL it. No ruling covers this; the conservative reading (keep the guard) is encoded and the
question is raised rather than resolved. First live test: the `plan` pilot.

## [v0.25.0] `verify-output` ancestry clause
- **Disposition:** deleted; the operative accepted-residual-risk sentence ("the floor is the backstop") kept
- **Tier failed:** 2 (provenance — the retired gate's name adds no check)
- **Content:** "The floor revives the retired `verify-output` altitude gate's floor+ceiling design"
- **Consumers assessed:** 3 consumer files checked at wave open; none reference the clause

## [v0.25.0] KEPT: the entire remaining body (whole-skill survivor ruling)
- **Tier-2 evidence:** contested as a whole at the under-band pass and kept — every line is a
  numbered floor check with grep markers and named single-source homes, or a ceiling check with
  its evidence rule; the verdict block is the output contract. Session ruling: batch-3
  ratification 2026-07-25.
