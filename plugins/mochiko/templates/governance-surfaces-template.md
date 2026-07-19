<!--
GOVERNANCE SURFACES TEMPLATE — the canonical shapes of the dissolved constitution
=================================================================================
There is NO constitution.md (design: .mochiko/brainstorms/constitution-native-surfaces/record.md,
D1–D8). Governance is a SET, authored by `authoring-constitution` and graded as one deliverable
by `validation-constitution`:

  1. A marked governance region in CLAUDE.md          (always-on; short-form only)
  2. `paths`-scoped rules files under .claude/rules/mochiko/   (scope-on-touch)
  3. Skill pointers                                    (procedure lives in skills)
  4. The governance ledger at .mochiko/memory/governance-ledger.md  (setup/amend + validator only)
  5. The trace summary manifest                        (the validator's grading surface)

Ownership (D8): the region between the markers — and everything under .claude/rules/mochiko/ and
the ledger — is setup-owned and idempotently REGENERATED on re-runs and amends. Content outside
the markers is user territory: NEVER touched. HTML comments in CLAUDE.md are stripped before
context injection (doc-confirmed) — trace stamps there are context-free; comment handling in
rules files is undocumented, so the ledger is always the canonical metadata record.
-->

# Shape 1 — the CLAUDE.md governance region

Regenerated in place between the markers. Everything short-form: one line per entry, detail in
the ledger, module detail behind pointers.

```markdown
<!-- mochiko:governance:begin -->
## Governance

**Ratified:** v[X.Y.Z] · [YYYY-MM-DD] · tier: [poc|internal|production|regulated] <!-- GI-TIER -->

### Principles

<!-- Universal principles: the operative line IS the governance. Scope-bound and
     procedure-shaped principles: the line is an index entry pointing at the home. -->
- [Imperative universal principle, RFC 2119, one line] [(NON-NEGOTIABLE) where the tier mandates] <!-- GI-XXX -->
- [Imperative universal principle] <!-- GI-XXX -->
- [Concern name] — see `.claude/rules/mochiko/[file].md` <!-- GI-XXX -->
- [Procedure name] — follow skill `[skill-name]` when [trigger] <!-- GI-XXX -->

### Technology stack

- [Language/runtime + version] · [framework] · [key mandated choices, one line each] <!-- GI-XXX -->

### Quality gates

- `[actual command]` MUST pass before merge <!-- GI-XXX -->
- Coverage ≥ [tier-parameterized]% on new code (`[actual command]`) <!-- GI-XXX -->

### Governance operations

- Ledger (waivers · amendment policy · exceptions · Three-Part metadata): `.mochiko/memory/governance-ledger.md`
- Amend via `/mochiko:setup` (tier bumps and un-waives are governance events)
[- Path-scoped rules inject on **Read**, not Write (observed behavior, kinako dogfood 2026-07-19) — before creating a new file under [the actual scoped paths, e.g. `src/`, `tests/`], read the matching `.claude/rules/mochiko/` file or read back the file you created <!-- standing line: emit whenever the set includes any rules file -->]
[- Operating docs (knowledge-management module): sessions in `.mochiko/brainstorms/` + `index.md`; rulings graduate to `ROADMAP.md`; threads land in `BACKLOG.md` <!-- GI-XXX -->]
[- Release gates: [one-line summary] — detail in the ledger <!-- GI-XXX -->]
<!-- mochiko:governance:end -->
```

# Shape 2 — a scope-bound rules file (`.claude/rules/mochiko/<concern>.md`)

One concern per file. The `paths` globs must be honest to the concern — and honest cuts both
ways: a glob that matches everything is a universal principle wearing a costume (those belong in
the region), while globs that stop at the mechanism's home layer silently exempt code that can
violate the concern. Cover **every path whose code can violate the rule** — including layers that
orchestrate the governed operation through ports/interfaces, not just the layer that implements
it (kinako dogfood 2026-07-19: storage invariants scoped to domain+infrastructure missed the
application layer's use cases persisting through ports).

```markdown
---
paths:
  - "[honest glob, e.g. src/api/**/*.py]"
  - "[second glob if the concern truly spans]"
---

# [Concern name] <!-- GI-XXX -->

- [Operative rule, RFC 2119] 
- [Operative rule]
- [Operative rule]

Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md`, GI-XXX.
```

# Shape 3 — the governance ledger (`.mochiko/memory/governance-ledger.md`)

Read by setup/amend runs and the validator only — never force-loaded into working sessions.

```markdown
# Governance Ledger

**Tier:** [tier] · **Graduation path:** [next tier + trigger] · **Trace:** GI-TIER
**Version:** [X.Y.Z] (must match the region stamp)

## Waivers

| Category | Waiving tier | Revisit trigger | Trace |
|----------|--------------|-----------------|-------|
| [floor category or "None."] | [tier] | [concrete trigger] | GI-XXX |

## Amendment policy

- Route: `/mochiko:setup` amend mode; tier bumps and un-waives are governance events.
- Semver: MAJOR — principle removal / incompatible redefinition / tier change · MINOR — new
  principle or waiver change · PATCH — clarification.
- Approvers: [from the synthesis's team reality]

## Exception registry

| Exception | Principle (GI-ID) | Granted | Expires/revisit |
|-----------|-------------------|---------|-----------------|
| [none yet] | | | |

## Principles (Three-Part metadata, keyed by GI-ID)

### GI-XXX — [Principle Name] · home: [CLAUDE.md | rules/mochiko/<file>.md | skill:<name>]

**Enforcement**:
- [How compliance is verified — specific commands or processes]

**Testability**:
- Pass: [criterion] · Fail: [criterion]

**Rationale**: [Failure mode prevented; success enabled.]

**Trace**: GI-XXX (deck-kept: CARD-ID | minted | floor-preset: CARD-ID)

[## Evolution notes (brownfield — the evolution-notes module's ledger section)]
[Floor status table · gap references · session confrontation rulings.]

## Amendment log

| Version | Date | Change | GI delta |
|---------|------|--------|----------|
| [X.Y.Z] | [date] | ratified | [elements] |
```

# Shape 4 — the trace summary manifest

Emitted by the producer with every authoring round; presented at the acceptance gate; the
validator's grading surface. One row per principle-bearing GI element:

```markdown
## Trace summary

| GI-ID | Principle | Source | Primary home | Companions present |
|-------|-----------|--------|--------------|--------------------|
| GI-001 | [name] | deck-kept: CARD-ID | CLAUDE.md region line | index ✓ · ledger ✓ |
| GI-002 | [name] | minted | rules/mochiko/api.md | index ✓ · ledger ✓ |
| GI-003 | [name] | floor-preset: CARD-ID | skill:mochiko:executing-tdd-cycle | index ✓ · ledger ✓ |

Flagged proposals: [none | list — each awaiting the user's ruling at acceptance]
Waivers: [none | GI-IDs]
```
