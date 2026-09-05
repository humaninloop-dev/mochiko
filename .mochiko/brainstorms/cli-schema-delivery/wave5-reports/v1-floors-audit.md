# V1 — audit of P1's `floors:` preamble line

**Verdict: PASS.** Seven checks, seven PASS, two minor non-blocking notes. Graded from the diff and
my own runs, never the report's say-so. Unit: `crates/mochiko-cli/src/render.rs` (+21/−4) and
`crates/mochiko-cli/tests/render.rs` (+289/−0). No other crate file moved; the plugin skills,
`evals/contract/run.py` and `expected-skills.json` appearing mid-audit are P2's and P3's.

**1. Placement and format — PASS.** `render.rs:172` pushes `"\nfloors: {index}\n"` after the
`- class: floor · N rules` pin and before `body.push_str(LEGEND)`; `LEGEND` opens `"\nlegend\n"`, so
the line stands as its own blank-line-separated block like every other. The live `implement` render
shows blank line, one unwrapped `floors: … · …` line, blank line, `legend`. The empty branch renders
the literal `floors: none`, fixture-proved in `a_primitive_with_no_floor_rule_indexes_none`.

**2. Render order and pin agreement by construction — PASS.** One collected iterator feeds both —
`schema.rules().filter(|rule| rule.is_floor()).map(|rule| rule.id.as_str()).collect()` — then
`floors.len()` prints the pin and `floors.join(" · ")` the index. `RuleSchema::rules()`
(`model.rs:486`) walks sections in declared order then blocks, and a command or skill carries no
blocks, so this is render order. Confirmed independently rather than from the goldens: I rendered
`implement`'s six sections in declared order with the release binary and pulled every
`[class: floor` id in print order. The concatenation — reserved 3, tools 1, ways-of-working 1,
boundaries 14, fail-conditions 15 — is byte-for-byte the preamble's line. `implement`: pin 34,
34 ids, first `impl.gate-design-checkpoint`, last `impl.fail.no-acceptance`. `review-brainstorm`:
pin 9, 9 ids. Both off `cargo build --release -p mochiko-cli`, `--plugin-root plugins/mochiko`.

**3. Whole-corpus invariant — PASS.** Scripted over all 36 shipped primitives, enumerated from the
migration log's own `import-document` names and each rendered by the release binary from
`plugins/mochiko/migrations`: **36 primitives, 0 mismatches** between pin and id count. Pins run 2
(`patterns-plan-minimalism`) to 34 (`implement`).

**4. Legend and end line untouched — PASS.** The `LEGEND` const at `render.rs:36` sits outside both
diff hunks. `tests/render.rs` is purely additive, so the `assert_eq!(LEGEND.len(), 845, …)` pin at
line 626 is unedited and the two `out.contains(LEGEND)` goldens still bind the const to the render.
The end line renders `mochiko-cli rules end · implement · preamble · 0 rules`, and
`the_floor_index_sits_between_the_pins_and_the_legend` asserts it.

**5. Tests, tally, four layers — PASS.** Seven tests added, all seven exercising the line. Red-first
proved, not accepted: I copied the crate and plugin corpus to a scratch tree, restored `render.rs`
from `HEAD` there, kept the new tests, and ran the suite — exactly the seven fail, 30 pass, and each
failure is `the preamble carries no floors line` or `the floor index is present`, the missing line
and no fixture defect. The `none` fixture imports `review-floorless` with the six canonical review
sections (independence · scope · inputs · verdict · output · reserved) and no floor rule; its
inherited `review-common.shared-verdict` carries no class, so the empty set is genuine. The command
fixture's three floors sit in three non-adjacent sections, which is what makes the order assertion
discriminating. P1's tally reproduces: `tests/render.rs` **37 passed**, crate **338** across eleven
binaries (5+35+12+2+48+3+33+37+55+98+10), `cargo test --all` green. `cargo fmt --all --check`,
`cargo clippy --all-targets -- -D warnings` and `cargo audit --deny warnings` each exit 0.

**6. No other behavior change — PASS.** The pin's format string moved from `{floors}` (a `count()`)
to `{}` fed by `floors.len()` — identical output. The rest is the new block plus a three-line
doc-comment rewording. The only callers of `render::preamble` / `render::section` are
`crates/mochiko-cli/src/cli.rs:321` and `:323`. The contract suite's `parse_preamble`
(`evals/contract/run.py:1168`) only enters on a bare `sections` line and only reads `- ` rows, so
the new line cannot reach it. `no_shipped_section_renders_past_the_inline_ceiling` covers preambles
and still passes unedited; the largest preamble is 4,465 bytes against a 30,000 ceiling.

**7. Report honesty — PASS, two notes.** Byte deltas recomputed from my own 36 renders. Summing each
line's bytes gives 13,809, and the block adds two newlines per primitive, so **13,881 total** — the
report's figure exactly. Mean 13,881/36 = 385.6, reported as +386. `implement` 3,485 → 4,465 (+980,
978-byte line + 2) and `review-brainstorm` 1,972 → 2,366 (+394) both reproduce; smallest is
`patterns-plan-minimalism` at +99. The widest line is `implement` at 945 chars / 978 bytes, the
33-byte gap being the 33 multi-byte separators, and it is widest by chars as well as bytes. The
plan's ambiguity between §1 "the size pin updated" and §2 "the legend size pin unchanged" is
disclosed in P1's plan and resolved the conservative way: legend pin untouched, new pin added.

- **Minor (accuracy).** The report calls the source diff "(+25/−4)"; `git diff --numstat` gives
  **+21/−4**. 25 is the `--stat` total-changed count, not insertions. Cosmetic, nothing hidden.
- **Observation (not P1's to fix).** The two shipped-corpus tests use `shipped_state()`, which reads
  `plugins/mochiko/schemas/` and `skills/*/schema.yaml` — the derived views, not the migration log
  the binary serves from. A golden could pass while delivery differed. It does not today: my corpus
  run went through the log and agreed everywhere. Pre-existing harness shape, pre-wave-6.
