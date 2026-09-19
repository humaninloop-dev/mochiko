# Wave 1 — seat P3 (corpus) plan

**Seat:** P3 — genesis import and fidelity fixture, the derived-view emitter, the three ported
probe matrices, the similarity scorer, release machinery, the CI filter, the contract-suite
skeleton.
**Status:** planning only. Nothing built. Nothing written outside this file.
**Substrate:** P1 frozen at `cd5a333`. P2 is mid-flight (its `Cargo.toml` already carries `clap`;
`src/cli.rs` and `src/render.rs` are not yet on disk), so this plan binds only to the delta it
requests, never to P2's internal shape.

**Read before planning:** the wave plan (§1 P3 row, §5, §6, §7), the record (D1, D2, D6, D8 as
amended, D9, D11, and the post-acceptance amendments), `wave0-probe-report.md`, P1's cycle report
whole, `v1-core-audit.md`, `.claude/rules/mochiko/rust-cli.md`, the crate's five source files and
`migrations/README.md`, the three Python matrices and the detector, `evals/run.py` and
`evals/README.md`, `.github/workflows/ci.yml`, the sidecar head, and the wave-0 probe plugin.

**Two measurements run during planning** (session scratchpad only, no repo writes): the probe
census, by executing each matrix's `probes()`; and difflib parity, by re-implementing the ratio
algorithm and comparing it against `difflib` over the live corpus. Both are reported below at
(d) and (e). Scripts live at
`/private/tmp/claude-501/-Users-deepeshadmin-Documents-GitHub-mochiko/5e176b4c-4d84-42d8-8d7c-18e6d3d2f3df/scratchpad/`
(`count_probes.py`, `classify_probes.py`, `ratio_parity.py`, `corpus_size.py`, `headers.py`,
`headcmp.py`).

---

## (a) The genesis generator

`src/genesis.rs` exposes `build(root: &Path) -> Result<String, Vec<Finding>>`, returning the
migration file's text.

**How the 50 files become ops.** The generator walks `plugins/mochiko/schemas/*.yaml` (20) and
`plugins/mochiko/skills/*/schema.yaml` (30). Address derivation is lifted from
`tests/validate.rs::shipped_documents` rather than re-invented: a document's own `kind:` field
names its kind, a skill's name is its directory, and everything else takes the file stem. Each
file is parsed to a `serde_norway::Value`, decoded with `Document::from_value(kind, &value)`, and
re-encoded with `Document::to_value()` as the `content:` of one `import-document` op. Ops are
emitted in `DocRef` order (kind, then name) so the file is deterministic across runs and machines.

**Anchors.** `.mochiko/provenance.yaml` is read once. Its `kind:` must be `primitive-provenance`
or `command-provenance`; anything else is a rejecting finding. All 597 entries are folded onto
their rules as `anchor:` fields before encoding. The sidecar file itself is never written (D2). A
key matching no live rule is a reported finding, never a silent drop — this is the state-level
port of the checkers' "dangling entry" probe. A key matching a rule in more than one document is
also a finding; the corpus's prefixes make exactly-one-owner true today, and the generator asserts
it rather than assuming it.

**The two comment-carried reasons.** P1's handoff names `setup.fail.unclosed-trace` and
`setup.fail.floor-category-uncovered`, whose empty-`enforces:` reasons live in
`# D6 empty-with-reason:` YAML comments at `plugins/mochiko/schemas/setup.yaml` lines 387 and 406.
The generator scans the raw file for a `# D6 empty-with-reason:` comment block directly above any
`enforces: []` and lifts the comment body, minus the marker, into that rule's `note:`. Generalised
rather than hard-coded, for two reasons: a reworded comment carries without a code change, and a
third occurrence is caught instead of missed. An empty `enforces:` with no such comment is a
finding. The two rule ids are still asserted by name in the test, so the general path cannot
quietly stop covering the known cases.

**Header.** `grammar: 1`, `id: 0001-genesis`, `sequence: 1`, a one-line `intent:`, and
`anchor: "2026-09-03 cli-schema-delivery D2"` (see j6). The `hash:` is stamped by
`migration::with_hash`, which is now the only sanctioned path and replaces a stale value.

