# Build log — `author-grader-consolidation`

Append-only. One `##` entry per wave event; the D9 outcome lines land here.

## Wave 1 — gate form — opened 2026-09-19

Plan: [wave1-gate-form.md](wave1-gate-form.md). Target 0.111.0. Audited under the old form (D10).
Sequence allocated: `0008-gate-form.yaml`. Seats: P1 schema · P2 prose · P3 crate tests (all
`general-purpose`, `model: opus` explicit) · A1/A2/A3 `mochiko:validator` (persona default `opus`).
Sound-loop floor read back (6 floors); trigger fired; legs 1–3 wired per plan §0.

- P1 plan-only dispatched 2026-09-19.
- P2 plan-only dispatched 2026-09-19 (section ids pinned in plan §3.1).
- **P1 plan APPROVED** ([reports/p1-plan.md](reports/p1-plan.md), floor count 11 by construction).
  Lead rulings on its risks: R1 zero-baseline guard in `run.py` approved (mirrors the summary
  printer) · R2 `PROBE_ARGUMENTS` row approved · R3 allowlist ownership → P1, rows only for edges
  the sweep reports · R4 anchor on the common block kept, fallback recorded if rejected · R5–R7
  noted. One wording fix required: `.outcome-line` carries D9's literal line grammar. P1 resumed to
  execute.
- **P2 plan APPROVED** ([reports/p2-plan.md](reports/p2-plan.md); description 730 chars, body
  3,449). Lead rulings: R1 P2's correction accepted — scaffold headings/order are **[judgment]**,
  only the `!` enumeration and the `allowed-tools` literal are **[suite]** (wave plan §3.4 fixed) ·
  R2 the ledger gains a fourth seeding path, **a ruled birth seed**, one sentence anchored to this
  record's build item 1 (precedent-consistent with producer-plan D4's "first-seed budget") · R3
  domain-lens re-pointing approved, CLAUDE.md reconciliation stays wave 3 · R6's three protected
  lines leave under this record's D3/D7 (repo rule — no strip; recorded here as the ruling) ·
  R4/R5/R7/R8 noted. P2 resumed to execute; payload figures filled once P1's render exists.
- **P1 HALTED at C1 (grammar gap):** no op adds a block to an existing common library —
  `mint-rule` needs a live section, `mint-section` on a common doc rejects (`section-set`),
  `replace-document` rejects (`op-inapplicable`). C1–C3 block together; C4 + C5 probed green
  (0 rejecting · 74 documents · 1065 rules · pin 11 · 3 clusters → 3 allowlist rows owed). Draft
  parked at `inputs/0008-gate-form.draft.yaml`, log left clean. No binary released yet (no
  `mochiko-cli-v*` tag), so a grammar widening carries no consumer cost. **Fork put to the user**:
  A widen the grammar (crate change, own plan + non-author review) · B second common document ·
  C home the bound on `setup` (amends D4). Awaiting ruling.
- **P2 EXECUTED:** 7 files — `skills/validation-primitive-edit/SKILL.md` (new; description 730 ·
  body 3,449) · router (paragraph + gate table) · budget ledger (fourth seeding path, two rows,
  R5 disclosure paragraph) · `primitive-edits.md` Check section (23 criteria preserved, 36 tags)
  · CHANGELOG 0.111.0 entry (§3.4 correction named) · both manifests 0.111.0. `PAYLOAD`/`BODY`/
  `RENDER` placeholders in two spots until the skill renders.
