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

**No schema file ships.** The snapshot copies the plugin carried under the transition clause —
`plugins/mochiko/schemas/*.yaml` and `plugins/mochiko/skills/*/schema.yaml` — were deleted at
wave 6, and the clause expired with them. There is no file a run can read instead of asking the
binary, which is what makes the GI-020 dependency literal rather than a posture.

The human-readable projection of the log is the derived views at `.mochiko/schema-views/`, laid
out by document kind and committed to the repository. They live outside `plugins/`, so they are
never installed and no primitive can reach one. Regenerate them with:

```
mochiko-cli views emit --plugin-root plugins/mochiko --out .mochiko/schema-views
```

They are read, reviewed and diffed; they are never hand-edited. The CI view ≡ replay test compares
each emitted view against its committed file, so a view that disagrees with the log fails the
build, and the fix is always to regenerate.

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
| `grammar` | the log's grammar version. A binary declares the range it reads and halts loudly outside it, naming the upgrade command — never a best-effort partial read. Each file declares its own, and this binary reads `1..2`; grammar 2 adds the home fields in *Home documents* below. |
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
`template`, `shelf`, `home`.

`home` arrived at the 2026-09-13 `hook-enforced-artifact-schema` wave and is opaque like
`template` and `shelf`: it declares one artifact directory — its path as ordered segments, the
closed set of file names it admits, the template bound to each, and its size posture — and only
`import-document` and `replace-document` apply to it. Its fields are below.

### Home documents

A home document is one mapping. It is read with unknown keys ignored, which is why a field added to
it takes a grammar bump (*Grammar 2*, below). `mochiko-cli home <path>` prints what a path resolves
to under the current state.

| field | meaning |
|---|---|
| `home` · `title` | the home's name, as findings and `home` print it, and its one-line title. |
| `path` | the directory as ordered segments, each a literal or one of the eight tokens below. Where two homes match a path, the one with more literal segments wins. |
| `bounds` | `template` (each file's bound is its template's per-section budgets), `whole-file` (one `max_lines` per deliverable), or `elsewhere` (size is not checked here, and `bounds_cite`, then required, names the constraint home the bounds live in). |
| `deliverables` | the closed set of files the home admits. Each carries `file`, a name pattern over the same tokens, and how it is bounded: a `template` whose conformance block shapes it, a whole-file `max_lines`, a `bound_reason` saying why it has none (never together with `max_lines`), or a `form`. A deliverable with none of the four is rejected unless the home's `bounds` is `elsewhere`. |
| `form` | `log`: an append-only log, one `##` block per entry, bounded by `entry_max_lines` and never per file. `entries` (grammar 2): a cumulative store bounded per entry, below. |
| `subdirs` | the sub-directories the home admits. Each is governed by its own home document, or by nothing until one lands. |
| `reports` | the home's `reports/` directory: `envelope` names the template whose `report:` enum every report must hold, and `by_type` maps a type to its own template. Report file names are free. |
| `raw_output` | grammar 2: `true` on the one raw-output home, below. |

**The eight path tokens** are the binary's vocabulary, so a new token is a crate change: `<slug>`
(lower-case kebab) · `<date-slug>` (`YYYY-MM-DD-<slug>`, the date range-checked) · `<FEAT-ID>` and
`<EPIC-ID>` (the prefix and at least three digits, optionally `-<slug>`) · `<AX-ID>` (the same, the
slug required) · `<n>` (digits) · `<any>` · `<run-id>`, which is `<owner>-run<n>`, the owner a
`FEAT-` or `EPIC-` id or `lane-<slug>` (joint-build seam R5).

**`form: entries`** (grammar 2) bounds a cumulative store — a product baseline, the architecture
store — by entry, with no whole-file bound. The deliverable declares:

- `entry_heading`, `##` or `###`. An entry runs from its heading to the next heading at its level
  or above; a heading inside a fence is not a boundary.
- `entry_max_lines`, the per-entry bound. Required.
- `section_max_lines`, legal only with `###` entries: the bound on each `##` section's own text,
  from its `##` line to its first `###` entry — the summary tables and dated notes outside every
  entry. With `##` entries it is rejected, since every line after the first entry heading belongs
  to an entry.
- `entry_exempt_fields`, field names whose `**<Name>:**` lines — bare, or as a `- ` list item —
  an entry's count skips, so a lifecycle marker written at every run cannot push an entry over.

