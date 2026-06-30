# Per-disposition recipes (detail)

Each recipe is body-treatment steps + structural-move steps, then the always-on wiring pass (see `SKILL.md`). Examples reference the Round-1 dry-run dispositions for the HIL `setup` cluster.

---

## Body recipes

### `keep-verbatim`
1. Copy the body unchanged into the mochiko path.
2. Do **not** improve prose "while you're here" — that risks drift from the validated original.
3. Proceed to the structural move + wiring pass.
> Example: `syncing-claude-md` (correctness is machine-decidable; body is clean).

### `port-with-edits`
1. Identify the specific lines coupled to kernel/DAG/catalog or to old paths.
2. Edit only those; preserve section structure, headings, and voice.
3. Note each edit in the trace (`port-with-edits` responsibilities, `kept-but-rebind` for paths).
> Example: `validation-constitution` (good body; de-collide generic triggers, then promote).

### `redesign`
1. Extract the **responsibilities** from the old body (not its mechanism).
2. Rewrite around mochiko primitives (skills/agents/supervisor), kernel-free.
3. Every original responsibility must reappear with a trace tag — redesign is the highest-risk path for silent loss.
> Example: `brownfield-constitution` (redesign its body into a branch of the merged sibling).

> **Command/workflow target shape (altitude).** When the primitive being redesigned IS a loop, the target is a *thin* supervisor whose only job is to stitch a team to a goal under a contract:
> - **goal + lead declaration** (one or two lines);
> - **the team casting** — which agent produces, which independent agent grades;
> - **the per-workflow contract parameters** — *this* loop's done-condition end-state, the cap number, the named human-gate placements;
> - **workflow-unique steps** — anything true only of this workflow (e.g. input enrichment, a prerequisite check, evidence→phase recovery);
> - **references**, not restatements — `loop-discipline` for the four rules/tiers/gap-routing/anti-rationalization, `agent-dispatch` for briefing, and a **filled `workflow-contract` artifact** instead of an inlined contract.
>
> Do NOT restate the four requirements, validator tiers, gap-routing, or a filled contract inline — those `dedupe` into `loop-discipline` (see `assess-primitive` Step 5 Altitude rule). Generic discipline restated inline is the verbosity defect `verify-output` fails.

### `drop`
1. Confirm the responsibility genuinely should not exist in mochiko (usually kernel/DAG/catalog plumbing).
2. Remove the primitive (or the responsibility).
3. Emit a **degenerate trace**: each dropped responsibility tagged `dropped + reason`; the lead/human gate must accept each reason.

---

## Structural recipes

### `standalone`
1. Place the artifact in its own `skills/<name>/` or `agents/<name>.md`.
2. No partner needed (correctness either machine-decidable or no reviewable artifact).

### `split` (1 → N)
1. From the trace, divide responsibilities into the keeper and the spin-out(s).
2. The most common split is **producer ↔ validator**: the original keeps authoring; create a new validator **agent** + a `verify-*` **skill** for grading.
3. Wire the validator into the workflow as the independent gate; never mount both on one agent.
> Example: `principal-architect` → keep authoring; spin out an independent constitution validator; extract the orphaned feasibility-review procedure into a skill.

### `merge-into-sibling` (N → 1)
1. Confirm a genuinely shared core with the sibling.
2. Fold the variant in as a branch (e.g. greenfield/brownfield) of the sibling skill.
3. Keep only the variant-unique slice; tag moved responsibilities `moved-to-sibling-skill`.
> Example: `brownfield-constitution` → into `authoring-constitution` (only Emergent Ceiling + Evolution Notes + the analysis prerequisite are brownfield-unique).

### `promote`
1. Take the check/skill and make it the load-bearing tool of an **independent validator** agent.
2. Ensure the validator agent is different from any producer agent that mounts the sibling skills.
> Example: `validation-constitution` → the concrete instance of external-grounded-validation for the constitution loop.

### `absorb-into-lead`
1. Confirm the primitive is pure orchestration with no reusable body.
2. Move only **workflow-specific** orchestration into the `commands/*.md` supervisor (the lead): sequencing unique to this workflow, this loop's contract parameters, workflow-unique steps. Tag these `moved-to-lead`.
3. **Do not absorb generic loop-discipline.** Default-FAIL, independence, the four guards, validator tiers, tamper-proofing, gap-routing, and anti-rationalization already live in `loop-discipline` (+ `workflow-contract`, `agent-dispatch`) — the lead **references** them and those responsibilities are tagged `dedupe`. Absorbing them copies the kernel into the command body (the altitude defect).
4. Delete the now-empty primitive.

### `rewire-cluster`
1. Leave the body essentially as-is.
2. Fix the surrounding wiring: the agent's `skills:` list, the dispatch order, the missing gates.
> Example: `authoring-constitution` (skill is near-clean; the fix is removing `validation-constitution` from the producer agent and wiring the independent gate — work in the agent/workflow, not `SKILL.md`).

---

## The wiring pass (reproduced for convenience)

Runs on every recipe above:

1. **Classification** — user-invoked (`disable-model-invocation: true`) vs model-invoked; agents get a `skills:` list.
2. **Router registration** — add to the `mochiko` router with when-to-reach-it guidance.
3. **Triggers** — graded exact-phrase, work-context triggers for model-invoked skills.
4. **Path rebinding** — `.humaninloop/` → `.mochiko/`; drop kernel/catalog/MCP paths; record `kept-but-rebind`.
5. **Decouple** — remove sibling-agent names (independence by *role*), the word `dispatch`, workflow modes/paths/phases in a persona, and "workflow-agnostic" meta-labels; keystone-test the rest.
6. **Single-source / de-duplicate** — reference shared doctrine + templates (`loop-discipline`, `workflow-contract`, `agent-dispatch`) rather than restating them; a command references `loop-discipline` and fills a contract artifact, never inlines the rules or a filled contract.
