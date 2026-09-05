# P1 — crate + log (report)

Built on the lead-approved plan `p1-op-plan.md`, as amended by the lead's frozen-fixture ruling and
the legend addendum. Everything in the seat's scope landed; nothing is left undone. No git mutation,
no commit.

## State hash, before and after

Verbatim from `cargo run -q -p mochiko-cli -- migrate status --plugin-root plugins/mochiko`.

Before:

```
log plugins/mochiko/migrations · grammar 1 · sequences 1..1 (1 migration)
state sha256:8b61de5a3b4cca8ed581df70f6ec036d8f05a8e67de06b66c6f1ae9b86c8bdd4 · 50 documents · 1016 rules
```

After:

```
log plugins/mochiko/migrations · grammar 1 · sequences 1..2 (2 migrations)
state sha256:8972891099f77b1080b243f73adc7ea5ae0c3c9479cb17414cbe1fd97cfd43fd · 50 documents · 1016 rules
```

The document and rule counts are unchanged, which is what a prose reword should do. `migrate
validate --plugin-root plugins/mochiko` reports `0 rejecting · 105 advisory`, the same advisory
figure recorded at wave 1. The D6 hard-set census is unmoved: the fidelity census test still pins
321 command rules, 695 skill rules, 226 skill floors, 110 command floors, 36 command fail nodes.

## What was built

**The `reword-section` op.** `ChangeOp::RewordSection`, sixteenth in the grammar, placed between
`mint-section` and `tombstone-section`. Each of `title:`, `intent:` and `note:` parses into a
three-state `SectionEdit` — `Untouched`, `Clear`, `Set` — so a field the change never names stays
distinct from one it clears. Parse rejects, all as `op-malformed`: naming none of the three; `~` on
a `title:` or an `intent:`, neither of which a section may lose; an empty or blank value for any of
the three, `note: ~` being the clear; a value that is not a scalar; a missing `schema:` or `id:`.
Apply reports a tombstoned section as retired rather than as absent, falls through to "no such live
section" for an unknown id, and rejects a rules-less document. It reads and writes nothing under
`section.rules`, so no ruling anchor is owed — protection is a per-rule property. Grammar stays 1.

**`migrate stamp <file>`.** Reads the file, stamps it through `migration::with_hash` under its own
basename so the filename-versus-`sequence:` check applies, and rewrites it in place. Exit 0; 1 on a
body that is not a well-formed migration, which is left untouched on disk; 2 on a path that cannot
be read or written. The bytes come from the view writer over the re-parsed stamped body, and a
leading comment block is carried through verbatim.

**Migration `0002-fail-conditions-intent.yaml`**, six `reword-section` changes, header anchor
`2026-09-03 cli-schema-delivery D3`, hash
`sha256:47abe5a344e71b73fb234c48f1305c9e80af40001d9629e3277ade56c5236757`. Stamping reformatted the
hand-written body into the log's own folded layout, which is the point of routing the write through
the view writer rather than serde.

**The six snapshot lines**, applied by hand, `git diff --stat plugins/mochiko/schemas/`:

| file | changed lines |
|---|---|
| architecture.yaml · brainstorm.yaml · feature.yaml · implement.yaml · setup.yaml · specify.yaml | 1 each |

**The frozen corpus fixture**, per the lead's ruling, at
`crates/mochiko-cli/tests/fixtures/genesis-corpus/`: 51 files — 20 command and family schemas, 30
skill schemas, the provenance sidecar — mirroring the repo layout the builder expects. 603,801 bytes
of content, 700 KB on disk. Every file was verified byte-identical to its HEAD original, so the
freeze is genuinely the pre-edit corpus. `the_committed_genesis_regenerates_byte_identically` now
builds from it and carries a one-line comment naming record D8 and the 2026-09-04 freeze date, with
the reasoning in the helper's doc comment.

**README.** One `reword-section` row in the change-ops table, `migrate stamp` added to the
working-on-the-log command block, and the "Writing the hash" section rewritten around the new
subcommand as the authoring path. Nothing else changed.

**The legend widening** (lead addendum). The preamble's fixed legend block gains the three lines
verbatim as given, after the existing six. The block is still fixed text, it is still delivered to
skills as well as commands, and the preamble's end line still reads `preamble · 0 rules` — checked
against the live render, not only the fixture.

| legend | bytes | grammar lines |
|---|---|---|
| before | 612 | 6 |
| after | 845 | 9 |

That is **+233 bytes**, paid once per `preamble` render by every converted primitive. The size is
now pinned by its own test, so it cannot grow unnoticed.

## Test tally

331 tests pass, 0 fail, across the whole suite. 29 are new, taking the five touched files from 136
to 165.

| file | before | after |
|---|---|---|
| tests/migration.rs | 25 | 33 |
| tests/replay.rs | 46 | 55 |
| tests/cli.rs | 26 | 35 |
| tests/fidelity.rs | 10 | 12 |
| tests/render.rs | 29 | 30 |

