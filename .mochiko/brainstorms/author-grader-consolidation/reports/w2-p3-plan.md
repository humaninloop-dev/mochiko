---
report: disclosure
round: 2
seat: P3 (crate tests)
wave: wave 2 — the plan-QA leg
unit: crates/mochiko-cli/tests/{fidelity,validate,matrix_similar,views,render}.rs
status: plan only — nothing outside this file was written
measured_against: the tree at 9f256f0, sequences 1..8, 74 documents, 1067 rules
---

## Scope

Re-key the five frozen-census test files to the post-`0009-plan-qa-leg.yaml` state, on the pattern the wave-1 re-key set at `2ec189b` and the CI fix at `8968d43`. No crate source is touched and no version bump is made: `0.2.0` stands because wave 2 carries no grammar change.

`0009` adds one imported skill document (`review-seat-plan`) and rewords existing rules. Every reword keeps its id and its pin, so only the skill-side counts, the document count, and the sequence list move; the command-side counts are expected to hold. The exact figures are read from the tree at resume — `mochiko-cli migrate status --log-dir plugins/mochiko/migrations` and the emitted render — never copied from the wave plan, which is explicit that P1 confirms the new skill's floor pin from the render and whose own §3.1 item 6 leaves the floor count internally contradictory (it says six, then names five and rules out the sixth).

This is a fixture refresh, not a finding. Any number that does not reproduce, and any cluster that survives the allowlist, halts to the lead rather than being written over.

## Write set

The five files named in the wave plan §2 P3 row, and nothing else:

- `crates/mochiko-cli/tests/fidelity.rs`
- `crates/mochiko-cli/tests/validate.rs`
- `crates/mochiko-cli/tests/matrix_similar.rs`
- `crates/mochiko-cli/tests/views.rs`
- `crates/mochiko-cli/tests/render.rs`

No crate source, no `Cargo.toml`, no migration, no view, no allowlist row, no strip entry, no CHANGELOG line, no manifest. `scripts/similar-rules-allowlist.yaml` is P1's declared path: the clusters check below reads it and never writes, renames, moves, or otherwise disturbs it. The only file written outside the repository is the empty scratch allowlist named in the clusters section, under this session's scratchpad.

`crates/mochiko-cli/tests/fixtures/genesis-corpus/**` is a frozen input and is never touched.

## Reads

Already read for this plan: the wave plan §2, §3.1, §3.3; `.claude/rules/mochiko/rust-cli.md`; the two precedent re-keys (`git show 8968d43` and `git show 2ec189b -- <the five files>`); all five test files' census sites; `plugins/mochiko/migrations/README.md` "The anchor rule"; `0007`'s and `0008`'s reword and mint ops; the frozen provenance sidecar; `mochiko-cli migrate status`; `mochiko-cli home` and `mochiko-cli template report-envelope` for this file's own shape.

Read for the clusters route specifically: `crates/mochiko-cli/tests/matrix_similar.rs:990`, where the corpus sweep passes an explicit allowlist path into `similar::clusters(&state, similar::DEFAULT_THRESHOLD, Some(&allowlist))`; the in-file scratch helper `write_allowlist` at `:689-699`, which already writes throwaway allowlists outside the source tree; and `crates/mochiko-cli/src/similar.rs` for `clusters`, `find_allowlist` and `ALLOWLIST`.

To be read at resume, after P1 lands: `plugins/mochiko/migrations/0009-plan-qa-leg.yaml` whole, every op; the regenerated `.mochiko/schema-views/skills/review-seat-plan.yaml`; `mochiko-cli migrate status` for the document, rule and sequence figures; `mochiko-cli rules review-seat-plan --section preamble` for the floor pin and index; the four reworded sidecar-anchored rules in the replayed state; P1's report for the allowlist rows it added.

## Assertions to move

Every right-hand value is read from the tree at resume, never predicted. `N` is `review-seat-plan`'s rule count and `F` its floor count, both from the render. Each site gains a comment naming `0009-plan-qa-leg.yaml` and the ruling it executes — `2026-09-03 producer-plan-enforcement D8` — in the wave-1 comment's form.

