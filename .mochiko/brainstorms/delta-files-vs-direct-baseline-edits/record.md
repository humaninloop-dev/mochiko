# Delta files vs direct baseline edits — decision record

**Status:** **accepted 2026-09-24** (user: "accept") — review round 1 folded 2026-09-24 (17/17 survivors dispositioned, user-ruled:
C1 and C2 one by one, the five coverage survivors ruled inline, the I/M batch "as recommended");
verify round 1 NOT CLEAN (1 blocking + 13 nits) → V1 re-put and user-ruled "as recommended"
(the checkpoint table stands in for a missing sign-off commit), the 13 nits lead-repaired;
delta-check CLEAN on blocking, 3 nits (d1 count scope · d2 the groom read-around's git form · d3
the constraints bullet's annotation set) lead-repaired, closed seat-unverified, disclosed;
reviewer's final status `ready` · **Landed:**
`DECISIONS.md` row 2026-09-24 (the five prior rows and the field review's row annotated
superseded in part) · `BACKLOG.md` build item under *Hook-enforced artifact schema build*, riding
the field review's waves (the 2026-09-23 audit item's gap 3 annotated dissolved) · `ROADMAP.md`
Template-schema CLI Next row touched (cap held at 7) · both indexes updated (this entry; the six
prior entries annotated) · **Opened:** 2026-09-23 · **Lead:** session lead (inline questioning via
`mochiko:analysis-iterative`) · **Review:** solo cold review — `mochiko:devils-advocate` seat on
`mochiko:review-brainstorm`, blind two-message dispatch (message 1: topic + goal line only; the
index and the session directory fenced; `record_contact: none` attested) · **Transport
(patterns-transport-floor):** one reviewer seat, lead-relayed messaging, no cross-seat mesh; single
writer per file — the record is the lead's, `reports/angle-map.md` and `reports/review.md` the
reviewer's

## Topic

Driver ask (user, 2026-09-23): "pull context from the reports mochiko generated during the run,
specially the feature level architecture.md or delta files that is supposed to be additive to
product level docs. I want to understand why are we not changing the product level files directly
and creating these delta files. Since we already have git and git commit and diff, these delta files
create additional complexity in my view."

**Goal line:** decide whether the product baselines (`.mochiko/product/` — `data-model.md`,
`constraints-and-decisions.md`, `contracts/`, `quickstart.md`, `design/`, and the architecture
store) keep taking per-feature delta files plus a graded landing fold, or are edited directly under
git — and, if directly, what of the delta model's stated purposes survives and how — leaving one
hardened decision record.

**Prior-session relations.** The delta model was ruled in five sessions and one floor rule:
`architecture-design-primitive` D3 (2026-07-30) · `feature-sizing-and-entry-points` D9/D15
(2026-08-10) · `multi-feature-plan-implement` D2/D10 (2026-08-14) · `product-architecture-schema`
D3/D10/D12 (2026-08-19) · `impeccable-design-integration` D6/D7 (2026-09-19; migrations `0013`,
`0018` — the design baseline's landing write from shipped code) · `impl.baselines-never-in-place`
(floor, `0001-genesis.yaml`) with its companion store floor
`authoring-architecture-store.sign-off-is-write-gate`. The sibling `hook-enforcement-field-review`
session was **accepted and landed 2026-09-23** (its `DECISIONS.md` row, its four-wave `BACKLOG.md`
build item) while this record was frozen for review; it rules per-entry budgets for the same
stores (its D2, amended at its review to leave `contracts/*` out — no gate bound applies to them),
and its OQ1 census table now explicitly rules "the fold shape (dated fold block appended vs
in-place `[EXTEND]`)" and lists `baseline-delta.md` as an entry-class candidate — both answered
here (D1, D5); this record supersedes those two clauses in part (D6a).

## Ground facts (F)

*Sources: the five session records; `plugins/mochiko/migrations/0001-genesis.yaml`,
`0005-artifact-homes.yaml`, `0013`, `0018`; the kinako tree at HEAD `b149eea` (`.mochiko/product/`,
`.mochiko/features/FEAT-001|002|006/`, `.mochiko/epics/`, `reports/`, `archive/`); kinako git log;
kinako memory `feat-001-run-4-state.md` and `user-commits-during-runs.md`; this repo's
`BACKLOG.md` audit item (2026-09-23) and the field-review record and review. Reviewer
re-verification: F1–F11 confirmed against source; corrections folded where noted.*

- **F1 — The delta model was ruled four times for the baselines, each with a reason; the design
  baseline's path a fifth time (F13).**
  (a) `architecture-design-primitive` D3: plan's architecture artifact is a *delta view* — current
  state plus proposed target, sign-off approves the target, landing folds the built reality in.
  Reason given: "editing `ARCHITECTURE.md` directly would inject unbuilt proposals into a doc whose
  contract is present-tense/current-state-only, and an abandoned plan would leave a lie in the
  living view."
  (b) `feature-sizing-and-entry-points` D9 (user-shaped): two altitudes — product baselines say what
  the product HAS, the feature directory says what the feature CHANGES; acceptance folds deltas into
  baselines in the landing that flips map status. D15: every fold is graded by a three-way diff
  (pre-fold baseline + delta vs folded result), so deltas must be in appliable before/after form.
  Reason: "the fold is the highest-blast-radius write in the pipeline — a corrupted baseline poisons
  every later extend-mode run silently."
  (c) `product-architecture-schema` D3/D10: one store whose elements carry lifecycle statuses
  (`ruled` → `in-flight (FEAT-XXX)` → `built`); a delta is drafted in the package with the store
  untouched, the user's sign-off is the write gate, the signed delta lands *into the store* as
  in-flight elements, landing flips `built`. Reason: "sign-off-as-write-gate keeps the store
  ruled-truth-only"; marked `Assumed` — "designed live, never walked against a real feature."
  (d) `multi-feature-plan-implement` D2/D10: an epic keeps a spine plus per-feature deltas; a
  shared-baseline delta is authored once under a single pen-holder
  (`impl.epic-shared-baseline-single-pen`, `authoring-epic.shared-baseline-single-pen-holder`).
