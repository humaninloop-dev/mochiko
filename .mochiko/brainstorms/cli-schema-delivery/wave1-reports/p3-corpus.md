# Wave 1 — seat P3 (corpus) cycle report

**Seat:** P3 — genesis import and fidelity fixture, the derived-view emitter, the three ported
probe matrices, the similarity scorer, release machinery, the CI filter, the contract-suite
skeleton.
**Plan:** `p3-corpus-plan.md`, lead-approved with all eight open questions ruled (j1 port all 114 ·
j2 single genesis file · j3 `similar` wired into `--report` · j4 `--out` required · j5 lift the
comments generically · j6 carry the header anchor · j7 `cross` for arm64 · j8 views mirror the
repo path).
**Substrate:** P1 at `cd5a333`, P2 at `07a39b4`.
**Shipped files:** no file under `plugins/` changed byte-wise.

**Gates at close (fix round 1):** `cargo fmt --all --check` clean · `cargo clippy --all-targets
-- -D warnings` clean · `cargo audit --deny warnings` clean · `cargo test --all --no-fail-fast`
**242 passed, 0 failed** across eleven test binaries, in 35 seconds rather than 137 (§12, A8).
`migrate validate --log-dir migrations` reports `0 rejecting · 92 advisory` from any working
directory, and `views emit` writes all 50 views from the log. Nothing in the seat is open.

---

## 1. What was built

| file | lines | what it carries |
|---|---|---|
| `migrations/0001-genesis.yaml` | 11,701 | the generated genesis migration — 50 `import-document` ops, 618,122 bytes |
| `crates/mochiko-cli/src/genesis.rs` | 607 | the generator: the corpus scan, the anchor fold, the comment lift, the stamped file |
| `crates/mochiko-cli/src/views.rs` | 456 | the derived-view emitter and the YAML writer both it and genesis use |
| `crates/mochiko-cli/src/similar.rs` | 975 | the similar-rule detector, including a faithful `difflib.SequenceMatcher` |
| `crates/mochiko-cli/tests/fidelity.rs` | 427 | 10 tests — regeneration, field-by-field fidelity, the two deltas, the census |
| `crates/mochiko-cli/tests/views.rs` | 309 | 8 tests — the writer, semantic equality over all 50, the header, the out-dir fence |
| `crates/mochiko-cli/tests/matrix_command.rs` | 919 | the 134-probe command matrix and its four ledgers |
| `crates/mochiko-cli/tests/matrix_skill.rs` | 822 | the 114-probe skill matrix and its four ledgers |
| `crates/mochiko-cli/tests/matrix_similar.rs` | 952 | the 48-probe detector matrix, the difflib vectors, the corpus pin |
| `crates/mochiko-cli/tests/matrix/mod.rs` | 440 | the shared matrix harness (a disclosed addition — §5.1) |
| `crates/mochiko-cli/tests/anchor_grammar.rs` | 117 | 5 tests pinning the decision-segment grammar against the live corpus (§6) |
| `evals/contract/run.py` | 371 | the contract-suite runner |
| `evals/contract/README.md`, `fixture/probe-plugin/` | — | the gate split, the prerequisites, the one-`!`-line probe command |
| `.github/workflows/release.yml` | 104 | four targets, stripped and checksummed; `publish` present and disabled |
| `.github/workflows/ci.yml` | +2 | the path filter gains `migrations/**` and `evals/contract/**` |

`src/lib.rs` gained three `pub mod` lines. `src/cli.rs` gained the three granted arms and nothing
else. `src/model.rs` gained the one lead-granted delta of §6 — one expression and its doc comment,
nothing more. `Cargo.toml` is untouched: **no new dependency.**

### Genesis

The generator reads the 20 files under `plugins/mochiko/schemas/` and the 30 in-directory skill
schemas, decodes each through `Document::from_value`, and writes one `import-document` op per
document in address order. Three properties are asserted rather than assumed: it regenerates
byte-identically, it round-trips losslessly, and it fails loudly.

**The anchor fold.** All 597 sidecar entries land as `anchor:` fields on their rules. The sidecar
file is never written — `the_sidecar_file_is_never_written` builds genesis and compares the file's
bytes before and after, because the Python checkers stay authoritative on it until they retire.
An anchor naming no live rule, or an id two documents both carry, stops the build.

