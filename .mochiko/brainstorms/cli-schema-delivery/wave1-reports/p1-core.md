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

---

# Fix round 1

**Audit:** `v1-core-audit.md` — FAIL, 3 blocking, 12 advisory. All fifteen taken, plus the two P2
deltas the lead granted. Attempt 1 of 3.
**Gates at close:** `cargo test --all` 127 passed / 0 failed (was 99) · `cargo fmt --all --check`
clean · `cargo clippy --all-targets -- -D warnings` clean · `cargo audit --deny warnings` clean
over the same 25 dependencies. No new dependency. Still no file under `plugins/` changed.

The audit was right, and B1 is the finding that matters: the guarantee the whole wave is built to
make mechanical did not hold. Everything below was fixed test-first — a red test reproducing the
finding, then the change.

## Blocking

**B1 — protection was bypassable in one migration.** `Rule::is_protected()` re-derives protection
from the rule's current `class`, `kind` and `anchor`, and `set-rule-field` may write all three.
Ops in one `changes:` list apply in order, so a migration downgraded a floor rule and then retired
it, and the retire step saw an ordinary rule. The audit's probe retired a floor rule, a fail rule
and an anchored rule with zero findings.

Fixed per the lead's ruling: `lowers_protection` (`replay.rs`) asks whether a `set-rule-field`
would remove one of the three protection sources — `class` away from `floor`, `kind` away from
`fail`, or an `anchor` cleared or changed — and if so the op requires the **migration's own header
`anchor:`**, well-formed, exactly as `supersede-rule` does. Raising protection needs no authority.
The migration's anchor now travels into `apply` rather than being re-derived from the state the
change is about to alter.

Six new tests pin it, including the audit's exact probe and its inverse:
`lowering_protection_without_a_ruling_is_a_protected_exit`,
`clearing_protection_without_a_ruling_is_a_protected_exit`,
`clearing_or_changing_an_anchor_without_a_ruling_is_a_protected_exit`,
`a_ruling_anchor_on_the_migration_authorises_lowering_protection`,
`a_malformed_migration_anchor_does_not_authorise_lowering_protection`, and
`raising_protection_never_needs_a_ruling`.

**One residual, stated rather than hidden.** The ruling is per migration, so an anchored migration
may downgrade a floor and a later unanchored migration may retire what is by then an ordinary
rule. That is coherent — removing a floor's floor-ness is itself the protected exit, and it was
ruled — but it is weaker than the audit's preferred sticky-set design, which would keep the rule
protected until superseded. I implemented what was ruled. If the lead wants the stronger form, it
is a per-document protected-id set in `State` and about twenty lines.

**B2 — the body hash was optional.** Three spellings skipped the check: the key absent, `hash: ""`,
and `hash: ~`. That nullified the Q1 amendment, since an editor need not forge a hash to change an
anchor, only delete a line. The hash is now required and reports as `grammar-header` when absent.

Because fixtures need valid hashes, the stamping helper is public: `migration::with_hash(file,
source)` returns the body carrying its correct hash, replacing any stale one, and
`migration::compute_hash(&migration)` returns the value alone. Both are documented in
`migrations/README.md`. Writing `with_hash` surfaced a second bug in my own code, caught by
`with_hash_replaces_an_existing_hash_rather_than_duplicating_it`: the stamping path validated the
very hash it was about to overwrite, so it could not repair a stale one — which is most of what it
is for.

**B3 — `replay::load` never ran the hard set.** `validate::validate` had no caller in `src/`, so
`load`'s `Ok` meant only that the ops applied. My own report handed P2 a stronger contract than the
code kept, which is the worse half of the finding. `replay()` now runs the hard set over the
finished state and stores it in `Replay::validation`; `is_deliverable()` and `load` consult both
passes. `Replay::findings` still holds replay findings alone, so the two stay distinguishable, and
`load`'s `Err` carries everything including the advisory reports.

That change made the replay suite's genesis fixture inadequate — it carried two sections, not the
canonical six — so the fixture is now a minimal but genuinely valid corpus. That is the honest fix:
a genesis fixture that cannot pass validation was never a good fixture.

## Advisory

