# Governance Ledger

**Governance Floor:** production (asserted) · **Modules:** compliance: none (GI-001 negatives confirmed, incl. no-UI) · template: knowledge-management (core + CHANGELOG elective) · release-gates · **Trace:** GI-001 (fact profile)
**Version:** 1.0.0 (must match the region stamp)

## Waivers

Any asserted standard, with a recorded justification (D4); permanent pending the D4.1 revisit.
**Legal-mandate module obligations are unwaivable (D4.2)** — a waiver row naming one is a
validator FAIL.

| Standard | Justification | Revisit trigger (optional) | Trace |
|----------|---------------|----------------------------|-------|
| FLOOR-TEST as applied to the 6 helper scripts (1 bash, 5 python) | Thin standalone validators/detectors; no shared deps; test/lint infrastructure absent and not worth erecting for them today | Script count grows, or a script becomes load-bearing in a shipped flow (lead-composed, user-ratified 2026-08-06) | GI-008 |

*(FLOOR-SEC's secret-scanning clause is a recorded **narrowing**, not a waiver — see GI-003 below.)*

## Amendment policy

- Route: `/mochiko:setup` amend mode; fact-profile changes (module attach/detach) and
  un-waives are governance events.
- Semver: MAJOR — principle removal / incompatible redefinition / floor-level change / module
  attach or detach · MINOR — new principle or waiver change · PATCH — clarification.
- Approvers: Deepesh (solo maintainer — GI-002 team reality; no code-review lean, enforcement is
  procedural gates + audits).
- Standing amend triggers recorded in the synthesis: public-product transition (GI-002 —
  compat obligations) · CI arrives (GI-003 — un-narrow secret scanning) · GLOSSARY.md gains
  content (GI-009 deviation) · helper-script waiver trigger (GI-008).

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
- Secret-scanning clause **narrowed by ruling** (not waived): gitignore + pre-commit vigilance;
  no CI obligation while no CI exists. Un-narrowing is a governance event (amend) when CI
  arrives.

**Testability**:
- Pass: `git ls-files | grep settings.local.json` empty; `git check-ignore .claude/settings.local.json` matches; no grep hit for token patterns in tracked files.
- Fail: any tracked file carries a live credential, or the gitignore entry is removed.

**Rationale**: A live `ANTHROPIC_AUTH_TOKEN` sat one `git add -A` from public history (detected
this run). A prose repo has exactly one secret class — local tool credentials — and one cheap,
total control.

**Trace**: GI-003 (floor-asserted: FLOOR-SEC; narrowing recorded; fold of GI-015)

### GI-004 — Primitive Audit Ratchet · home: CLAUDE.md region line

**Enforcement**:
- Every shipped-primitive edit passes the independent author≠grader audit (`mochiko:validator`
  against the primitive's own text) before the `plugin.json` version bump that ships it — the
  existing ceremony at `.claude/rules/mochiko/primitive-edits.md`, referenced not restated
  (GI-017).
- Helper scripts carved out by the GI-008 waiver above.

**Testability**:
- Pass: each bump's landed edits have audit PASS on record (session record or decision row).
- Fail: a bump ships a primitive edit with no audit trail.

**Rationale**: Prose has no coverage percentage; the audit IS the test suite. Translation of
FLOOR-TEST's ratchet — the baseline (all shipped primitives audited) MUST NOT decrease.

**Trace**: GI-004 (floor-asserted: FLOOR-TEST; expression translated; coverage/smoke subsumed
inapplicable per GI-007)

### GI-005 — Record-Layer Integrity · home: CLAUDE.md region line

**Enforcement**:
- Protected content leaves only by recorded ruling (strips/supersession) — the existing
  primitive-edit ceremony and KM landing ritual, referenced not restated (GI-017).
- Dead pointers are defects: the KM dead-pointer scan
  (`.mochiko/memory/knowledge-management.md` invariants) runs at command boundaries under
  fix-on-sight.

**Testability**:
- Pass: every `ROADMAP.md`/`DECISIONS.md`/`BACKLOG.md` pointer resolves or carries the
  `provenance: unrecoverable` terminal stamp; no silent deletion of protected lines.
- Fail: a broken pointer without the stamp, or a protected line gone with no strip entry.

**Rationale**: This repo's failure mode isn't runtime corruption — it's provenance corruption.
Translation of FLOOR-ERR: the record layer is the data that must never silently corrupt.

**Trace**: GI-005 (floor-asserted: FLOOR-ERR; expression translated; runtime clauses subsumed
inapplicable per GI-007)

### GI-006 — Traceability as Observability · home: CLAUDE.md region line

**Enforcement**:
- Every primitive edit reconstructible from the record layer: strips ledger
  (`.mochiko/strips/`) + `DECISIONS.md` + version stamps — existing carriers, referenced not
  restated (GI-017).

**Testability**:
- Pass: for any shipped primitive line, its origin (decision row, strip entry, or session
  record) is findable; strip entries carry version stamps.
- Fail: a primitive change whose provenance cannot be reconstructed.

**Rationale**: No logs exist; the audit trail is the observability surface. Translation of
FLOOR-OBS.

**Trace**: GI-006 (floor-asserted: FLOOR-OBS; expression translated; log/health clauses subsumed
inapplicable per GI-007)

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
   0.10.0-vs-0.53.0 lag is the tracked defect this gate exists to close (GI-016).

Environments: none — nothing deploys; distribution is the Claude Code marketplace ·
Cadence: manual, at `plugin.json` bumps (synthesis real-commands table).
Rollback: `git revert` of the bump commit + marketplace metadata re-sync — executable by the
solo maintainer; no time-bound SLO (no operated service).

**Testability**:
- Pass: all five checks hold at the bump commit. · Fail: any missing.

**Rationale**: Releases are the only distribution moment; the gates codify the existing ritual
plus the one detected drift (marketplace lag) it silently permitted.

**Trace**: GI-012 (module: release-gates; GI-016 folded as gate 5)

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
