# Strip notes — `templates/command-shape.md`

Entry formats: `strips/README.md`. Wave context: workflow-token-reduction wave 1 (design:
`.mochiko/brainstorms/workflow-token-reduction/record.md`, D6c; ratified 2026-07-23).
Shape v3 also *added* the Run-cost entry element (D2 — recorded in ROADMAP, not a strip).

---

# v0.33.0 — shape v4 → v5, the goal-shaped anatomy

**Wave context:** command goal-shape rebuild, **step 1 of 4** (design:
`.mochiko/brainstorms/command-succinctness-strip/record.md`, accepted 2026-07-30 — rulings
CS-D1–D10 + D2′; `DECISIONS.md` 2026-07-30). Step 1 lands shape v5 **and** the
`validation-command-shape` revision together, publishing the slot map and the floor arithmetic
below before any command is re-authored. **No command file was edited this step** — the six
remain v4-conformant by ruling.

**Re-audit coverage.** The shape's own rule is one edit here plus a re-audit of the conformant
commands. By CS-D10 that re-audit set is **the pilot (`plan`, step 2) plus the five-command wave
(step 4)** — not a step-1 sweep: the commands are deliberately left at v4 this step, so there is
nothing conformant-to-v5 to grade yet. The v5 revision itself is graded now under
`validation-command-shape`'s shape-revision checks (16–19). The step-4 ceremony carries its own
shape-home edit (the `loop-discipline` read-drop, if the pilot checkpoint approves it) whose
re-audit is that ceremony's audit of the five **plus a named delta re-audit of pilot `plan`**
(one file, one clause) — named here so it cannot be silently skipped (record, verify fold V1).
The pilot-checkpoint split adds one primitive to that same re-audit surface:
`templates/sized-end-stage-review.md` v1, whose two consumers (`brainstorm`, `setup`) pick up its
conditional read when they are re-authored — `setup` in the step-4 wave, `brainstorm` likewise.
Neither is the pilot, so **no v5 command exercises the conditional read until step 4**; the pilot
(`plan`) declares the in-loop branch and must *not* load it, which is itself the first live test
of check 1's negative direction.

**Additions this revision** — recorded for the decision row, not as strips (the v3 run-cost
precedent): the five-block anatomy specification and its conformance bar (D5 + folds a–c) · the
conditional-empty-block rule keyed on the binding, not the heading (fold c) · the P1–P16 slot
index · the KM-landing-as-end-state condition with the generic ritual homed here so commands
state only their own additions (D5's "KM landing" element) · the Seat transport section absorbed
from `agent-dispatch.md` (D6 — logged as an arrival below, since its departure is a strip on the
other side) · the v5 transition note retaining the obligated `loop-discipline` read (D7/D10 as
amended at verify V1). **Added at the audit's direction (v0.33.0 fix pass):** the countable
gate-line form for P7 (`- **<label>** — evidence: … · rules: … · decides: …`), which is what
makes the grader's `G` term deterministic · one clause in Layer 2 Independence stating that
**structural separation** is what independence-by-seat-assignment means · one clause stating that
a **respawn is cold by design** — both added because check 8 named them as homed markers while no
home carried the text, and a marker with no home cannot license relocating the line out of a
command.

## [v0.35.0] Ceremony polish — stale shared-read-floor figures and the six-run net table corrected

- **Disposition:** corrected in place (correction class)
- **Tier failed:** n/a — figure correction at the wave ceremony
- **Cause:** the **stale-summary failure mode** again — the shared-floor arithmetic was measured
  when `command-shape.md` stood at 16,405 B and never re-swept after the file's own later edits
  took it to **16,735 B**. Every figure derived from that base was 330 B optimistic.
- **Content:** `command-shape.md` 16,405 → **16,735 B**; shared reads 29,611 → **32,836 B**, so
  the per-run delta is **+3,225 B**, not +2,895 (live check: 16,735 + 4,175 dispatch + 11,926
  `loop-discipline` = 32,836). The two sibling scenario figures shift by the same 330:
  **+3,806** before the Run-cost drop (was +3,476) and **+5,717** before the split (was +5,387).
  **The +5,387 → +5,717 shift was not in the ceremony's named fix list** (which specified
  +2,895 and +3,476) — it is included because it derives off the same corrected base, and fixing
  its two siblings while leaving it would have manufactured a fresh inconsistency in the same
  sentence. Logged rather than folded in silently; approved at the ceremony.
- **The net table re-based to landed sizes** (second fix round, at the bounded confirm's
  direction). The first round corrected only the shared column, which left the table
  **mixed-basis** — a landed shared Δ over a floor-projected own-file Δ — and that is a worse
  defect than the stale figure it replaced, because the nets then described no real quantity at
  all. All six own-file deltas are now the shipped file against its pre-wave size, `wc -c`-measured:
  brainstorm **−1,095** · specify **−1,489** · slice **−3,176** · setup **−5,629** · implement
  **−10,542** · plan **−19,749**. Nets become **+5,122 / +1,736 / +49 / +588 / −7,317 / −16,524**,
  total **−16,346 B**. The `before the split` column is re-based on the same footing (own landed
  + 5,717, since pre-split there was no conditional add-on), totalling **−7,378**; leaving it
  projected would have re-created the mixed-basis defect one column over. plan's row now reads
  **−19,749**, which reconciles this table with `.mochiko/strips/plan.md`'s run-level figure —
  the two notes previously disagreed.
  *Superseded projection pair, recorded per the correction-class discipline:* nets
  **+2,664 / +1,149 / −748 / −2,313 / −10,642 / −17,681, total −27,571**, against a `before the
  split` total of −20,583. **Cause:** those rows are `today (B) − floor (B)` — a projection of the
  measured floor, computed before any command shipped — and the wave then landed every command
  *above* its floor row (deliberately, per CS-D8), so the projection overstated the saving on all
  six. It was never a run cost.
- **Two prose claims corrected, both false of the shipped surface:** `slice` does **not** flip
  net-better (**+49**, marginally worse), and **four** commands regress per run at landed sizes —
  brainstorm, specify, slice, setup — so it is **three besides brainstorm**, not one. Both claims
  were true of the projection and survived the first fix round unchecked, which is the same
  failure mode one level up: a *claim* keyed to a figure, not re-read when the figure moved.
