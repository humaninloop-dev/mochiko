# Transform — `analysis-specifications` (P6)

**Run:** transform-cluster / `specify` · Phase 3 (transform) · **Transformed:** 2026-06-27 · **Producer:** `mochiko:transform-producer`
**Skill used:** `mochiko:transform-recipes` (port-with-edits × standalone) · APPLY ONLY — independent grading is `verify-output` (a different agent).
**Source:** `human-in-loop/plugins/humaninloop/skills/analysis-specifications/SKILL.md` (single file; **no reference bundle** — directory listed, only `SKILL.md` present).
**Inputs:** `reconcile.md` §D row P6 + §E P6 trace + §G wiring + Agenda 1d (verdict-ownership boundary); `assess-analysis-specifications.md`.

---

```
TRANSFORM: analysis-specifications
Applied:   port-with-edits × standalone + wiring-pass
Artifacts: plugins/mochiko/skills/analysis-specifications/SKILL.md  (CREATE; whole dir = 1 file, no references)
New partners: none (validator side of an EXISTING pair — devils-advocate ⇄ requirements-analyst; nothing split/promoted/created)
Wiring:    classification=model-invoked (no disable-model-invocation; skill → no skills: list)
           router=NOT registered here (separate §D line item: skills/mochiko/SKILL.md EDIT — out of this primitive's scope; "DO NOT edit the shared router")
           triggers=work-context, post-draft; de-collided from analysis-iterative (pre-spec enrichment)
           rebinds=[L27 humaninloop:authoring-requirements → mochiko:authoring-requirements]
Trace (realized): every responsibility tagged below; no clearing verdict added; no silent loss.
```

---

## A. Body edits applied (port-with-edits — localized; structure + voice preserved)

| # | Where | Before (HIL) | After (mochiko) | Tag / reason |
|---|-------|--------------|-----------------|--------------|
| 1 | **L19 When-to-Use** (the one canonical HARD decouple hit, §G) | `- When Devil's Advocate reviews specification artifacts` | `- When reviewing a drafted specification for gaps as an independent reviewer` | `kept-but-rebind` — sibling-agent **name** removed; independence stated by **role** (independent reviewer). Agent NOT named. Keystone-survives (true of any spec reviewer). |
| 2 | **L15 When-to-Use** (soft phase vocab) | `- Validating requirements completeness after specification phase` | `- Validating requirements completeness after a spec is drafted` | `kept-but-rebind` — "specification phase" → activity ("after a spec is drafted"). Keystone survives. |
| 3 | **L18 When-to-Use** (soft phase vocab) | `- Quality gate before planning phase begins` | `- As a gap-review checkpoint before downstream planning and design begins` | `kept-but-rebind` — "planning phase" → "downstream planning and design"; "Quality gate" → "gap-review checkpoint" (see Deviation D1: keeps this a gap-finder, not a verdict-gate). |
| 4 | **L74 What-to-Avoid** (soft phase vocab) | `These are valid concerns but belong in the planning phase, not specification.` | `These are valid concerns but belong in later design and implementation work, not specification.` | `kept-but-rebind` — "planning phase" → "later design and implementation work." Defer-technical-to-design point survives. |
| 5 | **L27 When-NOT-to-Use** (namespace rebind) | `- **When spec doesn't exist yet** - Use \`humaninloop:authoring-requirements\` first` | `- **When spec doesn't exist yet** - Use \`mochiko:authoring-requirements\` first` | `kept-but-rebind` — `humaninloop:` → `mochiko:`. Within-cluster ref (P7 created this run). |
| 6 | **L3 description / triggers** (classification + work-context) | `…MUST be invoked when the user says "review spec", "find gaps", "what's missing", or "clarify requirements". SHOULD also invoke when reviewing spec.md for completeness. Focuses on product decisions…` | work-context description (see §C): "reviewing an already-drafted specification for gaps…"; graded MUST/SHOULD; post-draft anchor; de-collision pointer to `mochiko:analysis-iterative`; explicit "gap-finding INPUT, not a clearing PASS/FAIL." | `kept-but-rebind` — agent-consumed skill ⇒ describe the **work context**, not "when the user says." Same trigger phrases, re-anchored. |

