---
name: assess-primitive
description: This skill MUST be invoked when examining a single human-in-loop primitive (skill, agent, command, or template) to decide how it ports into mochiko form — diagnosing its class, choosing a disposition, and tracing its responsibilities, before any edit is made. SHOULD also invoke at the start of a transform run, once per primitive in the cluster, whenever a disposition or responsibility trace is needed. Emits a disposition (or flag-for-reconcile) plus a responsibility trace.
---

# Assess Primitive

## Overview

Examine one human-in-loop primitive and decide *what it is, what it does, and how it should sit in mochiko* — without changing it yet. The output is a **disposition** (a body-treatment × a structural-move) and a **responsibility trace** (where every responsibility it currently holds will live after the port). Some moves can only be decided with sibling context; those are emitted as `flag-for-reconcile` and resolved later by `reconcile-cluster`.

This skill **absorbs decomposition**: the responsibility trace *is* decomposition, and the trace *is* this skill's done-condition. There is no separate "decompose" step.

**Assess does not transform.** Producing the artifact is `transform-recipes`; grading it is `verify-output`. Assess only diagnoses.

## When to Use

- At the start of a transform run, once per primitive in the cluster
- When you need a disposition + trace before applying any recipe
- When deciding whether a primitive's real problem is its body, its wiring, or its siblings

## When NOT to Use

- To actually edit the primitive (→ `transform-recipes`)
- To resolve cross-primitive relations like pairing/merging (→ `reconcile-cluster`; emit `flag-for-reconcile` instead)
- To grade a finished transformation (→ `verify-output`)

## Step 1: Branch by primitive class

A command/workflow **IS** a loop; an agent/skill **PLAYS a role in** a loop. Running identical checks on both is lossy.

| Class | Branch | What matters most |
|-------|--------|-------------------|
| **command / workflow** | IS-a-loop | who drives the loop, the done-condition, where validation + human gates sit (or are missing) |
| **agent** | PLAYS-a-role | persona vs procedure split, the team-role it confers, its `skills:` independence, **persona decoupling** (no sibling-agent names / "dispatch" / workflow modes-paths-phases / "workflow-agnostic" meta-labels) |
| **skill** | PLAYS-a-role | consumed-procedure vs emits-artifact, trigger reliability, sibling overlap, **decoupling** (no agent names; independence stated by *role*) |
| **template** | artifact | placeholders, what consumes it, path coupling |

Pick the branch first; it changes which checks below carry weight.

## Step 2: Fast-path triage gate

Three yes/no questions. If **all three are "no"**, skip the full lens — apply the convention-wiring pass (see `transform-recipes`) and go straight to `verify-output`.

1. **Orchestration-coupled?** Does it depend on a kernel, a markdown supervisor, a command, or a DAG to function?
2. **Multi-responsibility / fans out?** Does it hold more than one responsibility, or feed more than one consumer?
3. **Emits an artifact whose correctness is NOT machine-checkable?** (If correctness is a version-equality or schema check, the validator degenerates to a deterministic assert — no producer↔validator pairing needed.)

Reserve the full 7-check lens for primitives that answer "yes" to at least one. Record which gate(s) tripped.

## Step 3: Run the lens

Apply the seven checks — weighting per the Step-1 branch. Full detail and per-check prompts in [references/LENS.md](references/LENS.md).

| # | Check | The question |
|---|-------|--------------|
| 1 | **Orchestration test** | What layer orchestrates this — kernel, markdown supervisor, or command — and who inherits each responsibility when it dissolves? Separate content-coupling from orchestration-coupling. |
| 2 | **Role (two altitudes)** | The skill-role (consumed-procedure / emits-artifact) AND the team-role it confers on its caller (producer / validator / referee / lead). |
| 3 | **Independence** | Does one agent both produce and grade? The leak usually hides in an agent's `skills:` list, invisible at the skill itself. |
| 4 | **Verdict-sink / loop-driver** | Who consumes this primitive's output, and what loops on FAIL? (The biggest thing a kernel/supervisor owned.) |
| 5 | **Sibling / overlap** | Shared core, shared validator, or trigger collisions with siblings → emits a merge/split signal. The "look sideways" check; usually becomes a `flag-for-reconcile`. |
| 6 | **Coupling audit** | Hardcoded paths, upstream prerequisites/handoffs, and the determinism boundary (deterministic script vs model judgment). |
| 7 | **Conventions + loop placement** | The five conventions, and whether it supplies a done-condition / independent validation / human gate. **Decoupling scan:** flag any persona or skill carrying sibling-agent names, "dispatch," injected workflow modes/paths/phases, or "workflow-agnostic" meta-labels → a `port-with-edits` decouple action (independence stated by *role*, not agent name). |

## Step 4: Emit the disposition

A disposition is **one body value × one structural value** (full vocabulary and how to choose in `transform-recipes`):

- **Body treatment:** `keep-verbatim` · `port-with-edits` · `redesign` · `drop`
- **Structural move:** `standalone` · `split` · `merge-into-sibling` · `promote` · `absorb-into-lead` · `rewire-cluster`

If the structural move depends on a sibling (`split` into a validator partner, `merge-into-sibling`, `promote` into another agent's tool, `pair`), **do not guess it here** — emit `flag-for-reconcile` with the signal and let `reconcile-cluster` decide with full cluster context. The body treatment can still be proposed.

## Step 5: Emit the responsibility trace

Decomposition done right: list every responsibility the primitive currently holds, and tag where each one lands. No silent loss. Tag vocabulary in [references/TRACE-TAGS.md](references/TRACE-TAGS.md):

`kept` · `kept-but-rebind` · `folded-into-skill` · `moved-to-lead` · `moved-to-validator` · `moved-to-sibling-skill` · `moved-to-other-cluster` · `dedupe` · `dropped + reason`

The trace is this skill's **done-condition**: assessment is not complete until every responsibility carries a tag. The trace later doubles as the `REGISTRY.md` migration record and is audited by `verify-output`.

## Output format

```
ASSESSMENT: <primitive-name>
Class:        <command|agent|skill|template> → branch <IS-a-loop|PLAYS-a-role|artifact>
Triage:       gate1=<y/n> gate2=<y/n> gate3=<y/n>  [fast-path | full-lens]
Disposition:  <body> × <structural | flag-for-reconcile: signal>
Trace:
  - <responsibility> → <tag>[ + reason if dropped]
  - ...
Reconcile flags: <none | the relational signals for reconcile-cluster>
```

## Common Mistakes

| Mistake | Fix |
|---------|-----|
| Guessing a relational move (split/merge/promote) solo | Emit `flag-for-reconcile`; only `reconcile-cluster` has the sibling context. |
| Calling orchestration "kernel" when it's a markdown supervisor | Check 1 separates them. Most HIL setup-cluster coupling is supervisor, not kernel. |
| Skipping the trace because the disposition is `keep-verbatim` | Even kept primitives pay the convention-wiring floor and need a complete trace. |
| Running classification checks against an agent | Classification is a category error for agents; loop-ownership sits on the workflow. Use the branch. |

## Related

- `reconcile-cluster` — resolves every `flag-for-reconcile` with full cluster context
- `transform-recipes` — turns the disposition into the actual edit
- `verify-output` — audits the trace this skill emits
