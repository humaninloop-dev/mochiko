# Agent ↔ Workflow Decoupling — Analysis Synthesis

> Brainstorm output (analysis-iterative). Design artifact feeding the build stage.
> **Two passes:** (1) the design layer — what to decouple and why; (2) the operational layer —
> how a persona is written so coupling cannot creep back. Pass 2 **revised** pass 1 (see the
> Operational rules); where they differ, pass 2 wins.

## Problem Statement

The primitives migrated from human-in-loop carry their coupling forward: agent personas name their
**sibling agents**, assume a specific **supervisor/loop**, and **bake their loop role** into the
persona. The goal is the inverse: an agent that knows only its **craft** and **the skills it can
lean on**, reusable by any loop.

The pass-2 sharpening — learned the hard way, by my first (wrong) edits re-introducing coupling in
disguise:

> **Decoupling is proven by the *absence* of coupling, not by a *declaration* of independence.**
> You do not make an agent workflow-agnostic by *telling* it so and then handing it a
> workflow-shaped input list. You make it so by ensuring its persona contains **no trace** of any
> workflow — no modes, no paths, no phases, no role-labels, no "dispatch," no "I am decoupled"
> meta-commentary. An agent is a **self-contained professional**: it carries expertise, reaches for
> tools (skills) when the work is in their domain, and **degrades gracefully** — competent even with
> a thin instruction or an ill-fitting skill, because the competence lives in the persona, not in
> the scaffolding around it.

## Context & Constraints

