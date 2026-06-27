# Transform — `agents/requirements-analyst.md` (P2, PRODUCER)

Run: `specify` cluster transform · Phase 3 (transform) · Producer: `transform-producer` · Skill: `mochiko:transform-recipes`
Source: `human-in-loop/plugins/humaninloop/agents/requirements-analyst.md`
Inputs: reconcile §D row P2 · §E P2 trace · §G wiring · `assess-requirements-analyst.md`
Disposition (FINALIZED, human-gate-ACCEPTED): **port-with-edits × standalone** · Team-role: **PRODUCER** · Gate: none (§F "not gated by §4 — persona-clean port")

> **Transform applies a decision; it does not make one, and it does not grade its own output.** The grade is `verify-output`, run by a different agent.

---

## TRANSFORM output block

```
TRANSFORM: requirements-analyst
Applied:   port-with-edits × standalone + wiring-pass
Artifacts: plugins/mochiko/agents/requirements-analyst.md (created)
New partners: none (producer↔validator pair already exists; validator = separate agent, disjoint skills — CONFIRMED, not constructed)
Wiring:    classification=agent(skills-list, bare) · router=deferred-to-router-agent (not edited here) · triggers=n/a-for-agents(<example> blocks instead) · rebinds=[humaninloop:authoring-requirements→mochiko:, humaninloop:authoring-user-stories→mochiko:]
Trace (realized): every responsibility carries a realized tag (table below); 0 silent drops
```

---

## Body recipe applied — `port-with-edits` (light end)

Per RECIPES `port-with-edits`: edited only the coupled/duplicated lines; preserved section structure, headings, and voice. The HIL section order is preserved (intro → Skills Available → Core Identity → What You Produce → Your Process → Quality Standards → What You Reject → What You Embrace → Your Judgment); **not** reordered to match `principal-architect` (port-with-edits preserves source structure).

| Edit | What changed | Why |
|------|--------------|-----|
| **Persona prose — KEPT VERBATIM** | Intro line, Core Identity, What You Produce, Your Process (the 5-step method), What You Reject, What You Embrace, Your Judgment — byte-for-byte from HIL | Disposition: "keep the persona prose + 'Your Process' verbatim (it ports clean)". Keystone-clean; 0 deny-list hits. |
| **Skill refs → `mochiko:` rebind** | "Skills Available" entries `authoring-requirements`/`authoring-user-stories` → **`` `mochiko:authoring-requirements` ``** / **`` `mochiko:authoring-user-stories` ``** (matching `principal-architect` body convention); closing line points to the skills "for your output artifacts" | Wiring step 4 (namespace rebind). Frontmatter `skills:` stays **bare** (per assessment §6 + `principal-architect` precedent). Both skills **stay mounted**. |
| **Quality Standards → `dedupe`** | Dropped the literal format templates (`As a [role]…`, the must/shall format list, the literal AC format) **from the persona**; kept the analyst's *taste* (measurable-over-vague + reject "fast/user-friendly/secure", testable pass/fail + specific + complete, independent + benefit-anchored); added a pointer: the literal templates "live in `mochiko:authoring-user-stories` and `mochiko:authoring-requirements`… one source of truth" | Disposition: dedupe the duplicated format spec → defer to the two mounted skills as canonical home; keep the persona's judgment. Mirrors `principal-architect`'s "Essential Floor Knowledge" dedupe precedent (taste stays; canonical definition deferred to the skill). |

## Structural recipe applied — `standalone`

Placed as a single self-contained file in `agents/`. No partner spun out, none merged, nothing promoted, nothing absorbed.

- **No `split`** — `skills:` are both *producing* skills (no grader on this agent) → no self-grade leak to fix.
- **No `pair`/new agent** — the validator partner already exists as a **separate** agent with a disjoint skill set (reconcile §C: "none — pair already exists"). Independence holds **by construction**, not by anything declared in this persona.
- **No `merge`/`promote`/`absorb`** — complementary (not variant) sibling; not a check; a real reusable producer body.
- The DAG→agent-team rewiring of the *cluster* is **P1's** structural move (the command lead), not this agent's. This agent simply lands `standalone`.

---

## Convention-wiring pass (all five — always runs)

1. **Classification** — Agent. Carries a `skills:` list (bare: `authoring-requirements, authoring-user-stories`). Agents are **not** user/model router-classified (§G: "P2/P3 → agents; not router-classified as user/model"). Persona-vs-procedure split honored: persona (what the analyst *cares about*) in the agent; procedure (step-by-step *how* to format) in the two skills.
2. **Router registration** — **NOT performed here.** Per task instruction, the shared router (`skills/mochiko/SKILL.md`) is owned by a dedicated wiring agent. §G earmarks registering `requirements-analyst` in the `mochiko` router — recorded as a pending router-agent edit, not applied in this primitive transform.
3. **Trigger phrasing** — N/A for agents (no model-invoked `description` trigger grammar). The mochiko agent convention substitutes **`<example>` blocks** (absent in HIL) — **3 added**, all producer/spec-authoring contexts (full-spec-from-vague-request; quantify "fast"→SC-XXX; prioritized user stories + Given/When/Then), each with `<commentary>`.
4. **Path rebinding** — `humaninloop:` → `mochiko:` on the two body skill refs (3 occurrences: lines 46, 47, 84). **No hardcoded filesystem paths** existed in the HIL persona (it carried no write-location handoff), so none to rebind — confirmed by grep (no `specs/`, `.workflow/`, `.humaninloop/`).
5. **Decouple persona/skill** — **0 deny-list tokens** (grep-confirmed: no sibling-agent names, no "dispatch," no workflow modes/paths/phases, no "workflow-agnostic"/independence-by-declaration meta-label, no hardcoded write-location). The persona is **decoupled by absence** (carried over from HIL). Independence is stated **by role** in the description — "Authors the specification; does not grade its own output" — naming **no** sibling agent and **no** workflow.

