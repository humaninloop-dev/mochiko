# Wave 1 — crate build plan (staff-engineer seat, awaiting lead approval)

**Ruling home:** `record.md` D1, D3, D4, D6, D10, D11 wave 1 (exit = the crate published, the user's
act), OQ4. **Floor:** `.claude/rules/mochiko/rust-cli.md`. **Done condition:** the four gates green ·
every §5 item present · **no file under `plugins/mochiko/` changed byte-wise** (wave 1 adds no
migration: conformance *data* is wave 3's, the *mechanism* wave 1's) · no `plugin.json` bump · the
§7 cost figure reported.

## 1. Grammar decision — no bump; the range freezes at wave 1's publish

`grammar` stays `1`, `GRAMMAR_RANGE` stays `(1, 1)`. Grounds: (a) `migrations/README.md` § Change
ops — a bump "is reserved for a change that would make an existing file mean something different",
and a new document kind plus two optional template keys change no existing file's meaning; (b) same
section — "while no binary is published … the D5 range `1..1` is frozen at the first publish with
whatever ops the grammar carries by then", and no `mochiko-cli-v*` tag exists (`git tag --list`
empty, `Cargo.toml` `publish = false`, `CHANGELOG.md:69` records the publish as owed), so wave 1's
own exit is that freeze and `home` is inside it; (c) an unknown `kind:` already halts loudly rather
than skipping — `src/migration.rs:619`, "`kind: {kind_token}` is not a document kind".

**Forward constraint owed to wave 3:** `schema::Section` ignores unknown keys by design
(`src/schema.rs` doc comment), so a *post-publish* migration adding a conformance key alone would
degrade silently on an older binary and must take a bump. Wave 3 is safe only because its
conformance keys ride the same file as the `home` `import-document` ops, which halt an older reader
first — **wave 3 must keep them in one migration**. **Scope consequence:** anything the wave-4
wrappers call must exist in the wave-1 binary or wave 4 forces a second publish, which is why the D9
sniff is built now (§3.8), not at wave 4.

## 2. Data shapes

### 2a. `home` — one document per home, `home/<name>`

Keyed per home, not one registry blob: `template/<name>` is the precedent, `views emit` writes
`homes/<name>.yaml`, wave 3's table stays diffable per home. Carried **opaque** like
`template`/`shelf` (`model.rs:768` returns `Document::Opaque` for any kind neither rule-bearing nor
a registry), decoded at point of use as `render::template_of` does — so `model.rs` gains no decode
path.

```yaml
kind: home
name: feature
content:
  home: feature
  title: Feature work home
  path: [".mochiko", "features", "<FEAT-ID>"]   # ordered segments, §2b
  bounds: template                              # template | whole-file | elsewhere
  bounds_cite: ~                                # required iff bounds == elsewhere (D4f/V2)
  deliverables:
    - {file: tasks.md, template: tasks, max_lines: ~}   # max_lines required iff template is ~
    - {file: implement-log.md, form: log, entry_max_lines: 60}   # per entry, never per file (D4d)
  subdirs: ["stories", "prototype"]             # each carries its own home doc
  reports: {envelope: report-envelope, by_type: {}}   # ~ = no reports/ dir; by_type = D2/M8 budgets
```

### 2b. Path patterns — a closed segment-token vocabulary in the binary

A segment is a literal or one of seven tokens the binary knows: `<slug>` · `<date-slug>`
(`YYYY-MM-DD-<slug>`) · `<FEAT-ID>` (`FEAT-` + ≥3 digits + optional `-<slug>`) · `<EPIC-ID>` ·
`<AX-ID>` · `<n>` (digits) · `<any>`. **`deliverables[].file` takes the same tokens**, because F9's
homes name files by pattern as well as literally — `.mochiko/decisions/<date-slug>.md`,
`stories/US-<n>.md`, `concerns/<AX-ID>.md` — and a literal-only file set could not declare them.
Stdlib `str` segment matching covers every one. **Adopt-first:** `regex` and
`globset` are the shelf candidates, both **declined** — a log-side regex could express patterns a
deny reason cannot explain back to a seat, and neither earns a new `cargo audit` surface; a seventh
token is a crate change, the same friction D2 wants for a new deliverable kind. Resolution: most
**literal** segments matched wins, and `validate` rejects two homes declaring identical patterns, so
ambiguity is a log defect rather than a runtime coin-flip.

### 2c. The conformance block on `template` — two section keys plus one template key

