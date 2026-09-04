# The migration log

This directory is the source of truth for mochiko's schema corpus — the command content schemas,
the skill content schemas, the family common libraries, the two label registries, the artifact
templates, and the shelf data files. Every schema change is a migration file committed here.
`mochiko-cli` validates each file against the grammar below and replays the whole log in memory
at each invocation; the current state is a projection, never an edited artifact.

Two consequences follow, and they are the point of the design:

- **A reword is a migration file, not an in-place edit.** The verbatim prior content is in the log
  by construction, so schema-content strips are redundant rather than reformatted.
- **The derived views are regenerated, never hand-edited.** A view that disagrees with the replay
  is a defect in the view.

The shipped snapshot files — `plugins/mochiko/schemas/*.yaml` and `plugins/mochiko/skills/*/schema.yaml` —
are transition-clause copies, kept semantically equal to the replay by the CI view ≡ replay test. A
migration that changes their content is mirrored into them by a hand edit of the same lines, never by
a regeneration: regenerating would drop the in-body comments, spacing, and fold width those files
carry, and the header comments are protected content under the strip ceremony. Wave 6 retires
them, and the clause with them.

## File shape

One file per migration, named `NNNN-<slug>.yaml`, ordered by the header's `sequence`.

```yaml
grammar: 1
id: 0002-widen-fail-set
sequence: 2
intent: One line stating what this migration does and why.
anchor: "2026-09-03 cli-schema-delivery D2"     # required in the cases below
hash: "sha256:<64 hex characters>"              # required
changes:
  - op: reword-rule
    schema: command/specify
    id: spec.register
    text: The reworded rule text.
```

| field | meaning |
|---|---|
| `grammar` | the log's grammar version. A binary declares the range it reads and halts loudly outside it, naming the upgrade command — never a best-effort partial read. |
| `id` | the file's own stem, `NNNN-<slug>`. |
| `sequence` | the migration's place in the log, as an integer. It must agree with the filename's numeric prefix. Gaps are legal; collisions are not. |
| `intent` | one line. Deliberately outside the hash, so it can be corrected without invalidating the file. |
| `anchor` | the ruling this migration executes, as `YYYY-MM-DD <session-slug>` with an optional trailing decision segment, written either `D2` or `[D2]`, a lettered sub-decision such as `D2a` accepted (the corpus carries two). |
| `hash` | the canonical hash of `{id, sequence, anchor, changes}`. Required, and it must match. |

**Every file in the log is a migration, and every migration is named `NNNN-<slug>.yaml`.** A
`.yaml` that is not so named is reported rather than skipped: a file called `genesis.yaml`, or
`O001-genesis.yaml` typed with a letter O, would otherwise replay as if it were not there.

### Writing the hash

The hash is required, so nothing can be written by hand alone. Write the migration without one and
stamp it in place:

```
mochiko-cli migrate stamp <file>
```

That is the authoring path every new migration takes. It rejects a body that is not a well-formed
migration rather than stamping it, and it rejects a filename whose numeric prefix disagrees with the
header's `sequence:`. The file is rewritten in the log's own layout, a leading comment block carried
through; nothing else on disk is touched. In the crate, `migration::with_hash(file, source)` returns
the same migration carrying its correct `hash:` header and replaces any stale hash already there,
and `migration::compute_hash(&migration)` returns the value on its own.

An optional hash would be no protection at all. The hash covers the `anchor:`, which is the
evidence that protected content left by ruling, so an editor who need not forge a hash would need
only to delete one line.

### Documents

A change names its document as `<kind>/<name>` — `command/specify`, `skill/review-feasibility`,
`skill-common/skill-review-common`, `template/spec`. A bare `<kind>` means the name equals the
kind, which is how the two singleton registries are written (`command-labels`). The kinds are
`command`, `skill`, `command-common`, `skill-common`, `command-labels`, `skill-labels`,
`template`, `shelf`.

### Change ops

Each change is independently citable: a rule's history is the set of ops naming its id.

