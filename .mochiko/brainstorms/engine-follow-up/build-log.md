# Build log — `engine-follow-up` (the engine follow-up build)

Append-only. One `##` entry per event, 60 lines at most.

## 2026-10-15 — build opened

- Record accepted 2026-10-15; its close ritual ran and was committed (`c840469`, "Record the
  engine follow-up session"). This build carries `BACKLOG.md` *Engine follow-up build*:
  `engine-follow-up` D1–D10 as review-amended and the 24 defaults, one combined build started now
  (D9-one-build-now, `Contested`).
- Branch `engine-follow-up-build` from `main` @ `cbe3f5f` (the merge of PR #45, v0.120.0), cut by
  the user's word ("commit on a new branch and start implemtaion on that branch").
- Baseline before the first dispatch: `mochiko-cli migrate status` → sequences 1..84 (76
  migrations), state `sha256:58f165c8…`, 93 documents, 1,243 rules; `migrate validate --report` →
  0 rejecting, 116 advisory, clusters 0, 184 allowlist-suppressed edges; plugin 0.120.0; binary
  0.4.0 (grammar 1..2).
- Render sizes (bytes, all sections): implement 61,086 — `impl.sec.tools` alone 27,651, near the
  ~30k inline ceiling the 0.120.0 build noted, so implement's additions go to its other sections
  wherever the rule's kind allows · architecture 28,394 · specify 27,560 · brainstorm 19,074 ·
  feature 18,780 (`feat.sec.tools` 5,264) · setup 17,941.
- The lead's own writes so far: this log (a session artifact). No plugin primitive touched by the
  lead; every primitive edit goes through the sound loop (a producing seat on a peer-graded,
  lead-approved plan; one gate grader per wave; the user rules).
- Open question to the user before the wave plan is written: the wave split and the bump.

## 2026-10-15 — wave split ruled

- The user ruled the lead's recommendation ("as recommded"): one bump, 0.121.0, built in three
  waves, the same shape as 0.120.0.
- W1 — the shared asking line (the engine subset outside a stage, D10) · `/mochiko:feature` on
  the engine (D2, D3, D7, defaults 6–14, 23–24) · setup's thinking-stage pointer (default 17).
- W2 — implement's later moments (the D4/D10 implement rows, D5, D6, D8, defaults 12, 24 on the
  implement side).
- W3 — the rest of the table (specify, setup, architecture-desk rows) · the carry-forward
  readers · the 0.120.0 residuals (default 21) · then the bump, `CHANGELOG.md`, marketplace sync,
  the release gates and the contract suite (`sbx login` is the user's).
- No bump at any wave close; each wave closes on its own gate audit all PASS.

## 2026-10-16 — W1 plan written, P1 planned

- `wave1-feature.md` written (287 lines): sequences 0085–0089 (0090–0091 held), seats P1 schema ·
  P2 prose · P3 tests and kits, eight gate units.
- P1 (persona-less `general-purpose`, `model: opus`) dispatched plan-only. Tree snapshot before
  and after: identical (only this log and the wave plan untracked) — not dirty. The seat wrote
  scratch only, outside the repo: a stamped dry-run copy of the log with 0085–0089.
- The seat's claims (to be tested at the peer grade and the gate): dry run 0 rejecting, 118
  advisory (116 before), 0 clusters after two allowlist rows; 0089 lands (the entry's work-row
  grammar has no place for the pointer); no engine edit, no slot, no grammar or crate change, no
  floor or fail lowered; feature render 18,780 → 24,969 bytes, setup 17,941 → 18,287.
- Deviations disclosed by the seat: its five `Explore` haiku dispatches each ended "without
  delivering a report through SubagentHandback", so it did the locate reads by direct tool calls
  · `common.*` blocks cannot carry `class:` (`validate.rs:1845`), so the class sits on the
  extender · `feat.asking-line` placed in `feat.sec.ways-of-working`, not boundaries (that section
  holds floors only) · `reword-rule` carries no `disposition` field (`migration.rs:769`), so each
  earlier ruling is named in the file's `intent`.
- Transport: the seat's final message reached the lead truncated in the idle notice. Asked to
  copy it to scratch, the seat's write was stopped partway by a safety classifier and it declined
  to retry. The lead recovered the complete returned text (563 lines) from the seat's own
  transcript into scratch, unchanged; that copy is the plan the peer grades.

## 2026-10-16 — P1 plan grade: FAIL, re-plan sent

- Peer grader (fresh `general-purpose`, `model: opus`, the `review-seat-plan` render by file,
  verbatim, both lines on all 7 blocks): FAIL · 3 blocking · 6 advisory. Item 1 scope fidelity
  PASS (every text checked against its card) · 2 write set PASS · 3 reads named FAIL (the
  `<date-slug>` claim unsourced; true at `home.rs:276-290`) · 4 rung claims FAIL (none present) ·
  5 stops and hand-offs FAIL (the gate hand-off, the fix-round hold and the bounds unnamed) · 6 no
  self-clearing PASS · 7 size no finding. The grader's own two `Explore` haiku dispatches also
  returned no handback; it read directly.
- Re-plan bound consumed: 1 of 1 on the counter P1, P2 and P3 share (wave plan §0) — a second
  consumption by any producer goes to the user.
- Order sent to P1 (content-pinned, the three fixes and six advisories quoted): return an addendum
  only, under ~150 lines, amending the recovered plan; any changed YAML op shown in full with its
  dry-run hash. The same grader re-grades it, resumed (`review-seat-plan`: "the same grader seat
  re-grades the revision").

## 2026-10-16 — P1 re-plan PASS, approved, build opened

- P1's addendum (112 lines; the idle notice truncated it again, recovered unchanged from the
  seat's transcript): the three fixes plus all six advisories taken — Sibling files dropped from
  the template, `feat.ruling-places` anchored D10, the allowlist reasons citing
  `analysis-iterative.caller-slots`, a caller-slot map for gate unit 5. Four YAML ops changed.
- Re-grade by the same grader, resumed: PASS · 0 blocking · 1 advisory (the
  `feat.card-blind-list` rung claim also fixes the seat type, which is no engine slot — honesty
  only; no YAML change). The grader reproduced the dry run itself: 0 rejecting · 118 advisory ·
  0 clusters · 186 suppressed; the 0086/0087 hashes match.
- The lead approved. The approved plan is the two recovered files, pinned by sha256 in the resume
  order (`aff6561e…` full plan, `ef7dfbec…` addendum; the addendum wins where they differ). Tree
  snapshot taken before the resume.

## 2026-10-16 — P1 built; P2 and P3 dispatched plan-only

- P1's build report (truncated in the idle notice, recovered from the transcript): five files
  written and stamped, every hash matching the approved plan (0085 `77a78fea…`, 0086 `5b1e07b7…`,
  0087 `897d1d65…`, 0088 `87d41e99…`, 0089 `7b039115…`); two allowlist rows; six views
  re-emitted (five changed, `templates/feature-visit` new).
- The lead's own checks: `git status` shows exactly P1's declared write set beside the two session
  files · `migrate validate` → `0 rejecting · 118 advisory`, `clusters: 0`, `186` suppressed ·
  `migrate status` → `sequences 1..89 (81 migrations)`, `94 documents · 1254 rules`, state
  `sha256:5ee3c452…` · `template feature-visit` 3,280 bytes.
- Deviations P1 disclosed: `cargo test --test views` fails 3 of 11 on document-count pins 93 vs
  94 (`views.rs:278`, `:356`, `:464`) — P3's write set, P1 halted there correctly; one cluster,
  not two, before the allowlist rows (a stale figure in its plan); the template size figure in
  the addendum carried the scratch log path in its footer (content equal, hash proves it).
- The lead's §3.8 read of the built feature render: every row has a carrier — D2 (visit_kind,
  card-set moment, `feat.card-stage`, `feat.thinking-stage`), D3 (`feat.card-unit`), D7 (0086,
  `feat.visit-file`, `feat.card-writer`, three rewords, 0089), D10 (`common.asking-line`,
  `feat.asking-line`, `feat.ruling-places`), defaults 8–11, 13, 17, 18, 23, 24. Noted for the
  gate: `feat.card-unit` carries no `when: visit_kind=card`. This read clears nothing.
- P2 (prose) and P3 (tests and kits) dispatched plan-only in parallel — persona-less
  `general-purpose`, `model: opus`, disjoint write sets per §2, tree snapshot taken; each brief
  says the re-plan bound is spent, so a failed plan grade goes to the user.

## 2026-10-16 — P2 planned; peer grade dispatched

- P2's plan (401 lines; truncated in the idle notice, recovered unchanged from the transcript,
  sha256 `00d9f64a…`). Tree after planning equals the snapshot — not dirty. Its three `Explore`
  haiku dispatches also returned no handback; it read directly (the third seat this wave to hit
  it).
- The plan: ten paths — `feature.md` (default 14's sentence, both old clauses kept word for word,
  one clause added pointing at `feat.card-stage`) · `analysis-iterative` description (594 → 713
  characters) and Overview · router `:66` · `ARCHITECTURE.md:373` (still hand-maintained legacy:
  the store is scaffold-only) · one supersession entry in `strips/feature.md` (the line is
  protected — the [v0.97.0] kept set) plus wave-context comments in four strip files · three
  ledger rows (analysis-iterative payload +212, description +118, authoring-feature-map +148 on
  its standing +533) · `CHANGELOG.md` `## [0.121.0]` W1 part. `setup.md`, the authoring-feature-
  map body and `README.md` untouched, each with its reason.
- Two corrections to the lead's plan, raised as questions: Q1 — §3.6's "`feature`'s and `setup`'s
  payload rows" do not exist (commands are unbudgeted, ledger `:560–566`; a budget is never
  invented, `:16`), so it mints none · Q2 — `feature.md` holds no "card on the entry" line and no
  Not-done count (`:64–67` cites the pin). The lead reads both as right; answered at approval.
