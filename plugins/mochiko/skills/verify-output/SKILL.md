---
name: verify-output
description: This skill MUST be invoked to grade a transformed primitive against the transform done-condition — confirming conformance to the five conventions and sound-loop placement, and auditing the responsibility trace for silent loss. SHOULD also invoke whenever a transform-recipes artifact needs an independent PASS/FAIL before it is accepted. The done-condition checker; defaults to FAIL; run by a DIFFERENT agent than the one that produced the artifact.
---

# Verify Output

## Overview

Grade a transformed primitive (or cluster) against the transform done-condition: it must (a) **conform** — satisfy the five conventions, sit correctly in a sound loop, and be kernel-free — and (b) carry a **complete responsibility trace** — every original responsibility accounted for, every drop justified. The verdict **defaults to FAIL** and only flips on evidence Read from the artifact itself.

**This skill is the independent gate.** It MUST be run by a different agent than the one that produced the artifact (the `transform-validator`, not the `transform-producer`). An author grading their own work is not verification — it is the exact self-grade failure mochiko exists to prevent.

```
NO PASS WITHOUT EVIDENCE READ FROM THE ARTIFACT.
A PASS asserted from the producer's report, a summary, or "it looks right" is INVALID.
The result is FAIL until each check is confirmed against the real file.
```

**Violating the letter of these checks is violating the spirit.** "It basically conforms" is FAIL until proven PASS.

## When to Use

- After `transform-recipes` produces an artifact + trace, before it is accepted
- As the verify stage of the transform loop, run by the independent validator
- To audit a whole cluster's transformation against the done-condition

## When NOT to Use

- By the agent that produced the artifact (independence violation — hand to `transform-validator`)
- To decide or apply a disposition (→ `assess-primitive` / `reconcile-cluster` / `transform-recipes`)

## Part A: Conformance check

Read the artifact. For each item, confirm against the file — not the report.

| # | Convention / requirement | PASS means (confirmed in the artifact) |
|---|--------------------------|----------------------------------------|
| 1 | **Classification** | Frontmatter declares user-invoked (`disable-model-invocation: true`) or model-invoked; no skill invokes another user-invoked skill. |
| 2 | **Discoverability** | Registered in the `mochiko` router with when-to-reach-it guidance. |
| 3 | **Reliable model-invocation** | Model-invoked skills carry graded, exact-phrase, work-context triggers in `description`. |
| 4 | **Agent↔skill composition** | Agents declare a `skills:` list; persona (cares-about) in the agent, procedure (how) in the skill. |
| 5 | **Producer↔validator pairing** | If the artifact emits a reviewable artifact, an independent validator (different agent + different skill) exists — or correctness is machine-decidable and the check degenerates to a deterministic assert (note which). |
| 6 | **Sound-loop placement** | The loop it sits in has a pre-declared done-condition (default FAIL), independent validation, bounded iteration, and a named human gate — i.e. a filled `workflow-contract`. |
| 7 | **Kernel-free** | No Python/MCP brain code, no DAG/catalog dependency, no orchestration plumbing reintroduced. |

Any item not confirmed against the artifact → **FAIL**.

## Part B: Trace audit

Read the responsibility trace against the original primitive.

1. **Completeness** — every responsibility the original held appears in the trace with a tag. A missing responsibility is a **silent drop** → FAIL.
2. **Justified drops** — every `dropped` carries a reason, and the reason is one the lead/human gate accepted. A bare or unaccepted drop → FAIL.
3. **Landing integrity** — every `moved-to-*` names a real destination, and that destination actually received the responsibility (spot-check the receiving primitive).
4. **No capability loss** — the union of all tags covers the original's behavior. If a user could do X before and can do X nowhere after, that is loss → FAIL.

## Step sequence

1. **Read the artifact file(s)** — record what you Read (the tamper-proofing: no PASS without this).
2. Run **Part A** conformance, item by item.
3. Run **Part B** trace audit against the original primitive.
4. Emit a **binary verdict**. No "mostly passes."

## Output format

```
VERIFY: <primitive-or-cluster>
Evidence read: <files Read this run>           # required; absent ⇒ verdict is FAIL
Conformance: [1..7 each PASS/FAIL with one-line evidence]
Trace audit: [completeness/justified-drops/landing/no-loss each PASS/FAIL]

VERDICT: PASS | FAIL
Failing items: <list, or "none">
Required fixes: <specific, actionable — or "none">
```

## Red Flags — STOP, the verdict is FAIL

If you catch yourself thinking any of these, you are rationalizing a PASS:

- "The producer said it conforms"
- "It looks complete, I don't need to open the file"
- "The trace is probably fine"
- "This drop is obviously okay" (without an accepted reason)
- "Close enough to pass"
- "I wrote part of this, so I know it's good" (you should not be grading it at all)

## Common Rationalizations

| Excuse | Reality |
|--------|---------|
| "Reading the producer's report is enough" | The report is the producer's claim. Verification reads the artifact. |
| "Machine-decidable, so skip the check" | Then run the deterministic assert and record its result — that *is* the evidence. |
| "One missing responsibility is minor" | Silent capability loss is the failure this skill exists to catch. FAIL. |
| "The drop is clearly justified" | Justification the lead accepted, in writing, or it's an unaccepted drop. FAIL. |
| "It mostly conforms" | The verdict is binary. Mostly = FAIL with a fix list. |

## Related

- `transform-recipes` — produces the artifact this skill grades (by a different agent)
- `assess-primitive` — emits the trace this skill audits
- `loop-discipline` — defines the sound-loop placement checked in Part A item 6
