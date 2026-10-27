# Wave 3 — implement keys the lever; the ship (`implement-lead-adaptive-rigor`)

**Opened:** 2026-10-28 · **Lead:** the build session lead · **Branch:** `adaptive-rigor-build`
(W1 closed at `5c4ba7c`, W2 at `c936678`) · **Target version:** 0.123.0, the one bump, after this
wave's gate · **Record:** `.mochiko/brainstorms/implement-lead-adaptive-rigor/record.md` —
D1-floor-four-questions, D2-stakes-read-ruling (as amended), D3-harm-band-smoke,
D5-band-step-latitude (as amended, S3/S7), D6-prior-rulings-fate, D7-third-swing-card,
D8-home-sound-loop, DF2, DF5 (as amended, S6), DF7, DF8, DF9, DF10, DF11, with
`research/floor-inventory.md` (the one re-class, the nine candidate steps). This plan names
carriers, never restates a decision; where it paraphrases, the card wins.

Wave 3 makes `/mochiko:implement` consume what W1 and W2 minted: the run declares `stakes`,
resolved at run-open; every thinnable `class: must` verification step is keyed on it and carries
its lighter form in the band; the two fail rules take their widened and stated-skip forms; the
run-open line and the trail land where DF5 says; the smoke dispatch is placed in the run; the
standard thin plan lands as the worked example the lead cites by name. Then the ship: the W1
floor-set replacement in the contract suite's freeze, the three W2 residuals, the bump, the
release gates, the KM landing.

## 0. Sound-loop wiring

Unchanged from W1/W2 (`wave1-sound-loop.md` §0, `wave2-spokes.md` §0): persona-less
`general-purpose` producers, `model: opus`, plan-only dispatch first, tree snapshot before and
after each plan round, a fresh generic peer per plan on `mochiko:review-seat-plan` (render pasted
verbatim), the lead approving only a PASS, one gate grader for the wave (DF11 — plugin edits ride
the locked gate form, never their own lever). The briefs carry **both attempt bounds** — the
re-plan bound is one, on a counter the two producers share, a second consumption to the user;
the gate fix loop is `common.gate-loop-bound`'s, a second audit FAIL to the user — and **the
Explore fallback** — a haiku `Explore` that returns no report (every dispatch so far) means the
locate reads run on-seat, disclosed. Repo writes go through Write/Edit, never a shell redirect
(the artifact hook). The lead's own writes are transcription and bookkeeping: this plan, the
build log, the budget-row verdict heads after the gate, the bump edits, the KM landing.

## 1. Sequence allocation

- **`0104-implement-stakes.yaml`** — three schemas in one migration: `command/implement` (the
  bulk), `skill/review-sufficiency` (one residual reword), `skill/patterns-sound-loop` (the
  worked-example pointer on the band rule). Header anchor `2026-10-27 implement-lead-adaptive-rigor`
  — **required**: the migration rewords two `kind: fail` floors and one `class: floor` bound and
  clears a `when:` on a fail rule (the log's anchor rule; widening is not a lowering, but the
  header carries the authority either way, the `0080`/`0095`/`0102` precedent). Each minted rule
  anchored to its card (`… D3`, `… D5`, `… D1`). Sequence 104. No floor minted, retired or
  lowered: implement stays 40 floors · 17 fails; rules 131 → 133.

## 2. Seats and ownership