**Emission.** Through the shared YAML writer that `views.rs` owns, so the artifact is
block-scalar readable and diffable rather than a wall of one-line quoted strings.

**Expected size.** The corpus is 534 KB across 12,731 lines, of which 439 lines are comments.
Dropping comments, adding the indentation the ops nest the content under, and adding 597 anchor
lines gives roughly **600 to 650 KB, about 14,000 lines**. Measured exactly once the file exists.

**Keeping the committed file in sync.** `tests/fidelity.rs::genesis_regenerates_byte_identically`
rebuilds the file from the shipped sources and compares it byte-for-byte with the committed
`migrations/0001-genesis.yaml`, failing with the first differing line. Byte comparison, not a
hash, because the file is human-reviewed and a reviewer needs the line.

## (b) The fidelity fixture (`tests/fidelity.rs`)

For every shipped file: the original YAML through `Document::from_value` is the expected value;
the document at the same address in `replay::load_full(migrations/)`'s state is the actual value.

**41 rule-bearing documents** (6 commands, 30 skills, 3 common libraries, 2 registries) are
compared field by field, strings byte-exact. Document level: `declared_kind`, `declared_name`,
`vars` (order and values), `conditions` (values, resolution, note), `moments`, the section list
(id, title, intent, note), and `tombstones` (id, disposition). Rule level: id, labels with
`None` distinguished from `Some([])`, class, kind, text, `when` including term order and the
scalar-versus-list shape, pointer, extends, enforces with the explicitly empty case preserved,
note, anchor. Registries: label order, meanings, and the retired list.

**9 opaque documents** (8 templates plus the shelf data file) are compared by `canonical_hash`
over the raw value.

Divergences accumulate into one list and fail the test once, each named in the form
`command/setup · setup.fail.unclosed-trace · enforces`, so a broken genesis reports its whole
blast radius in a single run rather than one field per run.

Two deltas are expected and are asserted rather than ignored: the two `note:` fields genesis adds
are part of the expected value, and rule field order is compared field-wise rather than as a key
sequence, because P1's A11 normalises rule field order on emit.

## (c) The derived-view emitter (`src/views.rs`)

`emit(&State) -> Vec<(PathBuf, String)>` and `emit_to(&State, dir)`. Paths mirror the repo:
`<out>/plugins/mochiko/schemas/<name>.yaml` and `<out>/plugins/mochiko/skills/<name>/schema.yaml`.
There is no default output directory, so a shipped file cannot be written even by mistake.

**Key order** comes straight from `Document::to_value()`, which already writes the corpus's order:
document keys run kind, command or skill, vars, conditions, moments, sections, rules, tombstones;
rule fields run id, labels, class, kind, text, when, extends, pointer, enforces, note, anchor.

