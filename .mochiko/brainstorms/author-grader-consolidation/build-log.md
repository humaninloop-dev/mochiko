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
  DECISIONS status · commit suggested, never run.
