# Governance Ledger

**Governance Floor:** production (asserted) · **Depth level:** high (user-declared, one-way; `high` terminal — GI-021, minted at AM-1 discharging the legacy-default pointer; set up under the single floor pre-adaptive-depth, already conformed to full depth #7 fold 2026-08-11) · **Modules:** compliance: none (GI-001 negatives confirmed, incl. no-UI) · template: knowledge-management (core + CHANGELOG elective) · release-gates · **Trace:** GI-001 (fact profile) · GI-021 (depth level)
**Version:** 3.2.0 (must match the region stamp)

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
never waived — they rest on GI-019's advisory-checker clause — and they retired into the crate's
validator at wave 6 (v0.107.0, 2026-09-05: the three scripts and their tests deleted), which
makes the observation moot; it is recorded here so the gap is
not read later as an unrecorded lapse.

## Amendment policy

- Route: `/mochiko:setup` amend mode; fact-profile changes (module attach/detach) and
  un-waives are governance events.
- Semver: MAJOR — principle removal / incompatible redefinition / floor-level change / module
  attach or detach · MINOR — new principle, a principle significantly expanded, or a waiver
  change · PATCH — clarification.
- Approvers: Deepesh (solo maintainer — GI-002 team reality; no code-review lean, enforcement is
  procedural gates + audits).
- Standing amend triggers recorded in the synthesis: public-product transition (GI-002 —
  compat obligations) · CI arrives / the Rust crate lands — a widened consequence set: GI-003
  un-narrow secret scanning · GI-002 tech-stack statement re-expressed (home: CLAUDE.md GI-002
  line) · GI-004/GI-007 inapplicability clauses re-expressed · GI-012 dormant gates activate
  (cargo test PASS + schema/binary consistency) · GLOSSARY.md gains content (GI-009 deviation) ·
  helper-script waiver trigger (GI-008). **The CI-arrives/crate-lands trigger fired and discharged
  at v0.76.0** — its consequence set landed (see the 2.0.1 amendment-log row). **The
  GLOSSARY.md-content trigger fired and discharged at AM-4** (v3.1.2, 2026-09-23) —
  `GLOSSARY.md` gained content at plugin v0.114.0 and joined the knowledge-management module's
  live operating docs (see the 3.1.2 row); it is struck from the standing set. Public-product
  transition and the helper-script waiver remain standing.
- **The crate's first-public-release trigger (GI-002) fired at AM-2 and its discharge is
  conditional** (2026-09-04, review I5, user-ruled): four controls are named — `cargo audit
  --deny warnings` in CI (present, `.github/workflows/ci.yml`) · sha256-published release assets
  (present, `.github/workflows/release.yml`) · `cargo publish` behind a manual-approval GitHub
  environment (**owed** — the `crates-io` environment is declared in `release.yml`, but its
  approval rule is a GitHub setting not visible in the tree, and the job is `if: false` today) ·
  signed release tags (**owed** — no signing exists in the repo). The trigger discharges only
  when all four exist at the first publish; until then it stays open, and the two owed controls
  are wave-2 tail obligations gating that publish. `cargo audit --deny warnings` is also one of
  the four crate layers in GI-012's release train. **From AM-3 (v3.1.0, review C5, user-ruled
  2026-09-14) the two still-owed controls — signed release tags and the `crates-io` environment's
  approval rule (the publish job is `if: false` at `.github/workflows/release.yml:100` today) —
  are hard preconditions of the wave-4 hook ship:** the `plugin.json` bump that ships the
  conformance hooks MUST NOT land before the first publish with all four controls in place, and a
  maintainer break-glass install never substitutes for consumers. The same clause is recorded on
  GI-012's release limb. Once the wave-4 bump has landed under this precondition, striking the
  clause from the region gates line and these two paragraphs is a pre-authorized PATCH amendment,
  recorded in the log row. *(Void at AM-5 — this strike can never fire; the exception row's expiry
  strike set replaces it.)* **Breached at AM-5 (2026-09-24):** the conformance hooks shipped at
  plugin 0.109.0–0.114.0 before any publish, so the pre-authorized strike above can never fire;
  the breach is excepted by the exception-registry row below, which carries the expiry strike set —
  this paragraph's "hard preconditions of the wave-4 hook ship" limb is struck or re-worded when
  that row expires.
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
| Conformance hooks shipped at plugin 0.109.0–0.114.0 (first on public `main` 2026-09-19, `2f92b09`) before the first `mochiko-cli` publish carrying the four named controls. **How:** the 0.109.0 bump was committed on its branch as "bump staged, not landed" (`5d8fc69`, 2026-09-15) and reached `main` inside the unrelated PR #36 merge; the path-scoped rules inject on Read, so nothing fired at the merge (recurrence rule: GI-012, held bumps). **Exposure accepted:** only the maintainer installs today (user-confirmed 2026-09-24), from unpinned `main` (`README.md`'s `cargo install --git` — no tag, checksum, or signature), until the publish. Further `plugin.json` bumps may ship the hooks under this row, each bump's `CHANGELOG.md` entry citing it. The GI-002 first-public-release trigger stays open (its two owed controls unchanged). | GI-012 (the wave-4 hook-ship precondition) | 2026-09-24, retroactively, by user ruling (AM-5 Q3) | Expires at the first `mochiko-cli-v*` publish with all four controls (the field review's wave-1 publish). **Expiry strike set:** the precondition clause on the region gates line · GI-012's two paragraphs — the closing first-publish sentence of the crate-release-train paragraph ("The two owed first-publish controls … gate the first publish under the GI-002 trigger recorded in the amendment policy above."; the crate tag-train rule in that paragraph stays) and the AM-3 precondition paragraph ("**From AM-3 (v3.1.0, review C5, user-ruled 2026-09-14) those two owed controls gate the wave-4 hook ship too:** … recorded in the log row.") — plus the AM-5 breach-marker paragraph before it ("**Breached at plugin 0.109.0 …**") · the amendment policy's first-publish paragraph (its "hard preconditions of the wave-4 hook ship" limb) · GI-012's Testability Pass and Fail limbs for the hook-ship bump · the region amend-triggers line's "those two are hard preconditions of the wave-4 hook ship" clause and its breach mark — struck or re-worded and the row closed, a PATCH recorded in the log row. **Tripwire:** anyone else starts installing mochiko before the publish → revisited at the next setup run. **Backstop:** still open on **2026-12-31** → put to the user at the next setup run (keep, change, or act). |

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
- Every shipped-primitive edit passes the independent author≠grader audit (a plain fresh seat
  running the rendered `mochiko:validation-primitive-edit` contract against the unit's own files)
  before the `plugin.json` version bump that ships it — the
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
  diff, graded by the primitive-edit gate seat on five criteria (intent stated · anchor present where
  required · ID lifecycle right · floor and fail survival · register), with the CLI's apply
  result as the deterministic pre-pass (record D6). The `.md`'s independent count self-check is
  **retired and booked as a loss** (record D3): the version triple confirms delivery, the CLI's
  printed counts assert completeness, and the contract suite tests it. The `primitive-edits.md`
  re-key landing this at wave 6 is a pre-authorized PATCH activation of this ruling, not a fresh
  amend run (review I2).
- **Author≠grader extends to code** (crate, wave-1 precedent): every unit of
  `crates/mochiko-cli` lands on a lead-approved plan with an independent non-author review.
  Touch-time carrier: `.claude/rules/mochiko/rust-cli.md`.
- *Editorial 2026-09-20 (v0.113.0):* grader identity re-pointed from the retired `validator`
  persona to a plain fresh seat running the rendered `mochiko:validation-primitive-edit` contract
  (`producer-plan-enforcement` D3 · `author-grader-consolidation` D7). The ratchet, the audit-unit
  keys, and the five schema-content criteria are unchanged; recorded as the PATCH 3.1.1 row; no
  fresh `/mochiko:setup` amend.

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
- *AM-4 (2026-09-23, v3.1.2):* the GLOSSARY.md deferral is **discharged** — the doc gained
  content at plugin v0.114.0 and joins the module's live operating docs under the pinned term
  format (`.mochiko/memory/knowledge-management.md`); the rules-file carrier already globs
  `GLOSSARY.md`.

**Testability**:
- Pass: KM invariants (bijection · status-agreement · open-only · caps · bounds · dead-pointer ·
  in-flight agreement · presence) hold at command boundaries.
- Fail: any invariant trips without a groom invocation.

**Rationale**: The operating-docs layer is this repo's live, proven governance backbone —
codified from working reality, not imposed.

**Trace**: GI-009 (module: knowledge-management-core; brownfield codification of the 2026-07-25
pin; AM-4 2026-09-23: GLOSSARY.md deviation discharged)

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

**Held bumps (AM-5, review R3):** a held version bump stays on its own branch; merging a branch
that carries a held bump, directly or inside another PR, is a breach.

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

**Breached at plugin 0.109.0 (on public `main` 2026-09-19, `2f92b09`) — excepted until the first
publish by the exception-registry row (AM-5); the paragraph below stands, marked, until that row
expires.**

**From AM-3 (v3.1.0, review C5, user-ruled 2026-09-14) those two owed controls gate the wave-4
hook ship too:** the `plugin.json` bump that ships the conformance hooks MUST NOT land before the
first publish with all four named controls in place, because from wave 4 the plugin's hooks
execute the author's code on every gated artifact write in a consuming project and once per
spawned seat, not only at command and skill fire — a maintainer break-glass install never
substitutes for consumers. Wave 4's bump also takes the contract suite with the new hook cases
(D10). Once the wave-4 bump has landed under this precondition, striking the clause from the
region gates line and these two paragraphs is a pre-authorized PATCH amendment, recorded in the
log row. *(Void at AM-5 — it can never fire; the exception row's expiry strike set replaces it.)*

**Gate note:** the behavioural read-back metric is **reported, never gating** (record D8) — it
informs the wave-3 pilot's abort criteria (GI-020's revisit trigger); it does not block a bump.
The same posture binds the **aggregate per-run hook cost cap (≤ 60 s, `hook-enforced-artifact-schema`
OQ4): a D10 watch by ruling, never a bump gate** (AM-3, review C9, user-ruled 2026-09-14) — a
wall-clock cap in the Docker contract suite is nondeterministic and gate 6 is the deterministic
set; the `${CLAUDE_PLUGIN_DATA}` replay cache (`cli-schema-delivery` D1, deferred on measured
need) is the named response if the budget trips.

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
- Pass: all six checks hold at the bump commit (gates 1–5 every bump; gate 6 `cargo test` PASS from v0.76.0, the crate having landed, plus the contract suite at exit 0 from v3.0.0); a `mochiko-cli-v*` tag has the four crate layers, the suite against the tagged binary, and an unchanged output shape or a coordinated plugin bump; the wave-4 hook-shipping bump lands only after a first publish carrying all four named controls — breached at 0.109.0, excepted (exception registry, AM-5). · Fail: any missing; a contract suite reported SKIPPED (exit 3) and treated as green; a crate tag landed on any of the three train conditions unmet; the wave-4 hook-shipping bump landed before that publish — **true since 0.109.0**; see the exception registry (AM-5); a held bump reaching `main` inside any merge.

**Rationale**: Releases are the only distribution moment; the gates codify the existing ritual
plus the one detected drift (marketplace lag) it silently permitted. From AM-2 the crate's
release is a second distribution moment of the same system — consumers run the binary — so it
carries its own train rather than riding the plugin's.

**Trace**: GI-012 (module: release-gates; GI-016 folded as gate 5; AM-1 dormant crate-gate clause **activated v0.76.0** — gate 6 `cargo test` + gate-5 schema/binary consistency; **widened AM-2 v3.0.0** — contract suite on gate 6, SKIPPED blocks, gate 5 = view ≡ replay, crate release train, ToS substrate mark `Contested`; **extended AM-3 v3.1.0** — the wave-4 hook-ship precondition (review C5) and the ≤ 60 s aggregate hook-cost watch (review C9, never a bump gate); driver `.mochiko/brainstorms/cli-schema-delivery/record.md` D8/D10.3 · `.mochiko/brainstorms/hook-enforced-artifact-schema/record.md` D10 · **AM-5 v3.2.0** — the wave-4 hook-ship precondition marked breached at 0.109.0 and excepted retroactively (registry row, expiring at the first `mochiko-cli-v*` publish with all four controls; tripwire + 2026-12-31 backstop), the AM-3 pre-authorized strike void, the held-bump rule (review R3); driver `.mochiko/brainstorms/hook-enforcement-field-review/record.md` S10 · the AM-5 synthesis Q3)

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
  never waived — they rest on that clause — and they retired into the crate's validator at wave 6
  (v0.107.0: `mochiko-cli migrate validate --report` is the advisory pre-pass) under the admission
  above. The six skill-shipped helpers under waiver GI-008 are untouched.
- **Admission ruling — mechanical conformance gates on artifact writes (AM-3, v3.1.0):**
  `hook-enforced-artifact-schema` (accepted 2026-09-13; D1/D7, wave-0 probe PROCEED) is the
  recorded admission for two plugin-shipped hooks beyond the dependency halt: **(1)** a
  `PreToolUse` gate on `Write|Edit|Bash|PowerShell` that pipes the payload to `mochiko-cli check
  --hook-json -` and denies a write failing a **mechanical** check — path against a declared
  home's pattern, with a path under `<root>/.mochiko/` that resolves to no declared home denied
  (the closed world) · file name against the home's declared set, and a `runs/<run-id>/` name
  against the run-key form · required `##` headings in the declared order, with an undeclared `##`
  denied · required frontmatter fields and enum values · declared placeholder tokens ·
  per-section and per-entry line budgets · a `.md` opening with report frontmatter under
  `.mochiko/runs/` or the declared `prototype/`, `inputs/`, `research/`, `referents/`
  sub-directories (the report sniff) · a shell command placing a declared-home path in a write
  position (`hook-enforcement-field-review` record D5) · on a `runs/` write, the ignore guard and
  the worktree test; **(2)** a `SubagentStart` hook injecting one self-identified
  reminder line per seat (D1b as amended 2026-09-13, superseding the `Read`-time form). Every
  check is decidable by string and count against data the log carries and the repository's own
  layout, with no judgment — the `home` document kind and each template's conformance block, plus
  yes/no facts about the repository: where its tree root is (the nearest ancestor holding `.git`),
  whether `.gitignore` carries `.mochiko/runs/` (the ignore guard), and whether a path sits under a
  git worktree (read from its `.git` pointer file). **Narrow supersession:**
  `cli-schema-delivery` D7's "behavior-gating hooks are declined" stands for **judgment and sequencing**; the gate holds one
  write until it conforms — or, over an existing file already failing a relaxable measure, until
  the write does not worsen that measure — and a passing draft pays nothing.
- **Stated limits of the gate** (`hook-enforced-artifact-schema` record § Build trail; AM-5):
  repo-root writes uncaught · three shell shapes evade the write-position parse · `PowerShell`
  routing unverifiable on macOS · hooks-disabled projects keep the procedural ceremony only.
- **The bright-line argument, clause (iv) (D1/D4):** mechanical conformance is structural validity
  of an artifact against the store's own declared shape — the same class as clause (ii)'s hard
  constraints at migration apply, a compiler on its own language, applied at write time to the
  artifact the shape governs. The repository facts the gate consults — the tree root, the ignore
  guard, the worktree test — are yes/no facts about the repository's layout, not grades,
  sequencing, or skill-owned judgment: the bright line's purpose is to keep the gate from judging
  or ordering work, not to confine where it reads plain facts (AM-5, R1). It is not a grade: no
  check reads meaning, ranks quality, or judges adequacy — those stay with the review skills and the author≠grader ceremony. It is not
  sequencing: the gate names no seat, no order, no stage. It is not a pipeline gate: a denied
  write is re-emitted, the run continues, and the two-strike halt sentence is advisory (D9).
  **Condition (review C1, user-ruled 2026-09-14, road (a)) — DISCHARGED 2026-09-15: the budget table was ratified by the user with the C1 amendment (record/synthesis unbounded · template-less pipeline deliverables and wave files 300 · entry-class 150 · `build-log.md` a log at 60 per entry); status flip transcribed at the wave-3 close and carried into the amendment log at AM-5 (the 3.2.0 row, item (e)); the pre-authorized PATCH meant to carry it (re-keyed v3.1.1 → v3.1.2 → v3.1.3) folded into AM-5 and its number retired unminted. Field result (AM-5, 2026-09-24): the ratified table did not admit honest content — nine per-run whole-file denies across two kinako runs, all on feature-home and desk files (`baseline-delta.md`, the feature `constraints-and-decisions.md`, `architecture.md`, `plan.md`, a desk `derivation.md` — `hook-enforcement-field-review` record F2), and product baselines already far over the `product` home's 300-line bound that no seat could grow at landing, so run 3's folds landed through the user's own shell and run 4's stayed owed (record F4); re-keyed at the field review's wave 2 as per-entry budgets (record D2; the census stands, the fold shape superseded 2026-09-24 by `delta-files-vs-direct-baseline-edits` D1/D6a). As written at AM-3:** the clause holds on a
  user-ratified budget table (D6, wave 3's gate) that admits honest content. A bound honest
  content cannot meet is a table defect, corrected by ruling at the table and never by the hook;
  record OQ1 — whether a prose-shaped deliverable carries a budget or `max_lines: none` — is ruled
  there, not here. The review's measurement is carried to that ruling: the working-tree
  migration's 150-line whole-file bound on `record.md` against 43 of 66 brainstorm records in this
  repo exceeding it (largest 1,536; this amend's own driver 738), with no in-run route for a fresh
  over-budget deliverable — amnesty needs a baseline, `reports/` takes only report-typed content,
  and a new kind takes a migration and a plugin release. The declined road (b) — whole-file bounds
  advisory at create time, deny reserved for templated per-section checks — would amend D1/D6 in
  the brainstorm record and was not taken.
- **First-touch amnesty (D4e, as corrected at the verify pass against the built binary; file-set
  limb corrected at AM-5 on the wave-4 measurement):** on any write over an existing file, `Write`
  and `Edit` alike, the on-disk file is the baseline. Where the baseline already fails a
  **relaxable** measure — file set (an undeclared name in a declared home), shape (headings ·
  frontmatter · placeholders), or budget — a write that does not worsen that measure is allowed;
  **path is not a relaxable measure** — inside `<root>/.mochiko/` a write resolving to no declared
  home is denied whether or not the file exists (the closed world, Reach below); outside it a
  `.md` write is gated only when its content opens with mochiko report frontmatter or a template's
  `## Header` signature. The binary names a standing file-set,
  shape, or budget violation in `additionalContext`. **Known gap at AM-3, corrected at wave 4
  (v0.109.0):** the AM-3 verify pass read an `Edit` over an existing undeclared name as a bare
  allow; measured at wave 4, the pre-fix binary denied both a `Write` and an `Edit` over an
  existing undeclared name at exit 4 — no allow existed — and the file-set limb was built at wave 4
  as ratified: an existing undeclared name is editable with the name in `additionalContext`, a new
  undeclared name still denies. An existing file in a declared home is never wedged, and the
  rewrite that fixes one is itself an allowed non-worsening write.
- **Reach of the gate (D9, review C4; re-worded at AM-5 for the closed world —
  `hook-enforcement-field-review` D3/D4/D5/D7 as amended):** homes resolve against the tree root —
  the nearest ancestor holding `.git`, a directory or a worktree's pointer file — and only
  `<root>/.mochiko/` is a home tree; a nested `.mochiko/` (fixtures, eval workspaces) is never a
  home. Inside `<root>/.mochiko/` the world is closed: a write resolving to no declared home is
  denied, and the deny names the run folder. `.mochiko/runs/<run-id>/` is the raw-output home: its
  name must match the run-key form; a write there is refused unless `.gitignore` carries
  `.mochiko/runs/`; the report sniff refuses a `.md` there, and in the declared `prototype/`,
  `inputs/`, `research/`, and `referents/` sub-directories, when it opens with report frontmatter;
  shell writes are admitted there for non-`.md` targets only; the folder is always the main
  tree's, so a write to a worktree's own `.mochiko/runs/` is refused with the main tree's path;
  deletion is procedural, a landing step at acceptance. The shell leg treats a home path as a
  write target only in a write position. Outside `<root>/.mochiko/` a `.md` write is gated only
  when its content opens with mochiko report frontmatter or a template's `## Header` signature — a
  plain `.md` elsewhere is not mochiko's business. `NotebookEdit` and any MCP file writer a
  consumer enables are knowingly outside the matcher set; escapes past that check and hook-disabled
  consumers are the reviewer's under the author≠grader ceremony. The gate is a floor, not the
  whole fence.
- **What the gate reads (review C7; widened at AM-5):** the raw `PreToolUse` payload —
  `tool_input.content`, `old_string`/`new_string`, `command` — and, on `Edit`, the on-disk file,
  to apply the edit in memory; the ancestor walk for `.git` that finds the tree root; on a `runs/`
  write, `.gitignore`; and a worktree's `.git` pointer file, to name the main tree's run folder. It
  reads them against the log the plugin carries. Nothing leaves the machine.
- **Build state at AM-5 (2026-09-24) — the admission's check list and the amnesty, reach and reads
  paragraphs above read ahead of the binary** (user-ruled road B, `Contested` — chosen for
  simplicity against the lead's recommended dormant-clause road, the trade-off put). At
  ratification the field-review wave 1 is unbuilt, and the shipped binary (`mochiko-cli` 0.2.0)
  behaves per the AM-3 text — an undeclared path under `.mochiko/` passes, home resolution is
  cwd-relative, the shell leg scans by substring — until the field-review build ships; the AM-3
  wording is in git history at `8f4e5ab`. **Strike trigger:** this line and the
  `.claude/rules/mochiko/rust-cli.md` bright-line build-state pointer are struck at the
  `plugin.json` bump that carries both the wave-1 binary range and the wave-2 home migration
  (`runs/`, `archive/`, `strips/`, `schema-views/` declared), never at the wave-1 tag alone; that
  strike PATCH checks the re-worded paragraphs against what was built and corrects any drift in the
  same amendment-log row.
- **Hook floor, re-ratified for the new hooks (D7c):** a 5-second `timeout` on every shipped hook;
  fail-open when a hook cannot run or times out; hooks ship to every consuming project and execute
  the plugin author's code on every gated call — ratified knowingly. **Explicit-allow rule (D3/C2,
  wave-0 platform fact):** the platform denies a background subagent's call when no hook returns a
  decision, so the wrapper always exits 0 and emits an explicit `permissionDecision: allow` on
  every non-deny path, including CLI exit 1 (log unsound), 2 (usage, or a binary predating
  `check`) and 3 (grammar skew). A conformance verdict — `check` exit 4, `EXIT_CONFORMANCE` in
  `crates/mochiko-cli/src/hook.rs` — is the only deny; the dependency halt stays the existing
  hooks' job. The exit-code contract and the `check --hook-json -` interface are `Assumed` (record
  D3 — derived from GI-020 and `cli-schema-delivery` D1, never put to the user as a fork); the
  exit-4 fact was verified against the built crate at this amend.
- **Scope of the guarantee (D7e, reconciled with GI-020 at Card 5):** the gate is a floor for
  consumers who keep the plugin's hooks enabled. A project that sets `disableAllHooks`, or an
  enterprise under `allowManagedHooksOnly`, stays **outside GI-020's supported set** — D7e states
  what remains there (the procedural author≠grader ceremony), it grants no support; a ratified
  consequence, not a defect. The environments GI-020 already declares unsupported are unchanged;
  the `PowerShell` matcher arm ships on the doc quote, unverifiable on macOS (wave 0).

*Recorded supersession — `.claude/rules/mochiko/rust-cli.md`'s bright-line bullet (D7d, Card 4).*
The bullet was rewritten at AM-3 and its prior text is preserved here so the change is
reconstructible (GI-006); no `.mochiko/strips/` entry, because that file is a governance surface,
not a plugin primitive — AM-2's full rewrite of the same file took none. *The prior bullet read:*

> - **Bright line (GI-019).** The tool renders, replays, and validates its own data. It MUST NOT
>   grade an artifact, MUST NOT dispatch or sequence agents, and MUST NOT hold judgment that skills
>   own. Its hooks MUST block only on the binary's absence or a log outside its grammar range,
>   never on behavior. Home: CLAUDE.md `## Non-negotiable constraints`; detail: ledger GI-019.

The two superseded clauses are "MUST NOT grade an artifact" and "hooks MUST block only on the
binary's absence or a log outside its grammar range, never on behavior": "validates an artifact's
mechanical conformance against its own data" is the admitted reading, and grading of meaning or
quality stays forbidden.

*Re-worded again at AM-5 (R1).* The AM-3 bullet read:

> - **Bright line (GI-019).** The tool renders, replays, and validates its own data — including
>   an artifact's **mechanical conformance** to the shape the log declares for its home (path ·
>   file set · headings · frontmatter · placeholders · per-section size; `mochiko-cli check`,
>   AM-3). It MUST NOT grade an artifact's meaning or quality, MUST NOT dispatch or sequence
>   agents, and MUST NOT hold judgment that skills own. Its hooks block on exactly two grounds:
>   the binary's absence or a log outside its grammar range (the dependency halt), and a
>   conformance deny from `check` (exit 4) — never on behavior or judgment; every other outcome
>   is an explicit `allow`. Home: CLAUDE.md `## Non-negotiable constraints`; detail: ledger GI-019
>   (clause iv).

The change adds the repository-layout facts to what `check` decides against, and per-entry size and
the closed-world path to the conformance list; the MUST NOT clauses are unchanged.

**Testability** — two tiers, in the GI-020 idiom: the conformance tier below is active from AM-5.
**Assertable at ratification (2026-09-14):**
- Pass: every kernel-class component in the tree traces to a recorded admission ruling and
  satisfies the bright-line clauses; every shipped hook blocks only on the grounds the conformance
  tier names (re-pointed at AM-5 (a)); `mochiko-cli check` exists and returns exit 4 on a
  conformance verdict (`EXIT_CONFORMANCE`, `crates/mochiko-cli/src/hook.rs`, verified against the built crate
  this run). · Fail: an admitted binary that gates the pipeline, sequences agents, or holds
  skill-owned judgment; a kernel-class component with no admission ruling; a shipped hook that
  blocks on anything other than the grounds the conformance tier names (re-pointed at AM-5 (a)).

**Conformance tier — active from AM-5 (v3.2.0, 2026-09-24):** the hooks are live from plugin
0.109.0 (`hooks.json`: `artifact-gate.sh` on `Write|Edit`, `Bash`, `PowerShell`;
`seat-reminder.sh` on `SubagentStart`); tested by the contract cases `gate-input` ·
`reminder-input` · `if-placement` · `gate-live` · `reminder-spawn` (`evals/contract/run.py`) and
the crate matrices (`crates/mochiko-cli/tests/{home,conform,hook}.rs`). Dormant at AM-3 until the
wave-4 hook ship; activated by AM-5 Q1(a), the pre-authorized PATCH folded in.
- Pass: every shipped hook blocks on exactly two grounds — the binary's absence or a log outside
  its grammar range, and a conformance deny from `check` — with every other outcome an explicit
  `allow`; `check` reads no meaning and names no seat, order, or stage, and reads beyond the
  payload, the on-disk target and the log only the repository-layout facts the admission names. ·
  Fail: a hook blocking on any other ground; a `check` that grades meaning or quality; a hook that
  sequences; a `check` reading any other repository state.

**Rationale**: Skills and agents are the primary quality surface; an unbounded kernel is the exact
failure mode mochiko was built against. The bright line keeps admitted tooling to
delivery/composition roles, never the judgment or orchestration skills own. Softened from the
v1.0.0 absolute no-kernel position per the D11 ruling (evidence basis n=0 — the recorded concession
is that template delivery alone would not carry the CLI; the machine rides the foundation bet).

**Trace**: GI-019 (minted at AM-1; driver: `.mochiko/brainstorms/schema-based-template-guidance/record.md` D11 · **widened admission: `cli-schema-delivery` D11 (AM-2, v3.0.0)** — bright-line text unchanged, the three clauses recorded as its argument; AM-1's template-scope limb discharged, the no-general-kernel and no-orchestration limbs standing; **conformance-gate admission: `hook-enforced-artifact-schema` D1/D7 (AM-3, v3.1.0)** — bright-line text unchanged, clause (iv) recorded with its C1 budget-table condition discharged 2026-09-15 by the ratified table; `cli-schema-delivery` D7 narrowly superseded, its judgment/sequencing decline standing; `.claude/rules/mochiko/rust-cli.md`'s bright-line bullet superseded by ruling, prior text preserved above · **AM-5 (v3.2.0)** — check-source predicate widened to the repository's own layout with no judgment (R1), check-kind list and clause (iv) argument body with it; Reach / amnesty / reads re-worded for `hook-enforcement-field-review` D3–D7 as amended, stated as the gate with a build-state line (Q2 `Contested`, struck at the wave-1 + wave-2 bump); C1 note gains the field result; amnesty known-gap corrected (wave-4 measurement); stated-limits pointer; Testability conformance tier activated; `rust-cli.md` bright-line bullet re-worded, AM-3 text preserved above; drivers `.mochiko/brainstorms/hook-enforcement-field-review/record.md` · `.mochiko/brainstorms/hook-enforced-artifact-schema/wave5-bump-patch.md` (a, c, d, e))

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
- **Transition clause — expired at v3.0.3 (2026-09-05, plugin v0.107.0, the wave-6 end state):**
  from v3.0.0 until the wave-6 landing, primitives not yet re-pointed read the derived snapshot
  files shipped in the plugin. No schema file ships now — `plugins/mochiko/schemas/` and every
  `skills/*/schema.yaml` are deleted, the human-readable views live repo-side at
  `.mochiko/schema-views/` — and the contract suite's run-wide no-Read assert holds it. The expiry
  was the pre-authorized PATCH (amendment policy above), recorded in the log row.
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
halt waves 4–5 and return the posture to the user. **Evaluated 2026-09-04 (wave 3): not tripped**
(read-back 3/3 at the 3/3 bar; 10,839 bytes delivered against the 12,819-byte baseline).
**Tripped at wave 4 (2026-09-04)** on `implement` 1/3 · `setup` 1/3 · `specify` 0/3 with delivery
verified on every transcript; the pre-registered diagnostic (record, wave-4 diagnostic section)
showed the exact-id read-back measured probe compliance and long-list recall, not delivery.
**Re-keyed at v3.0.2, user-ruled:** criterion (1) is now the deterministic assert that every
`class: floor` id of the converted primitive is present verbatim, with its class line, in the
transcript the model read (gating in the contract suite); the read-back is reported, never
gating (D8's own words); criterion (2) unchanged. The trigger stays live for wave 5 per
converted primitive on the re-keyed terms. Reversal after this ratification costs a
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
| 3.0.2 | 2026-09-04 | PATCH — the GI-020 revisit trigger re-keyed after the wave-4 trip (user-ruled "re key", record wave-4 + diagnostic sections): criterion (1) becomes the deterministic floor-delivery assert (every `class: floor` id verbatim in the transcript), the exact-id read-back reported never gating; wave 5 opened on those terms. In the same row: `.claude/rules/mochiko/primitive-edits.md`'s skill-pair criteria gain five one-sentence converted-skill clauses (criteria 1, 2, 3, 6, and 8's budgeted quantity — the sibling of the v3.0.1 command clauses) as wave 5 converts the first skill, and the budget ledger re-seeds every converted skill to body + rendered blocks per record D10.6. Mints no principle; no fresh `/mochiko:setup` amend | GI-004 · GI-020 |
| 3.0.3 | 2026-09-05 | PATCH — the wave-6 landing (pre-authorized at AM-2): the GI-020 **transition clause expired** at plugin v0.107.0 — no schema file ships, the derived views live at `.mochiko/schema-views/`, the contract suite's run-wide no-Read assert holds it; `.claude/rules/mochiko/primitive-edits.md` re-keyed (schema content edits are migration files and take no strip; criterion 9's deterministic pre-pass is `mochiko-cli migrate validate --report`; anchors live on the log's rules, the provenance sidecar frozen at `.mochiko/archive/`; the converted-primitive clauses are now the only form); `rust-cli.md`'s clause struck; the three first-live-run delivery watches closed to the trail as superseded by record D10.6; the three Python checkers retired (GI-008's waived helper-script count falls). Mints no principle; no fresh `/mochiko:setup` amend | GI-004 · GI-005 · GI-008 · GI-020 |
| 3.1.0 | 2026-09-14 | AM-3 — mechanical conformance gates on artifact writes (MINOR — a widened admission under an unchanged non-negotiable; user-ruled Card 1 with the MAJOR reading in view, re-put at review C6 and re-ruled MINOR as a **recorded departure** from the AM-1/AM-2 MAJOR precedents: those two redefined a principle, AM-3 widens an admission under a principle whose text does not change, and the GI-019 Testability inversion is read as re-keying a formulation narrower than the principle; the MINOR limb gains "principle significantly expanded" at this amend so policy and the validator's bump grammar agree). Driver: hook-enforced-artifact-schema D1–D11, wave-0 probe PROCEED, wave-1 crate built and accepted 2026-09-13 | GI-019 admission widened (clause iv: mechanical conformance ≠ judgment, its C1 budget-table condition standing open) · `cli-schema-delivery` D7 narrowly superseded, its judgment/sequencing decline standing · hook floor re-ratified + explicit-allow rule + first-touch amnesty + reach + what the gate reads + hooks-disabled scope reconciled with GI-020 · GI-019 Testability re-keyed · GI-012 gains the wave-4 hook-ship precondition (review C5) and the ≤ 60 s hook-cost watch (review C9) · amendment-policy MINOR limb and first-publish paragraph extended · `.claude/rules/mochiko/rust-cli.md` bright-line bullet superseded by ruling, prior text preserved in GI-019 · **fix round (validator B1/M1/M2):** GI-019 Testability re-keyed into two tiers, the conformance limb **dormant until the wave-4 hook ship** and activated at that `plugin.json` bump as a pre-authorized PATCH (the v2.0.1 / v3.0.1 / v3.0.3 idiom) · the wave-4 precondition's strike, once that bump has landed under it, is itself a pre-authorized PATCH recorded in the log row (M2, at both ledger homes) · `rust-cli.md` `paths` += `plugins/mochiko/.claude-plugin/plugin.json` · `.claude-plugin/marketplace.json` · `CHANGELOG.md` (FP-5, B1) |
| 3.1.1 | 2026-09-20 | PATCH — the `validator` persona retired at plugin v0.113.0 by ruling (`producer-plan-enforcement` D3 · `author-grader-consolidation` D7, that path's wave 3; the row itself user-ruled): GI-004's two detail lines re-point the grader identity from the persona to a plain fresh seat running the rendered `mochiko:validation-primitive-edit` contract, an editorial note beside them; the ratchet, the audit-unit keys and the five schema-content criteria are unchanged. Mints no principle; no fresh `/mochiko:setup` amend (the v3.0.1–v3.0.3 idiom) | GI-004 |
| 3.1.2 | 2026-09-23 | AM-4 — GI-009's carried `GLOSSARY.md` deviation discharged (PATCH — a recorded deviation discharged, no principle added, removed, or redefined; user-ruled). Driver: the standing amend trigger "GLOSSARY.md gains content" fired when `impeccable-design-integration` build item 10 scaffolded `GLOSSARY.md` with eight terms at plugin v0.114.0 (`18b533c`); stress test waived by user ruling (recorded in the synthesis) | GI-009 deviation discharged — `GLOSSARY.md` a live operating doc under the pinned term format · KM invariants' deferral clause + revisit trigger retired · standing amend trigger struck (amendment policy · region governance-operations line) · region operating-docs line gains `GLOSSARY.md` (FP-1, user-ruled at acceptance) · the hook wave's owed pre-authorized PATCH re-keyed v3.1.2 → v3.1.3 (GI-019 clause-iv pointer) |
| 3.2.0 | 2026-09-24 | AM-5 — the hook field review folded into GI-019, the owed hook-ship PATCH folded with it, the 0.109.0 publish-gate breach recorded as a GI-012 exception (MINOR — the exception excuses a MUST NOT for a bounded window, waiver-class; the gate's reach and reads widen, "principle significantly expanded"; user-ruled Q4, PATCH v3.1.3 rejected). Drivers: `hook-enforcement-field-review` D1–D9 as review-amended (accepted 2026-09-23; superseded in part 2026-09-24 by `delta-files-vs-direct-baseline-edits` D1/D6a — nothing carried rests on those parts) · the owed PATCH at `.mochiko/brainstorms/hook-enforced-artifact-schema/wave5-bump-patch.md`. Cold intent review solo (critical-gaps, 9 survivors folded, verify CLEAN on blocking); synthesis ratified 2026-09-24 | GI-019: predicate widened (R1) with check list + clause (iv) body · Reach / amnesty / reads re-worded, build-state line (Q2 `Contested`, strike PATCH pre-ruled) · C1 field result · amnesty known-gap corrected (c) · stated-limits pointer (d) · Testability conformance tier activated (a) · `rust-cli.md` bright-line bullet aligned, AM-3 text preserved · GI-012: precondition marked breached with pointers, held-bump rule (R3), Testability pointers · amendment policy first-publish paragraph points at the exception · region gates line (`CLAUDE.md` release-gates line) and amend-triggers line marked breached with pointers · exception registry first row (expiry strike set, tripwire, 2026-12-31 backstop; the amend-triggers strike-set item (FP-1, user-accepted at acceptance)) · **(e) carried:** the `DECISIONS.md` 2026-09-13 row ("ratified 2026-09-15 with the C1 amendment") and GI-019 clause (iv)'s condition + trace ("DISCHARGED 2026-09-15"), both transcribed 2026-09-15 under the KM status-agreement invariant · **v3.1.3 retired unminted** (AM-3 pre-authorized only a and b; c, d, e ruled at AM-5 Q1; b replaced by the exception) · **the AM-3 pre-authorized precondition strike void** (it could never fire) · fact profile, modules, floor, waivers, depth `high` unchanged; no principle minted |

*AM-3 addendum (2026-09-14):* outside the governance region, and under the Card 2 ruling ratified
2026-09-14, the `## Non-negotiable constraints` kernel-class paragraph's trace parenthetical in
`CLAUDE.md` was extended with the conformance-gate admission pointer (GI-019). The GI-020
paragraph and every other line outside the markers are untouched. Mints no principle. Three
further touches are disclosed rather than silent: the amendment policy's MINOR limb gained
"principle significantly expanded" — a lead addition outside the deck of 8, disclosed in the
synthesis's AM-3 Scope bullet (verify R3) — so this ledger and `validation-constitution`'s
version-bump grammar, which already reads that wording, agree; GI-012's gate note gained the
≤ 60 s aggregate hook-cost watch (review C9), the second GI-012 touch beyond Card 7's C5 clause,
authored as a producer-flagged proposal and ruled at the acceptance gate; and
`.claude/rules/mochiko/rust-cli.md`'s log-is-truth bullet had its bare `(record D1/D2/D6)`
qualified to `(record `cli-schema-delivery` D1/D2/D6)`, a lead-ruled formulation touch outside
Card 4's literal scope, because this run's header rewrite introduces a second driver record whose
own D1 would otherwise be the nearer reading. At the validator's blocking finding B1,
`.claude/rules/mochiko/rust-cli.md`'s `paths` gained three globs —
`plugins/mochiko/.claude-plugin/plugin.json`, `.claude-plugin/marketplace.json` and
`CHANGELOG.md` — as a producer-flagged proposal ruled at the acceptance gate (FP-5, the AM-2
FP-1/FP-2 precedent): every GI-012 gate fires at the `plugin.json` bump and AM-3 puts the set's
strictest MUST NOT there, and `CHANGELOG.md` is GI-010's home, so a globs-honest reading has to
reach all three. Mints no principle.

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
