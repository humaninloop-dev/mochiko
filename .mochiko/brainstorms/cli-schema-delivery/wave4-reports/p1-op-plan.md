# P1 — `reword-section`, `migrate stamp`, migration 0002 (plan)

**Pre-wave state** (`migrate status --plugin-root plugins/mochiko`, this worktree): `grammar 1 ·
sequences 1..1 (1 migration)` / `state sha256:8b61de5a3b4cca8ed581df70f6ec036d8f05a8e67de06b66c6f1ae9b86c8bdd4 · 50 documents · 1016 rules`.

## 0. Blocking finding — the snapshot edits break genesis regeneration

`tests/fidelity.rs::the_committed_genesis_regenerates_byte_identically` rebuilds genesis from the
**shipped corpus** and byte-compares it with the committed `0001-genesis.yaml`, so a snapshot intent
edit turns it red. Probed on a scratchpad copy (no repo file touched): the clean copy regenerates
byte-identically; after one intent edit the rebuild differs at the `hash:` header and that intent.

Structural, not a plan defect: `fidelity.rs` asserts both *corpus → genesis bytes* and *corpus ≡
full-log replay*, and both hold only while the log is genesis alone. Leaving the snapshots unedited
does not escape it — `every_shipped_document_survives_the_log_field_by_field` compares section
intents and goes red instead, and the §5 release gate wants the derived views current.

**Recommendation:** re-scope the byte test to rebuild from the documents the committed genesis
itself imports, via a new `genesis::build_from(&[(DocRef, Document)])` that `build(root)` delegates
to. That keeps the whole writer/format determinism guarantee and drops only "genesis is a faithful
import of the *current* corpus" — false by construction after any content migration, and its content
half is already carried by the corpus-vs-full-log field-by-field test plus the required `hash:`,
which rejects an unstamped hand-edit of genesis at parse. `genesis::PREAMBLE` stays byte-frozen: it
is emitted into the file. Sole deviation from §2; I touch `fidelity.rs` only on the lead's word.

## 1. `reword-section` — parse

`ChangeOp::RewordSection` ("reword-section"), added to `ChangeOp::ALL` (15 → 16); grammar stays 1
(§7). `Change::RewordSection { doc, id, title, intent, note }`, each field a
`SectionEdit { Untouched | Clear | Set(String) }` so absent and explicitly-null stay distinct, as
`set-rule-field` already distinguishes them. `target_id()` returns the section id, beside
`TombstoneSection`. Rejections, all `MalformedChange` (`op-malformed`), all at parse: none of the
three fields present · `title: ~` or `intent: ~` (a section always carries both) · an empty or
whitespace-only value for any of the three, `note: ~` being the clear · a non-scalar value ·
`schema:` missing or naming no kind, `id:` missing (existing `doc_ref` / `field_str` paths).

## 2. `reword-section` — apply

In `replay::apply`, ahead of the lookup: a tombstoned id rejects with its own message, an unknown id
falls through to "no such live section", a rules-less document rejects via `schema_of`. Then `Set`
writes `section.title` / `intent` / `note = Some(..)`, `Clear` writes `note = None`, `Untouched`
writes nothing; `section.rules` is never read or written. No anchor obligation — protection is
per-rule (floor · fail · anchored) and no rule is touched, so `lowers_protection` and `retire` are
off this path. The hard set does not key on intents: the only intent-facing checks are the retired
`fail-condition` selector and non-emptiness (`validate.rs:1449`, `:1460`), and the new text trips
neither.

## 3. `migrate stamp <file>`

`MigrateAction::Stamp { file: PathBuf }` → `run_stamp`. Reads the file, runs
`migration::with_hash(<basename>, &source)` so the filename-vs-`sequence:` check applies, writes it
back. Exit 0; **1** on a body that is not a well-formed migration, printing the rejection; **2** when
the path cannot be read or written (usage, as `views emit` treats a write failure). No other file is
opened. The bytes written are `views::to_yaml` over the re-parsed stamped value, not `with_hash`'s
own string: `genesis::build` routes the same way and says why in place — `with_hash` re-serialises
through serde and its output is used "for the hash alone", the file written by the view writer. A
leading `#` comment block carries over verbatim, so stamping never strips a generated-file preamble.

## 4. Migration `0002-fail-conditions-intent.yaml` (body before stamping)

