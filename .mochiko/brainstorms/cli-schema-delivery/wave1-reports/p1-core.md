# Wave 1 — seat P1 (core) cycle report

**Seat:** P1 — the typed model, the migration grammar and parser, the replay engine and state
hash, the D6 hard-set validator, the log/grammar version contract.
**Plan:** returned and lead-approved with rulings on all seven open questions and four amendments
(A1 shape preservation, A2 deliverability signal, A3 corpus pins, A4 public canonical hash).
**Gates at close:** `cargo test --all` 99 passed / 0 failed · `cargo fmt --all --check` clean ·
`cargo clippy --all-targets -- -D warnings` clean · `cargo audit --deny warnings` clean over 25
crate dependencies.
**Shipped files:** no file under `plugins/` changed byte-wise. `git status plugins/` is empty.

---

## 1. What was built

| file | lines | what it carries |
|---|---|---|
| `crates/mochiko-cli/src/model.rs` | 1169 | the typed document model, its lossless YAML round trip, the canonical encoder, shared grammar helpers |
| `crates/mochiko-cli/src/migration.rs` | 730 | the migration header, the fifteen change ops, the body hash, the version contract |
| `crates/mochiko-cli/src/replay.rs` | 701 | log loading and ordering, the apply semantics of every op, the state and its content hash |
| `crates/mochiko-cli/src/validate.rs` | 1316 | the finding vocabulary, family and prefix derivation, the hard set, the advisory reports |
| `crates/mochiko-cli/tests/migration.rs` | 310 | 16 tests — canonical encoding, header, hash, version contract, op decoding |
| `crates/mochiko-cli/tests/replay.rs` | 917 | 33 tests — ordering, every op's apply semantics, the id lifecycle, protected content, determinism |
| `crates/mochiko-cli/tests/validate.rs` | 1176 | 38 tests — the positive control, one probe per state-level clause, the shipped-corpus smoke |
| `migrations/README.md` | 116 | grammar summary, the op table, the anchor rule, sequence-range allocation |

`Cargo.toml` gained `sha2 = "0.10"` and nothing else. `lib.rs` gained four `pub mod` lines and
nothing else. `main.rs`, `schema.rs` and `tests/render.rs` were not touched.

### The model

Two decisions shape everything else.

**Permissive parse, strict validate.** No decode rejects a *value*. An unrecognised `class:`,
an undeclared `when:` dimension, a rule with no text — all decode cleanly and surface as findings.
Only *shapes* are decode errors: a `sections:` key that is not a list, a rule that is not a
mapping. The reason is the ported negative-test matrix. A probe asserting the finding "class must
be floor|must|advisory (got 'x')" is unreproducible if the decoder rejects `x` first, so roughly a
third of the matrix would have had nowhere to land. `Class` and `RuleKind` therefore carry an
`Other(String)` arm that keeps the raw spelling for both the message and the round trip.

**Templates and shelf data stay opaque.** They are held as raw YAML values rather than as the
existing `schema::Template` struct, which derives only `Deserialize`. Typing state on it would
have forced a `Serialize` derive into P2's file. P2 re-bases `template <name>` with
`serde_norway::from_value::<Template>(value)`; no change to `schema.rs` was needed from this seat.

### The canonical encoder (A4)

`model::canonical_hash(&Value) -> String` and `model::canonical_bytes(&Value) -> Vec<u8>` are
public, as amendment A4 requires, so P3 can use the same encoder for derived-view equality.

The encoding is tagged and self-delimiting: every scalar length-prefixed, every container
count-prefixed, mapping entries sorted by their own encoded key bytes. Two properties carry the
wave's integrity claims and are asserted directly rather than assumed:

- **Mapping key order does not move the hash.** A YAML re-dump has no such property, which is why
  a hand-written encoder exists at all rather than hashing serialised text.
- **The encoding is injective.** `{ab: c}` and `{a: bc}` are the standard collision trap for a
  naive concatenation; they hash apart here, as do a scalar and a one-element list, null and the
  empty string, and a number and its string spelling.

Sequence order *is* covered, because a section's rule list is ordered data.

### The migration grammar

Fifteen ops, one file per migration, ordered by the header's `sequence`. The full table is in
`migrations/README.md`. Design points worth naming:

- **`set-rule-field` cannot set `id` or `text`.** An id is minted once, and a reword has its own
  op so that a reword is legible in a diff as a reword. The nine settable fields are `labels`,
  `class`, `kind`, `when`, `pointer`, `extends`, `enforces`, `anchor`, `note`; `value: ~` clears.