**The comment lift (j5).** Rather than hard-coding the two rules P1 named, the generator scans for
a `# D6 empty-with-reason:` comment block directly above any `enforces: []` and lifts whatever is
there. A reworded comment carries; a third occurrence is lifted rather than missed; an empty
mirror with no such comment is an error. The two ids are still asserted by name, and two further
tests pin the attribution rule: a comment separated from its mirror by any other line is never
claimed by it.

**Errors, not findings.** Genesis fails with `GenesisError`, not `validate::Finding`. The finding
vocabulary is the validator's, closed by P1's own coverage guard, and adding a variant would both
edit P1's file and break that guard. A generator that cannot read its inputs has not produced a
state to have findings about.

### The views

`views emit --out <dir>` writes every document back in the corpus's file shapes, under the
repository-mirroring path (j8), with no default output directory (j4). The writer is hand-rolled
because D6 makes readable text a constraint: a generic serializer would satisfy the parser and
destroy the surface the constraint is about. It picks literal blocks for text carrying newlines,
folded blocks for long single lines, and inline sequences and `when:` mappings exactly where the
corpus writes them.

**Semantic equality over all 50 passed on the first run.** The command runtime-kernel header is
regenerated exactly — measured first: all six shipped command headers are one template differing
only in the command's name.

### The detector

`similar.rs` ports the scorer, the buckets, the bonus and its cap, the short-text guard, the
same-block skip, the allowlist with its stale-id warnings, union-find clustering and the
classification tags. The ratio is CPython's Ratcliff/Obershelp with **autojunk**, ported rather
than approximated. It is wired into `migrate validate --report` (j3), advisory throughout: it
cannot move an exit code.

## 2. The red/green/refactor trail

Five cycles, each opened with a failing test.

| cycle | red | green | refactor |
|---|---|---|---|
| 1 — views | `tests/views.rs` would not compile: no `views`, no `genesis` | the corpus scan and the writer | — |
| 2 — genesis | `tests/fidelity.rs`, 7 of 10 failing on the absent file | the generator, then the committed file | — |
| 3 — similar | `tests/matrix_similar.rs` would not compile | the detector | the ratio core, below |
| 4 — matrices | 4 of 67 command probes failing | expectations corrected against what the validator says | the exact-accounting harness, below |
| 5 — machinery | the contract runner reporting no cases | the workflows, the fixture, the runner | — |

Three moments were worth more than the tests that passed.

**Cycle 3 — the suite took eight minutes.** The corpus parity test scores 146,572 pairs, and the
first implementation allocated two hash maps per row of `a`, which is what CPython does. Hoisting
the rows into two reusable arrays with the touched slots tracked for clearing took the file from
**475 s to 99 s** with the reference vectors and all four corpus figures unchanged — which is
what makes it a refactor rather than a rewrite.

**Cycle 4 — the accounting was wrong, and said so.** My first ledger summed to 136 of 134, because
two probes were claimed twice and two were claimed nowhere. Counting by hand is how that happens,
so the ledgers now name each Python probe verbatim and a test does the set algebra: every probe is
claimed exactly once, and a name that is not a Python probe fails just as loudly as one that is
missing. That guard found three more misfilings immediately.

**Cycle 4 — the fixtures were family-blind.** Two authoring-family probes failed because the Rust
detector resolves `extends:` through the validator's family-aware resolver, while the Python has no
family concept at all. The Python's fixture names (`zeta-producer`) cannot bind an
`authoring-common` block under that resolver. Renaming the fixtures to family-correct names
(`authoring-zeta`) keeps every assertion and respects the resolver — disclosed at §5.2.

## 3. The matrix port

**The census the record carries is wrong for the skill matrix.** `record.md` D6/D8 and the wave
plan say 86; executing the file's own `probes()` returns **114**. The figure grew with the
authoring-family and patterns-family waves. Ported at the lead's j1 ruling; `the_recorded_census_of_this_matrix_is_stale` pins the real number.

| matrix | probes | ported | genesis-side | not applicable | outside the hard set |
|---|---|---|---|---|---|
| command | 134 | 66 (+3 beyond the matrix) | 8 | 14 | 46 |
| skill | 114 | 63 (+1 beyond) | 7 | 9 | 35 |
| similar | 48 | 42 | — | 6 | — |

