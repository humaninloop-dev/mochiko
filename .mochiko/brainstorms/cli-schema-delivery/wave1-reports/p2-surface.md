# Wave 1 — seat P2 (surface) cycle report

**Seat:** P2 — the `clap` command surface, the per-section rules render, the `template`/`--check`
re-base onto the replayed state, `migrate validate|status`, `--version`, `--plugin-root`, and the
exit-code contract.
**Plan:** returned 2026-09-03, lead-approved with rulings on all eight open questions (Q1–Q4, Q7,
Q8 all as recommended) and three of five deltas granted to this seat (D-3 lib.rs pen widening,
D-5 fixtures; D-1 and D-2 were landed by P1 in its fix round; D-4 goes to P3).
**Base:** P1 frozen at `cd5a333`.
**Gates at close:** `cargo test --all` 161 passed / 0 failed · `cargo fmt --all --check` clean ·
`cargo clippy --all-targets -- -D warnings` clean · `cargo audit --deny warnings` clean over 31
crate dependencies.
**Shipped files:** no file under `plugins/` changed byte-wise. `git status plugins/` is empty.

---

## 1. What was built

| file | lines | what it carries |
|---|---|---|
| `crates/mochiko-cli/src/cli.rs` | 389 | the `clap` command tree, the resolution order, the exit-code contract |
| `crates/mochiko-cli/src/render.rs` | 362 | the preamble and section renders, the head/tail lines, the re-based template views |
| `crates/mochiko-cli/src/schema.rs` | 113 (was 239) | the template model and its two views — resolution and the embedded copies removed |
| `crates/mochiko-cli/src/lib.rs` | 23 | module declarations only |
| `crates/mochiko-cli/src/main.rs` | 8 | the process entry: parse argv, dispatch, exit |
| `crates/mochiko-cli/tests/cli.rs` | 732 | 20 tests — the command tree, every exit code, both resolution orders |
| `crates/mochiko-cli/tests/render.rs` | 897 | 26 tests — the render contract, the ceiling measurement, template byte equality |
| `crates/mochiko-cli/tests/fixtures/template/` | 16 files, 108 KB | today's producer and check views, captured before `schema.rs` was touched |

`Cargo.toml` gained `clap` and nothing else. P1's `model.rs`, `migration.rs`, `replay.rs` and
`validate.rs` were not touched.

### The command tree

```
mochiko-cli [--plugin-root <path>] [--log-dir <path>] [-V|--version] <command>
  rules <primitive> --section <id>
  template <name> [--check]
  migrate validate [--report]
  migrate status
```

Both path flags are `global`, so §4's trailing form (`rules demo --section roles --plugin-root X`)
and the leading form both parse. `--section` is required: D3 as amended rules out a
whole-primitive render, and the preamble carries the section list that replaces it.

**Exit codes.** `0` ok · `1` the log is absent, empty or unsound · `2` a usage error or a name the
log does not carry · `3` the version contract. Three beats one, deliberately: a log the binary
cannot read raises findings that are artefacts of the misreading, so the D5 halt prints alone.
That message is `ParseError::GrammarVersion`'s own text, taken off the finding rather than
re-authored, per P1's handoff — the CLI carries no second copy of it.

**An empty log is a delivery failure, not an unknown name.** A `migrations/` directory holding no
migration file replays cleanly to an empty state, which would otherwise make every primitive an
unknown name (exit 2) and hide the real problem. `load_for_delivery` checks for it and exits 1
naming the directory. This is an addition to the plan, argued in §4 below.

### The render contract

Head line and tail line exactly as wave-plan §4 and D3-as-amended specify, one blank line between
head, body and tail. The preamble carries the identity line, the resolved `vars`, the
`conditions` block, `moments` (commands only), the count pins, and the section list with per-section
rule counts; its own count is always zero. A section carries `## title`, the intent, and one block
per live rule: `### id`, a bracket line (`class`, then `kind` only when the effective kind is not
`constraint`, then `when`, `labels`, `pointer`), the resolved text, and `enforces:` for a fail
node. An empty section renders its `note:` in place of the rules.

