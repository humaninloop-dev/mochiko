# ASSESSMENT: authoring-technical-requirements (P5)

**Run:** `plan` cluster · **Role:** assess/diagnose only (no transform, no grade) · **Date:** 2026-06-30
**Source:** `human-in-loop/plugins/humaninloop/skills/authoring-technical-requirements/` (SKILL.md + `references/ARTIFACT-TEMPLATES.md` + `references/TRACEABILITY-PATTERNS.md`; no scripts present)

```
Class:        skill → branch PLAYS-a-role
Triage:       gate1=n  gate2=y  gate3=y   → full-lens
Disposition:  port-with-edits × standalone   (+ 5 flag-for-reconcile, see below)
```

---

## Step 1 — Branch

**skill → PLAYS-a-role.** What carries weight for this branch: consumed-procedure vs emits-artifact, trigger reliability, sibling overlap, decoupling (independence stated by *role*, not agent name). This is one of the plan **producer**'s (technical-analyst, P2) authoring skills — the analysis-phase authoring procedure.

## Step 2 — Triage (3 gates)

- **gate1 Orchestration-coupled? = no (with note).** It does **not depend on** a kernel / supervisor / command / DAG *to function* — the core authoring procedure (how to write a TR / C / D / NFR / IP) is workflow-agnostic and runs standalone. It **does carry content-coupling**: the body names workflow phases and a sibling command (`Phase 1 (Analysis) of /humaninloop:plan`, `Phase 2 (Design)`). That is cosmetic content-reference, not a functional dependency — scrubbed by `port-with-edits` (see Check 1 + Check 7), not a reason to call it orchestration-coupled.
- **gate2 Multi-responsibility / fans out? = yes.** Authors **six** artifact families (TR-XXX, C-XXX/D-XXX, NFR-XXX, IP-XXX, INT-XXX, DS-XXX) feeding multiple consumers (requirements.md, constraints-and-decisions.md, nfrs.md, plus Phase-2 data-model.md and contracts/api.yaml).
- **gate3 Non-machine-checkable artifact? = yes.** Structural slices are deterministic (ID padding/sequence, FR↔TR coverage presence, cross-ref resolution), but the substance is model-judgment: genuine decomposition vs transcription, whether an NFR target is *meaningful* and business-traced, constraint-vs-disguised-preference, completeness of failure modes, correctness of data classification. → a real (non-degenerate) validator partner is warranted.

Two gates fire → **full 7-check lens** (below).

---

## Step 3 — The 7-check lens

### 1. Orchestration test
- **Orchestrating layer:** the HIL `/humaninloop:plan` command supervisor (DAG/catalog-mediated in HIL). This skill is a **consumed-procedure** the technical-analyst invokes via the Skill tool during the Analysis phase.
- **Content-coupling (body fix, `port-with-edits`):** workflow-phase / command references — `produced during Phase 1 (Analysis) of /humaninloop:plan` (SKILL.md:32), `embedded ... during Phase 2 (Design)` (SKILL.md:34, 113, 115, 121, 123), `[Phase 2]` section tags (113, 121), `Quality gate before architectural design begins` (SKILL.md:21), and the `humaninloop:` skill cross-refs (25, 34, 115, 123). The ARTIFACT-TEMPLATES bundle stamps `Generated during plan workflow` on every template and frames itself as "the six plan artifacts" (ARTIFACT-TEMPLATES.md:1).
- **Orchestration-coupling (structural):** **none originating here.** The skill never loops, dispatches, or drives a verdict. The FAIL-loop / verdict / done-condition live in the plan command (P1), not in this skill. Nothing to rehome *from* P5.
- **Body is kernel-clean:** no MCP tools, no catalog JSON, no DAG references, no `${CLAUDE_PLUGIN_ROOT}`, no brain scripts. Confirms `port-with-edits`, not `redesign`.