**Scalar style.** A string containing a newline is written as a literal block (`|` or `|-`), which
the template schemas need — `governance-intent.yaml` uses them. A long single-line string is
written folded (`>-`) and wrapped at word boundaries, which is what the rule corpus uses. A short
string stays plain and is quoted only where YAML would otherwise misread it. Short scalar lists
(`labels`, `enforces`, a condition's closed `values`) are written in flow style as `[a, b]`,
matching the corpus.

**Header.** Command views get the 8-line runtime-kernel header regenerated from a template with
the command name substituted. Measured during planning: all six shipped command headers are
byte-identical apart from the command name, so this limb is exact. Every other kind gets a
one-line generated banner.

**The test** (`tests/views.rs`) asserts semantic equality, never bytes: for each of the 50,
`canonical_hash(reparse(emitted)) == canonical_hash(shipped)`. A second assertion re-decodes the
emitted text through `Document::from_value` and compares it with the state's document, so the
writer is proved to be an inverse of the decoder and not merely hash-compatible. All writes land
under `CARGO_TARGET_TMPDIR`.

**What today's shape cannot preserve, disclosed rather than glossed:** every comment — all 439
lines, including the per-skill headers, which vary from 5 to 8 lines and carry family-specific
wording that no template can reconstruct, plus the inline `vars` comments and the two
`enforces` reasons that are now data; rule field order (P1 A11); blank-line layout; and the
choice between folded and plain style for any given scalar. The CI comparison is therefore
`diff -r target/views/plugins plugins` read by a human as a semantic diff, with the byte-level
noise expected.

## (d) The matrix port

**Census, by execution rather than by reading.** Command **134**, skill **114**, similar **48**;
total **296**. The record and the wave plan both say 86 for the skill matrix. That figure predates
the authoring-family and patterns-family waves, which grew the file. Flagged at (j1) rather than
silently used.

**`tests/matrix_command.rs`.** The Python `baseline_schema()` is translated into an in-memory
`State` carrying the demo command, the `command-labels` registry, and the `common` library. Each
probe mutates that state, runs `validate::validate(&state)`, and asserts the finding `Code` plus
the offending id. A message fragment is asserted only where the Python's expectation separates two
findings that share one code, so P1 keeps the freedom to reword messages without breaking 100
tests.

**`tests/matrix_skill.rs`.** Three fixtures, because `Family::of` derives the family from the
skill's stem: `demo-grader` (review family), `authoring-demo` (authoring family), and
`patterns-demo` (patterns family, which ships no common library, so any `extends:` there is a
finding). The 7 sweep-mode probes become whole-state assertions, which is what the Rust validator
does natively — it never has a single-skill mode.

**`tests/matrix_similar.rs`.** The 28 unit probes map one-to-one onto `similar.rs`. The 20
end-to-end probes become in-memory `State` fixtures asserting the returned cluster set, replacing
temp directories and subprocesses. The rendered report string is asserted once, separately, so
report formatting is covered without 20 tests depending on it.

**Not applicable under D6.** Every probe is accounted for; none is silently dropped.

| group | count | disposition |
|---|---|---|
| command `.md` scaffold | 19 | 6 count-pin probes port as computed-count assertions over state; 13 die with the scaffold |
| command sidecar | 7 | 3 port to genesis-side tests; anchor format is already P1's `is_anchor`; DECISIONS resolution is out of wave-1 scope; foreign-prefix becomes match-by-id |
| skill `.md` scaffold | 13 | same split: 5 count-pin probes port, 8 die |
| skill sidecar | 7 | same disposition as the command sidecar |

The count-pin probes port because the desync class they guard is exactly what the CLI's printed
pins remove: the number is computed from state, so the test asserts the computed `kind: fail` and
`class: floor` counts rather than a hand-written numeral in an `.md`. The 21 scaffold probes that
die (headings, heading order, Rules-block enumeration, prose token resolution, Not-done placement,
the retired label named in prose, and the missing-file branch) have no state-level referent once
the `.md` stops enumerating sections and pinning counts — D6 collapses scaffold criteria 2 and the
count limb of 3. On the sidecar side, three probes port to `genesis.rs` (the kind discriminator
accepted and rejected, the anchors-mapping shape, and a dangling entry reported); anchor format is
already probed in P1's suite; `resolves to no DECISIONS.md row` is deferred by wave-plan §3, which
scopes wave 1 to anchor format with resolution advisory until the repo path is known; and the
foreign-prefix-skipped probe becomes "an anchor is matched by rule id, never by prefix".

**Expected counts.** `matrix_command.rs` about **115** tests (108 state-level, 6 computed-count,
1 positive control). `matrix_skill.rs` about **97** (94 state-level, 3 controls, one per family).
`matrix_similar.rs` **48** plus the corpus pin. Exact figures land in the cycle report with the
not-applicable list beside them.

## (e) `src/similar.rs`

**Ported behaviour, from `scripts/find-similar-rules.py`.** `norm_for_sim`: lowercase, `${var}` to
«var», `/mochiko:<cmd>` to «cmd», own-prefix citations `<prefix>.(sec.|fail.)?` to «self»., strip
everything outside `[a-z0-9«».\s]`, squeeze whitespace. Kind buckets, with an absent `kind:`
reading constraint, and no cross-kind pair ever scored. `struct_bonus`: 0.08 for an equal non-null
pointer, 0.04 for an equal section slug, 0.04 for a labels Jaccard at or above 0.5, capped at
0.12, combined score capped at 1.00. The short-text guard: when either side has fewer than 6
normalised tokens the pair needs text similarity at or above 0.80. The same-`extends` skip,
compared on the raw target so it is grammar-agnostic. The allowlist as unordered id pairs, with
stale-id and missing-reason warnings preserved. Union-find clustering, the classification tags
(COMMON-CANDIDATE, CROSS-PAIR, INTRA-SCHEMA, plus EXTEND-GAP), the cluster sort key, and the floor
flag on members carrying `class: floor`.

Inputs come from state, not files: every `command` and `skill` document, with `extends:` resolved
through P1's public `validate::resolve_extends`, which inherits exactly text, labels and pointer —
the same three fields as the Python's `INHERITED_FIELDS`. The prefix comes from
`validate::derive_prefix`; a document whose prefix will not derive is skipped with a warning, as
the Python does.

**The ratio.** CPython's `difflib.SequenceMatcher` Ratcliff/Obershelp, implemented directly: the
`b2j` index over the second sequence, **autojunk** (at length 200 or more, every element occurring
more than `n // 100 + 1` times is dropped from the index), `find_longest_match` with its two
non-junk extension loops, the iterative matching-block queue, and `ratio = 2M / T`.
`real_quick_ratio` and `quick_ratio` are ported as the same early-outs, which is sound here
because token-sorting preserves the character multiset that `quick_ratio` bounds. `isjunk` is
always `None` in the detector, so the junk-extension loops are dead code and are not written.

**Measured, not assumed.** During planning I re-implemented that algorithm in Python — the exact
shape intended for Rust — and compared it against `difflib` over the live detector's own loaded
corpus, using the detector's own normalisation so the inputs are the strings the probes see.

| measure | result |
|---|---|
| pairs compared | 18,577 |
| `ratio()` mismatches | 0 |
| `text_sim()` mismatches | 0 |
| worst absolute delta | 0.0 |
| long-`b` pairs where autojunk moves the ratio | 962 of 2,000 |

Autojunk is load-bearing rather than a curiosity: it changes the answer on roughly half the long
pairs, so a port that skipped it would diverge on the corpus and on any probe whose text runs past
200 characters. It is ported.

**Pins in the test suite.** A golden table of about 30 `(a, b, ratio)` triples captured from
Python, spanning identical, disjoint, reordered, short, just under 200, just over 200 and
autojunk-sensitive inputs, asserted to 1e-12. Plus the corpus parity pin, measured during planning
by running the live detector over the real tree: **1,016 rules scanned, 146,572 in-kind pairs
scored, 0 clusters, 181 allowlist-suppressed edges**, against a 214-row allowlist.

**Surface.** `similar::clusters(&state, threshold, allowlist) -> Vec<Cluster>` and
`similar::render_report(...) -> String`. Advisory, exit 0, never a gate — the GI-019 posture the
detector already holds. The allowlist is read from `scripts/similar-rules-allowlist.yaml`,
maintainer-side, never shipped.

## (f) Release machinery and the CI filter

**`.github/workflows/release.yml`.** Triggers on tag `mochiko-cli-v*`, with
`permissions: contents: write`. Job `build` covers four targets: `aarch64-apple-darwin` and
`x86_64-apple-darwin` on macOS, `x86_64-unknown-linux-gnu` and `aarch64-unknown-linux-gnu` on
Linux, the arm64 leg through `cross` (see j7). Each target runs `cargo build --release --target`,
strips the binary, tars it, writes a `sha256sum`, and uploads the archive as a release asset, which
is what `cargo binstall` consumes. Job `publish` carries `needs: build`,
`environment: crates-io`, `if: false`, and a `cargo publish` body — present and disabled, so the
wave-2 landing lifts one line here and one line (`publish = false`) in `Cargo.toml`. Actions are
pinned to release tags, matching `ci.yml`'s existing convention and its recorded SHA-pinning
follow-up.

**`.github/workflows/ci.yml`.** The path filter gains `migrations/**` and `evals/contract/**` on
both the `push` and `pull_request` lists. Nothing else in that file changes.

## (g) The contract-suite skeleton (`evals/contract/run.py`)

**Imports, never forks.** `evals/run.py` is loaded through `importlib` (its filename is
import-safe) and the suite takes `run_session`, `sbx_sh`, `synth_plugin`, `parse_stream`, `WORK`
and `SANDBOX` from it. The sandbox plumbing, the neutral cwd, `--setting-sources ""`, and the
stored-auth path are all reused as they stand.

**Assertion functions**, one per clause of D8's set: `assert_bang_ran` (the marker the fixture's
`!` line emits is present in what reached the model); `assert_version_triple` (the first line of a
section render matches the D3-as-amended shape, binary, grammar and plugin version);
`assert_end_line` (the closing `mochiko-cli rules end · <primitive> · <section> · <N> rules` line
is present, so a render truncated at the ~30k ceiling cannot pass on its head line alone);
`assert_no_schema_read` (no `Read` tool use whose path ends in `schema.yaml` or sits under
`plugins/mochiko/schemas/`); and `assert_halted` (the install line is present and no work was
performed).