Three properties are worth naming because they were choices, not defaults:

- **`extends:` and `${var}` resolve through P1's implementations, never a second copy.**
  `validate::resolve_extends` is called with a discarded findings sink — a state that reached the
  delivery path already passed the hard set, so there is nothing left to report — and
  `validate::placeholders` supplies the placeholder grammar the substitution walks. A renderer
  that resolved either differently from the validator would show guidance the hard set never
  graded.
- **An unbound placeholder is left standing rather than blanked.** The hard set rejects one, so
  its appearance in a render is evidence of a defect. Blanking it would hide that.
- **Maintainer metadata never reaches a render.** A rule's ruling `anchor:` and its authoring
  `note:` are excluded (Q4, D2 and the D16 posture); the section-level `note:` on a deliberately
  empty section does render, because it is the section's runtime content.

### The template re-base

`TEMPLATE_NAMES`, the eight `include_str!` embedded copies, the three-step `resolve`, and
`schema::parse` are gone. A template is now an opaque document in the replayed state, decoded into
the typed model at the point of use. `Template`, `Section`, `producer_view` and `check_view` are
byte-for-byte the code they were — the interface the views produce was preserved, only its data
source moved.

The provenance footer is the one line that could not survive the move honestly: `schemas: embedded`
would now be false. It reads `schemas: replayed from <log-dir>`. The 16 fixtures were captured from
the pre-re-base binary with that line stripped, and the equality assertion is
`render == fixture + the new source line` — so every character of guidance is proven unchanged and
the single changed line is named rather than absorbed.

## 2. The red/green/refactor trail

Both test files were written first and failed to compile (`unresolved import mochiko_cli::render`,
`no cli in the root`), then the surface was built against them. Five failures during the green
phase were worth more than the tests that passed; all five were defects in my own tests or
fixtures, not in the code under test, which is what a fixture-heavy suite should expect.

| # | failure | what it actually was |
|---|---|---|
| 1 | `not readable as YAML ... line 119` | the fixture wrote `intent: The kind: fail set.` unquoted. A bare `: ` inside a YAML scalar. The parser was right. |
| 2 | `extends: common.shared-block names no block` ×2 | the fixture put a common library's blocks under `blocks:`; the shipped corpus and the decoder both use `rules:`. Corrected against `plugins/mochiko/schemas/common.yaml` rather than against my memory of it. |
| 3 | skill floor pin `3` vs rendered `2` | I miscounted the fixture's own floors when writing the assertion. The render was right; the test was fixed, not the code. |
| 4 | `0 advisory` unreachable | see below — the one that changed a design assumption. |
| 5 | `· 1 advisory` vs `3` | same cause. |

**Failures 4 and 5 — the advisory tally is not a problem count.** I had assumed a clean log
produces no advisory findings. It cannot: `validate` emits a `budget` report per document
unconditionally, and the fixture also raised `enforces-coverage` because its only fail node is
itself a floor no other fail node enforces. Both are correct behaviour. The assertions were
re-keyed: the sound-log test now asserts `0 rejecting` and that nothing but the tally prints, and
the `--report` test asserts the **delta** the deictic rule makes (`baseline + 1`) against the same
corpus without it, rather than an absolute figure the budget report would keep moving. The delta
form is the stronger assertion — it survives any future advisory the validator gains.

## 3. Deviations from the approved plan, each named

1. **`render::largest_shipped_section` was not built.** The plan listed it as a public render
   function for the ceiling test. It is test-only scaffolding, so it moved into `tests/render.rs`,
   which builds the shipped corpus as state through the public `Document::from_value` and
   `State::docs` and measures every render itself. No test-only surface in the library.
2. **`schema::parse` was removed** along with the resolution machinery. Nothing calls it after the
   re-base; the two remaining callers use `serde_norway::from_value` and `from_str` directly.
   Removing it was not in the plan, which named only `TEMPLATE_NAMES`, the embedded copies and
   `resolve`.
3. **An empty log directory exits 1 with a named directory.** Not in the plan. Without it an empty
   log makes every primitive an unknown name, which reports the wrong failure.
