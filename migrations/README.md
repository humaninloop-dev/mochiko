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

## File shape

One file per migration, named `NNNN-<slug>.yaml`, ordered by the header's `sequence`.

```yaml
grammar: 1
id: 0002-widen-fail-set
sequence: 2
intent: One line stating what this migration does and why.
anchor: "2026-09-03 cli-schema-delivery [D2]"   # required in the cases below
hash: "sha256:<64 hex characters>"              # optional, and binding once written
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
| `anchor` | the ruling this migration executes, as `YYYY-MM-DD <session-slug>` with an optional trailing `[D#]`. |
| `hash` | the canonical hash of `{id, sequence, anchor, changes}`. A file that records one must match it. |

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
| `mint-section` | `schema`, `section` | The section starts empty. |
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

## The anchor rule

A migration MUST carry a ruling anchor whenever it supersedes or tombstones **protected
content**, which is any of:

- a rule of `class: floor`;
- a rule of `kind: fail`;
- any rule already carrying an `anchor:`.

Protected content leaves only through `supersede-rule` with a well-formed anchor. A bare
`tombstone-rule` on any of the three is rejected, and so is a section tombstone that would carry
rules out with it. This is what makes the record layer's protection mechanical for schema rules
rather than procedural: a floor cannot be dropped quietly, because the tool will not write the
state in which it has been.

Anchor format is `YYYY-MM-DD <session-slug>`, optionally followed by ` [D#]`. The format is
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
```

`migrate validate` prints one finding per line as `code · schema · id · message`. A rejecting
finding means the state in memory is partial and nothing may be rendered from it; the advisory
reports (`--report`) print alongside and exit 0.
