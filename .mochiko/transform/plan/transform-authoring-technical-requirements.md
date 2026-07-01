# Transform (realized) — `authoring-technical-requirements` (P5, plan cluster)

**Producer:** `mochiko:transform-producer` · **Skill:** `mochiko:transform-recipes` · **Date:** 2026-06-30
**Inputs:** `assess-authoring-technical-requirements.md` (P5 trace R1–R17, flags F1–F5) · `reconcile.md` (§RQ6 seam boundaries; §relational-verdicts P5 rows; §Job-2D producer-content homes) · HIL source dir `human-in-loop/plugins/humaninloop/skills/authoring-technical-requirements/` (SKILL.md + 2 references; **no script present**) · sibling shape `plugins/mochiko/skills/authoring-requirements/`
**Disposition (finalized, reconcile-locked):** **`port-with-edits × standalone`** — the producer's analysis-phase authoring skill. DS-XXX → P7, INT-XXX → P8 (`moved-to-sibling-skill`); P6 boundary + `dedupe`; thin declarations + references.
**Role:** APPLY + WIRE only. **NOT graded here** (independence: `verify-output` via a different agent — `mochiko:validator`).

---

```
TRANSFORM: authoring-technical-requirements
Applied:   port-with-edits × standalone + wiring-pass (all 6)
Artifacts: plugins/mochiko/skills/authoring-technical-requirements/SKILL.md                          (port-with-edits)
           plugins/mochiko/skills/authoring-technical-requirements/references/ARTIFACT-TEMPLATES.md  (port-with-edits; §4/§5/§6 moved out)
           plugins/mochiko/skills/authoring-technical-requirements/references/TRACEABILITY-PATTERNS.md (port-with-edits; design-side → seam pointer)
New partners: none (standalone). The independent validator already exists at cluster level —
           completeness reviewer = devils-advocate + validation-plan-artifacts (P9); feasibility
           reviewer = principal-architect + validation-feasibility — both DIFFERENT agents than the
           producer (technical-analyst) that mounts this skill. Producer↔validator independence holds.
Wiring:    classification=model-invoked (default; no disable-model-invocation field)
           router=NOT edited here (shared router/plugin.json out of scope by instruction); Plan-cluster
                  entry NOTED below for the cluster's single router-edit task
           triggers=re-voiced "when the user says…" → WORK-CONTEXT; graded MUST/SHOULD kept; ID anchors
                    narrowed to P5-owned TR-/C-/NFR-/IP-; de-collided vs P6 (artifact+traceability, NOT
                    the decision-evaluation technique); DS-/INT- demoted to body declarations (canonical
                    triggers now live on P7/P8)
           rebinds=[ humaninloop:authoring-requirements → mochiko:authoring-requirements ;
                     humaninloop:patterns-api-contracts → mochiko:patterns-api-contracts ;
                     humaninloop:patterns-entity-modeling → mochiko:patterns-entity-modeling ;
                     (new) → mochiko:patterns-technical-decisions ; (new) → mochiko:validation-plan-artifacts ]
           decouple=injected workflow-phase coupling scrubbed (Phase 1/Phase 2, /humaninloop:plan,
                    "[Phase 2]" tags, "quality gate before architectural design begins",
                    "Generated during plan workflow" stamps, "six plan artifacts"); zero agent-names existed
           single-source=DS taxonomy → P7; x-integration → P8; decision technique → P6; cross-artifact
                    grading → P9 — referenced, not restated
Trace (realized): every responsibility tagged below; one lead-accepted drop (R12); no silent loss.
```

---

## Body recipe applied — `port-with-edits`

The body was already kernel-clean (no MCP/DAG/catalog/`.humaninloop/` paths, impersonal voice, zero deny-list agent-names — confirmed by grep across the whole ported dir). All edits are localized; section structure, the field tables, the WHAT-not-HOW discipline, the Quality Checklist, the anti-rationalization sections, and the voice are preserved. The edits (HIL source line → port):