- **`note:` carries what was a YAML comment.** See the handoff to P3 in section 5.
- **A rule is decoded by exactly one code path** whether it arrives through `import-document` or
  through `mint-rule`, so the two can never diverge in how a rule is understood.

### Replay and the deliverability signal (A2)

`replay::replay_dir` collects findings and skips the failing op rather than stopping at the first,
which is what `migrate validate` wants: a maintainer fixing a log should see every problem in it.
That means a state can exist that nobody may render from, so amendment A2's signal is explicit:

- `Replay::is_deliverable()` is false whenever any rejecting finding was raised.
- `Replay::rejecting()` iterates just those findings.
- **`replay::load(dir) -> Result<State, Vec<Finding>>`** is the delivery path's entry point.
  `Ok` is a state safe to render from; `Err` is the finding list the caller prints before exiting
  1. P2's `rules` and `template` should call `load`, never `replay_dir`.

`State::content_hash()` covers documents only, not the mint-once ledger, per the Q5 ruling. It is
a view-drift signal, so it must not move when nothing renders differently.

### The hard set

Thirty-four rejecting codes and six advisory ones, each a stable handle so message wording can
improve without breaking the suite. Findings render as `code · schema · id · message` with `-` for
an unfilled column. Severity is a property of the code, never of the occurrence, so a code can
never be advisory in one place and blocking in another.

Family and prefix derivation reproduce the shipped checkers exactly:

- A skill's family comes from its directory stem: `authoring-` and `patterns-` name their own
  families and everything else falls through to the review set, which the small families reuse by
  ruling. Each family carries its own six sections; the patterns family ships no common library,
  so any `extends:` there is a finding.
- A command's prefix is read off its own section ids. Sections that disagree raise a finding and
  the set-wise check is skipped rather than guessed at.
- A common library's block prefix comes from its own name, by stripping a leading `skill-`:
  `skill-review-common` holds `review-common.<slug>` blocks.
- `${var}` closure is checked on *resolved* text against the **binding** document's vars. See the
  defect in section 3.

## 2. The red/green/refactor trail

Four cycles, each opened with a failing test.

| cycle | red | green | refactor |
|---|---|---|---|
| 1 — canonical encoder | `tests/migration.rs` would not compile: no `model`, no `migration` | `model.rs` types plus the encoder | — |
| 2 — migration grammar | 15/16, one failure | the header, ops and hash | the failing test was a broken fixture, repaired (below) |
| 3 — replay | `tests/replay.rs` would not compile: no `replay`, no `validate` | `validate.rs` finding vocabulary, `replay.rs` engine | 31/32, one real design question resolved (below) |
| 4 — the hard set | 35/37, two failures | the validator | one real defect fixed, one honest exemption recorded |

Three failures were worth more than the tests that passed.

**Cycle 2 — a broken fixture, not a broken parser.** The "missing header field" test built its
fixtures by dropping one line, which for `changes:` left the list items orphaned and produced
invalid YAML rather than a migration missing a field. The parser was right; the fixture now drops
the block the key owns.

**Cycle 3 — tombstoning a populated section.** The plan did not say what happens when
`tombstone-section` names a section that still holds rules, and my first fixture assumed it would
succeed. It should not: a section tombstone would carry its rules out implicitly, and a floor rule
would then leave with no anchor — the protected-exit check bypassed by one level of indirection.
The op is rejected while the section holds rules, and `tombstoning_a_section_that_still_holds_rules_is_rejected`
pins it. This is an addition to the plan's grammar, disclosed in section 4.

**Cycle 4 — a real defect in the validator.** See section 3.

## 3. The defect the corpus smoke caught

The `${var}` closure check ran on the common libraries themselves, and reported six unbound
placeholders across `common.yaml`, `skill-review-common.yaml` and `skill-authoring-common.yaml`.

The check was wrong. A placeholder in a library block substitutes from the **binding** schema's
vars, never from the library's — a library declares no vars at all, by design. The shipped
checkers get this right by only ever checking resolved text inside a binding schema, and the
library is loaded as a block source rather than validated as a document in its own right.

Fixed: closure is skipped for `command-common` and `skill-common` documents. Nothing is lost,
because every binding of every block is still checked in its own right; coverage is transitive.
A second, smaller bug surfaced alongside it — a placeholder repeated inside one rule's text
reported twice — and is now deduplicated per rule.

This is the value of running the hard set against the real corpus rather than only against
synthetic fixtures, and it is why that test exists.

## 4. Deviations from the wave plan, each named

