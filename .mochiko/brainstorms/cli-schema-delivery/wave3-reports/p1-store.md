# P1 — store move + crate: report

Built per `p1-store-plan.md`, lead-approved 2026-09-04. Nothing committed.

## What was built

**The move.** `git mv migrations plugins/mochiko/migrations` — the log and its README, both bytes-unchanged. No repo-root `migrations/` remains. `migrations/README.md` moved unedited as ruled: it carries no self-referencing repo path (it says "This directory", and its only two commands, `mochiko-cli migrate validate` and `migrate status`, take no path argument).

**The legend.** `render::preamble` (`crates/mochiko-cli/src/render.rs`) gained a fixed `LEGEND` const, pushed between the `pins` block and the `sections` block, unconditional for commands and skills. Six lines verbatim from wave-3 plan §2. It is fixed text, derived from nothing in state, so the preamble's end line still reports `0 rules`.

**Paths re-pointed.** `tests/fidelity.rs` (`log_dir()`, the module doc, the committed-file `expect`, the regenerate hint), `tests/matrix_similar.rs` (the `find_allowlist` walk), plus three crate doc comments that named the old path (`tests/render.rs` module doc, `src/lib.rs`, `src/genesis.rs`).

**CI.** `.github/workflows/ci.yml` — `"migrations/**"` became `"plugins/mochiko/migrations/**"` in both the `push` and `pull_request` path filters. No other workflow change.

**Rules file.** `.claude/rules/mochiko/rust-cli.md` — the repo-root `"migrations/**"` glob dropped from `paths`; the opening paragraph's "at the repo root `migrations/` until then" clause struck, "from wave 3" reduced to "since wave 3".

## Test tally

`cargo test --all`: **302 passed, 0 failed**, across 14 test binaries. Two tests added (`the_preamble_carries_the_fixed_legend_block`, `a_skill_preamble_carries_the_same_legend`), one existing assertion tightened (below).

Test-first was honored on both changes. The re-pathed fidelity suite went red first (7 of 10 failing, `plugins/mochiko/migrations: the migration log cannot be read`) and green after the move; the two legend tests went red first and green after the render edit.

| layer | result |
|---|---|
| `cargo test --all` | 302 passed, 0 failed |
| `cargo fmt --all --check` | clean (after one `cargo fmt --all`, see deviations) |
| `cargo clippy --all-targets -- -D warnings` | clean |
| `mochiko-cli migrate validate --plugin-root plugins/mochiko` | `0 rejecting · 105 advisory` |

`the_committed_genesis_regenerates_byte_identically` passes at the new path, which is the second independent witness that the genesis file survived the move intact.

## State hash, before and after

Before (recorded in the plan, run before any change):

```
log migrations · grammar 1 · sequences 1..1 (1 migration)
state sha256:8b61de5a3b4cca8ed581df70f6ec036d8f05a8e67de06b66c6f1ae9b86c8bdd4 · 50 documents · 1016 rules
```

After (`mochiko-cli migrate status --plugin-root plugins/mochiko`, no `--log-dir`):

```
log plugins/mochiko/migrations · grammar 1 · sequences 1..1 (1 migration)
state sha256:8b61de5a3b4cca8ed581df70f6ec036d8f05a8e67de06b66c6f1ae9b86c8bdd4 · 50 documents · 1016 rules
```

The hash, the document count and the rule count are identical. Only the log directory line differs, and the post-move invocation resolves through `--plugin-root` alone.

## The move, as git sees it

`git diff --stat -M --cached`:

```
 {migrations => plugins/mochiko/migrations}/0001-genesis.yaml | 0
 {migrations => plugins/mochiko/migrations}/README.md         | 0
 2 files changed, 0 insertions(+), 0 deletions(-)
```

Pure rename detection on both files, zero insertions and zero deletions. The working-tree edits are separate:

```
 .claude/rules/mochiko/rust-cli.md          |  5 ++-
 .github/workflows/ci.yml                   |  4 +--
 crates/mochiko-cli/src/genesis.rs          |  8 ++---
 crates/mochiko-cli/src/lib.rs              |  2 +-
 crates/mochiko-cli/src/render.rs           | 14 +++++++++
 crates/mochiko-cli/tests/fidelity.rs       | 17 ++++++-----
 crates/mochiko-cli/tests/matrix_similar.rs |  4 ++-
 crates/mochiko-cli/tests/render.rs         | 49 ++++++++++++++++++++++++++++--
 8 files changed, 82 insertions(+), 21 deletions(-)
```

## Deviations from the plan

1. **One extra test edit, not in the approved list.** `a_skill_preamble_omits_moments_and_the_fail_pin` asserted `!out.contains("kind: fail")` as a proxy for the fail pin being command-only grammar. The legend's `enforces:` line reads "on a kind: fail rule names the rules…", so that substring is now delivered to skills too and the test failed. The assertion was tightened to the pin's own rendered shape, `!out.contains("- kind: fail · ")`, with a comment saying why. This preserves what the test was checking rather than weakening it — the legend text itself is verbatim per §2 and was not touched. Flagged for V1: this is the one behavior-adjacent test change beyond re-pathing.

2. **A second legend test, where the plan named one.** `a_skill_preamble_carries_the_same_legend` asserts the block is delivered on the skill path too. The legend is unconditional by design, and deviation 1 proves a skill preamble is a distinct surface worth pinning.

3. **Fewer re-pathings than §2 anticipated.** `tests/views.rs` and the `shipped_state` helper in `tests/render.rs` both read `plugins/mochiko/schemas`, never the log, so neither needed a change. `tests/cli.rs` needed none, as §2 predicted: its remaining `migrations` occurrences are scratch plugin roots building their own log subdirectory, which is the resolution constant `LOG_DIR_NAME` rather than a repo path.

4. **`cargo fmt --all` was run once** to reflow two of my own edits (a wrapped `expect` in `tests/fidelity.rs`, a chained `find` in `tests/render.rs`). `--check` is clean now. No pre-existing formatting was disturbed.

## Measurements for the lead's §5 hand-off

| figure | value |
|---|---|
| rendered `preamble`, post-legend | 2,055 chars / 2,102 bytes |
| rendered `brainstorm`, all seven blocks, post-legend | **10,700 bytes** |
| the legend block itself | 612 bytes / 600 chars |
| the same seven blocks less the legend | 10,088 bytes |

That last row is plan §0's pre-legend figure exactly: 10,700 less the legend's 612 is 10,088, so §0's number is a byte count and the legend is the whole of the difference. Against the 12,819-byte raw-read baseline the post-legend delta is **−16.5 %**, comfortably inside abort criterion (2). The preamble's pins render `kind: fail · 4 rules` and `class: floor · 7 rules`, matching §0.

The legend figure is measured from the rendered preamble, sliced between its `legend` and `sections` headers, so it counts the const's leading newline as delivered. An earlier draft of this report said 599 bytes; that was the character count of a scratch file written without the leading newline, and it is what produced the wrong 10,101 subtotal.

## Left undone, by scope

The `evals/contract/run.py:460` skew-log hand-off is P3's through the lead, as ruled. Nothing under `plugins/mochiko/` changed except the moved `migrations/` directory. No commit was made.