| # | what changed |
|---|---|
| A1 | A `.yaml` in the log that is not `NNNN-<slug>.yaml` raises the new `log-file-name` code instead of being dropped. `genesis.yaml`, or `O001-` typed with a letter O, previously replayed as if absent. |
| A2 | `every_rejecting_code_is_raised_by_some_probe` replaces the tautology. It runs every state-level mutation and builds thirteen real migration logs for the log-level codes, then asserts the raised set equals `Code::REJECTING` in both directions. A new code with no probe now fails it. |
| A3 | `is_anchor` is anchored at both ends, range-checks month and day, and validates the decision segment. The dead `!slug.is_empty()` branch is gone. **The corpus decided the grammar:** all 597 sidecar anchors write the tail bare (`D4`), not bracketed, so both spellings are accepted and nothing else is. Tightening to brackets alone would have rejected the whole sidecar. |
| A4 | `read_grammar` parses the grammar field before any header integer cap, so every out-of-range whole number reaches the D5 halt with its install line. Probed at 2, 99, 999999 and `u32::MAX`. |
| A5 | `mint-section` rejects a section value carrying `rules:` rather than discarding them. |
| A6 | The deixis list is ported exactly — ten markers including `see below` and the `there is no <X> section` wildcard — matched on word boundaries, so `this sectional` no longer fires. |
| A7 | The unused-declared-moments advisory is ported, including the shipped checker's deliberate weakness: the prose half is a bare substring test, so it under-reports and never invents. |
| A8 | `encode_canonical` is depth-bounded at `MAX_CANONICAL_DEPTH` (64) and emits a marker past it rather than recursing off the stack; `canonical_depth` is public, and an over-deep opaque document raises the new `depth-exceeded` code. |
| A9 | `ParseError::Change` is split into `UnknownOp` and `MalformedChange`, so a known op missing a field reports as the new `op-malformed` rather than as an unrecognised op. |
| A10 | A non-string `when:` dimension key and a non-string list item are findings; neither is coerced to `""`. |
| A11 | `the_round_trip_preserves_declaration_order` asserts document key order, section key order, and the declaration order of `vars`, `conditions`, `moments` and registry `labels`. **One honest limit:** rule *field* order is normalised to a fixed canonical order, because preserving it would mean storing a per-rule key sequence and deciding where every op inserts. P3's view test is semantic equality, so this costs nothing there; it is disclosed rather than claimed. |
| A12 | Section ids get the `id-format` check directly, so a malformed one is reported as what it is and is still caught when the prefix will not derive. |

## P2 deltas

- **D-1** `Replay::grammar() -> Option<u32>` carries the applied log's grammar; `load_full` returns
  the whole `Replay`, and `load` is re-expressed on it.
- **D-2** `ResolvedRule` (was the private `Resolved`), `resolve_extends` (was `check_extends`) and
  `placeholders` are public, so the render path resolves `extends:` and `${var}` through this
  implementation. A renderer resolving inheritance differently from the validator would show
  guidance the hard set never graded.

## Codes

`Code::REJECTING` grew from 34 to 37 — `op-malformed`, `log-file-name`, `depth-exceeded`.
`Code::ADVISORY` grew from 6 to 7 — `unused-moment`. Every one is probed, and A2's guard now
enforces that.

## Test tally

| suite | before | after |
|---|---|---|
| `tests/migration.rs` | 16 | 25 |
| `tests/replay.rs` | 33 | 46 |
| `tests/validate.rs` | 38 | 44 |
| `tests/render.rs` | 12 | 12 (untouched) |
| **total** | **99** | **127** |

The corpus pins are unchanged and still green: 50 documents, 321 command rules, 695 skill rules,
226 skill floors, 110 declared command floors, 36 fail nodes. The round trip still passes over all
50 shipped files with no normalisation.

## Documentation

`migrations/README.md` gains the required-hash rule and the stamping helpers, the file-naming rule,
the lowering-protection clause, the corrected anchor grammar, the `mint-section` rule, and the
`load` contract.

## Suggested commit

Nothing committed. The original message stands, with a second paragraph:

```
Fix round 1 closes the independent audit's three blocking findings: a
protected exit reachable by downgrading a rule and retiring it in the same
migration, an optional body hash that left the anchor it covers editable,
and a load path that never ran the hard set. Twelve advisory findings and
two P2 deltas land with them. 127 tests, no new dependency.
```
