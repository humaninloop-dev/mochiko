# Build vs Off-the-Shelf as a Plan-Time Minimalism Discipline

**Status:** accepted · **Opened:** 2026-08-15 · **Accepted:** 2026-08-15 (post round-2 CLEAN
verify) · **Shape:** iterative analysis
**Driver:** kinako FEAT-006 (corpus stewardship) — the pipeline hand-rolled a file storage engine
without ever seriously weighing SQLite; the user caught it after the plan (FEAT-006 paused after
cycle 6 of 18), and the `kinako-sqlite-transition` retrospect maps the transition but deliberately
declines to autopsy the miss. This session writes the miss diagnosis and rules where the
build-vs-off-the-shelf discipline lives in mochiko.

## Problem Statement

No seat in the mochiko pipeline ever asked "is there a proven component that already does this?"
for kinako FEAT-006's storage layer. Six cycles built a lockfile, a staged-replace commit pointer,
an in-flight marker, and atomic-write machinery that `BEGIN`/`COMMIT` supplies for free. The human
was the safety net — the user identified the issue after the plan, not any seat, gate, or ladder.
The user's framing: build-vs-get-off-the-shelf should be a principal-architect / tech-lead-grade
decision and part of the minimalism architecture.

## Ground Facts

Evidence gathered 2026-08-15 from a fresh clone of `humaninloop-dev/kinako` (depth-50, scratchpad)
and the mochiko working tree. Verbatim quotes verified by two independent readers; kinako line
numbers refer to that clone.

- **F1 — What was built.** FEAT-006 cycles 1–5 built a hand-rolled file-based storage engine —
  directory-per-transcript JSON records, single-writer lockfile, staged-replace commit point
  (`control/content.ptr`), in-flight marker — per D-005 and the spec's "corpus is files on disk"
  constraint. Paused after cycle 6 of 18 by user ruling. The transition record prices the
  replacement's test debt at "~2,000 lines" re-expressed against transactions
  (`kinako-sqlite-transition/record.md:8-14,101-103`). All six cycles passed verification — no
  gate failed; the stop was a user ruling.

- **F2 — SQLite *was* on the table, once, anonymously.** D-005's alternatives table
  (`.mochiko/features/FEAT-006/constraints-and-decisions.md:273-278`) lists option
  "D. Embedded relational store" — one row, pros "transactional by construction; console queries
  cheap", three cons. It is never named SQLite, never scored against the eight criteria, and the
  choice rationale (`:280-284`) adjudicates only A vs C — options B and D are dropped without a
  word. Zero occurrences of "SQLite"/"database"/"sql" in any FEAT-006 plan artifact
  (grep-verified across `constraints-and-decisions.md`, `plan.md`, `contest-brief.md`,
  `requirements.md`, `architecture.md`, `data-model.md`, `tasks.md`).

- **F3 (as softened at review — C12) — The dispositive con was an upstream spec constraint.**
  Option D's row carried three cons, two of them merits arguments (single point of corruption
  for the one non-re-derivable record class; SC-005 still needing a Kinako-free rendering); the
  constraint con is the only one dispositive by nature: "it moves the corpus off the
  file-computable floor the spec's constraints name."
  The spec (`.mochiko/specs/corpus-stewardship/spec.md:15`) ratified "corpus is files on disk,
  consistent with D5's file-computable floor; format choice is plan-level" at acceptance
  2026-08-14 — frozen input to the plan. No kinako governance item mandates plain files;
  `corpus.md` is storage-agnostic, and GI-006's "atomic and durable" is exactly what option D's
  pro column offered.

- **F4 — The constraint was a misreading.** The D5 floor's real requirement (Claude-integration
  record) was that the sandboxed `kinako` CLI can compute retrieval locally without the app
  running. The transition record amends it to "SQL-computable over `corpus.db` … unchanged in
  spirit" (`kinako-sqlite-transition/record.md:79-82`) — SQLite satisfied the real requirement
  all along. "Files on disk" was implementation posture smuggled into a spec constraint. The
  Claude-integration reviewers had even caught the same over-constraint on the *atom* layer (B1)
  and repaired it there — nobody carried the objection to the corpus layer, which inherited the
  constraint silently.

