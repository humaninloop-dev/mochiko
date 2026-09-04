# Governance Ledger

**Governance Floor:** production (asserted) · **Depth level:** high (user-declared, one-way; `high` terminal — GI-021, minted at AM-1 discharging the legacy-default pointer; set up under the single floor pre-adaptive-depth, already conformed to full depth #7 fold 2026-08-11) · **Modules:** compliance: none (GI-001 negatives confirmed, incl. no-UI) · template: knowledge-management (core + CHANGELOG elective) · release-gates · **Trace:** GI-001 (fact profile) · GI-021 (depth level)
**Version:** 3.0.1 (must match the region stamp)

## Waivers

Any asserted standard, with a recorded justification (D4); permanent pending the D4.1 revisit.
**Legal-mandate module obligations are unwaivable (D4.2)** — a waiver row naming one is a
validator FAIL.

| Standard | Justification | Revisit trigger (optional) | Trace |
|----------|---------------|----------------------------|-------|
| FLOOR-TEST as applied to the 6 helper scripts (1 bash, 5 python) | Thin standalone validators/detectors; no shared deps; test/lint infrastructure absent and not worth erecting for them today | Script count grows, or a script becomes load-bearing in a shipped flow (lead-composed, user-ratified 2026-08-06) | GI-008 |

*(FLOOR-SEC's secret-scanning clause is a recorded **narrowing**, not a waiver — see GI-003 below.)*

*AM-2 note (2026-09-04, no change to the row):* the waived six are the skill-shipped helpers
under `plugins/mochiko/skills/*/scripts/`. The row's "script count grows" trigger was tripped by
three repo-level checkers (`scripts/check-command-schema.py` v0.92.0 · `find-similar-rules.py`
v0.99.0 · `check-skill-schema.py` v0.100.0) with no disposition recorded at the time. They were
never waived — they rest on GI-019's advisory-checker clause — and they retire into the crate's
validator at wave 6, which makes the observation moot then; it is recorded here so the gap is
not read later as an unrecorded lapse.

## Amendment policy

- Route: `/mochiko:setup` amend mode; fact-profile changes (module attach/detach) and
  un-waives are governance events.
- Semver: MAJOR — principle removal / incompatible redefinition / floor-level change / module
  attach or detach · MINOR — new principle or waiver change · PATCH — clarification.
- Approvers: Deepesh (solo maintainer — GI-002 team reality; no code-review lean, enforcement is
  procedural gates + audits).
- Standing amend triggers recorded in the synthesis: public-product transition (GI-002 —
  compat obligations) · CI arrives / the Rust crate lands — a widened consequence set: GI-003
  un-narrow secret scanning · GI-002 tech-stack statement re-expressed (home: CLAUDE.md GI-002
  line) · GI-004/GI-007 inapplicability clauses re-expressed · GI-012 dormant gates activate
  (cargo test PASS + schema/binary consistency) · GLOSSARY.md gains content (GI-009 deviation) ·
  helper-script waiver trigger (GI-008). **The CI-arrives/crate-lands trigger fired and discharged
  at v0.76.0** — its consequence set landed (see the 2.0.1 amendment-log row); public-product
  transition, GLOSSARY.md content, and the helper-script waiver remain standing.
- **The crate's first-public-release trigger (GI-002) fired at AM-2 and its discharge is
  conditional** (2026-09-04, review I5, user-ruled): four controls are named — `cargo audit
  --deny warnings` in CI (present, `.github/workflows/ci.yml`) · sha256-published release assets
  (present, `.github/workflows/release.yml`) · `cargo publish` behind a manual-approval GitHub
  environment (**owed** — the `crates-io` environment is declared in `release.yml`, but its
  approval rule is a GitHub setting not visible in the tree, and the job is `if: false` today) ·
  signed release tags (**owed** — no signing exists in the repo). The trigger discharges only
  when all four exist at the first publish; until then it stays open, and the two owed controls
  are wave-2 tail obligations gating that publish. `cargo audit --deny warnings` is also one of
  the four crate layers in GI-012's release train.
- **The GI-020 transition clause's expiry is a pre-authorized PATCH amendment** (AM-2, the AM-1
  activation idiom): when wave 6 lands and no schema file ships in the plugin, the clause is
  struck and the version-log row recorded without a fresh `/mochiko:setup` amend run. The
  wave-6 re-key of `.claude/rules/mochiko/primitive-edits.md` is pre-authorized on the same
  footing (review I2), and its scope is the whole of: the ceremony re-keyed from schema strips
  to migrations, **and** that file's `paths` globs gaining `plugins/mochiko/migrations/**` and
  `plugins/mochiko/hooks/**`. The glob half is a **wave-3 obligation** — it lands the moment
  those directories ship, not at wave 6, because an unscoped shipped directory is a primitive
  edited with no ceremony reaching its author.
- Modules declined durable at AM-1 (not re-offered on future amends): GI-013 layer-rules ·
  GI-014 evolution-notes — rationale in the synthesis; re-openable only by explicit user ruling.
  Declined durable at v1.0.0 and homed here at v3.0.1 (AM-2 O-1/A5): GI-011 the
  knowledge-management `RUNBOOK.md` elective — nothing deployed, nothing operated.

## Exception registry

| Exception | Principle (GI-ID) | Granted | Expires/revisit |
|-----------|-------------------|---------|-----------------|
| (none yet) | | | |

## Principles (Three-Part metadata, keyed by GI-ID)

### GI-003 — Secrets Out of the Repo · home: CLAUDE.md region line

**Enforcement**:
- `.claude/settings.local.json` listed in `.gitignore` (landed 2026-08-06, this run — the GI-015
  confrontation fix); any new local-credential file joins it before first use.
- No credentials, tokens, or keys in primitives, records, or brainstorm artifacts — checked in
  the author≠grader audit pass on any primitive edit.
- Secret-scanning clause **un-narrowed at v0.76.0** (CI arrived with the template-schema Rust
  crate — the pre-worded consequence of the amend-policy CI-arrival trigger): a CI secret-scan
  step now runs on push/PR touching `crates/**` or `plugins/mochiko/schemas/**`
  (`.github/workflows/ci.yml` — `git ls-files | grep` token patterns, no third-party action,
  matching this principle's own testability), joining the gitignore + pre-commit vigilance. The
  prior narrowing (no CI obligation while no CI existed) is discharged.

**Testability**:
- Pass: `git ls-files | grep settings.local.json` empty; `git check-ignore .claude/settings.local.json` matches; no grep hit for token patterns in tracked files — the token-pattern grep now also runs as the CI secret-scan step (v0.76.0).
- Fail: any tracked file carries a live credential, or the gitignore entry is removed.

**Rationale**: A live `ANTHROPIC_AUTH_TOKEN` sat one `git add -A` from public history (detected
this run). A prose repo has exactly one secret class — local tool credentials — and one cheap,
total control.

**Trace**: GI-003 (floor-asserted: FLOOR-SEC; narrowing recorded then **un-narrowed v0.76.0** on CI arrival; fold of GI-015)

### GI-004 — Primitive Audit Ratchet · home: CLAUDE.md region line

**Enforcement**:
- Every shipped-primitive edit passes the independent author≠grader audit (`mochiko:validator`
  against the primitive's own text) before the `plugin.json` version bump that ships it — the
  existing ceremony at `.claude/rules/mochiko/primitive-edits.md`, referenced not restated
  (GI-017). The schema data files (`plugins/mochiko/schemas/**`) are shipped primitives from
  v0.76.0 and take the same strip + author≠grader ceremony (primitive-edits path scope extended).
- Helper scripts carved out by the GI-008 waiver above.
- **Re-expressed at v0.76.0 (crate landed):** the prose audit ratchet stays the test suite for
  markdown primitives; the Rust crate `crates/mochiko-cli` additionally carries a real `cargo
  test` suite (11 tests, the M6 gate) plus an independent non-author code review (author≠grader
  extended to code artifacts). For `crates/**` the FLOOR-TEST expression is executable rather than
  translated — the GI-007 coverage/smoke inapplicability holds only for the markdown plane, not
  the crate.
- **Re-expressed at AM-2 (v3.0.0) for schema content** — the ratchet is unchanged, the audit
  unit is not. For markdown primitives the unit stays the primitive's own text. For schema
  content the steady-state unit becomes the migration file plus the regenerated derived-view
  diff, graded by `mochiko:validator` on five criteria (intent stated · anchor present where
  required · ID lifecycle right · floor and fail survival · register), with the CLI's apply
  result as the deterministic pre-pass (record D6). The `.md`'s independent count self-check is
  **retired and booked as a loss** (record D3): the version triple confirms delivery, the CLI's
  printed counts assert completeness, and the contract suite tests it. The `primitive-edits.md`
  re-key landing this at wave 6 is a pre-authorized PATCH activation of this ruling, not a fresh
  amend run (review I2).
- **Author≠grader extends to code** (crate, wave-1 precedent): every unit of
  `crates/mochiko-cli` lands on a lead-approved plan with an independent non-author review.
  Touch-time carrier: `.claude/rules/mochiko/rust-cli.md`.

**Testability**:
- Pass: each bump's landed edits have audit PASS on record (session record or decision row); a
  landed schema-content change has an audit PASS over its migration file and the regenerated
  view diff; a landed crate unit has a non-author review on record.
- Fail: a bump ships a primitive edit with no audit trail; a schema-content change lands on the
  CLI's apply result alone with no graded audit; a crate unit lands reviewed by its author.

**Rationale**: Prose has no coverage percentage; the audit IS the test suite. Translation of
FLOOR-TEST's ratchet — the baseline (all shipped primitives audited) MUST NOT decrease.

**Trace**: GI-004 (floor-asserted: FLOOR-TEST; expression translated for markdown primitives;
**re-expressed v0.76.0** — executable `cargo test` for `crates/**`; GI-007 coverage/smoke
inapplicability now scoped to the markdown plane only; **re-expressed AM-2 v3.0.0** — schema-content
audit unit re-keyed to the migration file plus the view diff, count self-check retired; driver
`.mochiko/brainstorms/cli-schema-delivery/record.md` D3/D6. The three executable gates now live
in three homes, one each: the crate's `cargo test` here on GI-004 · the log's hard set on GI-005 ·
the plugin path's contract suite on GI-012)

### GI-005 — Record-Layer Integrity · home: CLAUDE.md region line

**Enforcement**:
- Protected content leaves only by recorded ruling (strips/supersession) — the existing
  primitive-edit ceremony and KM landing ritual, referenced not restated (GI-017).
- Dead pointers are defects: the KM dead-pointer scan
  (`.mochiko/memory/knowledge-management.md` invariants) runs at command boundaries under
  fix-on-sight.
- **Two regimes from AM-2 (v3.0.0), each named** (record D2; review I3). The **prose limb**
  is unchanged and stays procedural: strips, supersession-by-ruling, and the dead-pointer scan.
  The **schema-rule limb becomes mechanical**: protected content in schema rules leaves only
  through a migration carrying a ruling anchor, rejected at apply by `mochiko-cli`'s hard set.
  The rule grammar itself lives in the schema and the log, referenced not restated (GI-017).

**Testability**:
- Pass: every `ROADMAP.md`/`DECISIONS.md`/`BACKLOG.md` pointer resolves or carries the
  `provenance: unrecoverable` terminal stamp; no silent deletion of protected lines; the log's
  hard set rejects nothing at validate (`mochiko-cli migrate validate --log-dir migrations
  --plugin-root plugins/mochiko`, 0 rejecting at ratification, 2026-09-04).
- Fail: a broken pointer without the stamp; a protected line gone with no strip entry; a
  protected schema rule superseded or tombstoned by a migration carrying no ruling anchor.

**Rationale**: This repo's failure mode isn't runtime corruption — it's provenance corruption.
Translation of FLOOR-ERR: the record layer is the data that must never silently corrupt.

**Trace**: GI-005 (floor-asserted: FLOOR-ERR; expression translated; runtime clauses subsumed
inapplicable per GI-007; **re-expressed AM-2 v3.0.0** — the schema-rule limb mechanized at
migration apply, the prose limb unchanged; driver
`.mochiko/brainstorms/cli-schema-delivery/record.md` D2)

### GI-006 — Traceability as Observability · home: CLAUDE.md region line

**Enforcement**:
- Every primitive edit reconstructible from the record layer: strips ledger
  (`.mochiko/strips/`) + `DECISIONS.md` + version stamps — existing carriers, referenced not
  restated (GI-017).
- **The migration log joins the carriers at AM-2 (v3.0.0)** — from wave 6 it is the verbatim
  record for schema rules, each migration carrying its own intent; strips keep the prose plane.
  Its home is `plugins/mochiko/migrations/` in the plugin from wave 3, the repo root
  `migrations/` until then.

**Testability**:
- Pass: for any shipped primitive line, its origin (decision row, strip entry, migration entry,
  or session record) is findable; strip entries carry version stamps.
- Fail: a primitive change whose provenance cannot be reconstructed; a schema-rule change with
  no migration entry stating its intent.

**Rationale**: No logs exist; the audit trail is the observability surface. Translation of
FLOOR-OBS.

**Trace**: GI-006 (floor-asserted: FLOOR-OBS; expression translated; log/health clauses subsumed
inapplicable per GI-007; **re-expressed AM-2 v3.0.0** — the migration log added as the verbatim
record for schema rules; driver `.mochiko/brainstorms/cli-schema-delivery/record.md` D1/D2)

### GI-009 — Knowledge-Management Core (pin ratified) · home: `.mochiko/memory/knowledge-management.md`

**Enforcement**:
- The project-pinned invariants file is the runtime source; command landing steps and
  `mochiko:grooming-operating-docs` resolve against it. Rules-file carrier:
  `.claude/rules/mochiko/operating-docs.md` (pre-existing, preserved).
- Pin status updated this run: **ratified as the ruled core** (revisit trigger discharged);
  ARCHITECTURE.md deferral retired (doc has content); GLOSSARY.md deferral carried — scaffold
  when it gains content.

**Testability**:
- Pass: KM invariants (bijection · status-agreement · open-only · caps · bounds · dead-pointer ·
  in-flight agreement · presence) hold at command boundaries.
- Fail: any invariant trips without a groom invocation.

**Rationale**: The operating-docs layer is this repo's live, proven governance backbone —
codified from working reality, not imposed.

**Trace**: GI-009 (module: knowledge-management-core; brownfield codification of the 2026-07-25
pin)

### GI-010 — CHANGELOG Elective · home: `CHANGELOG.md`

**Enforcement**:
- Adopted elective of the KM module: every `plugin.json` version bump appends a `CHANGELOG.md`
  entry — enforced as a release gate (GI-012).

**Testability**:
- Pass: top `CHANGELOG.md` entry's version matches `plugin.json`.
- Fail: bump lands with no matching entry.

**Rationale**: Release-shaped project (semver, marketplace); the roadmap stamp line carries
groom history but no user-facing release history.

**Trace**: GI-010 (module: knowledge-management-elective-changelog)

### GI-012 — Release Gates · home: CLAUDE.md region line (summary); this entry (detail)

**Enforcement** — a `plugin.json` version bump MUST NOT land unless:
1. Author≠grader audits PASS for every shipped-primitive edit in the bump (GI-004);
2. Strip/supersession entries recorded for every removal (GI-005);
3. The landing ritual is complete (decision row · trail move · ROADMAP touch — KM pin);
4. `CHANGELOG.md` entry appended (GI-010);
5. Marketplace metadata (`marketplace.json`) synced to the bumped version — the current
   0.10.0-vs-0.53.0 lag is the tracked defect this gate exists to close (GI-016); **from v0.76.0
   also schema-data/binary consistency**, which **at AM-2 (v3.0.0) becomes derived view ≡ replay
   under the released binary range**, asserted by the crate's `views` and `fidelity` suites.
6. **`cargo test` PASS for the Rust crate `crates/mochiko-cli`** (the M6 executable gate) —
   active from v0.76.0 — **and, from AM-2 (v3.0.0), the plugin contract suite's deterministic
   set green**: `python3 evals/contract/run.py` exit 0 in the `claude-mochiko` Docker sandbox,
   maintainer-side at every `plugin.json` bump (GitHub CI keeps the crate layers only). **A
   SKIPPED suite (exit 3) is not green — it blocks the bump** until the suite actually runs
   (review I6, user-ruled): a gate that can pass by being unable to run is not a gate.

*Activated at v0.76.0 (AM-1 pre-wording) — the template-schema Rust crate landed:* gate 6 (`cargo
test` PASS) and the gate-5 schema-data/binary-consistency addition are now blocking. The clause
was dormant while the plugin was markdown-only; it fired on the crate's arrival.

**The crate's own release train is gated from AM-2 (v3.0.0)** (review I8, user-ruled) — the
binary is the dependency every consumer runs, so a `mochiko-cli-v*` tag MUST NOT land without:
the four crate layers green (`cargo test --all` · `cargo fmt --all --check` · `cargo clippy
--all-targets -- -D warnings` · `cargo audit --deny warnings`); the contract suite green against
the tagged binary; and the render's head-and-tail output shape unchanged — or a coordinated
`plugin.json` bump when that shape changes, because the `.md` halt clauses key on it and the
grammar range does not version it. The two owed first-publish controls (signed release tags ·
`cargo publish` behind a manual-approval environment) gate the first publish under the GI-002
trigger recorded in the amendment policy above.

**Gate note:** the behavioural read-back metric is **reported, never gating** (record D8) — it
informs the wave-3 pilot's abort criteria (GI-020's revisit trigger); it does not block a bump.

**Substrate caveat, carried (review I7):** the contract suite runs on the Docker sandbox's stored
consumer-subscription auth — a `Contested` ruling sustained against adverse Terms-of-Service
evidence (kinako D8; record D8). GI-001's "contractual commitments: none" stands: this is a
third-party ToS exposure on **this gate's substrate**, not a contractual commitment of the
project, not a fact-profile dimension, and not a module trigger. If the substrate becomes
unusable the gate needs a new one before the next bump; it does not lapse.

Environments: none — nothing deploys; distribution is the Claude Code marketplace ·
Cadence: manual, at `plugin.json` bumps (synthesis real-commands table).
Rollback: `git revert` of the bump commit + marketplace metadata re-sync — executable by the
solo maintainer; no time-bound SLO (no operated service).

**Testability**:
- Pass: all six checks hold at the bump commit (gates 1–5 every bump; gate 6 `cargo test` PASS from v0.76.0, the crate having landed, plus the contract suite at exit 0 from v3.0.0); a `mochiko-cli-v*` tag has the four crate layers, the suite against the tagged binary, and an unchanged output shape or a coordinated plugin bump. · Fail: any missing; a contract suite reported SKIPPED (exit 3) and treated as green; a crate tag landed on any of the three train conditions unmet.

**Rationale**: Releases are the only distribution moment; the gates codify the existing ritual
plus the one detected drift (marketplace lag) it silently permitted. From AM-2 the crate's
release is a second distribution moment of the same system — consumers run the binary — so it
carries its own train rather than riding the plugin's.

**Trace**: GI-012 (module: release-gates; GI-016 folded as gate 5; AM-1 dormant crate-gate clause **activated v0.76.0** — gate 6 `cargo test` + gate-5 schema/binary consistency; **widened AM-2 v3.0.0** — contract suite on gate 6, SKIPPED blocks, gate 5 = view ≡ replay, crate release train, ToS substrate mark `Contested`; driver `.mochiko/brainstorms/cli-schema-delivery/record.md` D8/D10.3)

### GI-017 — Pointer-Only Region · home: the governance region itself

**Enforcement**:
- The region and this ledger **point at** existing constraint homes (CLAUDE.md prose · rules
  files · KM pin); they never restate them. Restating an existing constraint on a governance
  surface is a trace violation against this element — checked by the validator and by any
  future amend run.

**Testability**:
- Pass: no governance-surface line duplicates the operative text of an existing home.
- Fail: a restated constraint (two homes, drift risk).

**Rationale**: Single-sourcing is a live convention here (analysis finding 6); a second home for
any rule is where drift starts. User ruling, dimension 9: "leave these out."

**Trace**: GI-017 (minted: pointer-only selection constraint)

### GI-019 — Kernel-Class Tooling Admission (bright line) · home: CLAUDE.md `## Non-negotiable constraints` (prose)

**Enforcement**:
- Kernel-class tooling — executable tooling whose output primitives depend on (source-of-truth
  delivery, composition, or any standing infrastructure role) — is admitted ONLY by a recorded
  ruling (a `DECISIONS.md` row + the session/decision record). No admission without a ruling on
  record.
- The standing bright line binds every admitted instance: it never gates pipeline progress, never
  dispatches or sequences agents, never holds judgment that skills own — checked at the admission
  ruling and in the author≠grader audit of the admitting change.
- No general kernel and no orchestration/brain code is licensed (Python/MCP brains, capability
  catalogs, DAG-mediated orchestration stay banned). Advisory post-hoc checkers consumed as
  optional exit-code signals are NOT kernel-class — the 6 existing scripts (5 `.py` validators,
  1 `.sh` detector) land outside, carried by waiver GI-008.
- **Admission ruling — the widened role (AM-2, v3.0.0):** `cli-schema-delivery` (accepted
  2026-09-03) is the recorded admission for `mochiko-cli`'s widened role — delivery of every
  command's and skill's rules from the migration log, hard constraints on the store's own data,
  and dependency-halt hooks (`SessionStart` presence; `UserPromptExpansion` on `mochiko:*`
  commands and `PreToolUse` on `Skill` for `mochiko:*` skills, blocking only when the binary is
  absent or out of range). The 2026-08-16 admission covered template delivery only and did not
  extend on its own.
- **The bright-line argument, recorded (D11):** (i) a required binary whose absence halts a run,
  and a hook that blocks the plugin's own commands only when that binary is absent or out of
  range, are infrastructure dependencies in the delivery role the line licenses — present and in
  range, the binary renders and the hook delivers; absent, nothing can be delivered and the halt
  is the honest report of that fact, never a verdict on the run's work; (ii) hard constraints at
  migration apply are maintainer-time definition of the store's own data — a ruled carve: the
  landing ritual is a pipeline in this repo's sense, and a rejection there is a structural-validity
  check on data the tool owns (a compiler on its own language), never a grade of a primitive's
  judgment content, which the author≠grader audit keeps; (iii) the judgment and sequencing clauses
  are untouched — the CLI grades no artifact and dispatches nothing, and behavior-gating hooks are
  declined (`producer-plan-enforcement` D1 respected). The argument is `Assumed` until this run's
  validator grades it (record D11).
- **The advisory clause, applied:** the three repo-level checkers
  (`scripts/check-command-schema.py` · `check-skill-schema.py` · `find-similar-rules.py`) were
  never waived — they rest on that clause — and they retire into the crate's validator at wave 6
  under the admission above. The six skill-shipped helpers under waiver GI-008 are untouched.

**Testability**:
- Pass: every kernel-class component in the tree traces to a recorded admission ruling and
  satisfies all three bright-line clauses; every shipped hook blocks only on the binary's absence
  or a log outside its grammar range. · Fail: an admitted binary that gates the pipeline,
  sequences agents, or holds skill-owned judgment; a kernel-class component with no admission
  ruling; a shipped hook that blocks on anything other than the binary's absence or grammar skew.

**Rationale**: Skills and agents are the primary quality surface; an unbounded kernel is the exact
failure mode mochiko was built against. The bright line keeps admitted tooling to
delivery/composition roles, never the judgment or orchestration skills own. Softened from the
v1.0.0 absolute no-kernel position per the D11 ruling (evidence basis n=0 — the recorded concession
is that template delivery alone would not carry the CLI; the machine rides the foundation bet).

**Trace**: GI-019 (minted at AM-1; driver: `.mochiko/brainstorms/schema-based-template-guidance/record.md` D11 · **widened admission: `cli-schema-delivery` D11 (AM-2, v3.0.0)** — bright-line text unchanged, the three clauses recorded as its argument; AM-1's template-scope limb discharged, the no-general-kernel and no-orchestration limbs standing)

### GI-020 — Clone-Only Install with a Required `mochiko-cli` Dependency · home: CLAUDE.md `## Non-negotiable constraints` (prose)

*Superseded-by-ruling at AM-2 (v3.0.0). The AM-1 entry read: the plugin installs and functions
markdown-only, any admitted binary strictly additive, the schema data files Read raw as the
first-class degraded path (record D8). That degraded path is withdrawn by ruling — the
supersession is the whole point of the MAJOR bump, and the prior text is preserved here so the
change is reconstructible (GI-006).*

**Enforcement**:
- The plugin MUST install by a plain marketplace clone — no install-time build step, no fetch
  beyond the clone, no submodule-class burden. The no-build-step property is a property of the
  **plugin**, not of the tool install.
- Every command and skill depends on the separately installed `mochiko-cli` binary (record D4: a
  developer tool — `cargo install mochiko-cli` or the Homebrew tap; never shipped in the plugin).
  It serves their rules from the migration log the plugin carries from wave 3 at
  `plugins/mochiko/migrations/`; until wave 3 the log lives at the repo root `migrations/` and no
  installed plugin carries it.
- Absence of the binary, or a log outside its grammar range, **halts loudly at first use and
  never degrades** — measured (record F13): the failing `!` line aborts the command before any
  model turn, the `UserPromptExpansion` hook carries the install line, `SessionStart` reports
  presence. At the end state the shipped plugin carries no schema file a run could read instead
  (records D3, D9 wave 6).
- **Transition clause:** from this ratification (v3.0.0) until the wave-6 landing, primitives not
  yet re-pointed read the derived snapshot files shipped in the plugin. The clause expires when
  no schema file ships in the plugin, asserted by the contract suite's run-wide no-Read assert.
  Its expiry is a pre-authorized PATCH amendment (amendment policy above).
- **Declared unsupported** (dimension-10 exclusions, review I4): environments that disable skill
  shell execution (`disableSkillShellExecution`, Cowork and synced skills) or hooks by policy ·
  **PowerShell-only Windows** (`shell: bash` fails without Git Bash). Windows is served only with
  Git Bash present and only via `cargo install`, which compiles from source — a Windows user runs
  a Rust build once, at tool-install time, never at plugin-install time.

**Testability** — two tiers, per review I1. **Assertable at ratification (2026-09-04):**
- Pass: the contract suite's absence and skew cases green (2/2, 2026-09-04); the log's hard set
  rejecting nothing (`mochiko-cli migrate validate --log-dir migrations --plugin-root
  plugins/mochiko`, 0 rejecting). · Fail: either regressing.

**Activated at v3.0.1 by the wave-3 pilot (2026-09-04, plugin v0.104.0; the AM-1 dormant-clause
idiom):** `brainstorm` renders from a fresh staged install (contract suite 10/10, read-back 3/3);
with the binary absent the fire halts before a model turn on the hook's install line; a
`grammar: 99` log halts with the D5 message. The rows below are now assertable as written:
- Pass: a fresh clone plus the documented tool install renders every command's and skill's rules;
  with the binary absent every mochiko fire halts before a model turn with the shell error
  visible and the hooks' install line delivered; a log outside the binary's range halts with the
  message naming the install command. · Fail: any mochiko run proceeding without the version-triple
  head and end lines; any schema file shipped in the plugin after the transition clause expires;
  any install step heavier than the clone plus the documented tool install.

**Revisit trigger (review M1):** the wave-3 pilot's abort criteria — the floor read-back metric
below its pre-registered bar, or the per-invoke read cost above the pre-conversion baseline —
halt waves 4–5 and return the posture to the user. **Evaluated 2026-09-04: not tripped**
(read-back 3/3 at the 3/3 bar; 10,839 bytes delivered against the 12,819-byte baseline). The
trigger stays live for waves 4–5 per converted primitive. Reversal after this ratification costs a
second amend run plus re-pointing every converted `.md` (record D9, priced).

**Rationale**: the record's ranked drivers (B change management, C integrity in one toolchain —
`high`; A delivery — `medium`) and the user's explicit no-fallback instruction; the governance
cost is attributed to driver A at `medium` on that instruction (record, "Roads rejected at the
frame"). The clone-only property the 2026-07-21 submodule removal protected is kept intact; the
binary dependency is added by ruling, with its access-loss class named and accepted eyes-open
(record D4: users who cannot install developer tooling lose the plugin entirely). The log ships
in the plugin rather than in the binary because embedding would make every rule edit a crate
release and a user reinstall (lockstep, D5 rejected), and a network fetch is the
silent-degradation class this ruling exists to exclude; priced 2026-09-04 at +604 KiB on the
wave-3 ship, a peak near 1.1 MB while snapshots still ship, and roughly +70 KiB net at the
wave-6 end state.

**Marks** (mirroring record D4/D10): the no-fallback ruling and the plugin-carries-the-log ruling
**Confident** (user-ruled); the distribution basis **Contested** — a user-installed binary was
chosen against the lead's recommendation of committed prebuilt binaries, the user's reasons
inferred and `Assumed`; the transition clause's validity **Assumed** until this run's validator
grades it (record D10).

**Trace**: GI-020 (minted at AM-1, user-declared; **superseded-by-ruling at AM-2, v3.0.0** —
driver `.mochiko/brainstorms/cli-schema-delivery/record.md` D4/D9/D10/F13, log home user-ruled at
review C1)

### GI-022 — No Repo-Level Feature Map for mochiko Itself · home: CLAUDE.md region line

**Enforcement**:
- mochiko's own repository keeps no `FEATURES.md` and no per-capability feature entries. Its
  planning surfaces are the knowledge-management core: `ROADMAP.md` (direction) · `BACKLOG.md`
  (open threads) · `DECISIONS.md` (rulings), with sessions in `.mochiko/brainstorms/`.
- A command limb that demands the map in this repo (`setup.fail.no-feature-map` and its kin) is
  satisfied by that core; a run MUST NOT mint the map to clear the limb. Re-openable only by an
  explicit user ruling, not by a producer's judgment.

**Testability**:
- Pass: no `FEATURES.md` or `.mochiko/features/` in the tree, and the KM four are current.
- Fail: a feature map minted in this repo with no user ruling on record.

**Rationale**: the feature-map layer governs the product projects mochiko is used in, not the
plugin repository itself; a map here would duplicate the KM core and start the drift that
single-sourcing exists to prevent. The amend run surfaced the absence as the rule requires and
offered a reconstruction; the user declined durably.

**Trace**: GI-022 (minted at AM-2, v3.0.0; declined durable, user-ruled at deck Card 8; elicited
from the `setup.fail.no-feature-map` amend limb)

## Confrontation rulings (brownfield, this run)

| Ruling | Disposition | Trace |
|--------|-------------|-------|
| Live `ANTHROPIC_AUTH_TOKEN` in `.claude/settings.local.json`, absent from `.gitignore` | Fixed this run — gitignore entry landed 2026-08-06; folded into GI-003 enforcement | GI-015 |
| `marketplace.json` 0.10.0 vs `plugin.json` 0.53.0 | Becomes release gate 5 (GI-012) — sync obligation, not one-off fix | GI-016 |
| `ARCHITECTURE.md` header cites v0.48.0 vs plugin v0.53.0 | Accepted as intentional — doc updates only at component-changing landings, per its own contract | GI-018 |

**Floor status at ratification (brownfield assessment):** Security partial→codified (GI-003) ·
Testing translated (GI-004, audit ratchet live) · Error Handling translated (GI-005, record
layer) · Observability translated (GI-006, traceability). No MUST-implement gaps — every
category is live in translated form; application-shaped machinery inapplicable in kind (GI-007).

## Amendment log

| Version | Date | Change | GI delta |
|---------|------|--------|----------|
| 1.0.0 | 2026-08-06 | ratified (first setup run; brownfield; KM pin ratified into ruled core) | GI-001–018 |
| 2.0.0 | 2026-08-16 | AM-1 — no-kernel softening + additive CLI (MAJOR: a non-negotiable's meaning changes; user-ruled). Driver: schema-based-template-guidance D11 | +GI-019 (kernel-class admission) · +GI-020 (additive install) · +GI-021 (depth: high) · GI-002 annotated (identity + tech-stack dormant) · GI-012 dormant crate-gate clause · GI-013/GI-014 declined-durable |
| 2.0.1 | 2026-08-16 | AM-1 dormant crate-gate clauses **activated** — the template-schema Rust crate `crates/mochiko-cli` landed at plugin v0.76.0 (PATCH: pre-authorized activation per the AM-1 pre-wording, four principles' enforcement text re-expressed, no fresh `/mochiko:setup` amend). Driver: schema-based-template-guidance build | GI-012 active (gate 6 `cargo test` PASS + gate-5 schema/binary consistency) · GI-002 tech-stack re-expressed (Rust crate, compiled binary, CI present) · GI-003 un-narrowed (CI secret-scan present) · GI-004/GI-007 re-expressed (crate carries a real `cargo test` suite coexisting with the prose audit ratchet) · `plugins/mochiko/schemas/**` added to the primitive-edit path scope |
| 3.0.0 | 2026-09-04 | AM-2 — clone-only install with a required `mochiko-cli` dependency; no file-read fallback (MAJOR: a non-negotiable's meaning changes — user-ruled). Driver: cli-schema-delivery D1–D11, F13 | GI-020 **superseded-by-ruling** (clone-only kept · required binary · measured halt · transition clause until wave 6 · unsupported environments · pilot abort criteria as revisit trigger) · GI-019 widened admission + the three D11 clauses + advisory-checker placement · GI-012 gates widened (gate 6 contract suite, SKIPPED blocks, gate 5 = view ≡ replay, crate release train, ToS substrate mark `Contested`) · GI-004 schema-content audit unit re-keyed, count self-check retired · GI-005 schema-rule limb mechanized · GI-006 migration log added as a carrier · GI-002 identity and risk re-expressed, first-public-release trigger fired with a conditional discharge (two controls owed) · **+GI-022** (no feature map for this repo, declined durable) · GI-008 untouched, trigger observation recorded · `.claude/rules/mochiko/rust-cli.md` rewritten, `paths` widened |
| 3.0.1 | 2026-09-04 | PATCH — the wave-3 pilot landing at plugin v0.104.0 (pre-authorized: the AM-2 glob half; the wave-open Q-B ruling; O-1/A5). `.claude/rules/mochiko/primitive-edits.md` `paths` += `plugins/mochiko/migrations/**` · `plugins/mochiko/hooks/**`; criteria 1, 3, and 11 gain converted-command clauses (heading `## Rules — delivered by mochiko-cli` + the grant, the CLI-printed count pin, no raw common-file Read where the render resolves stubs) · GI-011 homed on the declined-durable line · GI-020 Testability dormant tier **activated** and the revisit trigger evaluated, not tripped · the CLAUDE.md region's "today the log lives at the repo root" clauses struck and the new-file read line re-pathed. Mints no principle; no fresh `/mochiko:setup` amend | GI-004 · GI-011 · GI-020 |

*AM-2 addendum (2026-09-04):* `.claude/rules/mochiko/rust-cli.md`'s `paths` gained
`migrations/**`, `plugins/mochiko/migrations/**`, and `evals/contract/**` per the ratified scope,
plus `.github/workflows/**` and `plugins/mochiko/hooks/**` as producer-flagged proposals ruled at
the acceptance gate — the release-train gate (GI-012) is violable in the workflows, and the
hook-blocking clause (GI-019) in the hook file wave 3 authors. Mints no principle. In the same
run, and outside the governance region, the two `## Non-negotiable constraints` paragraphs in
`CLAUDE.md` were edited under the Card 1 ruling ratified 2026-09-04: the plugin-install paragraph
replaced wholesale (GI-020's supersession) and the kernel-class paragraph's trace parenthetical
extended with the widened admission (GI-019). No other content outside the markers was touched.

*AM-1 addendum (2026-08-16, post-acceptance):* `.claude/rules/mochiko/rust-cli.md` added when the
crate path `crates/mochiko-cli/` was chosen — a touch-time reminder scoped to the crate, pointing
at GI-019 (bright line) · GI-020 (additive install) · GI-012 (M6/cargo-test dormant gate). Mints
no principle. The AM-1 "no rules touch needed" assertion was accurate as of acceptance (no crate
path existed to scope to); this is an of-its-time addendum, not a reversal.
