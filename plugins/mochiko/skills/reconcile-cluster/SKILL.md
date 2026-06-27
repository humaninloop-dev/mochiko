---
name: reconcile-cluster
description: This skill MUST be invoked after every primitive in a cluster has been assessed, to resolve cross-primitive decisions — turning flag-for-reconcile signals into concrete moves (pair / split / merge-into-sibling / promote / dedupe) and re-homing the orchestration a dissolving supervisor owned. SHOULD also invoke whenever assess-primitive emitted relational trace tags or unresolved flags remain. Owns the relational verdicts and rehome-orchestration that no single-primitive assessment can decide.
---

# Reconcile Cluster

## Overview

Per-primitive assessment can only see one primitive at a time. The moves that define mochiko form — pairing a producer with an independent validator, merging a thin variant into its sibling, promoting a check into a validator's tool, and re-landing the responsibilities of a dissolving supervisor — are **relations between primitives**. This skill resolves them with full cluster context.

Reconcile runs **once per cluster**, after all primitives are assessed, and consumes their dispositions, traces, and `flag-for-reconcile` signals. Its output is the finalized disposition set plus a **rehome map** — and it must leave no flag and no orphaned responsibility unresolved.

**Reconcile decides; it does not edit.** Applying the resolved dispositions is `transform-recipes`.

## When to Use

- After `assess-primitive` has run on every primitive in the cluster
- To turn `flag-for-reconcile` signals into concrete structural moves
- To decide producer↔validator pairings and which responsibilities re-home where
- To re-land orchestration the cluster's old supervisor owned

## When NOT to Use

- Before all primitives are assessed (you'd reconcile against an incomplete picture)
- For a single primitive with no sibling relations and no orphaned orchestration (its assess disposition stands as-is)
- To apply the changes (→ `transform-recipes`)

## Job 1: Resolve relational verdicts

Take every `flag-for-reconcile` and every relational signal from the traces, and assign a concrete structural move:

| Signal from assess | Resolved move | What it means |
|--------------------|---------------|---------------|
| emits a reviewable artifact, no independent validator | **pair** → `split` | spin out a validator partner (new agent + `verify-*` skill); the producer keeps authoring only |
| self-grade leak (one agent produces + grades) | **split** the agent | move the grading skill to a peer validator agent (`moved-to-validator`) |
| thin variant over a shared core | **merge-into-sibling** | consolidate into one skill with branches; keep only the variant-unique slice (`moved-to-sibling-skill`) |
| a check that confers a validator team-role | **promote** | elevate it to the load-bearing tool of an independent validator |
| shared substrate mounted twice | **dedupe** | collapse to one source, both consumers point at it |
| trigger collision between siblings | de-collide (wiring) | not structural — hand to the convention-wiring pass in `transform-recipes` |

**Independence is the test that overrides convenience.** Never resolve a pairing by mounting producer and validator skills on the same agent. If a resolution would put produce + grade on one agent, it is wrong — re-resolve as a `split`.

## Job 2: Rehome-orchestration

When a cluster's old orchestrator dissolves (a markdown supervisor is re-homed onto a mochiko Workflow/lead, or a kernel/DAG is shed), its responsibilities must land somewhere explicit. Walk every `moved-to-lead` trace tag and every loop gap from check 7, and build the **rehome map**:

| Orchestration responsibility | Re-homes to |
|------------------------------|-------------|
| sequencing / dispatch order | the command supervisor (lead) |
| loop-driving on FAIL | the supervisor's bounded loop (round cap + no-progress) |
| the **missing** validation gate | a new independent validator (pair/split from Job 1) |
| the **missing** human gate | the supervisor's named human gate |
| input/prerequisite wiring | explicit handoff edges in the contract |

This is where the dry-run's strategic finding lives: a cluster's real transformation is often **not** "shed a kernel" but "add the independent-validation and human gates the original never had." Reconcile is where those gates get a home.

## Job 3: Close the trace

After Jobs 1–2, **every** `flag-for-reconcile` is resolved and **every** orphaned/dropped responsibility has a landing place. Re-emit the per-primitive traces with relational tags now assigned (`moved-to-validator`, `moved-to-sibling-skill`, `dedupe`). Any unresolved flag or homeless responsibility means reconcile is not done.

## Output format

```
RECONCILE: <cluster-name>
Relational verdicts:
  - <primitive> : <flag> → <resolved move> (partner: <primitive>)
  - ...
Rehome map (orchestration):
  - <responsibility> → <lead | new-validator | human-gate | handoff-edge>
  - ...
New primitives required:
  - <name> (<agent|skill>) — <why> (e.g. independent validator partner)
Finalized dispositions: <per-primitive body × structural>
Open flags: <MUST be none>
```

## Common Mistakes

| Mistake | Fix |
|---------|-----|
| Resolving a pairing onto one agent | That recreates the self-grade sin. A pairing is always a `split` across two agents. |
| Forgetting the missing gates | Rehome-orchestration must add validation + human gates the original lacked, not just relocate what existed. |
| Leaving a `flag-for-reconcile` open | Reconcile's done-condition is zero open flags and zero homeless responsibilities. |
| Merging primitives that only look similar | Merge only on a genuinely shared core; keep the variant-unique slice via `moved-to-sibling-skill`. |

## Related

- `assess-primitive` — produces the flags and traces this skill consumes
- `transform-recipes` — applies the finalized dispositions and the rehome map
- `loop-discipline` — the source of the validation + human gate requirements rehome-orchestration must satisfy
