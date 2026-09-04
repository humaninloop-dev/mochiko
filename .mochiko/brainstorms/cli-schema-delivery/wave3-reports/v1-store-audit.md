# V1 — P1 store move + crate: audit

**Verdict: FAIL** — on item 10 alone. The move, the legend, the re-pathing, the four layers and
every workflow and rules edit are correct and need no rework. Two figures in the report's
measurement table are wrong, and the sentence they support reaches the right conclusion by
accident rather than by the exact identity that is available. Fix the report, not the code.

Unit reviewed: `git diff -M -- crates/ .github/workflows/ci.yml .claude/rules/mochiko/rust-cli.md`
plus the staged renames. `plugins/mochiko/commands/brainstorm.md` and `plugins/mochiko/hooks/`
were ignored as seat P2's.

## Per-item findings

**1. Pure rename — PASS.** `git diff --cached -M --numstat` reports `0 0` for both
`{migrations => plugins/mochiko/migrations}/0001-genesis.yaml` and `.../README.md`. Byte identity
confirmed independently by blob hash: `git rev-parse HEAD:migrations/0001-genesis.yaml` and
`git hash-object plugins/mochiko/migrations/0001-genesis.yaml` are both
`e00700344ab73b26d20f5cf0cf28995994a3d8f4`, and the README pair is `92e71f2b…3ffe`. `ls migrations`
returns `No such file or directory`.

**2. State hash — PASS.** `cargo run -q -p mochiko-cli -- migrate status --plugin-root plugins/mochiko`:
`state sha256:8b61de5a3b4cca8ed581df70f6ec036d8f05a8e67de06b66c6f1ae9b86c8bdd4 · 50 documents · 1016 rules`,
identical to the plan's pre-move line. Only the log line moved.

**3. Path references — PASS.** The grep over `src`, `tests`, `ci.yml` and `rust-cli.md` leaves no
repo-root path. Every survivor is licensed: `src/cli.rs:25` and `tests/cli.rs:178` are
`LOG_DIR_NAME`; `src/cli.rs:198`, `src/cli.rs:550`, `tests/cli.rs:695-855` are the CWD last-resort
and scratch roots; `src/replay.rs` uses `migrations` as a local binding; `tests/matrix_similar.rs:991`
is the synthetic `nested/deeper/migrations` fixture. A widened grep over `crates/**/*.toml` and
`crates/**/*.md` found nothing further.