| # | Location (HIL source) | Before | After | Why |
|---|---|---|---|---|
| 1 | frontmatter `description` (L3) | "…when the user says 'write technical requirements', 'classify data sensitivity', 'map integrations'…" + `INT-`/`DS-`/`system integration`/`data classification` triggers | work-context idiom; graded MUST/SHOULD kept; ID anchors narrowed to **TR-/C-/NFR-/IP-**; explicit P6 de-collision ("owns the artifact structure + traceability — NOT the decision-evaluation technique") | **Trigger re-voice** (convention 3) + **de-collide vs P6**. `DS-`/`INT-` dropped from the trigger surface — canonical triggers now live on P7/P8 (the move); P5 still declares them in the body. |
| 2 | "When to Use" (L19–21) | "Cataloguing external system integrations (INT-XXX)" · "Classifying data elements by sensitivity level (DS-XXX)" · "Quality gate before architectural design begins" | INT/DS reframed to **thin analysis declarations** with refs to P8/P7; added **IP-XXX** bullet (P5 owns it); **dropped** the "quality gate before architectural design begins" bullet | **DS/INT move** (thin declaration) + **R12 phase-coupling drop** + IP completeness. |
| 3 | "When NOT to Use" (L25) | "Writing business requirements -- Use `humaninloop:authoring-requirements`" | `mochiko:authoring-requirements` | **Namespace rebind** (sibling is ported). |
| 4 | "When NOT to Use" (new bullet) | — | "**Evaluating alternatives / making a technology decision** -- this skill owns the constraints-and-decisions.md artifact structure (D-XXX field schema) + C↔D traceability; the decision *technique* lives in `mochiko:patterns-technical-decisions`" | **P6 boundary** (convention 6 single-source + de-collide). |
| 5 | "The Three Analysis Artifacts" intro (L32) | "These are produced during **Phase 1 (Analysis) of `/humaninloop:plan`**: requirements.md…" | "Each artifact uses a distinct ID prefix and traces to business sources: requirements.md (TR-XXX), constraints-and-decisions.md (C/D/IP), nfrs.md (NFR-XXX)." | **Scrub injected workflow-phase + command coupling** (R12). |
| 6 | the `> Note:` block (L34) | "Integration maps (INT-XXX) are now embedded… **during Phase 2 (Design)**… DS-XXX… **during Phase 2 (Design)**. See `humaninloop:patterns-api-contracts` and `humaninloop:patterns-entity-modeling`." | rewritten as the **analysis-declaration-vs-downstream-authoring** seam: declare INT-XXX/DS-XXX here; author the `x-integration` boundary in `mochiko:patterns-api-contracts` and the per-attribute sensitivity taxonomy + canonical `data-model.md` in `mochiko:patterns-entity-modeling` | **DS/INT move** + **repair the dangling forward reference** (point at the now-canonical mochiko homes) + scrub Phase-2 tags. |
| 7 | Section 2 Decisions (after L82) | — | added `>` pointer: technique (evaluation matrix, scoring, ADR depth, ≥2-alternatives) → `mochiko:patterns-technical-decisions`; this skill owns the D-XXX **field schema** + C↔D traceability; "Do not restate the evaluation method" | **P6 boundary** (resolves F2; `dedupe` the technique). |
| 8 | "### 4. [Phase 2] System Integrations" (L113–119) | "[Phase 2] … embedded in contracts/api.yaml … during Phase 2 (Design). See `humaninloop:patterns-api-contracts`." + full field list | reframed to a **thin INT-XXX analysis declaration** ("flag *which* external systems + *how critical*"); canonical-home callout → `mochiko:patterns-api-contracts` for the per-endpoint `x-integration` format | **INT-XXX `moved-to-sibling-skill` → P8** + scrub "[Phase 2]". |
| 9 | "### 5. [Phase 2] Data Sensitivity" (L121–125) | "[Phase 2] … embedded in data-model.md … during Phase 2 (Design). See `humaninloop:patterns-entity-modeling`." + full field list | reframed to a **thin DS-XXX analysis declaration** ("flag *which* data is sensitive"); canonical-home callout → `mochiko:patterns-entity-modeling` for the 4-level taxonomy + canonical `data-model.md` | **DS-XXX `moved-to-sibling-skill` → P7** + scrub "[Phase 2]". |
| 10 | "Common Mistakes" (L178, L184) | "Missing Failure Modes" / "Unclassified Data" (… "**before design begins**" phase framing) | "Integrations Declared but Never Bounded" / "Unclassified Sensitive Data" — kept the analysis-level discipline, routed the heavy authoring to P8/P7, scrubbed "before design begins" | **DS/INT move** consistency + minor phase scrub. |
| 11 | Traceability Rules (L138) | mandatory links list | added the **C/NFR → IP** link explicitly (P5 owns IP traceability) | completeness (P5-owned IP). |