- **F5 (as amended at review — C1) — Visibility asymmetry, not a gate.** The original claim —
  that kinako's crate registry made every new dependency a governance event, so adoption looked
  expensive while hand-rolling looked free — misread its own quote. The registry is scoped to
  **domain-layer crates only** (`engine-port.md` registry: `serde` · `uuid` · `jiff` ·
  `thiserror`; kinako `CLAUDE.md:43`), and "an addition would need a human ruling first"
  (`constraints-and-decisions.md:261-264`) refers to that registry. `rusqlite` is adapter-side:
  adopting it required only a PR-justification line (GI-009, kinako `CLAUDE.md:28`) — no gate,
  no ruling. What the evidence does show is a **ceremony asymmetry**: adoption is visible (PR
  line, registry ritual for domain crates, trust-signal levels) while hand-rolling carried no
  ceremony anywhere — the pen wrote "no registry change" (`:292`) as a satisfied consequence,
  zero-dependency posture displayed as a virtue, and no instrument priced the bespoke code.
  D-006 (lock) and D-007 (replace primitive) — the two decisions SQLite supersedes *outright* —
  weigh only filesystem primitives in their option tables; neither contains a "let the store
  handle it" row. Side-flag for the D6 probe: kinako's own transition record repeats the
  overread ("every new dependency needs a registry entry") and plans a registry entry for
  `rusqlite` — deliberate scope-widening or the same misread; kinako's call at the re-plan.

- **F6 — No seat held the mandate to challenge.** The principal-architect's contest fired against
  the plan-the-plan *proposal* (artifact list), before D-005 existed, and explicitly scoped the
  spec out: "I read each as given scope … I did not reopen any of them"
  (`contest-brief.md:351-355`). It read "format choice is plan-level" as a grant of authority to
  pick a format, and sustained the on-disk contract as one of three right-sized boundaries
  (`:300-306`). The one adversarial seat was charter-excluded from the one document holding the
  constraint that made SQLite ineligible.