- Fresh generic peer (`general-purpose`, `model: opus`) dispatched to grade it.

## 2026-10-16 — P3 planned; peer grade dispatched

- P3's plan (254 lines; truncated in the idle notice, recovered unchanged, sha256 `30284191…`).
  Tracked tree after planning equals the snapshot. Disclosed: a `run.py` import created a
  gitignored `__pycache__` file, which the seat deleted; its one `Explore` haiku dispatch returned
  no handback (the fourth seat to hit it).
- Red observed now: `cargo test --all --no-fail-fast` exits 101, 7 failed (`fidelity.rs:185`,
  `:853` · `matrix_similar.rs:947` · `validate.rs:1111` · `views.rs:278`, `:356`, `:464`); the full
  similarity run fails at `matrix_similar.rs:1270`. A scratch census probe read the hidden
  figures: new log 94 docs · 412 command rules · 842 skill rules · 1,254 total · 129 command
  floors · 40 fail nodes · 16 templates. P1's claims confirmed, and seven places it missed found.
- The plan: twelve paths, dry-run in a scratch mirror where `cargo test --all` (765 passed),
  the full sweep, `fmt --check` and `clippy -D warnings` all exit 0; the `feature-visit` fixture
  pair by the method that reproduces the `architecture-visit` pair byte for byte; `run.py`
  `TEMPLATE_NAMES` and the feature/setup floor sets; the contract README's floor-id count 126 →
  129; both `observable.yaml` files re-keyed (`check-rubric`: feature 57 observable · 4 out · 61,
  setup 34 · 8 · 42); the feature preregistration recounted (never run; the 0.120.0 architecture
  recount precedent), fixing two older problems in it.
- Two points for the lead at approval: `evals/plan/README.md:46` (`feature 48 of 51` → `57 of 61`)
  sits outside §2's P3 row · keeping the preregistration's baseline arms is the seat's lean.
- Fresh generic peer (`general-purpose`, `model: opus`) dispatched to grade it.

## 2026-10-16 — P2 plan PASS, approved, build opened

- P2's peer grade: PASS · 0 blocking · 4 advisory. Raising Q1 and Q2 rather than deciding them
  read as right (ledger `:15–16`, `:564–566`; primitive-edits criterion 3). The grader's four
  `Explore` haiku dispatches also returned no report (the fifth seat).
