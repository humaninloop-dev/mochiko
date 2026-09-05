# Wave 2 — governance amendment proposal (input to the `/mochiko:setup` amend run)

**Status:** lead-drafted proposal, 2026-09-04 · **Ruling home:** `record.md` D10 (envelope, adopted
2026-09-03, amended at Q13), D11 (GI-019 admission argument, `Assumed` until this run's
validator grades it), D4 (distribution), D7 (hooks), D9 wave 2, F13 (measured halt shape) ·
**Authority:** none of this text lands by this file. Governance surfaces are written only by the
`/mochiko:setup` amend run — its producer (`mochiko:authoring-constitution`), its cold intent
review (`mochiko:review-governance-intent`), its validator (`mochiko:validation-constitution`),
and the user's ratification. This file is the run's input, verbatim where marked, delta where
marked. **Semver: MAJOR — v2.0.1 → v3.0.0** (amendment policy: a non-negotiable's meaning changes;
v2.0.0 precedent).

**Trigger, stated:** `cli-schema-delivery` D10.1 — GI-020 is contradicted by the ruled no-fallback
delivery; wave 1 (crate) is built and accepted; wave 3 (the first `.md` re-point) MUST NOT open
before this amendment is ratified (D9). Also fired: the standing GI-002 trigger "revisit at the
crate's first public release" (D10.4) — the first crates.io publish follows this run.

---

## A. CLAUDE.md `## Non-negotiable constraints` — replace the "Plugin install stays additive" paragraph

**Current (verbatim):**
> **Plugin install stays additive** (governance trace GI-020). The plugin MUST install and function as a markdown-only plugin — no install-time build step, no binary dependency, no submodule-class fetch burden. Any admitted binary is strictly additive: its schema data files are Read raw as the first-class degraded path when the binary is absent. A distribution mechanism that makes install heavier violates this.

**Proposed replacement:**
> **Plugin install stays clone-only; delivery depends on `mochiko-cli`** (governance trace GI-020, superseded-by-ruling v3.0.0 — `cli-schema-delivery` D4/D10). The plugin MUST install by a plain marketplace clone — no install-time build step, no fetch beyond the clone, no submodule-class burden. Every command and skill depends on the separately installed `mochiko-cli` binary (a developer tool the user installs: `cargo install mochiko-cli` / the Homebrew tap); its absence, or a log outside the binary's grammar range, halts a run loudly at first use and never degrades — the shipped plugin carries **no schema file a run could read instead**. *Transition clause:* from this ratification until the wave-6 landing, primitives not yet re-pointed read the derived snapshot files shipped in the plugin; the clause expires when no schema file ships in the plugin, asserted by the contract suite's run-wide no-Read assert. Environments that disable skill shell execution or hooks by policy are declared unsupported.

**Also in the same section, the kernel-class paragraph:** leave the text; the trace comment
gains `· widened admission: cli-schema-delivery D11` (pointer only, GI-017 — no restatement).

## B. CLAUDE.md `## Governance` region

1. **Ratified line** → `**Ratified:** v3.0.0 · 2026-09-04 (AM-2) · production floor · depth: high ·
   modules: compliance none · knowledge-management (core + CHANGELOG) · release-gates`
   (GI-001 / GI-021 comments unchanged).
