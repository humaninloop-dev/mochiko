# Build log — `specify-brainstorm-discovery`

Append-only. One `##` entry per event, 60 lines at most.

## 2026-10-10 — wave 1 opened

- Record accepted 2026-10-10 ("accepte"); the close ritual ran before any build step (the
  decisions row, the backlog section, the roadmap fold, the index entry).
- Branch `specify-story-stage` created from `main` @ `ea57167` on the user's word ("open the
  build now"). It carries the uncommitted close ritual and the open sibling session's directory
  (`architecture-brainstorm-interview`, the user's other session). Nothing committed, nothing
  pushed.
- Plan: `wave1-story-stage.md`. Sequences 0051–0056 allocated, 0057–0058 held.
- Seats: three persona-less producers (`general-purpose`, `model: opus`) — P1 schema, P2 prose,
  P3 tests and kits; a fresh generic peer per plan; one gate grader for the wave.
- Baseline before the first dispatch: `mochiko-cli migrate validate --report` → 0 rejecting,
  113 advisory, clusters 0, 185 allowlist-suppressed edges; log at sequence 0050 (48 migrations,
  state `sha256:c7e23ab0…`, 87 documents, 1,175 rules); plugin 0.118.0; binary 0.4.0 (grammar
  1..2); crate census pins read: command rules 362 · skill rules 813 · command floors 119 · skill
  floors 264; contract `Expected["specify"]` freezes 17 floor ids (8 floors + 9 fails).
- The lead's own writes so far are the close ritual (transcription of the user's rulings), the
  plan and this log (session artifacts). No plugin primitive has been touched by the lead.

## 2026-10-10 — P1 schema: plan round

- P1 (`p1-schema`, persona-less `general-purpose`, `model: opus`) dispatched plan-only against
  §3.1–§3.6 with the specify render and the four skill renders in the brief; it planned
  0051–0056 and stopped. Tree snapshot before and after: unchanged.
