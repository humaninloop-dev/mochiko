# TRANSFORM — validation-plan-artifacts (P9)

**Run:** `/mochiko:transform-cluster` (plan-core) · **Phase 3 (transform)** · **Date:** 2026-06-30
**Producer:** `mochiko:transform-producer` · **Skill:** `mochiko:transform-recipes`
**Role:** apply + wire only — **did NOT grade** this output (independence; `verify-output` is run by a different agent).
**Inputs consumed:** finalized disposition (reconcile §RQ1/§RQ3 + P9/P14 verdict rows), `assess-validation-plan-artifacts.md`,
`assess-cross-artifact-checklist.md` (P14, absorbed), HIL source (SKILL.md + PHASE-CHECKLISTS.md + ISSUE-TEMPLATES.md + check-artifacts.py),
HIL `templates/cross-artifact-checklist.md` (P14, folded), sibling shape `plugins/mochiko/skills/validation-constitution/`.

---

## Applied disposition

**`port-with-edits × standalone`** — the **COMPLETENESS** reviewer's **mirror-checklist** skill (convention-5 objective-acceptance form),
wrapped in anti-rationalization discipline. **Absorbs P14** (`merge-into-sibling` target). **Dedupes feasibility OUT** to the new
`validation-feasibility` (reconcile RQ1 option (i): two distinct validators).

Form preserved: enumerable named checks + per-check severity (Critical/Important/Minor) + a verdict **mechanically derived from issue
counts** (3-state `ready / needs-revision / critical-gaps`) + the deterministic `check-artifacts.py` (stdlib, kept as a Tier-1 pre-assert).

## Artifacts (created)

- `plugins/mochiko/skills/validation-plan-artifacts/SKILL.md` — mirror-checklist body, reorganized by **artifact-type**, scope boundary surfaced, incremental mode parameterized.
- `plugins/mochiko/skills/validation-plan-artifacts/references/ARTIFACT-CHECKLISTS.md` — ex-`PHASE-CHECKLISTS.md`, reorganized by artifact-type (phase axis dropped); feasibility checks removed; **P14 content folded in** as the single cross-artifact-consistency source; the explicit **boundary table** for `validation-feasibility`.
- `plugins/mochiko/skills/validation-plan-artifacts/references/ISSUE-TEMPLATES.md` — kept; phase field/column dropped; single-source pointer to `advocate-report-template` for the deliverable.
- `plugins/mochiko/skills/validation-plan-artifacts/scripts/check-artifacts.py` — kept (stdlib, kernel-free, runs; exit 0/1); softened the OpenAPI cross-ref (no hardcoded sibling filename).

## New partners

**None created here.** `validation-feasibility` (the feasibility reviewer's driver, reconcile Job 2b) is a **sibling primitive built in
this same wave by its own producer** — this transform only **hands off the boundary** (see below) so that producer can mirror it. P9 ports
`standalone`; it does not spin out the partner.

---

## THE P9 ↔ validation-feasibility CHECK BOUNDARY (for the feasibility producer to mirror)

P9 keeps the **left** column; `validation-feasibility` owns the **right**. Mirrored verbatim from
`ARTIFACT-CHECKLISTS.md#scope-boundary--handoff-to-validation-feasibility`.

| Family | P9 keeps (completeness lens) | `validation-feasibility` owns (buildability lens) |
|--------|------------------------------|---------------------------------------------------|
| Coverage / traceability | FR→TR coverage, orphan TRs, NFR source tracing, entity coverage, endpoint coverage, FR→TR→entity→endpoint traceability, IP / IP-NFR coverage | — |
| Measurability | testable TR criteria; NFR measurable target present; measurement method defined | — |
| Presence | sourced constraints; ≥2 alternatives; rationale (WHY); trade-offs; DS annotations + sensitivity details present; integration-boundary x-integration present; failure modes documented; auth/examples present | — |
| Consistency (does design honor the decisions?) | requirements-decisions alignment; decisions-model consistency; model-contract / schema-model consistency; sensitivity-contract alignment; integration-contract alignment; constraint-decision cross-refs; constitution compliance | — |
| **Contradiction** (do artifacts conflict?) | — | **TR ↔ constraint contradictions; NFR ↔ constraint conflicts; NFR ↔ NFR impossibilities** |
| **Buildability** (can it be built / met?) | — | **NFR-design feasibility (can the design meet the NFR targets?); constraint-design buildability (can the design satisfy the constraints?); integration failure modes realistic vs aspirational** |

**The four checks removed from P9's checklist rows** (deduped to `validation-feasibility`): `TR-constraint contradictions` (was P1
constraints), `NFR-constraint conflicts` (was P1 NFRs), `NFR-design feasibility` (was P3 cross-artifact), `Constraint-design alignment` →
reframed as constraint-design buildability (was P3 cross-artifact). Plus the design key-question "are integration failure modes realistic or
aspirational?" and the analysis key-questions on requirement↔constraint and NFR↔NFR conflict.