- `fidelity.rs:170` docs `74` → `migrate status` documents · `:172-176` `vec![1,…,8]` → `vec![1,…,9]`, comment tail gains "`0009`'s plan-QA leg" · `:625` skill rules `738` → +N · `:626` total `1067` → +N · `:627` skill floors `252` → +F · census comment `:611-623` gains a `0009` paragraph stating one imported skill document, N rules, F floors, every other op a reword that keeps its id and pin, so the command side and the fail set do not move.
- `validate.rs:1101` docs `74` → `75` · `:1120` `738` → +N · `:1121` `1067` → +N · `:1122` `252` → +F · comment `:1108-1118` gains the same `0009` paragraph. Mechanical twin of `fidelity.rs`: identical figures, independently re-derived, different reader.
- `matrix_similar.rs:1041` scanned `1067` → +N · `:1042` scored `156_764` → the figure the run reports, never hand-computed · `:1044` suppressed `171` → `171` plus P1's new rows · comment `:1022-1039` extended with `0009`'s mechanism (it retires nothing, so every bucket it touches only grows) and with the suppressed-zero mapping below.
- `views.rs:269`, `:342`, `:447` each `74` → `75`, each with the `0009` comment in the `0008` comment's form · `:298`'s divergence message still reads "of 73 views", stale since `0008`, re-keyed to the same figure.
- `render.rs:1169` `checked` `37` → `38`, comment "thirty-one skills" → thirty-two · `:1053`'s `measured > 200` bound holds, but its "36 primitives" comment is stale since `0008` and is corrected on sight.

**16 assertions certain to move**, plus 4 conditional sites named below and 2 stale comment/message strings.

## Pins you expect unchanged

Rewords keep their ids, their class and their `anchor:` field — `0007`'s reword of `patterns-model-tiering.class-key-session-tier` carries only `text:` and names the new ruling inside that text, which is the precedent `0009`'s rewords are planned against. So:

- `fidelity.rs:624` / `validate.rs:1119` command rules `329`, and `:628` / `:1127` command floors `117`: `0009`'s command-side work is entirely rewords plus the `common.plan-approval-producers` reword the three stubs inherit. A reword mints nothing.
- `fidelity.rs:629` / `validate.rs:1137` fail nodes `36`: `0009` mints no `kind: fail` rule.
- `fidelity.rs:991` the `patterns-model-tiering` floor pin `8`: `0009` carries no op on that skill.
- `fidelity.rs:496`/`:516` the sidecar census `597` (both): it reads the frozen `genesis-corpus` sidecar and genesis-only state, neither of which any migration moves.
- `render.rs` `IMPLEMENT_FLOORS` (35) and `REVIEW_BRAINSTORM_FLOORS` (9), and the widest-index pin `("implement", 966, 1000)`: `impl.sound-loop-floor` is reworded, so its id and its place in the index are unchanged, and the floors line lists ids only.
- `validate.rs:2622` pointer count `84`: the new skill's rules point by name (`mochiko:review-seat-plan`), and a name-shaped pointer is not counted. Verified against the report at resume, not assumed.

Each of these is re-derived at resume from the replayed state; an unchanged pin is confirmed, never skipped.

## Clusters confirmation

`matrix_similar.rs:1043` asserts `clusters == 0`, and after `0008` that zero is a *suppressed* zero — three allowlist rows stand behind it. Writing `0` without classifying it would hide a real near-dup, so the four-step check runs before any similarity number is written:

1. **Pinned run.** `MOCHIKO_FULL_SIMILAR=1 cargo test -p mochiko-cli --test matrix_similar -- --nocapture` with the repo allowlist in place; read `scanned`, `scored`, `clusters`, `suppressed_hits` off the assertion failures.
2. **Raw run, inside my own file.** `matrix_similar.rs:990` passes an explicit allowlist path into `similar::clusters`, so the raw set is reachable without touching P1's path: a transient `#[test]` added to `matrix_similar.rs` builds the same `corpus_state(|_| true)` and passes an empty allowlist written to `/private/tmp/claude-501/-Users-deepeshadmin-Documents-GitHub-mochiko/1bfb0a40-a0fc-41dc-a33a-7193653cf38d/scratchpad/raw-corpus-allowlist.yaml` holding exactly `suppressions: []`, printing the unsuppressed clusters. The empty sequence is required: `load_allowlist` at `crates/mochiko-cli/src/similar.rs:783` takes `as_sequence()` on the key, so a bare `suppressions:` parses as null, warns "carries no `suppressions:` list", and returns early down a different path. An empty file rather than `None` keeps the code path identical to step 1, so the two runs differ only in the rows. The probe is removed before the gate diff, and the final `cargo test` run is made after its removal.
3. **Map.** Every cluster present in the raw run and absent in the pinned run must map to one exact allowlist row, by rule-id pair, in the set P1's report says it added. An unmapped disappearance is a detector question, not a fixture refresh.
4. **Record.** If the raw count is non-zero, the zero is suppressed: the test comment names each row, its id pair, and why it was adjudicated keep-distinct — the wave-1 comment at `matrix_similar.rs:1029-1039` is the exact model, written so a later dropped row reads as an allowlist edit rather than a detector regression. If the raw count is itself zero, that is recorded as a naive zero in the same comment.

A cluster that survives the allowlist halts to the lead under the wave plan §3.1 item 4 converge-via-`common.yaml` doctrine. The same four steps run over the command-family tuple at `:971`, whose `suppressed` figure `60` moves only if P1 added command-side rows.

## Command sequence

Run from the repository root, in this order, each gate read before the next is started:

```
cargo build
cargo test -p mochiko-cli
MOCHIKO_FULL_SIMILAR=1 cargo test -p mochiko-cli --test matrix_similar
cargo fmt --all --check
cargo clippy --all-targets -- -D warnings
```

The first two run before any edit as well, to reproduce the post-`0009` failures and confirm the failing set is exactly the sites this plan names. A failure outside that set is a stop, not a widening: the wave-1 precedent is that P3 halted rather than editing files outside its declared scope, and the lead recorded the widening. `cargo audit` is a CI-side gate on the same crate and is not re-run here; no dependency moves.

## Stops and hand-offs

- **Before execution.** This plan is graded by a fresh peer against the seven items; the lead approves only a PASS and resumes this seat with the approval and the plan quoted. Nothing in the write set is touched until then.
- **P1 first.** Execution begins only after P1 lands `0009-plan-qa-leg.yaml` and re-emits the views. Every figure is read from that landed tree. If P1's landed migration differs from §3.1 — a different rule set, a different floor count, a command-side mint — the plan's expected-unchanged list is re-derived, and a command-side mint is reported to the lead before any census line is written.
- **Re-plan bound.** One round on the shared counter. A second consumption goes to the user, per wave plan §0.
- **After execution.** The gate grader reads the diff of these five files as one unit under the new gate form; on a FAIL this seat fixes and the same grader is resumed on the delta only, and a second FAIL halts to the user.
- **Never.** No `git` mutation, no commit, no push, no version bump, no edit outside the write set.

## Risks

1. **A reworded rule's `anchor:` moves, and `the_sidecar_anchors_ride_their_rules` fails with "the anchor moved".** Four of `0009`'s reword targets carry frozen sidecar anchors of `2026-08-13 charter-ritual-balance`: `patterns-sound-loop.leg-1-seat-produces`, `patterns-sound-loop.disclosure-line`, `arch.sound-loop-floor`, `feat.sound-loop-floor`. The wave plan calls for a "rule-level anchor `… D8`" on these, which reads either as the `anchor:` field or as naming the ruling in the text. `0007`'s precedent is the latter, and the grammar also offers `set-rule-field`. That test has an escape hatch for a *retired* anchored rule (`RETIRED_SIDECAR_ANCHORS`) but none for a *re-anchored* one. If P1 moves an anchor field, re-keying the test is a protected-content question for the lead, not a silent fixture edit — this seat reports it and stops.
2. **The transient raw-run probe survives into the gate diff.** The clusters check adds a temporary `#[test]` to `matrix_similar.rs`; leaving it in would ship a scratchpad-absolute path in a committed test and would read to the grader as unplanned scope. Mitigated by removing it before the final `cargo test` run, and by a `grep` for the scratchpad path and for the probe's function name across all five files as the last step before the diff gate — a hit is a stop. The probe stays inside my own write set throughout, and P1's `scripts/similar-rules-allowlist.yaml` is only read.

