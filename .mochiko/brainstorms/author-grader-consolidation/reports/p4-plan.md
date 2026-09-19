---
report: disclosure
seat: P4 crate producer
wave: 1
status: plan — awaiting lead approval, no code written
---

# P4 — widen `mint-rule` to mint a common-library block

## The gap and the ruling

The migration log's grammar has no operation that adds a block to an existing common library
(`command-common/common`, `skill-common/skill-review-common`, `skill-common/skill-authoring-common`).
`mint-rule` requires a live `section:`, and a common library carries blocks at the document's top
level and no sections at all; `mint-section` on a common document rejects with `section-set`;
`replace-document` rejects with `op-inapplicable`, being reserved for templates, shelf data and
homes. Genesis is the only migration that has ever written a common library.

Ruled change: `mint-rule` with `section:` **absent**, applied to a `command-common` or
`skill-common` document, appends the rule as a block at the document's top level. Everything else
stands as today.

## No grammar bump

`git tag -l 'mochiko-cli-v*'` returns nothing — no `mochiko-cli` release exists, so no deployed
reader can meet a file it cannot understand. The log README's rule (§ Change ops, "Adding an op
does not bump the grammar, while no binary is published") covers a widened op a fortiori: the
`1..1` range freezes at the first publish with whatever the grammar carries by then, and the first
published binary reads the optional `section:`. The log stays at `grammar: 1`.

## Source changes

### `crates/mochiko-cli/src/migration.rs`

1. `Change::MintRule` (≈ line 304): `section: String` becomes `section: Option<String>`.
2. `parse_change`, the `ChangeOp::MintRule` arm (≈ line 691). Today it is a three-field struct
   literal; it becomes a block that reads the document first, so the kind is in hand:

   - `let doc = doc_ref(map, "schema", file, index)?;`
   - `section` is `Some(field_str(map, "section", …)?)` when the key is present, `None` when absent.
   - Key present but not a scalar keeps today's rejection verbatim: `op-malformed`,
     "`section:` missing or not text".
   - Key absent **and** `doc.kind` is not `CommandCommon`/`SkillCommon`: `op-malformed`, with the
     new message naming the one legal shape — only a common library takes a section-less mint, and
     the rule is appended there as a top-level block.
   - Key absent and the kind is a common library: `None`, and the op parses.

3. Untouched: `ChangeOp::ALL`, `ChangeOp::as_str`, `Change::op()`, `Change::doc()`, `RuleField`.
4. Hash: `Migration::body_hash` canonicalises the raw `hashed_body` YAML, not a re-encoded `Change`,
   so an omitted `section:` changes the hash by construction. No hashing code is touched, and the
   op stays inside the hashed body exactly as every other op is.

### `crates/mochiko-cli/src/replay.rs`

5. `apply`, the `Change::MintRule` arm (≈ line 471). The decode-then-mint-once order is preserved
   exactly, because it is what makes a rule that does not decode fail before it can consume an id:

   - `decode_rule` → `was_minted` → `mint_once` → `schema_of`, all unchanged.
   - `Some(section)`: today's path, `schema.sections.iter_mut().find(|s| s.id == *section)`, with
     `op-inapplicable` "no such live section" on a miss.
   - `Some(section)` where `doc.kind` is a common library: still `op-inapplicable` — the code does
     not change — but the message is sharpened to say a common library carries blocks at the top
     level and never sections, and that omitting `section:` is the way to append one. This is the
     "`section:` present on a common document stays a rejection" limb.
   - `None`: `schema.blocks.push(decoded)`, then `state.mint(&doc, &id)` as before.
   - `None` on a non-common document is unreachable (parse rejects it), but the arm is written
     total: guard on the kind and return `op-inapplicable` rather than pushing a block onto a
     command schema. A crate caller constructing a `Change` directly must not be able to write the
     state that `flat-rules` exists to reject.

### `crates/mochiko-cli/src/validate.rs` — no change

Validation runs over the finished state, so an appended block takes exactly the checks a
genesis-imported block takes, and nothing new is needed:

- `check_sections` (≈ 1545) returns `common_prefix_of(doc)` for a common library, and `check_ids`
  (≈ 1724) then requires `common.` for the command library and `<family>-common.` for a skill
  family. A wrong prefix is `id-prefix`; a non-slug tail is `id-format`.
- `check_class_and_kind` (≈ 1768) `is_block` branch emits `extends-class-local` for `class:`, and
  again for each of `kind:` / `when:` / `enforces:`.
- `find_block` (≈ 2185) and `resolve_extends` (≈ 2204) both locate the block off
  `RuleSchema.blocks`, so a dependent stub's `extends:` resolves against a minted block.
- `document_empty` (≈ 1534) and `flat-rules` (≈ 1512) are unaffected.

### `crates/mochiko-cli/src/model.rs` — no change

`RuleSchema.blocks` already exists; `rules()`, `rules_with_section()`, `find_rule_mut()` and
`live_ids()` already chain it, and `Document::to_value` (≈ 1021) already emits blocks as the
top-level `rules:` sequence the views round-trip through. `State::seed_minted` seeds through
`live_ids()`, so mint-once already covers a genesis-imported block.

## Tests

New fixture in `crates/mochiko-cli/tests/replay.rs`: a second genesis const importing a
`command-common/common` library with one block, a `skill-common/skill-review-common` library with
one block, and a `command/demo` stub that binds a block by `extends:` with a local `class:`. The
shared `GENESIS` const is **not** extended — every existing followup test replays it, and adding a
document there would widen the surface of assertions that are about something else.

| # | test | asserts |
|---|---|---|
| 1 | `mint_rule_without_a_section_appends_a_block_to_a_common_library` | clean replay; the rule lands in `schema.blocks`, after the genesis blocks, and in no section |
| 2 | `a_minted_common_block_resolves_a_dependent_stub_s_extends` | `replay::load` returns `Ok` for a log that mints `common.<new>` and sets a `command/demo` rule's `extends: common.<new>` with a local `class:` — no `extends-unresolved`, no `extends-class-local` |
| 3 | `mint_rule_without_a_section_appends_a_block_to_a_skill_common_library` | same on `skill-common/skill-review-common`, id prefix `review-common.` |
| 4 | `mint_rule_naming_a_section_on_a_common_library_is_rejected` | `op-inapplicable`; the library's blocks are untouched |
| 5 | `mint_rule_with_no_section_on_a_command_schema_is_malformed` | `op-malformed` |
| 6 | `a_minted_common_block_carrying_a_class_is_rejected` | replay applies, `validate` emits `extends-class-local`, `load` is `Err` |
| 7 | `minting_a_block_id_that_is_already_live_is_rejected` | `mint-once`, in both directions: against a genesis-seeded block and against one minted by an earlier migration in the same log |

New in `crates/mochiko-cli/tests/migration.rs`, at the parse layer:

| # | test | asserts |
|---|---|---|
| 8 | `mint_rule_on_a_common_library_parses_with_no_section` | the parsed change is `Change::MintRule { section: None, .. }` |
| 9 | `mint_rule_on_a_command_schema_needs_a_section` | `err.code() == "op-malformed"` |

Test 1 (and 3) mint a block that no stub binds, which raises the advisory `orphan-block` from the
library check (≈ 874). That is advisory, not rejecting, so the replay is still clean and `load`
still succeeds; test 1 asserts the finding is advisory rather than pretending it is absent, and
test 2 is the one that binds its block.

## Fixtures and census that will not move

This unit adds no migration to the shipped log, so the replayed state and every derived view are
byte-identical:

- `the_shipped_corpus_matches_its_recorded_census` (tests/validate.rs), `the_corpus_census_holds_through_the_log` (tests/fidelity.rs)
- `every_emitted_view_matches_the_committed_one`, `a_view_re_decodes_into_the_document_it_came_from`, `the_committed_views_tree_holds_no_file_the_emitter_does_not_write` (tests/views.rs)
- the two template fixture tests in tests/render.rs, and the two shared-fixture-log tests in tests/home.rs
- `crates/mochiko-cli/src/conform.rs` and `src/hook.rs` are untouched — `check` reads homes and
  templates, never rule blocks
