<!--
WORKFLOW CONTRACT TEMPLATE
==========================
Fill one of these in for every mochiko workflow. The completed contract is the
inspectable proof that the loop satisfies the four requirements of the
`loop-discipline` skill. A reviewer reads it to confirm: (a) the done-condition
defaults to FAIL, (b) the validator is a DIFFERENT agent + DIFFERENT skill than
the producer, (c) iteration is deterministically bounded, (d) a human gate is named.

INSTRUCTIONS:
- Replace every [PLACEHOLDER]. Do not leave brackets in a finalized contract.
- Delete the HTML comments before committing the filled copy alongside the workflow.
- If you cannot fill a field truthfully, the loop is not ready — fix the loop, not the contract.
- Store the filled copy with the workflow run (e.g. .mochiko/<workflow>/contract.md).
-->

# Workflow Contract — [WORKFLOW_NAME]

**Workflow:** [WORKFLOW_NAME] · **Carrier:** [commands/<name>.md supervisor | agent team] · **Filled:** [YYYY-MM-DD]

## 1. Done-condition (DEFAULTS TO FAIL)

<!-- The artifact starts FAILing and only flips on real evidence. State all three parts. -->

- **Measurable end state:** [the observable fact that means done — e.g. "verify-output returns PASS for every primitive in the cluster"]
- **Stated check (how it is proven):** [named in advance — e.g. "the validator agent Reads each transformed artifact + its trace and confirms conformance"]
- **Constraints (must not be violated):** [e.g. "no original responsibility silently dropped; kernel-free maintained"]
- **Initial state:** `FAIL` <!-- always FAIL until proven otherwise -->

## 2. Producer ↔ Validator (independence on two axes)

| Role | Agent | Skill(s) | Notes |
|------|-------|----------|-------|
| **Producer** | [mochiko:<agent>] | [skill, skill] | emits the artifact under review |
| **Validator** | [mochiko:<DIFFERENT agent>] | [DIFFERENT skill] | grades from the artifact itself, never the producer's say-so |

- **Independence check:** producer agent ≠ validator agent **AND** producer skills ∩ validator skill = ∅. [confirm both]
- **Validator trustworthiness tier:** [1 deterministic ground truth | 2 separate-context grounded LLM | 3 LLM-judge] — [why this is the highest the artifact allows]
- **Tamper-proofing:** [how a PASS is gated on evidence actually Read — e.g. "no PASS unless the artifact file was Read this run"]

## 3. Bounded iteration

- **Hard round cap:** [N] produce↔validate rounds, counted by the supervisor.
- **No-progress exit:** [what counts as no progress — e.g. "a round where the validator's failing items are unchanged"]
- **Budget / kill-switch:** [token/cost ceiling or out-of-band halt]
- **On hitting a guard:** escalate to the human gate with failure context. [never report done on cap exhaustion]

## 4. Human gate

- **Placement:** [every cycle | low validator-confidence only | preference-gap only]
- **Where it fires:** [the phase/condition — e.g. "before applying redesign/absorb/promote dispositions, and on any cap-exhaustion escalation"]
- **What the human decides:** [e.g. "accept/override the disposition; accept a dropped-responsibility reason; final verdict"]

---

<!-- All four sections filled with no remaining brackets = the loop is sound per loop-discipline. -->

**Contract version:** [v1] · **Governed by:** `loop-discipline`
