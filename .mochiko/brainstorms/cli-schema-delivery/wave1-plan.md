# Wave 1 — crate build plan (lead-drafted referent)

**Ruling home:** `record.md` D1 (log = truth; replay in memory; SQLite deferred), D2 (full
schema class; anchors), D3 as amended (per-section render, head + tail lines), D5 (version
contract), D6 (hard set; matrix port; audit unit; views stay text), D8 as amended (contract
suite in the sandbox), D9 (wave 1 scope), D11 (bright line). **Wave open:** user-confirmed
2026-09-03 with two adopt-first rulings — `clap` (derive) adopted for argument parsing;
`sha2` adopted for content hashes. **Floor:** sound loop tripped (judgment-authored product
code) — seats produce on lead-approved plans, fresh validators review, the user accepts;
transport floor topology lane fired on the crate → **sequential single pen-holder**: one
producer seat writes at a time; message legs held (mesh hold, content-pinned orders, no
resends, fan-in).

**Done condition (fixed):** `cargo test --all` green, `cargo fmt --check` and `cargo clippy
--all-targets -- -D warnings` clean, with every item in §6 present; independent code review
PASS on every seat unit; **no shipped file under `plugins/mochiko/` changes byte-wise this
wave**; no `plugin.json` bump.

---

## 1. Scope and ownership (three sequential producer seats)

| seat | owns (exclusively while it holds the pen) | delivers |
|---|---|---|
| **P1 — core** | `crates/mochiko-cli/Cargo.toml`, `src/lib.rs`, `src/model.rs`, `src/migration.rs`, `src/replay.rs`, `src/validate.rs`, `tests/migration.rs`, `tests/replay.rs`, `tests/validate.rs`, `migrations/README.md` | the typed model, the migration grammar + parser, the replay engine + state hash, the D6 hard-set validator, the log/grammar version contract |
| **P2 — surface** | `src/main.rs`, `src/cli.rs`, `src/render.rs`, `src/schema.rs` (re-base), `tests/render.rs` (extend), `tests/cli.rs` | `clap` CLI, `rules --section` render with head/tail lines, `template`/`--check` re-based on the replay, `migrate validate|status`, `views emit`, `--version`, `--plugin-root` resolution |
| **P3 — corpus** | `migrations/0001-genesis.yaml` (generated), `src/genesis.rs`, `src/views.rs`, `tests/fidelity.rs`, `tests/matrix_command.rs`, `tests/matrix_skill.rs`, `tests/matrix_similar.rs`, `src/similar.rs`, `.github/workflows/release.yml`, `.github/workflows/ci.yml` (filter edit), `evals/contract/` (runner skeleton) | genesis import + fidelity fixture, derived-view emitter + semantic-equality test, the three ported probe matrices, release machinery (publish gated), CI filter, contract-suite skeleton |

Sequence P1 → P2 → P3. A later seat may add `mod` lines to `lib.rs` and deps to `Cargo.toml`
only as the pen-holder of its turn (P1's files are released to the next seat at P1's
close). Shared types live in `src/model.rs`; P2 and P3 extend it only additively and
disclose every addition in their reports.

## 2. Migration grammar (P1 owns; frozen at P1's close)

- **Log location:** `migrations/` at the repo root (shipped inside the plugin from wave 3 —
  the CLI resolves it as `<plugin-root>/migrations/`, wave 1 reads the repo path); ordered
  by the numeric prefix; file name `NNNN-<slug>.yaml`.
- **File header:** `grammar: 1` · `id: NNNN-<slug>` · `sequence: <NNNN as int>` ·
  `intent: <one line>` · `anchor: "<YYYY-MM-DD> <session-slug> [D#]"` (optional; **required**
  when any change supersedes or tombstones a `class: floor` rule, a `kind: fail` rule, or a
  rule carrying an anchor in state) · `hash: sha256:<hex>` over the canonical body (the
  `changes:` list serialized canonically) — mismatch rejects. Sequence collisions (two files
  with one `sequence`) reject. Gaps are legal.