1. **The migration hash covers `{id, sequence, anchor, changes}`**, not `changes:` alone as
   wave-plan §2 specifies. Lead-ruled at plan approval (Q1). Without it the anchor — which is the
   evidence that protected content left by ruling — would be editable after the fact.
   `intent:` remains outside the hash so prose can be corrected. Both limbs are asserted.
2. **Command rule ids must lead with their schema's prefix.** Wave-plan §3 requires it; the
   shipped command checker derives a prefix from section ids but never asserts that rule ids use
   it. Lead-ruled (Q2). Verified before building: all six shipped command schemas already conform,
   with zero non-conforming rule ids, so this closes a gap rather than widening the corpus's
   obligations.
3. **`tombstone-section` is rejected while the section holds rules.** Not in the plan; argued in
   section 2 above. Without it the anchor rule is bypassable.
4. **`import-document` over an existing document is rejected** (Q3); **`registry-retire` records
   rather than deletes** (Q7); **`registry-add` over a live label is rejected**. All lead-ruled.
5. **`registry-add` and `registry-retire` keep the plan's `registry:` field name**, carrying the
   same document-reference type as `schema:` elsewhere. No grammar difference, only the field name
   the plan already chose.
6. **The two shipped rules with an empty `enforces:` raise a finding when the raw files are
   loaded.** This is correct behaviour, not a deviation in the validator; see section 5.

## 5. Handoffs

**To P3 — the comment-to-data carry.** `setup.yaml` carries two rules with an explicitly empty
`enforces:` whose reason lives in a `# D6 empty-with-reason:` YAML comment above the field, at
lines 387 and 406. The shipped checker reads that comment straight off the file. Comments do not
survive a typed model, so the grammar carries the reason as a rule `note:` instead, and
**genesis must lift both comments into `note:` fields.** The rules are
`setup.fail.unclosed-trace` and `setup.fail.floor-category-uncovered`.

Two tests pin this rather than papering over it. `the_shipped_corpus_validates_with_no_rejecting_finding`
allows exactly these two findings and no others, failing if a third appears. Its sibling,
`the_shipped_corpus_is_clean_once_the_comment_carried_reasons_are_data`, sets the two `note:`
fields the way genesis will and asserts the corpus is then clean outright — so the exemption
cannot hide a real defect. Both findings disappear when genesis lands.