Each matrix's ledgers are asserted to account for its Python list exactly, by name.

The similar row read `45 / 3` until fix round 1. It was a hand count, and it was wrong by three:
the three `e2e: exit 0 by default` siblings had no Rust referent and appeared in no ledger. The
row above is now a set equation the test runner checks. §12/B1 has the detail.

### Not applicable under D6 — the `.md` scaffold and the sidecar

Command: 13 `.md` scaffold probes (headings, heading order, Rules-block enumeration, prose token
resolution, Not-done placement, the retired label in prose, the missing file) plus the
DECISIONS.md anchor resolution, which wave-plan §3 defers. Skill: 8 scaffold probes plus the same
resolution probe. The count pins port as **computed-count** assertions: 2 on the command side, 2
on the skill side, because the desync class they guard is exactly what a computed pin removes.

### Outside the hard set — the honest cost of the retirement

**This is the finding of the port**, and it is larger than the plan predicted: **46 command and 35
skill probes exercise a Python check the Rust hard set does not carry.** They are not lost
because they moved — they are lost if the scripts retire as-is. Four families:

1. **Shape errors the decoder rejects** (`when:` as a list, `conditions:` not a mapping, a rule
   that is not a mapping, and so on). These are genuinely covered — the decoder refuses them
   before the validator runs — but the coverage is a decode error, not a finding, so a probe
   asserting a finding has nowhere to land. Roughly a third of each list.
2. **Checks D6 did not carry into the hard set**: in-text ID citation resolution (8 command, 6
   skill probes), pointer file resolution (5 skill), the retired-selector prose lint, the
   pointless-override and orphan-block warnings, the absence-meaningful-field guard on library
   blocks, the zero-member label warning, the label-less rule check, the `{{...}}` sigil warning,
   the inline `ruling:` field, and the flat top-level `rules:` guard.
3. **Sweep-mode claims** (7 skill probes) — the Rust validator has no per-skill mode; it always
   grades the whole state, so "a single-skill run makes no orphan claim" has no referent.
4. **Report wording** — the coverage report's floor carve reads differently.

Family 1 is safe. **Families 2 to 4 are a real narrowing of what is checked**, and they are the
wave's to rule on before the scripts are deleted at wave 6. I have not smoothed that over.

## 4. The difflib parity claim, measured

The 48 probes are written against `difflib.SequenceMatcher`'s exact numbers, so the port
reproduces the algorithm rather than approximating it.

| measure | result |
|---|---|
| corpus pairs compared against Python during planning | 18,577 |
| `ratio()` mismatches | 0 |
| `text_sim()` mismatches | 0 |
| worst absolute delta | 0.0 |
| long-`b` pairs where autojunk changes the ratio | 962 of 2,000 |

Autojunk is load-bearing, not cosmetic, so it is ported. Two pins keep it that way: 17 reference
vectors captured from CPython (asserted to 1e-12, including the witness where a port without
autojunk would return about 0.95 instead of 0.067), and the whole-corpus figures the live detector
reports — **1,016 rules scanned · 146,572 in-kind pairs scored · 0 clusters · 181
allowlist-suppressed edges**, reproduced exactly.

## 5. Deviations from the plan, each named

1. **A shared test module, `tests/matrix/mod.rs`.** Not in the pen list. The two matrices need one
   fixture harness; duplicating 440 lines would be worse. Additive, test-only, used by my files
   alone.
2. **The authoring fixtures are renamed** to family-correct names (§2). The Python's names cannot
   resolve under a family-aware resolver. Every assertion is unchanged.
3. **Four probes are ported under a different code than the Python's message implies**, because
   the Rust reports the same defect through a different limb: a non-prefixed rule id reports
   `id-prefix` rather than `id-format` (the prefix limb returns first), a section-less schema
   reports `id-prefix` (the prefix cannot derive), a foreign-stem section id reports `id-format`,
   and an empty rule id reports `id-prefix`. Each is filed with `Probe::porting`, which records the
   Python name beside the Rust one.
4. **Four probes are beyond the Python matrix** (three command, one skill), filed with
   `Probe::extra` so they never inflate the port's coverage claim.
5. **The sidecar's absence is an error, not a warning.** The Python checker warns; genesis cannot
   fold anchors without it. A severity change, disclosed in the genesis-side ledger.