```yaml
grammar: 1
id: 0002-fail-conditions-intent
sequence: 2
intent: 'Reword the six fail-conditions section intents: the count is printed by the render, never hard-coded in the .md.'
anchor: 2026-09-03 cli-schema-delivery D3
changes:
  - {op: reword-section, schema: command/architecture, id: arch.sec.fail-conditions, intent: 'The kind: fail set — any one standing fails the visit; the .md Not-done line cites the count this render prints.'}
  - {op: reword-section, schema: command/feature, id: feat.sec.fail-conditions, intent: 'The kind: fail set — any one standing fails the visit; the .md Not-done line cites the count this render prints.'}
  - {op: reword-section, schema: command/brainstorm, id: brainstorm.sec.fail-conditions, intent: 'The kind: fail set — any one standing fails the run; the .md Not-done line cites the count this render prints.'}
  - {op: reword-section, schema: command/implement, id: impl.sec.fail-conditions, intent: 'The kind: fail set — any one standing fails the run; the .md Not-done line cites the count this render prints.'}
  - {op: reword-section, schema: command/setup, id: setup.sec.fail-conditions, intent: 'The kind: fail set — any one standing fails the run; the .md Not-done line cites the count this render prints.'}
  - {op: reword-section, schema: command/specify, id: spec.sec.fail-conditions, intent: 'The kind: fail set — any one standing fails the run; the .md Not-done line cites the count this render prints.'}
```

Written in flow style because the committed bytes come from the stamp writer regardless; the two
intent texts are verbatim per §2, `visit` for the two desks and `run` for the four runs. The header
anchor is not required (nothing protected exits) and rides as provenance, as genesis carries its
own. Sequence 2 is the lead's allocation.

## 5. The six snapshot edits

One hand edit per `plugins/mochiko/schemas/<cmd>.yaml`: `hard-codes this set's count.` becomes
`cites the count this render prints.`, nothing else. Five are a single line (`architecture.yaml:484`
· `brainstorm.yaml:246` · `implement.yaml:888` · `setup.yaml:372` · `specify.yaml:452`);
`feature.yaml` already folds across two and its first line ends at "the .md Not-done line", which the
new text preserves, so only `:469` changes. Six files, one changed line each.

## 6. Tests, written before the code

- `tests/migration.rs` — all three fields; each alone; no field rejects; `title: ~` / `intent: ~`
  reject; `note: ~` decodes as `Clear`; empty strings reject; a non-scalar rejects; the op-coverage
  fixture at `:291` gains the op, its count moving 15 → 16.
- `tests/replay.rs` — title, intent and note set; note cleared; the section's rules and ids
  unchanged; unknown section, tombstoned section and rules-less document each inapplicable with
  their own message; a section of `kind: fail` rules reworded with **no** header anchor raises no
  `ProtectedExit`; two migrations replayed twice give one `content_hash`, order-swapped they do not.
- `tests/cli.rs` — `migrate stamp` stamps an unstamped body; replaces a stale hash; is idempotent;
  rejects an unparseable body (exit 1, file unchanged on disk); rejects a filename/`sequence:`
  mismatch; leaves every sibling file byte-identical; exits 2 on a missing path. Probe test, kept
  only if green: stamping a *copy* of the committed genesis leaves it byte-identical — if the writer
  is not a fixed point there I drop the test and record why rather than change the emitter.
- `tests/fidelity.rs` — sequences asserted `1..2`; a new
  `the_second_migration_reworded_the_six_fail_conditions_intents` replays the real log, pinning each
  section's intent and its unchanged rule-id list, and asserts `render::section` for
  `brainstorm.sec.fail-conditions` carries the new line.
  `every_shipped_document_survives_the_log_field_by_field` is left alone and becomes the proof that
  snapshots and log agree. The byte test is re-scoped per §0.

## 7. README entry

One row under `tombstone-section` in the change-ops table — `reword-section` | `schema`, `id`,
`title?`, `intent?`, `note?` | at least one of the three; the section must be live; `note: ~` clears;
the section's rules are untouched. Plus a line under "Working on the log" for `mochiko-cli migrate
stamp <file>` as the authoring path that writes the required hash, and a pointer to it from "Writing
the hash". Nothing else changes; the grammar table and the anchor rule stand.

## 8. Verification

`cargo fmt --all --check` · `cargo clippy --all-targets -- -D warnings` · `cargo test --all` ·
`cargo audit --deny warnings` · `migrate validate --plugin-root plugins/mochiko --report`
(0 rejecting) · `migrate status --plugin-root plugins/mochiko` (sequences 1..2, new state hash
recorded beside the pre-wave one) · `rules brainstorm --section brainstorm.sec.fail-conditions
--plugin-root plugins/mochiko` · `git diff --stat plugins/mochiko/schemas/` (six files, one line
each). Nothing outside this seat's §1 file set is touched.
