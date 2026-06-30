---
name: verify-output
description: This skill MUST be invoked to grade a transformed primitive against the transform done-condition — confirming conformance to the five conventions and sound-loop placement, and auditing the responsibility trace for silent loss. SHOULD also invoke whenever a transform-recipes artifact needs an independent PASS/FAIL before it is accepted. The done-condition checker; defaults to FAIL; run by a DIFFERENT agent than the one that produced the artifact.
---

# Verify Output

## Overview

Grade a transformed primitive (or cluster) against the transform done-condition: it must (a) **conform** — satisfy the five conventions, sit correctly in a sound loop, be kernel-free, and sit at the right **altitude** (reference shared doctrine, never restate it) — and (b) carry a **complete responsibility trace** — every original responsibility accounted for, every drop justified. The verdict **defaults to FAIL** and only flips on evidence Read from the artifact itself.

**This skill is the independent gate.** It MUST be run by someone who did **not** author the artifact — a different agent than the one that produced it. An author grading their own work is not verification — it is the exact self-grade failure mochiko exists to prevent.

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

- By the agent that produced the artifact (independence violation — it must be graded by someone who did not author it)
- To decide or apply a disposition (→ `assess-primitive` / `reconcile-cluster` / `transform-recipes`)

## Part A: Conformance check

Read the artifact. For each item, confirm against the file — not the report.

| # | Convention / requirement | PASS means (confirmed in the artifact) |
|---|--------------------------|----------------------------------------|
| 1 | **Classification** | Frontmatter declares user-invoked (`disable-model-invocation: true`) or model-invoked; no skill invokes another user-invoked skill. |
| 2 | **Discoverability** | Registered in the `mochiko` router with when-to-reach-it guidance. |
| 3 | **Reliable model-invocation** | Model-invoked skills carry graded, exact-phrase, work-context triggers in `description`. |
| 4 | **Agent↔skill composition & decoupling** | Agents declare a `skills:` list; the persona carries self-sufficient method + judgment, the skill carries the procedure. **AND the artifact is decoupled** — run the deny-list scan below over any persona or skill; any hit the keystone test confirms as coupling → FAIL. |
| 5 | **Producer↔validator pairing** | If the artifact emits a reviewable artifact, an independent validator (different agent + different skill) exists — independence is *structural*, never a persona's self-declaration — or correctness is machine-decidable and the check degenerates to a deterministic assert (note which). |
| 6 | **Sound-loop placement** | The loop it sits in has a pre-declared done-condition (default FAIL), independent validation, bounded iteration, and a named human gate — i.e. a filled `workflow-contract`. |
| 7 | **Kernel-free** | No Python/MCP brain code, no DAG/catalog dependency, no orchestration plumbing reintroduced. |
| 8 | **Altitude / single-source** (commands & loops) | The artifact **references** shared doctrine rather than restating it: a command cites `loop-discipline`, fills a `workflow-contract` *artifact* (does not inline one), and inlines none of the four rules / validator tiers / gap-routing / anti-rationalization. Every substantive line is workflow-specific or a reference. Run the altitude scan below. |

Any item not confirmed against the artifact → **FAIL**.

### The decoupling scan (Part A item 4 — grep first, then judge)

Decoupling is proven by the **absence** of workflow coupling, not by a declaration of independence. Grep any persona or skill for the deny-list; treat each hit as a candidate FAIL and confirm with the keystone test.

**Deny-list (grep these):** sibling-agent **names** (e.g. `transform-validator`, `principal-architect`) · the word **dispatch** · **workflow-agnostic** and other independence-by-declaration meta-labels ("you are the producer/validator in a pair", "you are reusable") · injected workflow **modes/paths/phases** in a persona (mode names like `greenfield`/`brownfield`/`amend` stated as run-scope, workspace paths like `.mochiko/…` baked into a persona, phase names).

