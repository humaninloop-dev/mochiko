# Build log — orchestrator-model-selection

Append-only. One `##` entry per event; the wave plan is `wave1-build.md`.

## 2026-09-19 — wave 1 opened, seats dispatched, audits round 1

- **Base.** Worktree `seat-default-key` off `main` (v0.108.0, log sequences 1..4), created after the
  record's acceptance; the session directory and index entry copied in from the
  `primitive-evals-v2` checkout where the session ran. Target version 0.110.0 (0.109.0 is claimed on
  that branch); migration sequence 0007 (0005 and 0006 are claimed there too — renumbered from 0006
  mid-wave on the lead's re-check of the branch's log; rule content unchanged).
- **Landing (lead, transcription).** `DECISIONS.md` row · `model-tiered-seats` D5/F6 superseded
  (index + record) · BACKLOG build item + dogfood-watch item · ROADMAP token-epic Next row, Later
  row retired · session index entry accepted+landed.
- **Floors.** `floor: tripped · seats: P1 + P2 (staff-engineer, each on a lead-approved plan) /
  V1 + V2 (validator, fresh seats)`. Transport: single writer per surface, all messaging through the
  lead, fan-in confirmed on each deliverable.
- **P1 (skill pair).** Plan approved with flag rulings A yes / B `must` / C no command edited.
  Delivered migration `0007-seat-default-key.yaml` (11 ops: 1 supersede, 5 mints — 2 floors — 3
  floor rewords, 2 section rewords), views regenerated (view ≡ replay), `expected-skills.json`
  floor set 6 → 7, description 998 → 1,208 chars, tagline + Overview + router row reworded, strips
  in `patterns-model-tiering.md` and `mochiko.md`. Gates: validate 0 rejecting · host suite 4/4.
  Payload 14,696 → 17,859.
- **P2 (personas).** Plan approved with option-B agents-row wording and ownership widened to
  `ARCHITECTURE.md` 59/61/62; P2's further three `schemas/` dead-pointer repairs in the same file
  ruled in scope post hoc. Four frontmatter flips, four strips, seven ARCHITECTURE.md locations.
- **V2 → PASS** (5/5 items; advisory N1 citation fix applied; N4 confirmed: the option-B text is
  the lead's; N2/N5 move with the bump; N3 pre-existing, out of scope).
- **V1 → FAIL, overage HOLDS** (12/12 criteria pass; two fidelity defects in floor text + two minor):
  F1 second "strong tier" occurrence in `class-key-session-tier` · F2 the `fable` caveat missing
  G8's third item · F3 `inherit` bar stated twice · F4 report §1 stale at 0006. Routed to P1 in
  place; V1 confirm pass to follow.
- **Deferred.** D7 item 4 (persona-kit runner `ARM_MODEL` → per-persona default): the runner is
  on `primitive-evals-v2`, not `main`; carried in the BACKLOG build item.
- **Cross-tree note.** The original checkout holds stale pre-worktree copies of the session
  directory and the index entry; removal awaits the user's ruling.

## 2026-09-19 — fix round, audits close, release, landing closed

- **P1 fix round.** V1's F1-F4 applied in place on `0007-seat-default-key.yaml`: two floor-text
  fidelity fixes (a second "strong tier" occurrence the vocabulary-collision flag missed;
  a missing third `fable` caveat), one duplication removed (`inherit` bar single-homed), and
  P1's own report re-keyed to the file's final state. New hash
  `sha256:0287aa5d0681d9ebac8a370c7092556e5c9b4a32142063d2ff3773f3856d4ef3`. Payload 17,864
  (body 3,112 + render 14,752).
- **V1 confirm pass → PASS.** Overage still ruled HOLDS at +7,012 over the 10,852 budget — a
  genuine new obligation set (replay-diffed against the log with 0007 removed; nothing from the
  three prior strip entries returned). All twelve skill-pair criteria pass.
- **Mid-wave correction (lead).** The `primitive-evals-v2` branch already claims migration
  sequences 5 and 6; renumbered `0006-seat-default-key` → `0007-seat-default-key` across the
  migration file, both strip files, the session record, the wave plan, DECISIONS, BACKLOG.
  P1 re-stamped and re-ran both gates post-rename (green); P2 fixed the same reference in its
  four persona strips once flagged.
- **Cross-tree note.** The stale pre-worktree copies of the session directory and index entry in
  the original `primitive-evals-v2` checkout were removed by the user directly.
- **Release gate.** `cargo test -p mochiko-cli`: 11/11 green (crate untouched). Full contract
  suite in the sandbox: **82/82 cases passed, 285 measurements recorded, exit 0** — every command
  and skill delivery/absence case, both fixture cases, the three mechanism cases, the two-session
  preload case, and the four host cases, all green; no SKIPPED. Host re-run after the bump: 4/4
  green.
- **Bump.** `plugin.json` and `marketplace.json` 0.108.0 → **0.110.0**; `ARCHITECTURE.md`'s stamp
  line moved with it; `CHANGELOG.md` gained the `[0.110.0]` entry with the migration, gate, and
  overage summary; `.mochiko/memory/primitive-cost-budgets.md`'s `patterns-model-tiering` row
  carries the ruled overage.
- **Landing closed.** `DECISIONS.md` row status → `ruled + built (v0.110.0, worktree
  seat-default-key)`; `model-tiered-seats` D5/F6 supersession note already in place; BACKLOG build
  item compressed to one line and moved to `.mochiko/archive/backlog-trail.md`; the dogfood-watch
  item stays open in BACKLOG; ROADMAP token-epic row updated to built; brainstorms index and the
  session record's own status line both read built.
- **Deferred, unchanged.** D7 item 4 — the persona-kit runner's `ARM_MODEL` constant → a
  per-persona default — carried in the dogfood-watch item for the `primitive-evals-v2` merge.
- **Disclosure.** `floor: tripped · seats: P1 + P2 (staff-engineer) produced / V1 + V2 (validator)
  reviewed`. Wave closed.

## 2026-09-19 — CI fix (PR #37): three crate test files stale against 0007

- **Trigger.** PR #37 CI failed at the `Test` step — four `fidelity.rs` assertions frozen before
  migration `0007-seat-default-key.yaml` landed; census counts, a needle string, and a floor pin
  all stale. Governed by `.claude/rules/mochiko/rust-cli.md` — lead-approved plan
  (`wave2-ci-fix.md`), producer plans first, independent non-author review before commit.
- **P3 (crate fix).** Original plan's four `fidelity.rs` edits applied and green; read-back
  surfaced two more stale-pin files outside the plan's named scope (`validate.rs`,
  `matrix_similar.rs`) carrying the identical drift class — P3 correctly stopped rather than
  silently widening its own mandate.