- **F7 — Mochiko-side blindness is structural and total.** Plugin-wide grep: "off-the-shelf",
  "build vs buy", "COTS", "roll your own", "home-grown", "reinvent" — zero occurrences anywhere
  in `plugins/mochiko/`. Per surface:
  - `patterns-technical-decisions/SKILL.md:31,74` — the only alternative-generation rule is a
    *count* ("2-3 alternatives minimum"); three hand-built variants satisfy it. SQLite's sole
    *affirmative* plugin appearance (the prohibitory mention in `validate-requirements.py:27` is
    F10's — C11 fix) is `references/EVALUATION-MATRIX.md:92`, inside tables self-declared
    "illustrative examples" (`:74`), reachable only if the D-XXX is already framed as a storage
    *technology* choice — D-005 was framed as granularity/serialization, D-006/D-007 as
    mechanism choices, so the table never fires.
  - `patterns-plan-minimalism/SKILL.md:34-50` — the ladder runs over package elements (not
    technology choices), and rung 3 "Already exists?" is repo-bounded: "a baseline, the current
    system, or an installed dependency." A hand-built store + lock + atomic-replace clears every
    rung honestly.
  - `patterns-code-minimalism/SKILL.md:49-51` — rung 5 covers only deps already in the manifest;
    the parenthetical explicitly excludes adopting a new one ("Adding a NEW dependency is not a
    rung — it rides the domain-registry ruling **where `references/DOMAIN-DEPENDENCIES.md`
    applies**, and is never auto-approved" — scope qualifier restored per V1; that registry
    rules I/O libraries inadmissible, so an adapter-side dependency rides no registry and hits
    no gate there either). So even at build time, reaching for SQLite is an out-of-ladder move —
    not because a gate blocks it, but because no rung expresses it and the cards already commit
    the hand-built shape.
  - `agents/principal-architect.md:69-72,84,93,101` — the minimalism vocabulary is *fewer* boxes
    ("structure that no requirement pays for"), never *cheaper* boxes; "extension over invention"
    is scoped to components already in the topology. The contest is "the ladder applied
    adversarially" (`commands/plan.md`), so it can only contest what the rungs can express.
  - `agents/tech-lead.md:87` + `review-feasibility/SKILL.md:37-39` — excess class 7 reduces by
    definition to plan-minimalism rung 1 ("which requirement pays for this?"). Hand-built locking
    answers that question correctly; the class is defenseless against machinery that is *needed
    but should have been bought*.
  - `patterns-system-design/SKILL.md:21-22,150` — delegates necessity to plan-minimalism and
    fork-evaluation to technical-decisions; the delta summary "links, never restates" — the
    architecture artifact inherits whatever the D-XXX decided and never reopens it.
  - Net asymmetry (as amended at verify — V1, aligning with F5's C1 correction): **adopting a
    component carries visible ceremony — PR justification, and registry ritual with trust levels
    where the domain registry applies — while hand-building the same capability carries no
    ceremony anywhere.** Adoption's ceremony is lighter than this record first claimed (no gate,
    no ruling for adapter-side dependencies); hand-rolling still has none at all, which makes
    the conclusion stronger, not weaker.

- **F8 — The retrospect contains no process post-mortem.** `kinako-sqlite-transition/record.md`
  states "This session's job was not to relitigate that ruling" (`:11-12`); "hand-rolled" is the
  entire implicit critique. The miss diagnosis (F2–F7) is authored fresh in this session.

- **F9 — Color.** Cycle 6's verification report observed WebKit dropping `hsts-storage.sqlite`
  into the app-data root beside the corpus (`reports/cycle-06-verification.md:102`) — SQLite was
  physically on disk, one directory away, throughout the six cycles spent hand-rolling
  transactions.

- **F10 — Adjacent irony in mochiko's own tooling.**
  `authoring-requirements/scripts/validate-requirements.py:27` bans "sqlite" in FRs as
  technology leakage — but "files on disk" passed, because a storage *posture* stated without a
  product name is invisible to the banned-terms check. The leakage detector catches brand names,
  not implementation commitments.

## Failure Chain (the miss, reconstructed)

1. **Specify time:** "corpus is files on disk" entered the spec's Constraints line as a
   restatement of D5's floor — an implementation posture wearing constraint clothes (F3, F4).
   No review class flags storage-posture leakage (F10).
2. **Plan time, framing:** the analysis pen framed storage as granularity/serialization (D-005)
   and mechanism selection (D-006/D-007) — frames whose alternative sets never contain "don't
   build this layer" (F2, F5, F7).
3. **Plan time, incentive (as amended at review — C1):** adoption had ceremony and visibility;
   hand-rolling had none — nothing priced the bespoke code, and the pen displayed "no registry
   change" as a virtue (F5, F7).
4. **Contest:** the adversarial seat's charter excluded the spec, and its instrument (the ladder)
   cannot express "cheaper component", only "fewer components" (F6, F7).
5. **Build time:** cards inherited the commitment; code-minimalism rung 1 cannot cut a card's
   acceptance criteria, and rung 5 cannot add a dependency (F7).
6. **Detection:** the user, post-plan (driver, per session opening).

## Candidate Carriers (pre-brainstorm inventory — unruled)

- `patterns-technical-decisions` — require an existing-component candidate in every
  infrastructure-shaped alternative set; a custom-build choice must name what it was weighed
  against and why custom wins.
- `patterns-plan-minimalism` — widen rung 3 beyond the repo, or insert a rung between 2 and 3:
  "a proven component carries it".
- `principal-architect` persona — a buy-vs-build lens ("cheaper boxes", not only "fewer boxes");
  possibly a mandate to flag upstream constraints that smuggle implementation posture.
- `review-feasibility` — extend excess class 7 (or add a class): hand-built infrastructure where
  a boring proven component exists.
- Specify-time hygiene — `authoring-requirements` / `review-specifications`: constraints state
  capabilities, not storage postures; the F10 gap.
- Governance symmetry — make "build custom infrastructure in a solved category" a priced,
  human-ruled event, mirroring the new-dependency gate. *(Superseded at review, C1: no such
  gate fires for adapter-side dependencies — the pricing instrument is D4's retrofit-cost
  gate, standing on its own precedent; the "symmetry / mirror" framing is dead.)*

## Session Trail

- **Q1 — session scope.** Options: (A) one discipline, full chain — all six candidate carriers ·
  (B) plan-time only · (C) governance symmetry only · (D) diagnose-first. Recommendation: A with
  C folded in as one of A's carriers — the asymmetry is the root incentive *(trail as spoken;
  the "gate asymmetry" premise was corrected at review, C1 — see F5 as amended)*, but alone it leaves
  seats without instruments to name the alternative; kinako shows the miss survives any
  single-point fix (constraint born at specify, framed at analysis, unchallenged at contest).
  **Ruled: as recommended — A with C folded in** (user confirmed the composition explicitly).
  → **D1** `Confident` (as amended at review — C1): session scope is the full chain —
  off-the-shelf-first as a minimalism discipline touching technical-decisions alternative sets,
  plan-minimalism reuse rung, architect/tech-lead lenses, specify-time constraint hygiene, and
  the pricing gate as carriers of one discipline. *(C1 amendment: the folded option C's
  substance — hand-rolling gets priced — survives as D4's retrofit-cost gate on its own
  precedent; the "governance-symmetry / mirror the dependency gate" framing died with the F5
  correction.)*