**Keystone test (judge each survivor):** *would this line be true of this professional on **any** job? → craft, keep. Does it only make sense inside **one** workflow? → coupling, FAIL.* Intrinsic traits survive (a reviewer *is* independent; an author does not grade their own work — integrity). This-loop machinery fails.

**Allowed (do not flag):** role words — "graded by an independent validator," "the command lead owns the loop," "by someone who did not author it" — state independence by *role*, not by an agent name. Skills legitimately keep their own disposition **and** procedure (they run agent-less). Caller-side files (`commands/*.md`, the router, `agent-dispatch.md`) may name agents and own workflow knowledge — the scan targets **personas and skills**, not callers.

### The altitude scan (Part A item 8 — grep first, then judge)

Applies to `command`/`workflow` artifacts (and any primitive that could restate shared doctrine). Altitude is proven by the **absence** of duplicated doctrine: the command stitches a team to a goal under a contract and *references* the rest. Grep first, then judge each survivor with the keystone test.

**Deterministic floor (grep — each hit is a candidate FAIL):**

- **Inlined filled contract** — the command body contains a filled `workflow-contract` (section headers like "Producer ↔ Validator", "Bounded iteration", "DEFAULTS TO FAIL", an independence-check line). The contract must be a runtime *artifact* (e.g. `.mochiko/<workflow>/contract.md`), referenced — not pasted into the command.
- **Restated doctrine** — the command restates rules that live in `loop-discipline`: the validator trustworthiness tiers, the three gap-types and their routing, tamper-proofing prose, anti-rationalization lists ("you cannot rationalize", "exhaustion ≠ done"), or a "Supervisor behaviors" section that re-lists the four requirements.
- **Missing reference** — the command does NOT cite `loop-discipline` by name (it must).
- **Transliterated mechanics** — literal `Task(...)` / `AskUserQuestion(...)` payload bodies or a hand-coded round counter spelled out where a reference to `agent-dispatch` + the contract's cap would do.

**Keystone test (judge each survivor):** *would this line be true of any sound mochiko loop? → it belongs in `loop-discipline`; FAIL as duplication. Is it true only of THIS workflow? → it is the per-workflow parameter the command legitimately owns; keep.*

**Allowed (do not flag):** a one-line goal + lead declaration; the team-casting table; *this* loop's contract parameters (done-condition end-state, the cap number, the named gate placements); workflow-unique steps; an evidence→phase recovery table; and references by name to `loop-discipline` / `workflow-contract` / `agent-dispatch`. A brief restatement of a *workflow-specific* done-condition is craft, not duplication.

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
- "The persona *says* it's workflow-agnostic, so it's decoupled" (a declaration is not the absence — grep the deny-list)
- "I wrote part of this, so I know it's good" (you should not be grading it at all)
- "The command spells out the loop, so it's clearly disciplined" (restated doctrine is the altitude defect — it must *reference* `loop-discipline`, not copy it)

## Common Rationalizations

| Excuse | Reality |
|--------|---------|
| "Reading the producer's report is enough" | The report is the producer's claim. Verification reads the artifact. |
| "Machine-decidable, so skip the check" | Then run the deterministic assert and record its result — that *is* the evidence. |
| "One missing responsibility is minor" | Silent capability loss is the failure this skill exists to catch. FAIL. |
| "The drop is clearly justified" | Justification the lead accepted, in writing, or it's an unaccepted drop. FAIL. |
| "It mostly conforms" | The verdict is binary. Mostly = FAIL with a fix list. |
| "More inline detail = more disciplined" | Restated doctrine drifts from its single source and is the verbosity defect. Reference `loop-discipline`; fill a contract *artifact*. Inlined doctrine = FAIL. |

## Related

- `transform-recipes` — produces the artifact this skill grades (by a different agent)
- `assess-primitive` — emits the trace this skill audits
- `loop-discipline` — defines the sound-loop placement checked in Part A item 6
