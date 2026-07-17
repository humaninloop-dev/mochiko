<!--
UNIVERSAL CORE TEMPLATE
=======================
This is the universal core every constitution shares. Type/tier-specific sections are
ATTACHABLE MODULES in `constitution-modules/` — the session synthesis
(`.mochiko/memory/governance-intent.md`) selects which modules attach, and the validator checks
core + exactly the selected modules (each module file carries its own checklist fragment).
Insert selected module sections where marked below. Do NOT inline module content for types
that didn't select it — that is the backend-bias failure this split exists to kill.
-->

<!--
SYNC IMPACT REPORT
==================
Version change: (none) → 1.0.0 (MAJOR: Initial constitution)

Modified principles: N/A (initial version)

Added sections:
- Governance Tier
- Core Principles
- Technology Stack
- Quality Gates
- [Selected modules, by name]
- Governance
- CLAUDE.md Synchronization

Removed sections: None

Configuration changes: N/A (initial version)

Templates requiring updates:
- CLAUDE.md: Requires sync after constitution creation ⚠️

Follow-up TODOs:
- Sync CLAUDE.md with constitution content

Previous reports: None (initial version)

INSTRUCTION: Maintain version history here. After each amendment, add the previous version to
"Previous reports" with a one-line summary.
-->

# [PROJECT_NAME] Constitution

## Overview

<!--
INSTRUCTION: Brief project description (2-3 sentences): what the project does, primary
technology, governance goals — sourced from the synthesis's identity element.
-->

[PROJECT_DESCRIPTION]

This constitution establishes enforceable development standards for [PROJECT_NAME]. It codifies principles that protect quality, security, and maintainability while enabling rapid, confident delivery.

## Governance Tier

<!--
INSTRUCTION: The declared scope tier from the synthesis (GI trace required). Strictness and
waiver posture throughout this constitution are calibrated to this tier.
-->

**Tier:** `[TIER]` — [one line: what this tier means for this project]
**Graduation path:** [next tier + trigger, or "none expected"]
**Trace**: GI-XXX

### Waivers

<!--
INSTRUCTION: One row per floor category waived at this tier — from the synthesis's waiver
elements. Delete the table and write "None." if nothing is waived. Waivers are only valid at
tiers whose posture permits them (see catalog/universal-floor.md). Accumulated waivers are the
governance re-entry checklist at tier graduation.
-->

| Floor category | Waiving tier | Revisit trigger | Trace |
|----------------|--------------|-----------------|-------|
| [CATEGORY] | [TIER] | [e.g. tier graduation] | GI-XXX |

---

## Core Principles

<!--
INSTRUCTION: One principle per principle-bearing synthesis element (deck-kept, minted, or
floor-preset). Principle count is driven by the synthesis, not a fixed quota. Each principle
MUST have Enforcement / Testability / Rationale, RFC 2119 keywords, and a Trace stamp naming
its synthesis source. Selection belongs to the synthesis; only FORMULATION happens here.
-->

### I. [PRINCIPLE_1_NAME]

[PRINCIPLE_1_DESCRIPTION — declarative constraint with RFC 2119 keywords, then specific rules]

**Enforcement**:
- [How compliance is verified — CI check, code review, tooling — with real commands]

**Testability**:
- Pass: [What passing looks like]
- Fail: [What failure looks like]

**Rationale**: [Why this constraint exists — what failure mode it prevents, what success it enables]

**Trace**: GI-XXX ([deck-kept: CARD-ID | minted | floor-preset: CARD-ID])

---

<!-- Repeat per principle (II, III, …). Floor principles first, then type/minted principles. -->

## Technology Stack

<!--
INSTRUCTION: Mandated technology choices with rationale, from the synthesis's real-commands
and existing-practices elements (brownfield: from codebase-analysis.md).
This section MUST match exactly in CLAUDE.md.
-->

| Category | Choice | Rationale |
|----------|--------|-----------|
| Language | [LANGUAGE] [VERSION] | [Why this choice] |
| Framework | [FRAMEWORK] | [Why this choice] |
| Testing | [TEST_FRAMEWORK] | [Why this choice] |
| Linting | [LINTER] | [Why this choice] |
| CI/CD | [CI_PLATFORM] | [Why this choice] |

## Quality Gates

<!--
INSTRUCTION: Automated checks that block merge. This section MUST match exactly in CLAUDE.md.
- Replace ALL placeholders with actual commands from the synthesis's real-commands table.
- Coverage thresholds are TIER-PARAMETERIZED — take the pre-seed from the FLOOR-TEST card's
  tier row (catalog/universal-floor.md) unless the session overrode it. Never hardcode one
  tier's numbers into another tier's constitution.
- Gates for waived floor categories are omitted — the waiver record above covers the absence.
-->