- **F2 — The floor rule, its one carve, and the carve's timing.** `impl.baselines-never-in-place`
  (class floor): "Baselines are never edited in place. They change only through the landing's graded
  fold. The design phase writes deltas beside them; a build-time technical decision takes the same
  path. One carve, and only one: a store write at the design checkpoint's user sign-off is legal, and
  only as in-flight-class delta elements." Its companion `authoring-architecture-store.sign-off-is-write-gate`
  (floor, user-gate): "a design-time write is legal only as an in-flight-class delta, and only after
  the user's sign-off on the rendered diagram plus the named row changes: the sign-off IS the write
  gate. No sign-off, no store write." Enforced by `impl.fail.baseline-in-place` and
  `impl.fail.ungraded-fold`; `impl.graded-fold` states the three-way diff;
  `impl.baseline-delta-grammar` binds the build-time path. So the store takes an in-place write
  with a status marker **at sign-off, never before**; every other baseline takes the delta-and-fold
  path. *(Corrected at review, C1: the first draft read the carve as "already the in-place model";
  the carve is at sign-off only.)*
- **F3 — Two delta species live in a feature home.** (a) Design-phase deltas —
  `architecture.md` (the C4 delta diagram + delta register), `data-model.md`,
  `constraints-and-decisions.md`, `contracts/*.md` — in `[EXTEND]`/`[MODIFY]`/`[NEW]` form, each
  package minting its own id sequence from `001`. (b) The build-time ledger `baseline-delta.md`
  (`BD-XXX` entries) raised by the builder seat, header "Appliable, never applied here"; each entry
  carries *Would amend* · *Status* · *Why it is a delta at all* · *Weighed*. The ledger is a
  declared deliverable of the `feature` and `product-lane` homes (`0005-artifact-homes.yaml`
  :166, :325); the `epic` home declares the shared-baseline copies, not the ledger (verify N6).
- **F4 — The design-phase deltas are overwritten per run and already lean on git for history.**
  kinako FEAT-001 `data-model.md` header: "Overwritten 2026-09-22 — this file's prior content (R1-4)
  was folded into the product baseline at the 2026-09-22 landing; that text persists at git HEAD
  `741585d`." `constraints-and-decisions.md` and `architecture.md` carry the same note. The delta
  file is not the history; git is. *(The current content of those two files is the run-4 package,
  unfolded — F5, C2.)*
- **F5 — The fold is a hand-authored patch file, graded, then applied by the user — or not at
  all.** Run 3: `reports/landing-fold-texts-2026-09-22.md` — 823 lines of YAML, five fold groups, 13
  `file:line` anchors each with `before:`/`after:` text and an `id_renumbering_map`. Run 4:
  `reports/landing-fold-text-run4-2026-09-23.md` — 619 lines (§A1–A4 baseline folds, §B records
  corrections, §C map entry, §D staged status lines). The run-4 package FAILed its grade round 1 on
  14 findings — F-01 id collision with the product sequence and no renumbering, F-02 two contract
  amendments silently dropped, F-06 wrong line arithmetic, F-07 an amendment's before/after not
  carried, F-10/F-11 stale line cites — then PASSed at revision 1. Application: run 3 by the user's
  own shell (`APPLY.sh`, `APPLY-2.sh`; commits `ec0b17f` +364/−19 and `76179e1`); run 4 closed DONE
  with the folds "owed by the principal's pen" (kinako memory `feat-001-run-4-state.md`; commit
  `5167f36` writes the store, the map entry and the operating docs, not `data-model.md` or
  `constraints-and-decisions.md`). **Run 4's fold never landed:** product
  `constraints-and-decisions.md` stops at `C-008` / `D-049`; product `data-model.md`,
  `contracts/ipc.md` and `constraints-and-decisions.md` hold zero R1-9/R1-10/R1-11 lines; no
  product commit after `5167f36` touches them — yet the feature-map entry landed at `5167f36`
  already cites the fold text's ids ("`D-061` corrected Amendments 3 and 6's bodies before the
  fold"). This is gap 2 of the six in the 2026-09-23 `BACKLOG.md` audit item.