**Reconnaissance that changed this design** (all eight templates rendered against
`--log-dir plugins/mochiko/migrations`): `sections[].name` is **not** reliably the artifact's `##`
heading grammar, so it cannot be reused as the heading list. Only `spec` maps exactly;
`governance-intent` and `feature-entry` map 1:1 but for a leading meta section (`## Header`,
`## Header & Status`) that is no heading at all; `codebase-analysis` declares
`## Part 1 — Inventory (Factual)` and skeletons `## Part 1: Inventory (Factual)`;
`architecture-store`'s sections name *files* (`## Store layout`, `## Concern ledger (AX rows)`);
`features-index` and `governance-surfaces` skeleton no `##` at all; `tasks` skeletons two of four.
So the extension stays on `sections` as briefed, but each section declares its heading rather than
implying it. Full per-template detail: `wave1-template-readiness.md`.

```yaml
sections:
  - {name: Overview, heading: "Overview", max_lines: 5, required: true}  # heading/max_lines NEW
conformance:               # NEW · template-level · absent = no shape check at all (D4f)
  frontmatter: {required: [feature, status], enum: {status: [draft, accepted, superseded]}}
  placeholders: ["[entity]", "<feature-id>", "FEAT-XXX"]
  extra_headings: deny     # deny (D4a default) | allow
```

An absent `heading` means the section governs none; an absent `max_lines` is OQ1's no-budget
disclosure. Required-heading order = declared order of the sections carrying a `heading`. Rendered
in the **producer view** as one `## Conformance` block before `## Skeleton`; `check_view` unchanged.

## 3. `check --hook-json -` decision algorithm

1. Parse stdin with `serde` (`hook_event_name`, `tool_name`, `cwd`,
   `tool_input.{file_path, content, old_string, new_string, command}`). Unparsable → **exit 2**.
   Wave 0 confirmed `Read` carries the same `tool_input.file_path` key, so one struct covers every
   tool; `Read` never reaches `check` (D1b registers no Read hook) and takes no branch.
2. `replay::load_full`. Absent/unsound → 1; grammar skew → 3. Both silent.
3. `Bash`/`PowerShell`: scan `command` for a write operator (`>`, `>>`, `tee`, `sed -i`, `cp`/`mv`
   destination, `<<`/`<<-` heredoc) whose target resolves under a declared home. Hit → **4** with
   the D1c reason, best-effort stated in the text. Miss → 0 with an explicit allow decision.