## Execution — round 1, halted at Risk 1

**HALTED at Risk 1 before any edit.** Nothing in the write set was touched; `git status --porcelain crates/` is empty. The probe was never added, so no scratchpad path exists in any test file.

`0009` carries five `op: set-rule-field · field: anchor` ops setting `2026-09-03 producer-plan-enforcement D8` (lines 20-24 and 121-140). Three targets carry frozen sidecar anchors of `2026-08-13 charter-ritual-balance` and are therefore re-anchored, not merely reworded: `patterns-sound-loop.leg-1-seat-produces`, `arch.sound-loop-floor`, `feat.sound-loop-floor`. The other two, `impl.sound-loop-floor` and `arch.author-grader-separation`, carry no sidecar anchor. `patterns-sound-loop.disclosure-line` is reworded with no `set-rule-field`, so its anchor survives. Pre-edit, `cargo build` was clean and `cargo test -p mochiko-cli --no-fail-fast` gave 9 failures — 8 of them the census sites this plan named, 1 the declared stop:

```
fidelity.rs:559  arch.sound-loop-floor: the anchor moved
  left "2026-09-03 producer-plan-enforcement D8" / right "2026-08-13 charter-ritual-balance"
fidelity.rs:170 75/74 · :625 753/738 · validate.rs:1101 75/74 · views.rs:269 :342 :447 75/74
render.rs:1169 38/37 · matrix_similar.rs:947 (329,12607,0,56)/(329,12607,0,60)
```

Read from the tree: 75 documents · 1082 rules · sequences 1..9 · `review-seat-plan` pin 5 floors · N = 15. The command-family `suppressed` figure *fell* 60 → 56 — four rows stopped matching because the rewords changed the texts. The sidecar walk has an escape hatch for a retired anchored rule and none for a re-anchored one, so widening it is a protected-content question for the lead. Two routes: **(a)** P1 drops the three anchor ops on sidecar-anchored rules and the ruling is named in the rule text, the `0007` precedent, no test change; **(b)** the re-anchoring stands and `fidelity.rs` gains a documented fixed-length re-anchored list mirroring `RETIRED_SIDECAR_ANCHORS`. Recommend (a); the ruling is the lead's.

## Execution

**Complete — all four layers green.** Changed exactly the five declared files, `+61 −26`: no crate source, no `Cargo.toml`, no version bump (0.2.0 stands). The transient probe was removed before the final runs, and `grep` over `crates/` for its name and for the scratchpad path returns nothing. `0009` re-stamped as ruled: `patterns-sound-loop.leg-1-seat-produces`, `arch.sound-loop-floor` and `feat.sound-loop-floor` keep `2026-08-13 charter-ritual-balance`; the two surviving anchor ops target rules the sidecar does not carry, so the sidecar walk passes untouched.

Read from the tree: 75 documents · 1,082 rules · sequences 1..9 · state `d5cb369a…` · `review-seat-plan` pin 5. Floor counts re-derived independently, by emitting the views to the scratchpad and counting rule-level `class: floor` lines: skills 257, commands 117, 75 view files. 16 assertions moved. The four conditional sites resolved as: the command-family tuple moved, corpus clusters held at 0, and the pointer count 84 and the `patterns-model-tiering` pin 8 both held untouched. Both stale strings were corrected.

Clusters, four steps on both limbs. Corpus raw, against an empty scratchpad allowlist: 77 clusters over 170 edges; pinned: 0 clusters, 170 suppressed. Command family raw: 29 clusters over 56 edges; pinned: 0 and 56. Every raw edge maps to a suppressing row and none survives, so both zeros are recorded in the test comments as suppressed zeros. The family figure *fell* 60 to 56 because rewording one side of an allowlisted pair retires its hit without touching the allowlist; P1's one command-side row adds one back.

```
cargo build                                        clean
cargo test -p mochiko-cli --no-fail-fast           472 passed · 0 failed
MOCHIKO_FULL_SIMILAR=1 … --test matrix_similar      48 passed · 0 failed · 92s
cargo fmt --all --check                            exit 0
cargo clippy --all-targets -- -D warnings          exit 0
```