| Gate | Requirement | Measurement | Enforcement |
|------|-------------|-------------|-------------|
| Static Analysis | Zero errors | `[actual lint command]` | CI automated |
| Test Suite | All tests pass | `[actual test command]` | CI automated |
| Test Coverage | [tier pre-seed or session override] | `[actual coverage command]` | CI automated |
| Security Scan | [tier-appropriate requirement] | `[actual security tool]` | CI automated |
| Code Review | ≥1 approval | PR status | Branch protection |

<!-- ═══════════════════ SELECTED MODULES ATTACH HERE ═══════════════════
INSTRUCTION: Insert each module section selected in the synthesis (constitution-modules/*.md),
in the order listed there. Every attached module traces to its GI module-selection element.
-->

## Governance

<!--
INSTRUCTION: Who approves constitution changes — CODEOWNERS (recommended), named role, or team
reference. Calibrate ceremony to team reality from the synthesis (a solo poc does not need an
architecture board).
-->

### Approvers

Constitution amendments MUST be approved by: [APPROVER]

### Amendment Process

1. Propose change via PR to constitution file
2. Document rationale for change in PR description
3. Review impact on existing code and templates
4. Obtain approval from designated approvers (see above)
5. Update version per semantic versioning rules below
6. Update CLAUDE.md to reflect changes (mandatory sync)

An amendment that **bumps the tier or un-waives a floor category** is a governance event: re-run
`/mochiko:setup` (amend mode) so the change passes through the interrogation delta and the
synthesis is updated — waivers are revisited against their recorded triggers.

### Version Policy

- **MAJOR**: Principle removal or incompatible redefinition; tier change
- **MINOR**: New principle added or significant expansion of existing principle; waiver added or removed
- **PATCH**: Clarification, wording improvement, or non-semantic refinement

### Exception Registry

When a principle cannot be followed, approved exceptions MUST be recorded in `docs/constitution-exceptions.md` with:

| Field | Description |
|-------|-------------|
| Exception ID | Unique identifier (EX-001, EX-002, etc.) |
| Principle | Which principle is being deviated from |
| Scope | File(s) or component(s) affected |
| Justification | Why the exception is necessary |
| Approved By | Reviewer who approved |
| Approved Date | ISO date of approval (YYYY-MM-DD) |
| Expiry | Date by which must be resolved, or "Permanent" with justification |
| Tracking Issue | Link to issue for resolution (if applicable) |

### Compliance Review

- All PRs MUST include constitution compliance verification
- Periodic audits SHOULD assess codebase alignment with principles (cadence per tier)
- Violations MUST be tracked and addressed

## CLAUDE.md Synchronization

The `CLAUDE.md` file at repository root MUST remain synchronized with this constitution. It serves as the primary instruction file for AI coding assistants and MUST contain accurate governance information.

### Mandatory Sync Artifacts

<!--
INSTRUCTION: This table defines what MUST be replicated in CLAUDE.md. Add a row per attached
module whose content AI agents need (e.g. layer-rules → Architecture section). When the
knowledge-management module is attached, include its rows below (delete them otherwise) — the
operating-manual text must reach every agent session via CLAUDE.md (sync machinery stub-backed
until `syncing-claude-md` ports; the rows still bind at review time).
-->

| Constitution Section | CLAUDE.md Section | Sync Rule |
|---------------------|-------------------|-----------|
| Core Principles | Principles Summary | MUST list all principles with enforcement keywords |
| Governance Tier | Governance Tier | MUST state tier + active waivers |
| Technology Stack | Technical Stack | MUST match exactly |
| Quality Gates | Quality Gates | MUST match exactly |
| [Attached module sections] | [Corresponding section] | MUST match per module |
| Knowledge Management *(module — only when adopted)* | Recording sessions & operating docs | MUST carry the operating-manual text: session-org + index maintenance contract, `BACKLOG.md`/`ROADMAP.md` roles and circulation |
| Knowledge Management *(module — only when adopted)* | Session index pointer | MUST name `.mochiko/brainstorms/index.md` as the session index, read-before-opening |
| Governance | Governance section | MUST include versioning and commit rules |

### Synchronization Process

When amending this constitution:

1. Update constitution version and content
2. Update CLAUDE.md to reflect all changes in the Mandatory Sync Artifacts table
3. Verify CLAUDE.md version matches constitution version
4. Include both files in the same commit
5. PR description MUST note "Constitution sync: CLAUDE.md updated"

### Enforcement

- Code review MUST verify CLAUDE.md is updated when constitution changes
- CLAUDE.md MUST display the same version number as the constitution
- Sync drift between files is a blocking issue for PRs that modify either file

**Rationale**: If CLAUDE.md diverges from the constitution, agents will operate with outdated or incorrect guidance, undermining the governance this constitution establishes.

---

**Version**: [CONSTITUTION_VERSION] | **Ratified**: [RATIFICATION_DATE] | **Last Amended**: [LAST_AMENDED_DATE]