### 2. Role (two altitudes)
- **Skill-role:** **emits-artifact** (culminates in authored requirements.md / constraints-and-decisions.md / nfrs.md with a quality checklist), not a pure no-artifact consumed procedure.
- **Team-role conferred on caller:** **producer.** Running this skill makes the technical-analyst a producer of analysis artifacts.
- **Emits a reviewable artifact → needs a validator partner.** The validator already exists **latently**: `validation-plan-artifacts` (P9), whose P1 checklist (FR coverage, orphan TRs, testable criteria, sourced constraints, decision alternatives, NFR measurability) is the exact mirror of this skill's quality/completeness standard. → **flag-for-reconcile F1** (pairing; rides on RQ1).

### 3. Independence
- **No self-grade leak.** The HIL `technical-analyst` agent (agents/technical-analyst.md:46) mounts only producer/authoring skills: `authoring-technical-requirements, patterns-technical-decisions, patterns-entity-modeling, patterns-api-contracts`. **No validation skill is co-mounted.** The validator side (`validation-plan-artifacts`, P9) sits on a **different** agent (devils-advocate). The producer↔validator boundary is already across two agents in HIL.
- **Output:** independence is structurally clean; the port must **preserve** it (never co-mount P9 with P5). No independence fix needed at this skill.

### 4. Verdict-sink / loop-driver
- **Consumers of P5's output:** (a) the plan validator (P9 `validation-plan-artifacts`, P1 phase); (b) downstream Phase-2 design skills (entity-modeling reads TR/sensitivity; api-contracts reads integrations); (c) the plan command lead.
- **What loops on FAIL:** the plan workflow loop, owned by the **plan command (P1)** — not by this skill. P5 holds **no loop-driving responsibility** (correct). Nothing to rehome from here; the loop home is P1.

### 5. Sibling / overlap ("look sideways") — the load-bearing check
- **P6 `patterns-technical-decisions` — shared core + shared output file.** P5 owns "Section 2: Technology Decisions (D-XXX)" (Context/Options/Choice/Consequences/Rationale + C↔D traceability) and writes the *same* file `constraints-and-decisions.md`. P6 *also* owns D-XXX decision records (evaluation matrix, scoring, ADR format, brownfield alignment) into the *same* file. Seam: P5 = the **artifact structure + traceability** authority; P6 = the **evaluation method** depth. → **flag-for-reconcile F2** (dedupe/seam) + a real **trigger collision** (Check 7).
- **P9 `validation-plan-artifacts` — the validator-side mirror (authoring↔validation pair).** P9's P1 checklist grades the exact standard P5 authors to. The pair already exists latently across two agents. The traceability/measurability/completeness standard is effectively **single-sourced content expressed twice** (author-to here, grade-against there). → **flag-for-reconcile F1** (pairing) **+ F5** (dedupe the canonical standard; both reference one source).
- **P7 `patterns-entity-modeling` — DS-XXX seam.** P5 still *describes* Data Sensitivity (DS-XXX: levels, encryption, retention, masking, compliance, decision tree) but flags it "[Phase 2] embedded in data-model.md" — P7's territory. ARTIFACT-TEMPLATES.md §4 carries a **full data-model.md template** (lines ~397–562) that duplicates P7. → **flag-for-reconcile F3** (keep-standard vs `moved-to-sibling-skill`; §4 is a dedupe candidate).
- **P8 `patterns-api-contracts` — INT-XXX seam.** P5 *describes* System Integrations (INT-XXX: protocol/criticality/failure-modes/fallback) but flags it "[Phase 2] embedded in contracts/api.yaml x-integration" — P8's territory. ARTIFACT-TEMPLATES.md §5 (api.yaml) + §6 (quickstart.md) duplicate P8. → **flag-for-reconcile F4** (keep-standard vs `moved-to-sibling-skill`; §5/§6 dedupe candidates).
- **`authoring-requirements` (ported, specify) — low collision.** Business FR-/SC- vs technical TR-/NFR-. The When-NOT-to-Use ("Writing business requirements — Use authoring-requirements", SKILL.md:25) already disambiguates; preserve it, rebind `humaninloop:`→`mochiko:`.