**References — `port-with-edits` (trimmed per the move), not `keep-verbatim`:**

- **`ARTIFACT-TEMPLATES.md`** (798 → 406 lines): kept **§1 requirements.md (TR)**, **§2 constraints-and-decisions.md (C/D/IP)**, **§3 nfrs.md (NFR)**, and the **ID Numbering Rules**. **Removed §4 Data Model (→ P7), §5 API Contracts (→ P8), §6 Integration Guide (→ P8)** — those design-artifact templates are the moved content; restating them here would be the dedupe defect. Header reframed "six plan artifacts" → "the three analysis artifacts this skill authors"; scrubbed every "Generated during plan workflow" stamp; added a seam-pointer note (design templates live with P7/P8) and a P6-technique pointer in §2.
- **`TRACEABILITY-PATTERNS.md`** (334 → 262 lines): kept the **analysis-side** web (TR→FR, TR→C, TR→NFR, C→D, NFR→source, **C/NFR→IP**), the analysis **completeness checks** (forward/backward/decision/NFR-measurability/infra-coverage), and the analysis-internal **consistency rules** (ID-resolve, bidirectional, no-contradictory-constraints) as **producer self-checks**. Dropped the "Phase 1/Phase 2" labels. **Replaced** the design-side cross-refs (entity→FR, attribute→sensitivity, schema→entity, endpoint→integration), the design completeness checks (sensitivity coverage, integration coverage), and the cross-layer consistency rules (schema-entity, NFR-vs-design feasibility, sensitivity-response) with a **"Design-layer traceability (referenced, not owned here)"** table pointing at P7/P8 (authoring) and **P9 `validation-plan-artifacts`** (the independent cross-artifact *grade*, per F5 — kept distinct from this producer self-check).

**No script ported** — the HIL source dir ships none (unlike the sibling `authoring-requirements`, which carries `validate-requirements.py`). Nothing to port kernel-free; recorded so `verify-output` does not expect one.

## Structural recipe applied — `standalone`

Placed as its own `plugins/mochiko/skills/authoring-technical-requirements/` dir (SKILL.md + references/). No partner spun out, no merge, no promote:

- **Standalone, scope-trimmed, not dissolved.** P5 remains the authoring authority for **TR / C / NFR / IP** and **owns the `constraints-and-decisions.md` artifact** (template + D-XXX field schema + C↔D/IP traceability). The F2–F5 overlaps trimmed its scope (DS→P7, INT→P8, technique→P6, grade→P9) but did not dissolve it.
- **Independence preserved (the non-negotiable).** This is the **producer half**. Its Quality Checklist + the analysis-internal consistency rules are a **producer self-check**, explicitly **NOT** the independent gate — the gate is `validation-plan-artifacts` (P9, completeness) and `validation-feasibility` (feasibility), each on a **different agent** (devils-advocate / principal-architect) than the producer (technical-analyst). The port never co-mounts a validator skill here.
- **Seams reciprocate the concurrently-ported siblings** (verified on disk): P7 `patterns-entity-modeling/SKILL.md:29,117` already names this skill as the DS *declarer* and itself as the *classifier*; P8 `patterns-api-contracts/SKILL.md:181` already names the upstream integration *concern* vs its per-endpoint *boundary*; P6 `patterns-technical-decisions/SKILL.md:98,104-105` already splits artifact (this skill) vs technique (P6). The phrasing below was written to those exact lines.

## The realized DS → P7 / INT → P8 move (exact reference phrasing, so the seams match)

