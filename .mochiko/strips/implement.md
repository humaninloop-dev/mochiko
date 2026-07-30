# Strip notes — `commands/implement.md`

Entry formats: `strips/README.md`. Wave context: the implement cluster wave (BACKLOG item 7, the
**fifth and final** one-shot-command wave after specify's v0.13.0, slice's v0.14.0, plan's v0.15.0,
and tasks' v0.16.0). The wave also ran the **D2 conversion assessment** (one-shot → team-form) and
re-checked the **S8 home-revision checkpoint** against implement's needs (a standing producer spanning
the whole cycle sequence + the fix-pass loop, a standing verifier fired once per cycle + a
whole-implementation final validation, and a per-cycle confidence gate that auto-approves
deterministic-CLI-pass cycles — **no new shape gap at that wave, when the shape was v2**, so it made
no template revision and no cross-command re-audit). **Stale as a standing claim:** the shape is now
**v4** (2026-07-30), and its D3 devolution changed exactly that confidence gate — see the v0.31.0
entries below. **Also stale:** the shape is **v5** as of the v0.35.0 wave below, and the
"standing producer / standing verifier" claim is superseded by that wave's seat-recycling binding.

---

# v0.35.0 — the goal-shape wave (CS-D10 step 4)

**Wave context:** command goal-shape rebuild, **step 4 of 4** — the five-command wave following the
plan pilot (design: `.mochiko/brainstorms/command-succinctness-strip/record.md`, CS-D3/D4/D5 + D8 +
D10; pilot checkpoint ADR `.mochiko/decisions/2026-07-30-goal-shape-pilot-checkpoint.md`). Authored
against **shape v5** with the obligated `mochiko:loop-discipline` read **retained** — the drop is
deferred to a named live-run trigger, so a v5 command that omits it is non-conformant, not early.
implement declares the **in-loop critique** branch of P6, so it must **not** reference
`templates/sized-end-stage-review.md` (check 1's negative direction) — it does not.

**Measured: 3,230 → 1,868 words (−42.2%), 23,873 → 13,331 B (−44.2%)** — `wc`-measured after the
**seat-recycling revert**, which was the last edit (the pre-revert file measured 1,962 w / 13,959 B;
those figures are superseded here and everywhere in this note, per the pilot's standing
headline-sweep habit). Against the wave's pre-authoring floor row of **1,354 w: +514 w (+38.0%)** —
over, not under, which is CS-D8's safe side. The overage is accounted by block: implement's protected
surface is the densest in the library and three blocks sit at 92–98% of their ceilings after every
trim below. The floor row assumed a G=5/S=3 file; this one lands **G=8, S=4** because two source
decision points were restored to gate form and the architecture dispatch was split into two seat rows
(both below).

Block sizes against the grader's ceilings, **with the term derivation as check 6 requires** —
**G=8** gate lines (bullets matching `^- \*\*` containing `evidence:`: G1 entry · package gate ·
governance surface · cycle checkpoint · architecture deviation · G3 · G4 · G5), **S=4** seat rows,
**A=6** own-outputs (working code · `cycle-report.md` · the per-cycle verification report · the
final-validation report · the built-vs-approved diff report · the `ARCHITECTURE.md` fold), **R=13**
resume rows: preamble 114/130 · Goal 144/150 · **Seats & checks 271/280 (96.8%)** · Constraints
796/900 (88.4%) · Bindings 178/192 (92.7%) · **Recovery 239/242 (98.8%)**.

> **At-risk measurement, flagged rather than resolved (auditor: check this term first).** Bindings
> passes at A=6 (178/192) and at A=5 (178/180) but **fails at A=4 (178/168)**. A=4 requires
> discounting two enumerated own-outputs — the built-vs-approved diff report and the
> `ARCHITECTURE.md` fold — both of which this run's own dispatches produce. The +30 KM term is
> claimed and the KM-landing binding is present. If the grader counts A=4, the fix is a real cut,
> not a re-argued term.

**Three ceiling pressures resolved by relocation or reduction, never by loosening a ceiling** (the
pilot's rule — recalibrating a ceiling to fit a file the author wrote is the forbidden
quota-override, and implement is the file most likely to want it):

1. **Constraints opened at 945/810 (G=7).** Resolved by restoring **two source decision points to
   gate form** — the pre-rewrite Phase 0 carried *step 1* (capture and resolve `<feature>`) and
   *step 2* (the plan-package-complete entry gate) as distinct steps with distinct evidence and
   distinct decisions, and *step 3* (governance) as a third. Collapsing all three into one `G1`
   line, as the first draft did, hid two real gates and cost the ceiling 180 words it was entitled
   to. G=8 → 900. The remaining 45 words came from the reductions in the ledger's *dropped* rows.
   The block finally lands at **796/900 (88%)** once the seat-recycling revert removes its 88 words —
   so the trims that bought its headroom stand, and the block is no longer the file's tightest.
2. **Bindings opened at 279/168.** Two bullets in the colliding draft (below) were
   **Constraints-class content parked in Bindings** — the governance obligated-read brief and
   project scaffolding are obligations, not referents. Both moved to their own class: governance to
   its gate line, scaffolding to an invariant.
3. **`·` separators counted as words** in the mechanical count (they are whitespace-delimited
   tokens, as in the pilot's own measurements). Two Bindings list lines were re-punctuated to commas
   (−9 tokens). **Recorded because it is a formatting change, not a content cut** — the auditor
   should treat those two lines as unreduced.

## [v0.35.0] The phase body and the Contract section retired into the five-block anatomy
- **Disposition:** superseded → the goal-shaped anatomy. `Team-form parameters` → the preamble's
  probe-seat line (the env check and transport mechanics are shape Layer 2, referenced) ·
  `Session constraints` → the package gate, the bounds' kill-switch, and Bindings' deliverable /
  ID-namespace lines · `The seats` → the **Seats & checks** table plus the validation-model line ·
  `Phase 0` → **G1** + the **package gate** + the **governance surface** + Bindings' design inputs +
  the scaffolding and slice-scope invariants · `Phase 1` → the **cycle checkpoint** + the
  **architecture deviation** gate + the ordering invariants · `Phase 2` → the ordering invariants'
  lead-routed final validation, the bounds' fix-pass cap, and Bindings' diff report · `Phase 3` →
  **G5** · `Phase 4` → the KM-landing binding + the Recovery table's `accepted` row · `Contract`'s
  four clauses → **Goal** (done-condition + not-done states), the **Seats & checks** table
  (producer↔validator), **Constraints** (bounds + gates) · `State recovery` → **Recovery**.
- **Tier failed:** n/a — supersession by ruling (**CS-D3** condition-first documents · **CS-D4** the
  connective procedure is deleted and what survives is restructured · **CS-D5** the five-block
  anatomy and the Contract-as-document inversion).
- **Content:** eleven `## Phase`/`## Contract`/`## State recovery`/roster sections, ~2,600 words of
  ordered procedure, appendix, and footer. Not reproduced verbatim — every *rule* inside them is
  resolved individually in the ledger below, and the deleted remainder is connective narration
  (step numbering, `Phase N step M` cross-references, per-phase restatements of the lead's job, and
  the reachability sentences that opened each phase). Recoverable in full at
  `git show 7898d86:plugins/mochiko/commands/implement.md` — the authoritative pre-wave baseline,
  292 lines / 3,230 w / 23,873 B (byte-identical at `c47684d`; the intervening step-1/step-2 commits
  landed shape v5 and the plan pilot without touching this file).
- **Kept deliberately:** every gate, bound, predicate, routing decision, trigger, ordering rule and
  artifact binding — resolved row by row in the CS-D8 ledger.

## [v0.35.0] The `What you own (not the seats)` footer deleted
- **Disposition:** deleted.
- **Tier failed:** 1 — a declared duplicate, and implement carried the longest instance in the
  library (**~190 words**) restating the cycle sequence, the round counters, the execute→verify
  pairing, the verdict ownership, the devolved-branch clearing, the deviation escalation, the diff
  trigger, the fix-pass bound, every gate, the entry and governance prerequisites, scaffolding, and
  the never-mount-verification rule — each of which is now a Constraints line, a Seats cell, or a
  Bindings entry. The v0.17.0 wave already deduped this class once (*Verdict-ownership
  triplication*) and it grew back; the anatomy leaves it nowhere to hide.
- **Kept deliberately:** the one clause with no other home — "verifying each seat actually wrote its
  expected files (a missing output → log and ask retry/abort)" — is **not** dropped as behavior: it
  survives as the Recovery block's evidence-driven resume (a missing report *is* a resume row: two
  rows key on "no `cycle-report.md` this round" and "no verification report this round") plus G4's
  escalation menu. Same disposition the pilot gave the identical clause; the anatomy's Recovery
  block is the structural-prevention claim, and it is checkable in the table.

## [v0.35.0] Seam-N1 narration replaced by two seat rows
- **Disposition:** superseded → structure. The `architecture scribe` roster entry's ~55 words of
  "**two distinct firing conditions**, kept separate (seam N1)" prose, plus the two mid-body
  "seam N1 — distinct from …" reminders, are replaced by **two rows in the Seats table**:
  `arch-diff` (spawn: at final validation, whenever an approved structural delta existed) and
  `arch-scribe` (spawn: at finalize, on a built structural change only).
- **Tier failed:** 1 (altitude) — the seam was being *asserted in prose* three times because the
  single row could not show it. Two rows make the broad/narrow trigger split mechanically visible in
  the parameter the shape already provides for it (P5's spawn column), which is what the v0.32.0
  build note asked the build to resolve.
- **Kept deliberately:** both triggers, in their exact breadth — the diff fires on
  *approved-delta-existed* (independent of what was built, so a silently-descoped approved delta
  cannot escape) and the fold on *built structural change*; the diff's report reaching **G5**; and
  "never the verifier seat", stated once on `arch-diff` and shown by the table for both rows.
- **Deliberate call, flagged:** both rows are `principal-architect` × `authoring-architecture`, so
  the pair appears twice. Check 7 is satisfied — neither row grades an artifact it authored, and the
  two rows touch different artifacts (`architecture.md` + built code for the diff; `ARCHITECTURE.md`
  for the fold) — but the repetition is deliberate and worth the grader's eye.

## [v0.35.0] Skill- and shape-owned content stripped from the command body
- **Disposition:** relocated → the homes that already carry it (no new home written).
- **Tier failed:** 1 (altitude).
- **Content:**
  - `input, never the gate` — stated **three times** (roster, Contract done-condition, footer). Home:
    `command-shape.md` Layer 2 *Clearing*, and it is check 8's keyed marker. Survives once, as the
    validation-model line's "qa's output is **lead-adjudicated input** and the verdict is yours" —
    the pilot's audit-cleared phrasing.
  - "Disjoint agents, disjoint skills, structurally separated" (Contract, Producer ↔ validator).
    Home: Layer 2 *Independence by structure*, which states both phrasings; the table *shows* it.
  - "a verifier respawn is cold by design" (State recovery preamble). Home: Layer 2 *Independence by
    structure* — "a respawn is cold by design". Recovery keeps only what respawning *re-reads*.
  - "Never modify git or push" (Phase 4). Home: Layer 1 *Ground rules*.
  - "out of rounds = escalate, never done" (Contract bounds). Home: the shape's Constraints block.
    **Contested-adjacent:** the audit-PASSed pilot keeps this sentence, so keeping it was permitted;
    it is cut here because it is verbatim shape prose and Constraints needed the words. The
    semantics survive in the Goal's not-done state "out of rounds".
  - "drift caught one cycle deep, never deferred to landing" (Phase 1 / footer) and "the same
    mechanism as plan's design-time return to sign-off". Home: this note's v0.32.0 entry — design
    rationale and a cross-command provenance pointer, both of the class the pilot relocated out of
    Constraints under audit pressure.
  - "Round reports are cleaned by default; never offer to delete a deliverable." **This line never
    existed in implement** — it entered from the colliding draft (below), imported from `plan.md`.
    Removed as an import, not stripped: implement's cycle reports are the audit trail **and** the
    input a recycled producer is briefed from, so a default clean would break the seat-recycling
    binding.

## [v0.35.0] CS-D8 survivor re-grade ledger — every protected line resolved

CS-D8 (extended by user ruling U4) protects `KEPT:`/Tier-2-evidenced lines **and** every line
traceable to a `DECISIONS.md` row. implement carries **no `KEPT:` survivor-provenance entries**; its
protection set is the *Kept deliberately* fields of the four prior entries (v0.17.0 conversion,
v0.31.0 ×2, v0.32.0) plus the DECISIONS row trace, grepped before any cut. **All 26 rows survive
translated; one is superseded with grounds; zero dropped.** Per the pilot's warning that losses hide
in *compressed evidence clauses* rather than deleted sections, the devolved-branch predicate and the
deviation triggers were re-read clause by clause against `git show c47684d` after the last edit.

| protected line | source | resolved |
|---|---|---|
| AD-D6.1 — the approved `architecture.md` is **briefed input**, read at entry and carried in the producer's per-cycle brief | DECISIONS row AD-D6; v0.32.0 | Bindings' design inputs, marked "the **anchor** for both architecture mechanisms"; the package gate's evidence; the producer seat row's deviation self-check |
| AD-D6.2 / R7 — the **diagram-anchored mechanical test**, verbatim: add/remove a box · add/remove/redirect an arrow · move a responsibility across a boundary | DECISIONS row AD-D6; v0.32.0 | The **Architecture deviation** gate's evidence clause, all four triggers intact |
| AD-D6.2 — the self-check runs at **cycle open AND cycle close** | v0.32.0 (emphasised there) | Same gate line: "run at cycle **open** and cycle **close**" — the conjunction kept, not compressed to "each cycle" |
| AD-D6.2 — a surfaced deviation **stops and surfaces**, never silently built; the user re-rules; the target is **amendable mid-implement with consent**, updated before the cycle resumes | DECISIONS row AD-D6 | Same gate line's rules/decides clauses + "never silently built" |
| AD-D6.3 / R8 — the **built-vs-approved diff** fires whenever an **approved structural delta existed** (broad, independent of what was built), at final validation, in **diff mode** | DECISIONS row AD-D6; v0.32.0 | `arch-diff` seat row (skill + diff mode + spawn trigger) · **G5**'s evidence · the Goal's end state and not-done state · a Recovery row |
| AD-D6.3 — the divergence **surfaces at the acceptance gate** | DECISIONS row AD-D6 | **G5** evidence: "**and** the built-vs-approved architecture result where an approved delta existed" |
| Seam N1 — the diff's broad trigger vs the fold's narrow **built-structural-change** trigger, kept separate | v0.32.0 (build-seam resolution) | Two seat rows + the KM-landing binding's "a **built** structural change" (entry above) |
| Team-method D3 — the **cycle** is the clearing unit and the devolved branch applies to it | DECISIONS row; v0.31.0 | Bindings' **Clearing unit + checkpoint keying** (P14) + the cycle-checkpoint gate line |
| Team-method D3 — the predicate, **exactly**: every verification a deterministic CLI check at **100% pass** AND no deviation reported AND `domain_deps_added` empty | v0.31.0 | Cycle checkpoint, with "skipped **exactly** when" and all three conjuncts — the compressed-evidence-clause check's primary target, re-read against the prior text |
| Team-method D3 — a clean cycle clears on qa's **PASS-with-evidence, unread by the lead**, counted from its clearance notice | v0.31.0 | Same line's devolved clause; the Goal's "every **escalated** cycle's reports" carries the not-re-read consequence |
| Team-method D3 — **everything else returns to the lead**: any failure · any GUI or subjective verification · any reported deviation · any registry addition | v0.31.0 | Same line's "Otherwise it fires — …", all four classes enumerated |
| AD-D6 × D3 fold — a surfaced architecture deviation **is** a reported deviation and **de-devolves** the cycle (no parallel gate) | v0.32.0 | Bindings' P14 line ("a surfaced architecture deviation **de-devolves** it") — kept out of the gate line so the fold is stated once |
| Domain-allowlist D2/F2 — the `domain_deps_added` **visibility floor**: disclosed in the cycle report and surfaced at the checkpoint | DECISIONS row (2026-07-21) | Cycle-checkpoint evidence + Bindings' uncertainty carrier (the field named in both) |
| Domain-allowlist — the **confidence-gate hook**: a registry addition at `production`/`regulated` **forces** the human checkpoint regardless of deterministic-CLI pass; lower tiers surface non-blocking | DECISIONS row | Bindings' P14 keying line, with the tier source (`CLAUDE.md` stamp) named |
| v4 mesh D1/D2 — verifier **cold at the first cycle verification**, standing after; the producer↔verifier **peer edge** declared on the roster | v0.31.0 | Verifier seat row (spawn + peer-edge cells) |
| v4 mesh — the **endgame is lead-routed**; the devolved branch clears cycles, never the final validation | v0.31.0 *Kept deliberately* | Ordering invariants: "The final validation is lead-routed, never devolved" + the verifier row's peer-edge cell |
| v4 mesh — a **retry is lead-dispatched** (a retry follows a failure; the verdict on a non-clean unit is the lead's) | v0.31.0 *Kept deliberately* | Producer seat row: "retries and fix passes are dispatched by you" |
| Delivery is a hand-off, not a start signal — the pairing is the lead's to enforce | v0.31.0 / v4 mesh | Ordering invariants: "the hand-off is peer-routed, the pairing is yours to enforce" |
| Standing-seat **D1/D2** — conditioned checkpoint recycling: cycle floor (~≥3) + gate-pause check, same-name successors, artifact-only respawn briefs, just-failed report relayed at dispatch, verifier per slice | DECISIONS row (2026-07-23) | The **Seat recycling** invariant + the producer/verifier spawn cells. **An addition, not a translation — see the contested call below.** |
| The **`@`-reference recovery** — empty `$ARGUMENTS` has a **named cause** (the `@`-reference drop bug) and a **two-option prompt** (re-enter, or confirm the detected feature) | Pilot fix-round restore; record §7 protected set | **G1**, cause and both options intact, plus the detection rule (explicit ID, else the most recent in-progress feature). The pilot lost this once in a compressed evidence clause; written first here and re-checked last |
| Roadmap-v2 — **sequential implement**; parallel cycle execution is a `deliberate-shortcut-ledger` deferral, **not a capability drop** | DECISIONS row (2026-06-27); BACKLOG parallelism item | Ordering invariants, both clauses |
| Foundation cycles before feature cycles; current cycle = the first with unchecked tasks | current body | Ordering invariants |
| The **execute→verify pairing** — every produced cycle verified in the same round, **never skipped** | current body / v0.17.0 | Ordering invariants |
| The verification skill is **never mounted on staff**, and staff never grades its own cycle | v0.17.0 *Verdict-ownership* strip (deduped to once) | Validation-model line, once: "The verification skill is **never** mounted on the producer" — the second half is shown by the table, not asserted |
| Bounds — targeted retry **max 3/cycle** re-opening only the failed tasks and never regressing passing code · fix pass **max 3**, failure-scoped, **unconstrained by cycle boundaries** · convergence stall at **2+ rounds** · no-progress = an unchanged failing set · `IMPLEMENT_STOP` checked **before each seat send** | v0.17.0 + current body | The **Bounds** line (caps, stall, no-progress, kill-switch) + the producer seat row (the fix pass's cycle-boundary freedom) |
| **No G2** — a single verifier, so no feasibility-rejection gate (the audit-passed reword) | v0.32.0 (preserved there) | Validation-model line, where it now *follows from* the single-verifier fact rather than sitting as a standalone note |
| Slice binding 1 — the package gate and the cycle loop read `slices/<slice>/tasks.md` | v0.17.0 slice strip, *four genuine bindings kept* | Slice-scope constraint |
| Slice binding 2 — design inputs = shared feature-root artifacts **plus** `slices/<slice>/{plan.md, task-mapping.md}` | same | Slice-scope constraint |
| Slice binding 3 — per-slice outputs land under `slices/<slice>/`, and what that does to the artifact set | same | Bindings' **Reports** line ("or `slices/<slice>/` when slice-scoped, where the Goal's artifact set reads them") — moved out of Constraints to the block that owns paths |
| Slice binding 4 — the quality gates run the **full repository suite** (implement's own operationalization of the contract's regression-safety rule) | same | Slice-scope constraint, with the regression-net reason kept |
| Slice binding 5 — at the last slice the **feature is declared, not verified**; Feature-Done executes at feature-close, owned by no workflow; surface it, never report feature completion | same (implement is the pipeline's terminal stage) | Slice-scope constraint's closing clause |
| The **Graduation contract** is the single home; do not restate it | v0.17.0 audit catch (the D1 churn liability) | Slice-scope constraint opens by naming it as the single home for the six rules and restates none — the defect that entry was written about is not reintroduced |
| Vertical-graduation — the slice-scoped entry variant | DECISIONS row (2026-07-02) | The Slice-scope constraint + Bindings' per-slice report layout |
| Governance prerequisite — surface a missing region (offer `/mochiko:setup`), **never auto-resolve**; governing context, not a blocking gate; present → the **one-line obligated read** of the `paths`-relevant `.claude/rules/mochiko/` files in each code-touching brief | current body / setup-cluster rulings | The **Governance surface** gate, all three parts |
| Workspace-as-state, **no registry field** | v0.17.0 (named a genuine survivor there) | Bindings' design-inputs line |
| The producer's **honest** `cycle-report.md` is the producer-authored uncertainty carrier, not confidence marks | current body (P11) | Bindings' uncertainty carrier, with `mochiko:executing-tdd-cycle`'s format referenced as the field owner |
| KM landing under fix-on-sight; implement records **what it builds** | v0.32.0 / KM invariants | The KM-landing binding. **Superseded with grounds:** the "implement records what it builds" half is deduped — `plan.md`'s KM binding already states the division ("Plan records only what plan itself established … implement records what it builds"), so it survives stated once, in plan. The *behavior* (the fold fires on built structural change) is unchanged here |

**One routing correction, recorded rather than folded silently.** The prior body's line "Route
knowledge / preference / scope gaps per `loop-discipline` (→ **G3** / **G4** / escalate)" read as
*preference → G4*, which contradicts `loop-discipline`'s own routing table (knowledge → research;
**preference → the human gate**; scope → halt or split, i.e. the escalation gate). Translated to the
doctrine's mapping: **G3** rules the preference gap and routes knowledge to `Explore`; **G4** takes
the scope gap. Flagged because the compressed original is ambiguous enough that a reader could call
this a behavior change rather than a correction.

## [v0.35.0] `RETURNED:` — the seat-recycling binding stays unbuilt (contested call, user-ruled out)

**Raised as a contested addition, ruled out by the user at wave ratification, and reverted in the
same version.** Recorded in full because the ruling that keeps it unbuilt is the useful artifact —
standing-seat **D1–D4** remains ruled, and this entry is where the next builder finds why the command
does not yet carry it.

**What was raised.** The wave brief listed "producer checkpoint-recycling (cycle floor + gate-pause
check, same-name successors)" among implement's protected, DECISIONS-traceable surface. It **is**
ruled (`DECISIONS.md` 2026-07-23, standing-seat lifecycle **D1–D4**) but it was **never built into
`implement.md`** — a grep of `git show 7898d86:plugins/mochiko/commands/implement.md` for `recycl`
returns zero, and the file instead declared the pre-D1 claim, "one **named standing seat** across the
whole cycle sequence and the Phase-2 fix-pass loop". `BACKLOG.md` carries the build open:
"**Standing-seat build items (deferred)** — conditioned checkpoint recycling · respawn briefs from
artifacts · the Layer-2 transport-vs-lifecycle rewrite (**v4+**) · per-seat measurement."

**Authored in the include direction** (an 88-w **Seat recycling** invariant plus "recycled per
Constraints" / "recycled per slice" in the two spawn cells), on the ground that the pre-rewrite text
was stale against a ruling, and **flagged rather than silently resolved**.

**Ruled out. Grounds, as given at ratification:** the wave's contract is **translation under
true-reductions-only** accounting, and this was its only line of new behavior; D1's cycle floor is
**probe-calibrated and the D4 probe is deferred**, so the command would carry an approximate
threshold; standing-seat **D3** — the Layer-2 *transport vs context-lifecycle* reframe that would
give each command a per-seat lifecycle `[PARAM]` — is **unbuilt** (v4+, open in BACKLOG, and
`.mochiko/strips/command-shape.md` names it as deliberately not combined into the mesh revision), so
the invariant was a workaround for a missing shape slot rather than a home; and the BACKLOG item
remains the build's proper home.

**Reverted, exactly as costed.** The `Seat recycling` invariant deleted (88 w); the producer's spawn
cell back to "standing across the cycle sequence and the fix-pass loop; **probe seat**, foundation
cycle 1"; the verifier's back to "cold at the first cycle verification, standing after". Measured
after: Constraints **796/900** (projected 796 — exact), Seats & checks **271/280**, file **1,868 w /
13,331 B** — 6 w under the ~1,874 projection, because the two spawn cells shed "recycled per
Constraints" / "recycled per slice" on top of the 88-w invariant. **The ruling is acknowledged here
and unbuilt by design** — the
standing-seat claim in the seat rows is therefore known-stale against D1, not an oversight.

**Never added, so the absence is not read as a drop:** D1's user escape hatch (the user may order a
recycle at any gate), D2's artifact-only respawn briefs and their sufficiency watch-item, and the
relay of the just-failed `cycle-report.md` at dispatch. All lifecycle policy with no consumer in the
goal-shaped file; they stay in the record until the D3 reframe lands. **Re-add trigger:** the
standing-seat build items shipping — the D3 Layer-2 reframe first, since it supplies the `[PARAM]`
this content belongs in.

## [v0.35.0] Collision note — an unledgered orphan draft occupied the working tree

**Baseline provenance, stated because it was briefly in doubt: this rewrite and its ledger derive
from HEAD**, not from the working tree. While the wave was in flight a since-terminated seat,
executing a superseded instruction, overwrote the working-tree `implement.md` at ~23:26 with a
**different** goal-shaped draft (1,934 w / 13,919 B, never committed, no strip entry). It was read
in full before being replaced and is snapshotted lead-side; it is an unledgered orphan and carries
no authority — **reference seed at most.** Every row of the CS-D8 ledger above was re-derived
against `git show 7898d86:…`.

It was also **not** conformant, which is why replacing it rather than extending it was the cheaper
path: measured against the grader's floor it failed check 6 on **four of six blocks** (Goal 155/150 ·
Seats & checks 258/235 at S=3 · **Constraints 792/630** at G=5 · **Bindings 279/168**), and check 13
on the architecture deviation, which it carried as a plain bullet with no
`evidence:`/`rules:`/`decides:` triple despite the body relying on the user re-ruling a surfaced
deviation.

**Three elements it shares with this file are independently HEAD-traceable** — verified line by line,
so nothing here rests on the orphan: the `No G2` note (HEAD:247, folded onto the validation-model
line here), "Neither ends the loop on its own" (HEAD:187), and `domain_deps_added` in the cycle
checkpoint (HEAD, 3 occurrences). The orphan influenced phrasing and placement only. **Two of its
calls were rejected:** the imported report-cleaning binding (see the strip entry above) and the
Constraints-class content parked in Bindings.

## [v0.32.0] Build note + shape-v4 re-conform — implement honors the approved architecture (AD-D6; 2026-07-30)

Design record: `.mochiko/brainstorms/architecture-design-primitive/record.md` (AD-D6 with folds R2/R7/R8,
seam note N1). Not a strip — **additions** (recorded in `DECISIONS.md` row AD-D6, lead-owned landing);
logged here with the version stamp for the audit trail and to name the seam-N1 resolution the record left
to build.

> **Version note:** originally stamped **v0.30.0**; while in flight, origin/main released **v0.30.0** and
> **v0.31.0** (the shape-v3→v4 mesh + devolved-cycle rewrite, the two entries below). The merge rebased
> these AD-D6 additions onto v4, so they land at **v0.32.0** and fold into v4's devolved branch (see the
> re-conform bullet).

- **Briefed input (D6.1):** the approved `architecture.md` joins the design inputs read at Phase 0 step 4
  and is added to the producer's per-cycle brief — it is the **anchor** for the two new mechanisms below.
- **Deviation escalation (D6.2 + R7) — the diagram-anchored mechanical test:** "does this cycle add/remove
  a box, add/remove/redirect an arrow, or move a responsibility across a boundary on the approved diagram?"
  — **self-checked by the producer at cycle open AND cycle close**, reported in `cycle-report.md` and
  surfaced at the cycle checkpoint (Phase 1 step 3). The user re-rules and the approved target is
  **amendable mid-implement with consent** (a consented target amendment updating `architecture.md`, the
  same mechanism as plan's design-time return to G3). Drift caught one cycle deep, never deferred to landing.
- **Built-vs-approved landing diff (D6.3 + R8) — new build capability:** at final validation (Phase 2 step
  3), when an **approved structural delta existed** in `architecture.md`, the `authoring-architecture`
  dispatch runs in **diff mode** (approved target + built code → "built as approved" or the divergence). The
  divergence is surfaced at the **G5** acceptance presentation. This is a *new* capability (R8 — the prior
  `authoring-architecture` only wrote prose from built code); assigned to that dispatch as a named build
  item, taking the approved artifact as input.
- **Seam N1 made explicit (the record's carry-forward):** the `authoring-architecture` dispatch now has
  **two distinct firing conditions**, kept separate at build — the **diff** fires on *approved-delta-existed*
  (broad, independent of what was built, so a silently-descoped approved delta cannot escape both mechanisms),
  run at final validation to reach the G5 decision; the **`ARCHITECTURE.md` fold** fires only on a *built
  structural change* (narrow, the KM writer moment), at Finalize. An approved-but-not-built delta triggers the
  diff without forcing a doc update. **Placement resolution (build decision):** the record has the diff "at
  landing" yet its divergence "surfaces at implement's acceptance," and acceptance (G5, Phase 3) precedes the
  Finalize landing (Phase 4) — resolved by running the diff at final validation (Phase 2, end) so its report
  is available at G5, while the doc fold stays at Finalize. Flagged as a build-seam resolution the record
  deferred (N1).
- **Shape-v4 re-conform (the merge work, this task):** the AD-D6 additions were re-applied onto main's
  v4-conformed implement (the two v0.31.0 entries below) rather than the v3 confidence gate they were first
  written against. The fold: v4 replaced the confidence gate with the **per-cycle checkpoint carrying the
  devolved branch** (a deterministic-CLI-100%-pass + no-deviation + empty-`domain_deps_added` cycle clears on
  qa's PASS-with-evidence, unread by the lead). The architecture deviation self-check **integrates as a
  reported deviation**: a surfaced deviation is a `cycle-report.md` deviation, which **de-devolves the cycle**
  (removing it from the clean branch → lead checkpoint + consented-target-amendment decision) — so the
  deviation rides v4's existing "any reported deviation returns to the lead" rule rather than adding a
  parallel gate. The built-vs-approved diff (Phase 2 step 3) sits on the **lead-routed endgame** (the devolved
  branch clears cycles, never the endgame), consistent with v4's "Clearing under the mesh". Verify hand-off is
  peer-routed (producer→verifier) per the mesh.
- **Consequent edits:** Phase 0 entry gate retargeted to `/mochiko:plan` (the package producer) after the
  `/mochiko:tasks` retirement (see `strips/tasks.md` v0.32.0); done-condition gains clauses **(4)** (the diff
  ran when an approved delta existed) and **(5)** (G5 cleared), atop v4's clause (3) (lead reads escalated
  cycles + final validation only); the per-cycle checkpoint predicate, G5 presentation, state-recovery table,
  and the "What you own" footer updated to carry the deviation check + the diff. The audit-passed "No G2"
  reword ("there is no feasibility-rejection gate") is preserved. **No shape gap** — both mechanisms are
  per-workflow gates/steps folded into v4 doctrine, not a shape revision; shape stays **v4**.

## [v0.31.0] Lead-as-switchboard routing superseded by the in-loop mesh (shape v4 conforming edit)
- **Disposition:** superseded → `templates/command-shape.md` v4 (Layer 2 — "Independence by structure" + "In-loop mesh"). Rewritten in place at command altitude: the verifier is still cold-spawned at the first cycle verification (a spawn-timing parameter), the producer↔verifier peer edge is now declared on the roster, and the doctrine stays in the shape.
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/team-method-vs-command-shape/record.md` **D1**, scoped by **D2**), not a minimalism strip. Permanent no-contact was the falsified claim; cold *arrival* survives as a property of the stage.
- **Content (superseded, verbatim):**
  - seat roster: "spawned **cold at the first cycle verification**, never in contact with the producer"
  - Phase 1 step 2: "**Verify — same round, never skipped.** Message the verifier to verify the cycle against real infrastructure"
  - Contract, Producer ↔ validator: "(verifier cold-spawned at the first cycle verification, evidence/reports lead-routed, no producer↔verifier contact)"
- **Kept deliberately (not superseded):** Phase 2 step 1's lead-routed final validation — the endgame is the lead's under v4, now stated rather than left implicit; and Phase 1 step 1's lead-dispatched targeted retry — a retry follows a failure, and the verdict on a non-clean unit is the lead's.

## [v0.31.0] The clean-cycle verdict devolves to the producer↔verifier pair (shape v4 conforming edit)
- **Disposition:** superseded → `templates/command-shape.md` v4 (Layer 2 — "Clearing under the mesh"). implement supplies the parameters: the **cycle** is its clearing unit, and the escalated branch's checkpoint keying is a `production`/`regulated`-tier domain-registry addition.
- **Tier failed:** n/a — supersession by ruling (record **D3**), not a minimalism strip.
- **Content (superseded, faithfully compressed):**
  - Phase 1 step 3 header + read: "**Confidence gate + verdict (you).** Read `cycle-report.md` + the verification report + qa's evidence." — the lead read every cycle, clean deterministic ones included; those now clear unread.
  - Phase 1 step 3 branch: "if every verification is a deterministic CLI check that passed 100%, **auto-approve** and advance to the next cycle" — the auto-approve was the lead's act; it is now the pair's, on qa's PASS-with-evidence.
  - Contract done-condition (3): "*you* Read the cycle-reports + verification reports" → the final-validation report plus every **escalated** cycle's reports; "qa's status is input, never the gate" gains "wherever judgment exists".
  - Contract human gates: "the **confidence gate** (per cycle: deterministic CLI verifications that 100% pass → auto-approve; GUI / subjective / any-failure / a `production`+-tier domain-registry addition → human checkpoint)" → restated as the per-cycle checkpoint carrying the **exact skip predicate**, per shape v4's Contract requirement.
  - "What you own": "the verdict against the default-FAIL done-condition (qa grades from real infrastructure, you Read the cycle-reports + verification reports and decide …)"
  - frontmatter `description:`: "with a confidence-based per-cycle gate"

## [v0.17.0] Conversion note (D2/S4 — one-shot → team-form, 2026-07-19)

- **Command-specific rationale (user-ratified):** implement runs a producer↔verifier cycle across a
  **variable-length cycle sequence** (foundation cycles before feature cycles, each execute→verify in
  the same round, targeted retry ≤3/cycle) then a **final-validation + fix-pass loop** (≤3 passes) — the
  **longest producer horizon of any converted command**: not two fixed phases but *N* cycles plus fix
  passes over a **codebase that accumulates as it goes**. The context-retention bet is implement's own
  and is its strongest: a **standing producer seat** (`staff-engineer`) carries (1) the conventions the
  foundation cycles set forward into the feature cycles (the brownfield "follow existing patterns"
  consistency, now *within its own* growing implementation), (2) whole-implementation knowledge into a
  **fix pass that is unconstrained by cycle boundaries** (it may touch any cycle's files — a cold spawn
  would rebuild the entire implementation's mental map from disk), and (3) targeted-retry coherence (it
  re-opens only the failed tasks of code it wrote). The verifier maps to a **standing qa seat**: cold at
  the first cycle verification, then messaged once per cycle and again for the whole-implementation final
  validation — its retained per-cycle context is what makes the final validation informed by what it
  already checked rather than a cold whole-repo read. The verifier never contacts the producer, and the
  verification skill is never mounted on staff — independence stays structural. Transport rides the v3
  fix (`agent-dispatch.md` Seat transport + addressability probe on the producer's first spawn, the
  foundation-cycle-1 implement).
- **Steelman recorded (user-ratified with the conversion):** zero successful team-form runs at
  conversion time (two setup defect runs; specify's, slice's, plan's, and tasks' own checkpoints all
  unfired; brainstorm v2 measured standing seats *more* expensive than dispatches). Implement is
  **two-seat** (nearer tasks'/slice's cost than plan's three-seat load), so its team-form tax is moderate
  if the retention payoff doesn't land. Two honest weak points. First, **implement's producer craft is
  specifically built to reconstruct context from disk**: `brownfield-integration`'s whole discipline is
  "read the full file first, identify its conventions, follow them" — so a cold per-cycle producer is
  *designed to be safe* re-reading the accumulating code, and the retention payoff is narrower than the
  raw cycle count suggests (it is the *authorial judgment* — why a pattern was chosen, what scope
  discipline deliberately left out — which the `cycle-report.md` records as prose but a cold reader must
  re-derive, not the code itself, which is fully on disk). Second, **the qa seat is the weaker team-form
  fit** (implement's analogue of plan's fire-once architect): its verification is **Tier-1 deterministic**
  — real-infra evidence + quality-gate exit codes, re-run fresh each cycle, and the final validation
  re-runs the full suite regardless — so a cold-respawned verifier would reconstruct almost nothing;
  modeled as a standing seat messaged per-cycle for uniform transport, its persistence buys the least of
  the two seats. Ruled team-form anyway per D2's declared default + S4 (no prior dogfood evidence
  required; checkpoint below).
- **Confirm-or-revert checkpoint:** the first post-conversion dogfood run (the open "Dogfood
  `/mochiko:implement`" BACKLOG item, Implement-port follow-ups) confirms the conversion or reverts it to
  one-shot Layer-1 form; a revert is logged as a `RETURNED:` entry here. Team-form named checks: the
  producer probe fires the addressability check (the foundation-cycle-1 implement); the standing producer
  seat is messaged (not respawned) across cycles, across targeted retries, **and across the cycle→fix-pass
  boundary** (whole-implementation knowledge carried into a cross-cycle fix pass); the verifier spawns
  **cold at the first cycle verification**, is messaged once per cycle and for the whole-implementation
  final validation, and **never contacts the producer**.

## [v0.17.0] Sound-loop paragraph + four-requirement enumeration
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, One lead) + the
  `mochiko:loop-discipline` reference
- **Tier failed:** 1
- **Content:** "This is a mochiko **sound loop**: invoke **`mochiko:loop-discipline`** and honor all four
  requirements (default-FAIL done-condition, independent validation, bounded iteration, named human
  gates), and brief each dispatch per **`agent-dispatch`**. Those rules are not restated here — this
  command states only what is specific to *this* workflow: the cycle sequence, the execute→verify
  pairing, the retry / fix-pass bounds, and the two implementation gates." — restated loop-discipline's
  own enumeration; the workflow-specific tail survives as the converted goal + the sections themselves.

## [v0.17.0] Per-run contract fill (`workflow-contract.md` → `implement-contract.md`)
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, Contract — the authoring-time-fill
  rule); the per-workflow values survive as the command's authoring-time Contract section (implement's are
  a four-part done-condition, the targeted-retry / fix-pass / convergence-stall bounds, and the confidence
  gate + G5 + G1/G3/G4 + the no-G2 note)
- **Tier failed:** 1 (the shape retired per-run fills whose values are constant at authoring time)
- **Content:** "## Contract parameters (fill the artifact — don't inline it) … Fill
  `templates/workflow-contract.md` → `.mochiko/specs/<feature>/implement-contract.md` with the values
  below, then confirm it against `mochiko:loop-discipline`. The filled artifact is the inspectable proof —
  not this command body."

## [v0.17.0] Verdict-ownership triplication
- **Disposition:** deduped to once (the Contract's Done-condition / Producer↔validator clause; the
  qa's-status-is-input boundary also lives on `qa-engineer`'s persona + REGISTRY's "independent Tier-1
  validator" row). The per-phase Verdict *steps* (Phase 1 step 3, Phase 2 step 2) are workflow mechanics
  and survive.
- **Tier failed:** 1
- **Content:** stated three times pre-wave — the lead framing ("qa presents evidence and a checkpoint
  recommendation; **you own the clearing verdict** … qa's status is input, never the gate"), the Contract
  Team clause ("verifier `mochiko:qa-engineer` … never implements … the verification skill is never
  mounted on staff"), and the footer ("the verdict (qa grades from real infrastructure, you Read the
  cycle-reports + verification reports and decide against the default-FAIL done-condition … qa's status is
  input)").

## [v0.17.0] Footer ground rules + Task-tool transport line
- **Disposition:** kernel-free/git relocated → `templates/command-shape.md` (Layer 1, Ground rules); the
  "always dispatch via the Task tool" line superseded by the team-form conversion (transport now per shape
  Layer 2 + `agent-dispatch.md` Seat transport)
- **Tier failed:** 1
- **Content:** "Stay kernel-free; brief agents per `agent-dispatch`; always dispatch via the Task tool
  (never inline agent behavior); do not modify git or push."

## [v0.17.0] Recovery memory-model parenthetical
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, Recovery — "never a context `phase`
  field")
- **Tier failed:** 1
- **Content:** "Resume from workspace evidence (there is no context-file `phase`/`status`)" + the
  entry-gate parenthetical "(workspace evidence — there is no context-file `status` to read)". The
  recovery table (evidence → resume-at) is the workflow-specific Recovery PARAM and survives, as does
  Phase 0 step 4's "workspace-as-state, no registry field" (a genuine survivor, as in the siblings).

## [v0.17.0] "Why this done-condition differs from HIL's" blockquote
- **Disposition:** deleted (user-ratified)
- **Tier failed:** 2 (no behavior produced — historical/motivational provenance; preserved in ROADMAP's
  Decision Trail + `.mochiko/transform/implement/`)
- **Content:** "> Why this done-condition differs from HIL's: HIL declared "no hard caps," routed on an
  autonomously-evaluated gate verdict, and had **no** final-acceptance gate — it could churn indefinitely
  or self-declare done. The deterministic caps, the lead-owned verdict (qa's status is input), and the new
  G5 acceptance gate close the gates HIL lacked." — the shape of specify's / plan's / tasks' deleted
  HIL-comparison blockquotes; its rationale is carried by the Contract done-condition (the deterministic
  caps + lead-owned verdict + G5), so no unique behavior is lost.

## [v0.17.0] Slice-scoped entry — de-restated to the Graduation-contract reference
- **Disposition:** relocated → `templates/slices-template.md` (the **Graduation contract** section — the
  single home of the consumption rules); Phase 0 step 6 now *applies* the contract by reference for slice
  resolution, the staleness guard, scope, extend-mode, graded amendment, and artifact layout
- **Tier failed:** 1 (the one-shot entry variant declared the Graduation contract "the single source of
  the consumption rules; do not restate it" and then restated slice-resolution + staleness-guard rules
  beneath that self-declaration — the same D1 churn liability the plan wave's `validation-command-shape`
  audit caught on plan's identical entry and the tasks wave de-restated in-conversion; applied here
  proactively by that prior ruling, **NOT contested**)
- **Content:** the copied rules — slice resolution ("named in `$ARGUMENTS`, else the first slice in
  Slice-order whose `slices/<slice>/tasks.md` has unchecked tasks") and the **staleness guard** ("the live
  `spec.md` story-ID set must match the Spec stamp — mismatch → block and point to `/mochiko:slice`").
  implement's genuine own bindings were **kept**: the entry gate + cycle loop read `slices/<slice>/tasks.md`;
  the design inputs are the shared feature-root artifacts plus `slices/<slice>/{plan.md, task-mapping.md}`;
  per-slice outputs (`cycle-report.md` + verification reports) land under `slices/<slice>/` and what that
  does to the done-condition's artifact set; the **full-repository-suite regression net** (implement is the
  only slice-scoped consumer that runs the quality gates, so "the gates run the full repo suite" is its own
  operationalization of the contract's regression-safety rule, not a restatement); and the
  **feature-declared-not-verified-at-last-slice** surfacing (implement is the pipeline's terminal stage —
  only it reaches the last slice's G5, so the Feature-Done handoff is uniquely its responsibility).
- **Note:** the Graduation contract is on the ≥3-consumer queue (plan/tasks/implement slice-scoped
  variants) — this strip relocates implement's *local restatement* to the contract home; it does not rule
  the shared contract. **implement was the last restating consumer** (per the tasks-wave queue note:
  "plan + tasks are now locally de-restated, and implement.md's entry variant is the remaining restating
  consumer") — with this strip, all three consumers are locally de-restated; only the shared-contract
  ruling remains queued.