A bound `template` still grades the store's shape; the entry budgets replace its per-section ones.
The text above the first heading at the entry level or above is the store's **preamble**, counted
in full from the file's first line — frontmatter, fences and deeper headings included — and
bounded as one entry at `entry_max_lines` (census row P1). No home field declares it; `home`
prints it as "N lines above the first heading". The faults are keyed `size:entry:<heading text>`,
`size:section:<heading text>` and `size:preamble`, and the first-touch amnesty holds per key: an
entry already over its bound stays writable while a write does not worsen it, and a renamed entry
is measured as a new one. Where one heading repeats, its faults are compared largest first against
the standing faults of the same key.

**The raw-output home** (grammar 2) is the one home carrying `raw_output: true`, and a second is
rejected: the run folder, `.mochiko/runs/<run-id>/` (hook field review D4 as amended). Every path
under it, at any depth and under any name, resolves as raw output with shape and size unchecked,
except the names it declares as deliverables (the run log). The gate applies its own controls
there instead: the folder is always the main tree's, never a worktree's; the main tree's
`.gitignore` must carry the folder's literal prefix, `.mochiko/runs/`; and a `.md` whose
frontmatter declares a mochiko report type is refused. The folder's parent belongs to this home
alone: a path under `.mochiko/runs/` that no run folder holds resolves to no home, so the closed
world names the run key it missed (census row N3).

#### Grammar 2

Grammar 2 is grammar 1 plus five home fields, spelled as findings name them: `raw_output`,
`form: entries` (a value of a grammar-1 key, so listed with its value), `entry_heading`,
`section_max_lines` and `entry_exempt_fields`. A home field bumps the grammar where an op does not:
an older binary rejects an op it does not know, but it decodes a home with unknown keys ignored, so
it would, for one, read the run folder as an ordinary home — quietly. A migration whose header
says `grammar: 1` and that carries any of the five is therefore rejected at parse, naming the
field; the fix is to raise its header to 2. The grammar is per file, so the two coexist in one log
(0033 and 0034 are grammar 2). Each file's grammar must fall inside the binary's range, and the
log's grammar — what `migrate status` reports — is the highest any file declares, never the last
file's.

### Change ops

Each change is independently citable: a rule's history is the set of ops naming its id.

| op | fields | notes |
|---|---|---|
| `import-document` | `kind`, `name`, `content` | How a document enters the log, once. Importing over an existing document is rejected. |
| `replace-document` | `kind`, `name`, `content` | Templates and shelf data only. Rule-bearing documents change one node at a time, so the log stays a per-rule history. |
| `mint-section` | `schema`, `section` | The section starts empty. A section value carrying `rules:` is rejected rather than having them dropped. |
| `reword-section` | `schema`, `id`, `title?`, `intent?`, `note?` | A section's prose. At least one of the three, or the change is rejected as rewording nothing. The section must be live — a tombstoned id says so rather than reading as absent. `note: ~` clears; a `title:` or an `intent:` is never cleared, because every section carries both. The section's id and its rules are untouched, so no ruling anchor is owed. |
| `tombstone-section` | `schema`, `id`, `disposition` | Rejected while the section still holds rules, so no rule is ever retired implicitly. |
| `mint-rule` | `schema`, `section?`, `rule` | `section:` is required on a command or skill schema and rejected on a common library, which carries its blocks at the document's top level: there, omit it and the rule is appended as a block. |
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
deployed reader that could meet a file it cannot understand: the D5 range is frozen at the first
publish — `1..2` in this tree — with whatever ops its grammars carry by then, and the first
published binary reads every one of them. After that publish the calculus changes, because an older
binary meeting a newer op is a real situation — and it is already handled. That binary rejects the
file loudly, naming the install command, rather than skipping the op and replaying a state that is
quietly missing a change. That is the version contract working, not a gap in it, which is why a new
op is additive here and a grammar bump is reserved for a change an older binary would misread rather
than reject: one that would make an existing file mean something different, or a new home field,
which an older binary ignores (*Grammar 2*, above).
Widening an existing op is the same case: `mint-rule` gained an optional `section:` at the
author-grader-consolidation wave and the log stayed at grammar 1.

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
mochiko-cli migrate status                # the state hash, the sequences, the log's grammar
mochiko-cli migrate stamp <file>          # write a migration's required hash header in place
```

`migrate validate` prints one finding per line as `code · schema · id · message`. A rejecting
finding means nothing may be rendered from the state; the advisory reports (`--report`) print
alongside and exit 0.

Rendering paths call `replay::load` (or `replay::load_full`, which also carries the log's grammar,
the highest any applied file declares). Its `Ok` means both things at once: every op applied, and
the finished state passes the hard set. A state that is complete but invalid is refused just as
firmly as a partial one.