### 6. Coupling audit
- **Hardcoded paths:** none of the kernel kind (no `.humaninloop/`, no catalog/MCP/DAG/script paths). Artifact names appear as bare filenames (requirements.md, constraints-and-decisions.md, nfrs.md) — an **output-location bind** decision for the plan workspace (`.mochiko/specs/<feature>/` per setup/specify precedent), owned by P1/reconcile, not a body fix here. → `kept-but-rebind`.
- **Upstream prerequisite / handoff:** **real.** The skill assumes a business spec with FR-XXX exists upstream ("Map every business FR to one or more TRs"; "Translate business specifications"). That is a handoff edge **spec.md (specify output) → P5**, which is the **plan command's** wiring (P1), not a body responsibility of P5.
- **Determinism boundary:** structural checks (ID format/sequence, FR↔TR coverage presence, cross-ref resolution) are deterministic and are candidates for a future validator-side assert (the ported sibling `authoring-requirements` added a `validate-requirements.py`; **this skill ships no script**). The substance (decomposition quality, NFR meaningfulness, constraint-vs-preference, classification correctness) is model-judgment → the P9 validator partner is **real, not degenerate**.

### 7. Conventions + loop placement
- **Classification:** model-invoked (agent-consumed by the producer). Assign default model-invoked (no `disable-model-invocation`). ✓
- **Discoverability / router:** **not yet registered** — the mochiko router has no Plan-cluster section. Needs a new "Plan cluster" entry at wiring time.
- **Trigger phrasing (convention 3):** HIL description is in **"when the user says…"** voice (SKILL.md:3). It already carries graded RFC-2119 **MUST/SHOULD** and the exact ID-prefix phrases (TR-/C-/NFR-/IP-/INT-/DS-, "non-functional", "system integration", "infrastructure provisioning", "data classification"). Per the ported-sibling precedent (`authoring-requirements` re-voiced to "when authoring the functional-requirements layer…"), **re-voice to work-context** while keeping the graded phrases + ID anchors. → `port-with-edits`.
- **Decoupling scan (deny-list):**
  - sibling-agent names → **none** (body is impersonal; no "Technical Analyst" / "Devil's Advocate"). ✓
  - "dispatch" → **none**. ✓
  - "workflow-agnostic" / independence-by-declaration meta-label → **none**. ✓
  - **injected workflow modes/paths/phases → PRESENT** (the only deny-list category that hits): `Phase 1`/`Phase 2`, `/humaninloop:plan`, `[Phase 2]` tags, "quality gate before architectural design", "Generated during plan workflow" stamps. Keystone test: *"produced during Phase 1 of /humaninloop:plan" → only-in-plan-workflow → CUT*; *"Map every business FR to one or more TRs" → true of anyone authoring technical requirements → KEEP.* (Confirms run-goal #3: the body is clean of agent-coupling; only workflow-phase coupling needs scrubbing.)
- **Producer↔validator pairing:** present latently (P9 on the devils-advocate, a different agent); must be carried forward, gated by RQ1. → F1.
- **Sound-loop placement:** the done-condition / independent gate / human gate live in the plan command (P1). P5 supplies a **Quality Checklist** that is a **producer self-check**, NOT an independent validation gate — it stays with the producer and must not be mistaken for, or merged with, the P9 gate. No loop responsibility to rehome from P5.

**Procedure-vs-persona note (as requested):** this is a **procedure-heavy** skill (field tables, ID rules, decomposition examples, traceability patterns, measurement-method writing, classification decision tree) → procedure **stays a skill** (convention 4). **Nothing should fold *into* the producer persona.** The only persona-adjacent content is the anti-rationalization *values* (Red Flags / Common Rationalizations), which already echo the technical-analyst persona's "What You Reject / Embrace." Treat that as **`dedupe`-awareness** (the *values* belong to the persona; the *procedural discipline* stays in the skill) — do **not** wholesale-move it.

---

## Step 4 — Disposition

**`port-with-edits × standalone`** (+ flags).

- **Body = port-with-edits:** body is mostly mochiko-clean (no kernel deps, impersonal voice); localized fixes only — (1) scrub workflow-phase/command coupling; (2) re-voice description to work-context; (3) rebind `humaninloop:`→`mochiko:`; (4) trim/seam the DS-XXX/INT-XXX sections + the data-model/api/quickstart templates in ARTIFACT-TEMPLATES that overlap P7/P8 (per reconcile). Not `keep-verbatim` (carries coupling); not `redesign` (approach is sound, no kernel).
- **Structural = standalone:** P5 stays one skill in its own home; it is the authoring authority for TR/C/NFR/IP. The overlaps (F2–F5) **trim its scope** via dedupe/seam but do **not** dissolve it. Relational moves are flagged, not guessed.

---

## Step 5 — Responsibility trace (every responsibility tagged)

| # | Responsibility | Tag |
|---|----------------|-----|
| R1 | TR-XXX authoring (FR→TR decomposition, acceptance criteria, no-orphan/no-unmapped) | **kept** |
| R2 | C-XXX hard-constraint authoring (type/source/severity/impact, constraint-vs-preference test) | **kept** |
| R3 | D-XXX technology-decision authoring (context/options/choice/consequences/rationale, C↔D links) | **flag-for-reconcile F2** → tentative `kept` (field schema + traceability) with evaluation depth `dedupe`→P6 |
| R4 | NFR-XXX authoring (numeric target + measurement method + source; "fast is not a requirement") | **kept** |
| R5 | IP-XXX infrastructure-provisioning authoring (type/source/acceptance; every platform-implying constraint → IP) | **kept** |
| R6 | DS-XXX data-sensitivity classification (levels, encryption, retention, masking, compliance, decision tree) | **flag-for-reconcile F3** → `kept` (standard) or `moved-to-sibling-skill` (P7); ARTIFACT-TEMPLATES §4 dedupe |
| R7 | INT-XXX integration mapping (protocol/criticality/failure-modes/fallback) | **flag-for-reconcile F4** → `kept` (standard) or `moved-to-sibling-skill` (P8); ARTIFACT-TEMPLATES §5/§6 dedupe |
| R8 | Traceability web / cross-artifact consistency rules (TR→FR, NFR→source, C→D, C→impact, C/NFR→IP) | **kept** + **dedupe F5** (single-source vs P9's grading checklist) |
| R9 | Quality Checklist / completeness checks (producer self-check before finalizing) | **kept** (stays with producer; NOT the independent gate) |
| R10 | Technology-agnostic writing standard (WHAT-not-HOW table; real-infra constraint exception) | **kept** |
| R11 | Anti-rationalization discipline (Red Flags, Common Rationalizations, "letter = spirit") | **kept** (keystone-passes) + dedupe-awareness vs technical-analyst persona values |
| R12 | Workflow-phase framing ("Phase 1 of /humaninloop:plan", "Phase 2 (Design)", "[Phase 2]" tags, "quality gate before architectural design", "Generated during plan workflow") | **dropped + reason:** workflow-coupling injected into the body; decoupling per convention 5 — the procedure is true regardless of which workflow drives it; independence stated by role, not by workflow phase/command. **Lead must accept.** |
| R13 | `humaninloop:` skill cross-references (authoring-requirements, patterns-api-contracts, patterns-entity-modeling) | **kept-but-rebind** (`humaninloop:`→`mochiko:`) |
| R14 | Description trigger phrases ("when the user says…" graded MUST/SHOULD list) | **kept-but-rebind** (re-voice to work-context; keep graded phrases + ID anchors; de-collide vs P6/P7/P8) |
| R15 | Output artifact location (bare filenames requirements.md / constraints-and-decisions.md / nfrs.md) | **kept-but-rebind** (bind to plan workspace `.mochiko/specs/<feature>/` per precedent; P1/reconcile decides path) |
| R16 | ARTIFACT-TEMPLATES.md bundle carrying data-model.md (§4) + api.yaml (§5) + quickstart.md (§6) templates | **flag-for-reconcile F3/F4** (`dedupe`/`moved-to-sibling-skill` → P7/P8; possible P10 plan-template / P12 report overlap) |
| R17 | Classification + router registration (model-invoked; Plan-cluster router entry) | **kept** as new convention-wiring (assigned at transform) |

Every responsibility carries a tag → trace complete (assessment done-condition met). One `dropped` (R12) carries a reason for the lead to accept.

---

## Reconcile flags (for `reconcile-cluster`, full cluster context)

- **F1 — pairing (P5 producer ↔ P9 validator).** P5 emits reviewable, model-judgment artifacts; the independent validator is `validation-plan-artifacts` (P9), already on a different agent (devils-advocate). Carry the producer↔validator pair forward; exact reviewer shape rides on **RQ1**. Never co-mount P5 + P9 on one agent.
- **F2 — overlap/seam with P6 `patterns-technical-decisions`.** Shared D-XXX decision-record concept + shared output file `constraints-and-decisions.md`. Seam to draw: P5 = artifact structure + C↔D traceability; P6 = evaluation method (criteria matrix, scoring, ADR depth). Decide dedupe vs explicit seam. (+ trigger collision, below.)
- **F3 — seam with P7 `patterns-entity-modeling`.** DS-XXX classification standard + the full data-model.md template in ARTIFACT-TEMPLATES §4. Decide `kept` (P5 keeps the classification *standard*) vs `moved-to-sibling-skill` (hand DS-XXX to P7); the §4 template is a dedupe candidate.
- **F4 — seam with P8 `patterns-api-contracts`.** INT-XXX mapping standard + the api.yaml (§5) / quickstart.md (§6) templates in ARTIFACT-TEMPLATES. Decide `kept` (standard) vs `moved-to-sibling-skill` (P8); §5/§6 dedupe candidates.
- **F5 — dedupe the authoring↔validation standard (P5 ↔ P9).** The traceability/measurability/completeness criteria are one standard expressed twice (author-to / grade-against). Decide the canonical home and have both reference it.

## Trigger-collision notes (convention 3)

- **P5 ↔ P6 — REAL collision.** Both fire on the decision/D-XXX space ("document decision", "decision record", "rationale", "trade-offs", constraints-and-decisions.md). A model could pick either for "document this technology decision." De-collide at the wiring pass (or resolve via F2 seam).
- **P5 ↔ P7 — minor.** "classify data sensitivity" / "DS-" / "data classification" overlap P7's data-model territory.
- **P5 ↔ P8 — minor.** "map integrations" / "INT-" / "system integration" overlap P8's api-contracts territory.
- **P5 ↔ authoring-requirements (specify, ported) — low.** Technical TR-/NFR- vs business FR-/SC-; When-NOT-to-Use already disambiguates. Preserve + rebind namespace.

## Decoupling hits (scrub list, `port-with-edits`)

Only one deny-list category fires — **injected workflow modes/paths/phases** (zero agent-names, zero "dispatch", zero "workflow-agnostic" labels):
- SKILL.md:21 "Quality gate before architectural design begins" (workflow-position framing)
- SKILL.md:32 "produced during Phase 1 (Analysis) of `/humaninloop:plan`"
- SKILL.md:34, 113, 115, 121, 123 "embedded … during Phase 2 (Design)" + `[Phase 2]` section tags
- SKILL.md:25, 34, 115, 123 `humaninloop:` skill cross-refs → rebind `mochiko:`
- ARTIFACT-TEMPLATES.md:1 "the six plan artifacts" + per-template "Generated during plan workflow" stamps
- Description (SKILL.md:3) "when the user says…" → re-voice to work-context (keep graded MUST/SHOULD + ID anchors)
```
