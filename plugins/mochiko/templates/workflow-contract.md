<!--
WORKFLOW CONTRACT TEMPLATE
==========================
WHEN TO FILL THIS IN — exactly two cases:
  1. A COMMAND RUN THAT DEPARTS from its command's stated default pipeline, or that
     declares non-default bounds. A default run needs no form: the command IS its
     contract (templates/command-shape.md), and its values were constant at authoring
     time. What a departing lead composes instead is what genuinely varies per run, so
     that is what this form carries.
  2. ANY NON-COMMAND LOOP — an agent loop, a skill's own produce → check.
A default-running command declares in ONE LINE on its deliverable and fills nothing here.

The completed contract is the inspectable proof that the loop satisfies the four
requirements of the `loop-discipline` skill. A reviewer reads it to confirm: (a) the
done-condition defaults to FAIL, (b) the validator is a DIFFERENT agent + DIFFERENT skill
than the producer, (c) iteration is deterministically bounded, (d) a human gate is named.

INSTRUCTIONS:
- Replace every [PLACEHOLDER]. Do not leave brackets in a finalized contract.
- Delete the HTML comments before committing the filled copy alongside the workflow.
- If you cannot fill a field truthfully, the loop is not ready — fix the loop, not the contract.
- Store the filled copy with the run (e.g. .mochiko/<workflow>/contract.md), beside the
  deliverable its departure trail lands on.
-->

# Workflow Contract — [WORKFLOW_NAME]

**Workflow:** [WORKFLOW_NAME] · **Carrier:** [commands/<name>.md supervisor | agent team] · **Filled:** [YYYY-MM-DD]
**Why this form exists for this run:** [departed from the command's stated default | declared non-default bounds | non-command loop]

## 1. Done-condition (DEFAULTS TO FAIL)

<!-- The artifact starts FAILing and only flips on real evidence. State all three parts. -->

- **Measurable end state:** [the observable fact that means done — e.g. "the validator returns PASS and every required section is present"]
- **Stated check (how it is proven):** [named in advance — e.g. "the validator agent Reads the artifact and confirms every checklist item"]
- **Constraints (must not be violated):** [e.g. "kernel-free maintained; no acceptance criterion left unverified"]
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
- **Declared cost range:** [e.g. "≈200–350k tokens" — a declared range IS a bound, not an estimate]
- **Counter for each bound above:** [the lead — named, because a bound nobody counts is not a bound]
- **On hitting a guard:** escalate to the human gate with failure context. [never report done on cap exhaustion]

<!-- Bound integrity (command runs): a bound rises ONLY at a user checkpoint, and every
     re-declaration is recorded in §5. Home: command-shape.md Layer 1, The floor, invariant 3. -->

## 4. Human gate

- **Placement:** [every cycle | low validator-confidence only | preference-gap only]
- **Where it fires:** [the phase/condition — e.g. "on low validator confidence, and on any cap-exhaustion escalation"]
- **What the human decides:** [e.g. "accept/override the validator's verdict; final acceptance"]

## 5. Composed process — departures, floor, and counter state

<!-- Command runs only; a non-command loop one-lines this section as "not applicable". -->

- **Departures from the stated default:** [one line each — what the default said, what ran instead, why. This is the same trail line the deliverable carries (P20), not a second record.]
- **Floor gates honored:** [the command's P18 floor gates, each ruled by the user — a departure never eats one]
- **Counted unit this run:** [what the bounds and the seat-lifecycle cadence count — named, because a composed run has no default denominator]
- **Counter state (Recovery reads this on resume):** rounds consumed [N] · bounds declared [list] · departures taken [N] · re-declarations [when, and at which user checkpoint]

---

<!-- All five sections filled with no remaining brackets = the loop is sound per loop-discipline. -->

**Contract version:** v2 (2026-08-01 — `lead-owned-process-flexibility` OQ-2, ruled at
acceptance A2: revived as the **per-run** carrier for a departing command run or a non-command
loop, a default run declaring in one line on its deliverable instead; §3 gains the declared cost
range and the named counter; §5 added for departures, floor gates, counted unit and counter
state; v1 — the fill-in form for every workflow) · **Governed by:** `loop-discipline`