4. **`--schemas-dir` was removed.** Planned and approved; recorded here because it is a
   user-facing flag deletion. Verified beforehand that nothing outside the crate references it —
   the only two hits in the repository are prose in this session's own record.
5. **The wave plan's §4 log-directory order was corrected**, per the lead's Q1 ruling: `--log-dir`
   → `<plugin-root>/migrations` → `$MOCHIKO_MIGRATIONS` → `./migrations`. §4 put the environment
   variable last, where `./migrations` shadows it in any directory holding one — the repository
   root included. The correction is stated in `resolve_log_dir`'s own doc comment so the next
   reader does not "fix" it back.
6. **`views emit` is absent**, per D-4: it belongs to P3's `views.rs`, and a subcommand arm calling
   a function that does not exist cannot compile.

## 4. Pre-code ladder disclosures

Run per `mochiko:patterns-code-minimalism`; what was **not** built, and why.

| rung | not built / chosen | why |
|---|---|---|
| exist at all | a whole-primitive `rules <primitive>` render | D3 as amended forbids it; the preamble carries the section list instead |
| exist at all | a persistent render cache | D1 defers it to a measured need. The largest render is 15,450 chars and replays in milliseconds |
| exist at all | `views emit` | P3's seat (D-4) |
| exist at all | `--schemas-dir`, the 8 embedded copies, `resolve`, `schema::parse` | the log is the only source under D1; the embedded copies were D8's raw-Read fallback, superseded by D10.5. Keeping them would ship a fallback the record rules out |
| exist at all | a `--kind` disambiguator for command-vs-skill names | the two name sets are disjoint; lookup order plus an ambiguity arm is enough, and an overlap reports rather than guesses |
| exist at all | `render::largest_shipped_section` | test scaffolding; it lives in the test (deviation 1) |
| in codebase | `extends:` resolution, the `${var}` placeholder grammar | P1's `resolve_extends` and `placeholders`, made public for exactly this (D-2) |
| in codebase | the D5 halt message | `ParseError::GrammarVersion`'s Display, read off the finding |
| in codebase | the finding line format, the rule/floor census | `Finding`'s Display, `validate::census`, `Rule::is_fail`/`is_floor` |
| in codebase | the two template views | `producer_view`/`check_view` unchanged; only their data source moved |
| stdlib | `tempfile` | `CARGO_TARGET_TMPDIR`, P1's pattern |
| installed dep | a JSON parser for `plugin.json` | JSON is a subset of YAML 1.2, so `serde_norway` reads the manifest. No `serde_json`, and no hand-rolled scan |
| installed dep | `clap` with `default-features = false` and `std, derive, help, usage, error-context` | drops the colour and suggestion chain (`anstream`, `anstyle-parse`, `anstyle-query`, `colorchoice`, `strsim`, `is_terminal_polyfill`) from the audit surface. Coloured help and "did you mean" are worth nothing to a CLI read by a model |
| one line | a re-export of `INSTALL_COMMAND` from `cli.rs` | written, then deleted: the tests name `migration::INSTALL_COMMAND` directly, so it pointed at nothing |

**Dependency total: 25 → 31.** The six are `clap`, `clap_builder`, `clap_derive`, `clap_lex`,
`anstyle`, `heck`. The trimmed feature set is what kept it to six; the default set brings eleven.
`cargo audit` clean.

## 5. Test tally

| suite | tests | this seat's delta |
|---|---|---|
| `tests/cli.rs` | 20 | +20, new |
| `tests/render.rs` | 26 | +14 (12 pre-existing, all re-based or moved) |
| `tests/migration.rs` | 25 | — (P1) |
| `tests/replay.rs` | 46 | — (P1) |
| `tests/validate.rs` | 44 | — (P1) |
| **total** | **161** | **+34** |

Of the 12 tests `tests/render.rs` carried before, the 6 dispatch and exit-code tests moved to
`tests/cli.rs` in re-based form, the 4 view tests were re-based onto the log, and the gate-5
shipped-data test was kept with its "every known name has a file" limb re-keyed off the fixture's
template manifest rather than the deleted `TEMPLATE_NAMES` constant.