- **Kept deliberately, each re-checked against the landed figures rather than assumed:** the
  split's 2,492 B/run relief and 500 B/run cost are independent of the basis and unmoved; the
  **8,968 B** six-run improvement survives exactly, since it is 4 × 2,492 saved less 2 × 500 paid;
  and the "brainstorm's net is 500 B worse un-split" comparison still holds precisely (+5,122
  against +4,622). The split's transition is stated as (**−7,378 → −16,346**) — the two re-based
  column totals — so the paragraph cannot contradict the table above it.
- **The three-component attribution retired** (final round, validator-prescribed). It read "the
  split itself 5,116 B · the Run-cost drop 3,486 B · the audit fix pass 366 B", with the caveat
  that "only the first is ruling B's; crediting all 8,968 B to the split would overstate it by
  75%". Under the corrected reading **all 8,968 B is ruling B's**: the split's effect *grew* from
  5,116 to 8,968 as the rest of the shape shrank, because the relocated block's share of the file
  rose from 1,850 to 2,492 B and each of the 642 B shaved off the shared floor is paid on all six
  runs (642 × 6 = 3,852; 5,116 + 3,852 = 8,968). The Run-cost drop and the fix pass are what made
  the split worth *more*, not separate credits competing with it — so the 75% overstatement caveat
  was itself the error, and it is deleted rather than re-scoped.
- **Also re-keyed here:** the floor table's **per-gate unit cost, 40–55 w → 69.2 w/gate** (37
  measured gate lines, 2,562 words). The `90·(G+2)` ceiling is untouched. The ceremony specified
  **74.6 w/gate over 39 gates**; that pair is recorded in the re-key paragraph as superseded,
  because it is the same measurement taken with the `evidence:`-only counting rule that this same
  ceremony's grader fix deletes.
- **The orphaned "down from 1,142 B" parenthetical deleted.** The first round left it as found,
  flagged as not reconstructible from this note. The bounded confirm resolved it: it derives from a
  **replaced intermediate that never shipped**, so it has no basis on any footing — projected or
  landed. Removed rather than re-derived, and the sentence reads correctly without it: brainstorm's
  net is 500 B worse un-split, which is the whole of the claim. A deletion on confirmed grounds,
  not a guess.

## [v0.34.0] The `loop-discipline` read-drop deferred to a named live-run trigger
- **Disposition:** superseded → the v5 transition note is rewritten. The read itself is
  **unchanged and still obligated**; what changed is the *status of its planned removal*:
  from "checkpoint-gated, lands at the wave ceremony" to "**deferred to a named trigger**".
- **Tier failed:** n/a — supersession by ruling (**pilot-checkpoint ruling**, user, 2026-07-30;
  ADR `.mochiko/decisions/2026-07-30-goal-shape-pilot-checkpoint.md`). Ground: the checkpoint found
  that **authoring-loop evidence cannot settle the question**. The pilot proved a goal-shaped file
  can *carry* the four requirements structurally; it cannot prove a lead *obeys* them mid-loop
  under rationalization pressure, which is the actual risk on record from the prior wave
  (`command-altitude/synthesis.md:47`). CS-D7's replacement guarantee is therefore not
  discharged by the pilot.
- **Content (the superseded note, verbatim):** "Its command-layer drop is checkpoint-gated, not
  pending: the pilot runs with the read retained, the pilot checkpoint rules the drop on that
  evidence, and — if approved — the drop lands as one shape-home edit inside the wave ceremony
  (`command-succinctness-strip` record, D7 + D10 steps 2–4)."
- **The named trigger, as encoded:** the first **live dogfooded run** of a rebuilt command in which
  the gates were not rationalized and the bounds held. Until that evidence exists, a command
  omitting the read is non-conformant, not early.
- **Kept deliberately:** the retention itself, its non-conformance consequence, and the
  transition note's presence in the always-read home — so the retention keeps reading as *ruled*
  rather than as an oversight, which was the note's original purpose.
- **Consequence for step 4:** the wave ceremony no longer carries a second shape-home edit. CS-D10
  step 4's "approved read-drop lands inside this same ceremony" is **spent** — with it, the named
  delta re-audit of pilot `plan` that existed only to cover that edit. The wave's re-audit surface
  is the five re-authored commands, nothing more.
- **Consumers assessed:** all six commands keep the read (five gain it in v5 form at this wave;
  `plan` already carries it). `loop-discipline` itself is untouched — this defers an edit to the
  *reference*, and never touched the skill.

## [v0.33.0] The Run-cost entry element dropped from the shape
- **Disposition:** superseded → **nothing** replaces it in the shape. Cost measurement leaves the
  command layer entirely; retirement is by this explicit entry, never by omission.
- **Tier failed:** n/a — supersession by ruling (**user ruling 2026-07-30**, step-1 fix round; ADR
  `.mochiko/decisions/2026-07-30-goal-shape-step1-adjudications.md`). This retires the element
  **shape v3** added (workflow-token-reduction wave 1, D2 — the manual-baseline cost carrier).
  Ruled **against** this author's recommendation, which was to slot it as P17 at the step-4
  ceremony so the parameter-completeness check could enforce it.