**DS-XXX → P7 `patterns-entity-modeling`** (`moved-to-sibling-skill` + `dedupe`). P5 keeps a thin declaration; the taxonomy + canonical `data-model.md` template are deduped out (ARTIFACT-TEMPLATES §4 removed). Exact P5 phrasing:
- *When-to-Use:* "Flagging which data the feature treats as sensitive, as an analysis concern (**DS-XXX declaration** — the per-attribute classification is authored in `mochiko:patterns-entity-modeling`)".
- *Seam note:* "The per-attribute data-sensitivity taxonomy (the four classification levels, encryption / retention / access / audit / masking, compliance mapping, and the canonical `data-model.md` template) is authored against the data model in `mochiko:patterns-entity-modeling`."
- *§5 canonical-home callout:* "**Canonical home: `mochiko:patterns-entity-modeling`.** When the data model is authored, each attribute receives its per-attribute sensitivity classification — the four levels (Public / Internal / Confidential / Restricted), the encryption / retention / access-control / audit / masking handling, the compliance mapping, and the classification decision tree — in the canonical `data-model.md`. **Declare the concern here; classify each entity there.**"
- *Reciprocity:* mirrors P7's "that declaration is `mochiko:authoring-technical-requirements`; this skill applies the per-attribute classification and carries it in the data-model.md" (P7 SKILL.md:29/117). The previously-dangling HIL forward ref is **repaired** by pointing at P7, which now contains exactly what P5 points to.

**INT-XXX → P8 `patterns-api-contracts`** (`moved-to-sibling-skill`). P5 keeps a thin declaration; the per-endpoint `x-integration` format is authored in P8 (ARTIFACT-TEMPLATES §5/§6 removed). Exact P5 phrasing:
- *When-to-Use:* "Flagging which external systems the feature depends on, as an analysis concern (**INT-XXX declaration** — the per-endpoint boundary is authored in `mochiko:patterns-api-contracts`)".
- *Seam note:* "The per-endpoint integration boundary — the `x-integration` OpenAPI extension (system, protocol, criticality, failure modes, authentication) — is authored on the wrapping operation in `mochiko:patterns-api-contracts`."
- *§4 canonical-home callout:* "**Canonical home: `mochiko:patterns-api-contracts`.** Once the API contract is designed, the per-endpoint integration boundary — the `x-integration` OpenAPI extension (system name, protocol, API version, criticality, failure modes [detection / impact / fallback], authentication) — is attached to the operation that actually wraps the external system. **Flag the dependency here as an upstream integration concern; the boundary itself is authored there.**"
- *Reciprocity:* mirrors P8's "A requirement may have already flagged the external dependency as an integration concern upstream; the design-phase job here is to attach the per-endpoint boundary to the operation that actually wraps it." (P8 SKILL.md:181). Dangling forward ref **repaired**.

**P5 ↔ P6 boundary** (`dedupe`, not a move). Exact P5 phrasing: description — "This skill owns the constraints-and-decisions.md artifact structure (the D-XXX field schema) and traceability — **NOT** the decision-evaluation technique (use `mochiko:patterns-technical-decisions` to evaluate alternatives and write ADRs)"; §2 pointer — "The decision *technique* lives in `mochiko:patterns-technical-decisions`… reach it… and record its result in the D-XXX slots defined here. **Do not restate the evaluation method in this artifact.**" Mirrors P6 SKILL.md:98/104-105.

## Convention-wiring pass (all 6)