---

## Realized responsibility trace (every responsibility tagged; 0 silent drops)

| # | Responsibility | Assess tag | Realized tag | Where it landed |
|---|----------------|-----------|--------------|-----------------|
| 1 | Analyst persona & judgment (transform ambiguity→clarity; state assumptions; flag critical gaps; never guess on security/data/user-facing) | `kept` | **`kept`** | Core Identity / What You Produce / What You Reject / What You Embrace / Your Judgment — verbatim |
| 2 | "Your Process" 5-step analytical method (core need → actors → happy path → edges → success) | `kept` | **`kept`** | "Your Process" — verbatim (keystone-clean) |
| 3 | Functional-requirements authoring (FR-XXX / SC-XXX / edge cases) via `authoring-requirements` | `kept-but-rebind` | **`kept-but-rebind`** | `mochiko:authoring-requirements` (body lines 46, 84); **stays mounted** in frontmatter (bare) |
| 4 | User-story authoring (P1/P2/P3, Given/When/Then) via `authoring-user-stories` | `kept-but-rebind` | **`kept-but-rebind`** | `mochiko:authoring-user-stories` (body lines 47, 84); **stays mounted** in frontmatter (bare) |
| 5 | PRODUCER team-role (authors `spec.md` when invoked) | `kept` | **`kept`** | Producer-only role preserved; independence-by-role in description; paired with the existing separate validator (skills ∩ = ∅) — no self-grade leak |
| 6 | "Quality Standards" literal format templates that duplicated the two mounted skills | `dedupe` | **`dedupe`** | Literal formats removed from persona; deferred to the two skills (single source of truth); the analyst's *taste* kept in "Quality Standards" |
| 7 | Classification + router/discoverability (`<example>` blocks per mochiko agent convention) | `kept-but-rebind` | **`kept-but-rebind`** | Classification set (agent + skills list); 3 `<example>` blocks added; router registration deferred to the dedicated router agent |

**Orchestration note (no tag originates here):** being invoked, fed inputs, and looped on the validator's verdict are **not** this agent's responsibilities — they re-home onto the `specify` command lead (P1, reconcile §B). This producer emits **no `moved-to-lead`** responsibility (it never owned the loop). The independent-validation gate the loop needs is already homed on the separate validator; the bounded loop + human gate are P1's.

**No responsibility is dropped.** Every responsibility carries a realized tag → transform done-condition's trace requirement met.

---

## Independence (producer-only) — CONFIRMED

- `skills:` = `{authoring-requirements, authoring-user-stories}` — **both producing**, neither a grader. Producer skills ∩ validator skills (the separate validator's `analysis-specifications`) = **∅**.
- The agent **authors** the spec; it **does not grade** it. Stated by role in the description, naming no sibling/workflow.
- This transform produced the artifact and **did not grade it** — grading is `verify-output`, run by a different agent.

## Deny-list / decoupling — CONFIRMED CLEAN

- **0 deny-list tokens** (grep over the full file): no `devil|advocate|state-analyst|supervisor|ui-designer|principal|…`, no `dispatch`, no `workflow|phase|node|catalog|dag|mcp|brain|.humaninloop|greenfield|brownfield`, no `workflow-agnostic`, no hardcoded `specs/`/`.workflow/` path.
- No `humaninloop:` namespace leftovers; both body skill refs are `mochiko:`-prefixed; frontmatter `skills:` is bare.
- Matches the assessment's "decoupled by absence" finding and the run's empirical decoupling-doctrine expectation.

## Deviations / notes

1. **One-clause role-framing added to the description** — "Authors the specification; does not grade its own output." This is the producer-by-role independence framing the disposition asked to *confirm*; it names no sibling and no workflow (keystone-clean: an author does not grade their own work). Recorded as a deliberate, in-scope addition, not a silent change.
2. **HIL section order preserved** (port-with-edits convention) — deliberately **not** reordered to match `principal-architect`'s layout.
3. **Router + `loop-discipline` NOT edited** — per task instruction, those are owned by dedicated agents; this transform touched only the agent artifact. The router entry for `requirements-analyst` is earmarked in §G for the router-wiring pass.

## Handoff

Artifact (`plugins/mochiko/agents/requirements-analyst.md`) + this realized trace → **`verify-output`**, run by a **different** agent (Phase 4). The trace stands alone for an independent reader to audit: every responsibility has a realized tag, independence is producer-only, and the deny-list scan is clean.