6. **`views.rs` also serves genesis.** One YAML writer, so the log is as readable as the views.

## 6. The cross-seat defect: found, routed, granted, applied

**The defect.** `model::is_decision_segment` accepted `D` plus digits only. The shipped sidecar
writes `D2a` twice, on `authoring-feature-map.selectability-specify-only` and
`.story-trace-provenance`. The Python's `RULING_RE` accepts it (`D\d+.*`), and both checkers pass
on the tree today, so this was P1's A3 tightening overshooting the corpus it ports.

The consequence was total: `anchor-format` is rejecting, so the genesis log was not deliverable
and nothing rendered from it. `migrate validate` reported exactly `2 rejecting · 92 advisory`, and
those two findings were the whole of it — every other hard-set check passed over all 50 documents.
Six tests waited on it: the five in `tests/fidelity.rs` that call `replay::load`, and P2's
`the_shipped_log_renders_every_section_of_every_primitive`, which P2 wrote to activate the moment
genesis landed. It activated, and it failed only here.

**The refusal, stated for the record.** *I did not normalise the anchor in genesis. Rewriting a
provenance anchor to fit a validator is the silent corruption the record layer exists to prevent.*
The seat stopped and routed the exact patch to the lead rather than editing another seat's file or
editing the corpus.

**The delta, as granted.** P1 file, one function, lead-granted, corpus-driven (`RULING_RE`
parity). The lead granted it to this seat because the crate's pen is held by one writer at a time
and P1 is closed. Scope, verbatim from the grant: `model::is_decision_segment` only — after `D`
and one or more digits, allow a trailing run of ASCII lowercase letters. The change is one match
arm:

```rust
Some(rest) => {
    let digits = rest.trim_end_matches(|c: char| c.is_ascii_lowercase());
    !digits.is_empty() && digits.chars().all(|c| c.is_ascii_digit())
}
```

plus a four-line doc paragraph saying why. The whole delta is `9 insertions(+), 1 deletion(-)`
in one function; nothing else in `model.rs` was touched, and the grammar stays anchored at both
ends — the suffix is letters only and must follow at least one digit.

**The pin.** `tests/anchor_grammar.rs`, a file this seat owns, holds five tests: the accepted
spellings (`D2a`, `[D2a]`, `D12abc`); the nine that stay malformed (`D`, `Da`, `D2 D3`, trailing
prose, `D2A`, `D2-a`, `D2a1`, `D2a3`, `2a`); the pre-existing forms unchanged (`D2`, `D4`, `[D2]`,
no segment, a missing date, an out-of-range month); the two live anchors asserted **by rule id**
and read from the sidecar rather than restated, so the test fails if the corpus stops carrying the
spelling that forced the widening; and a sweep asserting no anchor anywhere in the sidecar is
malformed. Every case the grant names is present by that spelling.

**Where the delta lives.** On branch `worktree-cli-context`, uncommitted, in the worktree at
`.claude/worktrees/cli-context`. The shared checkout is on `main` at `1ed5c19`, whose crate is
still the v0.76.0 seed — `src/schema.rs`, and no `model.rs` at all. All of wave 1 lives on the
worktree branch, so a read of the shared checkout shows none of it. Worth stating because it was
misread once.

**One doc line is now stale, and it is outside this seat's pen.** `migrations/README.md` says the
decision segment is "written either `D2` or `[D2]`" in two places — the `anchor` row of the field
table and the "Anchor format is …" paragraph of *The anchor rule*. My pen on that file is the
sequence-table row only, so I did not edit it. Named delta for the lead: add the lettered spelling
to both sentences.

## 7. Pre-code ladder disclosures

| rung | not built | why |
|---|---|---|
| exist at all | SQLite, a cache, an index | D1 defers all three to a measured need |
| exist at all | `.md` scaffold checks | dead under D6 |
| exist at all | DECISIONS.md anchor resolution | wave-plan §3 scopes wave 1 to format |
| exist at all | merge logic in `similar.rs` | combining is judgment (GI-019) |
| exist at all | a `--json` or `--exit-signal` surface | not in the advisory set the wave plan names |
| in codebase | a second decoder, resolver or hash | reused `from_value`/`to_value`, `canonical_hash`, `resolve_extends`, `derive_prefix`, `Family::of`, `with_hash`, `load_full`, `Finding`/`Code` |
| in codebase | a second YAML writer | one writer serves genesis and the views |
| stdlib | `walkdir` | two flat globs |
| stdlib | a graph crate | union-find is fifteen lines |
| stdlib | `regex` | P1's Q4 ruling holds; hand-written scanners throughout |
| installed dep | `strsim` | Levenshtein and Jaro, not Ratcliff/Obershelp with autojunk — it cannot reproduce the 48 probes. A deliberate step past this rung, argued by the parity measurement |
| one line | a build-tooling crate | `cargo`, `strip`, `tar` |
| one line | re-implemented sandbox plumbing | the contract runner imports `evals/run.py` |