- **Document model in state:** a map keyed by `(kind, name)` — kinds: `command` · `skill` ·
  `command-common` · `skill-common` · `command-labels` · `skill-labels` · `template` ·
  `shelf`. Rule-bearing kinds (`command`, `skill`, the two commons) are typed:
  `RuleSchema { kind, name, vars, conditions, moments (commands only), sections[{id, title,
  intent, note?, rules[Rule]}], tombstones[{id, disposition}] }` with `Rule { id, labels?,
  class: floor|must|advisory, kind?: constraint|duty|gate|reservation|binding|bound|routing|
  fail|latitude, text?, when?, pointer?, extends?, enforces?, anchor? }`. Labels registries
  are typed `{ labels: map<name, meaning>, retired?: [] }`. Templates keep the existing
  `schema::Template` shape. Shelf data stays a generic YAML value.
- **Change ops** (each an item of `changes:`; each independently citable):
  `import-document {kind, name, content}` (whole document — genesis, templates, shelf) ·
  `replace-document {kind, name, content}` (templates/shelf only) ·
  `mint-section {schema, section: {id, title, intent, note?}}` ·
  `tombstone-section {schema, id, disposition}` ·
  `mint-rule {schema, section, rule}` ·
  `reword-rule {schema, id, text}` ·
  `set-rule-field {schema, id, field, value}` (labels · class · kind · when · pointer ·
  extends · enforces · anchor; `null` clears) ·
  `move-rule {schema, id, section}` ·
  `tombstone-rule {schema, id, disposition}` ·
  `supersede-rule {schema, id, disposition, anchor}` ·
  `set-var {schema, name, value}` · `set-condition {schema, name, spec}` ·
  `set-moment {schema, name, text}` ·
  `registry-add {registry, label, meaning}` · `registry-retire {registry, label, note}`.
  Unknown op → reject. A schema-level `note` for a deliberately empty section is data.
- **ID lifecycle:** mint-once — an ID that ever existed (live or tombstoned) can never be
  minted again; tombstone integrity — never both live and tombstoned; a reword keeps the
  ID; `move-rule` keeps the ID.

## 3. The D6 hard set (P1's validator, run on the state after every migration)