**To P3 — the entry points.** `Document::from_value(kind, &value)` is the decoder genesis needs;
`tests/validate.rs::shipped_documents` shows the address derivation (a document's own `kind:`
field names its kind, a skill's name is its directory, everything else is the file stem) and can
be lifted into `genesis.rs`. `Document::to_value()` is the view emitter's data source, and
`model::canonical_hash` is the semantic-equality comparator.

**To P2 — three surfaces.** `replay::load` is the delivery entry point (section 1).
`migration::GRAMMAR_RANGE` is `(1, 1)` and `migration::INSTALL_COMMAND` holds the upgrade command
named in the D5 halt message, so the exact text has one home rather than a second copy in the CLI
surface. `migration::ParseError::GrammarVersion` renders the whole halt message already.

## 6. Pre-code ladder disclosures

Run per `mochiko:patterns-code-minimalism`; what was **not** built, and why.

| rung | not built | why |
|---|---|---|
| exist at all | SQLite, a persistent cache, an index file | D1 defers all three to a measured need. None is measured. |
| exist at all | a `genesis` emitter, a views emitter, a similarity scorer, the matrix ports | P3's seat. |
| exist at all | anything in `main.rs`, `cli.rs`, `render.rs`, `schema.rs` | P2's seat. This seat is library-only. |
| exist at all | `.md` scaffold checks (the Python 7c/7d probes) | Dead under D6: the `.md` no longer enumerates sections or pins counts. |
| exist at all | `DECISIONS.md` anchor *resolution* | Wave-plan §3 scopes wave 1 to anchor *format*. Resolution is advisory until the repo path is known. |
| exist at all | provenance-sidecar reading | Anchors arrive as `anchor:` fields through genesis. The model carries the field; nothing here reads the sidecar. |
| in codebase | a second rule decoder for `mint-rule` | Reuses the `import-document` path, so the two cannot diverge. |
| stdlib | `walkdir` | One flat directory. `fs::read_dir`. |
| stdlib | `indexmap` | `BTreeMap` gives the sorted order the hash needs anyway. |
| stdlib | `thiserror`, `anyhow` | Findings are a data type, not an error type. A plain enum with `Display` is enough. |
| stdlib | `serde_json` or a canonical-JSON crate | A recursive encoder over the YAML value type is about fifty lines and avoids a lossy YAML-to-JSON conversion. |
| installed dep | serde's tagged-enum dispatch for the change ops | `#[serde(tag = "op")]` would have worked, but hand-written dispatch produces the exact finding text the matrix port needs, including naming the unknown op. Deliberate step past this rung. |
| one line | `hex` | One `write!` loop over the digest bytes. |
| one line | `regex` | Lead-ruled (Q4). Every grammar here is fixed and small; hand-written scanners keep the audit surface at four dependencies. |

Dependency total after this seat: `serde`, `serde_norway`, `sha2`, and their transitives — 25
crates in `Cargo.lock`, `cargo audit` clean.

## 7. Test tally

| suite | tests | covers |
|---|---|---|
| `tests/migration.rs` | 16 | canonical encoding (order independence, injectivity, shape), the header, the body hash and what it covers, the version contract with its install line, one decode assertion per op |
| `tests/replay.rs` | 33 | ordering with legal gaps, sequence collision, the apply semantics of all fifteen ops, `null` clears, ids surviving reword and move, mint-once, tombstone integrity, protected content, anchor format, replay determinism, the A2 signal |
| `tests/validate.rs` | 38 | the synthetic positive control, one probe per state-level clause, family and prefix derivation, the finding's rendered shape, and the four corpus tests |
| `tests/render.rs` | 12 | pre-existing, untouched, still green |
| **total** | **99** | |

### The corpus pins (A3)

Read by the model from the 50 shipped files, asserted in
`the_shipped_corpus_matches_its_recorded_census`:

| figure | model reads | record says |
|---|---|---|
| documents | 50 | 50 |
| live command rules | 321 | 321 |
| live skill rules | 695 | 695 |
| live rules in total | 1016 | 1016 |
| skill floors | 226 | 226 |
| command fail nodes | 36 | — |
| declared command floors | **110** | 112 by grep, 110 by the checker |

**The instrument difference, as A3 asks.** The record's 112 is a `grep -c 'class: floor'` count.
Two of those matches are prose inside rule text rather than rule declarations —
`architecture.yaml:52` and `implement.yaml:75` each write the phrase `class: floor` inside a
sentence. The declared floors are 110, which is the same figure the shipped checker reports and
which the record already notes as an instrument gap. The model is not forcing the number; it
agrees with the checker.

### The round-trip guarantee (A1)

`every_shipped_document_round_trips_through_the_model` decodes each of the 50 shipped files and
re-encodes it, asserting canonical-hash equality with the original. **It passes over all 50 with
no normalisation anywhere**, which is a stronger result than the amendment asked for: rather than
documenting which fields normalise, there are none.

The two dual-shape fields the corpus uses in both forms are preserved exactly, as A1 requires:

- a `when:` term's value, a scalar 77 times and a list 19 times across the corpus;
- a condition's `values:`, the word `presence` 25 times and a list 20 times.

Declaration order is preserved for `vars:`, `conditions:`, `moments:` and a registry's `labels:`,
so a regenerated view can keep the corpus's key order. Canonical hashing sorts mapping keys
regardless, so ordering affects the emitted file's shape and never equality.

## 8. Open items

1. **Genesis must lift the two `enforces:` reason comments** into `note:` fields (section 5).
   Tracked by two tests that fail if a third such rule appears.
2. **Sequence-range allocation past wave 1** is a lead decision. `migrations/README.md` carries
   the table with wave 1 filled in and later waves marked as assigned at wave open.
3. **The advisory reports are complete but unexercised against the real corpus** beyond running
   cleanly. Their output is not asserted, because the wave plan puts the deixis marker list and
   the coverage reports in the advisory set where a false positive must never block. P3's
   similarity scorer joins them.
4. **`cargo audit` fetches the advisory database over the network.** It is clean today. The other
   three gates are offline.

## 9. Suggested commit

Nothing was committed. Suggested message:

```
Add the migration grammar, replay engine and hard-set validator

Wave 1 seat P1 of the CLI schema-delivery build. The crate gains the typed
document model with a lossless YAML round trip, the migration file grammar
and its canonical body hash, the in-memory replay engine with a content
hash over the replayed state, and the D6 hard-set validator with 34
rejecting finding codes and 6 advisory ones.

The validator runs clean over all 50 shipped schema files, matching the
recorded census (321 command rules, 695 skill rules, 226 skill floors, 110
declared command floors). No shipped file changed.

sha2 is the only new dependency.
```
