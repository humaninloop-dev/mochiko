---
name: authoring-constitution
description: This skill MUST be invoked when authoring or amending a project's governance surface set from a ratified session synthesis (`.mochiko/memory/governance-intent.md`), landing principles on native Claude Code surfaces; there is NO constitution.md. SHOULD also invoke when the work concerns principle enforcement, compliance modules, floor waivers, or an Essential Floor. The single governance-authoring skill for BOTH greenfield and brownfield projects — no separate brownfield skill.
allowed-tools: Bash(mochiko-cli *)
---

# Authoring Constitution — Governance on Native Surfaces

## Overview

Write project governance that teams — and Claude Code sessions — actually follow: actionable
constraints with measurable criteria, not vague aspirations. Governance lands on the
surfaces Claude Code natively loads, each at its disclosure level:

| Surface | Carries | Disclosure |
|---------|---------|------------|
| `CLAUDE.md` **governance region** | Ratified stamp · principle index · universal principles as short imperative lines · tech stack · quality-gates summary · module pointers | Always-on, every session and every spawned agent |
| `.claude/rules/mochiko/*.md` | Scope-bound principles, one file per concern, `paths` frontmatter | On matching-file reads (plus the dispatch-brief obligated read for authoring producers) |
| Skill pointers | Procedure-shaped standards — the index/rule points at the skill | On trigger / when a brief names it |
| `.mochiko/memory/governance-ledger.md` | Per-principle **Three-Part records** keyed by GI-ID · floor + attached compliance modules · waivers · amendment policy · exceptions · amendment log | Read only by setup/amend runs and the validator |

The synthesis is a **traceable contract, not a brief** — the floor is asserted, its
expression session-shaped; formulation is where this skill's judgment lives, and the
routing IS part of formulation quality.

## Rules — delivered by mochiko-cli

Your rules arrive below, rendered at fire by `mochiko-cli` from the migration log this plugin
carries — one block per section. Every block opens with a version-triple line
(`mochiko-cli rules authoring-constitution · section <id> · binary <v> · grammar <g> · plugin <p>`) and
closes with an end line (`mochiko-cli rules end · authoring-constitution · <id> · <N> rules`). **Proceed
only when every block carries both lines in that exact shape, from whichever channel delivered
it — this slot, or the plugin's dependency hook on a Skill-tool call.** Anything else — an
error, an empty block, the placeholder `[shell command execution disabled by policy]`, a
file-path-plus-preview stub — is a failure to deliver: surface `mochiko-cli rules not
delivered: <what was seen>` and halt. Never Read a schema file instead; there is no fallback.
The `legend` in the preamble block is the reading grammar; a `pointer:` binds you to that
file's or skill's procedure, referenced never restated.

!`mochiko-cli rules authoring-constitution --section preamble --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules authoring-constitution --section authoring-constitution.sec.independence --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules authoring-constitution --section authoring-constitution.sec.scope --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules authoring-constitution --section authoring-constitution.sec.inputs --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules authoring-constitution --section authoring-constitution.sec.artifact --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules authoring-constitution --section authoring-constitution.sec.output --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules authoring-constitution --section authoring-constitution.sec.reserved --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`

Before the first procedural step, state back the floor count the preamble's `class: floor` pin
prints and the ids its `floors:` line lists; a blank or partial read-back is a skipped read —
halt and surface it.

## Two modes, one shared core

| Mode | Use when |
|------|----------|
| **greenfield** | A new project with no existing code to honor — formulate the synthesis's deck rulings and minted intents. |
| **brownfield** | An existing codebase — *codify what is already there*: the Essential Floor assessed against the code (present/partial/absent), an Emergent Ceiling codifying good existing patterns, and the `evolution-notes` module. |

The shared core — the Three-Part Principle Rule, RFC 2119 keywords, surface routing, the
mandatory content inventory, and module assembly — is the same in both modes. The content
sources are the type-shelved principle deck
([references/catalog/](references/catalog/README.md)), the canonical floor definition
([references/ESSENTIAL-FLOOR.md](references/ESSENTIAL-FLOOR.md)), the compliance-module
library ([references/COMPLIANCE-MODULES.md](references/COMPLIANCE-MODULES.md)), and — for
brownfield —
[references/EMERGENT-CEILING-PATTERNS.md](references/EMERGENT-CEILING-PATTERNS.md).

## The Three-Part Principle Rule

**Enforcement** — how compliance is verified; without it, a principle is a suggestion.
**Testability** — what pass/fail looks like: binary outcome, measurable threshold where
applicable, observable without subjective judgment, reproducible by any team member.
**Rationale** — why the constraint exists: the failure mode prevented, the success enabled,
the justification for the enforcement overhead. Worked Three-Part examples: the four floor
principles in [references/ESSENTIAL-FLOOR.md](references/ESSENTIAL-FLOOR.md).

RFC 2119 keywords — MUST / MUST NOT (absolute) · SHOULD / SHOULD NOT (recommended /
discouraged; valid exceptions exist) · MAY (optional); detailed usage:
[references/RFC-2119-KEYWORDS.md](references/RFC-2119-KEYWORDS.md).

## Module assembly

Modules from [`templates/constitution-modules/`](../../templates/constitution-modules/)
attach per the synthesis's module selections — module content routes by surface, like
everything else (any in-file attach instructions that predate the dissolution are
superseded by this table):

| Module | Attach when | Routes to |
|--------|-------------|-----------|
| `layer-rules` | A layered-architecture principle was kept **or minted** (the module ruling lands in the synthesis either way — the interrogation's layered-architecture beat) | `paths`-scoped rules files (one per layer concern; the domain file carries the preserved registry block + policy preamble — `references/DOMAIN-DEPENDENCIES.md`) + index lines + ledger entries (incl. the Domain-dependency policy section) |
| `release-gates` | Always offered (a deployed/operated target class — PO-D1); content from the always-interrogated deployment dimension | Region: one summary line + pointer; detail in the ledger |
| `evolution-notes` | Mode is brownfield (always) | Ledger section (floor status, gap references, confrontation rulings) + region pointer |
| `knowledge-management` | The KM dimension elicited adoption (default-on, whole; a recorded decline is durable) | Region: the operating-manual **pointer** + index line; the bundle scaffolding and command carriers are unchanged (lead-executed at finalize) |
| **compliance modules** (`hipaa`, `pci-dss`, … — [references/COMPLIANCE-MODULES.md](references/COMPLIANCE-MODULES.md)) | The fact profile triggered them (recorded in the synthesis's Fact profile — never a session choice) | Obligations formulated as principles at their stratum, routed by scope like any principle; the ledger records module + stratum per obligation |

## Brownfield — floor and ceiling

Existing codebases have implicit conventions worth preserving (the Emergent Ceiling) but may
lack foundational governance (the Essential Floor). Read the codebase analysis for
**"Strengths to Preserve"** (ceiling candidates) and the **Essential-Floor status**
(present / partial / absent per category); the ceiling-vs-debt call rides the delivered
ceiling test.

## Related (cross-cluster; referenced, not mounted)

- **`analysis-codebase`** (in-cluster) — produces `.mochiko/memory/codebase-analysis.md`,
  the brownfield-mode input. Run before brownfield authoring (lead-sequenced).