## 8. Test tally

| suite | tests | state |
|---|---|---|
| `tests/views.rs` | 10 | pass |
| `tests/fidelity.rs` | 10 | pass |
| `tests/anchor_grammar.rs` | 5 | pass |
| `tests/matrix_command.rs` | 2 (69 probes) | pass |
| `tests/matrix_skill.rs` | 3 (64 probes) | pass |
| `tests/matrix_similar.rs` | 48 | pass |
| `tests/migration.rs` · `replay.rs` · `validate.rs` | 115 | pass, untouched |
| `tests/render.rs` · `cli.rs` | 49 | pass |
| **total** | **242** | **242 pass, 0 failed** |

Fix round 1 added eleven: two to `views.rs` (A2, A3) and nine to `matrix_similar.rs` (two
accounting tests for B1, five for allowlist resolution under A1, the command-family corpus pin
under A8, and the full sweep now reporting its own skip). Wall time went from 137 seconds to 35.

An earlier draft of this table put the total at 270 and the passes at 264. Both were arithmetic
errors in the summing row; the per-suite counts were right then and are right now. Every figure
above is read off `cargo test --all --no-fail-fast` directly.

Corpus pins re-asserted through the log: 50 documents · 321 command rules · 695 skill rules ·
1,016 total · 226 skill floors · 110 declared command floors · 36 fail nodes · 597 anchors folded.

## 9. The contract suite's preflight on this machine

Run today:

```
SKIPPED: the sandbox 'claude-mochiko' is not reachable: ERROR: Not authenticated to Docker
Sign in with: sbx login
exit 3 — the suite did not run, so nothing here is evidence of anything.
```

This is the expected honest result — wave 0 recorded the sandbox unauthenticated, and `sbx login`
is the user's own action. The case list prints before the skip, so "0 cases ran" is visible rather
than inferred, and the exit code is 3 rather than 0 so no gate can mistake it for a pass.

## 10. Open items

1. **Closed.** `migrations/README.md` carried the pre-delta anchor grammar; the lead's two
   sentences landed. The `D2a` delta itself is applied, pinned and green.
2. **The narrowed check set** (§3) — 81 probes across the two matrices exercise checks the Rust
   hard set does not carry. Families 2 to 4 need a wave ruling before the Python scripts are
   deleted at wave 6.
3. **The record's skill-matrix census** says 86; it is 114. A fact line for the wave landing.
4. **The similarity report's rendered text is still unasserted over the real corpus.** The
   figures are pinned twice (§12/A8) and the fixture runs assert the text, but no test reads the
   rendered report over all 1,016 rules. Narrower than it was, not closed.
6. **The command and skill matrices still carry transcribed `PYTHON_PROBES` arrays.** The
   detector matrix now re-derives its names from the script (§12/B1); the other two would take
   the same treatment cheaply.
7. **`.claude/rules/mochiko/rust-cli.md` does not document `MOCHIKO_FULL_SIMILAR`.** Outside this
   seat's pen; named for the lead.
5. **The sequence table needed no edit**: `migrations/README.md` already carries wave 1 as
   `0001 (genesis)`, which is what landed. The pen allowance there went unused.

## 11. Suggested commit

Nothing was committed. Suggested message:

```
Add the genesis migration, the derived views, and the ported probe matrices

Wave 1 seat P3 of the CLI schema-delivery build. The log gains its baseline:
migrations/0001-genesis.yaml imports all 50 shipped schema documents, folds
the provenance sidecar's 597 anchors onto their rules, and lifts the two
comment-carried `enforces: []` reasons into `note:` data. The crate gains the
generator behind it, the derived-view emitter with its YAML writer, and the
similar-rule detector, whose ratio reproduces CPython's difflib exactly,
autojunk included.

The three Python probe matrices are ported with exact accounting: every one of
the 134 command, 114 skill and 48 detector probes is claimed by precisely one
ledger, and a test does the set algebra. 81 of them exercise checks the Rust
hard set does not carry; each is named with its reason rather than dropped.

Release machinery lands with its publish job disabled until wave 2, and the
contract suite's two failure cases run in the sandbox, skipping loudly at
exit 3 when it is unauthenticated.

One lead-granted delta to another seat's file: model::is_decision_segment now
accepts a lettered decision number (D2a), the spelling two live provenance
anchors already use and the shipped Python checker already accepts. The anchor
was not normalised to fit the validator. tests/anchor_grammar.rs pins the
widening and the eight shapes that stay malformed.

No new dependency. No file under plugins/ changed.
```

## 12. Fix round 1

Against `v3-corpus-audit.md` (FAIL: 1 blocking, 9 advisory). Every finding taken. Each fix went
red first; the red is quoted where it names the defect better than prose would.

### B1 — the detector matrix now has name-level accounting (blocking)

`matrix_similar.rs` gained what the other two matrices already had, and then some:

- `PYTHON_PROBES: [&str; 48]`, every `check("…")` name verbatim and in source order.
- `PORTED`, built by a `ported!` macro from `<rust test> => [<python names>]`. The test is named
  once and used twice — as a function value the compiler resolves, and as the ledger's string —
  so a renamed or deleted test breaks the build instead of quietly unclaiming a probe.
- `NOT_APPLICABLE`, six rows with reasons.
- `EXTRA`, the six Rust tests with no Python referent, so the ported count cannot inflate.
- `the_whole_python_matrix_is_accounted_for`: every name claimed exactly once, no ledger name
  outside the Python list, and the split asserted as `(42, 6)`.

The first run, with the pre-audit three-row ledger, failed exactly where the audit said it would:

```
Python probes claimed by no ledger: [
    "e2e: exit 0 by default",
    "skill e2e: exit 0 by default",
    "authoring e2e: exit 0 by default",
]
```

Beyond the finding: `the_recorded_python_names_are_the_scripts_own` re-derives the 48 names by
scanning `scripts/test-find-similar-rules.py` and asserts the recorded array equals them. A
transcribed list can go stale in silence; this one cannot. The command and skill matrices still
carry transcribed arrays — a follow-up worth taking when someone next touches them.

### A1 — the allowlist resolves from the log, not the process working directory

`similar::default_allowlist(root)` became `similar::find_allowlist(log_dir)`, which walks up from
the log directory to the nearest ancestor carrying `scripts/similar-rules-allowlist.yaml`. The
granted arm in `cli.rs` passes `--log-dir` instead of `Path::new(".")`.

The report also stops going quiet. `Report` gained `allowlist: Option<PathBuf>` and `edges`, and
`render_report` now always prints one of two lines: `allowlist-suppressed edges: N` when a list
was read (including at zero, which was silent before), or `allowlist: none (N edges unsuppressed)`
when none was found. A 76-cluster read can no longer be mistaken for a clean one.

Verified end to end. The identical command over the identical log, run from a scratch directory
outside the repository:

```
rules scanned: 1016 · in-kind pairs scored: 146572 · clusters: 0 (none)
allowlist-suppressed edges: 181
```

Five tests pin it, including one asserting the resolution works while the process working
directory demonstrably lacks the file. Deliberate divergence from the Python, disclosed: the
Python prints nothing when it has no allowlist.

### A2 — `|+` writes the blank lines it promises to keep

`literal_block` chose `|+` for two or more trailing newlines and then emitted a body with none for
it to keep, so `"a\n\n"` read back as `"a\n"`. It now appends `trailing - 1` blank lines.
`a_multiline_scalar_keeps_every_trailing_newline` pins eight cases through a real parse.

### A3 — quoted scalars fold

`foldable` no longer requires `!needs_quote(text, false)`. Inside a `>-` block a colon, a hash, a
leading dash and a quote are all literal, and every emitted line is padded to the same column, so
the guard was buying nothing and costing readability. It now excludes control characters instead.
Measured over the 50 schema files and the 50 emitted views:

| | lines over 120 chars | longest line |
|---|---|---|
| shipped corpus, 50 schema files | 76 | 693 |
| emitted views, before | 307 | 944 |
| emitted views, after | 26 | 341 |
| generated log, before | 326 | 950 |
| generated log, after | 36 | 347 |

The "before" rows were measured by re-emitting with the old guard restored, not estimated. The
views are now shorter-lined than the corpus they mirror. The log's 36 survivors are markdown table
rows inside literal blocks, which cannot fold without changing content; it grew from 10,479 lines
to 11,701 and from 598,626 bytes to 618,122. **`migrations/0001-genesis.yaml` was regenerated**;
its `hash:` is unchanged at `sha256:361cf5d6…`, because the hash covers the decoded content and
only the styling moved.

### A4 — the offending node is asserted where the Python names one

Sixteen probes moved from `Expect::Reject(code)` to `Expect::RejectOn(code, id)` (the seat's own
tally said fourteen — six command, eight skill; the delta re-grade recounted sixteen and its
figures stand, lead-corrected here). Three Rust node addresses differ from the Python's and the Rust one is used, because
it names the node that is actually wrong: the three `enforces:` probes report on the rule carrying
the bad reference (`demo.fail.no-approval`) rather than on its unresolvable target, and the
condition probe reports on `mode` rather than `conditions.mode`.

Four could not move, and a comment in each file says so: the "canonical section absent" family
(one command, three skill) reports a finding with no id, because the node it would name is the one
that is not there.

| matrix | `Expect::Reject` | `Expect::RejectOn` |
|---|---|---|
| command | 21 | 28 |
| skill | 26 | 27 |

### A5 — the skew halt is read off the channel the binary writes it to

Two changes. The fixture's `!` line became
`` !`mochiko-cli rules brainstorm --section preamble 2>&1` `` so the halt reaches the model through
the channel wave 0 settled, and `case_skew` gained `assert_skew_halt_on_stderr`, which runs the
same log against the binary directly and asserts exit 3, empty stdout, and the message on stderr.
The case no longer rests on an untested assumption about what Claude Code does with stderr. One
assumption remains and is flagged in the fixture itself: that a redirect does not need its own
`allowed-tools` grant. It is confirmed at the first authenticated run, not before.

### A6 — an empty suite skips rather than sweeping

`if not CASES: return EXIT_SKIP`, before any case runs. Exit 0 now structurally means "every
declared case ran".

### A7 — the release archive name

`stage="${{ github.ref_name }}-${{ matrix.target }}"`, so the asset is
`mochiko-cli-vX.Y.Z-<target>.tar.gz` rather than doubling the crate prefix. That is the URL
template `cargo binstall` derives by default, so no `[package.metadata.binstall]` override is
needed. The upload list and the header comment match.

### A8 — the full-corpus sweep is opt-in, and the default suite keeps a real pin

`the_detector_reproduces_the_live_runs_figures_over_the_corpus` runs only under
`MOCHIKO_FULL_SIMILAR=1`, and prints its skip rather than passing silently. CI runs it in a step
of its own, so nothing is skipped where it matters.

The default suite gained `the_detector_reproduces_its_figures_over_the_command_family`, the same
assertion over the command grammar alone. Both figure sets are measured against the reference
implementation, not self-asserted:

| pin | rules scanned | pairs scored | clusters | suppressed | debug runtime |
|---|---|---|---|---|---|
| command family (default) | 321 | 12,154 | 0 | 60 | 5.2 s |
| whole corpus (opt-in) | 1,016 | 146,572 | 0 | 181 | 98 s |

Both reproduce `uv run scripts/find-similar-rules.py` exactly, the subset under
`--schemas-dir plugins/mochiko/schemas --allowlist scripts/similar-rules-allowlist.yaml`. The
whole suite is now 35 seconds rather than 137.

**Pen note:** adding a CI step is wider than this seat's `ci.yml` pen, which is the path filter
only. Taken because A8 asks for it in those words. Nothing else in that file moved.

### A9 — the report drifts

"nine that stay malformed" was already corrected when `D2a3` was added at the lead's restated
case list. §10.1 is closed: the two `migrations/README.md` sentences landed.

### Not asked for, and not done

`.claude/rules/mochiko/rust-cli.md` is where a maintainer would look for the
`MOCHIKO_FULL_SIMILAR` switch, and it is outside this seat's pen. Named delta for the lead.