- **Kernel-free, non-negotiable.** Decoupling is achieved by how primitives are written, not new plumbing.
- **Producer↔validator independence is load-bearing** (`loop-discipline`). It must relocate, not vanish.
- **`single-source-rule-fanout` is house law.** A rule lives in one place. Re-stating the input
  contract inside every agent (pass 1's "touch of C") is exactly the drift that let workflow
  vocabulary back in — so the input shape is single-sourced on the caller side, not per agent.
- **The transform cluster ports the next cluster** (`specify`). The rules must land in the
  conventions and the transform skills, or the next port reproduces the coupling.

## Key Decisions

| # | Decision area | Choice | Confidence | Status |
|---|---------------|--------|------------|--------|
| D1 | Decoupling model | **Thin persona / fat dispatch.** Independence becomes **structural** (different agent + disjoint skills, guaranteed by the loop), not a persona trait. | Confident | Held; **deepened** by the operational rules (O1–O4) into "self-contained professional / decoupling by absence." |
| D2 | Context-passing mechanism | ~~Formal dispatch template **+ each agent declares an "Inputs I require" interface.**~~ → **REVISED:** the agent declares **no** input interface; the input shape is single-sourced on the caller side (`agent-dispatch.md`), which is a **briefing guide, not a hard gate**. | Confident | **Revised** by O1 + O4. The "touch of C" is removed — it was the coupling vector. |
| D3 | Agent inventory | **Collapse the two validators into one generic `validator`; keep producers domain-specialized.** | Confident | Held. Personas now written per O1–O3. |
| D4 | Where the doctrine lands | **Rewrite conventions 4 & 5 in place; enforcement threaded through assess → recipes → verify.** | Confident | Held; convention 4 reworded further by O2, enforcement sharpened by O3. |

### Operational rules (the pass-2 deepening — these govern the rewrite)

| # | Rule | Decision |
|---|------|----------|
| O1 | **Inputs** | The persona describes **no inputs** and never uses the word "dispatch." Robustness comes from *professional character* — "if you lack what you need to do this well, ask; don't invent it" — which references nothing external. |
| O2 | **Persona vs. skill** | Persona = **self-sufficient general method + judgment + standards + output-shape** (competent alone). Skill = the **exact, standardized, repeatable procedure** (checklist/steps/templates — the rigor layer). The persona may state the *standard and the output shape* but must **not restate the skill's step list** (single-source). Skills are **strong assets**: leaned on first when the work is in their domain; fallen back from when it isn't. |
| O3 | **The keystone test** | For every line of a persona: **"Would this be true of this professional on *any* job? → craft, keep. Does it only make sense inside *one* workflow? → coupling, cut."** Intrinsic professional traits survive (a reviewer *is* independent/skeptical; doesn't grade their own work — integrity, true everywhere). This-loop machinery goes. |
| O4 | **Caller side** | `agent-dispatch.md` is a **briefing guide** (loop quality), not a correctness gate — the agent degrades gracefully, so a thin brief is suboptimal, not broken. The command **describes the work + context** (inputs, output location, what "good" looks like, any prior feedback) and **may name the skill as a hint**; the agent stays innocent. Caller-side knowledge of the agent's skills is fine. |
| O5 | **Skills too** | Skills get the same decoupling as agents — **no agent names** (state independence by *role*: "graded by someone who didn't author it", not "by `transform-validator`"), siblings referenced as **pointers not dependencies**, each skill **readable on its own**. The one difference: a skill is invoked **agent-less** (the session fires it directly), so it stays **self-sufficient** — it keeps its operating disposition *and* procedure. The persona/skill split single-sources the **procedure** in the skill and the **craft** in the persona, with the shared **disposition** deliberately in both (each must stand alone). No splitting or re-layering beyond decoupling. |

### The persona allow/deny list (O3 made concrete — the enforcement target)

**Allowed (craft — keep):** the profession and traits true of it everywhere (architect: opinionated
about enforceability; reviewer: independent, skeptical, evidence-demanding; migrator: thinks in
responsibilities not files) · judgment, standards, values (Three-Part Rule; default-FAIL-until-evidence;
minimalism; "no silent capability loss") · failure modes guarded against · the *shape/quality* of
output produced · the `skills:` list framed as assets to lean on in their domain · professional
character / graceful degradation ("ask if you're missing something; use your method when a tool doesn't fit").

**Forbidden (coupling — cut):** names of other agents · orchestration words as this-loop position
(`supervisor`, `lead`, `dispatch`, "your instructions will provide", "this cluster", "the loop",
`round`/`fix-list`/`cap`) · workflow vocabulary (mode names like `greenfield`/`brownfield`/`amend`,
phase names, workspace paths like `.mochiko/memory/…`, workflow-specific artifact filenames) ·
meta-labels ("workflow-agnostic", "you are reusable", "you are the producer/validator in a pair") ·
independence framed as loop-mechanics ("don't grade your own output *because the validator does*" —
keep "you don't review your own work" as integrity, drop the loop reference).

**The subtle line:** profession-*trait* (keep) vs. loop-*position* (cut).

**Applies to skills too (O5):** a skill names no agent — "run by `transform-validator`, not
`transform-producer`" → "run by someone who did **not** author the artifact"; "the producer" /
"the supervisor" → the *role* (the author / the loop's owner) without the agent name. A skill stays
self-sufficient (keeps its disposition + procedure) because it can be fired without an agent.

## Decision Trail

### D2 — reversed by the operational pass
- **Pass 1 chose:** dispatch template **+** each agent declares an "Inputs I require" interface (self-documentation).
- **What happened:** implementing it, the interface immediately filled with `greenfield|brownfield|amend`, `.mochiko/memory/constitution.md`, and a grader's "fix-list" — setup-vocabulary baked back into the persona. Hidden coupling, worse-disguised than the visible kind it replaced.
- **Pass 2 chooses:** **no agent-side input interface.** Input shape is single-sourced in `agent-dispatch.md` (caller-side). The agent works without it (degrades to asking/using judgment). Self-documentation moves to the caller-side contract + the `skills:` list.

### Persona vs. skill (O2) vs. convention 4
- **Tension:** convention 4 said "procedure → skill, persona is thin." O-principle says "must work when the skill doesn't fit," implying the persona carries real method.
- **Resolved:** the persona carries *general professional method* (stands alone, competently); the skill carries the *exact* procedure (rigor/consistency). A senior reviewer without your checklist still knows how to review. Convention 4 reworded accordingly.

## Risks

- **My partial edits are now wrong and must be redone.** `principal-architect` was thinned but given
  a coupling-laden "Inputs I require" + a "workflow-agnostic" meta-label; the new `validator` was
  written with "workflow-agnostic" + "run the skill named in your dispatch" + an input interface;
  `agent-dispatch.md` was framed as a hard gate. The *structural* moves are still right (two cluster
  validators deleted; one generic validator; thin producers) — only the **persona content + the
  template framing** need correcting per O1–O4. See Next Steps.
- **Enforcement is judgment, but now has a mechanical core.** The deny-list is grep-able (agent
  names, `supervisor`/`dispatch`, `.mochiko/` paths, mode words, "workflow-agnostic"); `verify-output`
  runs that scan first, then applies the keystone test for the subtle profession-vs-position cases.
- **Validation is empirical.** The real test is porting `specify` and confirming no persona acquires
  a deny-list token.

## Recommended Next Steps

1. **Reframe `agent-dispatch.md`** from hard-gate to **briefing guide**: drop "the agent learns from
   dispatch" and "under-filled = not ready"; keep the eight fields as a *caller's* briefing checklist
   for loop quality; state independence stays structural.
2. **Redo the three agent personas** to the allow/deny list:
   - `validator` — remove "workflow-agnostic", the "run the skill named in your dispatch" line, and
     the input interface; keep the skeptical/independent/evidence-demanding craft and default-FAIL
     method; list `validation-constitution`, `verify-output` as assets to lean on within their domain.
   - `principal-architect` — remove the meta-label and the "Inputs I require" section; keep
     self-sufficient governance-authoring method (Three-Part Rule, enforceability judgment); `skills:`
     as assets.
   - `transform-producer` — same treatment; keep responsibilities-not-files migration craft.
3. **Reword the conventions** in `PLAYBOOK.md` + `ROADMAP.md`: convention 4 → "persona = self-sufficient
   method + judgment; skill = exact procedure; apply the keystone test; no workflow vocabulary in a
   persona." Convention 5 → "independence is structural (different agent + disjoint skills), guaranteed
   by the loop, not asserted in a persona."
4. **`loop-discipline`** — keep the four hard requirements; add the keystone test + "dispatch is a
   briefing guide, not a fifth gate" as guidance; reference `agent-dispatch.md`.
5. **Decouple the existing skills** (`verify-output`, `validation-constitution`, `assess-primitive`,
   `transform-recipes`, `reconcile-cluster`): drop agent names, state independence by role, make each
   readable on its own — but keep their method/disposition (they run agent-less). No splitting/re-layering.
6. **Thread enforcement** through `assess-primitive` (flag deny-list coupling in *agents and skills* —
   agent names, paths, modes, meta-labels), `transform-recipes` (a "decouple persona/skill" step in the
   wiring pass), `verify-output` (deny-list scan + keystone test → FAIL on any hit, for both agents and skills).
7. **Update router / REGISTRY**, then **validate by porting `specify`** under the new rules.