- **F6 — Id renumbering is a cost the fold creates.** Package-local `C-001…C-004` → product
  `C-005…C-008`, `D-001…D-005` → `D-045…D-049`, `IP-001` → `IP-012`, `DS-001` → `DS-012`, every
  cross-reference rewritten, citations already naming a product id left alone (run 3 fold header at
  product `constraints-and-decisions.md:547-559`). In `baseline-delta.md`, parallel builders in two
  features minted the same `BD-` ids; FEAT-001's file now opens with a renumbering ledger
  (`BD-240→BD-261` … `BD-251→BD-272`), a standing rule ("a cross-epic id collision is qualified by
  feature and epic, never renumbered"), and a product-source citation (`corpus_driver.rs:1920`) that
  still names the old numbers. BD ids are cited from 108 kinako code files (`crates/` + `src/`, three-digit ids; 111 with
  `tools/` — verify N7, delta-check d1) and on 76 lines of the product baselines; 63 `.mochiko` files name the
  ledger path.
- **F7 — The product baselines are not folded in place; they accrete fold blocks.**
  `constraints-and-decisions.md` (733 lines): `## The EPIC-002 landing fold (2026-09-12)` at :371
  and `## The FEAT-001 landing fold (2026-09-22)` at :547, both *after* `## Where this baseline is
  deliberately silent`. `data-model.md` (1,206): the same two blocks at :907 and :1080 after
  `## Divergences found between the package and the code`. A `BoundSession [EXTEND]` lands as a new
  section ~430 lines below the entity it extends. "What the product HAS" now reads as the base
  sections plus every fold block. The field reviewer saw the same (review S6(b): "Observed folds
  append a dated `## The FEAT-001 landing fold` block (127–187 lines) … a fold shape the record
  leaves open"). The baselines also carry structures that are not entries: `data-model.md` opens
  "Eighteen entities are delivered. Seventeen are modelled here." (:32), an Entity summary table
  (:30), a Relationships table (:729), `## Validation rules, as delivered` (:759);
  `contracts/ipc.md` holds dated count-reconciliation sections ("the closed set is **46**", :427).
- **F8 — `baseline-delta.md` has never been folded and accumulates across runs.** FEAT-001 2,314
  lines / 62 entries (`BD-001`…`BD-272` with gaps) · FEAT-002 4,744 / 144 · FEAT-006 1,010 / 32 —
  8,068 lines and 238 entries of "appliable, never applied" decisions, their `Status` lines free
  text (`raised at C1-3, build time` · `routed to the lead` · `built` · `surfaced, not repaired`).
  The 2026-09-22 groom of FEAT-001's file could move only a 22-line method-notes block: "nothing in
  it has yet been folded into a ruled baseline, and the ledger itself is the sole surviving record
  until that fold (a separate, later dispatch) lands." The groom's line shift broke frozen
  citations (`cycle-C1-6-verification.md:337`) and its `BD-220` fold-check found the entity had in
  fact been edited in place at an earlier landing *(the last two claims not re-read by the
  reviewer)*.
- **F9 — The write-time gate compounds the fold problem, and the sibling session answered the
  size half.** Field review F4: every product baseline stands 2–4× over the `product` home's
  300-line whole-file bound, a fold only adds lines, so no seat can land one. Its D2 (accepted):
  cumulative stores take a per-entry budget and no whole-file bound; `contracts/*` are out of its
  scope (no gate bound applies to them today); the guarantee is conditional on its OQ1 census
  admitting the largest honest entry (177 lines observed). An in-place amendment to an entry
  already over its budget still worsens it and denies — D1 hits the same wall the fold hit until
  that census lands.
- **F10 — Git and worktrees are already in the loop; runs commit on `main` mid-run.**
  `impl.cold-verification` snapshots ride `.claude/worktrees/`; run 3 ran in
  `.claude/worktrees/mochiko-run3`; the user commits during runs from their own client
  ("`688627f checkpoint` swept an in-progress cycle's code in under a one-word message") and per run
  authorizes the lead to commit at checkpoints (kinako memory `user-commits-during-runs.md`);
  `impl.no-git-mutations` ("Suggest commits; never run git mutations, never push") lacks the per-run
  authorization carve both runs used (audit gap 5). A plain working-tree `git diff` therefore shows
  only the uncommitted tail of a run's change. Fold packages already anchor on `file:line` in the
  main checkout and cite worktree copies as sources. `git merge` flags overlapping hunks only: two
  branches each adding `### D-050` under different headings of one file merge clean (`merge featB
  exit=0`, both entries kept — reviewer's reproduction, re-run by the lead 2026-09-24).
- **F11 — The store's sign-off write worked in the same runs; the baselines' fields differ from
  the store's.** The design checkpoint signed the architecture delta and the store took in-flight
  elements; run 4's landing commit `5167f36` wrote `concerns.md` (+28) and `spine.md` (+18) directly
  and re-derived `ARCHITECTURE.md`. kinako `concerns.md` carries `in-flight (FEAT-001)` /
  `in-flight (FEAT-007)` rows today; the store's fold duty flips them to `built` and **clears the
  `FEAT-XXX` key** ("none on ruled/built", spine template check), and its `lifecycle-statuses` rule
  keeps status "the build-lifecycle axis only — stance is a separate axis". The product
  `constraints-and-decisions.md` entries carry a `**Status:**` field that holds the *stance* axis
  (`new` · `recommended` · `delivered`) and the fields `Source` · `Shaped by` · `Impact` (16
  occurrences); `data-model.md` carries no `Status` field and none of those three; `contracts/`
  carries one ruling stamp (`Status: RULED, 2026-09-03`, `plugin-bridge.md:53`). Outside implement
  runs, kinako fixes B126/B129 (`341bbb8`, `2004abe`) edited product `contracts/engine-port.md` and
  `contracts/ipc.md` in place together with code, under no marker — the never-in-place floor binds
  implement runs only.
- **F12 — What the delta files carry that a plain diff does not.** (a) the *why* per amendment
  (BD entries' *Why it is a delta at all* / *Weighed*; the design deltas' rationale lines); (b) the
  sign-off surface for a structural change — the container delta diagram and the changed-element
  table the principal signs (`architecture.md` §§ "Container delta diagram", "The checkpoint
  surface"); (c) the anchor — which baseline line each change amends; (d) the graded three-way
  diff at landing. Under git the natural carriers are: (a) the entry's own text and the cycle
  report; (b) the diagram, still drawn, plus the diff against a pinned base; (c) the diff hunk;
  (d) a diff review by a non-author seat.
- **F13 — The design baseline is a landing write from shipped code, not a design-phase delta.**
  `impl.design-landing` (0013): the designer "derives the system part from shipped code, never from
  intent" at a UX-bearing landing, and is "the design baseline's only writer, on exactly three
  paths: the setup leg (the truth part, the brownfield system-part seed, the scaffold), a
  build-time baseline-delta.md entry, and this fold"; `impl.design-first-write` (floor,
  user-gate): "the fold's first system-part write is the user's: at final acceptance the proposed
  write and every critique-advisory finding are presented for the user's signature before the
  baseline takes them"; the blind gap-finder's fence admits the design baseline as "as-built
  system". *(Added at review, I6; quotes verbatim per verify N8.)*

## Constraints carried in

- GI-004 / primitive-edits ceremony: any change lands through strips + audits + the migration log;
  `impl.baselines-never-in-place` and `authoring-architecture-store.sign-off-is-write-gate` are
  floors and leave only by recorded supersession.
- GI-019 bright line: no gate may hold judgment or sequence a run — whatever replaces the fold stays
  procedural or mechanical-only.
- The accepted field-review session's D2 and OQ1 govern the size of every in-place write D1 makes
  (`contracts/*` excepted); this record's amendments to that record are supersessions in part,
  landed as annotations on its `DECISIONS.md` row, its `BACKLOG.md` build item and its
  `.mochiko/brainstorms/index.md` entry (D6a; delta-check d3).

## Decisions (D)

*(recorded as ruled — statement · rationale · confidence; review folds marked)*

### D1 — Product baselines are edited in place; the change is the diff against a pinned base; delta files and the landing fold go

**Statement:** A seat that needs a product baseline to say something new — the analyst or architect
in the design phase, the builder at a build-time technical decision — writes it into the product
file itself (`.mochiko/product/data-model.md`, `constraints-and-decisions.md`, `contracts/*`,
`quickstart.md`, the truth part of `design/`, and the architecture store), on the run's working
tree, with a lifecycle marker on the entry (grammar: D2; timing against the checkpoint: D7). The
change is what git shows against the run's pinned base (D3c) — committed and uncommitted alike. No
per-feature delta copy of a baseline is authored (`data-model.md`, `constraints-and-decisions.md`,
`contracts/*.md` in the feature home), no `baseline-delta.md` ledger, no fold package, no
three-way transcription grade. The feature home keeps the run's own artifacts — plan, tasks,
reports, and the architecture delta's diagram surface (D4). The judgment review of the change
stays: a non-author seat reads the diff (D3c). **Carve (I6):** the design baseline's *system part*
stays as `impeccable-design-integration` ruled it — written from shipped code at the landing, the
first write the user's (`impl.design-first-write`); that landing write is already an in-place
graded write and is compatible with this decision; only its `baseline-delta.md` route dies.
**Scope (I5):** the rule binds implement runs (feature, epic, and product-lane); an out-of-run fix
that ships doc and code in one commit (kinako B126/B129) writes `built` directly, no marker.

**Rationale (restated at review, M2):** F4 — the delta files are overwritten each run and point at
git for their own history, so git is already the record. The fold's *residual* costs, the ones no
better execution removes: a transcription step that must be graded (F5's 14 findings were about
the copy, not the design), two copies of every entry, and the renumbering at every fold (F6). F5/F8
— in practice the fold was applied by the user's shell or not at all, and 8,068 lines of build-time
decisions were never folded. F12 — everything a delta file carried has a carrier under git. **What
D1 does not cure:** the gate's size denies (F9) — D1 depends on the field review's D2 per-entry
budgets and OQ1 census to make an in-place write to an over-budget store legal at all. Rejected:
**B** (keeps the fold for the design deltas — the transcription step stays) · **C** (better fold
hygiene keeps the transcription step, the renumbering, and the two copies of every entry; its
other legs — the budget deny, the fold shape, the undispatched ledger fold — are execution
defects the field review already addresses).

**Confidence:** Confident (user-ruled "yes as recommended" at Q1; carve and scope user-ruled at
review, I5/I6 inline). Supersedes in part, recorded at build: `feature-sizing-and-entry-points`
D9's fold clause and D15 · `architecture-design-primitive` D3's fold-at-landing clause ·
`product-architecture-schema` D10's "drafted in the plan package (store untouched)" step ·
`multi-feature-plan-implement` D2/D10's per-feature delta storage · `impeccable-design-integration`
D6's `baseline-delta.md` path · the `impl.baselines-never-in-place` floor and its fail rules · the
`authoring-architecture-store.sign-off-is-write-gate` floor (D7).

### D2 — Every baseline entry carries a lifecycle marker in its own field, keyed by the run's owner

**Statement:** An entry written or amended during a run carries a lifecycle field of its own —
`**Lifecycle:** proposed (<key>)` before the checkpoint, `in-flight (<key>)` after sign-off (D7),
`built` at landing — separate from any stance field an entry already has (kinako
`constraints-and-decisions.md`'s `**Status:** new · recommended · delivered` is the stance axis and
stays; I4). The key is the run's owner: `FEAT-XXX` for a feature run, `EPIC-XXX` for an epic's
joint write, `lane-<slug>` for a product-lane run (I5). The built form is the store's: at landing
the key is cleared and the entry reads `built` — one form across the store and every baseline (I4);
the commit carries the date; an entry with no `Lifecycle:` field reads `built` (verify N5). An
amendment to a ruled entry marks the entry `modifying (<key>)`,
a removal `removing (<key>)`; the prior text is not kept beside the new — the pinned-base diff
(D3c) shows it (M1; OQ2 closed). The landing sweep is mechanical: every `in-flight` / `modifying`
/ `removing` for the landed key flips to `built`, and the sweep reads the pinned-base diff, not
markers alone, so counts, summary rows, relationship tables and validation-rule lists a run
touched are swept with it (I3); an abandoned run's changes are found the same way and reverted.
The store's orphan rule extends to every baseline: a lifecycle key naming no open feature, epic or
lane is flagged by the sufficiency check at run-open and by the landing verifier for the run's own
key (M6). Which fields `data-model.md` and `contracts/*` entries gain (`Lifecycle:`, and for a
build-raised entry `Raised:` / `Weighed:`) is declared at the field review's census table (I4).

**Rationale:** F10 — runs commit on `main` mid-run and the lead commits at checkpoints, so the
branch cannot tell a reader of `data-model.md` what is proposed from what is built. F11 — the
store's vocabulary is the ready one, but its axis is lifecycle only and its built form clears the
key; putting lifecycle words into the constraints file's stance field would make `recommended`
and `in-flight` indistinguishable. Rejected: **B** no marker (fails at the first mid-run commit)
· **C** a per-feature manifest of touched entries (a side file that must agree with the baseline
— the shape D1 just removed) · prior text kept beside new (the two-copies cost D1 rejected road C
for; the diff carries it).

**Confidence:** Confident (user-ruled "as recommended" at Q2; the field split, key set, built
form and diff-read sweep user-ruled at review — I3/I4/I5/M1/M6 as recommended).

### D3 — Ids from the product sequence at write · the why on the entry · reviews read the diff against a pinned base

**Statement:** **(a) Ids.** A new entry takes the next free id of the product file's own sequence
at write time (the analyst reads the high-water `D-049`, writes `D-050`); no feature-local
sequence from `001`, no renumbering map at landing. Two runs in flight on separate worktrees may
mint the same id, and `git merge` will not catch it in non-overlapping hunks (F10); the catch is a
**duplicate-id check** — mechanical, a pass over every id in each baseline touched — run by the
non-author seat in the landing diff review (c), and the second run to land renumbers its own
entries (I2). Alternatives dealt (M4): block reservation per run (a registry to keep) ·
feature-qualified provisional ids flipped at landing (renumbering by another name). **(b) The
why.** An entry's rationale lives on the entry: the constraints file's existing `Source` · `Shaped
by` · `Impact`, and on every baseline a build-raised entry's `Raised: <cycle>` and one-line
`Weighed:` (fields declared at the census, D2); the cycle report discloses the decision as it does
today. The commit message is **not** a required carrier (I1) — the run cannot rely on writing it.
No ledger. Alternative dealt (M4): a ledger-lite rationale file per run (a side file again).
**(c) Review.** No new seat. The run pins its **base commit** at run-open and again at the design
checkpoint's sign-off, recorded in the run's log and on the checkpoint card (I1). The sign-off pin
is the commit the checkpoint card asks the user to make — suggested, never run by the lead
(`impl.no-git-mutations` stands); where no commit is made, the card records the flipped entry
set, and the built-vs-signed diff and `impl.deviation-gate` read the checkpoint table (D4) as the
signed state (verify V1, user-ruled "as recommended"). Every baseline
review reads `git diff <base> -- .mochiko/product/` — the committed and uncommitted change since
the base, whoever committed it. In the design phase the existing feasibility and plan-artifact
reviewers read that diff as part of the package they already grade; at build time the landing
verification seat grades the judgment content of build-raised entries, as
`impl.baseline-delta-grammar` already has it; at landing the store's built-vs-approved check
(`authoring-architecture-store` landing diff) stays, `impl.deviation-gate` and the built-vs-signed
diff read against the sign-off base, and the three-way transcription check dies — there is no
transcription to check. An **unmarked write** — the new fail condition — is a hunk in that diff
outside an entry marked for the run's key; excluded from that test are hunks already in the tree
at run-open (recorded as pre-existing on the run-open pin) and hunks inside an entry marked for
another open key, which belong to that run (verify N1).

**Rationale:** F6 — every renumbering cost in the runs came from feature-local sequences meeting
the product sequence at fold; writing into the product sequence removes the meeting, and the one
residual (parallel worktrees) takes a grep, not a ledger. F12 — the prose the ledger carried has a
home on the entry. F5 — the graded fold's transcription half is what the reviewers spent their
findings on; its judgment half already belongs to seats that exist. F10 — without a pinned base a
mid-run commit hides an edit from every review, the silent corruption F1(b) named.

**Confidence:** Confident (user-ruled "as recommended" at Q3, the three put together with each
item's recommendation named; the pinned base, the duplicate-id check and the dropped commit-message
carrier user-ruled at review — I1 inline, I2 and M4 as recommended).

### D4 — `architecture.md` stays as the drawing the principal signs; the feature home loses its baseline copies

**Statement:** The feature home keeps `architecture.md` scoped to the sign-off drawing: the
C4-container delta diagram, the sequence diagrams for qualifying flows, the deployment view when
`IP-XXX` rows exist, and the checkpoint table of changed elements. It carries no delta-register
text and no current-state narrative — the register is the pinned-base diff of `spine.md` /
`concerns.md`, which the architect writes in place as `proposed` elements before the checkpoint
and which the sign-off flips to `in-flight` (D7; the store's own carve was at sign-off only — F2,
corrected at review, C1). Overwritten per run, as today. The `feature` home's declared set becomes
`tasks.md` · `plan.md` · `requirements.md` · `design-closure.md` · `sufficiency-report.md` ·
`architecture.md` · `proposal.md` · `contest-brief.md` · `gates.md` plus `reports/`;
`baseline-delta.md`, `data-model.md`, `constraints-and-decisions.md` and the `feature-contracts`
home (`contracts/`) are withdrawn from it. The `epic` home's shared-baseline copies
(`data-model.md`, `constraints-and-decisions.md`, `quickstart.md`, `contracts/`) and the
product-lane home's `baseline-delta.md` go the same way (build detail against
`0005-artifact-homes.yaml`; I5).

**Rationale:** The diagram is a per-run drawing a person reads once at sign-off, not a store row;
F12(b). `patterns-system-design`'s register duplicates what the diff on the store now shows, so
it goes with the deltas. Rejected: **B** diagrams inside the store (`spine.md` grows a mermaid
block per run and must be pruned at every landing) · **C** diagrams as a report (a new envelope
type whose 15-line section budgets fight a diagram).

**Confidence:** Confident (user-ruled "yes A" at Q4).

### D5 — kinako clean-up at the plugin upgrade: apply run 4's fold first, freeze the ledgers, re-home the fold blocks, re-point every link

**Statement:** At the kinako pass that rides the plugin upgrade, in this order: **(i) Run 4's
graded fold text is applied in place first** (C2) — `reports/landing-fold-text-run4-2026-09-23.md`
§A (PASS at revision 1), each entry into the section it extends, the product high-water becoming
`C-011` / `D-061` so the ids the map entry and the fold text already cite resolve; graded by a
non-author seat reading the diff. **(ii)** The three `baseline-delta.md` files (FEAT-001 · FEAT-002
· FEAT-006, 8,068 lines) move to `.mochiko/archive/` unchanged — history and the archive keep
them, nothing is folded from them. **(iii)** The feature-dir baseline copies (`data-model.md`,
`constraints-and-decisions.md`, `contracts/`) are deleted: FEAT-001's because their content is
folded (run 3 by its headers; run 4 by step i), FEAT-002's and FEAT-006's on the evidence of their
epic landings (EPIC-002 `77ea3fb`; EPIC-001 `ac21ce8`). **(iv)** The two accreted fold blocks in
product `constraints-and-decisions.md` (:371, :547) and `data-model.md` (:907, :1080) are moved
entry by entry into the sections they extend and the `## The … landing fold` headings go — a
mechanical move graded by the same non-author diff read (every entry present once, text
unchanged, nothing else moved). **(v)** Every link to a moved or deleted path is re-pointed
mechanically in the same pass (63 `.mochiko` files name the ledger); no stub is left at an old
name — a stub in the feature home would be an undeclared write under the closed world; `BD-` ids
stay valid as citations into the archived ledger, whose header keeps the cross-epic qualification
rule (I9). The epic directories (`EPIC-001`, `EPIC-002`) are closed record
(`authoring-epic.close-semantics`): untouched, exempt from the census (I9). A ledger entry the
baselines never absorbed is booked only when a later run trips on it — the lazy path, not a
sweep. Steps (i)–(v) land outside any run and add no `Lifecycle:` marker; their entries read
`built` (verify N5).

**Rationale:** F5 — run 4's design exists nowhere but the feature copies and the fold text; deleting
the copies before applying the fold would leave the baselines permanently short of entities the
code and the map cite, and D3a's next id would collide with `D-050…D-061`. F8 — the ledgers'
`Status` lines are free text, so applying them is judgment over 238 entries with no uniform
"already reflected" mark; the runs have built against the baselines without them since
2026-09-03 *(not measurable from the tree — reviewer)*. F7 — the fold blocks are the current-truth
entries and only need to sit where a reader looks for them. F6 — the citation count makes a
pointer sweep the only honest move. Rejected: **B** a one-time fold pass (days of seat time on
statuses nobody normalised) · **C** leave as is (the baselines stay two-shaped for every later
reader; the ledgers rot in place) · archive the run-4 copies unapplied (C2's road B) · void run
4's ids and rewrite the landed map entry (C2's road C).

**Confidence:** Confident (user-ruled "as recommended" at Q5; step (i) user-ruled "as
recommended" at review, C2; the pointer sweep and the epic exemption user-ruled inline, I9).
Consumer-side work, booked in kinako's `BACKLOG.md`; rides the same pass as the field review's
D9(b) evidence clean-up.

### D6 — Wrap-up: the accepted field-review record superseded in part · epics one pen per file · no governance amend · build rides the field review's waves

**Statement:** **(a) The accepted `hook-enforcement-field-review` session (I8).** This record
supersedes two of its clauses in part, landed as annotations on its `DECISIONS.md` row, its
four-wave `BACKLOG.md` build item and its `.mochiko/brainstorms/index.md` entry at this record's
landing (both indexes, per the KM landing step 3 — verify N12): its OQ1 "fold shape" question is
answered — there is no fold, entries are written in place (D1) — and its `baseline-delta.md`
entry-class candidate dissolves (the file no longer exists). What stands: its D2 per-entry
budgets and the census table, which now also carry the `Lifecycle:` field, the per-baseline
fields of D2/D3b, and the feature/epic/product-lane home sets of D4; `contracts/*` stay outside
its size scope (no gate bound applies to them), so an in-place contract edit is size-ungated —
stated, not changed. Its reviewer's S6(b) stands against this record too — an in-place amendment
to an entry already over its budget denies — carried as OQ1 here and ruled at that table. **(b)
Epics.** A product baseline touched by two or more members is edited in place under one pen at a
time per file, the transport floor's single-writer leg as today, keyed `EPIC-XXX`; the `epic` home
drops its baseline copies and keeps `architecture.md` as the one signed drawing for the whole epic.
**(c) No governance amendment** (checked true by the reviewer). No principle in the `CLAUDE.md`
governance region names the fold; the floor rules live in the migration log and leave by recorded
supersession (GI-005/GI-006 satisfied by this record and the strips). The field review's own
wave-3 ledger amendment is unaffected. **(d) Build.** Rides the field review's waves under the
primitive-edits ceremony. **Its migration wave** takes: the `impl.baselines-never-in-place` floor
superseded by a new-id floor stating the in-place-with-marker rule; `authoring-architecture-store.sign-off-is-write-gate`
superseded by the D7 form; `impl.fail.baseline-in-place` → an unmarked baseline write (D3c);
`impl.fail.ungraded-fold` → an unreviewed baseline diff; `impl.graded-fold` → the pinned-base diff
review; `impl.baseline-delta-grammar` → the entry grammar with marker; `impl.landing-verifier-folds`;
`impl.landing-lane` ("the graded folds"); `impl.design-outputs-home` ("Design outputs land at
FEAT-XXX/ as deltas beside their baselines"); `impl.epic-shared-baseline-single-pen`;
`impl.verification-design-time-grades`; `impl.reports-envelope` and `feat.product-surface` ("delta
files overwrite only via the graded fold"); `feat.author-grader` and `feat.delta-cards` (name
`baseline-delta.md`); `patterns-adopt-first.baseline-delta-landing`;
`authoring-feature-map.map-side-altitude` ("the appliable before/after form");
`review-sufficiency.clause-in-flight` ("reading that feature's deltas" → the entries marked
`in-flight` for that feature, never `proposed`); `testing-gap-finding.blindness-fence-inclusion-list`
(floor; "the feature's design-phase deltas"); `authoring-epic.member-deltas-stay-per-feature` and
`.shared-baseline-single-pen-holder`; `authoring-architecture-store.fold-duty` → the landing flip
over every baseline; `.lifecycle-statuses` gaining `proposed` and the wider key set ("each
in-flight-class element MUST name the `FEAT-XXX`" → the run's key); `.orphan-rule` (keys beyond
`FEAT-XXX`, readers per D2); the spine template's status legend and check ("a FEAT-XXX key on
every in-flight/modifying/removing element and none on ruled/built" — verify N3); `impl.design-landing`,
`impl.design-first-write` and the `0013`/`0018` design-baseline template lines that name
`baseline-delta.md` (the landing write itself stands, I6); `review-plan-artifacts.store-delta-checklists`'
register rows; and the three home sets in `0005` (I7). **Build step before the migration op:** a
full-text sweep of the log for "delta", "fold", "appliable", "in place", "in-flight" and
"FEAT-XXX", and the gate audit diffs D6d's list against it — this list is `Assumed` complete
until that sweep (M4, N3). **Its prose
wave** takes the skills (`authoring-technical-requirements` · `authoring-architecture-store` ·
`patterns-system-design` · `patterns-entity-modeling` · `patterns-api-contracts` ·
`patterns-adopt-first` · `authoring-epic` · `review-plan-artifacts` checklists ·
`review-feasibility` · `review-sufficiency` · `testing-gap-finding` · the design-direction skills
where they name the ledger · the `mochiko` router lines) and the entry templates; **its kinako
wave** carries D5. The five prior `DECISIONS.md` rows (2026-07-30 · 2026-08-10 feature-sizing ·
2026-08-14 multi-feature · 2026-08-19 product-architecture store · 2026-09-19 Impeccable) are
annotated superseded in part (M3). **Audit gap 3 (M5)** — the mid-run spine groom — dissolves
into one line: a groom of a baseline during a run is deferred past the landing, or, by the user's
ruling, lands as its own commit and the baseline reviews read around it (`git diff
<base>..<groom>^` for the change before it and `git diff <groom>` — the groom commit against the
working tree, uncommitted changes included — for the change after it; a plain `git diff <base>`
would include the groom; verify N2, delta-check d2); a groom carries no lifecycle marker. **Audit gap 5** (the per-run commit carve) is not ruled here; D3c's sign-off pin depends
on it only softly — the checkpoint table stands in where no commit is made (V1) — and D3b no
longer depends on it. Untouched, because their "delta" or "fold" is the map row or the desk card:
`impl.landing-delta` · `impl.delta-reverification` · `impl.gates-fold` ·
`authoring-feature-map.delivered-sticky-rows-fold` · `patterns-sound-loop.no-delta-card-exemption`
· `review-sufficiency.clause9-delta-inapplicable`. Alternative dealt (M4): a separate build
sequence for this record (collides with the field review's edits to the same homes and
templates).

**Rationale:** (a) the two sessions rule the same files from two sides — size and shape — and one
census table is where both numbers land; the sibling closed while this record was frozen, so the
amendment lands as an annotation, the KM module's shape for a supersession of an accepted record;
(b) the single-pen rule was about shared *deltas*, and the shared *file* is the same object under
D1; (c) the admission scope is unchanged, no gate gains judgment; (d) the field review already
sequences a crate → migration → prose → kinako pass over the same homes and templates, and a
second sequence over the same files would collide.

**Confidence:** Confident on (a)–(c) (user-ruled "yes" at Q6; (a) restated at review, I8, as
recommended); **Assumed** on (d)'s completeness until the build-step sweep (M4/I7).

### D7 — Write timing: `proposed` before the checkpoint, `in-flight` from sign-off — the sign-off flips the word

**Statement:** A design-phase write to any baseline or to the store before the design checkpoint
carries `Lifecycle: proposed (<key>)`. The user's sign-off at the checkpoint — on the rendered
diagram, the changed-element table (D4) and the pinned-base diff of the baselines (D3c) — flips
every `proposed (<key>)` of that run to `in-flight (<key>)`; the flip is a mechanical
transcription — through `mochiko:authoring-architecture-store` for the store, which regenerates
the derived `ARCHITECTURE.md`, and directly on the other baselines — made as the checkpoint's
closing act and read by the landing audit (verify N4); the sign-off base is pinned there — the
user's commit, or the checkpoint table where none is made (D3c, V1). Every reader treats
`proposed` as nothing promised: the sufficiency check of another feature cites `in-flight` entries
as planned contract and `proposed` ones as absent; the blind gap-finder's fence admits `in-flight`
and `built`, never `proposed`; a run that ends before its checkpoint reverts its `proposed`
entries by the pinned-base diff. A build-raised entry (a build-time technical decision) is written
`in-flight (<key>)` directly — it is raised after the checkpoint and graded by the landing
verification seat (D3c). `authoring-architecture-store.sign-off-is-write-gate` ("No sign-off, no
store write") is superseded by this ruling: the sign-off is the gate between `proposed` and
`in-flight`, not between nothing and a write.

**Rationale:** C1 — D1 puts writes before the checkpoint so the review pair can read them, while
the store's floor allowed a write only at sign-off; without a pre-sign word an unsigned draft and a
signed element read the same to every other seat, and a second feature would be graded sufficient
against a design the user then reshaped. Rejected: **B** write only at sign-off, drafts held
elsewhere for the review pair (a delta-shaped draft file again) · **C** write at drafting as
`in-flight` and make the checkpoint a read gate (signed and unsigned indistinguishable to every
reader).

**Confidence:** Confident (user-ruled "as recommended" on C1, 2026-09-24 — a coverage survivor
put as three options and ruled one by one; one bounded verify round, no fresh cold read, per the
reopen-born bound).

## Review (cold, round 1 — `reports/review.md`; blind map `reports/angle-map.md`, 47 angles / 30 load-bearing / 8 classes)

**Reviewer:** `mochiko:devils-advocate` seat on `mochiko:review-brainstorm`, solo; two-message
dispatch, index fence held, `record_contact: none` attested before the record path was sent.
**Verdict:** `critical-gaps` — 25 raised, 17 survived: 2 Critical (C1 write timing · C2 run 4's
unfolded design) · 9 Important (I1–I9) · 6 Minor (M1–M6); coverage survivors C1, I1, I5, I6, I9,
M5. **Cross-examination:** eight questions answered from the record alone, five "not in the
record" (Q-b base, Q-c merge reviewer, Q-d run 4's owed folds, Q-e non-entry structures, Q-f lane
key); Q-h (the sibling's state) was answered wrongly by the record (I8).
**Verification honesty (reviewer):** F1–F11 confirmed against source; wrong: D5's header premise
(C2), D3a's merge claim (I2), D2's vocabulary claim (I4), D3b's fields claim (I4), the
feature-sizing date (M3), the sibling's state (I8); not verified: F8's citation-shift and `BD-220`
claims, F5's "PASSed at revision 1", D5's "built without them since 2026-09-03". **Lead's own
re-checks (2026-09-24):** C1 — `sign-off-is-write-gate` text confirmed verbatim; C2 — product
`constraints-and-decisions.md` high-water `D-049`, zero R1-9/10/11 lines in the three product
files, the map entry's `D-061` citation confirmed; I2 — the four-step merge reproduction re-run in
the session scratchpad (`merge featB exit=0`, `D-050` at lines 7 and 17); I7 — the eleven rule
ids resolve in the log (`impl.reports-envelope`, `feat.author-grader`,
`testing-gap-finding.blindness-fence-inclusion-list` twice each, the rest once) and the twelfth
site is the `0018` template; M3 — `DECISIONS.md` row 81 and the record's status line both say
2026-08-10; I9 — 63 `.mochiko` files name the ledger, 108 code files (`crates/` + `src/`, three-digit ids)
and 76 product-baseline lines cite `BD-` ids, both epic dirs carry the baseline copies. **Withdrawn by the reviewer:** G5 git-as-dependency, B6/B7 hybrids, E5, D6c,
the sound-loop leg placement, crash/resume.

**Dispositions (user-ruled 2026-09-24: "one by one for critical and rest as recommended"):**

| Finding | Lands on | Disposition |
|---|---|---|
| **C1** Critical · coverage | D1, D3c, D4, F2, F11 | **User-ruled A** on a three-option put: pre-sign status `proposed`, sign-off flips to `in-flight`, store floor superseded → **D7**; F2/F11/D4 corrected |
| **C2** Critical | D5, F4, F5 | **User-ruled A**: apply run 4's fold text in place before the copies go; high-water `C-011`/`D-061`; premise corrected; epic landings cited for FEAT-002/006 → D5(i)/(iii) |
| I1 Important · coverage | D1, D3, F10 | Ruled inline as recommended: base commit pinned at run-open and sign-off; reviews read `git diff <base>`; unmarked write = hunk outside a marked entry; commit message not a required carrier → D3b/D3c, F10 |
| I2 Important | D3 | As recommended: duplicate-id check by the non-author seat in the landing diff review; alternatives recorded; reproduction re-run by the lead → D3a, F10 |
| I3 Important | D2 | As recommended: the sweep reads the pinned-base diff, not markers alone; non-entry structures named → D2, F7 |
| I4 Important | D2, D3, F11 | As recommended: `Lifecycle:` in its own field, stance `Status:` untouched; built form `built` with key cleared; per-baseline fields declared at the census → D2, D3b, F11 |
| I5 Important · coverage | D2, D4, D6 | Ruled inline as recommended: keys `FEAT-XXX` / `EPIC-XXX` / `lane-<slug>`; orphan rule over all keys; ledger withdrawn from the product-lane home; out-of-run fixes write `built` directly → D1 scope, D2, D4 |
| I6 Important · coverage | D1, D6, relations | Ruled inline as recommended: `design/` system part carved out, landing write from shipped code stands, first-write floor stands; Impeccable ruling added to relations; `0018`, `impl.design-landing`, `impl.design-first-write` added to D6d → D1, F13, D6d |
| I7 Important | D6 | As recommended: twelve sites added; full-text sweep as a build step; list `Assumed` until the sweep → D6d |
| I8 Important | D6, relations | As recommended: sibling restated as accepted; supersession in part by annotation; home sets and "no fold" carried into its census; contracts scope stated → relations, F9, D6a |
| I9 Important · coverage | D5 | Ruled inline as recommended: links re-pointed mechanically, no stubs; BD ids valid into the archive; epic dirs closed record, exempt → D5(v), F6 |
| M1 Minor | D2, OQ2 | As recommended: "prior text beside new" struck; OQ2 closed — marker only, the diff shows prior text → D2 |
| M2 Minor | D1 | As recommended: rationale restated on residual costs; dependence on field-review D2/OQ1 recorded → D1 |
| M3 Minor | relations, D6 | As recommended: 2026-08-10 → relations, D6d |
| M4 Minor | D3, D6 | As recommended: alternatives recorded per sub-item; D6d `Assumed` on completeness → D3, D6 |
| M5 Minor · coverage | D1, D6 | Ruled inline as recommended: gap 3 dissolves — groom deferred past landing or in its own commit, no marker → D6d |
| M6 Minor | D2 | As recommended: orphan readers named — sufficiency check at run-open, landing verifier for the run's own key → D2 |

**Verify round 1 (`reports/review.md` § verify_round_1):** NOT CLEAN — all 17 folds landed where
the table says, in the form ruled; 1 blocking + 13 nits. **V1** (blocking; D3c, D7, D6d — from
the reviewer's own I1 resolution): the sign-off pin is a commit no rule guarantees, so the
built-vs-signed diff and the deviation gate could read an unsigned state → re-put to the user
as three options (the user's commit or the checkpoint table, recommended · rule the per-run
commit carve here · pin by content only) → **user-ruled A** ("as recommended"), folded into D3c,
D7, D6d. **N1–N13** (nits) lead-repaired the same round: N1 unmarked-write exclusions (D3c) · N2
grooms read around, not excluded (D6d) · N3 three more rule sites + two sweep terms (D6d, build
surface) · N4 the flip through the store skill, index regenerated (D7) · N5 a missing field reads
`built`; D5's writes carry no marker (D2, D5) · N6 the epic home never declared the ledger (F3) ·
N7 the code-file count's scope (F6, § Review) · N8 F13's quotes verbatim · N9 F1 "four times for
the baselines, a fifth for the design baseline" · N10 D7's confidence line · N11 the Q1 answer's
original words restored · N12 the field review's index entry added to D6a and build item 2 · N13
the eleven ids + the `0018` template, and Q-d for Q-h in the cross-exam summary. **Delta-check
(`reports/review.md` § delta_check):** CLEAN on blocking — V1 and N1–N13 all landed as stated,
N8's quotes checked word for word; 3 nits lead-repaired the same round, closed seat-unverified,
disclosed: d1 the `tools/` count is 111 for three-digit ids (F6) and § Review's count carries its
qualifier · d2 the groom read-around's second leg is `git diff <groom>` against the working tree,
since `<groom>..` would omit uncommitted changes (D6d) · d3 the constraints bullet names the
brainstorms-index entry too. Reviewer's final status recommendation: `ready`.

## Build surface (cold-buildable — rides the field review's waves, D6d)

1. Migration (with the field review's wave 2): rule supersessions and rewords as D6(d) lists them
   (the two floors by new id; the fail rules re-keyed to the unmarked write and the unreviewed
   diff; `lifecycle-statuses` gaining `proposed`) · the three home sets in `0005` per D4/D6(b) ·
   the entry grammar per baseline with the `Lifecycle:` field and the build-raised fields (one
   table with the per-entry budgets, user-ratified) · the log full-text sweep ("delta" · "fold" ·
   "appliable" · "in place" · "in-flight" · "FEAT-XXX") as the step before the op, audited
   against the list.
2. Prose (with the field review's wave 3): the skills D6(d) lists · templates · strips · audits ·
   `plugin.json` bump · five `DECISIONS.md` annotations · the field review's row, build item and
   brainstorms-index entry annotated (D6a).
3. Kinako (with the field review's wave 4): D5 in order — run 4's fold applied (i), ledgers to
   `archive/` (ii), copies deleted (iii), fold blocks re-homed (iv), links re-pointed (v); each
   write graded by a non-author diff read.

## Open questions

- **OQ1 — The per-entry budget must admit the largest honest entry (D6a; field review S6(b)).**
  kinako's largest entity entry is 177 lines; an in-place amendment to an entry over its budget
  denies under the gate's non-worsening rule. Ruled at the field review's census table, which now
  also declares the `Lifecycle:` field and the per-baseline fields of D2/D3b.
- **OQ2 — closed at review (M1):** an amendment carries the marker only; the pinned-base diff shows
  the prior text.

## Question trail (Q)

- **Q1** — the shape of the replacement: edit in place on the run's working tree with a status
  marker on the entry (A, recommended) · keep design-phase deltas as the sign-off surface, drop the
  build-time ledger (B) · keep the model, fix the fold mechanics (C). → **A** ("yes as
  recommended") → D1. User then asked why a diff still needs to happen given the commit —
  answered: the diff is not authored, it is how a reviewer sees the edit; the old three-way check
  graded a hand transcription that no longer exists; the commit is the user's acceptance, so the
  review reads the working-tree diff before it and `git show` after it. *(Review I1 sharpened
  this: the diff is read against a pinned base, since the user commits mid-run.)*
- **Q2** — the in-flight marker: the store's lifecycle words on every baseline entry (A,
  recommended) · no marker, the branch is the state (B) · a per-feature manifest of touched
  entries (C). → **A** ("as recommended") → D2.
- **Q3** — three mechanics as one question: ids from the product sequence at write · the why on
  the entry · no new reviewer, existing graders read the diff. → **all three** ("as recommended")
  → D3.
- **Q4** — the drawing: keep `architecture.md` scoped to the diagrams and checkpoint table, feature
  home loses its baseline copies (A, recommended) · diagrams into the store (B) · diagrams as a
  report (C). → **A** ("yes A") → D4.
- **Q5** — kinako clean-up: freeze the ledgers, re-home the fold blocks, no entry pass (A,
  recommended) · one-time fold pass over 238 entries (B) · leave as is (C). → **A** ("as
  recommended") → D5.
- **Q6** — wrap-up batch: field-review session amended · epics one pen per file · no governance
  amend · build rides the field review's waves, each with its recommendation. → **"yes"** → D6.
- **Review dispositions (2026-09-24)** — the user ruled "one by one for critical and rest as
  recommended". **C1** put as three options (pre-sign `proposed` status, recommended · write only
  at sign-off · write as `in-flight` with a read-only checkpoint) → **A** ("as recommended") →
  D7. **C2** put as three options (apply run 4's fold first, recommended · archive the copies
  unapplied · void run 4's ids) → **A** ("as recommended") → D5(i). The five coverage survivors
  (I1, I5, I6, I9, M5) took the "rule inline" path with the recommended ruling; the I/M batch
  landed as recommended.
- **Verify V1 (2026-09-24)** — the sign-off pin: the user's commit at the checkpoint, or the
  checkpoint table where none is made (A, recommended) · rule the per-run commit carve here (B) ·
  pin by content only (C). → **A** ("as recommended") → D3c, D7, D6d.