Reject the migration (exit 1, one finding per line, `code · schema · id · message`) on any of:
grammar parse · `kind` discriminator missing/unknown · **per-family section set** exact
(commands: `<prefix>.sec.` roles · reserved · tools · ways-of-working · boundaries ·
fail-conditions; review family + the dense five: independence · scope · inputs · verdict ·
output · reserved; authoring family: independence · scope · inputs · artifact · output ·
reserved; patterns family: trigger · scope · discipline · inputs · disclosure · reserved;
family derived from the skill name prefix exactly as `check-skill-schema.py` does; commons
and registries have no sections) · ID format (dotted slug, lowercase kebab segments) and
prefix (a rule/section ID leads with its schema's prefix: `spec.` / `impl.` / `arch.` /
`feat.` / `setup.` / `brainstorm.` for commands — read the prefix off the existing corpus,
never invent; `<skill-name>.` for skills; `common.` / `review-common.` /
`authoring-common.` for commons) · ID uniqueness · mint-once · tombstone integrity · every
label ∈ the matching registry · every `${var}` in text bound in `vars` (inherited text
substitutes from the binding schema's vars) · `extends` target resolves in the family's
common file, `class` declared locally on the stub, cross-family/cross-grammar `extends`
rejected · every `when` term resolves against declared `conditions` (values or `present`)
· `enforces` targets resolve to live local rules; `kind: fail` requires `enforces` (empty
list only with a `note`) · `kind: fail` ⇔ `.fail.` ID segment both ways (commands); `kind:
fail`, `enforces`, `moments` illegal in skill schemas · `class: floor`, `kind: fail`, or
anchored rule leaves only via `supersede-rule` with a resolving anchor (anchor format
`YYYY-MM-DD <slug>` + optional ` [D#]`; wave 1 checks format, `DECISIONS.md` resolution is
an advisory report until the repo path is known) · sequence collision · hash mismatch ·
grammar version outside the binary's range.

**Advisory (exit 0, `--report`):** deixis lint (the curated marker list from
`check-command-schema.py` 5b) · unused vars · unused declared conditions/moments ·
`enforces` reverse coverage (floors/gates no fail node enforces) · per-dimension condition
coverage · similarity clusters (P3's `similar.rs`, the detector's scoring ported) ·
budget figures (chars per schema).

## 4. Render contract (P2)

`mochiko-cli rules <primitive> --section <id> [--plugin-root <path>]` where `<primitive>`
is a command name (`specify`) or a skill name (`review-feasibility`), `<id>` is a section
slug (`roles`, `independence`, …) or **`preamble`**. Output, exactly:

```
mochiko-cli rules <primitive> · section <id> · binary <semver> · grammar <n> · plugin <version|unknown>
<body>
mochiko-cli rules end · <primitive> · <id> · <N> rules
```

- `preamble` body: the schema's identity line, resolved `vars` (name = value), the
  `conditions` block (dimension · values · resolution · note), `moments` (commands), and
  the **count pins**: `kind: fail` count (commands) and `class: floor` count (all), plus
  the section list. `N` = 0 for the preamble.
- Section body: `## <title>` · `<intent>` · then per live rule (tombstones omitted):
  `### <id>` · a bracket line `[class: <c>` + ` · kind: <k>` (omit when constraint) + ` ·
  when: <terms>` + ` · labels: <a, b>` + ` · pointer: <p>` + `]` · the text with `${var}`
  substituted and `extends:` resolved (inherited text/labels/pointer; local `class`/`kind`/
  `when`/`enforces` shown) · `enforces: <ids>` for fail nodes. A section with no rules
  renders its `note`.
- Plugin version: read from `<plugin-root>/.claude-plugin/plugin.json` when `--plugin-root`
  is given (else `unknown`); the log dir defaults to `<plugin-root>/migrations`, else
  `./migrations`, else `MOCHIKO_MIGRATIONS` env. Grammar range: the binary declares
  `[1, 1]`; out-of-range → exit 3 with the exact D5 message naming the install command.
- `template <name> [--check]` keeps today's output byte-for-byte for the 8 templates, now
  sourced from the replayed state (the closed `TEMPLATE_NAMES` and the embedded copies are
  removed; an unknown template still exits 2). `migrate validate [--report]` replays the
  log and prints findings; `migrate status` prints the state hash, sequence, grammar.
  `views emit --out <dir>` writes the derived views (§5). `--version` prints
  `mochiko-cli <semver> · grammar 1..1`.
- Exit codes: 0 ok · 1 validation findings · 2 usage/unknown name · 3 version contract.

## 5. Genesis, fidelity, views (P3)

- **Genesis:** `migrations/0001-genesis.yaml` is **generated** by a `genesis` module from
  the 50 shipped files at v0.103.0 (20 under `plugins/mochiko/schemas/`, 30 in-directory
  `skills/*/schema.yaml`) as `import-document` ops with the parsed content, plus the
  provenance sidecar's 597 anchors folded onto their rules as `anchor:` fields (the sidecar
  file itself untouched — D2). The generator is a test-time helper and a `genesis emit`
  subcommand; the committed file is the artifact.
- **Fidelity fixture (`tests/fidelity.rs`):** for every shipped file, parse the original
  YAML into the typed model and compare with the replayed state document field-by-field,
  strings byte-exact (IDs · text · class · kind · labels · when · extends · pointer ·
  enforces · tombstones · vars · conditions · moments · notes); for templates and the
  shelf file, generic-YAML equality. One failure per divergence, named.
- **Views (`src/views.rs`):** emit every state document as YAML to `<out>/<original
  relative path>` in the current file shape (same key order as the corpus, block scalars
  for long text, the 8-line runtime-kernel header for commands); `tests/views.rs` asserts
  **semantic equality** (`serde_norway::Value`) between each emitted view and the shipped
  file — comments are not preserved and are not compared. **Shipped files are never
  overwritten this wave**; the CI job emits to `target/views` and compares.
- **Matrix port:** every probe in `scripts/test-check-command-schema.py` (134),
  `scripts/test-check-skill-schema.py` (86), and `scripts/test-find-similar-rules.py` (48)
  becomes a Rust test: build the synthetic fixture as state, apply the mutation, assert the
  validator (or the similarity scorer) reports the named finding — and the positive control
  passes. Probes that test Python-specific surfaces (the `.md` scaffold checks 7c/7d, which
  die at D6) are ported as **`.md`-free** equivalents where a state-level assertion exists,
  otherwise listed in the report as "not applicable under D6" with the reason — never
  silently dropped.