| op | fields | notes |
|---|---|---|
| `import-document` | `kind`, `name`, `content` | How a document enters the log, once. Importing over an existing document is rejected. |
| `replace-document` | `kind`, `name`, `content` | Templates and shelf data only. Rule-bearing documents change one node at a time, so the log stays a per-rule history. |
| `mint-section` | `schema`, `section` | The section starts empty. A section value carrying `rules:` is rejected rather than having them dropped. |
| `reword-section` | `schema`, `id`, `title?`, `intent?`, `note?` | A section's prose. At least one of the three, or the change is rejected as rewording nothing. The section must be live — a tombstoned id says so rather than reading as absent. `note: ~` clears; a `title:` or an `intent:` is never cleared, because every section carries both. The section's id and its rules are untouched, so no ruling anchor is owed. |
| `tombstone-section` | `schema`, `id`, `disposition` | Rejected while the section still holds rules, so no rule is ever retired implicitly. |
| `mint-rule` | `schema`, `section`, `rule` | |
| `reword-rule` | `schema`, `id`, `text` | The id survives a reword. |
| `set-rule-field` | `schema`, `id`, `field`, `value` | `field` is one of `labels · class · kind · when · pointer · extends · enforces · anchor · note`. `value: ~` clears. An id is minted once and text has its own op, so neither is settable here. |
| `move-rule` | `schema`, `id`, `section` | The id survives a move. |
| `tombstone-rule` | `schema`, `id`, `disposition` | Never takes protected content — see below. |
| `supersede-rule` | `schema`, `id`, `disposition`, `anchor` | The only exit for protected content. |
| `set-var` | `schema`, `name`, `value` | `value: ~` clears. |
| `set-condition` | `schema`, `name`, `spec` | `spec: ~` clears. |
| `set-moment` | `schema`, `name`, `text` | Command schemas only; skills declare no moments. |
| `registry-add` | `registry`, `label`, `meaning` | Rejected when the label is already live. |
| `registry-retire` | `registry`, `label`, `note` | Moves the label into `retired`. Nothing deletes a label. |

An unrecognised op is rejected rather than skipped.

**Adding an op does not bump the grammar, while no binary is published.** `reword-section` was added
at wave 4 and the log stayed at grammar 1. No release of `mochiko-cli` exists yet, so there is no
deployed reader that could meet a file it cannot understand: the D5 range `1..1` is frozen at the
first publish with whatever ops the grammar carries by then, and the first published binary reads
every one of them. After that publish the calculus changes, because an older binary meeting a newer
op is a real situation — and it is already handled. That binary rejects the file loudly, naming the
install command, rather than skipping the op and replaying a state that is quietly missing a change.
That is the version contract working, not a gap in it, which is why a new op is additive here and a
grammar bump is reserved for a change that would make an existing file mean something different.

## The anchor rule

A migration MUST carry a ruling anchor whenever it supersedes or tombstones **protected
content**, which is any of:

- a rule of `class: floor`;
- a rule of `kind: fail`;
- any rule already carrying an `anchor:`.

Protected content leaves only through `supersede-rule` with a well-formed anchor. A bare
`tombstone-rule` on any of the three is rejected, and so is a section tombstone that would carry
rules out with it.

**Lowering protection is itself a protected exit.** Protection is read from a rule's own fields,
so a migration that changed `class:` away from `floor`, changed `kind:` away from `fail`, or
cleared an `anchor:` would leave an ordinary rule that the next op could retire freely. A
`set-rule-field` that does any of those three therefore requires the migration's own header
`anchor:`, exactly as `supersede-rule` requires one. Raising protection — promoting a rule to a
floor, giving it an anchor — needs no authority.

Corollary, by lead ruling at wave 1: protection is checked **per migration**. An anchored
migration may lower a rule's protection (floor → must, fail → another kind, anchor cleared);
once lowered, the rule is ordinary, and a later migration may tombstone it without an anchor.
The ruled exit is the anchored lowering itself, and the log records it there. A sticky
"once protected, always protected" set was considered and declined as stricter than the
record layer's rule (protected content leaves only by ruling — it did, at the lowering).

Together these make the record layer's protection mechanical for schema rules rather than
procedural: a floor cannot be dropped quietly, because the tool will not write the state in which
it has been.

Anchor format is `YYYY-MM-DD <session-slug>`, optionally followed by one decision segment written
either `D2` or `[D2]` — a lettered sub-decision such as `D2a` is accepted, the letters following at
least one digit (the corpus carries two such anchors) — and nothing after it. The month and day
are range-checked. The format is
checked here; resolving the anchor against a `DECISIONS.md` row is an advisory report.

## Sequence allocation

**Sequence numbers are assigned by the wave lead, in ranges, per wave. A seat never allocates its
own.** Two files claiming one sequence is a rejection, not a merge conflict to resolve later, so
the ranges are what keep concurrent seats from colliding in the first place.

| wave | range |
|---|---|
| 1 | 0001 (genesis) |
| later waves | assigned by the lead at wave open |

Gaps inside an allocated range are legal and expected — an abandoned migration leaves a hole, and
the hole is cheaper than renumbering.

## Working on the log

```
mochiko-cli migrate validate [--report]   # replay the log and print findings
mochiko-cli migrate status                # the state hash, the last sequence, the grammar version
mochiko-cli migrate stamp <file>          # write a migration's required hash header in place
```

`migrate validate` prints one finding per line as `code · schema · id · message`. A rejecting
finding means nothing may be rendered from the state; the advisory reports (`--report`) print
alongside and exit 0.

Rendering paths call `replay::load` (or `replay::load_full`, which also carries the log's grammar
version). Its `Ok` means both things at once: every op applied, and the finished state passes the
hard set. A state that is complete but invalid is refused just as firmly as a partial one.