- `src/genesis.rs` emits `import-document`, never `mint-rule`

## README change — exact old → new

`plugins/mochiko/migrations/README.md`, § "Change ops" table.

Old:

```
| `mint-rule` | `schema`, `section`, `rule` | |
```

New:

```
| `mint-rule` | `schema`, `section?`, `rule` | `section:` is required on a command or skill schema and rejected on a common library, which carries its blocks at the document's top level: there, omit it and the rule is appended as a block. |
```

And one sentence appended to the paragraph beginning "**Adding an op does not bump the grammar,
while no binary is published.**": widening an existing op is the same case — `mint-rule` gained an
optional `section:` at the author-grader-consolidation wave and the log stayed at grammar 1.

## Command sequence

```
cargo fmt --all
cargo test -p mochiko-cli
cargo fmt --all --check
cargo clippy --all-targets -- -D warnings
cargo audit --deny warnings
```

Ceremony per `.claude/rules/mochiko/rust-cli.md`: this plan is lead-approved before any code is
written, an independent non-author seat reviews the diff, and the four crate layers must be green.
The render output shape does not change, so no `plugin.json` bump is coupled to this unit.

## Risks

1. **A second block-resolution path.** There are two block lookups, `find_block` (validate.rs 2185)
   and the inline one inside `resolve_extends` (validate.rs 2279), plus `render.rs:300` calling
   `resolve_extends`. All three read `RuleSchema.blocks` off the state, so an appended block is
   visible to each. Mitigation: `rg '\.blocks'` across `src/` before the diff is final (today it
   returns 14 sites in five files, all state readers), and the reviewer re-runs it.
2. **Mint-once depends on `live_ids()` chaining blocks.** A genesis-imported block is seeded as
   minted only because `live_ids()` chains `rules()`, which chains `blocks`. If that chain ever
   narrowed, a minted block could silently collide with a genesis one. Test 7 pins both directions
   so the regression is a test failure rather than a corrupted library.
3. **The variant type change is a crate-wide break.** `Change::MintRule.section` going
   `String → Option<String>` breaks every match on the variant. `rg MintRule` shows six sites, all
   in `migration.rs` and `replay.rs`, so the compiler catches the set — but a `..` rest-pattern
   would swallow the change silently, and replay.rs:471 is written with one today. The diff must
   bind `section` explicitly there rather than relying on the rest pattern.

## Notes of note

The follow-on migration that actually mints the consolidated blocks is not in this unit. Whoever
lands it must regenerate the derived views in the same change, or the `views ≡ replay` CI test
fails — that is the one place where this widening does move a fixture.

The report type is `disclosure`; the envelope's closed set carries no "plan" type, and a design
plan disclosing its own scope and rung claims is the nearest member.

## Execution

Executed on the approved plan. Five files changed; no migration was added to the shipped log.

### Diff summary per file

- `crates/mochiko-cli/src/migration.rs` (+30 −6) — `Change::MintRule.section` is now
  `Option<String>`, carrying a doc comment on why the pairing is kind-decided. The `ChangeOp::MintRule`
  arm of `parse_change` became a block: it reads the document first, then takes `section` as `Some`
  when the key is present, `None` when it is absent on a `command-common` or `skill-common`
  document, and returns `op-malformed` when it is absent on anything else, naming the one shape
  that may omit it.
- `crates/mochiko-cli/src/replay.rs` (+33 −6) — the `Change::MintRule` arm of `apply` now matches
  on the section. The decode → mint-once → `schema_of` order is unchanged. `Some` on a library
  returns `op-inapplicable` with a message saying a library carries its blocks at the top level;
  `Some` elsewhere is today's section lookup verbatim; `None` on a library pushes to
  `schema.blocks`; `None` elsewhere returns `op-inapplicable` as an unreachable-but-total guard.