- **Release machinery:** `.github/workflows/release.yml` on tag `mochiko-cli-v*`: build
  release binaries (macOS arm64/x64, Linux x64/arm64) with `cargo build --release`, strip,
  upload as release assets (for `cargo binstall`), **publish to crates.io gated behind a
  manual approval job that stays disabled** (wave 2 lifts it). `ci.yml` path filter gains
  `migrations/**` and `evals/contract/**`.
- **Contract-suite skeleton:** `evals/contract/run.py` — a sibling of `evals/run.py`
  (import its sandbox helpers, never fork them) with the D8 assertion set as functions
  (`!` line executed · version triple present · end line present · no schema file Read ·
  absence halts · skew halts) and two runnable cases today: **absence** (binary off `PATH`
  → the probe command halts with the install line) and **skew** (a log with `grammar: 99`
  → exit 3 message) exercised against a scratch plugin carrying one `!` line; the
  per-primitive cases arrive at wave 3.

## 6. Deliverable checklist (the validators' referent)

- [ ] `Cargo.toml`: `clap` (derive) and `sha2` added; nothing else new without disclosure
- [ ] `src/model.rs` typed model; `src/migration.rs` parser + ops; `src/replay.rs` +
      canonical state hash; `src/validate.rs` hard set + advisory reports
- [ ] `tests/migration.rs`, `tests/replay.rs` (replay twice → identical hash; op-by-op
      semantics; mint-once; anchor-required rejection), `tests/validate.rs`
- [ ] `src/cli.rs` (`clap`), `src/render.rs`, `template`/`--check` re-based, `--version`,
      `--plugin-root`, exit codes; `tests/render.rs` extended; `tests/cli.rs`
- [ ] `migrations/0001-genesis.yaml` + `src/genesis.rs`; `tests/fidelity.rs` green over all 50
- [ ] `src/views.rs` + `tests/views.rs` semantic equality; shipped files byte-untouched
- [ ] `src/similar.rs` + `tests/matrix_similar.rs`; `tests/matrix_command.rs`;
      `tests/matrix_skill.rs`; the "not applicable under D6" list in P3's report
- [ ] `.github/workflows/release.yml` (publish disabled), `ci.yml` filter
- [ ] `evals/contract/run.py` skeleton with absence + skew cases runnable
- [ ] `migrations/README.md`: grammar summary, lead-assigned sequence ranges, the anchor rule
- [ ] fmt · clippy `-D warnings` · `cargo audit` clean · every test green
- [ ] each seat's cycle report: tasks, red/green/refactor trail, code-minimalism ladder
      disclosures, additions to shared files

## 7. Seat protocol

1. **Plan first.** The seat reads this plan, the record's D1/D2/D3/D5/D6/D11, the rule file
   `.claude/rules/mochiko/rust-cli.md`, and the crate, then returns a written plan — files
   it will create/modify, the type shapes, the test list in order, open questions — and
   **stops**. The lead approves or amends; work starts only on the lead's explicit open.
2. **TDD.** `mochiko:executing-tdd-cycle` — red/green/refactor per task; the pre-code
   ladder (`mochiko:patterns-code-minimalism`) run and disclosed; `mochiko:brownfield-
   integration` for every existing file.
3. **Pen discipline.** Write only the owned files; a needed change outside them is routed
   to the lead as a delta, never made.
4. **Report.** A cycle report at `.mochiko/brainstorms/cli-schema-delivery/wave1-reports/
   <seat>.md`: what was built, the test tally, ladder disclosures, deviations from this
   plan (each named), open items.
5. **Attempt bound:** 3 verification attempts per seat unit; a rework is test-first.

## 8. Review criteria (validators, fresh seats, author never grader)

Grade the seat's diff and report against §2–§6 and the record's rulings: every §6 item
present and green · the hard set complete (each D6 clause has a rejecting test) · the
render contract byte-exact to §4 · fidelity green over all 50 files · no shipped file
changed · no `unsafe`, no network, no writes outside `target/` at test time · GI-019
bright line intact (no artifact grading, no dispatch) · plan deviations disclosed ·
matrix-port completeness (every Python probe accounted for). Verdict PASS/FAIL with a fix
list; default FAIL.