- **Scope widening (lead, mid-wave).** `validate.rs`'s three-number census fix approved as
  mechanical (identical evidence already vetted for `fidelity.rs`). `matrix_similar.rs`'s full
  similarity-sweep constants gated on an explicit condition: verify `clusters == 0`
  independently before writing a number, hard-stop and escalate to the lead under the
  near-dup-convergence doctrine if a real cluster surfaced instead.
- **P3 (widened scope).** Both files fixed. `clusters` reproduced independently at `0` — a
  fixture refresh, not a near-dup finding; the coincident `scored` drop explained and recorded
  in the test comment (pairs bucket by `kind:`, the retired rule was kind-less in the large
  `constraint` bucket, 0007's five mints land in small typed buckets).
- **V3 → PASS.** All seven checks independently re-derived from source, none taken from P3's
  say-so: census re-derived by a standalone log replay (705/229, corrected the plan's own
  floor-count hedge — the superseded rule was itself a floor); both reworded needles confirmed
  live; `RETIRED_SIDECAR_ANCHORS` confirmed exactly one id, fixed-length array so silent growth
  is a compile error; a full-tree grep confirmed no other file references the stale numbers
  outside the frozen `tests/fixtures/genesis-corpus/` input (must not be touched); `cargo fmt`,
  `cargo clippy`, `MOCHIKO_FULL_SIMILAR=1 cargo test --all` all run independently and green (351
  passed); `matrix_similar`'s four numbers reproduced on both the source build and the installed
  binary, identical; the `scored`-count mechanism and magnitude both verified against
  `src/similar.rs` source. One non-blocking advisory: `evals/contract/README.md:266`'s prose
  floor-id count was stale (228 → 229, and the 4→6 ruling chain missing its 4→6→7 hop) — no
  live pin depends on it, fixed on sight as a fix-on-sight integrity repair (transcription, not
  judgment — `mochiko:patterns-sound-loop` exemption).
- **Gates.** `cargo fmt --all --check`, `cargo clippy --all-targets -- -D warnings`,
  `MOCHIKO_FULL_SIMILAR=1 cargo test --all`: 351 passed, 0 failed, exit 0 across all three.
- **Files.** `crates/mochiko-cli/tests/{fidelity,validate,matrix_similar}.rs` — no schema,
  migration, strip, or view touched; not a `plugin.json`-bump-class change.
- **Disclosure.** `floor: tripped · seats: P3 (staff-engineer) produced / V3 (validator)
  reviewed`. Wave closed.