- **Q2 — the trigger test.** Options: (A) disclosure floor + commodity test — every
  mechanism-shaped D-XXX *(trail as spoken; C8 later dropped "mechanism-shaped" as the operative
  term)* names one real off-the-shelf candidate or states "no shelf candidate
  exists", missing line itself a finding; judgment aid "is this problem older than this
  product?" · (B) structural test — trips only when the option set is all self-built · (C)
  supersession test — trips when one dependency would delete the decision · (D) named category
  list. Recommendation: A, with B's lesson baked in — kinako D-005 *had* the shelf row (option D)
  and still died in the table, so the obligation bites the **choice rationale**: custom wins only
  over a named shelf candidate, in writing. C presupposes the knowledge whose absence is the
  miss; D is a catalog by another name and the miss moves off-list. Disclosure-line form mirrors
  the external-research ruling (`verified:`/`memory-asserted` — silence becomes visible).
  **Ruled: as recommended** *(explicitly confirmed by the user at review disposition,
  2026-08-15 — C10)*.
  → **D2** `Confident` (as amended at review — C8 · C6 · C7 · C3): the discipline's instrument
  is a two-part obligation — (1) disclosure floor: the alternative set names at least one real
  off-the-shelf candidate or carries an explicit "no shelf candidate exists" line, absence itself
  a review finding; (2) rationale bite: choosing custom is legitimate only against the named
  shelf candidate — the choice paragraph must say why custom beats it (a shelf row silently
  dropped from the rationale, as in kinako D-005, is the defect this clause exists to catch).
  Commodity test as the judgment aid: "is this problem older than this product?" — infrastructure
  categories (storage, locking, serialization, queueing, caching, auth, search) presumptively
  qualify; the product's differentiating domain presumptively does not.
  Review amendments: **C8** — the trigger keys to the commodity categories; "mechanism-shaped"
  is dropped as the operative term (author framing must not gate the check — the kinako miss was
  itself a framing artifact). **C6** — the instrument is two-sided: the named candidate may lose
  on merits, and below the D4 line BE-DEP's "reasonably implementable in-house at <100 lines"
  red flag (`catalog/backend-service.md`) is a legitimate custom-wins rationale — weigh, choose
  on merits, disclose either way; no precedence conflict with BE-DEP once symmetric. **C7** — a
  named candidate is an outside-repo claim: it carries the external-research disclosure line
  (`verified: <source>` / `memory-asserted`), verified at review per that ruling
  (`EXTERNAL-CLAIMS.md` the single source). **C3** — graders and strength: the disclosure line
  is graded by `review-plan-artifacts` as conformance (blocking); the substantive
  needed-but-should-have-been-adopted case is `review-feasibility` class 7 (blocking-capable);
  rationale-bite honesty is advisory; the D4 gate is user-ruled by construction.