- **P1 — schema + what the schema points at.** Owns `plugins/mochiko/migrations/0104-implement-stakes.yaml`;
  the regenerated `.mochiko/schema-views/` (`mochiko-cli views emit`; expect `commands/implement.yaml`,
  `skills/review-sufficiency.yaml`, `skills/patterns-sound-loop.yaml` to change, nothing else);
  the new reference file `plugins/mochiko/skills/patterns-sound-loop/references/STANDARD-THIN-PLAN.md`
  (the band rule's `pointer:` target — one seat owns both halves of one pointer; budget-exempt);
  any `scripts/similar-rules-allowlist.yaml` row a near-dup cluster forces (admissible only for
  an edge whose extra content is command-specific, each row reported to the lead); and the
  contract-suite freeze replacement in `evals/contract/expected-skills.json` — the
  `patterns-sound-loop` row's `floor_ids` and `floor_pin` only, derived from the W1 render
  (`mochiko-cli rules patterns-sound-loop --section preamble --plugin-root plugins/mochiko`:
  the `floors:` line, ten ids, pin 10), every other field of the row byte-identical
  (`evals/contract/README.md` §freeze: "A replacement moves `floor_ids` and `floor_pin`, and
  nothing else"; `freeze_expectations.py` is not run — its two guards refuse the converted
  tree, so the set is derived under this plan's ruling and audited as a field-scoped diff).
  Nothing else.
- **P2 — prose + ledger.** Owns `plugins/mochiko/commands/implement.md` (one clause on the
  Entry step's run-open confirmation — the stakes line put for the user's ruling — a pure
  addition; frontmatter, Identity, Rules block, Goal and Not-done untouched);
  `plugins/mochiko/skills/patterns-transport-floor/SKILL.md` lines 13 and 49 (the W2 residual —
  the old remit paraphrase "who produces and who reviews" / "who produces, who reviews" widened
  by "and how deep", the sound-loop [v0.123.0] strip entry's wording); the strip entry
  `.mochiko/strips/patterns-transport-floor.md` `## [v0.123.0] …` (supersession by ruling, D8 with
  D6 row 2, the sound-loop entry as the shape); the router row
  `plugins/mochiko/skills/mochiko/SKILL.md:70` (`patterns-sound-loop` gains the stakes key and
  band — how deep the legs run after the trigger fires, never whether they fire — with the
  never-thins set, the five review-form invariants and the trail; a pure addition, no strip);
  and `.mochiko/memory/primitive-cost-budgets.md` (the three rows `patterns-sound-loop` ·
  `review-sufficiency` · `patterns-transport-floor`, re-measured after `0104` lands — the two
  [v0.123.0] ruled-HOLDS heads re-stated with the wave-3 figure and unit beside the wave-1/2
  one; the transport-floor row's body figure moved by the paraphrase fix). P2's measures wait
  for `0104`; P2's prose edits do not. Nothing else.
- **P3 — crate tests** *(added 2026-10-28 after the seven-unit gate passed: the pre-bump
  `cargo test --all` went red on the census pins — `fidelity.rs` and `validate.rs` pin the log's
  sequence list and live-rule census at 1..101 / 1,266, `matrix_similar.rs` the similarity
  matrix — a unit the W1 and W2 plans omitted and the engine-follow-up build carried in every
  wave as its "crate tests · crate non-author review" unit; `.claude/rules/mochiko/rust-cli.md`:
  every crate unit lands on a lead-approved plan with an independent non-author code review.)*
  Owns `crates/mochiko-cli/tests/fidelity.rs`, `crates/mochiko-cli/tests/validate.rs`,
  `crates/mochiko-cli/tests/matrix_similar.rs` — the census pins and their comment paragraphs
  re-measured for `0102`–`0104` (sequences gain 102, 103, 104; command rules 424 → 426; skill
  rules 842 → 857; total 1,266 → 1,283; skill floors 266 → 270; command floors 130 and the
  fail set 41 unmoved; documents 94; the similarity matrix's pair counts and suppressed-edge
  figure as the sweep measures them), every figure from the seat's own red-then-green runs,
  no production code touched. Nothing else.
- **The lead (after the gate):** the bump edits — `plugins/mochiko/.claude-plugin/plugin.json` and
  `.claude-plugin/marketplace.json` to 0.123.0, the `CHANGELOG.md` `[0.123.0]` entry (MINOR), the
  budget-row verdict heads — then the release gates, the KM landing, the commits (§5).

## 3. Carriers

Every `when:` below uses the list form, on the whole list —
`when: {stakes: [money, data, security, availability, wrong-answer, cosmetic]}` — as `0102`
and `0103` key theirs: a stakes-keyed rule is thereby marked as one the band may thin
(`patterns-sound-loop.stakes-band`: "any `class: must` verification step the command keys on
`stakes`"), its text carrying the band split; the fixed-band values stay covered, so no
`condition-coverage` advisory enters the baseline (118). A rule already keyed on another
condition takes the two-term form (`when: {ux_bearing: present, stakes: [...]}`, the `0001`/`0012`
precedent). Two wordings, by the step's position against the smoke — the W2 `once-per-seat-in-band`
reading, recorded in the build log: a step **before** the smoke thins "per the plan the user ruled
at run-open, provisionally until the cold smoke — a red smoke, or a money, data or security surface
in the diff, returns every later step to full depth and the rework to a full grade (DF9)"; a step
**after** the smoke thins "per the ruled plan, on a clean cold smoke". Every thinnable step ends
"money, data, security: <the full form>". A thinnable step stays `class: must` (DF7).

| id | op · class · kind | section | when | labels | source | gist |
|---|---|---|---|---|---|---|
| condition `stakes` on `command/implement` | `set-condition` | preamble | — | — | D2, D3, DF8 | values the six harm classes · `moment-resolved(run-open)` · note: the ruled harm class of the run's product work — proposed per unit with its cited line by the sufficiency seat (`review-sufficiency.stakes-proposal`), raised never lowered by the lead, ruled by the user on the run-open line; on an epic the highest class among the members, each read from its own spec lines |
| moment `run-open` | `set-moment` | preamble | — | — | D2, DF5 | "The confirmation that closes entry — batch, scope type, attempt bounds, the stakes line, done condition." |
| `impl.staffing-latitude` | reword (must · latitude, kept) | roles | — | seats | D5, DF7 | widened: beyond the floors, staffing, sequencing and running the cycles stay yours; and, inside the band the ruled class sets, which `stakes`-keyed steps thin or are skipped and how deep each runs — proposed in the run-open line, the user's ruling, never yours alone; the key, band, never-thins set and trail `mochiko:patterns-sound-loop`'s, referenced |
| `impl.user-runopen-rulings` | reword (must · reservation, kept) | reserved | — | user-gate, scope-entry | D2, DF5 | appends: "· the stakes read, the review form and the plan of thinnings, put as the run-open line — `stakes: <class> — <cited line> · smoke: <scope> at <point> · thins on clean: <steps> · never: <steps>` — the standard thin plan citable by name; a lowering after run-open is the user's checkpoint ruling" |
| `impl.sufficiency-report` | reword (must · binding, kept) | tools | — | evidence, reporting | D2, DF5 | appends: the proposed stakes class per unit with its cited line; and, after the ruling, the run-open line as ruled, appended by you — transcription of the user's word, dated |
| `impl.promises-kept-table` | reword (must · duty, kept) | reserved | — | user-gate, evidence | DF5 (S6) | appends: beneath the rows, the trail — each lowering line of the final-validation report shown as it stands, `none` where nothing thinned |
| `impl.craft-verify-bindings` | reword + `when` (must · binding, kept) | tools | six | floor-pointer, evidence | D5, DF10 | appends: in the band (before-smoke wording) one non-author grade may cover several small cycles — each cycle's `**TEST:**` cases still executed, never thinned — and the code-minimalism lens may be skipped, stated; money, data, security: every cycle graded, the lens run |
| `impl.gates-full-suite` | reword + `when` (must, kept) | tools | six | evidence | D5 | in the band (before-smoke wording) a cycle's gates may run the tests the cycle touches, the full suite once in the cold smoke (`impl.cold-verification`); money, data, security: the full suite on every cycle |
| `impl.regression-sweep` | reword + `when` (must, kept) | tools | six | evidence, landing | D5 | appends: in the band (after-smoke wording) the sweep may narrow to the gates of the territories this run touched, stated in the trail — the smoke's full suite covers the automated rest; only the hand-run gates of other delivered features go unrun, and only here; money, data, security: the whole accumulated set |
| `impl.cold-verification` | reword (must, kept; **no `when`** — never thins) | tools | — | evidence | D5 (S7), D6 row 5, DF8 | appends: where the ruled plan names a smoke, it runs in this snapshot (`testing-end-user.cold-run-always`), one per epic; the cold run always happens, on the exact head the user accepts, and repeats whenever rework changes code after it, within `impl.gap-rework-bound` — no CI result stands in for it |
| `impl.design-review-pair` | reword + `when` (must, kept) | tools | six | independence, floor-pointer | D5 | appends: in the band (before-smoke wording) the pair may thin to one non-author seat running `mochiko:review-plan-artifacts`, its meaning-kept grade still filling the checkpoint's landing table (`impl.gate-design-checkpoint`), the feasibility lens folded in where a store delta exists; money, data, security: the pair |
| `impl.plan-approval-producers` | reword + `when` (must, kept) | ways-of-working | six | seats, independence | D5, D6 row 4 | appends: in the band, per the ruled plan, the grade runs once per seat per run (`review-seat-plan.once-per-seat-in-band`); money, data, security: every plan |
| `impl.card-review-before-confirm` | reword + `when` (must, kept) | tools | six | independence | D5 | appends: in the band (before-smoke wording) the review's depth may thin as the line names — its seat stays a non-author and every "I'd know it works if…" line keeps its `**TEST:**` check; money, data, security: the full review |
| `impl.gap-finding-scope` | reword + `when` (must, kept) | tools | six | scope-entry, evidence | D5, D6 row 6 | the scope key stays; adds: in the band, on a clean cold smoke and per the ruled plan, the pass may be skipped, stated — the smoke's hand probes its lighter form, the non-author read before acceptance standing (`testing-gap-finding.band-skip-stated`); money, data, security: it runs whatever any probe says; every skip, by scope or by band, stated in the final-validation report — never a silent no-op |
| `impl.design-stage-blind-list` | reword + `when` (must · duty, kept) | roles | six | independence, seats | D5 | appends: in the band, per the plan the user ruled at run-open, it may be skipped at one or two gaps, stated in the trail; money, data, security: it runs |
| `impl.design-finish` | reword + `when` two-term (must · routing, kept; pointer kept) | tools | `{ux_bearing: present, stakes: [six]}` | evidence | D5, D6 row 3 | appends: in the band, on a clean cold smoke and per the ruled plan, the design audit may be skipped, its reason stated in the `design-audit:` block (`impl.design-audit-advisory`); the critique lens rides the gap-finding pass and goes with it where that pass is skipped stated; money, data, security: both run |
| `impl.attempt-per-grade` | reword (**floor** · bound, kept; header anchor) | boundaries | — | attempt-economy | DF10 | appends: a grade shared over several cycles charges one attempt to each cycle it covers; a FAIL on one charges only that cycle's rework |
| `impl.lowering-trail` | **mint** · must · duty | ways-of-working | — | reporting, evidence | D1 q3, DF5 (S6), D7 line 3 | every thinned or skipped step is written, one line per lowering, into the final-validation report file — the step, the smoke result it rode on, what still covers the failure kind — the durable home `impl.fail.skip-unstated` reads; shown beneath the promises-kept table; a raise needs no line (`patterns-sound-loop.trail-of-lowerings`, referenced) |
| `impl.smoke-probe` | **mint** · must · (constraint) · pointer `mochiko:testing-end-user` | tools | six | evidence, seats | D3, DF2, DF8, DF9 | where the ruled plan names a smoke, the run's QA seat (`qa_seat`) runs it at the point the line names — default after the build, before final validation — inside the cold snapshot, its legs and its report the pointer skill's; one smoke per epic; a red, or a money, data or security surface in the diff, returns the run to full depth — the rework graded full for the touched cycle, the band gone for the run, the cold run repeated; a clean smoke is a weak negative and thins only what the line named |
| `impl.fail.skip-unstated` | reword + `when: ~` + `enforces` (**floor** · fail, kept; header anchor) | fail-conditions | **cleared** (every scope) | reporting | D5, D7 line 3, inventory | "A run whose final-validation report does not state a thinned or skipped step — the gap-finding pass skipped by scope, or any step thinned or skipped in the band." `enforces: [impl.gap-finding-scope, impl.lowering-trail]` |
| `impl.fail.gap-finding-missing` | reword (**floor** · fail, kept; `when: scope=selection|epic` kept; enforces kept; header anchor) | fail-conditions | selection, epic | evidence | D1 (inventory re-class), D5 | the stated-skip form: "A selection-scope or epic run whose gap-finding pass neither ran nor is stated skipped — at run-open on the ruled plan and in the final-validation report." |
| `review-sufficiency.stakes-proposal` | reword (must · duty, kept; anchor and pointer kept) | verdict | — | verdict, evidence | D2, W2 grader note 1 | "(the spec, a baseline or the store)" → "(any surface the fence admits — the spec, a baseline, the store, the map entry, or under delta scope the delta card)" |
| `patterns-sound-loop.stakes-band` | reword + `set-rule-field pointer: references/STANDARD-THIN-PLAN.md` (must, kept; anchor kept) | trigger | six (kept) | trigger, ladder | D5, D7 line 6 | appends: "The standard thin plan (`references/STANDARD-THIN-PLAN.md`) is the worked example the lead cites by name in the run-open line instead of inventing one — an example, never a menu." |

**The reference file** `patterns-sound-loop/references/STANDARD-THIN-PLAN.md` carries D5's worked
example as one table — step · full form · thin form · what still catches the failure kind —
the seven rows of the card's statement (one non-author grade over several small cycles ·
affected tests per cycle, the full suite once in the cold smoke · regression gates of touched
territories only · design review by one seat on `mochiko:review-plan-artifacts`, feasibility
folded in where a store delta exists · plan-QA once per seat per run · gap-finding, the
code-minimalism lens and the design-stage blind list (at one or two gaps) skipped, stated), the
never-move set (D5: the baseline diff review's mechanical legs, the built-vs-signed diff, the
landing verifier, the sufficiency check, the cold run, the non-author read before acceptance,
`**TEST:**` execution), one example run-open line in DF5's grammar, and the head line "an
example, never a menu (D7)". It restates no rule; it is the example the band rule points at.

Floors: implement 40 (unchanged; three reworded under the header anchor) · fails 17 · rules
131 → 133 (`impl.sec.tools` 50 → 51, `impl.sec.ways-of-working` 15 → 16). `patterns-sound-loop`
10 floors · 21 rules; `review-sufficiency` 8 · 30 — unchanged. No `${var}` in new text except
`qa_seat` read as its var; no new label; every pointer resolves (a skill, or the reference file
base-dir-relative). Implement is a command: unbudgeted; `references/` exempt.

## 4. Gate units (one grader, `general-purpose`, `model: opus`, rendering `mochiko:validation-primitive-edit` first-hand)

Baseline for every diff: `git archive c936678 plugins/mochiko scripts/similar-rules-allowlist.yaml evals/contract/expected-skills.json`.
Pre-pass, run by the grader: `mochiko-cli migrate validate --report --plugin-root plugins/mochiko`
(0 rejecting; advisory noted) and the ledger's canonical snippet.

- **Unit 1 — schema content:** `0104-implement-stakes.yaml` + the three regenerated view diffs
  (+ any allowlist row). Items: intent stated · anchor present (required here) · ID lifecycle
  right (two mints; nineteen rewords keeping id, class, kind, labels, anchors) · floor and fail
  survival (40 · 17; the three protected rewords covered by D5/DF10/the inventory) · register.
- **Unit 2 — `implement` command pair:** `commands/implement.md` + its render; the canonical-scaffold
  criteria 1–11, criterion 7's implement branch in particular — the stakes line rides the
  **existing** run-open confirmation, no new gate (ADR 2026-08-13 ruling 3); criterion 3's pin
  citation unchanged (`kind: fail · 17 rules`); criterion 11 — the new condition declared, every
  `when:` term resolving, `enforces:` resolving on both fail rules, the cleared `when:` read as
  every scope.
- **Unit 3 — `patterns-sound-loop` skill pair:** `SKILL.md` (unchanged) + render (changed by the
  band reword and pointer) + `references/STANDARD-THIN-PLAN.md` (criterion 9 pointer resolution;
  the file's register; no rule restated). Budget: the payload re-measured (the [v0.123.0]
  ruled-HOLDS head carries the wave-1 figure; the wave-3 delta argued as D5's worked-example
  obligation).
- **Unit 4 — `review-sufficiency` skill pair:** render changed, `SKILL.md` unchanged; the anchor
  kept under reword; budget re-measured.
- **Unit 5 — `patterns-transport-floor` skill pair:** `SKILL.md` changed (two lines), render
  unchanged; the strip entry present and verbatim; budget re-measured.
- **Unit 6 — router `skills/mochiko/SKILL.md`:** prose primitive — internal coherence plus
  preserved responsibilities; the row addition only.
- **Unit 7 — `evals/contract/expected-skills.json`:** the field-scoped diff — `floor_ids` and
  `floor_pin` of the `patterns-sound-loop` row against the grader's own render of the preamble;
  every byte column and every other row byte-identical to `c936678`.
- **Unit 8 — crate tests (added 2026-10-28):** the three test files P3 edits — the crate
  non-author review: every pinned figure reproduced by the grader from the replayed state and its
  own `cargo test --all` (plus `MOCHIKO_FULL_SIMILAR=1 … --test matrix_similar`) green; comment
  paragraphs accurate to the three migrations; no production code touched; `cargo fmt --check`
  and `clippy -D warnings` clean.

## 5. Ship (the lead, after all units PASS)

1. Commit the wave on the branch: "W3: implement keys the lever; the freeze moves with it".
2. Bump edits: both manifests to 0.123.0; `CHANGELOG.md` `[0.123.0]` — MINOR, dated, the three
   waves in order (W1 `0102`, W2 `0103` + allowlist row, W3 `0104` + reference + freeze +
   residuals), the audit outcome lines' counts, the budget verdicts, the strip entries, the
   figures (rules 1,266 → 1,283; floors 266 → 270 across the skills; implement 40 · 17), the
   D6 row fates as landed, DF11's note that the build rode the locked gate form.
3. Release gates on the bumped tree, first-hand: `migrate validate` 0 rejecting · `views emit`
   ≡ committed views (derived view ≡ replay) · `migrate status` 1..104 · `cargo test --all` ·
   `cargo fmt --check` · `cargo clippy` · `cargo audit` · `mochiko-cli ids --check plugins/mochiko`
   · `gitleaks detect --no-git` · rebuild `target/release/mochiko-cli` · the contract suite
   `python3 evals/contract/run.py` — the deterministic set green, the sandbox cases on the user's
   `sbx login` (a SKIPPED suite is not green: halt and surface, the bump waits).
4. KM landing (one move, `.mochiko/memory/knowledge-management.md`): `DECISIONS.md` 2026-10-27 row
   stamped built at v0.123.0 (the seven annotated rows untouched — their fates landed at the
   session close); `BACKLOG.md` *Implement adaptive-rigor build* item to
   `.mochiko/archive/backlog-trail.md` (the OQ2 item stays open); `ROADMAP.md` Now line stamped
   BUILT; the brainstorms index entry and the record's Status line stamped built — statuses
   agreeing. Read `.claude/rules/mochiko/operating-docs.md` before the first of these edits.
5. Release commit on the branch; no push, no merge — the user's. Hand-off note: the branch, the
   two commits, the gates' figures, what the user's trials should watch (D7 line 6, D9).

## 6. Close

A FAIL on any unit → fix by the owning producer, re-audit by the same grader on the touched
files only; a second FAIL halts to the user with both fix lists. The disclosure line for the
wave rides the build log: `floor: tripped · seats: … · plans: w3-p1-schema:PASS|FAIL(n) ·
w3-p2-prose:PASS|FAIL(n) · stakes: n/a · thinned: none` — plugin primitives, fixed form.
