# TRANSFORM — `validation-feasibility` (NEW skill)

**Run:** `/mochiko:transform-cluster` (plan-core) · **Phase 3 (transform)** · **Date:** 2026-06-30
**Producer:** `mochiko:transform-producer` · **Skill applied:** `mochiko:transform-recipes`
**Role:** author the artifact + run the convention-wiring pass. **NOT** decide (assess/reconcile own that), **NOT** grade (`verify-output`, a different agent, owns that).
**Disposition (finalized, reconcile §JOB 2b / Finalized dispositions):** **NEW skill × standalone** — `redesign`-style synthesis (no single HIL skill exists; the feasibility procedure lived in command prose + the architect persona).

> Because this primitive is **new**, this "trace" is a **homing record**: it shows which existing HIL
> responsibilities land here (with their reconcile-assigned tags), so nothing is **invented** (every
> responsibility cites a HIL source) and nothing is **double-homed** (each landing is unique; the *not-mine*
> table records what deliberately stays elsewhere).

---

## TRANSFORM (transform-recipes output block)

```
TRANSFORM: validation-feasibility (NEW)
Applied:   NEW-skill (redesign-style synthesis) × standalone + wiring-pass
Artifacts: plugins/mochiko/skills/validation-feasibility/SKILL.md            (created)
           plugins/mochiko/skills/validation-feasibility/references/FEASIBILITY-LENS.md (created)
New partners: none authored here. Mounted by the re-broadened principal-architect (P3, later wave);
              emits into the feasibility-report (P13, later wave). Both noted, neither edited.
Wiring:    classification=model-invoked (no disable-model-invocation; work-context idiom)
           router=NOT edited (entry noted below — shared file; out of this primitive's scope)
           triggers=work-context graded RFC-2119 (review feasibility / cross-artifact contradictions /
                    constraint-decision conflicts / NFR-constraint impossibilities / buildability / feasibility verdict)
           rebinds=workspace-as-state (report destination = the feature feasibility-report, lead-supplied;
                    no .humaninloop/ or specs/{feature-id}/ HIL paths carried in)
           single-source=loop-discipline referenced for loop/bound/human-gate (NOT restated);
                    feasibility-report template owns the report markdown shape (NOT reproduced here)
Trace (realized): see homing trace below — every homed responsibility tagged; 0 invented; 0 double-homed
```

---

## Homing trace — HIL responsibilities this skill now holds