- `crates/mochiko-cli/tests/replay.rs` (+292) — a `LIBRARIES` migration fixture plus
  `with_libraries`, `library` and `validation` helpers, and the seven planned tests.
- `crates/mochiko-cli/tests/migration.rs` (+65) — a `mint_rule` fixture helper and the two planned
  parse-layer tests.
- `plugins/mochiko/migrations/README.md` (+3 −1) — the grammar-table row, exactly as planned, plus
  the two-line addition to the no-bump paragraph.

`src/validate.rs`, `src/model.rs`, `src/views.rs`, `src/render.rs`, `src/conform.rs`, `src/hook.rs`
and `src/genesis.rs` were not touched, as planned.

### Test totals

| | binaries | passed | failed |
|---|---|---|---|
| before (source changed, tests not yet added) | 17 | 463 | 0 |
| after | 17 | 472 | 0 |

The nine added tests are seven in `tests/replay.rs` (55 → 62) and two in `tests/migration.rs`
(33 → 35). Every pre-existing test stayed green across the source change.

### Layers

- `cargo test -p mochiko-cli` — PASS, 472 passed, 0 failed, 0 ignored.
- `cargo fmt --all --check` — PASS, clean (a preceding `cargo fmt --all` rewrote three files,
  whitespace only).
- `cargo clippy --all-targets -- -D warnings` — PASS, no warnings emitted.
- `cargo audit --deny warnings` — PASS, 31 crate dependencies scanned against 1251 advisories, no
  vulnerability and no warning.

### The `.blocks` sweep

`rg -n '\.blocks' crates/mochiko-cli/src/` returns **15 sites across 4 files**: `genesis.rs` 2,
`model.rs` 5, `replay.rs` 3, `validate.rs` 5. Fourteen of those predate this change; the fifteenth
is the new `schema.blocks.push(decoded)` in the mint arm. Every site reads or writes
`RuleSchema.blocks` on the replayed state, so there is no block-resolution path that an appended
block is invisible to. The two `extends` lookups are the ones the plan named, `find_block` and the
inline one inside `resolve_extends`.

### Deviations from the plan, with reasons

1. **Test 2 asserts over `Replay.validation`, not `replay::load` returning `Ok`.** `replay()`
   already runs the hard set into `.validation`, so the finding set is reachable without demanding
   that the small fixture corpus pass every clause — which is what the much larger synthetic corpus
   in `tests/validate.rs` exists for. The test is a control/treatment pair instead: the same stub
   raises `extends-unresolved` when the block is absent and raises nothing when it is minted. That
   is more decisive than an `Ok`, because an `Ok` cannot distinguish "resolved" from "never checked".
2. **The fixture libraries arrive in a second migration, not a second genesis const.** `GENESIS` is
   reused verbatim and `LIBRARIES` (sequence 2) imports the skill-label registry, the two libraries,
   and binds one block from `demo.plain`. Same isolation the plan wanted, with no duplicated corpus.
3. **Test 6 loops over three fields, not one.** `class`, `kind` and `enforces` all reach the same
   `extends-class-local` branch, so one loop covers three at no cost. `when:` is left out on
   purpose: a `when:` on a block would also raise `when-undeclared`, since a common library declares
   no conditions, and that would muddy what the assertion proves.
4. **Test 5 gained a parse-layer twin.** The planned replay-level case is there, and
   `mint_rule_on_a_command_or_skill_schema_needs_a_section` covers a command schema, a skill schema,
   and a non-scalar `section:` at the layer where `op-malformed` actually originates.
5. **The plan's `.blocks` count said "14 sites in five files"; it is four files.** A miscount in the
   plan, corrected above. No consequence for the change.
6. **Risk 3 needed no code beyond what was written.** The existing arm already bound `section`
   explicitly and its `..` swallowed only `doc`, which is read through `change.doc()` above the
   match, so `cargo check` caught the type change at once. The new arm keeps the explicit binding.