**Fixture.** `evals/contract/fixture/` holds a one-command scratch plugin carrying a single `!`
line, `allowed-tools: Bash(mochiko-cli *)` (wave-0 probe (a): the grant is load-bearing, and a
denied line is fail-closed), and the positive-confirmation halt clause. It is modelled on the
wave-0 probe plugin's granted-env command at
`.../scratchpad/wave0/plugins/probe2/commands/env-granted.md`. The plugin is staged under
`evals/.work/` so the sandbox sees it at the identical absolute path, which is how `evals/run.py`
already stages plugins.

**Two runnable cases today.** *Absence*: the binary is not on the sandbox `PATH`, and the probe
command must halt with the install line. *Skew*: the log carries `grammar: 99`, and the run must
halt with exit 3 and the exact D5 message. The per-primitive cases arrive at wave 3.

**An unauthenticated sandbox can never read as a pass.** The runner's preflight is a ladder: `sbx`
on `PATH`; the sandbox reachable (`sbx exec <name> true`); `claude --version` inside it; and one
1-turn probe returning a `result` event with no auth error. Any rung failing prints
`SKIPPED: <reason>` and exits **3**. An assertion failure exits 1. Exit 0 happens only when every
declared case actually ran and passed. The declared case list and each case's outcome print on
every path, so "0 cases ran" is visible on the terminal rather than inferred from a silent green.