### The ceiling measurement (test 29)

Every section of every shipped primitive, rendered and measured against the ≈30,000-character
Bash inline ceiling (F12e):

| figure | value |
|---|---|
| renders measured | 252 (36 primitives × preamble + 6 sections) |
| largest render | `implement · impl.sec.tools` |
| its size | **15,450 characters** |
| ceiling | 30,000 |
| headroom | 1.94× |

The corpus estimate in my plan was 15,115 for the same section; the render's own figure is 335
characters higher, which is the bracket lines and headings the estimate approximated. D3's
per-section chunking has close to double the headroom it needs at today's corpus size.

### The shipped-log test (test 15)

`the_shipped_log_renders_every_section_of_every_primitive` runs against the repository's own
`migrations/` directory. P3 generates `0001-genesis.yaml`; until it lands the test prints
`SKIPPED: ... this test is dark until then` and returns. It is dark today and will light up on
P3's close without any edit from this seat.

## 6. Handoffs

**To P3 — the `views emit` arm.** `src/cli.rs` has no `views` subcommand. Adding one is three
lines in the `Command` enum plus a match arm calling into `views.rs`; the lead granted P3 the
scoped `cli.rs` delta (D-4).

**To P3 — the shipped-log test lights up on genesis.** Test 15 above renders every section of
every primitive in `migrations/` and asserts the head and tail lines. If genesis produces a state
the render cannot walk, that test fails on P3's watch rather than silently passing.

**To P3 — the fixture pattern.** `tests/render.rs::template_log` builds an `import-document` op
per shipped file by parsing the YAML into a `Value` and stamping the migration through
`migration::with_hash`. It is the same shape genesis needs, at fixture scale.

**To the validator — the one place byte-equality is qualified.** The template views are asserted
byte-identical against captured fixtures *except* the trailing provenance line, which was ruled at
Q2. The fixtures hold everything up to and including the `---` separator; the assertion appends
`schemas: replayed from <log-dir>`. Nothing else in either view is exempt.

## 7. Open items

1. **`template <name>` cannot serve anything until genesis lands.** Between this seat's close and
   P3's, the eight templates have no source: the embedded copies are gone and the log is empty. The
   command exits 1 naming the empty log. This is wave-1-internal — no `.md` points at the CLI and
   nothing ships — but it is a real dark window and it is the reason test 15 is written to skip
   rather than to fail.
2. **The advisory tally counts reports, not problems.** `migrate validate` on a perfectly sound log
   reports a non-zero advisory figure because the budget report is per-document and unconditional.
   Worth a decision at some point about whether `budget` belongs in the same counter as
   `enforces-coverage` and the deixis lint; it is not a wave-1 question.
3. **`--plugin-root` is not read from the environment.** `${CLAUDE_PLUGIN_ROOT}` substitutes in
   `.md` bodies, so wave 3's `!` lines can pass it as a flag. If a case appears for reading it
   directly, it is a one-line addition to `resolve_plugin_version` and takes its own ruling.
4. **`cargo audit` fetches the advisory database over the network** — P1's item 4, unchanged.

## 8. Suggested commit

Nothing was committed. Suggested message:

```
Add the clap CLI surface, the per-section render, and the template re-base

Wave 1 seat P2 of the CLI schema-delivery build. The crate gains a clap
command tree (rules, template, migrate validate|status, --version), the
per-section rules render with its head and tail confirmation lines, and
the exit-code contract: 0 ok, 1 unsound log, 2 unknown name, 3 version
contract.

The eight artifact templates are re-based onto the replayed state. The
closed TEMPLATE_NAMES set, the compile-time embedded copies and the
three-step file resolution are removed; the two views render byte-for-byte
what they rendered before, asserted against fixtures captured from the
previous binary, with the provenance footer as the single changed line.

The largest section render measures 15,450 characters against the 30,000
character inline ceiling. No shipped file changed.

clap is the only new dependency.
```
