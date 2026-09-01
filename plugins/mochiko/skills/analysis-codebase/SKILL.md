---
name: analysis-codebase
description: This skill MUST be invoked when analyzing an existing codebase during a brownfield /mochiko:setup run, to produce `.mochiko/memory/codebase-analysis.md`. SHOULD also invoke when a setup/constitution producer needs a deterministic stack baseline (`detect-stack.sh`) or a present/partial/absent read of an existing project before authoring governance.
---

# Analyzing Codebase

## Overview

Systematically analyze an existing codebase to extract the structural information a brownfield
`/mochiko:setup` run needs: the tech stack, architecture, conventions, domain entities, and an
**assessment** of the Essential Floor (Security / Testing / Error Handling / Observability). The
deliverable is `.mochiko/memory/codebase-analysis.md` — the producer's read of "what this codebase
already is," consumed by the analysis checkpoint (the setup lead's human gate), the interrogation
session's existing-practices dimension, and the constitution author.

## Rules — load the schema first

Your first action, before any detection or analysis step: **Read `schema.yaml` (this
skill's own directory) raw, in full** — the small families ship no common file, so the
pair's own schema is the whole first action. The schema is the source of truth for this
skill's binding rules, nested in six sections, each addressable by its section ID:
`analysis-codebase.sec.independence` · `analysis-codebase.sec.scope` ·
`analysis-codebase.sec.inputs` · `analysis-codebase.sec.verdict` ·
`analysis-codebase.sec.output` · `analysis-codebase.sec.reserved`. Interpret it live: a
rule's `kind:` names what it is, and an absent `kind:` reads `constraint`; a rule of
`class: floor` is always read and always delivered; a `pointer:` rule binds you to that
file's or skill's procedure, referenced never restated; labels come from
`plugins/mochiko/schemas/skill-labels.yaml`. The floor pin: the 3 rules of `class: floor`
are non-waivable. Before the first detection or analysis step, state the floor count
back — a skipped or partial read leaves that count blank: halt and surface it, and halt
likewise if the schema's `class: floor` count disagrees with the pin.

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