**`evals/contract/README.md`** records the D8 gate split (the full suite is a maintainer-side gate
at `plugin.json` bumps, GitHub CI keeps the four crate layers and no headless runs), `sbx login`
as the user's own action, and the sandbox-auth Terms-of-Service caveat the record carries.

## (h) The ordered test list, red first

1. The YAML writer round-trips a hand-built value (red before `views.rs` exists).
2. Genesis address derivation: one `import-document` op per shipped file, in `DocRef` order.
3. Genesis lifts the two `enforces:` reasons into `note:` — P1's two exemption tests go clean.
4. Genesis folds 597 anchors; an unmatched sidecar key is reported, never dropped.
5. The committed `migrations/0001-genesis.yaml` regenerates byte-identically.
6. `tests/views.rs`: semantic equality over all 50 documents.
7. `tests/fidelity.rs`: field-by-field over the 41 rule-bearing docs, canonical hash over the 9
   opaque ones.
8. `similar.rs`: the golden ratio table, then norm, bonus, guards, classify.
9. `matrix_similar.rs`: the 48 probes, then the corpus pin (1016 / 146572 / 0 / 181).
10. `matrix_command.rs`: the positive control first, then one test per portable probe.
11. `matrix_skill.rs`: the same, across the three family fixtures.
12. `release.yml` and the `ci.yml` filter edit — read-checked, with `actionlint` if available.
13. `evals/contract/run.py`: the skip path first, because it runs with no sandbox, then the
    absence and skew cases.

## (i) Pre-code ladder disclosures (`mochiko:patterns-code-minimalism`)