- **Grammar-gap fork — user ruled A, "as recommended": widen the grammar.** Seat P4 (crate op)
  added to the wave plan §2: `mint-rule` with no `section:` on a common library appends a block;
  own plan, non-author review at A3 (rust-cli.md), `cargo test`/fmt/clippy; README grammar row.
  No grammar-version bump (no binary released; the README's additive-op rule). Order: P4 → P1
  resumes C1–C3 → P3 → P2 fills payload → audits. P4 plan-only dispatched.
- **P4 plan APPROVED** ([reports/p4-plan.md](reports/p4-plan.md)): `Change::MintRule.section`
  → `Option<String>` (migration.rs parse arm reads the doc kind; absent + common library = append
  block; absent + command/skill = `op-malformed`); replay.rs arm total on the kind; validate/model/
  views/render/conform/hook untouched; 7 replay + 2 parse tests; README grammar row + no-bump
  sentence; fmt/test/clippy/audit. No grammar bump (no release tag). P4 resumed to execute.
- **P4 EXECUTED — four layers green:** `cargo test -p mochiko-cli` 472 passed (463 → 472, nine
  new) · fmt clean · clippy clean · audit clean. Files: `src/migration.rs` (+30 −6) ·
  `src/replay.rs` (+33 −6) · `tests/migration.rs` (+65) · `tests/replay.rs` (+292) · migrations
  README (+3 −1). `rg '\.blocks'` 15 sites / 4 files, all state readers. Six deviations recorded in
  the plan's Execution section; the one of note: test 2 asserts a control/treatment pair over
  `Replay.validation` rather than `load == Ok`. Non-author review owed at A3. Consequence: the
  installed binary (0.1.0, pre-widening) must be rebuilt from the tree before the log carries the
  section-less op.
- **Lead (mechanical): binary reinstalled** from the widened crate (`cargo install --path
  crates/mochiko-cli --force`; `target/release` rebuilt for the suite's `HOST_BINARY`); log still
  0 rejecting · 104 advisory under it. Crate version bump (0.1.0 → 0.2.0) assigned to P3 with its
  census re-key. P1 resumed for C1–C3 + pre-registration; P3 plan-only dispatched.

## Wave 1 — migration 0008 landed — 2026-09-19

*(The opening entry reached its 60-line bound; the hook denied the append. New entry, per the
log's one-entry-per-event grammar.)*

- **P1 LANDED:** `0008-gate-form.yaml` (hash `698cb30b…`); state 74 documents · 1067 rules ·
  sequences 1..8 · sha `8950f9b9…`; validate 0 rejecting · 105 advisory (+1: the new skill's
  budget line; setup `enforces-coverage` 16 → 17 naming `setup.gate-loop-bound`); similarity
  scanned 1067 · clusters 0 after three allowlist rows · suppressed 171; pin 11 exact;
  `patterns-model-tiering` pin 8; command floors 117 · skill floors 252. R4: anchor on the common
  block ACCEPTED — the corpus's first anchored block, D6's number now protected. Views idempotent.
  Files: migration · four emitted views · `expected-skills.json` · `run.py` · suite README ·
  allowlist · plan Execution 2; draft kept in `inputs/` as the halt record.
- Lead check: `rules validation-primitive-edit --section preamble` → `class: floor · 11 rules`,
  eleven ids as planned, plugin 0.111.0; validate 0 rejecting · 105 advisory.
- **P2 COMPLETE:** payload filled — body 3,449 · render 11,573 (preamble 1,968 · independence
  1,974 · scope 1,383 · inputs 1,279 · verdict 2,729 · output 1,566 · reserved 674) · payload
  15,022, no headroom, `[v0.111.0]` ruled birth seed · description 730. Budget row + CHANGELOG
  budget sentence written; audit/gate figures left for landing.
- **Audits A1 (pairs) and A2 (schema content + pre-registration) dispatched** — old form, fresh
  `mochiko:validator` seats, full read, one per cluster. A3 (prose + crate) waits on P3.
- **P3 plan APPROVED** ([reports/p3-plan.md](reports/p3-plan.md)): 18 assertions move, all
  measured from the tree (74 · 1067 · 1..8 · 329/738 · floors 252/117 · pin 8 · tuple (329,
  12_607, 0, 60) · sweep 1067/156_764/0/171); clusters-0 confirmed as a suppressed zero via the
  plan's four-step check; version bump moves Cargo.toml + Cargo.lock only (test literals are
  synthetic fixtures). Risk 1 ruled: ownership widened to `tests/views.rs` + `tests/render.rs`
  census assertions. P3 resumed to execute.
- **A1 (pairs) round 1** ([reports/a1-audit.md](reports/a1-audit.md)): `setup` PASS ·
  `patterns-model-tiering` **FAIL** (item 8: payload 18,358 vs budget 10,852 — standing overage
  +7,012 at [v0.110.0] grew +494 with the new floor's render; row not restamped, overage not named
  in the brief) · `validation-primitive-edit` PASS. Fix → P2 (ledger owner): name the overage
  (D7, a genuine new obligation, closes F12) and restamp the row at [v0.111.0]; re-audit by a fresh
  seat, full read, old form.
  - `audit: setup pair · validator · opus · 2 files · 1 rounds · 0 blocking`
  - `audit: patterns-model-tiering pair · validator · opus · 2 files · 1 rounds · 1 blocking`
  - `audit: validation-primitive-edit pair · validator · opus · 3 files · 1 rounds · 0 blocking`
- **A2 round 1** ([reports/a2-audit.md](reports/a2-audit.md)): schema content **FAIL, 3
  blocking** — (1) `validation-primitive-edit.gate-loop-bound` restates the number D4 homes in
  `common.gate-loop-bound` · (2) `judgment-items-pair` lists scaffold headings/order under the
  pre-pass, which asserts no such thing (falls through both legs) · (3) `setup.validate-seat-form`
  says "a fresh seat", dropping D7's "plain". AM-2 five PASS; views ≡ replay by checksum; pin 11.
  Pre-registration **PASS** (two notes: allowlist comment calls a CROSS-PAIR edge stub-vs-local;
  README absolute case total inherited stale, 84 vs the builder's 89). Fix → P1: amend 0008 in
  place (unshipped), re-stamp, re-validate, re-emit; re-audit by a fresh seat, full read.
  - `audit: schema content (migration 0008-gate-form + view diff) · validator · opus · 5 files · 1 rounds · 3 blocking`
  - `audit: contract-suite pre-registration · validator · opus · 4 files · 1 rounds · 0 blocking`
- **P2 fix (A1):** `patterns-model-tiering` re-measured independently — body 3,112 · render
  15,246 · payload 18,358 · overage +7,506 (Δ +494, all render, all `persona-less-grader-pin`);
  row restamped [v0.111.0] ruled-HOLDS with the D7/F12 justification, prior chain kept. Re-audit
  of the pair dispatched (fresh seat, full read, overage named in the brief).
- **P3 EXECUTED:** 22 assertions re-keyed (18 + the 4 widened), every figure re-derived
  first-hand (74 · 1067 · 1..8 · 329/738 · 252/117 · pin 8 · (329, 12_607, 0, 60) · sweep
  1067/156_764/0/171); clusters 0 recorded as a suppressed zero (168 + 3 = 171; two edges
  stub-vs-local, one keep-distinct — P3 corrected its own plan). `cargo test` 472 green ·
  full-similarity sweep 48 green (111 s) · fmt · clippy. Crate 0.1.0 → 0.2.0 (Cargo.toml +
  Cargo.lock). Files: 5 test files + Cargo.toml + Cargo.lock. Lead reinstalled the binary at
  0.2.0 (mechanical). **A3 (prose + crate) dispatched.**

## Wave 1 — audit rounds and gates — 2026-09-19

*(Second entry reached its 60-line bound; hook denied the append. New entry.)*

- **P1 fix round (A2):** three rewordings in place, re-stamped (hash `80f48ff1…`, state sha
  `30333fa8…`), 0 rejecting · 105 advisory, pin 11 same ids (pre-registration untouched), views
  idempotent; allowlist comment corrected (two stub-vs-local, one keep-distinct); README case
  total corrected to the builder's 89 (stale base predated this wave). Comment-only fix-on-sight
  in `run.py` l.5087 → P1. **A2 re-audit dispatched** (fresh seat, full read).
- P1 comment fix landed (`run.py` l.5087 → 159 metered sessions; `py_compile` clean). P1 done.
- **A1 re-audit, `patterns-model-tiering` pair — PASS, 0 blocking**
  ([reports/a1r-tiering.md](reports/a1r-tiering.md)); the restamped overage justification holds.
  - `audit: patterns-model-tiering pair · validator · opus · 8 files · 2 rounds · 0 blocking`
- Lead gate check: `MOCHIKO_FULL_SIMILAR=1 cargo test -p mochiko-cli --test matrix_similar` after
  P1's rewordings — 48 passed, 0 failed (112 s); the frozen full-corpus figures held.
- **A2 re-audit, schema content — FAIL, 1 blocking** ([reports/a2r-schema.md](reports/a2r-schema.md)):
  round-1 findings fixed, AM-2 five PASS, D2/D3/D5/D6/D7/D9/D11 PASS; D4 fails — the number is
  still implied twice in the skill document: `sec.verdict` intent "the one re-audit the loop
  allows" and `gate-loop-bound`'s tail "a further FAIL halts the landing" (fixes N=1, duplicates
  `second-fail-user`). **Second FAIL on this unit** — admissible under the old form (D10); under
  the D6 rule this wave ships it would go to the user; disclosed to the user, fix round 2 run.
  Data point for the D9 watch: one unit at 3 rounds under the old form.
  - `audit: schema content (migration 0008-gate-form + view diff) · validator · opus · 12 files · 2 rounds · 1 blocking`
- **P1 fix round 2 (A2):** both rewordings in place, re-stamped (hash `f62b1922…`, state sha
  `f7b3ab2d…`), 0 rejecting · 105 advisory, pin 11, idempotent. P1's corpus sweep for
  count-fixing phrases: four hits — the common block itself, `second-fail-user` (carved out), and
  the `sec.reserved` title "the second FAIL is the user's" (borderline, flagged). Lead ruled the
  one-word retitle ("a repeat FAIL is the user's") to close the class; P1 applying. Round-3
  re-audit follows.
- **P1 retitle landed:** hash `b9356c80…`, state sha `7b58e8c1…`, pin 11, 0/105, clusters 0,
  idempotent; sweep now two hits only (`common.gate-loop-bound` · `second-fail-user`). P1 done.
  **A2 round-3 re-audit dispatched** (fresh seat, full read).
- **A3 round 1** ([reports/a3-audit.md](reports/a3-audit.md)): crate diff **PASS** · crate
  fixtures **PASS** · router **PASS** · `primitive-edits.md` **FAIL 2** (loop-bound paragraph
  restates "one fix and one re-audit"; skill criterion 1 tagged wholly [suite]) · budget ledger
  **FAIL 2** (new skill's render now 11,582 / payload 15,031 after P1's rewordings — the birth
  seed must be re-measured at the quiesced tree; R5 "largest description" false, tiering is
  1,208) · CHANGELOG + manifests **FAIL 1** (no mention of the `mint-rule` widening / the
  `mochiko-cli` 0.2.0 requirement). Eight advisories noted. Fixes → P2; re-audit of the three
  prose units by a fresh seat.
  - `audit: primitive-edits.md · validator · opus · 4 files · 1 rounds · 2 blocking`
  - `audit: router SKILL.md · validator · opus · 3 files · 1 rounds · 0 blocking`
  - `audit: primitive-cost-budgets.md · validator · opus · 3 files · 1 rounds · 2 blocking`
  - `audit: CHANGELOG + manifests · validator · opus · 3 files · 1 rounds · 1 blocking`
  - `audit: crate diff (migration.rs · replay.rs · their tests) · validator · opus · 4 files · 1 rounds · 0 blocking`
  - `audit: crate fixtures (5 test files · Cargo.toml · Cargo.lock · migrations README) · validator · opus · 8 files · 1 rounds · 0 blocking`
- **P2 fix round (A3):** all five done — loop-bound paragraph cites the id, no count; skill
  criterion 1 split [suite]/[judgment]; birth seed re-measured at state `7b58e8c1…` (body 3,449 ·
  render 11,519 · payload **14,968** · description 730; the two superseded readings 15,022 /
  15,031 recorded in the row); R5 paragraph corrected (tiering 1,208 is the largest); CHANGELOG
  gains the `mint-rule` widening + `mochiko-cli` 0.2.0 floor. Prose re-audit held until the
  round-3 schema verdict confirms the log is quiesced.
- **A2 round 3, schema content — PASS, 0 blocking** ([reports/a2r3-schema.md](reports/a2r3-schema.md)).
  Log quiesced at state `7b58e8c1…`. Prose re-audit + contract-suite gate start.
  - `audit: schema content (migration 0008-gate-form + view diff) · validator · opus · 5 files · 3 rounds · 0 blocking`
- **Gate 6 started:** `python3 evals/contract/run.py` (full, 89 cases, Docker sandbox via `sbx`,
  release binary 0.2.0), detached; early host cases green incl. the new skill's frozen floor set
  (11 ids) against its render. **A3 re-audit of the three prose units dispatched** (fresh seat;
  its budget re-measure is the release-gate sweep).

## Wave 1 — final audit rounds and landing — 2026-09-19

*(Third entry reached its 60-line bound; hook denied the append. New entry.)*

- **A3 round 2 (prose):** budget ledger **PASS** (release-gate sweep: new skill body 3,449 ·
  render 11,519 · payload 14,968 · description 730 — every stamp exact; tiering 18,358 exact) ·
  CHANGELOG + manifests **PASS** · `primitive-edits.md` **FAIL 1** (new: skill criterion 5 tagged
  wholly [CLI], but the tombstone disposition read is judgment — `disposition` is free text at
  `migration.rs:302`; command criteria 3/4 need the same limb widened). Report write denied twice
  by the artifact hook (`## Notes of note` 16 lines vs 15) — surfaced to the user; seat asked to
  write within the bound. Fix → P2; round-3 re-audit by a fresh seat.
  - `audit: .claude/rules/mochiko/primitive-edits.md · validator · opus · 8 files · 2 rounds · 1 blocking`
  - `audit: .mochiko/memory/primitive-cost-budgets.md · validator · opus · 5 files · 2 rounds · 0 blocking`
  - `audit: CHANGELOG.md [0.111.0] + both manifests · validator · opus · 6 files · 2 rounds · 0 blocking`
- **P2 fix (A3 r2):** disposition-read [judgment] limb on skill criterion 5 and command criteria
  3/4, verified against `migration.rs` + the `tombstone-integrity` arm before tagging; 23 criteria
  intact. **Round-3 audit of `primitive-edits.md` dispatched** (fresh seat, full read).
- **A3 round 3, `primitive-edits.md` — PASS, 0 blocking** ([reports/a3r3-rules.md](reports/a3r3-rules.md));
  two non-blocking notes (the [CLI] legend gloss vs render-sourced limbs on skill 3/8; "restate
  what it holds" overstates). First write denied: missing `report:` frontmatter — surfaced; the
  seat read `mochiko-cli template report-envelope` and conformed. **All eleven audit units PASS.**
  - `audit: .claude/rules/mochiko/primitive-edits.md · validator · opus · 1 files · 3 rounds · 0 blocking`
- Lead gate re-run at the quiesced tree (state `7b58e8c1…`): `cargo test -p mochiko-cli` all
  binaries ok · `cargo fmt --all --check` clean · `cargo clippy --all-targets -- -D warnings`
  clean · `MOCHIKO_FULL_SIMILAR=1` sweep 48 passed (96 s).
- **Gate 6 (contract suite) full run:** 88 of 89 cases `ok` incl. both new-skill cases,
  `preload`, `gate-live`; the last case `reminder-spawn` crashed the runner —
  `NameError: name 'golden' is not defined` at `run.py:4904` — a runner defect, not a plugin
  failure. **Provenance: pre-existing** — `git blame` puts the line at commit `5d8fc69c`
  (2026-09-15); `golden` was never defined in `case_reminder_spawn`, the function's own variable
  is `marker`; not in this wave's `run.py` diff. The suite's last case has crashed on `main` since.
  Fix (`golden` → `marker`) → P1 as a fix-on-sight defect close; fresh non-author review of the
  one-token diff (rust-cli.md); `--case reminder-spawn` re-run. Gate-6 evidence = the full run
  (88 ok + runner crash) plus the fixed case's run, disclosed; a full re-run offered to the user.
- **P1 fix-on-sight landed:** `golden` → `marker` (one token; `py_compile` clean; the crash sat in
  the evidence write after both assertions had passed). `--case reminder-spawn`: `ok`, both
  `R-SPAWN-UNNAMED` and `R-SPAWN-NAMED` green; runner: `1/1 cases passed · FILTERED — 1 of 89 · Not
  a gate run` (quoted as such). **Gate 6 deterministic set: 89/89 green across the two runs.**
  Non-author review of the `run.py` diff dispatched.
- **A4 (`run.py` fix-on-sight) — PASS, 0 blocking** ([reports/a4-runpy.md](reports/a4-runpy.md));
  one note: the l.5087 comment count moved by eight for one added case — prose, no counter.
  - `audit: run.py fix-on-sight (reminder-spawn NameError) · validator · opus · 1 files · 1 rounds · 0 blocking`

## Wave 1 — CLOSED 2026-09-19 at v0.111.0

- **Gates:** `migrate validate` 0 rejecting · 105 advisory (+1, the new skill's budget line) ·
  views ≡ replay (idempotent emit; crate test) · `cargo test -p mochiko-cli` green (472 incl.
  P4's nine) · fmt · clippy · audit · full similarity sweep 48/48 · **contract suite 89/89**
  (full run 88 ok + `reminder-spawn` runner crash, pre-existing; fixed, reviewed, case re-run
  green — disclosed; a clean full re-run is the user's option before the bump) · char-budget
  pre-assert on every touched budgeted primitive (release-gate sweep exact) · CHANGELOG entry ·
  manifests 0.111.0 · strips: none owed (asserted at audit).
- **Audit roll-up (old form, D9 lines above):** 12 units · 17 rounds · 8 blocking findings
  caught (all fixed): setup pair 1r · tiering pair 2r · new-skill pair 1r · schema content 3r ·
  pre-registration 1r · router 1r · budget ledger 2r · CHANGELOG+manifests 2r · crate diff 1r ·
  crate fixtures 1r · `primitive-edits.md` 3r · `run.py` fix 1r. Baseline samples for the D9
  watch: 5 of 12 units zero-blocking at round 1.
- **Halt fired once** (§7): the grammar gap at P1's C1 — user-ruled "widen the grammar" → P4.
- **`floor: tripped · seats: P1 (schema) · P2 (prose) · P3 (crate tests) · P4 (crate op) / A1 ·
  A1r · A2 · A2r · A2r3 · A3 · A3r · A3r3 · A4 (all `mochiko:validator`, opus)`** — all producers
  `general-purpose` at explicit `model: opus`; lead wrote only plans, logs, the mechanical binary
  reinstalls, and the landing transcription.
- **Landing:** record Status + index → BUILT · BACKLOG gate-form item → trail · ROADMAP row ·
  DECISIONS status · commit suggested, never run. **Committed by the user as `2ec189b` on `main`.**

## Wave 2 — the plan-QA leg — opened 2026-09-19

Plan: [wave2-plan-qa.md](wave2-plan-qa.md). Target 0.112.0. `producer-plan-enforcement` D8 items
0–4, 5′, 6–10 under the NEW gate form + the D9 double-grade of the `review-seat-plan` pair. Sequence
allocated: `0009-plan-qa-leg.yaml`. Seats: P1 schema · P2 prose · P3 crate tests (persona-less,
`model: opus`); per-producer fresh generic peer graders on PPE D5; one plain gate grader for every
unit; one old-form `mochiko:validator` for the double-grade. Watch: PPE D8 item 7 stop condition.

- **Install-refresh gate (PPE D9 abort gate) open:** installed copy 0.110.0, marketplace checkout at
  `9f256f0`; repo at 0.111.0 (`2ec189b`, unpushed). User asked to push + refresh, or to waive with
  the pre-install path (PPE D4/X15) recorded per grade. No planning dispatch until ruled.
- **Gate cleared — user chose A:** pushed `main`, `/plugin` updated mochiko, `/reload-plugins
  --force` → 47 skills; installed cache 0.111.0 (39 skill dirs incl. `validation-primitive-edit`).
  Tree snapshot: clean. **P1, P2, P3 plan-only dispatched under the leg** (read-only, disjoint;
  each diffed against the snapshot on return; fresh generic peer grades each plan on PPE D5).
- **P3 plan returned** ([reports/w2-p3-plan.md](reports/w2-p3-plan.md); 16 assertions + 4
  conditional + 2 stale strings). Dirty-tree check: clean (only plan files + this log). Lead
  rulings passed to the peer: R1 existing `anchor:` fields stay (D8 rides the header anchor; no
  re-anchoring escape hatch in `fidelity.rs`) · R2 nothing outside the write set touched even
  temporarily (scratchpad plugin-root copy or suppressed-count arithmetic instead of renaming the
  allowlist). **Peer grade dispatched** (fresh generic seat, D5 criteria from the brief —
  pre-install path recorded).
- **P2 plan returned** ([reports/w2-p2-plan.md](reports/w2-p2-plan.md); description 839, body
  3,534; three risks incl. the `review-*` name on a lead-binding grade — PPE D4's name, ships
  visible). Dirty-tree check: clean. **Peer grade dispatched.**
- **PLAN GRADE: P3 · FAIL · items 2, 3** (peer: the clusters step renamed P1's allowlist —
  replace with the scratchpad allowlist route, `matrix_similar.rs:990` takes an explicit path;
  the "only way" claim unsourced and false). Items 1, 4, 5, 6 PASS; 7 advisory PASS. Re-plan
  round 1 of 1 consumed (`plans: P3:FAIL(1)` so far). Peer also flagged the wave plan §3.1 item
  6 floor count as self-contradictory — lead corrected it to 5. P3 resumed to revise; same peer
  re-grades.
- **P1 plan returned** ([reports/w2-p1-plan.md](reports/w2-p1-plan.md); `review-seat-plan` 15
  rules / 5 floors, agrees with the corrected §3.1). Lead rulings: (a) `.fresh-peer-grader` labels
  `[independence]` only — `seats` is not in the skill-labels registry and no `registry-add` is
  minted; (b) `leg-1-seat-produces` keeps its opening clause "Leg 1 — production sits with a seat,
  never the lead:" ahead of the user-authored D8 text — preserving the existing floor
  responsibility the D8 quote did not rule out; disclosed to the user. Dirty-tree check run.
  **Peer grade dispatched.**
- **PLAN GRADE: P2 · PASS** (items 1–6 PASS; 7 advisory: 326 lines, mostly drafted artifact
  text). Peer's note, accepted as a disclosed deviation: plans are persisted here as
  `report: disclosure` files although PPE D2 makes run plans transient — they double as the
  execution records the primitive-edit ceremony needs; the grade ran on the plan text. **P2
  approved, resumed to execute** (render-dependent figures placeholder until P1 lands).
- **P3 revised** (scratchpad empty-allowlist probe inside its own `matrix_similar.rs`, removed
  before the gate diff; reads cite `matrix_similar.rs:990`, `:689-699`, `similar.rs`). Same peer
  re-grades (round 2 of the shared counter — n=1 consumed).
- **PLAN GRADE: P3 · PASS (re-grade)** — items 2/3 discharged, no regression; two nits carried
  into approval (probe allowlist as `suppressions: []` so the code path is truly identical;
  envelope `round:` → 2). `plans: P3:PASS(1)`. **P3 approved; executes after P1 lands.**
- **PLAN GRADE: P1 · FAIL · item 1** (leg-1 text drops the opening clause "production sits with
  a seat, never the lead" — the plan predates lead ruling R3; all ten D8 item-2 sites present,
  `.seven-items` verbatim, items 2–6 PASS, 7 clear). Re-plan round 1 of 1 consumed
  (`plans: P1:FAIL(1)` so far). P1 resumed to prefix the clause; same peer re-grades.
- **WATCH TRIPPED (PPE D8 item 7):** first-round plan FAILs 2 of 3 producers (P3, P1) — exceeds
  half. Both FAILs trace to lead rulings (R2, R3) issued after the plans were written, not to
  seat quality; both repaired within the one re-plan round; no dirty tree; no second re-plan.
  Reported to the user before any execute dispatch beyond P2's (already approved on a clean
  PASS). Continue / pause is the user's call.

## Wave 2 — execution and audits — 2026-09-19

*(Opening entry reached its 60-line bound; hook denied the append. New entry.)*

- **P2 EXECUTED** (approved on a clean PASS before the trip): 7 files — `review-seat-plan/SKILL.md`
  (new; description 839, body 3,534, seven `!` lines) · router Seat-plan QA table · budget rows +
  disclosure (placeholders `<P>`/`<R>` until P1's render) · `primitive-edits.md` criterion 6 ·
  CHANGELOG `[0.112.0]` · both manifests 0.112.0. No deviations.
- P1 revised (opening clause restored, `round:` 2); same peer re-grading. P3 envelope fixed,
  holding. Watch trip reported; awaiting the user's continue/pause.
- **PLAN GRADE: P1 · PASS (re-grade)** — item 1 discharged byte-exact, no regression. Round
  figures complete: `plans: P1:PASS(1) · P2:PASS(0) · P3:PASS(1)`; no dirty tree; no second
  re-plan. Execution of P1 and P3 held for the user's ruling on the tripped watch.
- **User ruled "continue"** on the tripped watch (both first-round FAILs traced to late lead
  rulings; repaired in-bound). **P1 resumed to execute**; P3 after P1 lands.
- **P1 LANDED:** `0009-plan-qa-leg.yaml` (hash `a58d7ab9…`); state 75 documents · 1082 rules ·
  sequences 1..9 · sha `0c6b1460…`; pin `review-seat-plan` = 5 exact; validate 0 rejecting ·
  106 advisory (+1, the new skill's budget line); four new 2-member clusters → four allowlist
  rows, suppressed 166 → 170, clusters 0; no 3+ identical texts (convergence call did not fire).
  `run.py --list` 91 cases; `--host-only` 7/7 green. Fix-on-sight: a second stale README session
  figure (→ 162). **P3 resumed to execute; P2 resumed to fill placeholders + read-back.**
- **P3 HALTED at its Risk 1 (correctly):** 0009 carries five `set-rule-field · anchor` ops; three
  re-anchor sidecar-anchored floors (`leg-1-seat-produces`, `arch.sound-loop-floor`,
  `feat.sound-loop-floor`: `charter-ritual-balance` → `producer-plan-enforcement D8`) against
  ruling R1; `fidelity.rs:559` fails "the anchor moved". Lead ruled route (a): P1 drops those
  three ops (the two on previously unanchored rules stand — raising protection needs no
  authority), re-stamps, re-validates, re-emits. P3 holds. Pre-edit census: 455 pass / 9 fail
  (8 expected census sites + the anchor stop); command-family suppressed 60 → 56 (rewords
  un-matched four rows — measured, not predicted).
- **P2 filled:** `review-seat-plan` body 3,534 · render 9,167 · payload **12,701** · description
  839 · pin 5; `!` lines match the render's order (no SKILL.md edit). Render convention
  calibrated against `validation-primitive-edit`'s 11,519 control (trailing newlines counted).
  Row cites state `0c6b1460…`; re-measure owed after P1's re-stamp (anchors are not rendered, so
  the payload should hold). Hook note: a heredoc append was refused; redone with Edit.
- **P1 R1 repair landed:** hash `2f0af318…`; the three sidecar anchors emit as
  `charter-ritual-balance`; `impl.sound-loop-floor` + `arch.author-grader-separation` keep their
  new D8 anchor (previously unanchored); 0/106, clusters 0, suppressed 170; state `d5cb369a…`
  at 75 · 1082 · 1..9; host cases 7/7. **P3 "go"; P2 re-measure at the new state.**
- **Double-grade (old form) dispatched:** fresh `mochiko:validator` on the `review-seat-plan`
  pair, skill-pair criteria, full read — its finding set is one half of the D9 baseline. The
  gate grader (new form) follows with every unit; crate fixtures join it on resume once P3 lands.
- **Gate grader dispatched (new form):** `gate-grader-w2`, plain `general-purpose` at explicit
  `model: opus`; brief = the `validation-primitive-edit` render pasted verbatim (seven blocks,
  eleven floors, captured from the tree) + 13 units with paths + pre-pass commands; the crate
  fixtures unit joins on resume. Sidechain transcript to be read for cache-creation tokens (I2).
- **I2 cost read (F13 method, first turn, `claude-opus-5` both):** gate grader
  `agent-agate-grader-w2-…` cache_creation **27,634** + cache_read 27,111 (project context, shared)
  · `validator` `agent-adg-validator-…` cache_creation **22,404** + cache_read 27,111. The plain
  seat carrying the pasted render and 13 units costs ~5.2K MORE than the persona with its skill
  preload and one unit. **D7's cost limb: no instantiation saving measured** — the saving, if
  any, is D11's one-seat-per-wave (one instantiation for 14 units vs one per cluster). Lands in
  the record at wave close; the D9 watch item carries it.
- P2 re-measured at state `d5cb369a…`: figures unchanged (12,701 / 839 / pin 5); row sha updated.
  P2 done.
- **Double-grade, old-form half — PASS, 0 blocking, 4 non-blocking** (D5 items 4/7 reworded not
  verbatim · "reads the repo pair by path" under-specified · one escalation overlap · one label
  divergence) ([reports/w2-dg-validator.md](reports/w2-dg-validator.md)). Side-by-side pending
  the gate grader's verdict on the same unit.
  - `audit: review-seat-plan pair (old form, double-grade) · validator · opus · 3 files · 1 rounds · 0 blocking`

## Wave 2 — gate audit rounds — 2026-09-19

*(Previous entry reached its 60-line bound; hook denied the append. New entry.)*

- **Gate grader round 1 — 12 PASS, 1 FAIL** ([reports/w2-gate-audit.md](reports/w2-gate-audit.md)):
  `patterns-sound-loop` pair item 8 — payload 12,581 (body 2,812 + render 9,769) vs 12,388
  budget, +193 from the two floor rewords (+430 rule text, −237 render-format constant), no
  overage named, no ledger row. Fix → P2: standing-overage row [v0.112.0] ruled-HOLDS with the
  D8 justification. Same grader resumes for the delta re-audit (D6) + unit 14 after P3.
  **Double-grade side by side (unit 7 `review-seat-plan` pair):** new form PASS 0 blocking · old
  form PASS 0 blocking + 4 non-blocking — baseline recorded.
  - `audit: patterns-sound-loop pair · gate-grader-w2 · opus · 3 files · 1 rounds · 1 blocking`
  - `audit: patterns-plan-minimalism pair · gate-grader-w2 · opus · 3 files · 1 rounds · 0 blocking`
  - `audit: architecture command pair · gate-grader-w2 · opus · 3 files · 1 rounds · 0 blocking`
  - `audit: feature command pair · gate-grader-w2 · opus · 3 files · 1 rounds · 0 blocking`
  - `audit: implement command pair · gate-grader-w2 · opus · 3 files · 1 rounds · 0 blocking`
  - `audit: brainstorm/setup/specify command pairs · gate-grader-w2 · opus · 7 files · 1 rounds · 0 blocking`
  - `audit: review-seat-plan pair · gate-grader-w2 · opus · 3 files · 1 rounds · 0 blocking`
  - `audit: schema content 0009-plan-qa-leg · gate-grader-w2 · opus · 8 files · 1 rounds · 0 blocking`
  - `audit: contract-suite pre-registration · gate-grader-w2 · opus · 4 files · 1 rounds · 0 blocking`
  - `audit: router skills/mochiko/SKILL.md · gate-grader-w2 · opus · 1 files · 1 rounds · 0 blocking`
  - `audit: primitive-cost-budgets.md · gate-grader-w2 · opus · 1 files · 1 rounds · 0 blocking`
  - `audit: primitive-edits.md criterion 6 · gate-grader-w2 · opus · 1 files · 1 rounds · 0 blocking`
  - `audit: CHANGELOG + manifests · gate-grader-w2 · opus · 3 files · 1 rounds · 0 blocking`
- **Gate 6 started:** full contract suite (91 cases) detached; plugin-tree inputs final (P1
  landed; P2's remaining fix is the ledger; P3 is crate tests). Early host cases green incl. the
  `review-seat-plan` pre-registration against its render.
- **P2 fix (gate r1):** `patterns-sound-loop` re-measured — body 2,812 · render 9,769 · payload
  12,581 · +193; attribution measured against a scratchpad replay without 0009 (five sections
  byte-identical; leg-1 +177, disclosure-line +253, render-format constant −237); row restamped
  [v0.112.0] ruled-HOLDS. Gate grader resumed for the unit-1 delta re-audit (D6, same seat).
- **P3 EXECUTED:** five test files (+61 −26), no crate source, no bump; `cargo test` 472 green ·
  full sweep 48 green (92 s) · fmt · clippy. Sidecar walk untouched and passing. Floor counts
  re-derived from scratchpad-emitted views: 257 skill / 117 command over 75 views. Both similarity
  zeros recorded as suppressed with raw counts (corpus 77 clusters / 170 edges bare → 0 / 170
  suppressed; command family 29 / 56 → 0 / 56 — the 60 → 56 fall explained: rewording one side
  of an allowlisted pair retires its hit). Probe removed, grep-verified. **Gate grader queued for
  unit 14.**
- **D9 double-grade baseline — `review-seat-plan` pair, same tree, both `claude-opus-5`:**
  OLD form (`validator`, skill-pair checklist, one unit): PASS · 0 blocking · **4 non-blocking**
  (D5 items 4/7 reworded not verbatim · "reads the repo pair by path" under-specified ·
  escalation stated in two rules · one label divergence from the plan's pin) · first-turn
  cache_creation 22,404. NEW form (plain seat, rendered contract, 13 units): PASS · 0 blocking ·
  **0 notes on this unit** (the non-extension of `evidence-floor` read as ruled) · first-turn
  cache_creation 27,634. Reading: identical verdicts; the old form surfaced four nits the new
  form did not name; the new seat was not cheaper to instantiate but graded 13 units for one
  instantiation. This is the baseline the D9 watch reads later lines against — recorded, not
  interpreted further here.
- **Gate grader round 2, unit 1 — PASS, 0 blocking** (delta read: the ledger row + item 8; every
  figure reproduced independently; pre-pass byte-identical to round 1; unit 11 re-checked
  standing). **13/13 PASS**; unit 14 (crate fixtures) queued on the same seat. Hook deny surfaced
  by the seat (Failure narrative 16 vs 15 lines) — conformed, nothing lost.
  - `audit: patterns-sound-loop pair · gate-grader-w2 · opus · 4 files · 2 rounds · 0 blocking`

## Wave 2 — close — 2026-09-19

*(Previous entry reached its 60-line bound; hook denied the append. New entry.)*

- **Gate grader unit 14 (crate fixtures) — PASS, 0 blocking.** Pre-pass first-hand: `cargo test`
  472 green · fmt · clippy · audit clean; every moved number verified from `migrate status` / the
  views (75 · 1..9 · 1,082 · 329/753 · floors 117/257 · fail 36 · sweep 1,082/161,376/0/170);
  16 asserts replaced same-shape, `checked` 37 → 38 widens; suppressed zeros named with raw
  figures; no source, Cargo unchanged. One pre-existing note: `render.rs`'s `measured > 200`
  message names a count the assert does not check. **Wave 2: 14/14 units PASS, 15 outcome lines,
  1 fix round.**
  - `audit: crate fixtures · gate-grader-w2 · opus · 5 files · 1 rounds · 0 blocking`
- **Lead release check at the final tree:** `cargo test -p mochiko-cli` 472 passed · fmt clean ·
  clippy clean · `migrate validate` 0 rejecting · 106 advisory · views ≡ replay (content hash
  identical before/after a fresh emit). Gate 6 (contract suite, 91 cases) still running.
- Landing writes independent of the suite: record D7 cost limb `Assumed` → **Measured** (I2 read);
  BACKLOG D9 watch item carries the double-grade baseline + cost read.
- **Gate 6 — contract suite 91/91.** One full sandbox run: `contract suite: 91/91 cases passed,
  91 ran, 303 measurement(s) recorded and not asserted` (log `contract-suite-w2.log`, session
  scratchpad). No crash, no fix-on-sight this run. Every wave-2 gate is now green.
- **Gates at the bump (v0.112.0):** `migrate validate` 0 rejecting · 106 advisory · views ≡ replay
  (hash) · `cargo test -p mochiko-cli` 472 · fmt · clippy · audit · similarity sweep 48/48 ·
  contract suite 91/91 · char-budget pre-assert (two ledger rows restamped) · `plugin.json` and
  `marketplace.json` synced at 0.112.0. The CHANGELOG Gates sentence carries the same figures.
- **Disclosure line:** `floor: tripped · seats: P1 (schema) · P2 (prose) · P3 (crate tests) /
  peer-p1w2 · peer-p2w2 · peer-p3w2 · gate-grader-w2 · dg-validator · plans: P1:PASS(1) ·
  P2:PASS(0) · P3:PASS(1)`. Producers, peers and the gate grader were plain `general-purpose`
  seats at explicit `model: opus`; `dg-validator` rode its persona default (`opus`). The lead
  wrote only the wave plan, this log, the operating-doc landing and the mechanical version bumps.
- **Halts and watches:** one halt — P1's execution breached its R1 fence (three sidecar floors
  re-anchored); the three ops were dropped and the migration re-stamped before the gate. The D8
  item-7 watch tripped on 2/3 first-round plan FAILs; reported, user ruled continue. No dirty
  tree, no second re-plan, no second FAIL at the gate.
- **Landing set (this close):** CHANGELOG `[0.112.0]` Gates sentence filled · record Status line
  and the index entry → wave 2 BUILT · DECISIONS 2026-09-19 row → wave 2 built, wave 3 pending;
  2026-09-03 producer-plan row → wave 1 (plan-QA leg) built · BACKLOG producer-plan item re-cut
  to the retirement wave only, the plan-QA half to the trail as DONE · ROADMAP "Floor builds" row
  → wave 2 built · KM invariants run. Commit pending the user.
- **Wave 2 CLOSED 2026-09-19 at v0.112.0.** Next: wave 3, the `validator` retirement
  (producer-plan D8 item 5), audited under the gate form — one gate grader, no double-grade.

## Wave 3 — the `validator` retirement — opened 2026-09-19

Plan: [wave3-validator-retirement.md](wave3-validator-retirement.md). Target 0.113.0.
`producer-plan-enforcement` D3 / D8 item 5 / D9 wave 2, sequenced as this path's wave 3 (D10).
Sequence allocated: `0010-validator-retirement.yaml` (three `reword-rule` ops, no id or pin moves).
Seats: P1 schema + tiering pair · P2 prose sweep · P3 suite/crate fixtures/eval kit (persona-less,
`model: opus`); per-producer fresh generic peer graders per `mochiko:review-seat-plan` (installed at
0.112.0 — both delivery ways live); one plain gate grader for every unit. Watch: PPE D8 item 7.

- **Sound-loop read-back:** `class: floor · 6 rules` — `two-part-trigger · leg-1-seat-produces ·
  leg-2-non-author-review · no-delta-card-exemption · only-real-seats · leg-3-user-gate-stays`.
  Trigger fired on every item (judgment-authored writes to plugin primitives, CLAUDE.md, the
  governance ledger). Leg 1 under the plan-QA form; leg 2 the gate grader; leg 3 the user's bump.
- **Gate for this wave (D3 / D9):** carrier live (v0.111.0, 14 units graded at wave 2); plan-grade
  figures in (`P1:PASS(1) · P2:PASS(0) · P3:PASS(1)`, no dirty tree). Installed copy 0.112.0 after
  the user's `/plugin` update + `/reload-plugins --force` (48 skills, 20 agents across plugins).
- **Inventory (lead, grep over the live tree):** persona named in `agents/validator.md`,
  `plugin.json`, the router (family sentence · roster row · mount doctrine · one hygiene line),
  `validation-constitution/SKILL.md` (description + one body line), one `review-brainstorm`
  reference pointer, `CLAUDE.md` (three mentions + axis 5), `primitive-edits.md` (one
  parenthetical), `ARCHITECTURE.md` (six lines), the governance ledger (GI-004 + GI-005 detail),
  the budget ledger (one row), and three log rules (`patterns-model-tiering.seat-default-key`,
  `authoring-constitution.never-co-mounted`, `authoring-constitution.grading-routing`). The contract
  suite's preload case is `devils-advocate` × `review-specifications` — untouched. Role-noun uses
  and frozen records stay (plan §8). `.mochiko/provenance.yaml` no longer exists — D3's anchor item
  is moot; the `validator-scope-and-verbosity` anchor strings in the log are history.
- **Lead's default, stated to the user:** the ledger's GI-004/GI-005 grader-identity lines are a
  detail edit under this wave's audit (meaning unchanged), not an amend run; user said "go".
- **Tree snapshot clean (2 lead paths); P1, P2, P3 plan-only dispatched under the leg** — each plan
  returned verbatim (the notification channel caps near 4,000 chars, so each seat re-sent its
  remainder in labelled parts via SendMessage; nothing persisted); diff against the snapshot after
  all three returned: identical. Fresh generic peers (`general-purpose`, `model: opus`) grade each
  plan per `mochiko:review-seat-plan`, the skill named and its 0.112.0 render pasted (two-way).
- **P3 plan — `PLAN GRADE: P3w3 · PASS · none`** (peer-p3w3, every cited line re-read; two
  advisories inside item 3: six citations drift 1–10 lines, two pending checks worded past-tense).
  Its two questions ruled lead-level: a sweep edge surfaced by the `never-co-mounted` reword routes
  to P1's allowlist path (§2 already licenses it); the anchor question is answered by P3's own read
  of the landed 0010 (rewords only). Execution waits on P1's landing.
- **Rulings on P1/P2 questions, for the approvals:** P1 gains the `authoring-constitution` payload
  row (+66 render chars from its two rewords, ruled-HOLDS restamp, covered under unit 1's argued
  overage); P2 gains the router's `patterns-model-tiering` row count (six → five) and the axis-5
  title ("Producer↔grader pairing", the same role-noun alignment §3.2 licenses for the router
  hygiene line); the GI-005 editorial note drops — that block never named the persona, both lines
  sit under GI-004 (plan §3.2 misattributed line 126; corrected at approval). The ledger
  amendment-log row (PATCH 3.1.1 + `Ratified:` header, the v3.0.1–v3.0.3 idiom) is put to the
  user: A add the row (recommended) · B no row; P2 executes everything else meanwhile.
- **P1 plan — `PLAN GRADE: P1w3 · PASS · no items failed`** (peer-p1w3; rule texts re-measured
  784/224/134 → 772/268/156, roster 10 → 9 personas confirmed; two slips noted, no action: 0009
  has 13 rewords not 12, four rules files not two). **Approved and resumed** with the
  `authoring-constitution` budget-row amendment and a single-writer rule: P1 is the only writer of
  `primitive-cost-budgets.md` until it reports done, P2's rows wait. Peer's note on the questions
  was cut by the channel cap after the seven items; both were already ruled above.
- **P2 plan — `PLAN GRADE: P2w3 · FAIL · item 3`** (peer-p2w3): the `agents/validator.md` read
  claim said "whole, 1-93"; the file is 105 lines and strip entry A promised every section named,
  so `## Delegating Cheap Reads` (95-105, shared boilerplate homed on `patterns-model-tiering`)
  was missing from the record. Fix sent with the peer's two advisories (porcelain check as a
  before/after delta; the bare-`validator` survivor list gains the two governance references) and
  the lead rulings (GI-004 only, no GI-005 note · axis-5 title aligned · router row 71 count · the
  budget rows held until P1 lands · the amendment-log row held for the user). **Re-plan bound:
  first consumption (1 of 1 shared).** Watch: 1 of 3 first-round FAILs — under half, no halt.
## Wave 3 — execution — 2026-09-20

*(Previous entry stood at exactly its 60-line bound; the hook denied two appends — the second
counted the blank separator. User ruled A: this heading follows the entry with no blank line.)*

- **P1 LANDED** ([reports/w3-p1-disclosure.md](reports/w3-p1-disclosure.md)): `0010-validator-retirement.yaml`
  stamped (`sha256:f33dc11c…`), `migrate status` → `sequences 1..10 · state sha256:d03d5dee… · 75
  documents · 1082 rules`; `migrate validate` 0 rejecting · 106 advisory; two views re-emitted,
  emit idempotent; rule texts 772/268/156 as approved; full sweep 48 green, 0 clusters, 170
  suppressed, no allowlist row; budget rows restamped — `patterns-model-tiering` 18,347 (+7,495
  ruled-HOLDS), `authoring-constitution` 29,890 (+276 ruled-HOLDS, the +66 argued to the two D3
  rewords); description 1,209. One hook deny (heredoc into `reports/`) — used Write, no rewrite
  around. Ledger released to P2. **P3 approved and resumed** on the landed 0010.
- **P2 revision (six parts) — `PLAN GRADE: P2w3 · PASS · none blocking`** (same peer resumed;
  eight section homes checked against the file; two advisories folded — `review-plan-artifacts:3`
  and the annotated EXTERNAL-CLAIMS pointer added to the survivor list). **Approved and resumed**
  with "P1 landed"; the amendment-log row and `Ratified:` line held for the user.
- **P2 LANDED** ([reports/w3-p2-disclosure.md](reports/w3-p2-disclosure.md)): agent file deleted,
  manifest at nine agents; five router lines; two `validation-constitution` edits (description
  475, payload 14,797, both under budget); EXTERNAL-CLAIMS annotated; CLAUDE.md three lines, axis 5
  re-titled; `primitive-edits.md` parenthetical; six ARCHITECTURE.md lines; GI-004 two lines + one
  editorial note; three budget rows after P1; four strip entries (pre-edit sha `56b44c2`). Sweep:
  `mochiko:validator` 0 hits; bare `validator` 16 hits, each a listed role-noun survivor
  (CLAUDE.md and ARCHITECTURE.md contribute none). One hook deny (Notes of note 16 > 15), trimmed.
- **P3 LANDED** ([reports/w3-p3-disclosure.md](reports/w3-p3-disclosure.md)): one figure moved —
  `fidelity.rs` sequence list 1..10, from its own `migrate status`; every other census figure
  held under re-derivation; `validate.rs` census comment gains the 0010 paragraph; raw run 77/170
  and 29/56 confirm both suppressed zeros; probe removed, `matrix_similar.rs` byte-identical;
  `cargo test` 472 · sweep 48 · fmt · clippy green; kit stamped, README line in; `run.py`,
  contract README, `agents.py` untouched. One hook deny (Notes of note 32 > 15), trimmed.
- **Lead writes before the gate:** supersession annotations on the four older rulings
  (`DECISIONS.md` rows 2026-08-26 ×2, 2026-08-10, 2026-08-01), the 2026-08-26 ADR's Status, both
  older records' Status lines and index entries — all "superseded (in part) 2026-09-20"; CHANGELOG
  `[0.113.0]` written with two close-time placeholders; both manifests → 0.113.0.
- **Gate audit dispatched:** `gate-grader-w3`, plain `general-purpose` at explicit `model: opus`,
  the `validation-primitive-edit` render at 0.113.0 pasted verbatim (identical to the wave-2
  render but for the version line), eleven units, pre-pass first-hand. Contract suite (91 cases)
  started in the background at the same tree, a stall-aware watcher on its log.
- **User ruled A on both open calls:** this log entry's placement, and the ledger PATCH row
  `3.1.1` + the CLAUDE.md `Ratified:` line (the v3.0.1–v3.0.3 idiom) — P2 writes both after the
  grader's first round, then the same grader re-reads units 7 and 8 as a delta.

## Wave 3 — gate audit rounds — 2026-09-20

- **Round 1 — 9 PASS · 2 FAIL · 2 blocking** (gate-grader-w3; pre-pass first-hand: validate 0
  rejecting · 106 advisory, status 1..10 / 75 / 1082, views ≡ replay by `diff -r`, cargo test 472,
  fmt, clippy, audit 31 deps clean, full sweep 48; every budget figure reproduced by control replay
  with 0010 removed — the +66 is `.sec.independence` +44 and `.sec.scope` +22). Outcome lines:
  - `audit: unit 1 schema content (0010-validator-retirement) · gate-grader-w3 · opus · 3 files · 1 rounds · 0 blocking`
  - `audit: unit 2 skill pair (patterns-model-tiering) · gate-grader-w3 · opus · 3 files · 1 rounds · 0 blocking`
  - `audit: unit 3 skill pair (validation-constitution) · gate-grader-w3 · opus · 3 files · 1 rounds · 0 blocking`
  - `audit: unit 4 prose primitive (validator retirement) · gate-grader-w3 · opus · 8 files · 1 rounds · 0 blocking`
  - `audit: unit 5 prose primitive (router) · gate-grader-w3 · opus · 2 files · 1 rounds · 0 blocking`
  - `audit: unit 6 prose primitive (EXTERNAL-CLAIMS pointer) · gate-grader-w3 · opus · 2 files · 1 rounds · 1 blocking`
  - `audit: unit 7 repo prose (CLAUDE.md · primitive-edits.md · ARCHITECTURE.md) · gate-grader-w3 · opus · 3 files · 1 rounds · 1 blocking`
  - `audit: unit 8 ledgers · gate-grader-w3 · opus · 2 files · 1 rounds · 0 blocking`
  - `audit: unit 9 suite + eval kit · gate-grader-w3 · opus · 5 files · 1 rounds · 0 blocking`
  - `audit: unit 10 crate fixtures · gate-grader-w3 · opus · 5 files · 1 rounds · 0 blocking`
  - `audit: unit 11 CHANGELOG + manifests · gate-grader-w3 · opus · 3 files · 1 rounds · 0 blocking`
- **The two FAILs:** unit 6 — the EXTERNAL-CLAIMS annotation named `from-file-floor` /
  `evidence-floor` as the source re-read clause's new home; the grader read all 21 rules of the
  gate render and none carries it — the persona's copy retired, not re-homed. Unit 7 —
  `ARCHITECTURE.md:26` still said "10 personas". Both P2's; fix round sent (annotation → "no
  successor consumer", strip A discloses the sub-clause as retired; count → 9), together with the
  user-ruled ledger row `3.1.1` + `Ratified:` header and three grader notes (a pre-existing
  "38 skills" drift at `ARCHITECTURE.md:27`, the editorial note's date, the −237 constant on the
  `validation-constitution` row). Round 2 = the same seat resumed on the delta: units 6, 7, 8.
- **Grader's report write halted:** two Write attempts at `reports/w3-gate-audit.md` denied —
  "`## Failure narrative` is 16 lines against a budget of 15" then "`## Notes of note` is 19 lines
  against a budget of 15"; the seat stopped at the second deny and surfaced both. Verdict blocks
  arrived in six messages plus an addendum. The report lands at round 2 within the budgets.
- **P2 fix round landed** (six files): EXTERNAL-CLAIMS tail → "consumer retired v0.113.0 with the
  persona, with no successor consumer"; strip D reconciled; strip A's `## Your Judgment` bullet
  discloses the source re-read sub-clause as retired, not re-homed; `ARCHITECTURE.md:26` → 9
  personas; ledger row `3.1.1` after 3.1.0, the GI-004 note now "recorded as the PATCH 3.1.1 row;
  no fresh `/mochiko:setup` amend"; `CLAUDE.md:109` → `v3.1.1 · 2026-09-20 (AM-3 + the
  validator-retirement PATCH)`, the rest of the line byte-identical. **Disclosed:** P2 made these
  six edits with scripted in-place string replacements rather than the Edit tool — the artifact
  hook's best-effort parse did not catch them; the lead ruled Edit/Write only from here and the
  content goes to the grader unchanged. Three grader notes (skills count, note date, −237
  constant) in flight under that rule — landed by the same mechanism before the ruling reached
  the seat, verified with Read, not re-laid.
- **Round 2 (same seat, delta) — units 4 · 6 · 7 PASS; unit 8 FAIL (1 blocking, first FAIL);
  unit 11 FAIL as a knock-on (1 blocking, first FAIL).** Unit 8: the ledger's `**Version:**` field
  still 3.1.0 against the v3.1.1 stamp, and the GI-019 clause-iv paragraph's "pre-authorized PATCH
  v3.1.1" pointer reserved that number for the hook-wave's own activation row, never recorded —
  our row took it. Unit 11: the CHANGELOG entry still said "no amendment". Fixes: P2 moves the
  field and re-keys the pointer to "still owed, now v3.1.2"; the lead rewrites the CHANGELOG's
  governance sentence. The owed hook-wave PATCH row is surfaced to the user as governance debt.
  - `audit: unit 4 prose primitive (validator retirement) · gate-grader-w3 · opus · 8 files · 2 rounds · 0 blocking`
  - `audit: unit 6 prose primitive (EXTERNAL-CLAIMS pointer) · gate-grader-w3 · opus · 2 files · 2 rounds · 0 blocking`
  - `audit: unit 7 repo prose (CLAUDE.md · primitive-edits.md · ARCHITECTURE.md) · gate-grader-w3 · opus · 3 files · 2 rounds · 0 blocking`
  - `audit: unit 8 ledgers · gate-grader-w3 · opus · 2 files · 2 rounds · 1 blocking`
  - `audit: unit 11 CHANGELOG + manifests · gate-grader-w3 · opus · 3 files · 2 rounds · 1 blocking`

## Wave 3 — close — 2026-09-20

- **Round 3 (same seat, delta on units 8 and 11) — both PASS. Wave: 11 PASS · 0 blocking; four
  blocking findings caught over three rounds; no unit reached its second FAIL.** Unit 8: the
  `**Version:**` field moved in lockstep with the stamp; the GI-019 pointer re-keyed to "still
  owed, now v3.1.2" — the grader checked v3.1.2 is free and the hook-wave PATCH is genuinely
  pre-authorized at the 3.1.0 row. Unit 11: the governance sentence now names the v3.1.0 → v3.1.1
  PATCH. Report: [reports/w3-gate-audit.md](reports/w3-gate-audit.md) (round 3).
  - `audit: unit 8 ledgers · gate-grader-w3 · opus · 2 files · 3 rounds · 0 blocking`
  - `audit: unit 11 CHANGELOG + manifests · gate-grader-w3 · opus · 3 files · 3 rounds · 0 blocking`
- **Correction to the "governance debt" reading:** the hook wave's PATCH v3.1.1 was a scheduled,
  pre-authorized row (`hook-enforced-artifact-schema/wave5-bump-patch.md`, gated on the crate
  publish and its gate-6 run), not an unrecorded one. Our row took the number, so that plan, the
  ledger pointer and BACKLOG's "what is left" item 2 now read v3.1.2 (re-key notes dated
  2026-09-20; no new BACKLOG item needed).
- **Lead release checks at the final tree:** `migrate validate` 0 rejecting · 106 advisory ·
  `migrate status` 1..10 / 75 / 1082 · views ≡ replay (`diff -rq` clean) · `cargo test -p
  mochiko-cli` 472/0 · fmt · clippy · manifests 0.113.0 ×2. Gate 6 (contract suite, 91 cases)
  running with a stall-aware watcher.
- **Gate 6 — contract suite 91/91**: `contract suite: 91/91 cases passed, 91 ran, 303
  measurement(s) recorded and not asserted`, exit 0 (log `contract-suite-w3.log`, session
  scratchpad). No crash, no fix-on-sight. **Every wave-3 gate green**; the CHANGELOG's Results
  and Gates sentences filled.
- **Disclosure line:** `floor: tripped · seats: P1 (schema + tiering pair) · P2 (prose sweep) ·
  P3 (suite · crate fixtures · eval kit) / peer-p1w3 · peer-p2w3 · peer-p3w3 · gate-grader-w3 ·
  plans: P1:PASS(0) · P2:PASS(1) · P3:PASS(0)`. Every seat a plain `general-purpose` spawn at
  explicit `model: opus`; no persona seat this wave. The lead wrote the wave plan, this log, the
  CHANGELOG entry, the manifests, the supersession annotations and the operating-doc landing.
- **Halts and watches:** no halt. The D8 item-7 watch stayed clear (1 of 3 first-round plan
  FAILs, no second re-plan, no dirty tree). Three hook denies surfaced and never rewritten around
  (this log at its bound ×2, the grader's report ×2, three seats' Notes budgets); one process
  disclosure (P2's scripted in-place writes, ruled Edit/Write only from then). No unit reached
  its second FAIL; the wave's fix-and-re-audit ran three rounds on the one seat.
- **Landing set (this close):** record Status + index entry → wave 3 BUILT, path complete ·
  DECISIONS 2026-09-19 row → all three waves built; 2026-09-03 producer-plan row → build
  complete · BACKLOG producer-plan item → trail as DONE; the D9 watch item carries the wave-3
  figures; the hook wave's "what is left" item 2 re-keyed to PATCH v3.1.2 · ROADMAP "Floor
  builds" row → the producer-plan build complete · KM invariants run (a false bound trip on the
  watch item, measured to the wrong boundary; the count watch stands at 87 vs 83, the delivery
  sweep still owed and offered) · commit pending the user.
- **Wave 3 CLOSED 2026-09-20 at v0.113.0 — the `author-grader-consolidation` path is built end
  to end.** Open after close: the D9 gate-form watch (five-wave read), the hook wave's owed PATCH
  v3.1.2, and the install refresh the user runs (agents 20 → 19 across plugins).
