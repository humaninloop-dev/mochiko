# Mochiko Transformation Playbook

How to turn a human-in-loop primitive (skill, agent, command, template) into mochiko form —
**repeatably, inspectably, and without silent capability loss.**

This file is **doctrine + pointers only**. The rules, tests, and recipes live single-sourced in the
skills it points to (`single-source-rule-fanout`); restating them here would guarantee drift. Design
rationale is in [`.mochiko/brainstorms/playbook-design/synthesis.md`](.mochiko/brainstorms/playbook-design/synthesis.md); the framework thesis is in
[`ROADMAP.md`](ROADMAP.md).

The playbook is **dogfooded**: it *is* a mochiko cluster (a command supervisor + a producer/validator
agent team + four skills) whose subject is "transform a primitive." It therefore obeys the same
sound-loop doctrine it enforces.

---

## What "mochiko form" means

A primitive is in mochiko form when it is **kernel-free**, satisfies the **five conventions**, is
**correctly placed inside a sound loop**, and sits at the **right altitude**:

1. **Classification** — declares user-invoked or model-invoked; user-invoked may call model-invoked, never each other.
2. **Discoverability** — registered in the `mochiko` router with when-to-reach-it guidance.
3. **Reliable model-invocation** — model-invoked skills carry graded, exact-phrase, work-context triggers.
4. **Agent↔skill composition** — agents declare a `skills:` list and lean on those skills when the work is in their domain. The persona carries *self-sufficient method + judgment + standards* (competent even when no skill fits); the skill carries the *exact procedure*. A persona names no workflow, no sibling agent, no "dispatch" — decoupling is proven by that **absence**, enforced by the keystone test (*true of this professional on any job → keep; only-in-this-workflow → cut*). Caller-side context lives in `agent-dispatch.md`, never the persona.
5. **Producer↔validator pairing** — reviewable artifacts get an independent validator, and independence is **structural**: a different agent running a different skill grades than produced it, guaranteed by the loop — never asserted in a persona. (Or correctness is machine-decidable and the check degenerates to a deterministic assert.)

→ Conventions detail: `ROADMAP.md`. Sound-loop rules: **`loop-discipline`** + **`workflow-contract`**.

**Altitude (single-sourcing).** A command's job is to stitch a team to a goal under a contract — it *references* shared doctrine, it does not restate it. A primitive that re-inlines what `loop-discipline`, `workflow-contract`, or `agent-dispatch` already single-source is at the wrong altitude (the verbosity defect), even when it is kernel-free and convention-clean. Enforced by `verify-output`'s altitude scan; full rationale in [`.mochiko/brainstorms/command-altitude/synthesis.md`](.mochiko/brainstorms/command-altitude/synthesis.md).

## The done-condition (default FAIL)

A transformation is **done** only when the output (a) conforms to the five conventions + sound-loop
placement + kernel-free, AND (b) carries a **responsibility trace** mapping every original
responsibility to where it lives now — with no silent drops. The verdict defaults to **FAIL**.

→ Operationalized by **`verify-output`**, run by an independent validator, never the producer.

## The disposition vocabulary

Every primitive gets **one body treatment × one structural move**, plus a wiring pass that always runs:

- **Body:** `keep-verbatim` · `port-with-edits` · `redesign` · `drop`
- **Structural:** `standalone` · `split` · `merge-into-sibling` · `promote` · `absorb-into-lead` · `rewire-cluster`
- **Convention-wiring pass:** always runs (classification, router registration, triggers, path rebinding) — there is no `keep-as-is`; the floor is never zero-work.

→ How to choose and how to apply: **`assess-primitive`** (choose body + flag relational moves),
**`reconcile-cluster`** (decide relational moves + rehome orchestration), **`transform-recipes`** (apply).

## The loop shape

```
/mochiko:transform-cluster <cluster>
   triage → assess-primitive → reconcile-cluster → transform-recipes → verify-output
            (per primitive)     (cluster scope)     (apply)             (independent gate)
   producer ────────────────────────────────────────┘   validator
   lead/referee = the transform-cluster command supervisor (owns verdict, arbitration, human gate)
```

→ Driven by **`commands/transform-cluster.md`**; transform a whole cluster per run, not stray primitives.

---

## Pointers (the single source for each rule)

| Concern | Lives in |
|---------|----------|
| The four sound-loop rules | `plugins/mochiko/skills/loop-discipline/SKILL.md` |
| The fill-in loop contract | `plugins/mochiko/templates/workflow-contract.md` |
| Class branching, 7-check lens, triage, responsibility trace | `plugins/mochiko/skills/assess-primitive/` (+ `references/`) |
| Relational verdicts + rehome-orchestration | `plugins/mochiko/skills/reconcile-cluster/SKILL.md` |
| Per-disposition recipes + wiring pass | `plugins/mochiko/skills/transform-recipes/` (+ `references/`) |
| Done-condition checker (conformance + trace audit) | `plugins/mochiko/skills/verify-output/SKILL.md` |
| Library index / discoverability | `plugins/mochiko/skills/mochiko/SKILL.md` (router) |
| The producer / validator personas | `plugins/mochiko/agents/transform-producer.md`, `validator.md` (one generic independent grader) |
| The caller-side dispatch briefing | `plugins/mochiko/templates/agent-dispatch.md` |
| The orchestration (lead/referee) | `plugins/mochiko/commands/transform-cluster.md` |

## How to run it

```
/mochiko:transform-cluster setup
```

The supervisor instantiates a `workflow-contract`, assesses each primitive, reconciles the cluster,
applies the dispositions, and gates each result through the independent validator and the human —
defaulting to FAIL until the done-condition is met. The first real run (the HIL `setup` cluster) is
the proving ground that will teach the empirical calls still open in `.mochiko/brainstorms/playbook-design/synthesis.md`
(human-gate placement, disposition short-circuiting).
