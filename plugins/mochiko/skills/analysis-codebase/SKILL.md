---
name: analysis-codebase
description: This skill MUST be invoked when analyzing an existing codebase during a brownfield /mochiko:setup run, to produce `.mochiko/memory/codebase-analysis.md`. SHOULD also invoke when a setup/constitution producer needs a deterministic stack baseline (`detect-stack.sh`) or a present/partial/absent read of an existing project before authoring governance.
allowed-tools: Bash(mochiko-cli *)
---

# Analyzing Codebase

## Overview

Systematically analyze an existing codebase to extract the structural information a brownfield
`/mochiko:setup` run needs: the tech stack, architecture, conventions, domain entities, and an
**assessment** of the Essential Floor (Security / Testing / Error Handling / Observability). The
deliverable is `.mochiko/memory/codebase-analysis.md` — the producer's read of "what this codebase
already is," consumed by the analysis checkpoint (the setup lead's human gate), the interrogation
session's existing-practices dimension, and the constitution author.

## Rules — delivered by mochiko-cli

Your rules arrive below, rendered at fire by `mochiko-cli` from the migration log this plugin
carries — one block per section. Every block opens with a version-triple line
(`mochiko-cli rules analysis-codebase · section <id> · binary <v> · grammar <g> · plugin <p>`) and
closes with an end line (`mochiko-cli rules end · analysis-codebase · <id> · <N> rules`). **Proceed
only when every block carries both lines in that exact shape, from whichever channel delivered
it — this slot, or the plugin's dependency hook on a Skill-tool call.** Anything else — an
error, an empty block, the placeholder `[shell command execution disabled by policy]`, a
file-path-plus-preview stub — is a failure to deliver: surface `mochiko-cli rules not
delivered: <what was seen>` and halt. Never Read a schema file instead; there is no fallback.
The `legend` in the preamble block is the reading grammar; a `pointer:` binds you to that
file's or skill's procedure, referenced never restated.

!`mochiko-cli rules analysis-codebase --section preamble --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules analysis-codebase --section analysis-codebase.sec.independence --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules analysis-codebase --section analysis-codebase.sec.scope --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules analysis-codebase --section analysis-codebase.sec.inputs --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules analysis-codebase --section analysis-codebase.sec.verdict --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules analysis-codebase --section analysis-codebase.sec.output --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules analysis-codebase --section analysis-codebase.sec.reserved --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`

Before the first procedural step, state back the floor count the preamble's `class: floor` pin
prints and the ids its `floors:` line lists; a blank or partial read-back is a skipped read —
halt and surface it.

## Common Mistakes

| Mistake | Problem | Fix |
|---------|---------|-----|
| Assuming framework | Guessing without evidence | Verify with code patterns |
| Missing directories | Only checking standard paths | Projects vary, explore |
| Over-extracting | Analyzing every file | Focus on config and patterns |
| Ignoring governance | Missing existing decisions | Check README, CLAUDE.md, CODEOWNERS, ADRs |
| Inventing findings | Documenting assumptions | Only report what is found |
| Redefining the Essential Floor | Restating the four categories here | Read the canonical definition; this skill only *assesses* status |

## Mode: Setup Brownfield (the wired path)

For `/mochiko:setup` on an existing codebase — comprehensive analysis combining the context
sub-procedure with domain-entity extraction and an Essential-Floor status assessment.

**What to Extract:**
- Tech stack, conventions, and architecture (the **Context-gathering sub-procedure** —
  see [references/CONTEXT-GATHERING.md](references/CONTEXT-GATHERING.md))
- Domain entities and relationships (where they live, and what is found)
- Essential-Floor **status** assessment (present / partial / absent, file-cited — the
  boundary and posture rules live in the schema's scope and verdict sections)
- Inconsistencies and strengths to preserve

**Output**: `.mochiko/memory/codebase-analysis.md` — the production binding, envelope, and
capability-signal seeding are the schema's `analysis-codebase.sec.output` rules.

## Detection Script

Run the automated detection script for fast, deterministic stack identification:

```bash
bash scripts/detect-stack.sh /path/to/project
```

## Related Skills

- **For brownfield constitutions**: **REQUIRED:** Use `mochiko:authoring-constitution` (brownfield branch) after analysis
- **For greenfield projects**: **OPTIONAL:** Use `mochiko:authoring-constitution` directly
- **For validation**: **OPTIONAL:** `mochiko:validation-constitution` grades the constitution (run by an independent validator — a different agent than the author)