**The one-line seam:** *"is it here, traceable, measurable, and does it honor the decisions?"* → **P9**. *"can these pieces be built together
without contradiction or overreach?"* → **`validation-feasibility`**. The boundary is drawn so that consistency-contradictions (design vs a
*decided* approach) stay in P9; contradictions *between requirements/constraints* and all buildability go to feasibility.

---

## P14 FOLD — confirmed (RQ3: `merge-into-sibling` → P9; standalone file dropped, content survives here)

P14 `cross-artifact-checklist.md` was an orphan duplicate of P9's already-bundled cross-artifact content. Its content now lives **once**, in
P9's [Cross-Artifact Consistency](references/ARTIFACT-CHECKLISTS.md) section:

| P14 content | Landing in P9 | Tag |
|-------------|---------------|-----|
| 4 check groups (Entity-Name / Requirement-Traceability / Decision-Consistency / Naming-Conventions) | ARTIFACT-CHECKLISTS.md → "Named consistency groups" | `folded-into-skill` |
| How-to-Use incremental method | already in SKILL.md → Incremental Review Mode | `dedupe` |
| Time Budget (1–2 min/artifact) | SKILL.md incremental mode + ARTIFACT-CHECKLISTS | `dedupe` |
| When-to-Escalate (2+ issues → full re-read) | SKILL.md → "When to escalate to a full re-review" | `dedupe` |
| Issue Classification (Critical/Important/Minor) | ISSUE-TEMPLATES.md + ARTIFACT-CHECKLISTS consistency table | `dedupe` |
| Artifact-name refs (`data-model.md`, `constraints-and-decisions.md`, `userId`/`user_id`) | rebound to mochiko plan artifact names under `.mochiko/specs/<feature>/` | `kept-but-rebind` |
| Tasks-cluster "reuse" (REGISTRY dual-file) | no orphan — the tasks validator self-contains its own checklist (deferred run) | confirmed, not owed |

**The standalone P14 file is dropped** (reconcile §E). Content is single-sourced in P9. No silent loss.

---

## INCREMENTAL-MODE SPLIT — confirmed

- **Procedure → `folded-into-skill`:** the full-review-of-{new}-set + consistency-check-of-{prior}-set + escalate-on-2+-inconsistencies
  discipline stays in P9, **parameterized by caller-supplied {new}/{prior} artifact sets** (SKILL.md → Incremental Review Mode). No HIL phase
  number in the body.
- **Mode SELECTION → `moved-to-lead`:** *which* artifacts are new (full) vs prior (consistency-check) is the lead's call. The HIL "Phase
  Application P1/P2 table" and "use incremental for Phase P2" are **gone** from the skill; the SKILL.md states explicitly that the caller
  supplies the sets and that sequencing is the lead's.
- **Phase axis decoupled:** the A0/P1/P2/P3 organizing axis is replaced by **artifact-type** (analysis artifacts / design artifacts /
  cross-artifact). Grep confirms zero phase-axis tokens in the ported skill.

---

## Convention-wiring pass (all 6)