- Grader `g1-plan-grader` (fresh generic peer, `model: opus`, `review-seat-plan` render pasted
  verbatim) returned `PLAN GRADE: P1 schema · FAIL · 4, 5, 6` — items 1–3 PASS, 7 no finding.
  Failed items: the 0056 header line carried a blind-list seat the ruling never named; moments
  were not named literally in any rule text (validate's `unused-moment` check); the slate-close
  vet and the post-authoring filter were folded into one reword.
- Lead rulings sent to P1 with the grade (recorded here, not in the record — they bind the
  build, not the ruling): provenance in the Story cell (Q1); seed in the index Test column only
  (Q2); no reword of `spec.reserved-to-user` (M1); `spec.two-confirm-screens` merged into
  `spec.gate-slate-confirm` + `spec.story-stage` (M2); `spec.handed-in-slate` split with
  `when: slate_source=handed-in` (S1); `slate-present` keyed on the `> Slate: …` header line;
  joined `US-<n>-<slug>` cites; the fail anchored to D1; header line carries the slate's source
  only; moments `story-stage` and `slate-confirm` named literally once; `slate_source` resolves
  `moment-resolved(story-stage)`; `handed in` marks any record-sourced row; new mint
  `spec.pm-vets-slate-once` in roles, `spec.filter-rejections-recorded` reworded only for the
  post-slate clause; allowlist rows only for specify↔brainstorm D5 pairs with one reason each;
  validate loop capped at two re-runs; the shared re-plan is spent by this revision.
- P1 returned its revision (delta only, nothing written; tree unchanged — lead's diff clean).
  Disclosed read: `crates/mochiko-cli/src/validate.rs` ~1488 (`unused-moment` passes on a
  literal substring or a moment-resolved condition). Net 0051: 19 mints, 3 rewords, 2 moments,
  1 condition. Expected after build: 1,196 rules (command 362→381, skill 813→815, command floors
  119→120); pins `kind: fail · 10`, `class: floor · 18`; specify render ~27,700 chars.
- Revision sent to the same grader for the delta-only re-grade. A second FAIL goes to the user.
- Round 2: `PLAN GRADE: P1 schema · PASS · —`. Three advisories, none blocking: (1) D6's close
  condition now lives in `spec.story-stage-stop-rule` and the `slate-confirm` moment text, not
  the gate — the gate grader's decision-by-decision read maps it there; (2) rewords made to clear
  a cluster are reported with before/after text; (3) the filter tail said "after" twice. Lead
  folded (2) and (3) as wording-only rulings into the approval; (1) goes into the gate grader's
  brief. Noted, out of scope: the 0046 skeleton shows US-2 `rejected` in the index while Derived
  features homes it on FEAT-YYY — a pre-existing template defect, carried byte-for-byte.
- Approval sent to P1 with the build open: stamp 0051–0056, validate (≤2 re-runs), views emit,
  report, stop.

## 2026-10-10 — P1 schema: build landed (lead-verified, not graded)

- New: `plugins/mochiko/migrations/0051-specify-story-stage.yaml` · `0052-stories-from-slate.yaml`
  · `0053-review-slate-checks.yaml` · `0054-fr-cites-story.yaml` · `0055-map-slate-touchpoint.yaml`
  · `0056-spec-template-slate.yaml`. Changed: the six schema views (specify, four skills, spec
  template) and `scripts/similar-rules-allowlist.yaml` (+1 row, D5 pair
  `brainstorm.own-rules-win`↔`spec.own-rules-win`). `git diff --stat`: 11 files, +319/−37 (that
  figure includes the close ritual's four operating docs).
- Validate loop: first run raised two clusters — the D5 pair above (0.87) and
  `setup.fail.pre-ratification-authoring`↔`spec.fail.story-outside-slate` (0.62). Re-run 1 after
  a reword of the fail text (before: "A story authored with no slate confirmed, or outside the
  confirmed slate." · after: "A story file with no confirmed slate behind it, or one outside the
  confirmed slate."; id, class, kind, enforces, anchor unchanged); re-run 2 after the allowlist
  row. Cap of two re-runs used exactly. 0057–0058 untouched.
- Lead pulled the output itself (fan-in): `mochiko-cli migrate validate --report` → `0 rejecting
  · 113 advisory` · `clusters: 0 (none)` · `allowlist-suppressed edges: 185` · `rules scanned:
  1196`; `migrate status` → `sequences 1..56 (54 migrations)` · `state sha256:fad81e34…` · `87
  documents · 1196 rules`; specify preamble pins `kind: fail · 10 rules` · `class: floor · 18
  rules`; moments `story-stage` and `slate-confirm` present, `slate_source` resolves
  `moment-resolved(story-stage)`; `views emit` to a scratch dir diffs clean against
  `.mochiko/schema-views` (views ≡ replay). Render sizes by the measure script: specify 27,999 ·
  authoring-user-stories 9,914 · review-specifications 14,664 · authoring-requirements 9,413 ·
  authoring-feature-map 16,829.
- Advisory delta P1 reports against a 0001–0050 replay: +1 `condition-coverage ·
  review-specifications · slate-present` absent (same shape as the standing `manifest-present`
  line); −1 `zero-member-label · command/specify · attempt-economy`; `enforces-coverage ·
  command/specify` lists 14 with the new fail; a pre-existing `unused-moment · stories-confirm`
  line stands. Suppressed edges 185 → 184 → 185: the filter reword likely took
  `spec.filter-rejections-recorded`↔`authoring-feature-map.complete-disposition` under threshold;
  its row stays as a tripwire.
- Disclosed deviation: the fail reword was made by editing 0051 in place and re-stamping, before
  any audit. Lead's ruling: in-loop, pre-audit, within "one reword inside the write set" — not
  the gate-fix ban, which binds after a unit verdict. Flagged to the gate grader as a disclosed
  fact for unit 1. No Explore dispatch in the build.
- Counts handed to P3 as claims to re-read from the tool: command rules 362 → 381 · skill rules
  813 → 815 · total 1196 · command floors 119 → 120 · skill floors 264 unchanged.
- P2 prose and P3 tests/kits dispatched plan-only in parallel (persona-less `general-purpose`,
  `model: opus`, named `p2-prose` and `p3-tests`). The wave's one shared re-plan is spent: a FAIL
  on either plan goes to the user (`review-seat-plan.approval-is-the-leads`).

## 2026-10-10 — lead's read of the built rules against §3.7 (clears nothing)

- Every row of the §3.7 coverage table has its carrier in the rendered rules or the template:
  D1 (`spec.story-stage`, moment `story-stage`, `spec.slate-rulings-users`, the fail) · D2
  (`spec.slate-unit`, `authoring-user-stories.independent-test-required` + `.slate-fidelity`) ·
  D3 (`spec.lead-holds-story-stage`, `spec.analyst-brief-and-pen`) · D4
  (`spec.blind-journey-list`, `spec.slate-provenance`) · D5 (`spec.own-rules-win`) · D6
  (`spec.gate-slate-confirm`; the close condition in `spec.story-stage-stop-rule` and the
  `slate-confirm` moment text) · mechanisms (`spec.story-forks`, stop rule, provenance,
  `spec.intent-probe-discipline`) · slate home and binding (`spec.slate-home`,
  `spec.slate-binds-authoring`, `review-specifications.slate-checks` on `slate-present`,
  `authoring-user-stories.story-structure`, the template's User Stories guidance) · ordering
  (`spec.slate-not-the-frame`) · filter (`spec.filter-rejections-recorded`,
  `spec.pm-vets-slate-once`, `authoring-feature-map.four-touchpoints`) · floor-obligated rows ·
  inputs in hand and skip (`spec.stage-inputs-in-hand`, `spec.handed-in-slate`, `slate_source`) ·
  UX (`spec.lockstep-prototyping`) · FR cites (`authoring-requirements.fr-format`, check (3)).
- Two observations, no finding: `spec.gate-slate-confirm` is `class: must · kind: gate`, as the
  intent gate is (`spec.intent-stage-first` + `spec.reserved-to-user`, both `must`); the end
  state is floor-protected by the fail. The `## User Stories` section carries no line budget, as
  before 0056. Both go to the gate grader as context, not as findings.

## 2026-10-10 — gate grader opened on units 1–3; P2 plan round

- Gate grader `g2-gate-grader` (persona-less `general-purpose`, `model: opus`) spawned with the
  `validation-primitive-edit` render pasted verbatim, units 1–3 (schema content: 0051 + specify
  view diff + the allowlist row · 0052–0055 + four skill view diffs · 0056 + template view diff),
  the pre-pass commands, and P1's disclosure of the in-place fail reword. Its verdict blocks land
  in `reports/audit.md` (`report: review`, round 1) under its own pen; outcome lines return to the
  lead. Units 4–9 follow by resume once P2 and P3 land. P1's files are frozen (quiesce).
- P2 plan received (plan-only; tree unchanged — lead's diff clean). Disclosed: the ripple-sweep
  Explore (haiku) returned no handback; P2 re-ran the grep itself; `docs/` is absent from the
  tree. Write set: `specify.md`, three skill bodies (authoring-user-stories, authoring-requirements,
  authoring-feature-map), four strip files, `README.md`, `ARCHITECTURE.md`, the router row, four
  ledger rows, the CHANGELOG 0.119.0 entry with three `[lead: …]` placeholders.
  `review-specifications/SKILL.md` untouched (no line made false); `spec-template.md` takes no
  strip (schema content, per `primitive-edits.md` lines 17–24).
- Lead rulings on P2's questions: Q1 router row is a pure addition, no strip · Q2 no edit to
  `review-specifications/SKILL.md` · Q3 no widening to ARCHITECTURE.md's seat table or diagram —
  the blind-journey-list seat's absence there is a follow-up for the architecture reconstruction
  (`ARCHITECTURE.md` is hand-maintained legacy until the store carries ruled content) · Q4 the
  lead flips "argued" to the ruling word at close, after the gate audit.
- P2's observation, outside its set: the `authoring-user-stories.sec.output` section note still
  reads "the story set lands in `spec.md` per the artifact rules" — true of the index rows; noted
  for the gate grader's unit 2 read, no action proposed.
- Plan sent verbatim to a fresh generic peer `g3-plan-grader-p2` (`model: opus`,
  `review-seat-plan` render pasted) with §3.8, the write sets and the rulings above.

## 2026-10-10 — gate audit round 1, units 1–3: unit 1 FAIL (register), units 2–3 PASS

- Verdict blocks: `reports/audit.md` (`report: review`, round 1), written by the grader. Outcome
  lines as returned:
  - `audit: 0051 + specify view diff + allowlist row · g2-gate-grader · opus · 3 files · 1 rounds · 1 blocking`
  - `audit: 0052–0055 + four skill view diffs · g2-gate-grader · opus · 8 files · 1 rounds · 0 blocking`
  - `audit: 0056 + spec template view diff · g2-gate-grader · opus · 3 files · 1 rounds · 0 blocking`
- Unit 1 blocking item (register): the specify schema declares `vars: pm_seat` and HEAD bound all
  six PM references to `${pm_seat}`; 0051's reword of `spec.filter-rejections-recorded` swapped
  the var for the literal `product-manager`, and the mints `spec.pm-vets-slate-once` and
  `spec.slate-rulings-users` name the seat literally. Lead verified: 0051 lines 47, 108, 123
  carry the literal; the view's `${pm_seat}` count fell 7 → 6. Fix list: 0057 from the held
  range, three `reword-rule` ops restoring `${pm_seat}`, text otherwise verbatim; re-stamp,
  validate, re-emit; the render stays byte-identical.
- Lead's fold into 0057, disclosed to the grader: the grader's non-blocking note on
  `spec.slate-home`'s literal "`mochiko-cli template spec`" where `${spec_schema}` is declared —
  same defect class, same migration, render unchanged.
- Grader pre-pass, first-hand: validate 0 rejecting · 113 advisory · clusters 0 · 185 suppressed ·
  1196 scanned; committed views equal a fresh emit. Payloads over ledger budget, carried into
  units 5–8: authoring-user-stories 14,445 / 13,444 · review-specifications 18,105 / 16,174 ·
  authoring-requirements 12,948 / 12,373 · authoring-feature-map 22,783 / 22,323; specify payload
  32,387, unbudgeted. Other non-blocking notes: a pre-existing 11-of-10 Intent line count in the
  spec skeleton (0046 carry); the contract suite's specify floor set must gain
  `spec.fail.story-outside-slate` (P3's unit 9).
- Fix round opened to P1 (content-pinned order). The same grader re-reads only 0057, its view
  diff and the validate output on resume.

## 2026-10-10 — 0057 landed (unit 1 fix), lead-verified; grader resumed for round 2

- `plugins/mochiko/migrations/0057-specify-var-bindings.yaml`: four `reword-rule` ops on
  `command/specify` — the three `${pm_seat}` restorations and the lead's `${spec_schema}` fold
  (backticked form as `spec.deliverable`, 0003:96). `spec.artifact-home`'s pre-wave literal left
  alone. No stamped file edited; 0058 held.
- Lead pulled: tree delta is exactly `?? plugins/mochiko/migrations/0057-specify-var-bindings.yaml`;
  `migrate status` → `sequences 1..57 (55 migrations)` · `state sha256:603bb294…` · `87 documents
  · 1196 rules`; validate `0 rejecting · 113 advisory` · `clusters: 0` · `allowlist-suppressed
  edges: 185`; the seven specify blocks diff empty against the pre-0057 capture (28,405 bytes
  both); the view carries `${pm_seat}` nine times and no literal `product-manager seat`; views ≡
  replay. The only advisory movement: `budget · command/specify · 71 rules · 15642 → 15616` —
  that advisory counts unsubstituted rule text (−26 = 3×5 + 11), not the ledger measure.
- `g2-gate-grader` resumed for the unit 1 round-2 read: 0057, its view diff, the pre-pass.

## 2026-10-10 — P2 plan: FAIL on item 3 — second consumption of the re-plan bound, to the user

- `g3-plan-grader-p2`: `PLAN GRADE: P2 prose · FAIL · 3: two on-trust script claims, both false
  on read.` Items 1, 2, 4, 5, 6 PASS; 7 no finding. Advisory: strips/ledger/CHANGELOG edits
  carry no rung tag; the Independent Test placeholder may read as duplication of
  `authoring-user-stories.slate-fidelity` at the artifact gate.
- Claim A (§3.3): "`validate-requirements.py` … no check reads it" — false. Lead verified:
  `plugins/mochiko/skills/authoring-requirements/scripts/validate-requirements.py`
  `check_rfc_keywords` (line 134) and `check_tech_agnostic` (line 152) scan `req['text']`, which
  under the reworded `authoring-requirements.fr-format` now ends in `(US-<n>-<slug>)`; a story
  slug holding a `BANNED_TERMS` substring (`queue`, `worker`, `class`, `hook`, `module`, `rust` in
  "trusted") would false-flag the FR, and one holding `may`/`optional`/`required` could mask a
  missing RFC keyword. The script is outside P2's declared write set.
- Claim B (§3.2): "`validate-user-stories.py` reads headings only" — not in P2's reads; the script
  parses header, priority, the Why line, the Independent Test (≥ 20 chars) and Given/When/Then.
  P2's conclusion (the script never reads the SKILL.md format block) stands once read.
- The wave's one shared re-plan was spent by P1; this FAIL is the second consumption of the bound
  — the disposition is the user's (`review-seat-plan.approval-is-the-leads`): re-plan again,
  re-staff, or narrow the scope. Also for the user: whether to widen P2's set to the checker
  script (strip the citation before its checks, strip entry, graded in unit 7) or leave the
  script and record the false-positive risk. Surfaced to the user; seats hold.

## 2026-10-10 — gate audit unit 1, round 2: PASS

- `audit: 0051 + specify view diff + allowlist row · g2-gate-grader · opus · 3 files · 2 rounds · 0 blocking`
- Round-2 block appended to `reports/audit.md` ("Unit 1, round 2 — 0057 re-audit"); round-1
  blocks unchanged. Grader verified first-hand: validate 0 rejecting · 113 advisory · clusters 0 ·
  185 suppressed · 1196 rules; status `sequences 1..57 (55 migrations)` · `sha256:603bb294…`;
  the full specify render byte-identical between a 0056 replay and the current state (27,999
  chars); committed views equal a fresh emit; 0001–0056 byte-identical to its round-1 copies;
  `${pm_seat}` covers all nine PM references; the `${spec_schema}` fold clears its round-1 note.
- Units 1–3 are now all PASS. Schema content frozen; the grader holds for units 4–9.

## 2026-10-10 — P3 plan round

- P3 plan received (plan-only; tree unchanged bar P1's 0057, landed mid-plan — lead's diff).
  Disclosed: the Explore (haiku) locate sweep returned no handback; P3 re-ran it itself, the
  exhaustive locate being `cargo test -p mochiko-cli --no-fail-fast` (six tests red today, the
  full sweep a seventh). P3 found three frozen sites the lead's grep missed: the fail-node asserts
  (36 → 37) in both census tests, the sequence list at `fidelity.rs:174-201`, and the two spec
  template fixtures (`tests/fixtures/template/spec.{producer,check}.txt`, recaptured by command).
- Write set: `fidelity.rs` · `validate.rs` · `matrix_similar.rs` · the two spec fixtures ·
  `evals/contract/run.py` · `evals/contract/README.md` · `evals/plan/specify/{observable.yaml,
  evals.json, preregistration.md}` · `evals/review-specifications/{rules.json, rekey.md}`.
  Partition of the 19 new ids: 15 observable · 4 out-of-instrument (`spec.own-rules-win`
  precedence; `spec.handed-in-slate` and `spec.floor-obligated-rows` conditional — no golden
  plants them; the fail). `expected-skills.json` untouched (35 rows, 0 mismatches). Counts from
  the tool: command 381 · skill 815 · total 1196 · command floors 120 · fail set 37 · command
  family similarity tuple (381, 17_989, 0, 58) · corpus scanned 1196 / scored 200_540.
- Lead rulings on P3's questions: Q1 `evals/plan/README.md:45` ("specify 39 of 52", stale since
  2026-09-19) added to the set, one figure, a disclosed widening · Q2 the two pre-existing
  `evals/review-specifications/` gaps disclosed only · Q3 the HEAD-log extraction lives in the
  scratchpad, deleted after · Q4 the gate grader's unit 9 carries the independent non-author code
  review of the crate test edits (`rust-cli.md`: author≠grader extends to code); no separate
  reviewer seat.
- Plan sent verbatim to a fresh generic peer `g4-plan-grader-p3` (`model: opus`,
  `review-seat-plan` render pasted) with §3.9, the write sets and the rulings. A FAIL would be a
  further consumption of the re-plan bound, routed to the user with P2's.
- `g4-plan-grader-p3`: `PLAN GRADE: P3 tests · PASS · —`. Items 1–6 met (item 1 read with the Q1
  widening); checked first-hand: the six red tests and their left/right values, the C(n,2) kind
  deltas (2,276 and 8,488), fixture recapture reproduces the feature-entry fixtures byte-identical,
  `Expected["specify"]` 17 ids, README:273 the only moved figure, the kit partition 19 added / 0
  removed / 3 changed, `rules.json` the id inventory. Its Explore (haiku) also returned no
  handback. Advisories folded as lead rulings in the approval: A1 `spec.floor-obligated-rows`
  may bind in s2 (failure recovery under a ratified floor) — P3 re-reads and places it, disclosed;
  A2 FEAT-004 is a possible duplicate, FEAT-005 the real one; A3 the `rules.json` entry carries
  `section:`; A4 the `evals/.work/` evidence dirs are named as tool output.
- Approval sent to P3 with the build open.

## 2026-10-10 — P3 build landed (lead-verified, not graded); unit 9 opened

- 13 files ` M`: `fidelity.rs` · `validate.rs` · `matrix_similar.rs` · the two spec template
  fixtures (recaptured by CLI) · `evals/contract/run.py` · `evals/contract/README.md` ·
  `evals/plan/README.md` · `evals/plan/specify/{observable.yaml, evals.json, preregistration.md}`
  · `evals/review-specifications/{rules.json, rekey.md}`; `git diff --stat`: +233/−54. No crate
  source, no `Cargo.toml`.
- Lead pulled: `cargo test --all` every binary `test result: ok`, 0 failed; `cargo fmt --all
  --check` clean; `cargo clippy --all-targets -- -D warnings` clean; `check-rubric specify` →
  `rubric OK: 56 observable, 15 out-of-instrument, 71 total`. P3 reports `cargo audit` clean, the
  full similarity sweep green (48 passed), `converted-shape` red-then-green (`18 ids`), host-only
  7/7, `check-fixtures` OK, partition 19 added / 0 removed / 3 changed, `evals.json`
  args/fixture/control_prompt byte-equal to HEAD.
- A1 disposition: `spec.floor-obligated-rows` moved to observable (record lines 266 and 726 gloss
  the floor's "recovery" as failure recovery; s2 is retry work under a ratified floor), so the
  partition is 56 · 15 · 71 (planned 55 · 16 · 71); knock-ons in the preregistration figures,
  `evals/plan/README.md:45` ("specify 56 of 71") and s2's expected output. Raw-vs-pinned runs
  confirm P1's one-out, one-in suppressed-edge claim (row :394 left at 0.61; the D5 pair arrived
  at 0.87). Deviation disclosed: `rules.json` `when` written as the string form the file uses.
  Tool output under gitignored `evals/.work/` (named in P3's report). Pre-existing, not repaired:
  a comma in observable.yaml's `spec.filter-disagreement-escalates` why parses as an extra YAML
  key; the two `evals/review-specifications/` gaps.
- `g2-gate-grader` resumed on unit 9 (test and kit content + the crate code review), P3's files
  frozen. Units 4–8 wait on P2.

## 2026-10-10 — gate audit unit 9, round 1: PASS

- `audit: crate fixtures + contract expectations + specify kit · g2-gate-grader · opus · 13 files · 1 rounds · 0 blocking`
- Block appended to `reports/audit.md`. Pre-pass run by the grader: `cargo test --all` 765 passed
  / 0 failed; full similarity sweep 48 passed, both detector pins ok; fmt, clippy, audit clean;
  status 1..57 / 1196 rules; validate 0 rejecting / clusters 0 / 185 suppressed; check-rubric
  56 · 15 · 71; check-fixtures OK; partition 3 changed / 0 removed / 19 added; converted-shape 1/1
  on the release build (FILTERED, not gate 6). Re-derived independently: both fixtures equal a
  fresh `template spec` render; the six command floor pins sum to 120; every per-kind pair delta
  matches n(n−1)/2 (+2,276 command family, +8,488 corpus); empty-allowlist sweeps over 1..50 and
  1..57 both give 78 clusters with exactly one edge out and one in; no crate source or
  `Cargo.toml` touched; nothing dropped from the partition, `Expected`, or the asserts. The crate
  code review (`rust-cli.md`) is carried by this unit.
- Non-blocking notes: the `spec.floor-obligated-rows` why says "s2 only" on s1's no-accounts
  ruling — that rules out an auth row, not a failure-recovery row on s1 (the why reaches no judge;
  partition unaffected); allowlist row :394 now suppresses no edge but stays legal; P3's disclosed
  pre-existing defects remain unrepaired.
- Units 1–3 and 9 PASS. Open: units 4–8 (P2), blocked on the user's ruling on the re-plan bound.

## 2026-10-10 — user's rulings on P2: one revision allowed; the FR checker joins P2's set

- The user ruled "as recommded for both" (their word): (1) P2 gets one revision — the second
  consumption of the re-plan bound, granted by the user, not the run; a further FAIL halts to
  the user. (2) P2's write set widens by
  `plugins/mochiko/skills/authoring-requirements/scripts/validate-requirements.py`: strip the
  trailing `(US-<n>-<slug>)` citation from each requirement's text before every check that scans
  `req['text']`, with a `[v0.119.0]` strip entry in `.mochiko/strips/authoring-requirements.md`
  and a failing-first fixture test; graded in unit 7 with the pair.
- Revision order sent to P2 (delta only; the grader's fix list verbatim; advisories folded: rung
  tags on the strip/ledger/CHANGELOG edits, the Independent Test placeholder trimmed or kept at
  P2's call, stated). `g3-plan-grader-p2` re-grades the delta.

## 2026-10-10 — P2 plan round 2: PASS; build opened

- P2's revision (delta only; tree unchanged — lead's diff clean at 33 porcelain lines): both
  scripts read in full; the fix lands at the single extraction point of `validate-requirements.py`
  (new `STORY_CITATION` constant after `RFC_KEYWORDS`; line 68 strips a trailing `(US-…)`
  citation before `req['text']` is stored, so `check_rfc_keywords`, `check_tech_agnostic` and
  `check_outcome_focus` all see the requirement's own words); two red-then-green fixtures (false
  `queue`/`rust` flags; a masked missing RFC keyword) and a 32-spec oracle byte-identical before
  and after; the Independent Test placeholder trimmed to the seed's expansion plus where the
  provisional mark sits (the script finds the field by the exact `**Independent Test**:` label);
  rung tags on strips/ledger/CHANGELOG; CHANGELOG names 0057 and the script fix.
- `g3-plan-grader-p2`: `PLAN GRADE: P2 prose · PASS · —`. Replayed the red/green and the oracle
  in memory itself (0 diffs); verified the user's ruling against this log. Advisories: pin the
  oracle set (its count differed only by a gitignored `evals/.work/` spec); strip field label is
  `**Content:**` per the README grammar; the "Provisional:" form is body guidance the rule does
  not state — the gate grader judges line-form-for-parser. Lead folded the first two as rulings
  in the approval.
- Approval sent to P2 with the build open (round-1 plan + revision, content-pinned; §9 order;
  §10 stops).

## 2026-10-10 — P2 build landed (lead-verified, not graded); units 4–8 opened

- 14 paths ` M`: `commands/specify.md` · `authoring-user-stories/SKILL.md` ·
  `authoring-requirements/SKILL.md` · `authoring-requirements/scripts/validate-requirements.py` ·
  `authoring-feature-map/SKILL.md` · the router row · four strip files (five `[v0.119.0]`
  entries; authoring-requirements carries two) · four ledger rows · `README.md` ·
  `ARCHITECTURE.md` · `CHANGELOG.md` (0.119.0 entry, three `[lead: …]` placeholders at lines 49,
  67, 71). `git diff --stat` over those surfaces: +157/−37. `review-specifications/SKILL.md`,
  its strip and `spec-template.md` untouched as ruled.
- Lead pulled: tree delta exactly the 14; `description:` diff lines 0 on all four skills;
  validate unchanged (`0 rejecting · 113 advisory`); the measure script reproduces P2's table —
  payloads authoring-user-stories 14,576 (budget 13,444, +1,132) · review-specifications 18,105
  (16,174, +1,931) · authoring-requirements 13,017 (12,373, +644) · authoring-feature-map 22,856
  (22,323, +533); specify render 27,999, body 4,667; the script parses, its directory holds only
  the script; the strip entries present.
- P2's stepped prefix replays attribute each render delta to one migration (0051 specify +7,881 ·
  0052 +717 · 0053 +747 · 0054 +74 · 0055 +78 · 0056/0057 0); the 1..50 replay reproduces the
  ledger's [v0.118.0] payloads exactly. Script red/green: fixture 1 exit 1 (queue · rust ·
  queue) → exit 0; fixture 2 exit 0 (masked) → exit 1 "Missing RFC 2119 keyword"; oracle 32
  tracked fixture specs byte-identical (combined digest d553bcf3de93481d both runs).
- Deviations disclosed: "above" not "below" in the format-block strip entry; no rung tag written
  (no field for one); the Goal paragraph tail re-wrapped; a transient `__pycache__` deleted; the
  ledger rows name the prefix-replay method. Scratchpad fixtures and replay copies deleted.
- `g2-gate-grader` resumed on units 4–8 with the named overages, the fixture texts, and the
  editor's disclosures.

## 2026-10-10 — gate audit units 4–8, round 1: all PASS — every unit of wave 1 cleared

- `audit: specify command pair · g2-gate-grader · opus · 3 files · 1 rounds · 0 blocking`
- `audit: authoring-user-stories skill pair · g2-gate-grader · opus · 4 files · 1 rounds · 0 blocking`
- `audit: review-specifications skill pair · g2-gate-grader · opus · 3 files · 1 rounds · 0 blocking`
- `audit: authoring-requirements + authoring-feature-map skill pairs + validate-requirements.py · g2-gate-grader · opus · 7 files · 1 rounds · 0 blocking`
- `audit: strips + ripple + budget ledger + changelog · g2-gate-grader · opus · 9 files · 1 rounds · 0 blocking`
- Blocks in `reports/audit.md` under one shared pre-pass section and one heading per unit.
  Pre-pass by the grader: validate 0 rejecting · 113 advisory · clusters 0 · 185 suppressed;
  status 1..57 · 1196 rules; description diff lines 0; payloads by the canonical snippet equal
  the editor's and the lead's (14,576 / 18,105 / 13,017 / 22,856; specify 32,666 unbudgeted).
  **All four overages ruled HOLDS** — each wave delta a ruled new obligation, none restored
  playbook prose. Script: fixture 1 HEAD exit 1 (queue/rust/queue) → current exit 0; fixture 2
  HEAD exit 0 → current exit 1 on `rfc_keywords`; all 47 tracked `evals/**/spec.md` byte-identical
  before and after (the 32 with FR lines: 19 × exit 0, 13 × exit 1). The plan grader's unit-5
  advisories judged not defects (the placeholder names the field's source and mark form; the
  validator's exact-label regex makes the in-field mark necessary). The ARCHITECTURE.md hand edit
  legal (store scaffold-only).
- Non-blocking notes: the review-specifications Procedure's layer list does not name the slate;
  the router's PM row and ARCHITECTURE's "frame brief · derivation brief" label do not name the
  slate-close vet; pre-existing — the authoring-feature-map description table row says 495 against
  598 measured, the ARCHITECTURE header says v0.110.0; the re-wrap leaves one short line.
- Audit tally: 9 units · 10 rounds (unit 1 took two) · 1 blocking found and fixed (0057) · one
  grader seat for the wave. Next: the lead's close writes (ledger ruling word, CHANGELOG
  placeholders, manifests 0.119.0), the §5 gates, the §6 landing.

## 2026-10-10 — wave 1 closed: gates green, v0.119.0 landed

- `floor: clear · seats: lead (inline) / p1-schema / p2-prose / p3-tests / g1-plan-grader /
  g3-plan-grader-p2 / g4-plan-grader-p3 / g2-gate-grader` — every producer on a peer-graded plan,
  every unit graded by a seat that authored nothing in it; the lead's own writes were the close
  ritual, the plan, this log, the CHANGELOG placeholders, the ledger ruling word, and the manifests
  (mechanical execution of the user's rulings and the grader's verdicts). All seats `opus`, explicit.
- plans: P1 FAIL(4,5,6) → PASS · P2 FAIL(3) → PASS (the second re-plan consumption, user-granted
  2026-10-10, with the checker widening) · P3 PASS(1). Haiku Explore handbacks failed for P2, P3
  and g4; each re-ran its sweep itself and disclosed it.
- audit lines (`reports/audit.md`):
  - `audit: 0051 + specify view diff + allowlist row · g2-gate-grader · opus · 3 files · 2 rounds · 0 blocking`
  - `audit: 0052–0055 + four skill view diffs · g2-gate-grader · opus · 8 files · 1 rounds · 0 blocking`
  - `audit: 0056 + spec template view diff · g2-gate-grader · opus · 3 files · 1 rounds · 0 blocking`
  - `audit: specify command pair · g2-gate-grader · opus · 3 files · 1 rounds · 0 blocking`
  - `audit: authoring-user-stories skill pair · g2-gate-grader · opus · 4 files · 1 rounds · 0 blocking`
  - `audit: review-specifications skill pair · g2-gate-grader · opus · 3 files · 1 rounds · 0 blocking`
  - `audit: authoring-requirements + authoring-feature-map skill pairs + validate-requirements.py · g2-gate-grader · opus · 7 files · 1 rounds · 0 blocking`
  - `audit: strips + ripple + budget ledger + changelog · g2-gate-grader · opus · 9 files · 1 rounds · 0 blocking`
  - `audit: crate fixtures + contract expectations + specify kit · g2-gate-grader · opus · 13 files · 1 rounds · 0 blocking`
- Lead's close writes: four ledger rows "argued" → ruled-HOLDS citing the audit (units 5–8);
  CHANGELOG `[lead: …]` placeholders filled (AM-5 exception line, the ruling word, the gates
  paragraph); `plugin.json` and `marketplace.json` 0.118.0 → 0.119.0.
- Gates (§5), on the bumped tree: `migrate validate` 0 rejecting · 113 advisory · clusters 0 · 185
  suppressed; `migrate status` 1..57 · 1,196 rules · `sha256:603bb294…`; views ≡ replay (fresh
  emit diffs clean; the render header reads `plugin 0.119.0`); `cargo test --all` 765 passed ·
  fmt · clippy · audit clean; `MOCHIKO_FULL_SIMILAR=1` sweep 48 passed; `gitleaks detect --no-git`
  no leaks (3.79 GB, `target/` included); `mochiko-cli ids --check plugins/mochiko` 0 bare · 0
  drift — the CI-equivalent run with `scripts/ids-check-excludes.txt` reports 17 bare, all the
  blind angle map's own `D1`–`D6` angle labels quoted in `record.md` (line 300 names them as the
  map's ids; advisory, a checker false positive, left); **contract suite 97/97, 97 ran, none
  skipped** (exit 0; 7 host cases on the release binary, 90 in the `claude-mochiko` Docker
  sandbox; 327 measurements recorded, none asserted — tally reconstructed from the 97
  `verdict.json` files after the lead's `tail` cut the summary line; two extra evidence dirs,
  `gate-skew` and `gate-unsound`, carry no verdict file by design).
- Landing (§6): `DECISIONS.md` 2026-10-10 row stamped built at v0.119.0 · BACKLOG: the build item
  to `.mochiko/archive/backlog-trail.md` (the shell append was denied by the artifact-home hook —
  third deny this session on a declared home, landed by Edit), replaced by the residual "First
  ai-fileops specify run" item carrying the build's non-blocking follow-ups; the convergence item
  stays; open count 103, unchanged (one out, one in), the count watch still tripped — groom still
  owed and offered · ROADMAP Now row touched (built, trail, remaining) · the record's Status line
  and *Acceptance and landing* stamped built · the index entry stamped built with the build links.
- Not done here, the user's: commit and push (never run by the run); upgrading the installed
  plugin copy and the ai-fileops copy, then the first `/mochiko:specify` run (D7's test).