**Additions (negative-space clarifications, not new responsibilities):**
- Overview gains one clause: "…the skill does not emit a clearing PASS/FAIL verdict of its own." (encodes Agenda 1d in the body; see Deviation D2.)
- A `## Related Skills` section (mochiko convention; present in sibling ports `analysis-codebase`, `validation-constitution`): houses the `authoring-requirements` ordering, the `analysis-iterative` de-collision boundary, and the **verdict-ownership** boundary. No HIL `## Related Skills` existed; this is the convention floor + the boundary record an independent reader needs.

**Kept verbatim (body is mochiko-clean — no kernel/DAG/catalog/MCP/`.humaninloop` content to redesign or drop):** Core Principle (product-vs-technical table), Question Format, Gap Categories taxonomy, Severity Classification, Output Format (`## Gaps Found` by severity), Review Process (7 steps), Quality Checklist (reviewer self-check), Common Mistakes (anti-rationalization).

---

## B. Convention-wiring pass (all five — always runs)

| # | Convention | Action this primitive | Status |
|---|-----------|------------------------|--------|
| 1 | **Classification tag** | model-invoked → **default** (no `disable-model-invocation`). Skill (not agent) ⇒ **no `skills:` list**. | done |
| 2 | **Router registration** | **Deferred to its own §D artifact line** (`skills/mochiko/SKILL.md` EDIT). Brief: "DO NOT edit the shared router." Recorded as a pending wiring item; `verify-output` Part-A will confirm the router carries `analysis-specifications` once that line lands. | deferred (scope) |
| 3 | **Trigger phrasing** | model-invoked + **agent-consumed** ⇒ work-context, not "when the user says." Post-draft anchor; severity-bucketed-INPUT framing in the description. | done |
| 4 | **Path rebinding** | Only one ref in the whole file: L27 `humaninloop:authoring-requirements` → `mochiko:`. No `.humaninloop/` paths, no catalog/MCP/DAG paths exist in this skill. | done |
| 5 | **Decouple persona/skill** | L19 agent name → role (#1 above). Soft phase vocab L15/L18/L74 (#2–#4). Keystone-tested the rest → no `dispatch`, no `workflow-agnostic`/independence-by-declaration meta-label, no injected modes/paths, no DAG/catalog/brain/`state-analyst`. Zero residual deny-list tokens. | done |

---

## C. Triggers — de-collided from `analysis-iterative` (Agenda 5c / §G)

- **This skill (P6) = post-draft gap-review.** Anchor: "an already-drafted specification," "a spec that already exists," "post-draft review work." Trigger phrases: `review spec` · `find gaps` · `what's missing` · `is the spec complete` (completeness) · `clarify requirements`.
- **`analysis-iterative` (P5) = pre-spec input enrichment** (sparse/vague idea, Who/Problem/Value). Explicitly routed away in BOTH the description ("For enriching a sparse or vague feature idea before any spec is drafted, use `mochiko:analysis-iterative` instead.") AND a When-NOT-to-Use bullet AND Related Skills.
- **Lifecycle marker is the de-collision lever:** a draft *exists and is being reviewed* (P6) vs *no draft yet, enriching input* (P5). Disjoint stages ⇒ disjoint triggers. The HIL "specification" overlap (P5 F3 / P6 flag-3) is resolved by anchoring P6 on "already-drafted / post-draft."

---

## D. Realized responsibility trace (every responsibility carries a final tag — no silent loss)

Source: assess `analysis-specifications` trace + reconcile §E P6. All `kept` / `kept-but-rebind`; **no `dropped`** (the body never held kernel plumbing).

| Responsibility | Final tag |
|---|---|
| Find spec gaps (missing reqs, ambiguities, edge cases, assumption gaps) | `kept` |
| Core principle: product-framed (WHAT) not implementation (HOW) questions | `kept` |
| Clarifying-question format (decision + concrete options + "why this matters") | `kept` |
| Gap-category taxonomy (user-exp / business-rules / scope / states / permissions) | `kept` |
| "What to avoid" boundary (defer technical concerns to design) | `kept` (phase wording softened — edit #4) |
| Severity classification (Critical / Important / Minor + action) | `kept` — **INPUT to the lead/referee's verdict, NOT the verdict** |
| Structured output (`## Gaps Found` by severity) | `kept` — **no clearing PASS/FAIL added** (Agenda 1d) |
| Review process (7 steps) | `kept` |
| Reviewer self-check (Quality Checklist) | `kept` |
| Reviewer anti-rationalization (Common Mistakes) | `kept` |
| Applicability bullet naming "Devil's Advocate" (L19) | `kept-but-rebind` — HARD decouple → role (edit #1) |
| Cross-skill pointer `humaninloop:authoring-requirements` (L27) | `kept-but-rebind` — → `mochiko:` (edit #5) |
| Workflow-phase framing ("specification/planning phase", L15/L18/L74) | `kept-but-rebind` — restate by activity (edits #2–#4) |
| Trigger phrasing "when the user says…" (L3 description) | `kept-but-rebind` — agent-consumed → work-context (edit #6) |
| Classification tag (model-invoked) | `kept-but-rebind` — confirmed model-invoked + agent-consumed (wiring #1) |

**Reconcile flags (from assess) — disposition status:**
- *confirm-pairing / independence* (Agenda 1a) — **CONFIRMED at the skill, no action needed here:** this skill is the validator side; it lands ONLY on the critic agent (`devils-advocate`, P3). It mounts no `authoring-*` skill; nothing here cross-mounts producer+grader. Independence holds by construction.
- *verdict-ownership boundary* (Agenda 1d) — **HONORED:** no clearing verdict added; see §E.
- *trigger de-collision* (Agenda 5c) — **DONE:** see §C.
- *cross-cluster consumer* (`commands/audit.md`, Agenda 7e) — **no move:** skill stays in specify; audit rebinds if/when it ports. Out of scope this run.

---

## E. Gap-finder confirmation (Agenda 1d — the critical boundary, NOT crossed)

**No clearing PASS/FAIL verdict was added.** The `validation-constitution` binary-validator shape (Output: `VALIDATION RESULT: [PASS/FAIL]`) was deliberately **NOT** transferred to this critic-side skill. Concretely:
- The `## Output Format` remains `## Gaps Found` grouped by severity (Critical / Important / Minor) + clarifying questions — a **gap report**, identical in shape to HIL.
- No "verdict," "PASS/FAIL," "ready/needs-revision/critical-gaps," or pass/fail status field was introduced anywhere in the body.
- The severity buckets are explicitly framed (Overview + Related Skills) as **INPUT** to the reviewer/lead, who **owns the clearing verdict** and drives any revision round (referee-owns-the-verdict invariant, so the producer↔critic pair cannot collude into agreeableness).

**Responsibilities this skill does NOT hold (so `verify-output` confirms the boundary, not silent loss):** the clearing verdict → lead/referee (P1); the "never bless" adversarial calibration → critic persona (P3, `devils-advocate`); the produce→critique→revise loop + round cap + human escalation → lead (P1/P4) + contract. This skill never held these; they are not drops.

---

## F. Deviations / notes for the independent verifier

- **D1 — L18 "Quality gate" → "gap-review checkpoint":** slightly beyond the literal "soften phase vocab," done to keep the skill unambiguously a gap-finder (per Agenda 1d). "Quality gate" risked reading as "this skill emits the gate verdict." The checkpoint *timing* (before downstream work) is preserved; only the verdict implication is removed. Minimal and verdict-safe.
- **D2 — Overview gains one boundary clause + a `## Related Skills` section:** additions, not new responsibilities. They encode the verdict-ownership boundary (Agenda 1d) and the P5 de-collision in the artifact itself so an independent reader can verify the gap-finder posture from the file alone. `## Related Skills` matches the sibling-port convention (`analysis-codebase`, `validation-constitution`).
- **No `redesign`, no `drop`, no `split/promote`:** body was mochiko-clean (assess Check 1: no content-coupling); the pair already exists (assess Check 3); minimalism governor ⇒ `port-with-edits × standalone`.
- **Within-cluster forward refs** (`mochiko:authoring-requirements` P7, `mochiko:analysis-iterative` P5) both resolve by end of Phase 3 (both are `CREATE` in §D). Not dangling at verify time.
- **Router (wiring #2)** intentionally not edited here (brief constraint + separate §D artifact line). Flag for the verifier: confirm `analysis-specifications` registration lands with the router EDIT, not against this file.

---

**Transform version:** v1 · **Governed by:** `loop-discipline` · **Disposition applied:** port-with-edits × standalone · **Next:** Phase 4 `verify-output` (independent agent) — REGISTRY flip `[ ]`→`[x]` on finalize; this trace is the migration record.
