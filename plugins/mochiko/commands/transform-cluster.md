---
description: Transform a human-in-loop primitive cluster into mochiko form via an independent producer/validator loop with a human gate.
---

# Transform Cluster

You are the **lead / referee** for transforming one human-in-loop cluster into mochiko form. There is no kernel and no separate lead agent — **you, executing this command, are the supervisor**. You own the loop, the verdict, reconciliation arbitration, and the human gate. The work is done by two dispatched agents: `mochiko:transform-producer` and `mochiko:transform-validator`.

This workflow is itself a mochiko sound loop. Before you start, you MUST instantiate its `workflow-contract` (Phase 0). Honor `mochiko:loop-discipline` throughout: pre-declared done-condition (default FAIL), independent validation (producer never grades its own output), bounded iteration, and a named human gate.

**Argument:** `$ARGUMENTS` = the cluster to transform (a path under `human-in-loop/plugins/humaninloop/`, or a named cluster from `REGISTRY.md`, e.g. `setup`).

---

## Done-condition (pre-declared, DEFAULT FAIL)

The run is **DONE** only when, for every primitive in the cluster:
- `verify-output` returns **PASS** (conformance + trace audit), AND
- every original responsibility carries a trace tag with no silent drops, AND
- the human gate has accepted any `redesign` / `absorb` / `promote` dispositions and any `dropped` reasons.

Until then the run is **FAIL**. Running out of rounds is FAIL-and-escalate, never "done."

---

## Phase 0 — Intake & contract

1. Resolve `$ARGUMENTS` to a concrete set of primitives (list the files in the cluster). Confirm the list with the user if ambiguous.
2. Create the run workspace: `.mochiko/transform/<cluster>/` with:
   - `contract.md` — fill in `templates/workflow-contract.md` for this run (done-condition above; producer = `transform-producer`; validator = `transform-validator`; round cap = **3**; no-progress exit; **budget / kill-switch** — a per-run token/cost ceiling plus an out-of-band halt: stop if `.mochiko/transform/<cluster>/STOP` exists; human-gate placement — default: **on redesign/absorb/promote dispositions and on any cap-exhaustion escalation**). Fill **every** field — `loop-discipline` requires all four iteration guards; an open bracket means the loop is not ready.
   - `context.md` — the user request, the resolved primitive list, and a running log.
3. Invoke `mochiko:loop-discipline` to confirm the contract satisfies all four requirements before proceeding. A contract with an open bracket is not ready.

> Human gate (placement decision): if the user wants a different gate placement (every cycle / low-confidence only), set it here.

## Phase 1 — Triage & assess (per primitive)

For each primitive, dispatch the producer:

```
Task(
  subagent_type: "mochiko:transform-producer",
  description: "Assess <primitive>",
  prompt: "Run mochiko:assess-primitive on <path>. Branch by class, run the fast-path triage,
           then the lens if warranted. Emit: class/branch, triage result, a disposition
           (body × structural) OR flag-for-reconcile, and a COMPLETE responsibility trace.
           Do not transform. Write your assessment to .mochiko/transform/<cluster>/assess-<primitive>.md
           and return the summary."
)
```

Collect all assessments, dispositions, traces, and `flag-for-reconcile` signals into `context.md`. Do **not** advance until every primitive is assessed (reconcile needs the whole picture).

## Phase 2 — Reconcile (cluster scope)

Dispatch the producer once for the whole cluster:

```
Task(
  subagent_type: "mochiko:transform-producer",
  description: "Reconcile <cluster>",
  prompt: "Run mochiko:reconcile-cluster over all assessments in .mochiko/transform/<cluster>/.
           Resolve every flag-for-reconcile into a concrete move (pair/split/merge/promote/dedupe),
           build the rehome map for orchestration the old supervisor owned (including the missing
           validation + human gates), name any new partner primitives required, and re-emit traces
           with relational tags assigned. Output MUST have zero open flags."
)
```

**Human gate (per placement):** present the reconciled plan — especially any `redesign`, `absorb-into-lead`, `promote`, or `dropped` decisions — to the user with `AskUserQuestion`. As referee, you own arbitration: accept, override, or send back. Record the decision in `context.md`.

## Phase 3 — Transform (apply dispositions)

For each primitive with a finalized disposition, dispatch the producer:

```
Task(
  subagent_type: "mochiko:transform-producer",
  description: "Transform <primitive>",
  prompt: "Run mochiko:transform-recipes to apply <body × structural> to <primitive>, including any
           new partner primitives from reconcile, and run the convention-wiring pass (classification,
           router registration, triggers, path rebinding). Produce the mochiko-form artifact and the
           realized responsibility trace. Do NOT grade your own output."
)
```

Write artifacts into the mochiko plugin tree; collect realized traces.

## Phase 4 — Verify (independent gate, bounded loop)

For each transformed primitive, dispatch the **validator** (never the producer):

```
Task(
  subagent_type: "mochiko:transform-validator",
  description: "Verify <primitive>",
  prompt: "Run mochiko:verify-output on the transformed <primitive> and its trace. Read the artifact
           itself. Return a binary PASS/FAIL with conformance items, trace audit, and a fix list.
           Default FAIL; no PASS without evidence Read from the artifact."
)
```

**Loop control (bounded iteration):**
- On any **FAIL**, send the fix list back to the producer (Phase 3) for that primitive only. Increment the round counter.
- **Round cap = 3.** On the 3rd failed round, stop looping and escalate.
- **No-progress exit:** if a round's failing items are unchanged from the previous round, stop and escalate.
- **Budget / kill-switch:** stop if the run's token/cost ceiling is hit, or if an out-of-band halt is signaled (`.mochiko/transform/<cluster>/STOP`). Treat as an escalation, never a done.
- **Escalate, don't die:** on cap / no-progress / budget, present the failing items to the user via `AskUserQuestion` (human gate) with the failure context. Never mark the run done on exhaustion.

The cluster is DONE only when **every** primitive returns PASS and the human gate has cleared the gated dispositions.

## Phase 5 — Finalize

1. Update `REGISTRY.md` from the realized traces (mark ported primitives; record where responsibilities landed; flag follow-ups).
2. Write a run report to `.mochiko/transform/<cluster>/report.md`: per-primitive disposition, verdict, trace summary, any accepted drops, any escalations.
3. Surface to the user: what landed, what was dropped (with accepted reasons), and any open follow-ups for `ROADMAP.md` / `BACKLOG.md`.

---

## State recovery

If interrupted, resume from `.mochiko/transform/<cluster>/`:

| Evidence in the workspace | Resume at |
|---------------------------|-----------|
| `contract.md` missing | Phase 0 |
| `assess-*.md` incomplete | Phase 1 (remaining primitives) |
| assessments complete, no reconcile decision logged | Phase 2 |
| reconcile done, artifacts missing | Phase 3 |
| artifacts present, no verdicts | Phase 4 |
| all PASS + gates cleared | Phase 5 |

## Supervisor behaviors (what you own, not the agents)

- Own the **loop**: the round counter, the no-progress check, the cap, the escalation.
- Own the **verdict**: the producer proposes, the validator grades, **you** declare done — and only against the pre-declared done-condition.
- Own **reconciliation arbitration**: accept/override the producer's relational verdicts at the human gate.
- Own the **human gate**: present gated dispositions and escalations; never auto-accept a drop or a redesign.
- Never let producer and validator collapse into one agent. Independence is the point.