- **Q3 — authority boundary at a ratified upstream constraint.** Options: (A) flag-and-route-back
  to the user · (B) specify-time hygiene only, constraints frozen at plan · (C) contest charter
  widens to reopen spec constraints directly. Recommendation: A — B's upstream hygiene still
  ships (it is a D1 carrier), A is the belt to B's suspenders; C buys speed at the cost of the
  ratification contract. User asked whether A means raising to the human user; confirmed — and
  ruled yes on that basis: "these are expensive decisions."
  **Ruled: as recommended — A, route-back raises to the user.**
  → **D3** `Confident`: a plan seat never silently overrides a ratified constraint. When the D2
  shelf check collides with one, the seat MUST file a constraint-challenge finding — the
  constraint's stated text, the real requirement it plausibly restates, the shelf candidate it
  excludes — and the collision routes to the **user** for ruling (amend the spec or sustain the
  constraint). Only the colliding decision pauses; the plan proceeds elsewhere. Rationale
  (user's): these are expensive decisions — human rules them. *(C1 amendment: "mirroring the
  dependency gate" struck from the rationale — no such gate fires for the adapter-side class at
  issue; the user's expensive-decisions basis stands alone.)*
  Specify-time hygiene ships alongside as its own carrier (belt and suspenders).

- **Q4 — the pricing gate: who rules custom-over-shelf absent a constraint collision.** Options:
  (A) user-ruled always, exact dependency-gate mirror · (B) seat-decides with disclosure ·
  (C) retrofit-cost split. Recommendation: C — A prices cheap reversible choices the same as
  six-cycle storage engines; B is what kinako D-005 was, minus the disclosure; the gate exists
  for the irreversible class, and the split line reuses the ratified "nothing expensive to
  retrofit" principle (adaptive-depth D5). **Ruled: as recommended** *(explicitly confirmed by
  the user at review disposition, 2026-08-15 — C10; option A's "exact dependency-gate mirror"
  framing reads per the C1 correction — the gate analogy is dead, the retrofit-cost basis is
  the ruling's whole ground)*.
  → **D4** `Confident`: custom-over-shelf is **user-ruled when the custom build is expensive to
  walk back** — persisted formats, storage engines, locking/concurrency primitives,
  migration-bearing shapes — and seat-decidable with D2 disclosure below that line. Kinako's
  D-005/D-006/D-007 all trip the gate; a custom retry helper does not. Split criterion is
  retrofit cost, deliberately the same principle the adaptive-depth ruling ratified.

- **Q5 — carrier shape.** Options: (A) new sibling skill, single source, thin references from
  every carrier · (B) fold into `patterns-technical-decisions` · (C) distribute per carrier.
  Recommendation: A — exact precedent (plan-minimalism, map-minimalism, sound-loop,
  transport-floor all landed as sibling skills with pointer touches); GI-017 surfaces point at
  homes, never restate; B's home is too narrow for the cross-stage D3 valve and specify hygiene;
  C is the known restatement-drift failure mode. Rung-3 widening over new-rung insertion to
  avoid renumbering every rung citation. **Ruled: A.** Name trail (Q5a): user first chose
  `patterns-tech-stack`; after the lead flagged near-name collision with governance's
  "Technology stack" section and `analysis-codebase`'s `detect-stack.sh` and re-offered
  alternatives, the user adopted the recommendation (`patterns-shelf-first`). (Q5b, at review —
  C5): the cold reviewer showed `shelf` is itself established mochiko vocabulary — the
  governance catalog's shelf model (`catalog/README.md` "Shelf model", "Shelf: Backend /
  Service", `patterns-code-minimalism`'s "pending the frontend shelf", ROADMAP's Tier-I
  shelves) — the same collision class Q5a amended for, and stronger. Renamed
  **`patterns-adopt-first`**; user ruled.
  → **D5** `Confident` (as amended at Q5a and Q5b): carrier is a new single-source sibling skill
  named **`patterns-adopt-first`** — names the bias, not the topic; "adoption" is already the
  library's word for taking a dependency, so the name reuses existing vocabulary instead of
  colliding with it. Owns the D2 disclosure floor + commodity test, the D3 route-back, and the
  D4 retrofit-cost gate. Touch surfaces reference it in 2–3 lines, never restate:
  `patterns-technical-decisions` at alternative generation · `patterns-plan-minimalism` rung 3
  widened ("…or an adoptable proven component carries it") · `review-plan-artifacts` disclosure
  conformance + `review-feasibility` class-7 extension (per D2's C3 amendment) ·
  `patterns-code-minimalism` rung-5 pointer (per C2) · `principal-architect` / `tech-lead`
  persona lens lines · `authoring-requirements` / `review-specifications` constraint-posture
  hygiene line.

- **Q6 — evidence and application.** Options: (A) kinako FEAT-006 re-plan as first-live-run
  probe · (B) generic next-plan watch · (C) block the re-plan on this build. Recommendation: A,
  explicitly not C — never hold a paused product feature on a mochiko skill build; if the
  re-plan runs first, the watch moves to the next plan run. **Ruled: as recommended**
  *(explicitly confirmed by the user at review disposition, 2026-08-15 — C10)*.
  → **D6** `Confident` (as amended at review — C13): evidence honesty — the whole base is n=1
  (kinako FEAT-006), marker on record. First-live-run probe = the already-scheduled kinako
  FEAT-006 re-plan, run under the new discipline once built. Directional expectation:
  superseding D-rows name shelf candidates in alternative sets AND rationale; the D4 gate fires
  on the storage decisions; the D3 valve is exercised against the amended constraint line.
  Watch lands in BACKLOG; kinako is never blocked on the mochiko build.
  **Failure criterion (C13):** a post-build plan run that hand-rolls solved-category
  infrastructure without the disclosure firing means the discipline failed — instrument
  revisited; disclosure lines that only restate an already-user-ruled choice with no new
  information are a ceremony signal, also a defect. The probe also carries the F5 side-flag to
  kinako (their transition record's registry-scope overread).

## Build Surface (per D1–D6 as amended at review — cold-buildable)

One wave under the sound-loop + transport floors; every shipped-primitive edit takes a strip +
author≠grader audit per the landing ritual.

1. **New skill `plugins/mochiko/skills/patterns-adopt-first/SKILL.md`** — single source: D2
   disclosure floor ("names one real shelf candidate or states no shelf candidate exists;
   absence itself a finding" — "shelf candidate" is retained as ordinary English for an
   off-the-shelf option; the C5 collision concerned the skill's *name* in the library's
   vocabulary namespace, not prose) + rationale bite (custom wins only over the named candidate, in
   writing) + commodity test as the trigger ("is this problem older than this product?",
   presumptive infrastructure categories, non-exhaustive — C8) + the two-sided limb (candidate
   may lose on merits; BE-DEP's <100-lines red flag a legitimate custom-wins rationale below
   the D4 line — C6) + candidate-naming bound to the external-claims disclosure line
   (`verified:`/`memory-asserted`, verify-at-review — C7) · D3 constraint-challenge route-back
   (finding format: constraint text · real requirement it plausibly restates · shelf candidate
   excluded; user rules; only the colliding decision pauses) · D4 retrofit-cost gate (user-ruled
   above the expensive-to-walk-back line, seat-decidable with disclosure below). **Scope bound
   (C9):** plan seats own in-process libraries and self-hostable components; managed-service /
   SaaS / whole-capability buy routes to IP-XXX rows and the PM/user's business call — the
   bound lives in the skill's description.
2. `patterns-technical-decisions/SKILL.md` — alternative-generation line gains the shelf-candidate
   obligation, 2–3 lines, pointer only.
3. `patterns-plan-minimalism/SKILL.md` — rung 3 text widened ("a baseline, the current system, an
   installed dependency — or an adoptable proven component, per `mochiko:patterns-adopt-first`");
   no renumbering.
4. `review-feasibility/SKILL.md` — class-7 extension: needed-but-should-have-been-adopted is
   remove-shaped excess; pointer to the skill. **Plus (C3)** `review-plan-artifacts/SKILL.md` —
   the D2 disclosure line joins the conformance checks (blocking); rationale-bite honesty rides
   the existing advisory rung-honesty lane. (Which lane inside `review-plan-artifacts` takes the
   disclosure check — conformance proper or a named sibling check at conformance strength — is
   the builder's call at build time; the ruling fixes grader and strength, not the lane.)
5. `agents/principal-architect.md` + `agents/tech-lead.md` — one lens line each: cheaper boxes,
   not only fewer boxes. (`agents/technical-analyst.md` a build-time ladder candidate — the seat
   that authors D-XXX rows; producer decides under plan-minimalism.)
6. `authoring-requirements/SKILL.md` + `review-specifications/SKILL.md` — constraint-posture
   hygiene line: constraints state capabilities, never storage/implementation postures ("files
   on disk" is the canonical miss; `validate-requirements.py` stays as-is — posture detection is
   judgment, not a term list).
7. `commands/plan.md` — pointer line homing the D3 route-back in the plan flow.
8. **(C2)** `patterns-code-minimalism/SKILL.md` — rung-5 parenthetical gains the pointer: a NEW
   dependency still rides the domain-registry ruling, and a plan-committed adopt-first choice
   reaches cards as a binding constraint. The stdlib-above-installed-dep rung ordering stays,
   deliberately: at code altitude, per-task stdlib reuse is the right bias — the adopt-first
   question fires at plan altitude, where whole mechanisms are on the table. This closes
   failure-chain step 5.
9. **(C4)** Precedent coordination — SD-D5 and OO-D6 already ruled "established, never
   hand-rolled" for tooling, with `authoring-constitution/references/STACK-TOOLING.md` the
   defaulted create-or-join home (both Tier-I builds still queued). Relationship ruled:
   `patterns-adopt-first` is the plan-time **decision discipline**; STACK-TOOLING.md is the
   governance-floor **tooling-defaults home** — no merge. Obligation both directions: the two
   queued builds add a pointer to `patterns-adopt-first` when they land; this build's BACKLOG
   item notes the coordination so whichever lands first carries it.
10. Router row for the new skill. **(C14)** the new skill takes a row in
    `.mochiko/memory/primitive-cost-budgets.md`; the deterministic pre-assert applies —
    over-budget is FAIL absent a justified overage.

## Review (2026-08-15)

Solo cold review via blind-map two-message dispatch (26-angle map, topic-only spawn; reviewer
source-verified 14 citations against the kinako clone and the plugin tree — 11 exact, 1 broken,
1 internally inconsistent, 1 causal overreach). 17 raised, 3 killed in the reviewer's own
cross-examination, 14 survived: **1 Critical (C1), 9 Important (C2–C10), 4 Minor (C11–C14)**.
Verdict: critical-gaps. The lead source-verified C1 and C5 against the kinako clone and the
plugin tree before relay. Dispositions user-ruled: C2–C9 in one batch "as recommended"; C5
rename ruled → `patterns-adopt-first`; C10 ruled by explicit confirmation of Q2/Q4/Q6 (marks
stand `Confident`, confirmations recorded at the rulings); C1 talked through individually and
folded — F5/step-3 rewritten to ceremony/visibility asymmetry, D1/D3/D4 rationale re-anchored
(retrofit cost stands alone; "mirror the dependency gate" struck), kinako side-flag added to
the D6 probe. Minors lead-repaired in place (C11 F7 wording · C12 F3 softened · C13 D6 failure
criterion · C14 budget note). All folds applied same day.

**Verify trail:** round 1 **NOT CLEAN** — one blocking stale-echo (V1: the C1-superseded gate
claim surviving verbatim in F7's net line, plus the scope qualifier elided from the
`patterns-code-minimalism` quote) and three non-blocking notes (shelf-grammar rationale · C3
lane precision · Q2 trail marker); all lead-repaired same round. Bounded round 2 **CLEAN** —
both V1 locations, the sweep, all three notes, and no-new-contradictions checked against F5,
step 3, D1/D3/D4 amendments, D5's touch list, and the index entry.

**Provenance (C10 secondary):** evidence gathered 2026-08-15 by two reader subagents dispatched
by the session lead; the lead authored this record; the user ruled every fork.

## Session Trail — status

Q1–Q6 ruled (D1–D6; D1–D6 carry review amendments, D5 as amended at Q5a and Q5b). Cold review
complete, 14/14 dispositioned, folds applied; verify round 1 NOT CLEAN (V1 lead-repaired same
round) → bounded round 2 CLEAN. **Accepted 2026-08-15.** Landing executed same day: DECISIONS.md
row (ruled) · BACKLOG "Adopt-first build" section (build item + kinako-re-plan probe watch) ·
ROADMAP Next row (its addition tripped Next 8/7 — restored by merging the delivered epic-build
row into the plan-surface-builds row, link kept, merge precedent cited on the stamp).
**Built same day at v0.73.0** — one wave under the sound-loop + transport floors: 2 producer
seats on lead-approved plans, disjoint file ownership (skill + router + ledger vs the nine
pointer touches + strip); mesh-hold briefs; fan-in confirmation on every deliverable; 2 fresh
author≠grader validator seats on the quiesced tree — 13/13 PASS round 1, zero fix rounds; two
advisory alignments producer-applied post-audit (pointer-link form, lane wording; final body
6,493/6,500). All producer judgment calls ruled HOLDS, notably: the STACK-TOOLING pointer
phrased "home queued at…" (file verified absent — dead-pointer avoidance), the
review-plan-artifacts lane as a named sibling check at conformance strength (folding into
proposal-scoped conformance would have widened its meaning), and the rung-3 tail
("extend, reference, or adopt") disclosed inside the same supersession strip. Validator-ruled
divergence recorded: `authoring-requirements`/`review-specifications` carry the hygiene line
with NO pointer to `patterns-adopt-first` — deliberate, the skill is plan-time-scoped and a
specify-time pointer would misroute (Build Surface item 6 the more specific text; correct, not
a gap). Audit-added watch: plan-minimalism's repo-bounded "Read before you claim" line vs
widened rung 3's outside-repo claims — covered via the skill's external-claims binding, checked
at the D6 probe (BACKLOG). Build item → trail; probe watch stays. Gates 4+5 executed
(CHANGELOG entry, plugin.json + marketplace 0.73.0).

## Open Questions

- ~~Authority boundary~~ — ruled, D3 (route-back to the user).
- ~~Catalog-free trigger~~ — ruled, D2 (disclosure floor + commodity judgment aid; no catalog,
  honoring the no-kernel constraint).
- ~~Kinako application~~ — ruled, D6 (re-plan is the probe; never blocked).
- Residual (build-time): does `technical-analyst` persona take a lens line, or does the
  technical-decisions skill touch suffice? Producer's plan-minimalism call at build.