| HIL responsibility | HIL source | Reconcile tag | Where it lands here |
|--------------------|-----------|---------------|---------------------|
| **Cross-artifact feasibility intersection review procedure** (the 4 conflict types: constraint-decision / NFR-constraint / requirement-constraint / decision-decision) | `commands/plan.md` §2.5 Focus Areas; `agents/principal-architect.md` §Feasibility Review | **`folded-into-skill`** (P3 #7, was `flag-for-reconcile` → resolved RQ1 (i)) | SKILL.md *What you hunt* classes 1–4; LENS classes 1–4 |
| **Feasibility verdict taxonomy** `feasible / needs-revision / infeasible` — incl. the distinct `infeasible` = business-level escalation | `agents/principal-architect.md:143–146`; `architect-report-template.md:22,26` | **`kept`** (P3 #7b — all 3 states preserved) | SKILL.md Step 5 verdict table + "Preserve `infeasible`"; LENS Verdict criteria |
| **Per-issue gate fuel** {description / evidence / impact / suggested_resolution} that feeds the human gate | `commands/plan.md` Feasibility Rejection Loop (`AskUserQuestion` fields, :626) | **`kept`** (the 4-field fuel travels with #7b) | SKILL.md Step 3 table; LENS "four-field gate fuel" |
| **Feasibility ≠ completeness division of labour** (the in/out-of-scope list) | `agents/principal-architect.md:137–141`; `commands/plan.md` §2.5 Out of Scope | **`kept`** + decouple (P3 #7c) | SKILL.md *The boundary* table; *When NOT to Use* |
| **Design-phase feasibility checks** — NFR-design feasibility, constraint/infrastructure-design buildability | `commands/plan.md:579–580`; P9 `PHASE-CHECKLISTS.md` P3 cross-artifact | **`dedupe` → here** (P9 R3, was `flag` RQ1; feasibility checks deduped OUT of P9 to this single source) | SKILL.md classes 5–6; LENS classes 5–6 |
| **Adversarial-critique FORM** (judgment "can these be built together?", not a mechanical checklist) | ROADMAP convention 5 two-form; reconcile RQ1 axis 3 | **`folded-into-skill`** (the critique form of the two-form) | SKILL.md Overview + *What you hunt* framing; LENS "lenses, not a checklist" |
| **Don't-rubber-stamp discipline** (default not-cleared; never award `feasible` on a skim) | reconcile §0/§Independence; `loop-discipline` req-1 spirit | **`kept`** (anti-rationalization, skill-local) | SKILL.md "Never default to `feasible`", Red Flags, Common Rationalizations |

**Invention check:** every row cites a concrete HIL source (plan.md prose, the architect persona, the architect-report template, or the P9 checklist). Nothing in the skill originates from outside the homed set; the synthesis only *relocates and decouples* existing responsibilities into one kernel-free skill.

---

## Not-mine — deliberate non-homes (the anti-double-home boundary)

Each item below is a responsibility this skill **could** have absorbed but must not — it is homed elsewhere. Recording it here is what proves no double-home.

| Responsibility | Stays at | Why not here |
|----------------|----------|--------------|
| Coverage / measurability / consistency / presence / traceability checks | **P9 `validation-plan-artifacts`** (completeness sibling) | The P9↔feasibility boundary. P9 keeps these; I take only contradiction/impossibility/buildability. Mirrored in SKILL.md *The boundary*. |
| The report's markdown shape (frontmatter, headers, tables) | **P13 `feasibility-report` template** (later wave) | A skill owns the *judgments to emit*; the template owns the *file shape*. I specify verdict + 4 fields; I do not reproduce the template (mirrors P9/P15 split). |
| Loop driving, round bound, verdict-sink, "review once before completeness" sequencing, Feasibility-Rejection-Loop routing, "Next Steps" | **the plan lead** (P1, `moved-to-lead`) | Workflow-specific orchestration + verdict consumption. Referenced via `loop-discipline`, never inlined (HIL architect-report §"Next Steps" deliberately dropped from the skill). |
| Constitution quality grading | **generic `validator` + `validation-constitution`** | **G1.** Different artifact domain. Never referenced or recreated here. |
| Producing/revising the analysis & design artifacts | **the producer** (technical-analyst, P2) | Independence — I grade a different agent's output; I never author what I grade. |

**Double-home check:** none. Each homed responsibility lands in exactly one place; each not-mine responsibility is owned by exactly one other primitive. The single most-overlapping seam (P9 feasibility checks) is resolved by the explicit boundary, not by duplication.

---

## P9 boundary mirrored

No P9 transform trace exists yet (P9 is `port-with-edits × standalone`, a later edit in this same wave), so the boundary is taken from the reconcile fallback the task specified and re-asserted symmetrically here so the P9 port can mirror it back:

```
P9  (validation-plan-artifacts, mirror-checklist) = COVERAGE / MEASURABILITY / CONSISTENCY / PRESENCE
ME  (validation-feasibility,   adversarial-critique) = CONTRADICTION / IMPOSSIBILITY / BUILDABILITY
```

Brush-points resolved explicitly: an NFR that is *both* unmeasurable *and* impossible → I take the **impossibility**, P9 takes the **measurability**. A schema that is *both* inconsistent with the model *and* unable to meet an NFR → P9 takes the **consistency**, I take the **NFR-design feasibility**. (SKILL.md *The boundary* + LENS classes 2/5 "Boundary watch".)

---

## G1 confirmation (HARD)

- Operates over **plan analysis/design artifacts** only (requirements, constraints-and-decisions, NFRs, data-model, contracts). The constitution is named **only** in *When NOT to Use* as an explicit exclusion.
- Is **not** `validation-constitution`; does **not** reference or recreate constitution grading anywhere.
- Independence stated by **role** (a different agent's artifacts; never the author's own) — no constitution self-grade re-created. Honors the assess/reconcile guardrail that `validation-constitution` is never mounted on the re-broadened principal-architect.

---

## Convention-wiring pass (all 6)

1. **Classification** — model-invoked (no `disable-model-invocation`); description in the **work-context** idiom (agent-consumed validator skill), not the user-utterance idiom. Mounted by an agent via its `skills:` list (the re-broadened principal-architect, P3 — a later wave).
2. **Router registration** — **NOT applied** (shared `skills/mochiko/SKILL.md` is out of this primitive's scope; per the task, note-only). Entry needed: under a **plan cluster** section (model-invoked) —
   `| `validation-feasibility` | adversarially grading a producer's plan analysis/design artifacts for cross-artifact feasibility — contradictions / impossibilities / buildability → 3-state feasible/needs-revision/infeasible (the critique half of the cross-artifact review pair; run by an independent reviewer, never the author) |`
   plus, when P3 re-broadens, add `validation-feasibility` to the `principal-architect` agent-row skill list. (A primitive absent from the router fails `verify-output`'s discoverability item — flagged for the router-owner.)
3. **Trigger phrasing** — graded RFC-2119 work-context phrases in `description`: *review feasibility*, *cross-artifact contradictions*, *constraint-decision conflicts*, *NFR-constraint impossibilities*, *buildability*, *feasibility verdict*. Describes the transformation work, not "when the user says…".
4. **Path rebinding** — workspace-as-state. No `.humaninloop/` paths and no HIL `specs/{feature-id}/.workflow/...` paths carried in; the report destination is the lead-supplied feature `feasibility-report`. Artifact **types** (requirements / constraints-and-decisions / NFRs / data-model / contracts) named generically, as HIL's feasibility section already did.
5. **Decouple persona/skill** — no sibling-**agent** names (`technical-analyst` / `principal-architect` / `devils-advocate` / `validator` absent; independence stated by *role* — "a different agent (the producer)"); no "dispatch"; no injected workflow modes/phases ("Phase 1/2" not used as an organizing axis — organized by contradiction-class and artifact-set instead); no "workflow-agnostic" meta-label. The completeness sibling is referenced by its **skill** namespace (`mochiko:validation-plan-artifacts`) — allowed (skill, not agent). HIL's one coupling token (`"(that is the reviewer's job)"`) is rephrased to role language ("that is the completeness sibling's").
6. **Single-source / de-duplicate** — references `loop-discipline` for loop ownership, the round bound, the human gate, and gap routing (never restates the four rules or the routing taxonomy); references the `feasibility-report` template for the report's markdown shape (never reproduces it); references P9 by role+namespace rather than restating its checklist.

---

## Realized disposition + open items

- **Disposition realized:** NEW skill × standalone, redesign-style synthesis, wiring-pass applied. ✓
- **Hand-off to `verify-output`** (a **different** agent): grade SKILL.md + LENS against the five conventions, sound-loop placement (verdict-as-input, lead owns loop), kernel-free, and this homing trace for silent loss / double-home. This producer does **not** grade its own output.
- **Cross-wave dependencies noted (not edited):** (a) `principal-architect` (P3) must add `validation-feasibility` to its `skills:` list when re-broadened; (b) the `feasibility-report` (P13) is the emission target; (c) the router (shared) needs the entry above. All three are other primitives'/owners' edits.

**Trace version:** v1 · **Governed by:** `loop-discipline` · **Role:** transform only — authored + wired; not graded.
