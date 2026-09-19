# P1 — the `floors:` preamble line (plan)

**Scope:** `crates/mochiko-cli/src/render.rs`, `crates/mochiko-cli/tests/render.rs`. Nothing else.

**Where it is built.** In `render::preamble`, immediately after the pins block and before
`body.push_str(LEGEND)`. The ids come from the same iterator the pin already counts —
`schema.rules().filter(|r| r.is_floor())` — so the pin's number and the line's id count cannot
disagree; `RuleSchema::rules()` walks sections in declared order and rules in section order, which
is the render order the wave plan names. Rendered for commands and skills alike, exactly as the
floor pin already is. Tombstoned rules leave the state's sections before the render sees them, so a
retired floor id can never appear.

**Exact format.** Its own blank-line-separated block, matching every other preamble block:
`\nfloors: <id> · <id> · …\n`, one line however long, no wrapping. `floors: none` when the primitive
carries no floor rule. The `legend` const, its byte pin, and the `preamble · 0 rules` end line are
all untouched.

## Tests added to `tests/render.rs` — written before the render change, red first

1. Fixture command `demo`: the exact line `floors: demo.stub · demo.boundary · demo.fail.unaccepted`
   — three ids drawn from three different sections, which is what proves the ordering.
2. Fixture skill `review-demo`: `floors: review-demo.not-the-author · review-demo.user-rules`.
3. Placement and neutrality: `pins` before `floors:` before `legend`, and the end line still reads
   `mochiko-cli rules end · demo · preamble · 0 rules`.
4. The empty branch: a second migration importing a floorless skill, asserting `floors: none` beside
   a `- class: floor · 0 rules` pin. No shipped primitive has zero floors — all 36 carry 2 to 34 —
   so this branch can only be fixture-tested. The extra migration follows the precedent of
   `no_render_carries_an_anchor_or_a_rule_note` rather than perturbing the shared `RULES_LOG`.
5. Shipped goldens against `shipped_state()`: `implement`'s 34 ids and `review-brainstorm`'s 9, each
   as an explicit ordered array. A floor set that changes shape breaks a test.
6. Corpus invariant: for every shipped command and skill, the id count on the `floors:` line equals
   the `class: floor` pin. This is the property P2's per-family check cites.
7. Size pin: `implement`'s line, the corpus worst case, at 945 chars / 978 bytes — the legend pin's
   analog, since this is a render-shape change the 0.106.0 bump names.

**Assumption to correct if wrong.** The wave plan's §1 says "the size pin updated" while §2 says
"the legend size pin unchanged". I read the specific clause as binding: the legend const stays at
845 bytes, and item 7 is a new pin on the new line rather than an edit to the legend's.

## Verification

- `cargo test --all`, `cargo fmt --all --check`, `cargo clippy --all-targets -- -D warnings`,
  `cargo audit --deny warnings`.
- Hand render: `mochiko-cli rules implement --section preamble --log-dir plugins/mochiko/migrations`
  and the same for `review-brainstorm`, cited in the report with the line and the pin agreeing.

**No ripple found.** The contract suite's `parse_preamble` reads only the `sections` block;
`tests/cli.rs` asserts head and tail lines only; `mochiko-cli views` emits state YAML, not renders.
The largest preamble grows from 3,485 to about 4,463 bytes, far under the 30,000 ceiling.