1. **Classification → model-invoked** (default; no `disable-model-invocation`). Agent-consumed by the producer (technical-analyst). ✓
2. **Router registration → NOTED, not applied.** Instruction scopes me out of the shared router/`plugin.json` (edited once per cluster). The **Plan-cluster entry needed** (for the cluster's single router-edit task):
   > `authoring-technical-requirements` | authoring the technical-requirements layer of a spec — TR-XXX decomposition, C-XXX hard constraints, NFR-XXX (numeric targets + measurement), IP-XXX provisioning, and the `constraints-and-decisions.md` artifact + C↔D/IP traceability. Declares DS-XXX / INT-XXX as analysis concerns only (the per-attribute sensitivity taxonomy lives in `patterns-entity-modeling`; the per-endpoint `x-integration` boundary in `patterns-api-contracts`; the decision technique in `patterns-technical-decisions`).

   A new "### Plan cluster (model-invoked …)" section is required (the router currently has Doctrine / Transformer / Setup / Specify sections only). ⏳ out of this artifact's scope, by design — flagged for `verify-output` to check against the router-edit task, not this skill dir.
3. **Trigger phrasing → work-context, graded, de-collided.** "when the user says…" removed; graded MUST/SHOULD retained; ID anchors = P5-owned **TR-/C-/NFR-/IP-**; **de-collided vs P6** (artifact + traceability ≠ decision-evaluation technique). `DS-`/`INT-`/"classify data sensitivity"/"map integrations"/"system integration" deliberately **dropped from the trigger surface** — those route to P7/P8 now; P5 keeps the body declarations. (P5↔P7, P5↔P8 minor collisions resolved by the move; P5↔`authoring-requirements` stays disambiguated by When-NOT-to-Use, rebound to `mochiko:`.) ✓
4. **Path rebinding →** namespace rebinds applied (`humaninloop:` → `mochiko:` ×3 + 2 new sibling refs). **No `.humaninloop/` filesystem paths existed** in body or refs; artifact names are bare filenames — the workspace bind (`.mochiko/specs/<feature>/`) is **lead/P1-owned** (rehome map A#15), not a body edit here → `kept-but-rebind` deferred to the lead. ✓
5. **Decouple persona/skill →** only the **injected-workflow-phase** deny-list category fired (zero agent-names, zero "dispatch," zero "workflow-agnostic" labels — re-confirmed by grep post-port). Scrubbed: `Phase 1`/`Phase 2`, `/humaninloop:plan`, `[Phase 2]` tags, "quality gate before architectural design begins", "Generated during plan workflow", "six plan artifacts". Independence stated by **role** (sibling-*skill* refs are boundaries, not agent names). ✓
6. **Single-source / de-duplicate →** DS taxonomy + `data-model.md` template → P7; `x-integration` format + quickstart → P8; decision technique/evaluation matrix/ADR depth → P6; cross-artifact consistency *grade* → P9. Each is **referenced, not restated**. The analysis web + producer self-checks (which P5 genuinely owns) stay. ✓

## Realized responsibility trace (assess R1–R17 → realized tag; zero silent loss)

| # | Responsibility | Realized tag |
|---|----------------|--------------|
| R1 | TR-XXX authoring (FR→TR decomposition, acceptance criteria, no-orphan/no-unmapped) | **kept** (verbatim §1 + ARTIFACT-TEMPLATES §1) |
| R2 | C-XXX hard-constraint authoring (type/source/severity/impact, constraint-vs-preference) | **kept** (verbatim §2.1 + template §2 Part 1) |
| R3 | D-XXX technology-decision authoring (field schema + C↔D links) | **kept** (D-XXX field schema + C↔D traceability) **+ dedupe → P6** (technique; boundary pointer added; F2 resolved) |
| R4 | NFR-XXX authoring (numeric target + measurement method + source) | **kept** (verbatim §3 + template §3) |
| R5 | IP-XXX infrastructure-provisioning authoring | **kept** (§2.3 + template §2 Part 3; added to When-to-Use + the C/NFR→IP traceability link) |
| R6 | DS-XXX data-sensitivity classification taxonomy | **moved-to-sibling-skill → P7** + **dedupe** (ARTIFACT-TEMPLATES §4 removed); P5 keeps the thin DS-XXX declaration + reference (F3 resolved) |
| R7 | INT-XXX integration mapping (protocol/criticality/failure-modes/fallback) | **moved-to-sibling-skill → P8** + **dedupe** (ARTIFACT-TEMPLATES §5/§6 removed); P5 keeps the thin INT-XXX declaration + reference (F4 resolved) |
| R8 | Traceability web / cross-artifact consistency rules | **kept** (analysis-side TR→FR, NFR→source, C→D, C→impact, **C/NFR→IP**, as producer self-check) ; design-side **referenced → P7/P8** ; cross-artifact *grade* **dedupe → P9** (kept distinct, F5 resolved) |
| R9 | Quality Checklist (producer self-check before finalizing) | **kept** (verbatim; explicitly the producer self-check, **NOT** the independent gate) |
| R10 | Technology-agnostic writing standard (WHAT-not-HOW; real-infra exception) | **kept** (verbatim) |
| R11 | Anti-rationalization discipline (Red Flags, Common Rationalizations, "letter = spirit") | **kept** (verbatim; keystone-passes — the procedural discipline stays in the skill; the *values* echo the technical-analyst persona by design) |
| R12 | Workflow-phase framing ("Phase 1 of /humaninloop:plan", "[Phase 2]", "quality gate before architectural design", "Generated during plan workflow", "six plan artifacts") | **dropped + reason** — injected workflow-coupling; the procedure is true regardless of which workflow drives it (decoupling, convention 5). **Lead accepts.** |
| R13 | `humaninloop:` skill cross-references | **kept-but-rebind** → `mochiko:` (authoring-requirements, patterns-api-contracts, patterns-entity-modeling) + 2 new refs (patterns-technical-decisions, validation-plan-artifacts); **all 5 resolve on disk** (cluster-siblings) — the previously-dangling HIL forward refs are now real |
| R14 | Description trigger phrases ("when the user says…" graded list) | **kept-but-rebind** — re-voiced to work-context; graded MUST/SHOULD + TR-/C-/NFR-/IP- anchors kept; de-collided vs P6; DS-/INT- demoted to body (route to P7/P8) |
| R15 | Output artifact location (bare filenames requirements.md / constraints-and-decisions.md / nfrs.md) | **kept-but-rebind (deferred to lead)** — no hardcoded path in the body to rebind; the `.mochiko/specs/<feature>/` workspace bind is lead/P1-owned (rehome map A#15) |
| R16 | ARTIFACT-TEMPLATES bundle carrying data-model.md (§4) + api.yaml (§5) + quickstart.md (§6) | **moved-to-sibling-skill** (§4 → P7; §5/§6 → P8) + **dedupe**; bundle reframed to the 3 analysis artifacts P5 authors (F3/F4 resolved) |
| R17 | Classification + router registration (new convention-wiring) | **kept** — classification = model-invoked **applied**; Plan-cluster router entry **NOTED** (shared router not edited here, by instruction) |

**Drops:** exactly one — **R12** (workflow-phase framing), carrying its reason for lead acceptance. No capability dropped: every authoring responsibility is `kept`, the two design slices are `moved-to-sibling-skill` to their now-real canonical homes (not deleted, not left dangling), and every cross-ref resolves.

## Deviations / notes

- **Router not registered in this artifact (by design).** The wiring pass's router step is owned by the cluster's single shared router-edit task; I am scoped out of `skills/mochiko/SKILL.md`. Plan-cluster entry text supplied above for that task and flagged for `verify-output`.
- **References were trimmed, not kept verbatim.** Unlike the sibling `authoring-requirements` (whose references were copy-verbatim), P5's references carry the moved DS/INT content, so the move *required* editing them (remove §4/§5/§6; redirect the design-side traceability). This is the DS/INT move applied to the references, not scope creep — restating P7/P8/P9 content would be the dedupe defect `verify-output` fails.
- **`DS-`/`INT-` removed from the trigger surface is intentional and not a discoverability loss.** P7's description now carries "classify data sensitivity"/"DS-XXX"; P8's carries "x-integration"/"integration boundary". P5 stays reachable via its owned TR/C/NFR/IP anchors and routes to P7/P8 in-body. Recorded for the verify pass.
- **No script** in HIL source → none ported (faithful). The deterministic structural checks (ID padding/sequence, FR↔TR coverage presence, cross-ref resolution) remain prose validation rules; no kernel-free script was invented.
- Otherwise no deviations from the finalized disposition.

---

**Done-condition (this transform):** disposition applied (`port-with-edits × standalone`) ✓ · whole HIL dir ported (SKILL + 2 trimmed references; no script to port) ✓ · DS→P7 / INT→P8 realized as thin-declaration + reciprocal reference (seams verified against P7:29/117, P8:181, P6:98/104-105 on disk) ✓ · all 6 wiring conventions applied (router noted, not edited, by instruction) ✓ · zero `humaninloop:` / zero workflow-phase coupling / zero deny-list tokens (grep-clean) ✓ · every responsibility tagged, one lead-accepted drop (R12), no silent loss ✓ · APPLY-only, not self-graded ✓.
**Next:** independent grade — `mochiko:verify-output` via `mochiko:validator` (a different agent), against the realized trace + the five conventions + sound-loop placement + the cluster router-edit task.
