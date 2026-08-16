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

## When NOT to Use

- **Greenfield projects**: no existing code to analyze; go straight to `mochiko:authoring-constitution`
- **Single-file scripts**: no architectural patterns to extract
- **Documentation-only review**: use standard file reading instead
- **Before the project directory exists**: nothing to analyze yet
- **When the user already provided complete context**: skip analysis if the tech stack and patterns are already documented
- **Collision detection / JSON inventory**: that is the spec/plan-cluster Brownfield mode (not wired this run)

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
- Domain entities and relationships (locate where entities live — models/, schema files, ORM
  annotations; document what is found — the deeper collision-risk inventory is the
  spec/plan-cluster Brownfield mode, not produced here)
- Essential-Floor **status** assessment (present / partial / absent — see below)
- Inconsistencies and strengths to preserve

**Output**: `.mochiko/memory/codebase-analysis.md`, following the `codebase-analysis` schema
(invoke `mochiko-cli template codebase-analysis` when the binary is available; otherwise Read
`plugins/mochiko/schemas/codebase-analysis.yaml` raw). The
artifact follows the deliverable envelope ([`artifact-format.md`](../../templates/artifact-format.md))
**slimmed but legible**: findings in tables with file-cited evidence, one line per check;
the judgment prose (strengths, recommendations) stays prose — the G2 gate reader needs it.
The extracted capability signals (routes, UI surfaces, services) also seed setup's
feature-map reconstruction (map machinery: `mochiko:authoring-feature-map`).

### Essential-Floor Status Assessment

The four Essential-Floor categories — **Security, Testing, Error Handling, Observability** — are
**defined canonically** in
[`authoring-constitution/references/ESSENTIAL-FLOOR.md`](../authoring-constitution/references/ESSENTIAL-FLOOR.md).
Governance owns *require-floor* (what the categories are, the asserted level, and the waiver
posture). This skill owns the other half — **assess-status**: detect each category's
present / partial / absent state in the existing codebase, with file-cited evidence. **Do not
redefine the categories here.** Read the canonical definition, then assess against it using the
indicators below.

**The assessment is intent-blind and waiver-blind by design.** Report what IS — the same codebase
gets the same status regardless of anything the session later rules. Downstream, the setup
session interprets the statuses against the asserted floor (an `absent` category becomes an open
confrontation, resolved as a MUST-implement principle or a recorded waiver) — that interpretation
belongs to the session and the constitution, never to this analysis. Do not soften an `absent` to
`partial` because the project "is young", and do not mark a category "waived" — waivers are
governance rulings, not codebase facts.

### Setup-Brownfield Quality Checklist

Before finalizing the analysis:

- [ ] Project identity complete (name, language, framework, entry points)
- [ ] Directory structure documented with purposes
- [ ] Architecture pattern identified with evidence
- [ ] Naming conventions documented (files, variables, functions, classes)
- [ ] All four Essential-Floor categories assessed (present/partial/absent, file-cited)
- [ ] Domain entities extracted with relationships
- [ ] External dependencies documented
- [ ] Strengths to preserve identified (minimum 2-3)
- [ ] Inconsistencies documented with severity
- [ ] Recommendations provided for constitution focus
- [ ] File paths cited for all findings
- [ ] Output written to `.mochiko/memory/codebase-analysis.md`

## Other modes (moved to other clusters — not wired this run)

Only **Setup-Brownfield** is wired here. **Brownfield / collision mode** (entity + endpoint
extraction → JSON collision inventory against a proposed spec) lives in the **spec/plan
cluster**; **standalone Context-report mode** (a human-facing Project Context Report) lives in
the **constitution-context cluster** — its extraction *sub-procedure* is kept here
(`references/CONTEXT-GATHERING.md`), folded into Setup-Brownfield.

## Detection Script

Run the automated detection script for fast, deterministic stack identification:

```bash
bash scripts/detect-stack.sh /path/to/project
```

> **Determinism boundary.** `detect-stack.sh` is the deterministic layer (pure `bash` + `jq`,
> reads project files, JSON to stdout — no kernel, no network). Framework/architecture inference
> and the Essential-Floor assessment are the model-judgment layer on top. Keep the boundary
> explicit.

## Related Skills

- **For brownfield constitutions**: **REQUIRED:** Use `mochiko:authoring-constitution` (brownfield branch) after analysis
- **For greenfield projects**: **OPTIONAL:** Use `mochiko:authoring-constitution` directly
- **For validation**: **OPTIONAL:** `mochiko:validation-constitution` grades the constitution (run by an independent validator — a different agent than the author)
