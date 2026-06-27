---
name: transform-recipes
description: This skill MUST be invoked to apply a finalized disposition (body × structural) to a primitive — turning the decision into an actual mochiko-form artifact — and to run the convention-wiring pass that EVERY ported primitive pays. SHOULD also invoke whenever an assessed and reconciled primitive is ready to be edited. Produces the artifact plus an updated responsibility trace; never grades its own output.
---

# Transform Recipes

## Overview

Apply a **finalized disposition** (a body treatment × a structural move, decided by `assess-primitive` and `reconcile-cluster`) to a primitive, producing the mochiko-form artifact and updating its responsibility trace. A disposition is two orthogonal choices plus a pass that always runs:

- **Body treatment** — what happens to the content
- **Structural move** — how it sits in the cluster
- **Convention-wiring pass** — runs on *every* port, no exceptions (this is why there is no `keep-as-is`)

Recipes are branches in this one skill, keyed to the vocabulary. Full step lists per disposition in [references/RECIPES.md](references/RECIPES.md).

**Transform applies a decision; it does not make one.** If you find yourself choosing between `merge` and `split` here, stop — that belongs to `reconcile-cluster`. And transform never grades its own output: that is `verify-output`, run by a different agent.

## When to Use

- Once a disposition is finalized for a primitive (post-assess, post-reconcile)
- To execute the convention-wiring pass on any ported primitive, including trivial ones
- To produce the transformed artifact + updated trace handed to `verify-output`

## When NOT to Use

- To decide the disposition (→ `assess-primitive` / `reconcile-cluster`)
- To verify the result (→ `verify-output`)

## The two-axis vocabulary

### Body treatment (content)

| Value | When | Recipe gist |
|-------|------|-------------|
| `keep-verbatim` | body is already mochiko-clean | copy the body unchanged; still run the wiring pass |
| `port-with-edits` | mostly good, localized fixes | edit the specific lines; preserve structure and voice |
| `redesign` | body assumes a kernel/DAG/catalog, or the approach is wrong for mochiko | rewrite around mochiko primitives; carry the responsibilities, not the mechanism |
| `drop` | the responsibility shouldn't exist in mochiko | remove; record `dropped + reason`; still emit a (degenerate) trace so the drop is auditable |

### Structural move (placement)

| Value | When | Recipe gist |
|-------|------|-------------|
| `standalone` | self-contained, one home | place as-is in its directory |
| `split` (1→N) | emits an artifact with no independent validator, or one agent produces+grades | spin out the partner (esp. producer↔validator); divide responsibilities by the trace |
| `merge-into-sibling` (N→1) | thin variant over a shared core | fold into the sibling as a branch; keep only the variant-unique slice |
| `promote` | a check that confers a validator role | elevate to an independent validator's load-bearing tool |
| `absorb-into-lead` | pure orchestration with no reusable body | move into the command supervisor; leave no orphan skill |
| `rewire-cluster` | body is fine; the problem is the agent/workflow around it | leave the body; fix the `skills:` list, the dispatch, the gates |

## The convention-wiring pass (ALWAYS runs)

Even `keep-verbatim × standalone` pays this. The floor is never zero-work. Run all five:

1. **Classification tag** — set user-invoked (`disable-model-invocation: true`) or model-invoked (default). Agents get a `skills:` list; the persona-vs-procedure split is honored.
2. **Router registration** — register the primitive in the `mochiko` router with when-to-reach-it guidance (user-invoked entries are *hinted*, not fired).
3. **Trigger phrasing** — for model-invoked skills, graded exact-phrase triggers in `description` describing the *work context*. (Agent-consumed skills describe transformation work, not "when the user says…", to avoid false auto-trigger expectations.)
4. **Path rebinding** — `.humaninloop/` → `.mochiko/`; drop catalog/MCP/DAG paths; fix prerequisite handoffs. Record each as `kept-but-rebind`.
5. **Decouple persona/skill** — scrub any persona or skill of workflow coupling: remove sibling-agent names (state independence by *role*), the word "dispatch," injected workflow modes/paths/phases in a persona, and "workflow-agnostic"/independence-by-declaration meta-labels. Keystone-test the rest (true of this professional on any job → keep); push caller-side context to `agent-dispatch.md`, not the primitive. Audited by `verify-output`'s decoupling scan.

## Step sequence

1. Read the finalized disposition + responsibility trace for the primitive.
2. Apply the **body** recipe → the content.
3. Apply the **structural** recipe → its placement and any new partner primitives (from reconcile).
4. Run the **convention-wiring pass** (all five).
5. Update the **trace**: flip each responsibility to its realized tag; confirm every one is accounted for.
6. Hand the artifact + updated trace to `verify-output` (via a different agent).

## Output format

```
TRANSFORM: <primitive-name>
Applied:   <body> × <structural> + wiring-pass
Artifacts: <paths created/edited>
New partners: <any split/promote products, e.g. new validator agent + skill>
Wiring:    classification=<...> router=<registered?> triggers=<...> rebinds=<list>
Trace (realized): <responsibility → final tag>...
```

## Common Mistakes

| Mistake | Fix |
|---------|-----|
| Treating a trivial primitive as zero-work | The wiring pass always runs. `keep-as-is` is a lie. |
| Re-deciding the disposition mid-transform | Go back to assess/reconcile; transform only applies. |
| Grading the result here | Independence requires `verify-output` run by a different agent. |
| Dropping without a trace | Even `drop` emits a trace so the removed responsibilities are auditable. |
| Splitting producer + validator skills onto one agent | The split exists precisely to keep them on different agents. |
| Leaving workflow vocab in a persona/skill | The wiring pass decouples it. A persona naming a sibling agent or "dispatch," or declaring itself "workflow-agnostic," fails `verify-output`. |

## Related

- `assess-primitive` / `reconcile-cluster` — decide the disposition this skill applies
- `verify-output` — independently grades the artifact this skill produces
- `loop-discipline` — the conventions the wiring pass enforces