2. **Principles — GI-020 pointer line** → `- Clone-only install with a required `mochiko-cli`
   dependency — see `## Non-negotiable constraints` (detail: ledger GI-020; transition clause
   until wave 6) <!-- GI-020 -->`. GI-019 pointer line unchanged.
3. **Technology stack — GI-002 line** → replace the clause "the crate is additive
   maintainer-side tooling, plugin install stays markdown-only (GI-020)" with "the crate is
   **kernel-class delivery infrastructure** — it serves every command's and skill's rules from a
   migration log shipped in the plugin (`migrations/`), replayed in memory at fire; the plugin
   depends on it (GI-020 as amended v3.0.0); it is distributed as a developer tool (crates.io +
   Homebrew tap; release workflow `.github/workflows/release.yml`), and the plugin still installs
   by clone alone". Keep the CI sentence; append "the maintainer-side contract suite
   (`evals/contract/`, Docker sandbox) is the plugin-path gate (GI-012 as amended)".
4. **Quality gates — GI-012 lines** → gate 6 reads "`cargo test` PASS **plus the plugin contract
   suite's deterministic set green** (maintainer-side, sandbox — `evals/contract/run.py`, exit 0)";
   the gate-5 clause "schema-data/binary consistency (on the marketplace-sync gate)" reads
   "**derived view ≡ replay under the released binary range** (`cargo test`'s `views` and
   `fidelity` suites)". The first line's "markdown primitives stay procedurally gated" stays.
5. **Governance operations — amend triggers line** → append "· the transition clause's expiry
   (wave 6: no schema file ships in the plugin) is a recorded PATCH amendment, pre-authorized
   here as the AM-1 activation idiom".
6. **Writing style block:** untouched (preserve-on-regenerate carve-out).

## C. Ledger `.mochiko/memory/governance-ledger.md`

### C1 — GI-020 entry, rewritten

**Enforcement:**
- The plugin MUST install by a plain marketplace clone — no install-time build step, no fetch
  beyond the clone, no submodule-class burden.
- Every command and skill depends on the separately installed `mochiko-cli` binary (D4: a
  developer tool — `cargo install mochiko-cli`, the Homebrew tap; never shipped in the plugin).
  Absence or grammar skew halts loudly at first use (F13: the failing `!` line aborts the command
  before any model turn; the `UserPromptExpansion` hook carries the install line; `SessionStart`
  reports presence) and never degrades — the shipped plugin carries no schema file a run could
  read instead (D3, D9 wave 6).
- **Transition clause:** from ratification (v3.0.0) until the wave-6 landing, primitives not yet
  re-pointed read the derived snapshot files shipped in the plugin; the clause expires when no
  schema file ships in the plugin, asserted by the contract suite's run-wide no-Read assert.
- Environments that disable skill shell execution (`disableSkillShellExecution`, Cowork/synced
  skills) or hooks by policy are **declared unsupported**.

**Testability:**
- Pass: a fresh clone plus the documented tool install renders every command's and skill's rules
  (contract suite, deterministic set); with the binary absent, every mochiko fire halts before a
  model turn with the shell error visible and the hooks' install line delivered; a log outside the
  binary's range halts with the D5 message naming the install command. · Fail: any mochiko run
  proceeding without the version-triple head and end lines; any schema file shipped in the plugin
  after the transition clause expires; any install step heavier than the clone plus the documented
  tool install.

**Rationale:** the record's ranked drivers (B change management, C integrity in one toolchain —
`high`; A delivery — `medium`) and the user's explicit no-fallback instruction; the governance
cost is attributed to driver A at `medium` on that instruction (record, "Roads rejected at the
frame"). The clone-only property the 2026-07-21 submodule removal protected is kept; the binary
dependency is added by ruling with its access-loss class named (D4: users who cannot install
developer tooling lose the plugin entirely).

**Trace:** GI-020 (minted at AM-1; **superseded-by-ruling at AM-2, v3.0.0** — driver
`.mochiko/brainstorms/cli-schema-delivery/record.md` D4/D10/F13).

### C2 — GI-019 entry, three clauses added (text of the bright line unchanged)

Append to **Enforcement**:
- **Admission ruling (widened role):** `cli-schema-delivery` (accepted 2026-09-03) is the recorded
  admission for `mochiko-cli`'s widened role — delivery of every command's and skill's rules from
  the migration log, hard constraints on the store's own data, and dependency-halt hooks
  (`SessionStart` presence; `UserPromptExpansion` on `mochiko:*` commands and `PreToolUse` on
  `Skill` for `mochiko:*` skills, exit 2 only when the binary is absent or out of range). The
  2026-08-16 admission covered template delivery only and did not extend on its own.
- **Bright-line argument, recorded (D11):** (i) a required binary whose absence halts a run, and a
  hook that blocks the plugin's own commands only when that binary is absent, are infrastructure
  dependencies in the delivery role the line licenses — present and in range, the binary renders
  and the hook delivers; absent, nothing can be delivered and the halt is the honest report of
  that fact, never a verdict on the run's work; (ii) hard constraints at migration apply are
  maintainer-time definition of the store's own data — a ruled carve: the landing ritual is a
  pipeline in this repo's sense, and a rejection there is a structural-validity check on data the
  tool owns (a compiler on its own language), never a grade of a primitive's judgment content,
  which the author≠grader audit keeps; (iii) the judgment and sequencing clauses are untouched —
  the CLI grades no artifact and dispatches nothing; behavior-gating hooks are declined
  (`producer-plan-enforcement` D1 respected).
- The advisory-checker clause: the three repo-level checkers (`scripts/check-command-schema.py`,
  `check-skill-schema.py`, `find-similar-rules.py`) were never waived — they rest on this clause —
  and retire into the crate's validator at wave 6 under the admission above; the six
  skill-shipped helpers under GI-008 are untouched.

**Testability** — add to Fail: "a shipped hook that blocks on anything other than the binary's
absence or grammar skew". **Trace:** append `· widened admission: cli-schema-delivery D11
(AM-2, v3.0.0)`.

### C3 — GI-012 entry

Gate 6 → "`cargo test` PASS for `crates/mochiko-cli` **and the plugin contract suite's
deterministic set green** (`python3 evals/contract/run.py` exit 0 in the `claude-mochiko`
sandbox — maintainer-side at every `plugin.json` bump; GitHub CI keeps the crate layers only)".
Gate 5's schema-data/binary clause → "**derived view ≡ replay under the released binary range**,
asserted by the crate's `views` and `fidelity` suites". Add gate note: "the behavioural read-back
metric is reported, never gating (D8)". **Trace:** append `· AM-2 v3.0.0 widening
(cli-schema-delivery D8/D10.3)`.

### C4 — GI-002 (identity + risk surface)

Discharge the "revisit at the crate's first public release" annotation: the first public release
(crates.io `mochiko-cli`, the `humaninloop-dev/homebrew-tap` formula) follows this run. Controls
named: signed release tags, `cargo audit --deny warnings` in CI, sha256-published release assets
(`release.yml`), `cargo publish` behind a manual-approval job. **Name the shipped hooks
explicitly** (D7/D10.4): plugin-authored code executes on every consumer's machine at every
session start and every mochiko fire, under a 5-second timeout, fail-open by platform design when
the hook cannot run; the user ratified that knowingly (record Q13). The public-product transition
trigger stays standing (this is not that transition).

### C5 — GI-008 (waiver) — note only, no change

The six skill-shipped helpers stay waived and untouched. Record the observation: the waiver's
"script count grows" trigger was tripped by the three repo-level checkers (v0.92.0 · v0.99.0 ·
v0.100.0) without a disposition; moot at wave 6 when they retire (record D10.7).

### C6 — Version log row

`| 3.0.0 | 2026-09-04 | AM-2 — clone-only install with a required `mochiko-cli` dependency; no
file-read fallback (MAJOR: GI-020 superseded-by-ruling; GI-019 widened admission recorded; GI-012
gates widened; GI-002 first-public-release revisit discharged, hooks named). Driver:
cli-schema-delivery D4/D7/D8/D10/D11, F13 | GI-020 rewritten · GI-019 +3 clauses · GI-012 gates 5/6
re-expressed · GI-002 annotated · GI-008 noted |`

## D. `.claude/rules/mochiko/rust-cli.md` — rewrite (governance surface, path-scoped)

The current file is stale on every bullet (renderer-only, additive install, D8 fallback,
8-template scope). Proposed body (frontmatter `paths: ["crates/mochiko-cli/**",
"migrations/**", "evals/contract/**"]`):

> # Rust CLI — kernel-class delivery under the bright line <!-- GI-019 · GI-020 · GI-012 -->
>
> `crates/mochiko-cli/` is mochiko's admitted kernel-class tool: it serves every command's and
> skill's rules from the migration log (`migrations/`, shipped in the plugin), replayed in memory
> at fire, and validates the log's own data. Admitted by two recorded rulings —
> `schema-based-template-guidance` D11 (template delivery, 2026-08-16) and **`cli-schema-delivery`
> D11 (the widened role, 2026-09-03)**; the standing bright line binds it.
>
> - **Bright line (GI-019).** The tool renders, replays, and validates its own data. It MUST NOT
>   grade an artifact, MUST NOT dispatch or sequence agents, MUST NOT hold judgment that skills
>   own. Its hooks block only on the binary's absence or grammar skew, never on behavior. Home:
>   CLAUDE.md `## Non-negotiable constraints`; detail: ledger GI-019.
> - **Dependency, not fallback (GI-020 as amended v3.0.0).** The plugin depends on this binary;
>   absence or skew halts loudly and never degrades. No code path may read a schema file as a
>   fallback; the transition clause (until wave 6) covers only primitives not yet re-pointed.
>   Detail: ledger GI-020.
> - **The log is truth (record D1/D2/D6).** `migrations/NNNN-<slug>.yaml` is the only editing
>   surface for schema content; a migration that supersedes or tombstones a `class: floor`,
>   `kind: fail`, or anchored rule MUST carry a ruling anchor or the CLI rejects it; `hash:` is
>   required; the derived views under `--out` are human-readable text and never hand-edited.
> - **Quality gate (GI-012).** `cargo test --all`, `fmt --check`, `clippy -D warnings`, and
>   `cargo audit --deny warnings` under CI; the full similarity sweep is opt-in
>   (`MOCHIKO_FULL_SIMILAR=1`, its own CI step); the plugin contract suite
>   (`python3 evals/contract/run.py`, sandbox) is the maintainer-side gate at every
>   `plugin.json` bump. Every crate unit lands on a lead-approved plan with an independent
>   non-author code review (author≠grader extends to code).
> - **Release (record D4).** Distributed as a developer tool — crates.io + the Homebrew tap via
>   `.github/workflows/release.yml` on `mochiko-cli-v*` tags; the plugin ships no binary.
>
> Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md` —
> GI-019, GI-020, GI-012.

## E. Not in this run (scope fence)

- No change to GI-003/004/005/006/017/021 text. GI-006 reconstructibility: the migration log plus
  strips plus `DECISIONS.md` plus version stamps; the amend run may add "or the migration log" to
  GI-006's wording as a PATCH clarification if the validator asks — not required.
- `primitive-edits.md` re-key (schema strips → migrations; pair criteria collapse) lands at
  **wave 6** (D10.6), not here.
- No plugin primitive is edited by this run (no `plugin.json` bump).

## F. After ratification (D9 wave 2 tail, outside the amend run)

1. Lift `publish = false` in `crates/mochiko-cli/Cargo.toml` and `if: false` on `release.yml`'s
   publish job; fill the crate's `repository`/`homepage`/`readme` metadata for crates.io.
2. Tag `mochiko-cli-v0.1.0`; the workflow builds four targets and publishes; add the Homebrew
   formula to `humaninloop-dev/homebrew-tap`.
3. Wave 3 opens: `brainstorm` pilot re-point, `hooks/hooks.json`, contract-suite per-primitive
   cases, read-cost and read-back measurements, the pilot abort criteria.