1. **Classification — model-invoked.** No `disable-model-invocation`; agent-consumed validator skill (default model-invoked). ✓
2. **Router registration — NOTED, not applied** (per instruction: do not edit the shared router/plugin.json). **Entry needed** in
   `plugins/mochiko/skills/mochiko/SKILL.md`, plan cluster table:
   > `validation-plan-artifacts` | independently grading a producer's plan artifacts for **completeness** — coverage / measurability /
   > presence / cross-artifact consistency over the analysis + design sets → severity-classified gaps + 3-state verdict (run by an independent
   > validator, never the author). Paired with `validation-feasibility` (the buildability half).
   Also note: `validation-feasibility` will need its own adjacent router entry (its producer adds it), and the plan agent roster row for
   `devils-advocate` broadens to mount this skill.
3. **Triggers — work-context, de-collided.** Rewrote the description from the HIL user-utterance idiom ("when the user says 'review
   research'…") to the **work-context** idiom (grade a producer's plan artifacts; FR→TR coverage; measurability; cross-artifact consistency;
   completeness-review step; re-review after FAIL-loop). **De-collided** off the generic phrases `"plan quality"` / `"phase review"` /
   `"artifact review"` (dropped) and off `validation-feasibility`'s territory (the description explicitly disclaims feasibility/buildability/
   contradiction). Same treatment `validation-constitution` got.
4. **Path rebinding — recorded.** `specs/{feature-id}/…` → `.mochiko/specs/<feature>/…` (4 occurrences across SKILL + checklists + script
   examples). `validate-openapi.py` hardcoded sibling path → **softened** to a capability reference ("the OpenAPI validator bundled with the
   api-contracts skill") in both the checklist prose and the script's skip-reason string. No `.humaninloop/` paths existed to swap. The
   bundled `scripts/check-artifacts.py` path stays relative to the skill.
5. **Decouple — clean.** No agent names (grep zero). "flag for supervisor" (HIL SKILL.md:106) **generalized** → "report it as an issue; the
   lead routes it." The "When NOT to Use" sibling refs are **`kept-but-rebind`**: `humaninloop:` → `mochiko:` (skill namespaces, not agent
   names) — and a new `mochiko:validation-feasibility` row added. No "dispatch," no injected workflow modes/paths/phases, no
   "workflow-agnostic" meta-label. Independence is stated by **role** (the completeness half of the producer↔validator pair), not by an agent
   name.