- The lead approved, the plan pinned by sha256 (`00d9f64a…`), with amendments: Q1 agreed (no
  command budget row) · Q2 agreed (no `feature.md` change for either) · A1 taken —
  `GLOSSARY.md:16` (**visit file**) added to P2's write set: it named `focus.md`, a stale
  pre-existing defect (the architecture desk's file is `visit.md`, 0065), and now misses the
  feature desk's `visit.md`; reworded to cover both, provenance kept and widened · A3 taken — the
  v0.121.0 strip comment and entry together above `strips/feature.md:3`, the cite slip fixed ·
  A2 plan-only · A4 the lead names the audit unit for the authoring-feature-map overage in the
  gate brief.

## 2026-10-16 — P2 built

- P2's build report (truncated in the notice, recovered from the transcript): eleven paths, Edit
  only, plan hash checked first; every declared-home write dry-run through `mochiko-cli check
  --path … --content -` (`allow`) and byte-identical to its dry run. Figures unmoved from the
  plan: analysis-iterative payload 16,067 → 16,279 (+212 over), description 594 → 713 (+118
  over, cap 1,536); authoring-feature-map payload 22,856 → 23,004 (+681 over: the standing +533
  plus +148); feature render 18,507 → 24,641 characters, setup 17,661 → 18,002 (unbudgeted).
- One deviation, disclosed: S1's consumers line also names this wave's `CHANGELOG.md` paragraph,
  which a post-build `git grep` found — an accuracy fix to its own new text.
- The lead's check: `git status` beyond the P1 snapshot shows exactly P2's eleven paths
  (`GLOSSARY.md` the approved twelfth surface, one line); `migrate validate` unchanged at
  `0 rejecting · 118 advisory`.

## 2026-10-16 — P3 plan PASS, approved, build opened

- P3's peer grade: PASS · 0 blocking · 3 advisory. The grader cut its own scratch copy of the
  current tree (P2's prose included), saw the same seven red, applied `plan.diff` clean and got
  `cargo test --all` 765/0, the full sweep 48/0, fmt and clippy 0, both kits green, the static
  contract check PASS. Its two `Explore` haiku dispatches also returned no report.
- The lead approved, the plan pinned by sha256 (`30284191…`), with rulings: `evals/plan/README.md:46`
  added to P3's write set (one line) · the preregistration's baseline arms kept (`794cea8`,
  control `feat.artifact-home`), its pre-wave drift fold-in approved knowingly · A1 taken — any
  red beyond the seven named is a stop · A2 taken — the s1 golden's "a bug card on the entry"
  phrase in `evals/plan/feature/evals.json`, made false by 0087, reworded minimally to the visit
  file (the 591f31e precedent), that phrase only · A3 settled by the arms ruling.
- Tree snapshot (P1 + P2, 25 lines) taken before the resume.

## 2026-10-16 — P3 built; gate audit dispatched

- P3's build report (truncated in the notice, recovered from the transcript): step-1 red was the
  seven named failures exactly (ruling 3 did not trip); fourteen paths — five crate tests, the
  two `feature-visit` fixtures (new), `evals/contract/run.py` and README, `evals/plan/README.md:46`,
  both `observable.yaml`, the feature preregistration, the one `evals.json` phrase. No deviation.
- Its checks: `cargo test --all --no-fail-fast` exit 0, 765/0 · `MOCHIKO_FULL_SIMILAR=1` 48/0 ·
  `fmt --check` 0 · `clippy -D warnings` 0 · `check-rubric` feature 57 · 4 · 61, setup 34 · 8 ·
  42 · `check-fixtures` OK both · the static contract check PASS (129 floor ids, 16/16
  templates). Sandbox suite and `cargo audit` not run (wave close and the bump).
- The lead's check: `git status` beyond the P1 + P2 snapshot shows exactly P3's fourteen paths;
  the only `__pycache__` files are pre-existing (dated 2026-09-19 to 2026-10-11, gitignored).
- The wave's one gate grader dispatched: a persona-less `general-purpose` seat, `model: opus`,
  the eight units of the wave plan §4, the contract rendered first-hand by the seat, the authoring-
  feature-map overage named under unit 3.

## 2026-10-18 — gate grader stopped on a usage limit; resumed

- The gate grader's run ended "You've hit your weekly limit" before `reports/audit.md` was
  written; its scratch pre-pass captures (renders, cargo test, fmt, clippy, a baseline copy)
  survive under the session scratchpad.
- Tree unchanged since the dispatch (39 porcelain lines: 37 wave paths plus the two session
  files). The user said "continue"; the same seat was resumed with the same brief
  (`validation-primitive-edit.one-seat-per-wave` — a fresh seat only if the first is gone), told
  to re-run any pre-pass command whose output it did not capture in full.

## 2026-10-18 — W1 gate audit: all PASS; wave 1 closed

- Report: `reports/audit.md` (510 lines, `report: review`, round 1, the conformance check
  allowed it before the write). Eight units, 0 blocking, no fix round. The outcome lines:

```
audit: unit 1 (schema content · 0085 + common view diff) · w1-gate-grader · opus · 2 files · 1 rounds · 0 blocking
audit: unit 2 (schema content · 0086 + feature-visit template + feature-desk home diff) · w1-gate-grader · opus · 3 files · 1 rounds · 0 blocking
audit: unit 3 (schema content · 0087 + 0089 + feature and authoring-feature-map view diffs + allowlist rows) · w1-gate-grader · opus · 5 files · 1 rounds · 0 blocking
audit: unit 4 (schema content · 0088 + setup view diff) · w1-gate-grader · opus · 2 files · 1 rounds · 0 blocking
audit: unit 5 (command pair · feature) · w1-gate-grader · opus · 1 files · 1 rounds · 0 blocking
audit: unit 6 (command pair · setup; skill pair · analysis-iterative, description and Overview) · w1-gate-grader · opus · 2 files · 1 rounds · 0 blocking
audit: unit 7 (prose primitives and records · router, four strips, ARCHITECTURE, GLOSSARY, budget ledger, CHANGELOG) · w1-gate-grader · opus · 9 files · 1 rounds · 0 blocking
audit: unit 8 (crate tests, template fixtures, contract expectations, plan kits · crate non-author review) · w1-gate-grader · opus · 14 files · 1 rounds · 0 blocking
```

- The grader's first-hand pre-pass: `migrate validate` 0 rejecting · 118 advisory (the two new are
  `visit_kind`'s unused `ruling` and `other`) · views ≡ replay · `cargo test --all` 765/0, fmt and
  clippy clean, the full similarity sweep green · both kits green · all 11 engine ids
  `common.asking-line` cites live in the `analysis-iterative` render.
- Named overages, measured first-hand, all HOLD: authoring-feature-map +681 (a replay without
  0089 isolates its +148; the rest the standing +533) · analysis-iterative payload +212 ·
  description +118 (713, cap 1,536).
- Carried to W2 (the grader's notes): the `feature-visit` template overview and `GLOSSARY.md:16`
  say implement reads the card in `visit.md`; implement's reader lands in W2, so the W2 gate
  re-reads those lines before the bump. The contract suite is the bump gate's.
- The lead's wave-close regression read: `cargo audit` clean (31 dependencies; no dependency
  changed since `c840469`). The sound loop held on every item: three producers planned, graded and
  approved (one re-plan, P1's); one non-author gate grader; no lead-authored primitive.
- Wave 1 closed. No bump. Next: the W2 plan (implement's later moments).

## 2026-10-18 — W2 plan written, P1 dispatched plan-only

- `wave2-implement.md` written (284 lines): sequences 0092–0095 (0095 conditional on
  `review-sufficiency`'s delta-card line; 0096–0097 held), seats P1 schema · P2 prose · P3 tests
  and kits, seven gate units. W1's carried note (the `feature-visit` overview and `GLOSSARY.md:16`
  saying implement reads the card in `visit.md`) is a §3.5 item and a unit-6 read.
- The tools ceiling named as a stop: `impl.sec.tools` 27,651 bytes against the 29,000-byte line
  (the 0.120.0 W3 gate's measure) — new rules go to other sections where their kind allows.
- P1 (persona-less `general-purpose`, `model: opus`) dispatched plan-only; tree snapshot taken (41
  porcelain lines: W1's 37 paths, the build log, the two wave plans and the W1 audit report).
- W1 sits uncommitted in the tree; the commit is the user's call.

## 2026-10-18 — W1 committed; commits delegated for the build

- The user: "yes , commit when you see fit during entire implementation" — the lead commits at
  natural boundaries for the rest of this build; push and merge stay the user's.
- W1 committed as `fccae34` ("Put feature on the shared engine and mint the shared asking line"):
  W1's 37 paths, the build log, both wave plans and `reports/audit.md`. Tree clean after.
- W2 P1 was planning (plan-only, no writes) across the commit; its dirty check now reads against
  the clean tree at `fccae34`.

## 2026-10-18 — W2 P1 planned; peer grade dispatched

- W2 P1's plan returned (538 lines; truncated in the notice, recovered unchanged, sha256
  `dd707710…`). Tree after: only the lead's own build-log edit — not dirty.
- The plan, dry-run: four files land (0092 card order · 0093 asking line, ruling places,
  promises-kept table, defaults 1–2 · 0094 the feature visit read in every scope, the outside
  need's note, D8's added input · 0095 `review-sufficiency`); six new implement rules (one floor
  fail), one moment, nine implement rewords; no template change (the `tasks` header takes the
  order line, conformance checks nothing above `## Overview`). Dry run `0 rejecting · 118
  advisory · 0 clusters · 185 suppressed`; `impl.sec.tools` 27,651 → 28,354 bytes (646 under the
  29,000 line), no new rule in tools. D6 lands as a new must beside the floor, the floor untouched.
- Four scope points raised for the lead, beyond the wave plan's allocation: L1 — D8's skill side,
  two `testing-gap-finding` floors (the inclusion list and the two-message dispatch) reworded to
  admit the user's quoted lines, else D8 contradicts the closed fence (the 0.120.0 0078/0082
  precedent) · L2 — `review-sufficiency.fence-read-set` widened to the card in the visit file ·
  L3 — `impl.desk-offer` qualified to the architecture desk's file · L4 — default 24 on
  `impl.delivered-territory-routing` too (lean: not included).
- Risks it flagged, not resolved: the order line's first `tasks.md` write must carry the
  template's required headings or the hook denies it · a pre-existing tension between
  `review-sufficiency.fence-never-reads` ("never … **TEST:** cases") and the delta card's
  **TEST:** bundle · two allowlist rows stop firing (left in place).
- Fresh generic peer (`model: opus`) dispatched; asked for its reading of whether L1 stays inside
  D8 ("the rest of the fence and the two-message dispatch are unchanged") or lowers a floor.

## 2026-10-18 — W2 P1 plan grade: FAIL; re-plan sent with the L rulings

- Peer grade: FAIL · 2 blocking (item 1) · 4 advisory; the dry run reproduced exactly (hashes,
  `0 rejecting · 118 advisory · 0 clusters · 185`, `94 documents · 1260 rules`, tools 28,354).
  Blocking: 1a — the `cycle-checkpoint` moment still says "land as one batch", the phrase the plan
  judged false on `impl.escalation-batching` · 1b — `impl.feature-visit-input` reads the card part
  only on delta/lane runs, but default 12 (RB3) and D7 read the card on every path (growth-row
  cards dispatch in selection scope).
- The grader's L1 reading: inside D8, no floor lowered — pure insertions of D8's one input, every
  other limb unchanged, the widening the user's explicit Q8 ruling over "fence as built", the
  0078/0082 precedent.
- The lead's rulings, sent with the re-plan: L1 include (the gate grades `testing-gap-finding` as
  its own skill-pair unit) · L2 include, with the grader's A1 resolved in the plan — word the
  read so it cannot contradict `review-sufficiency.fence-never-reads` ("never … **TEST:**
  cases"), or stop and raise it to the user if the per-card form truly needs the card's **TEST:**
  cases · L3 include · L4 include (default 24's words are general and W1's `feat.carry-forward`
  expects the note on any outside need; one clause, measured) · A2 moot · A3 no template change,
  the rung claim citing `analysis-iterative.contract-inside` · A4 the mock scoped off delta/lane.
- Re-plan bound consumed: 1 of 1 for W2 (the counter P1, P2 and P3 share). A second consumption
  goes to the user.

## 2026-10-18 — W2 P1 addendum returned; re-grade dispatched

- Addendum (161 lines; recovered unchanged, sha256 `1123e3b6…`), tree clean but the lead's log.
  Six ops changed: a `cycle-checkpoint` `set-moment` and an `impl.finding-severity-routing`
  reword ("joins the checkpoint batch" false on the same reading — 1a) · `impl.feature-visit-input`
  reads the cards in every scope (1b) · `impl.delivered-territory-routing` carries the run's note
  (L4) · `review-sufficiency.fence-read-set` reads "its scope and acceptance criteria, a bug's its
  reproduction, never its **TEST:** cases" (L2 — the per-card form's
  `clause-testable-criteria` grades a stateable oracle, not a written test) · `impl.card-order`
  scopes the mock off delta/lane (A4). Dry run `0 rejecting · 118 advisory · 0 clusters · 185`;
  `impl.sec.tools` 28,443 bytes (557 under the line).
- A risk the seat flags, not a stop: the `tasks` card shape has no standalone acceptance field
  ("the acceptance-ID citation lives per-case"), so a delta card's 1–3 criteria have no named line
  outside its **TEST:** bundle — a pre-existing card-shape question beside the
  `fence-never-reads` tension. Put to the grader for a reading; the lead places it after.
- The same grader resumed on the revision.

## 2026-10-18 — W2 P1 re-grade PASS, approved, build opened

- Re-grade: PASS · 0 blocking · 1 advisory. The grader re-ran every addendum figure from a fresh
  scratch tree (hashes, exactly six ops changed, stamp leaving hashes unmoved, `0 rejecting · 118
  advisory · 0 clusters · 185`, status `ea8e4a6c…`, three views, tools 28,443).
- Its reading of the L2 risk: not unworkable — a bug card is fully workable (the reproduction
  sits in the ruled set, outside **TEST:**); an improvement card's criteria-testable clause has
  no admissible object on the card, since the `tasks` card shape keeps criteria only as per-case
  Covers inside the **TEST:** bundle — it fails safe (never reads **TEST:**, never silently
  passes; grades the ruled set's seed or reads gap). Pre-existing; the owner is the card grammar
  (`feat.delta-cards`' "1–3 acceptance criteria" against the template's no-criteria-field).
- N1 placed by the lead: a BACKLOG item at the build's landing — *Delta-card criteria have no
  line outside the **TEST:** bundle* — to be ruled before the first improvement delta run; the
  optional semantics change (naming the seed as the oracle) not taken, since it changes what the
  clause grades and is a ruling.
- The lead approved; the plan pinned (`dd707710…` + `1123e3b6…`); tree snapshot (the log only).

## 2026-10-18 — W2 P1 built; P2 and P3 dispatched plan-only

- W2 P1's build report (recovered from the transcript): four files written byte-for-byte from the
  stamped scratch copies, stamp hashes unmoved (0092 `441a35b9…`, 0093 `31acb15c…`, 0094
  `856004e0…`, 0095 `cede5dc2…`); one allowlist row; three views re-emitted; views ≡ replay.
  `cargo test --no-fail-fast` 759/6, all six P3's pins — one (`render.rs:1173`, implement's
  pinned floor set) missed by the plan's red list, disclosed.
- The lead's checks: `git status` shows exactly P1's write set beside the log · `migrate
  validate` `0 rejecting · 118 advisory`, `clusters: 0`, `185` suppressed · status `1..95`,
  `ea8e4a6c…`, `94 documents · 1260 rules` · the six new ids and both moments render; the floors
  `impl.gate-card-confirm` and `impl.gate-final-acceptance` unchanged word for word.
- P2 (prose; write set widened to `testing-gap-finding`'s SKILL.md and strip, since L1 added the
  third primitive) and P3 (tests and kits; told the implement kit has `runs/`, so its
  preregistration decision needs a reason) dispatched plan-only in parallel; tree snapshot taken
  (9 lines). Both briefs say W2's re-plan bound is spent.

## 2026-10-18 — W2 P2 planned; peer grade dispatched

- W2 P2's plan (324 lines; recovered unchanged, sha256 `fc1141d3…`); tree after planning equals
  the snapshot. Seven files change: the router's `testing-gap-finding` row (it restates the
  closed inclusion list 0094 widened — one 214-character insertion) · four wave-context strip
  comments (`implement`, `review-sufficiency`, `testing-gap-finding`, `mochiko`) · two ledger rows
  (`review-sufficiency`, `testing-gap-finding`) · the CHANGELOG W2 part. No strip entry (no prose
  line leaves).
- Left unchanged, each with its reason: `implement.md` (the Entry step's delta card names its
  source and role, never its home), both skill bodies, `README.md`, `ARCHITECTURE.md`, and
  `GLOSSARY.md:16` — W1's carried note now true by `impl.feature-visit-input`.
- Fresh generic peer (`model: opus`) dispatched, asked to test the "left alone" calls hard.

## 2026-10-18 — W2 P2 plan PASS, approved, build opened

- Peer grade: PASS · 0 blocking · 3 advisory. Every "left alone" call tested against the built
  rules and held; router `:117` the one false line; the `feature-visit` overview and
  `GLOSSARY.md:16` both hold by `impl.feature-visit-input`. Figures checked first-hand
  (review-sufficiency payload 17,360 → 17,614; testing-gap-finding 23,305 → 23,629; router body
  48,776 → 48,990).
- The lead approved, the plan pinned (`fc1141d3…`), with amendments: A3 — "keyed to the run" in
  the router insertion · A1 — the CHANGELOG's carried-note sentence names both lines, the
  overview's citation corrected (P1's approved no-op, not "ruling A3") · A2 — the other zero-gap
  "straight / directly" sites named in the build report for unit 6.

## 2026-10-18 — W2 P2 built

- Seven paths, the three amendments applied, no deviation: router `:117` (+219 characters, "keyed
  to the run") · four wave-context strip comments · ledger rows `:80` (review-sufficiency
  17,360 → 17,614, +2,191 over 15,423) and `:92` (testing-gap-finding 23,305 → 23,629, +3,652
  over 19,977), verdict slots left for the gate · the CHANGELOG W2 part. Every declared-home write
  dry-run (`allow`) and byte-identical to it. One figure moved: the router body 48,990 → 48,995
  (A3's five characters).
- A2's sites read as holding ("straight / directly" means the design phase is skipped; the card
  order is not a phase): `ARCHITECTURE.md:243` · router `:90`, `:100`, `:143`.
- The lead's check: `git status` beyond the snapshot shows exactly the seven paths; `migrate
  validate` `0 rejecting · 118 advisory`.

## 2026-10-18 — W2 P3 planned; peer grade dispatched; usage limit reached

- W2 P3's plan (222 lines; recovered unchanged, sha256 `8dec0363…`, with its `plan.diff`
  `b43c1cc4…` part of the plan); no `crates/` or `evals/` path changed. Eight paths: four crate
  tests, `evals/contract/run.py` and README, `evals/plan/README.md`, the implement kit's
  `observable.yaml`. Red observed: the same six failures; the opt-in sweep fails at `:947` and
  `:1293`. The implement preregistration left as is (the kit has run — 108 tracked files under
  `runs/`; §3.6's rule; the setup ontology-grid precedent), its counter-precedent named.
- Fresh generic peer (`model: opus`) dispatched to grade it. The user's usage limit was reached
  during the dispatch; the grade may not have returned.
- **Resume here:** read the P3 grader's result (transcript, or re-dispatch the same brief if it
  failed) · on PASS approve P3 pinned by sha256 and resume it to build · then the W2 gate audit
  (wave plan §4, seven units plus the `testing-gap-finding` skill-pair unit; named overages:
  review-sufficiency +2,191, testing-gap-finding +3,652) · then commit W2 · then W3.

## 2026-10-18 — W2 P3 plan PASS (after a limit stop), approved, build opened

- The P3 grader's run ended on a session limit before its verdict; on the reset the same seat was
  resumed (tree unchanged, 16 lines). Grade: PASS · 0 blocking · 4 advisory, every key claim
  reproduced in its own mirrors (red 759/6, green 765/0, sweep 48/0, fmt and clippy 0, contract
  host run 6/7 → 7/7 "implement render (40 ids)", `check-rubric` 75 · 56 · 131, `check-fixtures`
  OK).
- The lead approved, the plan pinned (`8dec0363…` + `plan.diff` `b43c1cc4…`), with rulings: the
  implement preregistration left as is (§3.6; W1's setup precedent) · A1 taken — the footer scoped
  to this wave, implement's own 2026-10-11 recount named, reason 3 dropped or restated · A3 taken —
  a fresh mirror after the `run.py` edit for the green contract run · A4 the seat's call, one-line
  reason in the `why` · A2 noted. Any red beyond the six named is a stop.

## 2026-10-18 — W2 P3 built; W2 gate audit dispatched

- W2 P3's build: step-1 red the six named exactly; eight paths, Edit only, seven byte-identical to
  the approved after-hashes, `observable.yaml` differing only by rulings 2 and 4
  (`impl.feature-visit-input` kept conditional, its why giving the reason). Green: `cargo test
  --all` 765/0 · the opt-in sweep 48/0 · fmt 0 · clippy 0 · contract host run 6/7 → 7/7
  ("implement render (40 ids)") on a mirror cut after the edit · `check-rubric` 75 · 56 · 131 ·
  `check-fixtures` OK. No deviation; no new `.pyc`.
- The lead's check: `git status` beyond the snapshot shows exactly the eight paths (24 lines in
  all).
- The W2 gate grader dispatched: persona-less `general-purpose`, `model: opus`, eight units (the
  wave plan's §4 with `testing-gap-finding`'s skill pair as its own unit), the contract rendered
  first-hand, report at `reports/audit-w2.md`; named overages review-sufficiency +2,191 and
  testing-gap-finding +3,652; the tools ceiling measured under unit 5.

## 2026-10-18 — W2 gate audit: all PASS; wave 2 closed

- Report: `reports/audit-w2.md` (the conformance check allowed it before the write). Eight units,
  0 blocking, no fix round. The outcome lines:

```
audit: unit 1 (schema content · 0092 + implement view diff) · w2-gate-grader · opus · 2 files · 1 rounds · 0 blocking
audit: unit 2 (schema content · 0093 + implement view diff, ruling places checked live) · w2-gate-grader · opus · 2 files · 1 rounds · 0 blocking
audit: unit 3 (schema content · 0094 implement half + 0095 + implement and review-sufficiency view diffs + allowlist row) · w2-gate-grader · opus · 5 files · 1 rounds · 0 blocking
audit: unit 4 (schema content and skill pair · testing-gap-finding — 0094 skill half, view diff, SKILL.md and render) · w2-gate-grader · opus · 3 files · 1 rounds · 0 blocking
audit: unit 5 (command pair · implement) · w2-gate-grader · opus · 1 files · 1 rounds · 0 blocking
audit: unit 6 (skill pair · review-sufficiency) · w2-gate-grader · opus · 1 files · 1 rounds · 0 blocking
audit: unit 7 (prose primitives and records · router, four strips, budget ledger, CHANGELOG, README, ARCHITECTURE, GLOSSARY) · w2-gate-grader · opus · 10 files · 1 rounds · 0 blocking
audit: unit 8 (crate tests, contract expectations, plan kits · crate non-author review) · w2-gate-grader · opus · 8 files · 1 rounds · 0 blocking
```

- The grader's rulings: L1 inside D8, no floor lowered (insertions only, +271 and +53) · L2 does
  not contradict `review-sufficiency.fence-never-reads` · both overages HOLD (testing-gap-finding
  +3,652 = the ruled +3,328 plus +324; review-sufficiency +2,191 = the ruled +1,937 plus +254) ·
  tools 28,443 bytes, 557 under the line · all 22 ids `impl.ruling-places` names live, its writer
  settings 16 of 16 against the re-sort · W1's carried note: both lines now true.
- Advisories, placed by the lead: `impl.feature-visit-input` reads "any" visit file keyed to the
  FEAT, though dated visits accumulate (0095 uses the work row's pointer) — a W3 residual ·
  `impl.landing-verifier-folds` does not itself name the `gates.md` fold check that
  `impl.ruling-places` gives it — a W3 residual, one home kept · the order line's first `tasks.md`
  write must carry the template's headings — operational, left · two allowlist rows now suppress
  nothing — left, documented in `matrix_similar.rs` · the testing-gap-finding verdict slot in the
  ledger and CHANGELOG names unit 6; it is unit 4 — corrected when the lead fills the slots at the
  bump.
- The lead's wave-close regression read: `cargo audit` clean (31 dependencies). Wave 2 closed; no
  bump. Committed next.

## 2026-10-18 — W2 committed; W3 plan written, P1 dispatched plan-only

- W2 committed as `437b654` ("Move implement's later moments onto the shared asking line"); tree
  clean after.
- `wave3-rest.md` written (244 lines): sequences 0098–0101 (0102–0103 held) — specify, setup and
  the architecture desk each extend `common.asking-line` with a `<cmd>.ruling-places` binding (the
  re-sort's three remaining rows), specify's reader for default 23, and 0101 for the two W2 gate
  advisories; P2 and P3 take the default-21 residuals only where this build already touches the
  file; the lead holds the bump, the CHANGELOG slots and the KM landing. Seven gate units.
- W3 P1 (persona-less `general-purpose`, `model: opus`) dispatched plan-only; tree snapshot taken
  (the W3 plan the only untracked path).

## 2026-10-18 — W3 P1 planned; peer grade dispatched

- W3 P1's plan (547 lines; recovered unchanged, sha256 `6f8da241…`); tree unchanged but the
  lead's log. Four files (0098 specify · 0099 setup · 0100 desk · 0101 implement residuals); six
  mints (three asking-line stubs, three ruling-places bindings) and four rewords
  (`spec.stage-inputs-in-hand`, `impl.feature-visit-input` narrowed to the entry's pointer,
  `impl.landing-verifier-folds` the one home of the `gates.md` fold check, `impl.ruling-places`
  pointing at it). Dry run `0 rejecting · 118 advisory · 0 clusters · 185`, no allowlist row,
  `94 documents · 1266 rules`; `impl.sec.tools` untouched.
- The 39-row count closes: W3's 17 rows (on 3 · held 8 · off 5 · unchanged 1) with W1's and W2's
  give on 10 · held 13 · off 15 · unchanged 1 — the record's count; six gates across the commands.
- Specify's selection moves off "stays a gate" by `spec.ruling-places` alone, `spec.gate-selection`
  untouched (its `kind: gate` is a blocking checkpoint, its text names no yes / no form).
- Risks raised for the lead: R1 — the shelf-scope override (writer *on*) has no named grader ·
  R2 — default 23's reader reaches specify's intent stage, not only the story stage.
- Fresh generic peer (`model: opus`) dispatched, asked for readings on R1 and R2.

## 2026-10-18 — W3 P1 plan PASS, approved, build opened

- Peer grade: PASS · 0 blocking · 6 advisory, every deciding figure reproduced (`0 rejecting · 118
  advisory · 1266 scanned · 219979 pairs · 0 clusters · 185`; four views; sizes; `check` allow);
  the narrowed visit read checked not too narrow (a growth row's work row points at `visit.md`).
- The lead's rulings: R1 accepted — no grader invented for the shelf-scope override; its only
  check is the exact-text landing (a transcription, the re-sort row confirmed with no grader) ·
  R2 accepted — default 23's reader reaches the intent stage, the intent confirm standing · A4
  taken — 0101's intent says the `gates.md` fold differs from the "graded folds" 0026 removed ·
  A5 in the build report · A2, A3, A6 noted.
- A1, placed by the lead: four ruling places the re-sort never listed — `spec.missing-map-surfaced`,
  `spec.governance-region-absent` (offers to run setup, like implement's absent-surface offers),
  `setup.stress-test-cold-seat` (a recorded waiver), `arch.diagram-signoff` (a yes before a hosted
  render). `common.asking-line` already reaches them with the writer off; adding rows would go
  past the user's confirmed 39-row re-sort, so they stay unlisted — put to the user as a one-line
  note at the bump, not a fork.
- The plan pinned (`6f8da241…`); tree snapshot (the log and the W3 plan).

## 2026-10-18 — W3 P1 built; P2 and P3 dispatched plan-only

- W3 P1's build: four files written byte-for-byte (0101 carrying the A4 intent sentence, its hash
  unmoved, file sha256 `f991b04f…`); four views edited to the emitted hunks; no allowlist row. Red
  761/4, exactly the four P3 pins planned, plus the opt-in sweep's corpus pin.
- The lead's checks: `git status` exactly P1's write set beside the log and the W3 plan · `migrate
  validate` `0 rejecting · 118 advisory`, `clusters: 0` · status `1..101`, `300e26a8…`, `94
  documents · 1266 rules` · a fresh `views emit` equals the committed views · the three
  `*.ruling-places` render; `impl.landing-verifier-folds` names the `gates.md` fold.
- W3 P2 (prose, the default-21 prose residuals) and P3 (tests and the four kits, the default-21 kit
  residuals) dispatched plan-only in parallel; tree snapshot taken (10 lines).

## 2026-10-18 — W3 P2 planned; peer grade dispatched

- W3 P2's plan (318 lines; recovered unchanged, sha256 `941e7313…`); tree diff clean against the
  snapshot. Router: five pure insertions R1–R5, the default-21 residuals (body 48,995 to 49,938);
  `ARCHITECTURE.md`: four line fixes A1–A4, one the false `:301`; three strip comments (specify,
  implement, router); the CHANGELOG W3 part. The four command `.md`s, `README.md`, `GLOSSARY.md`
  and the budget ledger left alone, each with a reason.
- Q1 raised for the lead: `ARCHITECTURE.md:306–308` ("every touched product baseline takes its
  graded delta fold") false since D1-delta-files-retire, outside §3.5's named list; a ready edit
  offered, else left for the narrowed BACKLOG item.
- Fresh generic peer (`model: opus`) dispatched against the review-seat-plan render, asked to read
  Q1 too. P3 still planning.

## 2026-10-18 — W3 P2 plan PASS, approved with addendum, build opened

- Peer grade: PASS · 0 blocking · 7 advisory. a1 file count off by one · a2 Q1 misses two sibling
  false lines (`ARCHITECTURE.md:390–391`, `README.md:108`) · a3 A4's desk clause a widening · a4
  R1's user-added mark on the piece, not its gaps · a5 A3's "too" has no referent · a6 the
  CHANGELOG names the wrong ARCHITECTURE.md section · a7 three sections unnamed as read.
- The lead's rulings, sent as exact text (L1–L8): Q1 fixed, the ready edit taken (a false line
  in a file this build touches, default 21's rule) · `ARCHITECTURE.md:390–393` fixed by the same
  rule, sourced to `impl.baselines-in-place-marked` and `impl.baseline-diff-review`, the seat
  told to stop if the text overclaims · `README.md:106`/`:108` stay, since README is not touched
  by this build; they go to the narrowed BACKLOG item at landing · a4's mark moved to the gaps,
  per `review-sufficiency.user-added-regrade` · a5's "too" dropped · a1 and a6 corrected in the
  report and CHANGELOG · a3 kept, being W1's own ripple (`visit.md`), labelled so · a7 noted.
- The plan pinned (`941e7313…`); tree snapshot (10 lines). P3 still planning; its write set is
  disjoint.

## 2026-10-18 — W3 P2 built

- W3 P2's build: six paths (`strips/implement.md`, `strips/mochiko.md`, `strips/specify.md`,
  `ARCHITECTURE.md`, `CHANGELOG.md`, the router), 166 insertions and 19 deletions; each write
  dry-run through `mochiko-cli check` first (all allow) and `cmp`-matched to its dry-run copy.
  Router body 48,995 to 49,956 (the plan's 49,938 plus L5's 18). The file count is six; the plan
  summary's seven was a slip (a1).
- Four extras past the pinned text, all mechanical: the router strip comment follows L5 and the
  49,956 figure; three bare decision ids its own comments added are joined; the comments are
  rewrapped to 100 columns. `ids --check` over the six files: 2 bare, both in W2's existing
  comment (`strips/implement.md:41`, `:43`), for the gate grader.
- The lead's checks: `git status` is the 10 snapshot lines plus the six paths · L1 at
  `ARCHITECTURE.md:307–310`, L2 at `:394–398`, L6 at `:301`, L5 at the router `:90`, all read
  in the tree · `README.md` untouched, its two lines listed under the CHANGELOG's "Left in
  `BACKLOG.md`".

## 2026-10-18 — W3 P3 planned; peer grade dispatched

- W3 P3's plan (190 lines; recovered unchanged, sha256 `b0255ff8…`), exact bytes in its
  `plan.diff` (`ecc798de…`, 545 lines); no repo write. Ten paths: three crate tests (the four red
  pins plus three asserts masked behind them: 418 to 424, 1260 to 1266, the family `(424, 21068,
  0, 59)`, the corpus 219,979 scored, suppressed 185 unmoved), four kits' `observable.yaml`, the
  specify and architecture preregistrations, `evals/plan/README.md`. Mirror dry run green: cargo
  765/0, sweep 48/0, fmt and clippy 0, contract host 7/7, `check-rubric` on all four kits.
- Default-21 residuals taken: four bare `D3` lines in touched files, the 18 implement `why`s cut
  at a comma (re-quoted whole). Left: the brainstorm kit's three bare `D3` lines (untouched kit),
  `ids.rs`'s deliberate bare test inputs. Setup and implement preregistrations left alone, as
  run kits (W2's rule); specify and architecture recounted.
- Q1 for the lead: 47 more comma-cut `why`s BACKLOG never listed (specify 1, setup 2,
  architecture 33, feature 11); default leave and book at landing. Q2: specify's recount folds
  in drift since authoring; default take, per the W1 feature-kit ruling.
- Fresh generic peer (`model: opus`) dispatched against the review-seat-plan render, told to
  reproduce the red and the census itself and to check the comments' session date.

## 2026-10-18 — W3 P3 plan PASS, approved with addendum, build opened

- Peer grade: PASS · 0 blocking · 2 advisory, every figure reproduced in its own mirror (red
  761/4 and the masked asserts, sweep 46/2, the census and kind deltas, the C(n,2) sums, every
  partition shape, the 18 re-quotes byte-equal, the plantings true, no other expectation moving,
  host 7/7 with P2's edits in; the session date 2026-10-15 right). A1 the specify recount omits
  two rewords falling on rules already changed · A2 name the no-bytecode form for `uv` runs.
  Repro note: `matrix_similar.rs:1370` fails with the target dir outside the tree (environmental).
- The lead's rulings (L1–L4), sent as exact text or a stated transform: Q1 — all 47 comma-cut
  `why`s taken, double-quoted whole, words byte-equal, a stop on any `"` or `\`; the same rule
  as P2's Q1 (a defect in a file this build touches), the feature kit touched at W1 (`fccae34`),
  so it joins the write set (11 paths); a pinned footer sentence per kit · Q2 — the specify
  recount taken, A1 folded in as a pinned clause, checked against 0057 and 0061 first · A2 —
  `python3 -B` and `PYTHONDONTWRITEBYTECODE=1` · the target dir kept inside the tree.
- The plan pinned (`b0255ff8…`, `plan.diff` `ecc798de…`); tree snapshot (16 lines).

## 2026-10-18 — W3 P3 built

- W3 P3's build: 11 paths, +241 −110, Edit only; the planned 10 match `plan.diff`'s
  after-hashes except where L1 or L2 moved them. Red first, exactly as named (761/4 · sweep 46/2 ·
  `check-rubric` exit 1 on the three re-keyed kits · comma check 1·2·33·11·18 · 4 bare ids); then
  green: cargo 765/0 · sweep 48/0 · fmt and clippy 0 · `check-rubric` 59·14·73, 36·8·44,
  60·6·66, 75·56·131, feature 57·4·61 unchanged · `check-fixtures` OK ×5 · comma check 0 ×5 · 65
  re-quotes byte-equal, none carrying `"` or `\` · `ids --check` 0 bare, 0 drift · partitions
  match every footer · contract host 7/7 on a fresh mirror with P2's edits in.
- L2 checked first: 0057 and 0061 carry the two rewords; the clause landed.
- The lead's slip: L1's pinned footer, at N=1, reads "the 1 why … are double-quoted". A
  superseding order, voiding that one sentence, sent as exact text ("the one why …", "is
  double-quoted").
- The lead's check: `git status` is the 16 snapshot lines plus the 11 paths; no new `.pyc`.

## 2026-10-18 — W3 frozen; gate audit dispatched

- A second slip in the same pinned sentence ("their words unchanged" at N=1): one more
  superseding order, voiding those words only ("its words unchanged"), with "59 · 14 of 73."
  re-filled onto one line. Specify's four checks exit 0; file sha256 `970d4162…`.
- The wave frozen: 27 `git status` lines, 22 files changed (+649 −146); the diff without this
  log hashes `0d4f6b99…`. Writers hold until the verdict.
- One gate grader (`general-purpose`, `model: opus`) dispatched for the seven units of
  `wave3-rest.md` §4, its brief pinned (sha256 `6998a5ed…`): its contract rendered first-hand,
  the pre-pass run itself, the lead's W3 rulings named as its sources beside the record. Report
  at `reports/audit-w3.md`.

## 2026-10-18 — W3 gate audit: seven units PASS

- `reports/audit-w3.md` (454 lines), round 1, all PASS, 0 blocking; the frozen diff without this
  log held at `0d4f6b99…` from start to end. The outcome lines:
  - `audit: unit 1 (schema content · 0098 + specify view diff, ruling places checked live) · w3-gate-grader · opus · 2 files · 1 rounds · 0 blocking`
  - `audit: unit 2 (schema content · 0099 + setup view diff, ruling places checked live) · w3-gate-grader · opus · 2 files · 1 rounds · 0 blocking`
  - `audit: unit 3 (schema content · 0100 + architecture view diff, ruling places checked live) · w3-gate-grader · opus · 2 files · 1 rounds · 0 blocking`
  - `audit: unit 4 (schema content · 0101 + implement view diff, tools line measured) · w3-gate-grader · opus · 2 files · 1 rounds · 0 blocking`
  - `audit: unit 5 (command pairs · specify, setup, architecture + implement 0101 delta) · w3-gate-grader · opus · 4 files · 1 rounds · 0 blocking`
  - `audit: unit 6 (prose primitives and ripple · router, three strips, ARCHITECTURE, README, GLOSSARY, budget ledger, CHANGELOG) · w3-gate-grader · opus · 9 files · 1 rounds · 0 blocking`
  - `audit: unit 7 (tests and kits · three crate tests, contract expectations, five plan kits, two preregistrations, README · crate non-author review) · w3-gate-grader · opus · 11 files · 1 rounds · 0 blocking`
- Grader's rulings: every ruling-places id live, ways and writers matching the re-sort (specify
  7/7, setup 6/6, desk 4/4), the 39-row count closing · R1, R2 and A1 faithful · the tools line
  28,443 · W2's two bare ids at `strips/implement.md:41`, `:43` outside default 21 · every
  left-alone call holding · every moved figure reproduced, the 65 re-quotes byte-equal.
- Advisories, placed by the lead: 1 — A1's note goes to the user at the bump · 2 — the two bare
  ids booked in the narrowed bare-id BACKLOG item at landing · 3 — the CHANGELOG's W3 budgets
  line to read "no budgeted skill body or description changes", fixed at slot fill.
- The lead's §3.6 read, row by row (it clears nothing): D4 and D10 in units 1–3 · defaults 15 and
  16 in unit 2 · default 21 in units 6 and 7 · default 22's count owed at the bump · default 23 in
  unit 1 · the W2 advisories in unit 4. W3 committed next; then the bump.