| rung | not built | why |
|---|---|---|
| exist at all | SQLite, a persistent cache, an index file | D1 defers all three to a measured need; none is measured |
| exist at all | `.md` scaffold checks (the Python 7c/7d class) | dead under D6; the `.md` no longer enumerates sections or pins counts |
| exist at all | `DECISIONS.md` anchor resolution | wave-plan §3 scopes wave 1 to anchor format; resolution stays advisory |
| exist at all | merge or promotion logic in `similar.rs` | layer 2 is judgment, which GI-019's bright line keeps out of the tool |
| exist at all | anything in P1's or P2's files | pen discipline; needed changes are named as deltas at (j3) |
| in codebase | a second decoder, inheritance resolver, or hash | `Document::from_value`/`to_value`, `canonical_hash`, `resolve_extends`, `placeholders`, `derive_prefix`, `Family::of`, `migration::with_hash`, `replay::load_full`, `Finding`/`Code` are all reused as they stand |
| in codebase | a second YAML writer | one writer serves both genesis and the views |
| stdlib | `walkdir` | two flat globs; `fs::read_dir` is enough |
| stdlib | a graph crate | union-find over `BTreeMap` is about fifteen lines |
| stdlib | `regex` | P1's Q4 ruling holds; `${var}`, `/mochiko:` and prefix citations get hand-written scanners |
| installed dep | `strsim` | it implements Levenshtein and Jaro, not Ratcliff/Obershelp with autojunk, so it cannot reproduce the 48 probes. Deliberate step past this rung to about 90 hand-written lines, argued by the parity measurement at (e) |
| one line | a build-tooling crate in the release workflow | `cargo`, `strip` and `tar` do the job |
| one line | re-implemented sandbox plumbing in the contract runner | it imports `evals/run.py` |

**No new dependency.** The crate's dependency set stays `clap`, `serde`, `serde_norway`, `sha2`.

## (j) Open questions and requested deviations

1. **The skill matrix is 114 probes, not 86**, making the port total 296 rather than 268.
   *Recommendation:* port all 114 and record the corrected census in the cycle report. The
   record's figure is a fact line to repair at the wave landing, not a scope change.
2. **Genesis lands as one file of roughly 600 to 650 KB.** *Recommendation:* keep it single, as
   wave-plan §2 names it. A split by kind buys diff ergonomics for a file nobody hand-edits and
   costs a sequence-range grant. Say the word and I split it into 0001 through 0004.
3. **Wiring `similar.rs` into `migrate validate --report`** is one call inside P2's `src/cli.rs`.
   *Recommendation:* grant it alongside the already-granted `views emit --out <dir>` and
   `genesis emit` arms, since wave-plan §3 lists similarity clusters in the advisory set.
   Otherwise `similar.rs` lands library-only with its tests and wave 2 wires it.
4. **`genesis emit` takes `--out <path>` with no default.** Tests write to `CARGO_TARGET_TMPDIR`
   and the maintainer names `migrations/0001-genesis.yaml` explicitly. This keeps wave-plan §8's
   "no writes outside `target/` at test time" true by construction rather than by care.
5. **The two `enforces:` reasons are read from the raw file, not hard-coded.** A deviation from the
   literal handoff, which names the two rules. Reading the `# D6 empty-with-reason:` comment above
   any empty `enforces:` means a reworded comment carries and a third occurrence is caught. The two
   rule ids are still asserted by name, so nothing is lost.
6. **The genesis header's anchor.** Genesis imports protected content but supersedes nothing, so
   the grammar requires no anchor. *Recommendation:* carry
   `anchor: "2026-09-03 cli-schema-delivery D2"` anyway, as the wave's own provenance. Confirm or
   drop.
7. **arm64 Linux through `cross`** rather than an `ubuntu-24.04-arm` runner. *Recommendation:*
   `cross`, since it needs no runner-availability check and the release job is unexercised until
   wave 2 either way.
8. **View path shape** is `<out>/plugins/mochiko/...`, mirroring the repo so the CI check is
   `diff -r target/views/plugins plugins`. Confirm, or say if a flat `<out>/<kind>/<name>.yaml`
   is wanted instead.

---

**Standing commitments.** No file under `plugins/` changes byte-wise. No `plugin.json` bump. TDD
per `mochiko:executing-tdd-cycle`, with `mochiko:brownfield-integration` on every existing file.
Attempt bound 3 per unit; rework is test-first. Files outside the seat's pen are never edited; a
needed change is routed to the lead as a named delta.