- **Content (v5 text at removal, verbatim):** "**Run-cost entry.** Every run ends with one
  recorded cost row — a **manual baseline**, because the platform exposes no session-readable
  token total. At finalize, ask the user for the visible usage figure (e.g. from `/usage`) and
  append one row to `run-costs.md` beside the deliverable (the feature directory for
  feature-scoped runs; the workflow's artifact directory otherwise): `| date | command | scope |
  seats | rounds/cycles | review sizing | usage (user-supplied, or "unavailable") | notes |` An
  unavailable figure is recorded as `unavailable` with the run-shape counts beside it — the row is
  never skipped, because it is what makes any efficiency claim checkable against a before/after."
  (v3–v4 wording differed only in the closing justification; the table row and the
  manual-baseline framing are identical.)
- **Kept deliberately:**
  - **Existing `run-costs.md` artifacts remain valid history** — nothing is deleted or migrated,
    and a project holding rows keeps them as a record of the runs that produced them.
  - **The token epic's OTel probe remains the future cost-measurement path** — the capability is
    deferred to instrumentation, not abandoned.
  - **This wave's own before/after measurement is unaffected**: it rides the strip notes and the
    floor arithmetic in this file, which never depended on `run-costs.md`. That independence is
    why the drop costs this wave nothing.
- **The evidence that made the drop available:** the element carried no `[PARAM]` and was bound in
  **0 of 6 commands** across its entire v3→v5 life (measured 2026-07-30 at v0.32.0: `grep` for
  `run-cost` / `/usage` across `commands/` = 0 hits). A shape-mandated behavior that no command
  ever bound, and that no floor check could catch because it was unslotted, was either dead or
  silently non-conformant surface-wide for two shape versions. Raised at the checkpoint as an open
  question; ruled here rather than carried further.
- **Reference sweep (this entry's scope).** `command-shape.md` retains **two** run-cost mentions,
  both correct to keep and both in the footer: the **v5 stamp's drop record** ("the Run-cost entry
  element dropped") and the **v3 history line** ("run-cost entry added") — the latter is accurate
  version history, which the grader's check 16 requires be preserved. (An earlier count in this
  note said "one"; that was a case-sensitive grep for `run-cost` missing the capitalised
  `Run-cost` in the v5 stamp. Corrected.) The **grader carries no run-cost reference** (verified:
  zero hits in `validation-command-shape/SKILL.md`), and none is added — the element is gone, so
  there is nothing to check. One reference survives deliberately outside this file:
  `skills/authoring-commands/SKILL.md` cites "the v3 run-cost precedent" for the logging
  convention that *pure additions go in the decision row, not the strip note*. That precedent is
  still factually true of what v3 did and still governs how additions are logged, so it is not
  stale and is not swept — now glossed there as **"since retired"** so a reader does not go
  hunting for a shape element that no longer exists.
- **Consumers assessed:** **zero commands** bound it, so no command changes and no re-audit surface
  is added by this drop — the only surface-wide consequence is that the always-read shape is
  smaller for every run. `authoring-commands` Job 1 step 3's parameter walkthrough never listed it
  (it was untagged) — unaffected. `loop-discipline` untouched.

## [v0.33.0] Sized end-stage review relocated → `templates/sized-end-stage-review.md` (conditional)
- **Disposition:** relocated → `plugins/mochiko/templates/sized-end-stage-review.md` **v1** (new
  primitive; arrival note: `.mochiko/strips/sized-end-stage-review.md`). A ~60-word pointer
  replaces the block in Layer 1; the text is unchanged in substance.
- **Tier failed:** **1 (altitude — wrong load class, not wrong content).** The block is correct
  shape doctrine that was being paid on *every* run of *every* command while binding in only
  two of six.
- **Ground:** raised by this revision's floor arithmetic (finding 3 below) as an **unruled**
  remedy and explicitly declined at authoring time; **ruled adopted at the pilot checkpoint
  (ruling B, user, 2026-07-30)** with the craft left to the author. Same revision, same version
  bump — no second shape version.
- **The skip path, named per D4** (the sham-cut test: a body→reference move earns credit only
  where the strip note names the invocation path that skips it): a command's **P6 validation
  model** decides the load. **Binds and loads it (2):** `brainstorm` (the sized review of
  `record.md`) · `setup` (the pre-G3 synthesis review). **Declares the bounded in-loop critique
  and never loads it (4):** `specify` · `slice` · `plan` · `implement` — each states "no sized
  end-stage review (the shape's in-loop-critique branch)" today, so the skip is already
  declared in the current v4 files, not a hoped-for property of the rewrite. The grader enforces
  the skip in both directions (check 1: a sized-review file must contain
  `sized-end-stage-review`; an in-loop file must **not** — loading it there is the sham read
  the split exists to prevent).
- **Content:** the whole sized-review element — the user-ruled sizing gate and its waiver, cold
  mutually-withheld reviewers, the frozen artifact, the one-shot cross-exam pointer,
  per-reviewer survivors and tallies with the merge reserved to the lead, survivor routing by
  answer-owner, one disposition per survivor, the verify pass, and review+verify as the bound.
- **Kept deliberately in Layer 1:** the conditional pointer and the statement that the in-loop
  branch satisfies producer↔validator on its own — the branch *choice* is shape, so a reader of
  the shape alone still learns both branches exist and which slot decides.
- **No slot moved.** All four slots the block cites (P5 · P7 · P12 · P13) are declared in the
  anatomy section, not in the block — the P1–P16 set, the slot map above, and the grader's
  parameter-completeness check are unchanged by this relocation.
- **D4's `templates/`-as-destination clause, addressed not ignored:** the sibling wave's D4
  names `templates/` a forbidden relocation destination *because templates are read at
  authoring time, i.e. always-read*. That rationale targets content moved out of a skill into a
  template; here the content was **already** in a template and the split's whole purpose is to
  make one always-read template partly conditional at **runtime**, with the skip path named
  above. The checkpoint ruling placed it "beside the shape" explicitly. Residual stated
  honestly: the file does sit under `templates/`, so D4's letter is set aside for this split by
  ruling, and the test D4 actually protects (a real, nameable skip path) is met.
- **Consumers assessed:** the two binding commands must gain the conditional read when
  re-authored (pilot + wave) — they are v4 files this step and currently get the text inline
  from the shape, so nothing is broken in the interim. `authoring-commands` Job 1 step 3 lists
  the sizing-gate keying among the parameters to fill and points at the shape, not at this
  block — unaffected. `loop-discipline` is untouched (no shared-skill edit this step).
  `review-brainstorm`/`review-governance-intent` own the reviewer-side protocol and are
  unaffected; `CROSS-EXAM.md` remains the cross-exam's home, now cited from the new file.

## [v0.33.0] The `Contract` section retired as a section — it became the document
- **Disposition:** superseded → redistributed into the anatomy: **Goal** (done-condition),
  the **Seats & checks** table (producer↔validator), **Constraints** (bounds + human gates).
- **Tier failed:** n/a — supersession by ruling (record **D5**: "The Contract section disappears
  as a section because it becomes the document — `loop-discipline`'s four requirements are the
  file's skeleton … not its appendix").
- **Content (v4, verbatim):** "**Contract section (authoring-time fill).** The command closes
  with a `Contract` section — its authoring-time fill of `loop-discipline`'s four requirements:
  **Done-condition** (initial state FAIL; the concrete not-done states named; user acceptance of
  the deliverable is part of it — plain blocking text, never a timed prompt), **Producer ↔
  validator** (different agents, different skills, structural separation), **Bounds** (every cap,
  and who counts it — the lead), **Human gates** (every named gate; a gate carrying Layer 2's
  devolved branch states the exact predicate that skips it)."
- **Kept deliberately:** all four requirements' content survives, relocated — initial-state FAIL
  + not-done states + user acceptance → **Goal** (P3/P4); different agents / different skills /
  structural separation → the **Seats & checks** table (P5) and Layer 2 Independence; every cap
  and its counter-owner → **P8**; every named gate → **P7**. The prohibition
  "**No per-run contract file is written** — a per-run form whose values are constant at
  authoring time is ritual, not proof (`workflow-contract.md` stays the form for loops whose
  values genuinely vary per run)" is kept verbatim, restated beside the inversion.

## [v0.33.0] Layer 1's `**Recovery.**` block redistributed into the anatomy
- **Disposition:** superseded → the anatomy's **Recovery** block spec (P15 pause location · P16
  resume rows), which carries the same doctrine as the block's own specification rather than as
  a separate Layer 1 element.
- **Tier failed:** n/a — supersession by ruling (**D5**: Recovery is one of the five blocks, so
  its doctrine belongs in the block spec; the same move the Contract section made, logged
  separately because it is a separate v4 element).
- **Content (v4, verbatim):** "**Recovery.** Sessions and teams do not survive `/resume`, and a
  shared account limit can throttle the team and the main session together — escalation then has
  nowhere to go but pause. Pause posture: note resume state on the deliverable [PARAM: where].
  Resume from **workspace evidence**, never a context `phase` field, respawning only what the
  stage needs [PARAM: the evidence → resume-at mapping]."
- **Kept deliberately:** every clause — the no-survival fact, the throttle case and its
  pause-has-nowhere-to-go consequence, workspace-evidence resumption, the never-a-`phase`-field
  prohibition, and respawn-only-what-the-stage-needs — all present in the anatomy's Recovery
  spec. Both v4 slots survive (:106 → P15, :108 → P16), per the slot map.
- **Default restored (audit fix C17):** v4's "note resume state **on the deliverable**" named a
  default location; the first v5 draft reduced P15 to "where resume state is noted" and lost it.
  Restored as "**on the deliverable** by default; a command names another location only when it
  has one" — not superseded, and not a deliberate drop. Without the default, five commands that
  currently note state on a report or a `Status` header would each be inventing a convention.

## [v0.33.0] D6 clause 1 — the condition-first pass over the home's own prose (audit fix C18)

D6 has two clauses. Clause 2 (absorb Seat transport) was encoded and logged on first delivery;
**clause 1 — "`command-shape.md` gets the same condition-first rewrite as the commands" — was
executed but neither logged nor reported**, so the footer's D6 claim outran its evidence. The
pass has now been run deliberately over the prose D6 targets, and this entry is its honest
accounting. **The anatomy section is excluded from the pass as its object, not its subject:** it
is the new specification D5 mandates, not narration carried over from v4.

**What the pass cut** (Layer 1 and Layer 2 carried prose, condition-first — a rule kept, the
narration around it dropped): the Seat-transport preamble's rationale and the agent-teams doc
quote (kept: the one-line condition that the substrate picks wrong, so the lead carries the
mechanics; the quote's home is `.mochiko/strips/agent-dispatch.md`) · the run-cost element's
"deeper transcript forensics" aside and its closing justification, re-expressed as the operative
rule — **moot: the whole element was dropped later in this same fix round by user ruling** (entry
above), so that ~15 w of pass credit is withdrawn from the arithmetic below and the element's full
removal is accounted as the supersession, not as this pass · the no-fallback bet's parenthetical
framing · the Hard-requirement paragraph's closing sentence "Running the loop on one-shot
subagents is not a fallback — it is the defect this layer forbids" (a restatement: **the rule
itself survives** in Layer 2's Seat transport bullet, "a spawn without a `name:` is a one-shot
subagent — in a team-form command, the forbidden transport", which is also the marker's home for
check 8) · Clearing's closing rationale
clause ("for a green deterministic run the classification always was the gate and the lead's
read was ceremony" — its ground is the v0.31.0 entry below, which carries the dogfood evidence)
· connective phrasing in One lead, As-you-go artifact, Seats-not-dispatches and the traffic
classes. No rule left the file in this pass.

**Honest arithmetic, like-for-like** (words, measured; v4 1,849 → v5 **2,508**):

| | v4 | v5 | note |
|---|---|---|---|
| header | 135 | 139 | + the slot-map pointer |
| **anatomy (new spec)** | — | **1,137** | D5's five-block spec + folds a–c + slot index + transition note. **~173 w of it is v4's Contract (112) + Recovery (61) re-expressed**, so **~964 w is genuinely new specification** |
| Layer 1 | 978 | 306 | **675 w departed**: sized review 390 → the conditional file (ruling B) · Contract 112 + Recovery 61 → the anatomy · **Run-cost 112 → dropped outright** (user ruling). Carried prose **303 → ~246 w (−19%)**, and v5's 306 *includes* a new ~60 w conditional pointer |
| Layer 2 + footer | 736 | 926 | Seat transport **arrived** (+150 w, D6 clause 2); the footer grew +15 w recording the drop. Of the carried prose: the pass cut ~9 w net (**−1.2%**), and **+34 w of new content was added at the audit's direction** to give two check-8 markers real homes (`structural separation`, `respawn is cold by design`) |
| **total carried prose** | **1,039** | **~973** | **−6.4%** |

**Disposition of the remaining growth, against D6.** +659 w, and **none of it is narration that
survived the pass**: **+964** genuinely new specification (the anatomy D5 requires) · **+150**
relocation in (Seat transport, D6 clause 2) · **+34** audit-directed marker homing · **+19**
header and footer · **−390** relocation out (the sized review, ruling B) · **−112** the Run-cost
element dropped by ruling · **−6** net from this pass. A condition-first rewrite of a document
whose job is now to *specify* an anatomy is larger than one that only carried doctrine; that is
the shape of the ruling, not an escape from it. Under D2's accounting the relocations in and out
both count as zero reduction, so the **only true reductions in this file are the Run-cost drop
(112 w, a ruling not a pass) and 6 words from the pass itself** — reported that starkly because
a reader is entitled to know the condition-first pass is not what shrank this file.

**Why clause 1 yields so little on Layer 2 (−1.2% before the marker homing), stated rather than
padded.** That prose has
already been through three strip passes (the v0.11.0–v0.17.0 relocations *into* this home, the
v0.22.0 header relocation, the v0.31.0 mesh re-carve) and is close to pure condition already:
its remaining sentences are the traffic classes, the devolved branch's three conditions and
their explicit negation, the cold-arrival property, and the transport mechanics. Cutting further
would take rules, not narration — which the sibling wave's **D1** forbids ("no line is cut while
its evidence stands. No quota-override strips"). Reporting −1.2% is the honest outcome of
applying clause 1 to a layer that had little left to give; manufacturing a bigger number here
would be the quota-override D1 names.

## [v0.33.0] The devolved-branch predicate restatement mandate narrowed
- **Disposition:** superseded → **P14** binds the clearing unit, any *workflow-specific*
  de-devolving condition, and the escalated branch's checkpoint keying; the shape's own three
  devolving conditions are referenced, never restated.
- **Tier failed:** n/a — supersession by ruling. Ground: **D5**'s Contract-as-document inversion
  plus **D4**'s aggression against restated doctrine. v4's clause obliged every command carrying
  the branch to restate a predicate the home states exactly — a restatement mandate inside a
  shape whose own rule is that untagged content "lives here and only here".
- **Content (the retired clause, verbatim):** "a gate carrying Layer 2's devolved branch states
  the exact predicate that skips it".
- **Kept deliberately:** the branch itself, its three conditions, and the input-never-the-gate
  guard are untouched in Layer 2 Clearing. What a command still owns is its own additions —
  `implement`'s surfaced-architecture-deviation and non-empty `domain_deps_added` genuinely
  *de*-devolve a cycle and are not derivable from the home, so they stay parameters.
- **Note for the pilot:** this is the one v5 change that *reduces* what a command must say
  about a shape element. `implement` is its only live consumer.

## [v0.33.0] Seat transport arrived from `agent-dispatch.md` (Layer 2)
- **Disposition:** relocated **in** ← `templates/agent-dispatch.md` (its departure entry:
  `.mochiko/strips/agent-dispatch.md`, same version). Command-layer-only mechanics now homed
  with the rest of the team transport.
- **Tier failed:** n/a — supersession by ruling (**D6**: "The **Seat transport** section moves
  out of `agent-dispatch.md` into shape Layer 2 (command-layer-only content currently sitting in
  a file every skill dispatch also references)"). Rationale on record: the split cost a
  cross-file reference hop on every run.
- **Content:** the spawn-with-`name:` mechanics, the forbidden-transport rule, the
  `SendMessage`-to-the-same-name rule, and the post-spawn addressability probe — carried across
  substantively verbatim.
- **Superseded on arrival (v4's pointer, verbatim):** "Spawn mechanics, the `name:`
  discriminator, and the post-spawn **addressability check** are single-sourced at
  `templates/agent-dispatch.md` (Seat transport) — a command names its probe seat and points
  there." **Kept deliberately:** the live half — a command names its probe seat — survives as
  **P2**.
- **Consumers assessed:** all six commands cite `agent-dispatch.md` (Seat transport) for
  transport and will re-point at shape Layer 2 when re-authored (pilot + wave; they are v4
  files this step, so the old pointer is still correct for them). `authoring-commands` and
  `validation-command-shape` reference transport without restating it — the grader's check 8
  entry for `the forbidden form` / `forbidden transport` has had its **home updated** to
  `command-shape.md` Layer 2 this revision. Skill dispatches that are *not* commands lose
  nothing: they never used the section.

## [v0.33.0] The sized-review branch declaration re-homed off the Contract
- **Disposition:** superseded → **P6** (a one-line validation-model declaration beneath the
  Seats & checks table).
- **Tier failed:** n/a — supersession by ruling (**D5**, consequential on the Contract section's
  retirement: the branch could no longer be declared in a section that no longer exists).
- **Content (the retired phrasing, verbatim):** "[PARAM: the reviewed artifact — or a
  Contract-section declaration that the loop's bounded, in-loop independent critique is the
  workflow's validation, satisfying the producer↔validator clause without a sized review]".
- **Kept deliberately:** both branches, unchanged in substance — the sized end-stage review and
  the bounded in-loop critique — and the rule that the in-loop branch satisfies
  producer↔validator on its own. Only the declaration's home moved.

## [v0.33.0] KEPT: the obligated `mochiko:loop-discipline` read
- **Tier-2 evidence:** the read is retained at v5 **by ruling**, not by inertia. CS-D7's
  preferred first move is to drop it at the command layer, but the drop is checkpoint-gated:
  the pilot runs *with* the read, the pilot checkpoint rules the drop on evidence that the
  goal-shaped structure alone held a bounded loop (gates un-rationalized, bounds honored), and
  an approved drop lands inside the step-4 ceremony (record D7 + D10 steps 2–4, as amended at
  verify V1). The standing risk it answers is on record from the prior wave
  (`.mochiko/brainstorms/command-altitude/synthesis.md:47`): a thin command bets
  `loop-discipline` reliably *fires and is obeyed* mid-loop under rationalization pressure. The
  shape states this as a transition note so the retention reads as ruled, not as an oversight,
  and so no command omits the read "early".

## [v0.33.0] Slot map — v4's 13 `[PARAM]` tags → v5's P1–P16 (CS-D10 step 1)

**No v4 slot is retired.** Thirteen v4 tags land in eleven v5 slots (two merges); five slots are
new, and one of those (**P9**) is the declared home for content v4 left unslotted. Retirement by
omission is forbidden (D2′'s not-done states) — every row below resolves.

| v4 tag (v4 line) | what it bound | → v5 slot | v5 block |
|---|---|---|---|
| :30 | the artifact, its path, its ID scheme | **P10** artifact set | Bindings |
| :31 | the uncertainty carrier | **P11** uncertainty carrier | Bindings |
| :42 | reviewed artifact / in-loop-critique declaration | **P6** validation model | Seats & checks |
| :47 | the sizing default keying | **P7** gate lines (the sizing gate's line carries it) | Constraints |
| :50 | reviewer agent × skill × lens briefs | **P5** seat rows (merged) | Seats & checks |
| :68 | the fact seat | **P12** fact route | Bindings |
| :73 | the verify-pass owner | **P13** verify-pass owner | Bindings |
| :106 | the pause location | **P15** pause location | Recovery |
| :108 | the evidence → resume-at mapping | **P16** resume rows | Recovery |
| :115 | which seat spawns first | **P2** probe seat | Preamble |
| :128 | the seat roster incl. peer edges | **P5** seat rows (merged) | Seats & checks |
| :160 | the clearing unit | **P14** clearing unit + checkpoint keying (merged) | Bindings |
| :166 | the checkpoint keying | **P14** clearing unit + checkpoint keying (merged) | Bindings |

The three slots the review pair found unclaimed by any block (M-I8: :31 / :47 / :160) are homed
above — Bindings, Constraints, and Bindings respectively.

**New at v5** (no v4 antecedent; each was untagged v4 Contract or unslotted command content):
**P1** goal line · **P3** end state · **P4** not-done states · **P8** bounds · **P9** invariants
and `KEPT:` survivors. **P9 closes the measured v4 gap** — the checker map's §3 finding that "no
`[PARAM]` slot covers the flow/phase machinery", i.e. 26.1–45.3% of each phase-bearing command
sat under no declared slot and was bounded only by the grader's judgment ceiling. Entry gates,
guards, prerequisites, ordering rules and out-of-scope statements now have a declared home and a
ceiling keyed to it.

**Previously unslotted, now RESOLVED by ruling:** the **Run-cost entry** carried no `[PARAM]` and
was bound in **0 of 6 commands** (grep for `run-cost` / `/usage` across `commands/` = 0 hits,
measured 2026-07-30 at v0.32.0). Raised here as an open question — a shape-mandated behavior no
floor check could catch, because unslotted — with a recommendation to slot it as **P17** at the
step-4 ceremony. **The user ruled the other way: the element is dropped from the shape entirely**
(supersession entry above; ADR `2026-07-30-goal-shape-step1-adjudications`). So the slot set stays
at **P1–P16** and no P17 exists. Nothing in v5 is now shape-mandated-but-unslotted.

## [v0.33.0] Per-command parameter-floor arithmetic (CS-D10 step 1) — floor EXCEEDS the projection

**Method.** The floor is the *minimum* goal-shaped size: every v5 slot bound + one constraint
line per gate carrying all three of (opening evidence · who rules · what it decides) + one seat
row per seat + one resume row per resume state + each artifact's path/ID binding, at **D5 fold
(a)'s preservation standard** (every routing decision and trigger survives; narration dies) and
including D8's extended protection set (`KEPT:` survivors *and* every line traceable to a
`DECISIONS.md` row). Two full skeletons were drafted and measured as the calibration
instrument — `brainstorm` (the lightest: 0 numbered gates, 0 resume rows) and `plan` (the
heaviest: 6 seats, 7 gates, 17 resume rows, 15 artifacts, slice-scoping, the architecture
stage) — and the unit costs fitted from them were applied to the other four by their measured
counts. Words are the denominator (correction **C2**: line counts on this surface are wrap
artifacts; bytes are derived at each command's own measured bytes/word). Frontmatter
`description:` is held at today's size — the sibling wave's trigger-fidelity rule protects it,
and it is 585 words across the six.

Fitted unit costs: preamble ≈ 100–108 w · Goal ≈ 85 + 2.5·artifacts · Seats & checks ≈ 98 +
29·seats · Constraints ≈ **69.2 w per gate** + 50 w bounds + the workflow's invariant set ·
Bindings ≈ 65 + 9·artifacts (+30 where a KM/index binding applies) · Recovery ≈ 45 + 11·rows.
The `plan` skeleton reproduces at 1,662 body words against a fitted 1,655 (0.4%).

**Per-gate cost re-keyed at the v0.35.0 ceremony:** the fitted **40–55 w** was contradicted by
the built surface — the six conformant commands carry **37 gate lines totalling 2,562 words,
i.e. 69.2 w/gate**, measured under the three-part `evidence:`/`rules:`/`decides:` rule. The
grader's `90·(G+2)` Constraints ceiling is **unchanged**; only this fitted unit cost moves.
*Superseded figure, recorded because it was specified for this ceremony:* **74.6 w/gate over 39
gates**, which is the same measurement taken with the `evidence:`-only counting rule that the
ceremony's grader fix deletes — the 39 counts `brainstorm`'s Invariants and `specify`'s
Enrichment bullets as gates. Keying the unit cost to it would have preserved the miscount inside
the correction that removes it.

| command | seats | gates | resume rows | artifacts | today (w) | **floor (w)** | Δ | today (B) | floor (B) |
|---|---|---|---|---|---|---|---|---|---|
| brainstorm | 2 | 4 | 0 | 2 | 1,376 | **872** *(measured)* | −36.6% | 9,547 | 5,994 |
| specify | 2 | 4 | 7 | 3 | 1,273 | **991** | −22.2% | 9,390 | 7,314 |
| slice | 2 | 4 | 8 | 3 | 1,611 | **1,076** | −33.2% | 11,968 | 7,995 |
| setup | 3 | 8 | 13 | 6 | 2,768 | **1,629** | −41.1% | 20,731 | 12,201 |
| implement | 3 | 5 | 12 | 4 | 3,230 | **1,354** | −58.1% | 23,873 | 10,006 |
| plan | 6 | 7 | 17 | 15 | 4,439 | **1,791** *(measured)* | −59.7% | 33,833 | 12,927 |
| **TOTAL** | | | | | **14,697** | **7,713** | **−47.5%** | **109,342** | **56,437** |

Gate counts are **gate lines** in the goal-shaped Constraints block (the countable form the shape
mandates for P7), not numbered-gate labels: `plan` is **G = 7** (G1–G7; its escalation gate *is*
G6, not an eighth), measured as 7 gate-form bullets in the drafted skeleton — which is the value
the grader's Constraints ceiling of 90·(7+2) = 810 is keyed to. `brainstorm` is **G = 4** with
zero *numbered* gates. Counts for the four un-drafted commands are fitted from their current
gate inventories, not measured.

**Verdict against the record's projection (~5–6k words surface-wide): the floor EXCEEDED it.**
7,713 w was **+28.6% over the 6k top of the band and +54.3% over the 5k bottom.** Per CS-D10
step 1 ("a floor exceeding the projection changes the anatomy or the ambition *before* the
pilot") this went to a **user checkpoint before the pilot** rather than an author's judgment.

> **RE-KEYED at the pilot checkpoint (ruling A, user, 2026-07-30).** The **ambition** moved,
> not the anatomy: the pilot's calibration target is now the **measured floor of 7,713 words
> (−47.5% surface-wide)**, and the record's ~5–6k estimate is **superseded** — it was an
> unmeasured projection made before the anatomy existed, and D2′ had already demoted the
> percentage from a bar to a reporting line. **Option C (change the anatomy) was explicitly not
> taken: D5 stands untouched.** The floor table above is the target, per-command; a re-authored
> command landing materially *under* its row is as much a finding as one landing over — under
> means content was dropped, and D8's protection set applies. Nothing here is pass/fail
> (D2′: conformance is the done-criterion, not a percentage).

Three findings the checkpoint needs:

1. **The anatomy has a fixed per-command cost of ≈ 440 words** — preamble ~100 + Goal base ~85 +
   Seats scaffolding ~98 + bounds ~50 + Bindings base ~65 + Recovery prose ~45 — before any
   per-workflow content. Across six commands that is ≈ 2,640 w, i.e. **44–53% of the entire
   5–6k projection is anatomy scaffolding**, and it does not shrink with workflow simplicity.
   That is why the *thin* commands overshoot worst (specify −22.2%, slice −33.2%, brainstorm
   −36.6%) while the *heavy* ones land near the projection's proportional share (plan 108% of
   share, implement 112%). The projection was surface-wide, so no per-command bar was ruled;
   proportional shares above are this note's construction, flagged as such.
2. **The graded exemplar's own compression is 11.4% in words, not 60%.** D5 fold (a)'s standard
   distils `plan.md:188–195` — 8 lines, **114 words**, ~15 discrete rules — into 3 constraint
   lines of **101 words** (chars 776 → 663, −14.6%). The line count falls 62.5%; the word count
   barely moves. At the ruled fidelity, restructuring the flow narrative is a *line* and
   *narration* win, not a word win. Any projection calibrated on line counts (the 1,308-line
   baseline that correction C2 already disqualified) will overstate the word reduction available.
3. **Run-level, the shared always-read floor grows — and the sized-review split was ruled in
   because of it.** Shape v5 grows the always-read floor even after the split:
   `command-shape.md` 12,502 → **16,735 B** and `agent-dispatch.md` 5,183 → 4,175 B (−1,008),
   so shared reads (with `loop-discipline`'s 11,926 B retained) go **29,611 → 32,836 B, +3,225 B
   on every run** — down from **+5,717 B** before the split and **+3,806 B** before the Run-cost
   drop. `sized-end-stage-review.md` (2,992 B) is then a **conditional** add-on paid only by the
   two commands that bind that branch. (Figures measured after the audit fix pass and the
   Run-cost supersession; the drop alone returned 581 B to every run of every command.)

   **All figures are landed sizes** (v0.35.0 re-base): each `own file Δ` is the shipped file
   against its pre-wave size, `wc -c`-measured on the six delivered commands. This is a measured
   run cost, not a projection.

   | command | own file Δ | shared Δ | conditional | **net / run** | before the split |
   |---|---|---|---|---|---|
   | brainstorm *(binds)* | −1,095 | +3,225 | +2,992 | **+5,122** | +4,622 |
   | specify | −1,489 | +3,225 | — | **+1,736** | +4,228 |
   | slice | −3,176 | +3,225 | — | **+49** | +2,541 |
   | setup *(binds)* | −5,629 | +3,225 | +2,992 | **+588** | +88 |
   | implement | −10,542 | +3,225 | — | **−7,317** | −4,825 |
   | plan | −19,749 | +3,225 | — | **−16,524** | −14,032 |
   | **six-run total** | | | | **−16,346** | −7,378 |

   The split moves **2,492 B/run off each of the four non-binding commands** and costs the two
   binding ones **500 B/run each** (the 2,992 B file against the 2,492 B they also save) — the
   irreducible overhead of any split: the pointer left behind plus the new file's own header.
   At landed sizes `slice` does **not** flip net-better — it lands at **+49**, marginally worse per
   run — and `specify`'s regression falls from +4,228 to **+1,736** without clearing zero. **Four
   commands still regress per run: brainstorm, specify, slice and setup** — i.e. **three besides
   brainstorm**, not one. (Both of those claims read the other way until the v0.35.0 re-base; they
   were true of the floor projection and false of the shipped surface.) Six-run total improves by
   8,968 B against the un-split baseline (**−7,378 → −16,346**) — a figure independent of the
   re-base, since the split's effect is 4 × 2,492 saved less 2 × 500 paid. The split's effect grew
   from 5,116 B to 8,968 B as the rest of the shape shrank — the relocated block's share rose from
   1,850 to 2,492 B, and each of the 642 B shaved (581 Run-cost + 61 fix pass) is paid six times.
   **brainstorm and setup pay for
   the other four,
   knowingly** — they bind the review and keep the load (checkpoint ruling B's own framing), and
   brainstorm's net is 500 B *worse* than it would have been un-split. That is the honest price and
   it is not hidden:
   brainstorm is the thinnest command and the least able to amortize either the anatomy's fixed
   cost (finding 1) or this split's overhead. Under D2's true-reductions-only rule, the genuine
   reductions in this whole revision are **the Run-cost drop (581 B, every run), agent-dispatch's
   137 deduped words, and 6 words from the condition-first pass** — plus whatever the commands
   themselves shed at the pilot and the wave. Every byte that merely moved between always-read
   homes counts as zero.

## [v0.33.0] Coordination marker — the deferred Layer-2 rewrite now starts from v5
- `standing-seat-lifecycle`'s **D3** (`.mochiko/brainstorms/standing-seat-lifecycle/record.md`)
  is still unbuilt and still targets Layer 2's "Seats, not dispatches" paragraph. The v0.31.0
  marker below said to start from **v4**; it now starts from **v5**, where that paragraph is
  unchanged in wording but Layer 2 additionally carries **Seat transport**, and the
  fresh-spawn anti-pattern line it wants to retarget at transport-only now sits in the *same
  layer* as the transport section it would be retargeted to. Nothing in CS-D1–D10 rules that
  reframe; it stays deferred.

---

## [v0.31.0] "Independence by structure" re-carved — routing no longer carries independence for in-loop seats
- **Ruling source:** `.mochiko/brainstorms/team-method-vs-command-shape/record.md` — **D1** (the in-loop mesh becomes Layer 2's default) and **D2** (mesh scope = in-loop traffic only; cold end-stage review restated as a property of the review *stage*). Shape v3 → v4.
- **Disposition:** superseded → re-carved in place (Layer 2: "Independence by structure", plus the new "In-loop mesh — the default"). Two halves of the old sentence survive, restated: cold seats are still never in the room before their stage (now framed as a property of the stage, not of the traffic), and independence is still never carried by a persona's say-so.
- **Tier failed:** n/a — a ruled doctrine reversal, not a minimalism strip. The claim was falsified by dogfood evidence: in the kinako MVP-H1 run the qa seat was independent *behaviorally* — re-ran every `**TEST:**` task, checked premises — while never routing through the lead (30 peer-to-peer messages vs 2 lead messages).
- **Content (the removed clause, verbatim):** "…and producer↔validator traffic routes through the lead — who talks to whom is the independence guarantee, carried by the roster…"
- **Consumers assessed:** all seven commands read this run — `implement` · `plan` · `tasks` · `specify` · `slice` · `setup` restate lead-routed in-loop producer↔validator traffic and are in the handed-off re-audit set; `brainstorm` has no conflicting clause (its only teammate-to-teammate surface is the cold convergence review, which D2 preserves). Keeper skills `authoring-commands` and `validation-command-shape` reference the shape without restating routing — unaffected. **`loop-discipline`** (the shape's Governed-by skill) — two lines assessed: **SKILL.md:72** (the hard round cap, "a deterministic ceiling counted by the supervisor") is **unaffected** — every round that consumes a cap follows a failure, and under the Clearing block failures return to the lead, so no round can open unseen (this holds because the peer-routable retry clause was removed at audit fix 1; it would not have held with it). **SKILL.md:56** ("The lead/referee owns the verdict") is **narrowly qualified** by D3 and **needs a companion revision** — its paragraph's core (never let the producer self-grade; a different agent, a different skill, graded from the artifact) is untouched and D3 preserves it, but read literally the sentence forbids the devolved clean branch. Raised, **not ruled here**: `loop-discipline` is a ≥3-consumer shared primitive, so per `authoring-commands` Job 3 step 3 it escalates to a scheduled all-consumer pass rather than being edited in this wave.
- **Coordination — the next Layer-2 rewrite:** a marker so the deferred work finds this revision. `standing-seat-lifecycle`'s **D3** (`.mochiko/brainstorms/standing-seat-lifecycle/record.md`) is a still-unbuilt Layer-2 doctrine reframe targeting the **same surface at v4+**: it retires "continuity is what a standing seat buys" as written and re-splits Layer 2 into *team transport* (roster, messaging, independence-by-structure) versus *per-seat context lifecycle* (standing / stage-scoped / per-round recycled), retargeting the fresh-spawn anti-pattern line at transport only. **Sequencing decision (wave lead, 2026-07-30):** deliberately **not** combined into the D1–D3 mesh revision. Whoever builds it starts from **v4**, not v3 — the paragraph it rewrites ("Seats, not dispatches") now also carries the seat roster's peer-edge parameter, and "Independence by structure" is the re-carved text above, not the sentence its record quotes.

## [v0.22.0] HTML comment header relocated (runtime-loaded provenance)
- **Disposition:** relocated → here (D6c). The live kernel stayed in the visible body: the obligated-read consumption rule, the [PARAM] tagging meaning, the `<!-- shape-exception: why -->` marker, the two-layer conformance statement.
- **Tier failed:** pure waste (map §5): Read-tool template loads do not strip HTML comments — the 1,373 B header cost context on every run of every command.
- **Content (the relocated provenance, faithfully compressed):**
  - **Design provenance:** the shape /mochiko:brainstorm and /mochiko:setup were built in; design `.mochiko/brainstorms/pattern-codification-and-minimalism/record.md` — D1 codifies the artifact shape, D3/D8 make this template the surface, fold S2 makes it the SOLE home. Commands and the keeper skills (authoring-commands, validation-command-shape) reference it; a shape revision is one edit here plus a re-audit of the conformant commands (the D1 churn constraint, carried structurally).
  - **Layering provenance:** two layers deliberately (fold S2 closed D2's layering thread) — Layer 1 form-agnostic core, Layer 2 team transport.
  - **S8 checkpoint history:** the home-revision checkpoint ran at the first one-shot→team-form conversion (specify, 2026-07-19) → shape v2: the artifact's uncertainty carrier became a [PARAM] (lead-penned records carry confidence marks; producer-authored artifacts carry their own assumption/open-question surface), the sized end-stage review's applicability was scoped (an in-loop independent critique satisfies validation via the Contract clause), and the ground rules (kernel-free · no git mutations) were homed here from the command footers.