**4. Legend block — PASS.** `diff` of plan §2 lines 78-84 against the rendered block from
`target/release/mochiko-cli rules brainstorm --section preamble --plugin-root plugins/mochiko`
is empty: verbatim, all seven lines. In `src/render.rs` the `LEGEND` push sits between
`- class: floor · {floors} rules` (the pins block's last line) and `body.push_str("\nsections\n")`,
outside the `is_command` guard. Delivered on the skill path too, checked by eye against
`rules review-feasibility --section preamble`. Both renders end
`mochiko-cli rules end · <name> · preamble · 0 rules`.

**5. The tightened assertion — PASS, it still proves the property.** The old needle `kind: fail`
now appears in the legend's `enforces:` prose, so it had stopped testing what it named. The new
needle `- kind: fail · ` is the pin's exact emitted prefix from
`body.push_str(&format!("- kind: fail · {fails} rules\n"))`. Probe: the command preamble contains
that needle once, the skill preamble zero times while containing the bare words once. A leaked pin
would therefore still trip the assertion. The shape is anchored from the other side by
`the_preamble_pins_match_the_corpus`, which asserts the positive `- kind: fail · 1 rules`, so a
reformat of the pin fails a test rather than silently voiding this one. Not merely stopping a
failure.

**6. No behavior change beyond the legend — PASS.** `git diff -- crates/mochiko-cli/src` is three
files: doc comments in `genesis.rs` and `lib.rs`, and in `render.rs` the `LEGEND` const plus one
`push_str`. Nothing else.

**7. Four layers — PASS.** `cargo test --all`: **302 passed, 0 failed**, over 14 binaries
(5 + 26 + 10 + 2 + 48 + 3 + 25 + 29 + 46 + 98 + 10, plus three empty targets).
`cargo fmt --all --check` clean. `cargo clippy --all-targets -- -D warnings` exits 0 with no
warning or error lines. `cargo audit` not run, as instructed.

**8. CI filters — PASS.** The only two hunks in `ci.yml` swap `"migrations/**"` for
`"plugins/mochiko/migrations/**"` at lines 11 and 20, in `push` and `pull_request`.

**9. Rules file — PASS.** `paths` drops the repo-root glob and keeps `plugins/mochiko/migrations/**`.
The prose now reads "carried in the plugin at `plugins/mochiko/migrations/` since wave 3", present
tense, with the "until then" clause gone. No other hunk.

**10. Report honesty — FAIL.** Verified true: the 302/0 tally and the two added tests (render.rs
goes 27 to 29 `#[test]`), `0 rejecting · 105 advisory`, the before/after hash pair, the rename
stat, "7 of 10" fidelity tests reading `log_dir()`, the preamble at 2,055 chars / 2,102 bytes, the
seven-block total of 10,700 bytes, and the −16.5 % delta against 12,819. Two rows are wrong:

- "the legend block itself | **599 bytes**" — 599 is the *character* count. `wc -c` on the same
  block gives 611 bytes, and the `LEGEND` const carries a leading newline, so removing it saves
  **612 bytes**.
- "the same seven blocks less the legend | **10,101 bytes**" — that is 10,700 minus the char
  figure. The byte figure is 10,700 − 612 = **10,088**.

The consequence is that the report's cross-check is weaker than the truth. It argues that 10,101
"cross-checks plan §0's pre-legend figure of 10,088, so §0's number reads as bytes" from a 13-byte
near-miss. With 612 the identity is exact, and the conclusion is confirmed rather than inferred:
§0's "10,088 chars" is bytes. For completeness the true char total of the seven blocks is 10,513,
and less the legend 9,913 — which is not 10,088, closing the question.

## Fix list

1. In `p1-store.md`, change the legend row to `611 bytes rendered / 612 bytes as the const (599
   chars)` and the following row to `10,088 bytes`.
2. Rewrite the cross-check sentence so the match with plan §0 is stated as exact, and add the
   9,913-char figure as the disconfirming half.

No other change. Re-audit is a read of those two rows.

## Advisory, for the lead

The legend changes the render output shape, which is the condition `rust-cli.md` attaches to a
`mochiko-cli-v*` tag: unchanged shape, or a coordinated `plugin.json` bump. Wave 3 already lands
0.104.0, so the clause is satisfied, but the coordination should be named in the wave's landing
rather than left implicit. P1 did not flag it; this is not a defect in P1's unit.

## Delta re-audit — 2026-09-04, item 10 only

**Verdict: PASS.** The corrected rows match my own measurements, and the sentence they support
now states the identity rather than approximating it. Items 1 through 9 were not re-graded; the
diff stat over `crates/`, `.github/workflows/ci.yml` and `.claude/rules/mochiko/rust-cli.md` is
byte-for-byte what I graded in the first pass, so the earlier evidence stands.

- **Legend, 612 bytes / 600 chars — correct.** The rendered block sliced between its `legend` and
  `sections` headers measures 611 bytes and 599 chars; the `LEGEND` const carries a leading
  newline that is delivered, giving 612 and 600. The report's stated method matches how I
  measured it.
- **Seven blocks less the legend, 10,088 bytes — correct.** 10,700 − 612 = 10,088, and removing
  the const is the only edit to the rendered output, so nothing else shifts.
- **The identity sentence — correct and now exact.** It lands on plan §0's 10,088 with no
  remainder, which settles that §0's "10,088 chars" was a byte count. The disconfirming half holds
  too, though the report does not cite it: the seven blocks total 10,513 chars, and less the
  legend's 600 that is 9,913, which is not §0's figure.
- **Provenance note — accurate.** 599 is exactly `wc -m` of the block without the leading newline,
  which is what my first pass found and named as the source of the 10,101 subtotal.

Unchanged and still verified: the preamble at 2,055 chars / 2,102 bytes, the 10,700-byte total,
the −16.5 % delta against 12,819, and the pins reading `kind: fail · 4 rules` and
`class: floor · 7 rules`.

The advisory from the first pass is untouched by this correction: the legend changes the render
output shape, so the coordination clause in `rust-cli.md` fires and should be named in the wave's
landing. Wave 3's 0.104.0 bump satisfies it.