6. **Single-source / de-duplicate.** Anti-rationalization: the **generic** doctrine is **referenced** to `loop-discipline` (Overview + Related),
   the **review-specific** red flags/rationalizations stay local (`kept`). The deliverable report **references** `mochiko:advocate-report-template`
   rather than restating it (ISSUE-TEMPLATES keeps only the reviewer's working format + the plan-only consistency matrix). The 3-state verdict
   vocabulary is the same the lead already consumes from the advocate report — emitted here, not re-specified.

---

## Realized responsibility trace (every responsibility flipped to its final tag — no silent loss)

### From P9's assessment (R1–R22)

| # | Responsibility | Realized tag |
|---|----------------|--------------|
| R1 | Analysis completeness checks (FR→TR coverage, orphan TRs, testable criteria, deps, priority, RFC2119; sourced constraints, ≥2 alternatives, rationale, cross-refs, constitution alignment, IP + IP-NFR coverage, brownfield, trade-offs; NFR measurability, method, source tracing, category coverage) | **kept** — reorganized under "Analysis Artifacts"; phase label dropped. **Minus** TR-constraint contradictions + NFR-constraint conflicts → **`moved-to-sibling-skill`** (`validation-feasibility`) |
| R2 | Design completeness checks (entity/attr/relationship coverage, PII + sensitivity, compliance, retention, encryption, validation, state machines, standard fields, traceability; endpoint coverage, schema completeness, error handling, schema-model consistency, integration-boundary + failure-mode presence, auth, examples, naming; integration-guide flow/auth/error/external) | **kept** — reorganized under "Design Artifacts"; phase label dropped |
| R3 | Cross-artifact / consistency checks | **kept** (requirements-decisions / decisions-model / model-contract / sensitivity-contract / integration-contract alignment, traceability, constitution compliance, infra completeness). **Minus** Constraint-design alignment + NFR-design feasibility → **`moved-to-sibling-skill`** (`validation-feasibility`) — RQ1 resolved |
| R4 | Codebase-discovery review (Phase A0) | **`moved-to-other-cluster`** (brownfield/discovery; out of plan-core) — noted by reference in ARTIFACT-CHECKLISTS. Not dropped |
| R5 | Severity calibration (Critical/Important/Minor) | **kept** |
| R6 | Verdict emission (`ready`/`needs-revision`/`critical-gaps`) | **kept** — mechanically derived from counts (loop fuel; same 3-state vocab the lead consumes) |
| R7 | Review process (gather → check → cross-ref → report) | **kept** — + a Step-2 deterministic pre-assert made explicit |
| R8 | Issue documentation + report formats | **kept** (ISSUE-TEMPLATES.md) — phase field dropped; deliverable references `advocate-report-template` |
| R9 | Incremental-review **PROCEDURE** | **`folded-into-skill`** — parameterized by caller-supplied {new}/{prior} sets |
| R10 | Incremental-review **MODE-SELECTION** | **`moved-to-lead`** — lead picks the sets; phase table removed from the skill |
| R11 | Anti-rationalization discipline | **kept** (review-specific red flags/rationalizations) + **`dedupe`** (generic doctrine referenced to `loop-discipline`, not restated) |
| R12 | `check-artifacts.py` automated script | **kept** — Tier-1 pre-assert; stdlib-only; runs (exit 0/1); kernel-free confirmed |
| R13 | Script + checklist path conventions | **`kept-but-rebind`** — `specs/{feature}/` → `.mochiko/specs/<feature>/`; bundled script path stays relative |
| R14 | `validate-openapi.py` cross-skill reference | **`kept-but-rebind`** — **softened** to a capability reference (api-contracts OpenAPI validator); no hardcoded sibling path |
| R15 | Sibling skill cross-refs ("When NOT to Use") | **`kept-but-rebind`** — `humaninloop:`→`mochiko:`; **+ added** `mochiko:validation-feasibility` (the new boundary sibling) |
| R16 | Phase-identifier organizing axis (A0/P1/P2/P3) | **`moved-to-lead`** (phase sequencing) **+ body decouple** (skill reorganized by artifact-type; labels dropped — grep zero) |
| R17 | "flag for supervisor" routing | **`moved-to-lead`** — generalized to "report it; the lead routes" |
| R18 | Classification + trigger description | **`kept-but-rebind`** — stays model-invoked; triggers rephrased to work-context + de-collided; router entry noted (not applied) |
| R19 | Upstream handoff (artifacts already exist to review) | **`moved-to-lead`** — the analyst → {artifacts} → reviewer edge is the lead's |
| R20 | Verdict consumer + FAIL-loop + done-condition + human gate | **`moved-to-lead`** — P9 only *emits* the verdict; consuming/looping/gating is the plan lead's (rehome map, reconcile Job 2) |
| R21 | Validator-side independence role | **kept** (structural) — completeness half of the pair; **port hazard recorded:** never co-mount on the producer (`technical-analyst`); `devils-advocate` mounts review skills only |
| R22 | Cross-artifact CONSISTENCY checklist content (the P14-duplicated slice) | **kept** as the single source — see P14 fold above |

### From P14's assessment (absorbed; RQ3)

| # | Responsibility | Realized tag |
|---|----------------|--------------|
| P14-R1 | 4 cross-artifact check groups | **`folded-into-skill`** → ARTIFACT-CHECKLISTS "Named consistency groups" |
| P14-R2 | Incremental-validation method | **`dedupe`** → SKILL.md Incremental Review Mode |
| P14-R3 | Time Budget | **`dedupe`** → SKILL.md / ARTIFACT-CHECKLISTS |
| P14-R4 | When-to-Escalate | **`dedupe`** → SKILL.md |
| P14-R5 | Issue Classification | **`dedupe`** → ISSUE-TEMPLATES + ARTIFACT-CHECKLISTS |
| P14-R6 | Artifact-name / ID refs | **`kept-but-rebind`** → mochiko plan artifact names under `.mochiko/specs/<feature>/` |
| P14-R7 | Nominal tasks-cluster reuse | confirmed **no orphan** (tasks validator self-contains its own; tasks deferred this run) |
| P14-R8 | Consistency-check input surface | **`folded-into-skill`** → lands in P9's consistency section |
| P14-R9 | Convention-wiring floor | folded — **no separate router entry** for P14 (it is part of P9 now) |
| P14-file | The standalone template file | **`dropped`** — *reason:* near-total duplicate orphan; content survives single-sourced in P9 (reconcile §RQ3/§E, human-gated) |

**Trace complete.** Every responsibility carries a realized tag. **Zero silent drops** — R4 is `moved-to-other-cluster` with reason; the
only `dropped` is the standalone P14 file, whose content survives in P9 (an accepted, human-gated drop). The feasibility slice is
`moved-to-sibling-skill` (`validation-feasibility`), explicitly handed off via the boundary table above.

---

## Sequencing + record-only edges (for the rehome map / later waves)

- **P9-before-P4 sequencing:** P9 now lives at `plugins/mochiko/skills/validation-plan-artifacts/`. The **`devils-advocate` (P4)** re-mount —
  adding this skill to its `skills:` list, **completeness-only** — is a **later wave**; it may be applied now that P9 has landed (a live mount
  of an unported skill would have dangled). **Not edited here.**
- **Router + plugin.json:** **NOT edited** (per instruction). Entry wording recorded under wiring step 2.
- **`validation-feasibility`:** built by its own producer this wave; this transform supplies the boundary it must mirror (above).

## Output block

```
TRANSFORM: validation-plan-artifacts (P9)
Applied:   port-with-edits × standalone  +  absorb P14 (merge-into-sibling target)  +  dedupe feasibility → validation-feasibility  +  wiring-pass (all 6)
Artifacts: plugins/mochiko/skills/validation-plan-artifacts/{SKILL.md, references/ARTIFACT-CHECKLISTS.md, references/ISSUE-TEMPLATES.md, scripts/check-artifacts.py}
New partners: none built here (validation-feasibility is a sibling primitive; boundary handed off, not spun out)
Wiring:    classification=model-invoked · router=NOTED-not-applied (entry recorded) · triggers=work-context, de-collided (dropped "plan quality"/"phase review"/"artifact review"; disclaims feasibility) · rebinds=[specs/{feature-id}→.mochiko/specs/<feature>; validate-openapi.py→softened capability ref; humaninloop:→mochiko:; +validation-feasibility] · decouple=clean (no agent names; "flag for supervisor"→generalized; phase axis→artifact-type) · single-source=loop-discipline (anti-rationalization) + advocate-report-template (deliverable) referenced, not restated
Form:      MIRROR-CHECKLIST — named checks + per-check severity + verdict mechanically from counts (3-state) + Tier-1 check-artifacts.py (stdlib, runs)
P14:       FOLDED (content single-sourced in P9; standalone file dropped, content survives)
Incremental: procedure folded-into-skill ({new}/{prior} caller-supplied); selection moved-to-lead; phase axis decoupled
Boundary:  P9 = coverage/measurability/presence/consistency · validation-feasibility = contradiction/buildability (4 checks deduped out; table in ARTIFACT-CHECKLISTS.md)
Trace:     22 P9 responsibilities + 10 P14 items, all flipped to realized tags; 0 silent drops; feasibility slice moved-to-sibling-skill
Independence: did NOT grade own output — verify-output runs on a different agent
```

---

**Transform version:** v1 · **Governed by:** `loop-discipline` · **Role:** apply + wire only — no grade applied.
Handoff: artifact + this realized trace → `verify-output` (independent validator, a different agent).
