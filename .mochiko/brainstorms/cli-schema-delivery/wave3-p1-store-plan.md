# P1 — store move + crate: plan

**Pre-move state** (`cargo run -q -p mochiko-cli -- migrate status --plugin-root plugins/mochiko --log-dir migrations`):
`log migrations · grammar 1 · sequences 1..1 (1 migration)` / `state sha256:8b61de5a3b4cca8ed581df70f6ec036d8f05a8e67de06b66c6f1ae9b86c8bdd4 · 50 documents · 1016 rules`

## Files touched (one move plus eight edits)

`migrations/` (moved) · `crates/mochiko-cli/src/render.rs` · `crates/mochiko-cli/src/lib.rs` · `crates/mochiko-cli/src/genesis.rs` · `crates/mochiko-cli/tests/render.rs` · `crates/mochiko-cli/tests/fidelity.rs` · `crates/mochiko-cli/tests/matrix_similar.rs` · `.github/workflows/ci.yml` · `.claude/rules/mochiko/rust-cli.md`

## 1. Move (the only git mutation)

```
git mv migrations plugins/mochiko/migrations
```

Both files move (`0001-genesis.yaml`, `README.md`). Verify with `git diff --stat -M --cached` showing pure renames, `0` insertions/deletions on the genesis file.

`migrations/README.md` carries **no** self-referencing repo path (checked: it says "This directory"; its only commands are `mochiko-cli migrate validate|status` with no path argument). It moves unedited.

## 2. Test and source edits (test-first: edit the assertion, watch it fail, then move)

- `tests/fidelity.rs:29` — `log_dir()` becomes `repo_root().join("plugins/mochiko/migrations")`; the module doc (line 5), the `expect` string (line 88) and the regenerate hint (line 99) become `plugins/mochiko/migrations/0001-genesis.yaml` / `--out plugins/mochiko/migrations/0001-genesis.yaml`.
- `tests/matrix_similar.rs:1051` — `find_allowlist(&repo_root().join("plugins/mochiko/migrations"))`, same assertion (`repo_root().join(similar::ALLOWLIST)`). This is the "assert it" of plan §2: `find_allowlist` walks ancestors, so the deeper start still reaches the repo-root `scripts/similar-rules-allowlist.yaml`. Its scratch fixture (line 991, `nested/deeper/migrations`) is a synthetic tree and stays.
- `tests/render.rs:6` — module doc reference to `migrations/` re-pathed.
- `src/lib.rs:16`, `src/genesis.rs:3` — doc comments naming `migrations/0001-genesis.yaml` re-pathed.
- **No change** to `tests/cli.rs` (scratch roots build their own `migrations/` subdirectory; `LOG_DIR_NAME` is the resolution constant, not a repo path), `tests/views.rs` or `render.rs`'s `shipped_state` (both read `plugins/mochiko/schemas`), or `src/cli.rs:25` / `src/similar.rs`.

## 3. Legend block

In `render::preamble` (`src/render.rs`), inserted **between the `pins` block and the `sections` block** (plan §2: "after `pins`"), unconditional for commands and skills, the six lines verbatim from plan §2 under a `legend` header. It is fixed text, not derived from state.

Golden test in `tests/render.rs`, beside the preamble tests (after `the_preamble_pins_match_the_corpus`): `the_preamble_carries_the_fixed_legend_block` asserts the whole seven-line block as one `contains` against a `const` holding the verbatim text, asserts it sits after `\npins\n` and before `\nsections\n`, and re-asserts the end line is `... · preamble · 0 rules`.

**Flag for the lead/P3:** the legend adds ≈ 600 bytes to the preamble, so the §0 rendered-`brainstorm` figure moves 10,088 → ≈ 10,690 chars and the pre-pilot read-cost delta −21 % → ≈ −17 % against 12,819. Well inside abort criterion (2); P3's measurement should baseline against the post-legend render, not §0.

## 4. CI and rules file

- `.github/workflows/ci.yml` lines 11 and 20 — `"migrations/**"` becomes `"plugins/mochiko/migrations/**"` in both the `push` and `pull_request` path filters. No other workflow edit.
- `.claude/rules/mochiko/rust-cli.md` — drop the `"migrations/**"` glob from `paths` (line 4; `plugins/mochiko/migrations/**` is already there, line 5); in the opening paragraph strike ", at the repo root `migrations/` until then" and reduce "carried in the plugin at `plugins/mochiko/migrations/` from wave 3" to the present tense.

## 5. Verification

1. `cargo test --all`
2. `cargo fmt --all --check`
3. `cargo clippy --all-targets -- -D warnings`
4. `cargo run -q -p mochiko-cli -- migrate validate --plugin-root plugins/mochiko` — 0 rejecting
5. `cargo run -q -p mochiko-cli -- migrate status --plugin-root plugins/mochiko` — must print `state sha256:8b61de5a...bdd4 · 50 documents · 1016 rules` (the log line's directory changes; the hash must not)
6. `git diff --stat -M` — the genesis file shows as a pure rename

## 6. Hand-offs (outside P1's scope, noted)

- `evals/contract/run.py:460` `write_skew_log` builds `<case root>/migrations` and passes it as `--log-dir`. After the move the staged plugin carries its own `migrations/`; `--log-dir` still wins by resolution order, but P3 should confirm the skew case halts on the staged log it means to test.
- `CLAUDE.md`, `DECISIONS.md`, `BACKLOG.md`, `.mochiko/memory/governance-ledger.md` and `governance-intent.md` all carry "today the log lives at the repo root `migrations/`" clauses. Landing-ritual work (plan §6), not P1's.