Four layers, all green: `cargo fmt --all --check` clean, `cargo clippy --all-targets -- -D warnings`
clean, `cargo test --all` 331 passing, `cargo audit --deny warnings` exit 0. The opt-in full-corpus
similarity sweep also passes, 48 tests in 108 seconds, unchanged — section intents are not scored.

The live render was checked directly. `rules brainstorm --section brainstorm.sec.fail-conditions`
prints "fails the run", `rules architecture --section arch.sec.fail-conditions` prints "fails the
visit", and neither block contains "hard-codes".

## Deviations from the approved plan

1. **The §0 recommendation was replaced by the lead's ruling**, and the plan's `genesis::build_from`
   was not written. The frozen fixture is a strictly better answer: it proves the YAML to typed
   model to genesis round trip permanently, against real corpus content rather than against the
   log's own imports, and record D8 had already called for it.
2. **The op sits between `mint-section` and `tombstone-section`**, not after `tombstone-section` as
   the plan's README wording had it, mirroring the rule ops' own mint then reword then tombstone
   order. The README row was placed to match.
3. **The `migrate stamp` genesis probe was kept**, because it passed on the first run. Stamping a
   copy of the committed genesis returns it byte-identical, so the authoring path and the generator
   agree and the log's largest file cannot be silently reformatted by a stray stamp.
4. **`the_log_replays_into_a_deliverable_state` moved from `replay::load` to `replay::load_full`**
   so it could assert the sequence list is `1..2`. No existing assertion was dropped.
5. **The fidelity module doc was corrected** where it described the actual side of the comparison as
   the document built from the genesis file alone; it is now the whole log. One phrase.
6. **Two tests beyond the plan's list**: one pinning that a reword edits only the fields it names,
   and the render assertion split into its own test rather than folded into the intent pin.
7. **One existing assertion was narrowed for the legend addendum.**
   `a_skill_preamble_omits_moments_and_the_fail_pin` asserted the bare word "moments" never
   appears in a skill preamble. The new legend line names `moments:` in prose and the legend goes
   to skills too, so the assertion now checks for the `moments` block itself. This is the same
   narrowing the test already applied to the fail pin two lines below, for the same reason, and its
   comment says so. The guarantee is unchanged: a skill still renders no moments block.

## For the validator

No strip entry is owed for the six schema edits. Ruling D2 ended schema-content strips going
forward and the migrations README states the reason: the verbatim prior content is in the log by
construction, so the migration file is the record. The migration carries its ruling anchor. The
`.md` strip entries for wave 4 are P2's, not this seat's.

The plugin version still reads 0.104.0 in every render; the bump to 0.105.0 is the lead's at landing.

## Rework — V1 (2026-09-04, attempt 1 of 3)

All four V1 items are closed. None touched the op's behavior, so no test changed its meaning and
none was added.

**1. Why an additive op stays in grammar 1** — `plugins/mochiko/migrations/README.md`, a new
paragraph closing the change-ops section. It states that `reword-section` was added at wave 4 with
the log still at grammar 1 because no binary is published, so no deployed reader can meet a file it
cannot understand; that the D5 range `1..1` freezes at the first publish with whatever ops the
grammar carries by then; and that a later binary lacking an op rejects the file loudly under the
version contract, naming the install command, rather than skipping the op and replaying a state
quietly missing a change. It closes by naming what a grammar bump is actually reserved for: a change
that would make an existing file mean something different.

**2. The byte test's panic hint** — `crates/mochiko-cli/tests/fidelity.rs`. V1 is right that the old
hint was a trap: it told a maintainer to run `genesis emit` against the live tree, which after
migration 0002 would rewrite the committed genesis from a corpus later migrations have already
carried forward, folding their content back into sequence 1 and losing the history. The hint now
names the frozen fixture root explicitly, says the committed genesis changes only when that fixture
changes, and states the consequence of the live-tree build so the reason travels with the command.

**3. "330 passing"** — already 331 in both places when I checked, at the four-layers paragraph and
in the tally sentence above it. I corrected it when the legend addendum moved the count, which was
after the copy V1 graded. No edit was needed and none was made.

**4. The snapshot-file clarifying sentence** — `plugins/mochiko/migrations/README.md`, placed
directly under the "derived views are regenerated, never hand-edited" bullet, which is exactly the
line it qualifies. It records that the shipped snapshots are transition-clause copies kept
semantically equal to the replay by the CI view ≡ replay test, that a migration changing their
content is mirrored by a hand edit of the same lines rather than a regeneration because
regenerating would drop their protected header comments, and that wave 6 retires them.

**Four layers, re-run, all green:** `cargo fmt --all --check` clean, `cargo clippy --all-targets --
-D warnings` clean, `cargo test --all` **331 passing, 0 failing**, `cargo audit --deny warnings`
exit 0. The tally is unchanged from the pre-rework run, as it should be for three prose fixes and a
panic string. The log is untouched: state hash still
`sha256:8972891099f77b1080b243f73adc7ea5ae0c3c9479cb17414cbe1fd97cfd43fd` at sequences 1..2.