4. `Write`/`Edit`: relativize `file_path` against `cwd`; outside `cwd` → 0 with an explicit allow.
5. Resolve the repo-relative path to a home (§2b). No home → step 8.
6. Candidate content: `Write` → `content`; `Edit` → read the on-disk file and apply
   `old_string` → `new_string` in memory. No match → 0 with an explicit allow: the platform rejects
   that edit itself, and a second denial from the gate would only confuse the seat. (Built this way;
   this plan's draft said exit 2.)
7. Six checks in order, the first failure carrying the reason: **path** (segment pattern) · **file
   set** (name ∈ `deliverables`, or any name under a declared `reports/`) · **frontmatter**
   (required present, enum values listed) · **headings** (required present in declared order; an
   undeclared `##` denied under `extra_headings: deny`) · **placeholders** (declared tokens absent —
   frontmatter *values* and `##`/`###` heading text only, exact spelling, never a body substring) ·
   **size** (each heading's span from its `##` line to the next `##`, nested content included,
   against `max_lines`; `form: log` measures one entry; `bounds: elsewhere` skips size and cites
   `bounds_cite`). **First-touch amnesty:** where the file exists, every failing measure re-runs
   against the on-disk baseline, and one already failing and not worsened is **allowed** with the
   standing overage as `additionalContext`; a new file at a declared name takes its budget outright.
   Deny → **4**; clean → 0.
8. No home: sniff the candidate's frontmatter for a `report:` value in the union of every home's
   `reports.envelope` enum. Hit → **4** (a report smuggled outside a home). Miss → 0 with an
   explicit allow: a plain `.md` elsewhere is never mochiko's business (D9).

Stdout is the decision JSON the wrapper prints verbatim (`hookSpecificOutput.{hookEventName,
permissionDecision, permissionDecisionReason, additionalContext}`). **Never empty on an exit-0 or
exit-4 path** — the wave-1 open supersedes this plan's earlier "empty stdout" wording in steps 3 and
8, because the platform denies a background subagent's call when no hook returns a decision. Only
exit 1, 2 and 3 print nothing, and the wave-4 wrapper supplies their allow. Every deny reason closes
with D9's advisory halt sentence.

## 4. Exit-code mapping

| code | meaning | stdout | wrapper | write |
|---|---|---|---|---|
| 0 | conforming, amnesty-allowed, no home, or no verdict owed | **explicit `permissionDecision: allow`, always** | print it verbatim | proceeds |
| 1 | log absent, empty, unsound | empty | prints its own explicit allow | proceeds |
| 2 | usage error, unparsable payload, binary predates `check` | empty | prints its own explicit allow | proceeds |
| 3 | grammar range | empty | prints its own explicit allow | proceeds |
| **4** | **conformance deny — minted** | the deny JSON | print it verbatim | **denied** |

Minted, not reused: reusing 1 would make an unsound log indistinguishable from a non-conforming
artifact, exactly the pair D3 separates. **Wave-0 fold (explicit allow):** the platform denies a
background subagent's call when no hook returns a decision, so `check` never prints empty stdout on
a non-deny outcome — every exit-0 path emits
`{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"allow"}}`, carrying
`additionalContext` under amnesty. Codes 1/2/3 emit nothing and the wave-4 wrapper supplies the
explicit allow for them, so a binary that cannot read its log never denies a write.

## 5. Module touch list

New: `src/home.rs` (the `Home` model, `PathPattern` tokens, `resolve(path)`, `home_of`) ·
`src/conform.rs` (the six checks, the heading-span counter, amnesty, `Edit`-in-memory) ·
`src/hook.rs` (payload structs, the shell operator parse, decision-JSON emit). Modified:
`src/model.rs` (`DocKind::Home`, `ALL` 8→9, `is_replaceable`) · `src/schema.rs`
(`Section::{heading, max_lines}`, `Template::conformance`, the producer-view block) · `src/cli.rs`
(`Command::{Check, Home}`, `run_check`/`run_home`, exit 4 in the module doc's contract) ·
`src/validate.rs` (`Code::{HomeShape, HomePattern, HomeDuplicate, HomeBinding, HomeBounds}` + checks;
built with the `HomeShape` code this draft omitted) ·
`src/views.rs` (`view_path` arm `Home => "homes"`) · `src/lib.rs` (three `pub mod`). Tests: new
`home.rs`, `conform.rs`, `hook.rs`; `cli.rs` and `validate.rs` extended (this draft also named
`render.rs` and `views.rs`; their tests needed no change, and the `home` render goldens landed in
`tests/cli.rs`); `tests/fixtures/home-log/` — **the shared fixture log the plugin contract suite reads
rather than carrying one of its own** (wave-4 contract plan), covering four branches so no contract
row waits on wave 3's census: a templated deliverable with per-section `max_lines`; a template-less
deliverable with a whole-file bound; a `reports/` dir whose envelope enumerates report types; and a
`bounds: elsewhere` kind with its `bounds_cite`.

## 6. Test matrix (rows × expected)

| class | rows | expected |
|---|---|---|
| path · set | in-pattern · wrong depth · wrong literal · malformed `<FEAT-ID>` (`FEAT-1`, `B53`) · `<date-slug>` valid/invalid · two-home longest-literal race · declared literal name · patterned name (`<date-slug>.md`, `US-<n>.md`) matching/not · undeclared name · any name under `reports/` · undeclared sub-dir | allow · deny ×5 · the specific home · allow · allow/deny · deny · allow · deny |
| frontmatter · placeholders | all required · one missing · enum listed · enum unlisted · none at all · token in a frontmatter value · in a `##` heading · in body prose (`FEAT-XXX` named honestly) · absent | allow · deny · allow · deny · deny · deny · deny · **allow** · allow |
| headings | exact order · required missing · order swapped · extra `##` under deny · under allow · optional absent · `###` nested | allow · deny · deny · deny · allow · allow · allow |
| size · amnesty | within · over by one · nested `###` counted in · `form: log` entry within/over · `bounds: elsewhere` · new file over budget · existing over baseline unchanged · worsened · improved · `Write` and `Edit` legs of each | allow · deny · deny · allow/deny · allow · deny · allow + `additionalContext` · deny · allow · same both legs |
| Edit-in-memory | append adds an undeclared `##` · append crosses a budget · `old_string` absent | deny · deny · **explicit allow, exit 0** (built this way; the draft said exit 2 — the platform rejects that edit itself and a second denial would only confuse the seat) |
| shell · sniff | `>` · `>>` · `tee` · `sed -i` · heredoc · `cp`/`mv` into a home · `mv` out of one · `git status` · `>` outside every home · **the wave-0 probe line `printf 'retry probe' > "<abs>/probe-home/c.md"`, which the wrapper's grep/sed `field()` false-allowed** · enum `report:` written to `docs/` · unlisted `report:` · plain `.md` | deny ×6 · deny · 0 · 0 · **deny** · deny · allow · allow |
| exit codes | conforming · unsound log · malformed payload · `grammar: 99` · deny | 0 · 1 · 2 · 3 · 4 |
| explicit allow | every exit-0 path: conforming · amnesty · no home · shell miss · sniff miss | stdout carries `permissionDecision: allow`, never empty |
| replay · validate | log replayed twice · `home` content hash · `replace-document` on a home · unknown token · duplicate pattern · `template:` naming no live template · `elsewhere` without `bounds_cite` · template-less deliverable without `max_lines` | identical hash · stable · applies · one rejecting finding each |
| goldens | `home <path>` · producer-view `## Conformance` · `check_view` unchanged · a fixture producing-primitive section carrying the authoring-time rule (D10/V1; wave 3 mints the real one) | byte-exact |
| shared fixture | the on-disk log replays clean · each of the four contract branches present · each branch can both allow and deny · no non-deny outcome renders empty stdout | green, and a dropped branch fails |

## 7. Cost

Report `check --hook-json -` on a conforming `Write` against the real log
(`plugins/mochiko/migrations`, 4 files): wall-clock median of 20 runs, measured beside `rules` on
the same box so it is comparable to the record's 35 ms. **Wave-0 bar (folded):** wave 0 measured
49 ms per call for wrapper plus binary and set one cap — **≤ 60 s aggregate per run**, with the
cache trigger at a **> 100 ms median**. So the median is reported against 100 ms and no cache is
built this wave either way. **Cache seam, designed not built:** every path funnels through one
`load_for_delivery(dir, err)` in `cli.rs`, so a `${CLAUDE_PLUGIN_DATA}` cache slots there alone —
key = the existing `State::content_hash`, value = the encoded state — behind a runtime env check,
no new call site and no trait. Built only on the lead's word.

## 8. Task decomposition (TDD order) — 9 tasks

1. `DocKind::Home` + `is_replaceable` + `view_path` arm + replay/views tests — **sonnet**
   (compiler-guided, the existing suites are the oracle).
2. `src/home.rs` model, pattern tokens, resolution — the seat writes the semantics tests (§2b is
   judgment), **sonnet** greens them.
3. `validate.rs` hard set over `home` docs, four codes — the seat fixes the finding texts,
   **sonnet** implements.
4. `schema.rs` keys + the producer-view block + its golden — **seat**; the rendered shape is a
   design artifact.
5. `src/conform.rs`, the six checks — the seat authors the matrix rows, **sonnet** implements check
   by check.
6. Amnesty + `Edit`-in-memory on top of 5 — **seat**; the non-worsening comparison is the subtle leg.
7. `src/hook.rs` — the seat owns the write-operator table, **sonnet** implements the parser.
8. `cli.rs` `check`/`home`, exit 4, one `tests/cli.rs` case per code, the D9 sniff — **sonnet** once
   the seat has fixed the output shapes.
9. Fixture log, the four goldens, the cost measurement — **seat**; fixture data and goldens are design.

Every worker return is read back against the failing test the seat wrote before it counts, and each
dispatch is disclosed in the cycle report (`patterns-model-tiering`).

## 9. Plan-minimalism ladder — rung per element

- `home` kind — **required** (D3): a field on `template` fails, since D4f homes exist with no template.
- One doc per home — **simpler shape**: a registry blob diffs unreadably and `replace-document` rewrites everything.
- No grammar bump — **already exists**: the version contract's loud unknown-kind rejection is the mechanism.
- Heading list from `sections[].name` — **failed "already exists"**: 5 of 8 templates mismatch (§2c), so two optional keys instead.
- Segment tokens in the binary — **minimum now**: seven cover every home and file name in F9/F13; an eighth is a crate change.
- `regex` / `globset` — **cut at the pre-code ladder's installed-dep rung**: adopt-first, both named and declined (§2b).
- Exit code 4 — **required** (D3 asks mint-or-document): reuse of 1 conflates two states D3 separates.
- `home <path>` — **required** (D1a second arm).
- A plain `check <file>` form — **cut, minimum now**: `home <path>` covers the human read, `check` stays hook-shaped.
- Replay cache — **cut, minimum now**: seam named in §7, built on measured need only.
- A `report` document kind — **cut, already exists**: the envelope is a `template` with a `conformance` block.
- D9 sniff — **kept, required now**: wave 1's exit is the publish, and a wave-4 need forces a second one (§1).

## 10. Open question for the lead

**§2c's finding moves wave 3's surface, not only wave 1's.** Five of the eight shipped templates
cannot carry a heading-level conformance block as they stand: `codebase-analysis` needs `heading:`
overrides, `architecture-store`'s sections describe files rather than headings, and
`features-index`, `governance-surfaces`, and `tasks` skeleton no complete heading set. Wave 1 builds
and proves the mechanism on fixtures either way. **Does wave 1 also hand wave 3 a per-template
readiness table, or is that wave 3's own census discovery?**
