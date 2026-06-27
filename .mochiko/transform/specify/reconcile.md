# Reconcile — cluster `specify` (CORE scope)

Run: `transform-cluster specify` · Phase 2 (reconcile) · Producer: `transform-producer` · Skill: `mochiko:reconcile-cluster`
Reconciled: 2026-06-27 · Governed by `loop-discipline` · Inputs: P1–P14 assessments + contract.md + context.md + the setup precedent (`commands/setup.md`, `.mochiko/transform/setup/reconcile.md`)

> **Reconcile decides; it does not edit.** This is the finalized disposition set + the single coherent rehome map handed to Phase 3 (`transform-recipes`). **Zero open flags** (see §H). Honors the Phase-0 human gate: specify-CORE only (analyst producer + advocate critic + lead referee); design track deferred; `devils-advocate`'s `validation-plan-artifacts`/`validation-task-artifacts` rebind-by-reference only; kernel-free.
>
> **The headline of this cluster is the inverse of setup.** Setup had to *construct* an independent validator (split + promote). Specify already *ships* one — `requirements-analyst` (producer) ↔ `devils-advocate` (validator), disjoint skills, different agents. So **no new primitive is created this run.** The real transformation is (1) re-landing a *dissolving DAG orchestration* (`state-analyst` P4 + the two strategy skills P9/P10 + `context-template` P14) onto the redesigned `specify` command **lead**, and (2) **ADDING the four `loop-discipline` gates HIL never had** (default-FAIL done-condition, lead-owned verdict, hard bound + kill-switch, human acceptance gate). This is the cluster's #1 silent-drop guard.

---

## Decision-area summary (the 8 settled calls)

| # | Decision area | Resolved move |
|---|---------------|---------------|
| 1 | The single coherent rehome map onto the lead | **§B** — one map; every non-kernel responsibility of P4 + P14 + P9/P10 survivors + P1/P12/P13 consumption lands **exactly once** on the lead / loop-discipline / contract. No double-home, no inter-primitive drop. |
| 2 | Producer↔validator pairing | **CONFIRM, not construct.** `requirements-analyst` ↔ `devils-advocate`; skills disjoint; **verdict ownership → lead** (referee owns the clearing verdict; the advocate's severity buckets + recommended status are INPUT). **No new partner primitive.** |
| 3 | The two strategy skills + Gap Classification | **Both dissolve** (neither created as a mochiko skill). Generic patterns → `dedupe` into `loop-discipline`. **Gap Classification → folded into `loop-discipline`** (universal, additive). **Input Assessment → lead** (entry triage). Targeted-revision → lead (round-N>1 dispatch). Spec done-condition → lead's contract. |
| 4 | `authoring-requirements` ↔ `authoring-user-stories` | **keep-distinct-standalone** (both on the analyst). **Do NOT factor** the shared discipline substrate (low drift risk; factoring manufactures coupling). De-collide the "acceptance criteria" trigger (wiring). |
| 5 | `analysis-iterative` mount + dual-mode | **Lead invokes it as a model-invoked pre-step** when lead-owned Input Assessment flags sparse input. **Keep dual-mode in one skill** (the two modes share one questioning engine — splitting duplicates it). De-collide enrichment-vs-gap-review triggers with P6. |
| 6 | The ADDED loop machinery | All four ADDED to the lead (P1): default-FAIL done-condition (inlined contract), lead-owned verdict, hard round cap + no-progress + STOP kill-switch, **human acceptance gate** (the headline missing gate; setup-G3 precedent). |
| 7 | Templates + path/workspace rebind | spec-template header = workspace-as-state (lead-stamped; `status`→loop-done-condition; `feature_id` rebind off numbering). `iteration`→`round` **identical across P12/P13**. Verdict→loop-driver → lead. Lead seeds `spec.md`. Convention: **`.mochiko/specs/<feature>/`** + `.mochiko/memory/`. |
| 8 | Convention-wiring preview | **§G** — classifications, router registrations, `humaninloop:`→`mochiko:` rebinds, deny-list scrubs, dangling-ref scrubs (advocate's 2 deferred skills; authoring `patterns-*`; analysis design-track links). |

---

## A. Resolved relational verdicts — every `flag-for-reconcile` → one concrete move

Each row resolves a flag from the assessments. Rationale grounded in `loop-discipline` (LD), the minimalism governor (MG), or workflow-first (WF). The full flag inventory closed here: P1 RF1–RF7 · P2 (3) · P3 (2) · P4 (4) · P5 F1–F3 · P6 (4) · P7 (1) · P8 (3) · P9 (4) · P10 (4) · P11 F-1/F-2 · P12 (2) · P13 F-P13-1 · P14 F-CTX-1/2/3.

### Agenda 1 — INDEPENDENCE: CONFIRM the pair (the sharp contrast with setup)

| # | Flag (source) | Resolved move | One-line rationale |
|---|---|---|---|
| 1a | Producer↔validator pairing already satisfied (P1 RF2, P2 f1, P3 f1, P6 F1, P7 Check3, P8 f2, P11 Check3) | **pair — CONFIRM, do not construct.** Record the existing pair in the rehome map; create **no** new agent or skill. | LD-2: independence holds by construction — producer agent (`requirements-analyst`, skills `{authoring-requirements, authoring-user-stories}`) ≠ validator agent (`devils-advocate`, skill `{analysis-specifications}`); skill sets disjoint. The setup self-grade leak is **absent** here. |
| 1b | Verdict was evaluated *autonomously by the state-analyst* (P1 A18′/RF2/C2, P4 b6, P6 F2, P13 R5) | **verdict ownership → lead** (rehome §B). The advocate emits the grounded review (severity-bucketed gaps + clarifying questions + a *recommended* status `ready/needs-revision/critical-gaps`); the **lead owns the clearing verdict** and the done-condition. | LD-2: the referee owns the verdict so the pair cannot collude into agreeableness. The advocate's status is the validation **input**; the lead never lets it auto-complete the loop (that is the missing-human-gate fix). |
| 1c | Independence guard on the `state-analyst` absorb (P4 flag-3) | **Ratify:** the absorb lands routing/convergence/recommendation judgment on the lead **as referee only**. The lead mounts **no** producer or validator skill. | LD-2: loop-routing + verdict-ownership are referee duties; sound by construction. Explicitly ratified so transform cannot drift into a self-grade by co-mounting. |
| 1d | Do NOT add a clearing PASS/FAIL into `analysis-specifications` (P6 verdict-ownership boundary) | **Keep P6 a gap-finder.** Its severity buckets feed the lead's verdict; the skill stays input, not gate. | LD-2 + MG: the `validation-constitution` binary-validator shape does **not** transfer to a critic-side skill; that would double-own the verdict. |

**Result:** the pair is confirmed, not built. `principal-architect`'s split/promote has **no analogue** here. The single independence fix is structural verdict-ownership → lead (§B).

### Agenda 2 — THE DISSOLVING DAG ORCHESTRATION → LEAD (resolved as the rehome map §B)

| # | Flag (source) | Resolved move | One-line rationale |
|---|---|---|---|
| 2a | `state-analyst` (P4) dissolves; its non-kernel (b) judgment set must re-home, not vanish (P1 RF1, P4 flag-1 — the run's #1 silent-drop guard) | **absorb-into-lead** — every (b) responsibility lands on the lead in §B; kernel (a) mechanics `dropped + reason`. | LD + contract §1: briefing/recommendation/convergence-watch/report-parse must survive on the lead; only the DAG/MCP/catalog mechanism dies. |
| 2b | Report-parse boundary: lead-parses vs producer/validator emit lead-ready findings (P4 flag-2) | **the lead reads the reports directly** (structured by templates P12/P13). No separate parse layer; HIL's "Supervisor NEVER reads reports" was a kernel-era context-protection artifact. | LD-2: the referee reads the validator's grounded report to own the verdict. (This is P1 A30's deliberate reversal — gated.) |
| 2c | `context-template` (P14) is the live state carrier; its state/loop/dispatch fields re-home onto the *same* lead (P14 F-CTX-1) | **absorb-into-lead** — folded into in-session state + workspace-as-state (§B); coordinated with P4/P9/P10 so each field homes **once**. | MG + WF: mochiko runs the loop in one session; one state mechanism = the lead. CONFIRMS + STRENGTHENS setup's memory model. |
| 2d | The four `loop-discipline` gates are MISSING in HIL (P1 RF6/C1–C4, P12, P13, P14 R2) | **ADD** (rehome-orchestration Job 2): default-FAIL done-condition, lead-owned verdict, hard bound + no-progress + kill-switch, human acceptance gate. | LD-1/2/3/4: the real transform is installing the gates the original never had, not merely shedding the DAG. |

### Agenda 3 — THE TWO STRATEGY SKILLS + GAP CLASSIFICATION (P9 × P10 × loop-discipline, resolved jointly)

| # | Flag (source) | Resolved move | One-line rationale |
|---|---|---|---|
| 3a | 3-way dedupe: strategy-specification × strategy-core × loop-discipline; does ANY residual strategy skill survive? (P9 flag-1, P10 flag-1/flag-4, P4 flag-4) | **NEITHER survives as a mochiko skill.** Both dissolve. The generic patterns `dedupe` into `loop-discipline`; the additive/spec-specific slices rehome (§B). | MG + "same doctrine never authored twice": `loop-discipline` already encodes 6 of strategy-core's 9 patterns 1:1 + strategy-specification's produce-then-validate / no-progress / round-cap. A standalone strategy skill would re-author the doctrine. |
| 3b | Home the **Gap Classification** taxonomy residual (knowledge→research / preference→user / scope→halt) (P10 flag-2/flag-3 R8+R9, P9 c, P4 b4) | **fold into `loop-discipline`** as additive human-gate-routing / FAIL-routing guidance (an edit to the universal doctrine skill). The specify lead **applies it by consuming `loop-discipline`** (it already does). | It is a **universal** pattern (it lived in strategy-**core**, the cross-workflow skill), not specify-flavored; it extends LD Req-4 placement ("preference-gap only") + Req-3.4 escalate with the routing rule. Hosting it specify-only would re-create "authored twice" when plan/implement port. **It is a real pattern a real workflow (specify) needs now — extraction, not speculative pre-definition** (per CLAUDE.md). *Alternative offered at the gate: keep as specify-lead prose if the supervisor prefers `loop-discipline` frozen.* |
| 3c | Home strategy-specification's survivors: **Input Assessment** (sparse/rich triage), **targeted-revision**, **spec done-condition**, **constitution-prereq** (P9 flag-2, P4 b9) | **Input Assessment → lead** (spec-loop entry triage; pairs with Agenda 5). **Targeted-revision → lead** (round-N>1 dispatch instruction). **Spec done-condition → lead's contract** (fills LD's mandated done-condition). **Constitution-prereq → contract handoff edge.** | LD + MG: these are loop-driving *content* the lead owns; none is a reusable standalone skill. Targeted-revision mirrors setup's "address EVERY fix-list item; do not regress passing items." |
| 3d | "Re-enrichment / Post-pass-1 enrichment" anti-pattern mis-filed in strategy-core (P10 R7) | **dedupe → with Input Assessment on the lead** (rides with the enrichment triage). | MG: it is spec-flavored, not universal; lands where Input Assessment lands. |
| 3e | "advisory, not prescriptive / may deviate" stance (P10 R10) + "consumed by State Analyst" role (P10 R11, P9 framing) | **dropped + reason** — superseded by `loop-discipline`'s non-negotiable framing; the state-analyst consumer dissolves. | LD: carrying "may deviate" would contradict the doctrine; the brief-the-Supervisor consumption model is kernel-era. |

### Agenda 4 — THE AUTHORING PAIR (P7 × P8): keep distinct, do not factor

| # | Flag (source) | Resolved move | One-line rationale |
|---|---|---|---|
| 4a | `authoring-requirements` ↔ `authoring-user-stories`: shared tech-agnostic-discipline substrate + ~80%-identical validation-script scaffold; distinct artifact axes (P7 sibling-overlap, P8 f1) | **keep-distinct × standalone** for both (NOT merge). Resolves P8's `flag-for-reconcile` → `standalone`. | Reconcile rule: merge only on a genuinely shared *core*. Here each owns a substantial distinct body (FR-XXX/RFC-2119 vs P#/Given-When-Then); HIL deliberately separated them. No thin-variant-over-shared-core → no merge. |
| 4b | The shared discipline substrate — factor to one source or leave duplicated? (P7/P8 dedupe candidate) | **DO NOT factor.** Each skill keeps its own "WHAT-not-HOW / measurable-outcome" prose and its own script. | MG: unlike setup's Essential-Floor (a NON-NEGOTIABLE *definition* with real drift risk → earned dedupe), this is shared *taste*; drift is harmless, and factoring manufactures a new cross-skill coupling against kernel-free minimalism. Both assessors leaned this way. |
| 4c | Overlapping "acceptance criteria" trigger (P8 f1, P7) | **de-collide (wiring)** — `authoring-user-stories` keeps "acceptance scenarios / Given-When-Then"; `authoring-requirements` drops "acceptance criteria," anchors on "functional requirements / FR- / success criteria / SC-." | Not structural — handed to the convention-wiring pass (§G). Same caller (analyst) so misfire risk is low; phrasing still split to each skill's work-context. |

### Agenda 5 — ENRICHMENT MOUNT + DUAL-MODE (P5)

| # | Flag (source) | Resolved move | One-line rationale |
|---|---|---|---|
| 5a | Mount point: lead invokes enrichment as a pre-step vs analyst owns it (P5 F1, P4 b9, P14 R7) | **lead invokes `analysis-iterative` as a model-invoked pre-step** when lead-owned **Input Assessment** flags sparse input (missing Who/Problem/Value). Enrichment *procedure* stays in the skill; *when-to-enrich* is lead prose. | MG + decoupling: the triage is loop-entry orchestration (a referee duty, = P4 b9 `moved-to-lead`); the skill stays a clean reusable conditioner that does not name its caller or decide its own invocation (agent-dispatch convention). Independence-safe (P5 Check3: only mounting on the validator is forbidden). |
| 5b | Dual-mode scope: keep the general brainstorm mode or split it out (P5 F2) | **keep dual-mode in one skill** (port whole, decoupled). The general brainstorm mode rides along as an inherent capability of the same engine; note it as a **general/shared** skill in the router. | MG/WF distinction from setup: setup split `analysis-codebase` because its modes were *separable procedures with distinct consumers + a JSON schema*. Here the two modes are **one adaptive-questioning engine** with two output templates — splitting duplicates the engine or manufactures a shared-ref coupling. Keep whole. |
| 5c | Trigger de-collision with `analysis-specifications` (P5 F3, P6 flag-3) | **de-collide (wiring); boundary holds.** `analysis-iterative` = pre-spec enrichment ("enrich / brainstorm / think through / sparse description / Who-Problem-Value"); `analysis-specifications` = post-draft gap-review ("review spec / find gaps / what's missing"). | Different lifecycle stages (pre-authoring input vs post-authoring critique) → triggers stay disjoint. Convention-wiring (§G), not structural. |

### Agenda 6 — TEMPLATES (P11 / P12 / P13): path + identity + verdict rebind

| # | Flag (source) | Resolved move | One-line rationale |
|---|---|---|---|
| 6a | spec-template header identity + status model (P11 F-1) | **port-with-edits (variant b — soften):** `status` semantics rebind from DAG-verdict → the **lead's loop done-condition**; `created` lead-stamped; `feature_id` **kept-but-rebind** to a lead-derived workspace identifier (NOT dropped — workspace identity is useful) with the `create-new-feature.sh` *numbering scheme* dropped. | Run goal #2 (workspace-as-state): carrying DAG/branch presumptions into the artifact the whole loop converges on would fail the decoupling test. Consistent with P14 R3a (identity → lead) + R3b (numbering scheme → dropped). |
| 6b | Lead seeds `spec.md` from the template (P11 F-2, P1 A8′) | **moved-to-lead** — the excluded `create-new-feature.sh`'s `cp $TEMPLATE $SPEC_FILE` placement re-homes to the lead (kernel-free `mkdir -p` + seed, setup precedent). | WF: the placer script is excluded; the placement *capability* survives on the lead. |
| 6c | `{{iteration}}` rebind, kept IDENTICAL across the P12/P13 pair (P12 flag-2, P13 cross-ref) | **`{{iteration}}` → `{{round}}`** (lead's bounded-loop round counter) — **same vocabulary on both report templates.** | Matched producer/critic report pair; consistent header convention. The DAG `pass_number` → mochiko round counter. |
| 6d | P12 analyst-report edit depth, gated on P4 (P12 flag-1) | **port-with-edits:** with P4 resolved (lead reads reports directly), **demote the counts-first shape** to a brief optional disclosure; lead reads Summary / Assumptions / what-changed-this-round; the DAG **outcome-trajectory metric** feed → `dropped + reason` (kernel). | The counts duplicated spec.md (which the advocate reads anyway) and fed the dropped DAG trajectory metric; low value for a direct-reading lead. |
| 6e | P13 verdict→loop-driver + clarification→human-gate + gap→revision consumption (P13 F-P13-1) | **moved-to-lead** (§B). The verdict CONTENT stays in the template (`keep-verbatim`); the DAG-gate + state-analyst parse machinery dissolve; the lead reads the verdict directly. | Chain-integrity guarantee: if the lead does not pick up verdict consumption, the bounded loop is silently broken. |

### Agenda 7 — CROSS-CLUSTER, DEFERRED & ORPHANS

| # | Flag (source) | Resolved move | One-line rationale |
|---|---|---|---|
| 7a | `devils-advocate`'s `validation-plan-artifacts` + `validation-task-artifacts` (P3 f2, R7/R8) | **moved-to-other-cluster — stub / rebind-by-reference only.** Remove from the live `skills:` frontmatter so it never points at an unported skill; log the re-mount edge for when plan/tasks port. **NOT dropped.** | Contract pre-decides "rebound by reference only." A live mount of an unported skill dangles (`verify-output` Tier-1 FAIL). |
| 7b | `analysis-iterative` general brainstorm mode (P5 F2) | resolved at Agenda 5b: **kept** in the dual-mode skill; noted **general/shared** in the router. | (Not a move-out; the engine is shared.) |
| 7c | `context-template` cross-workflow schema (P14 F-CTX-2, R16) | **moved-to-other-cluster (note):** plan/tasks/implement **inherit** in-session-lead + workspace-as-state; they do **NOT** re-port a context carrier. | Absorbing here sets the workspace-as-state precedent for the peers. |
| 7d | strategy-core cross-workflow shared substrate (P10 flag-3) | **dedupe done once, cluster-globally:** plan/implement ports must **not** expect a strategy-core skill; their strategy patterns also dedupe into `loop-discipline`; Gap Classification lives in `loop-discipline` (shared), reachable by all. | WF: a cluster-global dedupe; hosting the survivor in the universal doctrine skill serves the peers. |
| 7e | `commands/audit.md` also references `analysis-specifications` (P6 flag-4) | **skill stays in specify** (no move); audit's reference rebinds to the mochiko-ported skill **if/when audit ports**. | Informational cross-ref; not a structural move this run. |
| 7f | Excluded kernel: `specify-catalog.json`, `create-new-feature.sh`, `check-prerequisites.sh`, `common.sh`, `setup-plan.sh`, `hil-dag` MCP; native `Explore` targeted-research node | **excluded — confirmed, no port** (already `[-]` in REGISTRY); the `Explore` research node is **not** ported (no specify-core consumer). | Kernel-free constraint; matches Phase-0 scope. |

---

## B. THE REHOME MAP — the dissolving DAG orchestration → its new home (THE #1 SILENT-DROP GUARD)

The HIL `state-analyst` (P4) + the `specify` command's kernel scaffolding (P1) + the two strategy skills (P9/P10) + the `context-template` carrier (P14) **all dissolve onto the SAME mochiko `specify` command lead.** This is **one** map. Every non-kernel responsibility lands **exactly once**; each conceptual home cross-references every assessment row that feeds it, so nothing is double-homed and nothing falls between primitives. **No kernel reintroduced** — every home is the lead command, an existing agent/skill, `loop-discipline`, the contract, or an accepted drop.

> **How to read the "Source rows" column:** the same responsibility is seen by several assessments from different angles (e.g. the round counter appears in P4 b8, P14 R2, P1 A26). They are **one** landing here. Where a doctrine is universal, the *doctrine* `dedupe`s into `loop-discipline` while the *instance* (the count, the sentinel path, the spec-specific content) lives on the lead.

### B.1 — Loop-control orchestration → lead (doctrine dedupes into `loop-discipline`)

| Orchestration responsibility | Re-homes to | Source rows (homed once) |
|---|---|---|
| Sequencing / produce→critique→revise dispatch order | **lead** (the fixed produce→validate loop) | P1 A17′/A21, P4 b10, P3 R13, P2/P7/P8 (loop they feed) |
| **Verdict ownership** — read the advocate's grounded report + own the clearing verdict | **lead** (referee); doctrine → `dedupe` LD-2 | P1 A18′/C2, P4 b6, P6 F2, P13 R5, P12 (loop) |
| FAIL-loop driving (`needs-revision` → new bounded round) | **lead**; doctrine → `dedupe` LD-2/3 | P1 A22, P4 b6, P3 R13, P13 R5 (`needs-revision`) |
| Convergence-watch / **no-progress exit** (gap set unchanged round-over-round) | **lead's no-progress guard**; doctrine → `dedupe` LD-3.2 | P1 A25, P4 b2/b11, P14 (loop), P10 R2 (Pass Evolution) |
| **Hard round cap** (HIL soft 5-pass → mochiko hard cap) | **lead's bounded loop**; doctrine → `dedupe` LD-3.1 | P1 A26, P4 b8, P14 R2, P9/P10 (5-pass guardrail) |
| **Kill-switch / budget** (out-of-band STOP, checked before each dispatch) | **lead** (NEW — HIL had none); doctrine → `dedupe` LD-3.3 | P1 C3 (NEW) |
| Escalation surfacing (`critical-gaps`, non-resolvable error → human gate) | **lead**; doctrine → `dedupe` LD-3.4 | P1 A24/A27, P4 b7/b13-esc, P13 R5 (`critical-gaps`), P10 R3 |
| Produce-then-validate ordering (never skip the advocate gate) | **lead**; doctrine → `dedupe` LD-2 | P4 b10, P9 (produce-then-validate), P10 R1 |
| Read/write referee boundary (lead authors nothing, grades nothing at skill level) | **lead** (referee posture); partial `dedupe` LD-2 | P4 b15, P1 (independence guard 1c) |

### B.2 — State + workspace → lead (workspace-as-state; **one** state mechanism)

| Orchestration responsibility | Re-homes to | Source rows (homed once) |
|---|---|---|
| Run/loop state (status lifecycle, round count, active feature) | **lead, in-session** (no separate context file) | P14 R1/R2/R3a, P4 b-state, P1 A12′ |
| Feature/workspace identity + layout | **lead** → `.mochiko/specs/<feature>/` (workspace-as-state) | P14 R3a/R10, P11 R3, P1 A8′/A14, P4 r2 |
| File-path registry (spec.md / analyst-report.md / advocate-report.md) | **lead** (fixed workspace layout, not a registry field) | P14 R10, P12/P13 output paths, P1 A13 |
| User-input + enriched-input + project-context transport | **lead, in-session** (assembled into dispatch prompts; project context read from `.mochiko/memory/`) | P14 R4/R6/R7/R8, P1 (context) |
| Clarification history (multi-round human Q&A) | **lead, in-session** human-gate history (persist to workspace if durability needed) | P14 R13, P4 b12, P13 R3 |
| State recovery (resume from workspace evidence, not a context `phase` field) | **lead** (simplified; setup precedent) | P1 A29′, P14 R1 |

### B.3 — Briefing / dispatch → lead (in-session, `agent-dispatch` convention)

| Orchestration responsibility | Re-homes to | Source rows (homed once) |
|---|---|---|
| State briefing (summary, round context, relevant patterns) | **lead, in-session** | P4 b1, P1 B1 |
| Dispatch-prompt construction / domain-agent briefing (point agents at workspace artifacts; minimal brief) | **lead** (`agent-dispatch.md` guide; not a file field) | P4 b11, P14 R12, P1 B8/A35 |
| **Report-parsing → lead reads reports directly** (structured by P12/P13; no parse layer) | **lead** | P4 b3, P1 A19/A30 (reversal), P12/P13 (read by lead) |
| Decision-node answer persistence (clarification answers) | **lead, in-session** (→ workspace if durable) | P4 b12, P14 R13 |

### B.4 — Additive survivors (NOT in `loop-discipline`) → their single homes

| Survivor | Re-homes to | Source rows |
|---|---|---|
| **Gap Classification** taxonomy (knowledge→research / preference→user/human-gate / scope→halt) + "preference-to-research" corollary | **`loop-discipline`** (additive edit; universal; applied by the lead via consumption) | P10 R8/R9, P9 c, P4 b4 |
| **Input Assessment** (sparse vs rich triage; domain-context can substitute for enrichment) + "post-pass-1 enrichment is wasteful" | **lead** (spec-loop entry triage → invokes `analysis-iterative` when sparse) | P9 (Input Assessment), P4 b9, P5 #5, P10 R7 |
| **Targeted-revision tactic** (carry the specific flagged gaps forward; focus revision; do not rewrite clean sections) | **lead** (round-N>1 dispatch instruction; setup fix-list precedent) | P9 (targeted-revision), P13 R2 (gap→revision) |
| **Spec done-condition content** (advocate verdict `ready`; spec.md with stories + FRs) | **lead's inlined contract** (fills LD's mandated done-condition) | P9 (spec done-condition), P1 A2′/C1 |
| Briefing/recommendation *judgment* (knowledge-before-preference, gap-resolving-before-production ordering) | **lead** (referee reasoning); doctrine half → `dedupe` LD / Gap Classification | P4 b5, P9/P10 |

### B.5 — Prerequisite / handoff edges → the contract

| Responsibility | Re-homes to | Source rows |
|---|---|---|
| Constitution-must-exist (HIL INV-002 `carry_forward`) | **contract handoff edge** — lead checks `.mochiko/memory/constitution.md` present (deterministic assert; drop the INV-002 label) | P4 b13/b14/r1, P9 (constitution-prereq), P14 R9, P1 B10 |
| Enriched-input → producer | **contract handoff edge** — lead assembles enriched-input into the producer's first dispatch | P5 #7, P14 R7, P1 (handoff) |
| spec.md (producer) → advocate | **contract handoff edge** — lead wires analyst → spec.md → advocate | P3 R14, P11 R7, P1 RF5 |
| Template placement (seed `spec.md` from `spec-template`) | **lead** (kernel-free seed; excluded `create-new-feature.sh`) | P11 R6/F-2, P1 A8′ |

### B.6 — ADDED gates (rehome-orchestration Job 2 — these never existed in HIL)

| MISSING gate | Re-homes to | Source rows |
|---|---|---|
| **Default-FAIL done-condition** + inlined `workflow-contract` (spec is FAIL until advocate-`ready` AND human acceptance) | **lead** (NEW; setup precedent) | P1 C1, Agenda 2d |
| **Lead-owned verdict** (replaces analyst-autonomous gate evaluation) | **lead** (NEW posture) | P1 C2, Agenda 1b |
| **Hard round cap + no-progress + STOP kill-switch** (HIL had a soft analyst-computed cap, no kill-switch) | **lead's bounded loop** (NEW guards) | P1 A25/A26/C3, Agenda 2d |
| **Human ACCEPTANCE gate** on the validated spec — the headline missing gate (HIL auto-completed on happy path) | **lead** (NEW named gate; setup-G3 precedent) | P1 C4, Agenda 2d |
| Human-**clarification** sub-gate inside the loop (advocate clarifications → `AskUserQuestion`) — NOT the done-condition | **lead** (kept as sub-gate, setup precedent) | P1 A18, P13 R3 |

### B.7 — Dropped (kernel/DAG/catalog/MCP/brain-script mechanism) → `dropped + reason`

All kernel mechanics dissolve with the excluded kernel — **no domain capability among them** (each capability they carried is re-homed above). Full list in §E's drop table. Headline: P4 a1–a10, P1 A3/A4/A5/A6/A9/A10/A11/A15/A16/A17/A19/A20/A28[mech]/A31/A32, P1 B3/B5/B6[mech]/B7/B9, P14 R3b/R5/R11/R15, P5 6a, P12 DAG-metric, P10 R10/R11.

---

## C. NEW primitives required

| Name | Type | Why |
|---|---|---|
| **(none)** | — | The producer↔validator pair already exists (`requirements-analyst` ↔ `devils-advocate`, disjoint skills, different agents). No reviewable artifact lacks a validator: `spec.md` → advocate; the analyst/advocate **reports** are disclosure shapes (not graded deliverables); **enriched-input** is a one-shot pre-loop conditioner with its own convergence + a natural human gate (not a graded gate). **No genuine validator gap found.** The sharp contrast with setup, which had to construct `constitution-validator`. |

> One **edit** to an existing shared primitive (not a new primitive): `loop-discipline` gains the Gap Classification routing guidance (Agenda 3b). Gated.

---

## D. FINAL PRIMITIVE SET — Phase 3 artifacts (every disposition RESOLVED, no flags)

| # | Mochiko artifact | Action | Body × Structural (final) | Gate? | Notes |
|---|---|---|---|---|---|
| P1 | `commands/specify.md` | **CREATE** | **redesign × rewire-cluster** (the lead; absorb-target for §B) | **GATED** | Loop spine rebuilt to LD: default-FAIL done-condition, lead-owned verdict, produce→critique→revise with the advocate, hard round cap + no-progress + STOP, escalate-on-cap, **human acceptance gate**. Inlines a filled `workflow-contract`. Phase **content** ported `port-with-edits`. |
| P2 | `agents/requirements-analyst.md` | **CREATE** | **port-with-edits × standalone** (PRODUCER) | no | Persona keystone-clean (0 deny-list hits). `skills: authoring-requirements, authoring-user-stories` (both stay mounted, `mochiko:`). `dedupe` the "Quality Standards" literal format templates → defer to the two skills. Add `<example>` blocks (convention). |
| P3 | `agents/devils-advocate.md` | **CREATE** | **port-with-edits × standalone** (VALIDATOR) | no | Persona clean. Trim `description` to the spec slice; prune "Skills Available" to `analysis-specifications`; **stub/remove** `validation-plan-artifacts`+`validation-task-artifacts` from `skills:` (rebind-by-reference, Agenda 7a); externalize phase phrasing to `agent-dispatch.md`. Soft-dedupe "What You Hunt For" vs P6 (optional). |
| P4 | `agents/state-analyst.md` | **NOT created** | **drop × absorb-into-lead** (→ P1) | **GATED** | The central dissolution. (b) judgment set → `moved-to-lead` (§B); (a) kernel mechanics → `dropped + reason`. Highest silent-drop risk; fully traced (§E). |
| P5 | `skills/analysis-iterative/` | **CREATE** | **port-with-edits × standalone** (dual-mode kept; lead-invoked pre-step) | **GATED** (only the 6a marker drop) | Decouple HARD hits (ENRICHMENT.md L27/L57; SPECIFICATION-INPUT.md L1/L3/L112/L115/L117) → state independence by role; drop DAG parse-markers (6a, output-contract change → human gate); push `mode:`/`missing:[…]` to `agent-dispatch.md`. Keep both modes (one engine). |
| P6 | `skills/analysis-specifications/` | **CREATE** | **port-with-edits × standalone** (critic's skill) | no | Decouple L19 "Devil's Advocate" → state by role (the one HARD hit); soften phase vocab (L15/L18/L74); rebind L27 `humaninloop:`→`mochiko:`; triggers → work-context. **Stays a gap-finder; no clearing verdict added.** |
| P7 | `skills/authoring-requirements/` | **CREATE** | **port-with-edits × standalone** (keep-distinct) | no | Body mochiko-clean. Reframe `description` triggers → work-context; `humaninloop:`→`mochiko:`; rebind example spec path → `.mochiko/specs/<feature>/`. **Do not factor** shared substrate (Agenda 4b). Scrub dangling `patterns-api-contracts`/`patterns-entity-modeling` links (§G). Keep `validate-requirements.py` (Tier-1 self-check). |
| P8 | `skills/authoring-user-stories/` | **CREATE** | **port-with-edits × standalone** (keep-distinct; flag RESOLVED) | no | Resolves P8's `flag-for-reconcile` → `standalone`. Reframe triggers → work-context; **de-collide "acceptance criteria"** with P7; `humaninloop:`→`mochiko:`; scrub dangling `patterns-technical-decisions`/`patterns-api-contracts` links. Keep `validate-user-stories.py`. |
| P9 | `skills/strategy-specification/` | **NOT created** | **absorb-into-lead + dedupe** (skill dissolved) | **GATED** | Generic patterns → `dedupe` (loop-discipline); survivors (Input Assessment, targeted-revision, spec-done-condition, constitution-prereq) → `moved-to-lead`/contract (§B.4/B.5). |
| P10 | `skills/strategy-core/` | **NOT created** | **drop × dedupe-into-loop-discipline** (skill dissolved) | **GATED** | 6 patterns `dedupe` 1:1 into loop-discipline; **Gap Classification folded into loop-discipline** (additive); "may-deviate"/state-analyst framing → `dropped`. |
| P11 | `templates/spec-template.md` | **CREATE** | **port-with-edits × standalone** (deliverable) | soft (feature-numbering drop) | 8 body section-slots `kept` verbatim. Header (4 lines): `status`→loop-done-condition semantics; `feature_id` kept-but-rebind (workspace id, numbering scheme dropped); `created` lead-stamped. Lead seeds it as the analyst's starting `spec.md`. |
| P12 | `templates/analyst-report-template.md` | **CREATE** | **port-with-edits × standalone** (deliverable) | no | `{{iteration}}`→`{{round}}`; **demote** counts-first shape (lead reads Summary/Assumptions/what-changed); DAG outcome-trajectory feed → `dropped`; align to mochiko template wrapper. |
| P13 | `templates/advocate-report-template.md` | **CREATE** | **keep-verbatim × standalone** (deliverable; verdict surface) | no | Body clean (0 deny-list tokens). `{{iteration}}`→`{{round}}` (grounding rebind, **same as P12** — no body edit beyond the rename). Verdict CONTENT kept; verdict→loop-driver CONSUMPTION → `moved-to-lead` (§B.1). |
| P14 | `templates/context-template.md` | **NOT created** | **drop × absorb-into-lead** (→ P1) | **GATED** | State carrier folded into the lead (in-session + workspace-as-state). CONFIRMS + STRENGTHENS setup's memory model. |
| — | `skills/loop-discipline/SKILL.md` | **EDIT** | wiring + additive Gap Classification | **GATED** | Add the knowledge→research / preference→human-gate / scope→halt routing under Req-4/Req-3.4; keystone-tested to stay universal (no specify-flavoring). |
| — | `skills/mochiko/SKILL.md` (router) | **EDIT** | wiring-pass | no | Register `/mochiko:specify` (user-invoked hint); agents `requirements-analyst`+`devils-advocate`; skills `analysis-iterative` (general/shared), `analysis-specifications`, `authoring-requirements`, `authoring-user-stories`; templates P11/P12/P13. |

**Net Phase-3 artifacts:** 1 command (P1), 2 agents (P2, P3), 4 skills (P5, P6, P7, P8), 3 templates (P11, P12, P13), 2 edits (loop-discipline, router). **NOT produced:** P4 (absorbed), P9 (dissolved), P10 (dissolved), P14 (absorbed). **NEW primitives:** none.

---

## E. RE-EMITTED TRACES — every responsibility carries a concrete relational tag (no `flag-for-reconcile` remains)

Only the flagged/relational items are re-stated with their now-assigned tags; the full per-primitive traces in the assess files stand. Tag vocabulary: `kept` · `kept-but-rebind` · `folded-into-skill` · `moved-to-lead` · `moved-to-validator` · `moved-to-sibling-skill` · `moved-to-other-cluster` · `dedupe` · `dropped + reason`.

**P1 `specify` command** — RF1 → §B (state-analyst absorbs onto this lead); RF2 → verdict-ownership `moved-to-lead` + pair confirmed; RF3 → strategy survivors `moved-to-lead`/`dedupe` (B.4); RF4 → context-template `absorb` (B.2); RF5 → template handoff edges (B.5); RF6 → C1–C4 `moved-to-lead` (NEW, B.6); RF7 → paths `kept-but-rebind` → `.mochiko/specs/<feature>/` + `.mochiko/memory/`. All A/B/C items tagged; kernel mechanism `dropped + reason`; no domain capability lost.

**P2 `requirements-analyst`** — independence: **confirmed, no action** (producer-only; pair exists); #6 Quality Standards templates → `dedupe` (canonical home = the two skills); skill refs → `kept-but-rebind` (`mochiko:`); orchestration items originate on P1, **not** this agent. 0 deny-list hits. All tagged.

**P3 `devils-advocate`** — R7/R8 deferred validation skills → `moved-to-other-cluster` (stub/rebind-by-reference); R9 `skills:` → `kept-but-rebind` (analysis-specifications mochiko; stubs removed); R11 plan/tasks examples → `moved-to-other-cluster`; R12 phase phrasing → `moved-to-lead` (`agent-dispatch.md`); R13 loop-driving + R14 handoff → `moved-to-lead`; R15 independent-validator role → `kept`. No drops. All tagged.

**P4 `state-analyst`** — (a) a1–a10 → `dropped + reason` (kernel-free; a9 silent-recovery removal = good riddance); (b) b1 briefing, b3 report-parse, b5 recommendation, b11 dispatch, b12 answers, b14 prereq, b15 boundary → `moved-to-lead`; b2 convergence, b4 gap-classification, b6 verdict, b7 escalate, b8 cap, b9 input-assessment, b10 produce-then-validate → `moved-to-lead` **+ `dedupe`** (loop-discipline; b4 → Gap Classification folded into loop-discipline); b13 constitution-gate → `moved-to-lead` (deterministic) + `kept-but-rebind`; o1–o6 → `moved-to-other-cluster`; r1/r2 → `kept-but-rebind`, r3 → `dropped`. **The four contract-named survivors (briefing b1, recommendation b5, convergence b2, report-parse b3) all `moved-to-lead` — none dropped.** All tagged.

**P5 `analysis-iterative`** — #1/#3/#10 core → `kept` (decoupled); #2 standard mode → `kept` (dual-mode retained, Agenda 5b); #4 enriched-input artifact → `kept-but-rebind` (lead reads directly); #5 detect-sparseness → `moved-to-lead` (Input Assessment triage); **6a DAG parse-markers → `dropped + reason`** (output-contract change → human gate); 6b sequencing → `moved-to-lead`; #7 handoff → `kept-but-rebind` (contract edge); #8 mode-dispatch → `kept-but-rebind` (`agent-dispatch.md`); #9 sibling refs + #11 triggers → `kept-but-rebind`. All tagged.

**P6 `analysis-specifications`** — gap-finding/format/taxonomy/process/checklist/mistakes → `kept`; severity buckets + structured output → `kept` (**INPUT to the lead's verdict, no clearing PASS/FAIL added**); L19 "Devil's Advocate" → `kept-but-rebind` (HARD decouple → role); L27 namespace → `kept-but-rebind` (`mochiko:`); phase vocab L15/L18/L74 → `kept-but-rebind` (soften); triggers → `kept-but-rebind` (work-context). All tagged. No drops.

**P7 `authoring-requirements`** — FR/edge/SC/entities/RFC-2119/checklists → `kept`; tech-agnostic discipline → `kept` (**dedupe candidate RESOLVED: keep-distinct, do NOT factor**, Agenda 4b); `validate-requirements.py` → `kept` (Tier-1); triggers → `kept-but-rebind` (work-context); sibling cross-refs → `kept-but-rebind` (ported → `mochiko:`; **deferred `patterns-api-contracts`/`patterns-entity-modeling` → scrub-with-reason**, §G); example path → `kept-but-rebind`; classification + router → `kept-but-rebind` (wiring). All tagged. No drops (the two dangling links carry a reason).

**P8 `authoring-user-stories`** — structural flag **RESOLVED → `standalone`** (keep-distinct, Agenda 4a); priority taxonomy/G-W-T/independent-test/examples/anti-rationalization → `kept`; tech-agnostic discipline → `kept` (**not factored**); `validate-user-stories.py` → `kept` (Tier-1); triggers → `kept-but-rebind` (**de-collide "acceptance criteria"** vs P7); sibling cross-refs → `kept-but-rebind` (deferred `patterns-technical-decisions`/`patterns-api-contracts` scrubbed-with-reason); router → `kept-but-rebind`. "future phase" false positive → `kept`. All tagged. No silent drops.

**P9 `strategy-specification`** — structural flag **RESOLVED → absorb-into-lead + dedupe** (skill not created): Input Assessment + targeted-revision + spec-done-condition → `moved-to-lead`/contract (B.4); constitution-prereq → `kept-but-rebind` (B.5 handoff edge); produce-then-validate + "never skip gate" + recurring-gap + 5-pass + iterate-on-FAIL → `dedupe` (loop-discipline); "Research Before User"/gap-typing → `dedupe` (→ Gap Classification in loop-discipline); state-analyst-briefing + DAG vocab → `dropped + reason`. All tagged.

**P10 `strategy-core`** — structural flag **RESOLVED → drop × dedupe-into-loop-discipline** (skill not created): R1–R6 → `dedupe` (loop-discipline Req 2/3/4); R7 re-enrichment → `dedupe` (→ lead, with Input Assessment); **R8 Gap Classification + R9 preference-to-research → folded into `loop-discipline`** (additive, universal); R10 may-deviate stance + R11 state-analyst role → `dropped + reason` (superseded / kernel-era). All tagged.

**P11 `spec-template`** — R1/R1a–d body slots + R2 feature_title + R7 done-condition surface → `kept`; **R3 feature_id → `kept-but-rebind`** (lead-stamped workspace id; numbering scheme dropped — **soft gate**); R4 created → `kept-but-rebind` (lead-stamped); R5 status → `kept-but-rebind` (→ loop done-condition semantics); R6 placement → `moved-to-lead` (lead seeds spec.md); R8 discoverability → `kept-but-rebind` (wiring). F-1 → variant (b) soften; F-2 → confirmed lead-seeds. All tagged.

**P12 `analyst-report-template`** — structure/assumptions/summary/notes → `kept`; `{{iteration}}` → `kept-but-rebind` (**→ `{{round}}`, identical to P13**); counts table → `kept-but-rebind` (**demoted** — lead reads prose directly); **DAG outcome-trajectory metric feed → `dropped + reason`** (kernel); heading-parse by state-analyst → `moved-to-lead` (lead reads directly); "Report format: follow <template>" → `kept-but-rebind` (path) + dispatch ref `moved-to-lead`. Edit-depth flag **RESOLVED** (demote, Agenda 6d). All tagged.

**P13 `advocate-report-template`** — R1a feature_id + R1b iteration → `kept-but-rebind` (**`{{iteration}}`→`{{round}}`, same as P12**; grounding to lead — no body edit); R1c timestamp/R2 gaps/R3 clarifications/R4 verdict/R6 strengths/R7 mustache → `kept` (verdict CONTENT survives — load-bearing); **R5 verdict→loop-driver consumption + R2 gap→revision + R3 clarification→human-gate → `moved-to-lead`** (B.1/B.6; F-P13-1 lands); R8 discoverability + R9 cross-cluster → `kept-but-rebind`/`kept`. Disposition stays `keep-verbatim × standalone`. All tagged.

**P14 `context-template`** — R1 status/R2 iteration(bounded)/R3a identity/R4 input-source/R6 user-input/R7 enrichment/R8 project-context/R9 constitution/R10 file-paths/R12 dispatch/R13 clarification-log/R14 cross-context-handoff → `moved-to-lead` (B.2/B.3); **R3b DAG numbering + create-new-feature.sh, R5 timestamps + type marker, R11 self-ref context_path, R15 ephemeral lifecycle → `dropped + reason`**; R16 shared schema → `moved-to-other-cluster` (peers inherit). All tagged.

### Dropped responsibilities (explicit — these go to the human gate)

| Dropped responsibility | Reason (for lead acceptance) | Capability lost? |
|---|---|---|
| P4 a1–a10 — `hil-dag` MCP, node assembly, pass-freezing, status updates, catalog resolution, DAG-as-state, recording protocol, graph invariants, the agent shell + 4-action protocol | Kernel-free: MCP/DAG/catalog excluded; mochiko has no separate orchestration agent (the lead owns the loop). | **No** — every judgment capability re-homed (§B). |
| P4 a9 — **silent `carry_forward` auto-resolution** ("Supervisor never informed") | Kernel-free **and** mochiko forbids silent recovery — a behavior change for the better. | No (the prerequisite check survives as an explicit lead handoff edge, B.5). |
| P1 A30 — **"NEVER read domain agent reports directly"** (context-isolation rule) — **a judgment-call reversal** | The state-analyst middle-man that required isolation is gone; the lead MUST read `spec.md` + the advocate report to own the verdict (LD-2). Deliberate reversal. | No (it is a posture change that *enables* the verdict-ownership fix). |
| P1 A3/A4/A5/A6/A9/A10/A11/A15/A16/A17/A19/A20/A28[mech]/A29′/A31/A32; B3/B5/B6[mech]/B7/B9/B10[mech] | Kernel/DAG/catalog/MCP/brain-script plumbing (DAG vocab, catalog path, ask-analyst verb, zero-CLI rule, MCP install, script JSON parse, dag dir, brief-assemble/parse-advance/update-advance, dispatch_mode, node-menu/recommendation, freeze, status writes, graph invariants, INV-002 mechanism). | **No** — each capability re-homed (A8′/A12′/A17′/A18′/B-series/C-series). |
| P5 6a — DAG parse-markers (`ENRICHMENT_COMPLETE`/`_OUTPUT_END`) | Kernel-free output-contract change: the lead reads the enrichment artifact directly; markers existed only so the brain could machine-lift output. | No (the enriched-input artifact survives). |
| P11 R3 — **feature-numbering scheme** (`create-new-feature.sh` BRANCH_NAME provenance) | Workspace-as-state supersedes feature-numbering; `feature_id` rebinds to a lead-derived workspace identifier rather than a branch number. | No (workspace identity survives; only the numbering kernel drops). |
| P12 — DAG **outcome-trajectory metric** feed (count-per-pass trend) | Kernel-free: the DAG metric machinery is excluded; the lead reads the report + spec.md directly. | No (the report content survives; the counts are demoted, not deleted). |
| P14 R3b — DAG numbering + `create-new-feature.sh` scaffolding | Excluded brain script; the lead derives the workspace dir. | No. |
| P14 R5 — `created`/`updated` timestamps + `type` node-kind marker | DAG state-file bookkeeping; workspace mtime/git history covers any need; node-typing has no kernel-free meaning. | No. |
| P14 R11 — self-referential `context_path` | No separate context file exists to point at under absorb. | No. |
| P14 R15 — ephemeral create/update/delete lifecycle | In-session + workspace state needs no ephemeral file to manage. | No. |
| P10 R10 — "advisory, not prescriptive / may deviate" stance | Superseded by `loop-discipline`'s non-negotiable framing ("you cannot rationalize your way out of any of the four"); carrying it would contradict the doctrine. | No (the patterns survive, hardened, via dedupe). |
| P10 R11 / P9 framing — "consumed by State Analyst → inform Supervisor briefings" role | The state-analyst consumer dissolves; the lead consumes `loop-discipline` directly. | No. |

**No other responsibility is dropped.** Every non-dropped responsibility has a concrete landing (`kept` / `kept-but-rebind` / `moved-to-lead` / `moved-to-sibling-skill` [n/a — no merge this cluster] / `dedupe` / `moved-to-other-cluster` / `folded-into` loop-discipline). **No `moved-to-validator`** in this cluster (the validator already exists; nothing is promoted onto it).

---

## F. HUMAN-GATED dispositions (contract §4a: redesign / absorb-into-lead / promote / dropped)

Present these at the Phase-2 human gate before Phase 3 applies them. The supervisor accepts / overrides / sends-back each.

1. **P1 `redesign` (rewire-cluster).** Rebuild the `specify` loop spine to `loop-discipline`: default-FAIL done-condition, lead-owned verdict, produce→critique→revise with hard round cap + no-progress + STOP, escalate-on-cap, and the **NEW human acceptance gate** the original lacked. *Accept / override / send-back.*
2. **P4 `state-analyst` `drop × absorb-into-lead`** — the central dissolution. Accept that (b) judgment re-homes to the lead (§B) and (a) kernel mechanics drop. Notable: the **A30 reversal** ("never read reports" → lead reads directly) and the **a9 silent-recovery removal**. *Accept the absorb + these two judgment-level drops.*
3. **P9 + P10 strategy `dedupe` / dissolution** (contract names this explicitly) — neither strategy skill is created; generic patterns dedupe into `loop-discipline`; **Gap Classification is folded into `loop-discipline`** (additive edit to a shared governance skill). *Accept the dissolution + the loop-discipline edit; alternative offered: keep Gap Classification as specify-lead prose if you prefer loop-discipline frozen.*
4. **P14 `context-template` `drop × absorb-into-lead`** — one state mechanism (in-session + `.mochiko/specs/<feature>/` + `.mochiko/memory/`). *Accept the absorb + the R3b/R5/R11/R15 drops.*
5. **P5 `analysis-iterative` 6a drop** — DAG parse-markers removed (output-contract change; lead reads the enrichment artifact directly). *Accept the output-contract change.*
6. **All dropped-responsibility reasons** (the §E table) — kernel/DAG/catalog/MCP/brain-script plumbing + the named judgment-level drops. *Accept or reject each reason.*

**Soft / scope items (surfaced alongside, not a §4 contract gate):**
7. **P11 `feature_id` rebind** — feature-numbering scheme dropped; `feature_id` rebinds to a lead-derived workspace identifier (header treatment variant b). *Accept the identity-model call (run goal #2).*
8. **Run-goal structural confirmations** (record, for ROADMAP promotion at the lead's discretion): specify **confirms setup** on both empirical calls — **human-gate placement** (gated dispositions + escalations + a NEW acceptance gate) and the **memory model** (in-session + `.mochiko/memory/` + workspace-as-state; context-handoff absorbed). P14 is a *stronger* confirmation than setup (no drift; more state, still dissolves). **Eligible to promote to ROADMAP Key Decisions + close BACKLOG OQ#3.** The decoupling doctrine **held empirically** (agent/skill bodies clean by absence; deny-list hits concentrated in the dissolving orchestration; the lone canonical case = `analysis-specifications` L19).

**Not gated by §4 (applied without explicit accept):**
- P2/P3 `port-with-edits` (persona-clean ports; the advocate's deferred-skill stub).
- P6/P7/P8 `port-with-edits` decouple/dedupe/de-collide (wiring).
- P12/P13 `port-with-edits`/`keep-verbatim` (iteration→round; counts demote).
- P7/P8 **keep-distinct** (NOT in the §4 enumerated set — a non-merge is not a gated structural move).
- Router edit; namespace rebinds; dangling-ref scrubs.

---

## G. CONVENTION-WIRING PREVIEW (for `transform-recipes`)

| Concern | Primitives + action |
|---|---|
| **Classification** | P1 → **user-invoked** (`disable-model-invocation: true`, slash command). P5/P6/P7/P8 → **model-invoked** (agent-consumed; work-context triggers, not "when the user says"). P2/P3 → agents (`skills:` lists; not router-classified as user/model). P11/P12/P13 → inert templates (no classification). P4/P9/P10/P14 → dissolved (no classification). |
| **Router registration** (`skills/mochiko/SKILL.md`) | Add: `/mochiko:specify` (user-invoked hint); agents `requirements-analyst`, `devils-advocate`; skills `analysis-iterative` (mark **general/shared**), `analysis-specifications`, `authoring-requirements`, `authoring-user-stories`; templates `spec-template`, `analyst-report-template`, `advocate-report-template`. `loop-discipline` already registered (gains Gap Classification). |
| **Namespace rebinds** (`humaninloop:`→`mochiko:`) | P2 body skill refs; P3 `skills:` (analysis-specifications); P5 cross-ref to analysis-specifications; P6 L27 cross-ref; P7 cross-ref to authoring-user-stories; P8 cross-refs. |
| **Path rebinds** | All `specs/{feature-id}/.workflow/` → **`.mochiko/specs/<feature>/`**; `.humaninloop/memory/constitution.md` → **`.mochiko/memory/constitution.md`**; catalog/DAG/MCP paths → dropped. Kill-switch sentinel → `.mochiko/specs/<feature>/SPECIFY_STOP`. **Convention name: mochiko workspace-as-state** — durable cross-feature governance in `.mochiko/memory/`; per-feature spec-loop state in `.mochiko/specs/<feature>/`; no separate context-handoff file. |
| **Trigger de-collision** | P7 vs P8 "acceptance criteria" (Agenda 4c); P5 vs P6 enrichment-vs-gap-review (Agenda 5c). |
| **Decouple (deny-list) scrubs** | **P6 L19** "Devil's Advocate" → role (the one canonical HARD hit). **P5** HARD hits (ENRICHMENT.md L27/L57; SPECIFICATION-INPUT.md L1/L3/L112/L115/L117) → role; push `mode:`/`missing:[…]` to `agent-dispatch.md`. P9/P10 "State Analyst"/"Supervisor"/"universal" → **moot** (skills dissolve). P2/P3/P7/P8/P11/P12/P13 → already clean (0 deny-list hits). `verify-output`'s decoupling grep + keystone test must confirm zero remain. |
| **Deferred / dangling cross-refs to scrub** | **P3** `validation-plan-artifacts` + `validation-task-artifacts` → remove from live `skills:` (stub; re-mount when plan/tasks port). **P7** `patterns-api-contracts` + `patterns-entity-modeling` (design-track deferred) → genericize/drop the dead links with reason. **P8** `patterns-technical-decisions` + `patterns-api-contracts` → same. **P5** `humaninloop:` sibling-skill prefixes → rebind/genericize. |
| **Template wrapper convention** | P12 align to the mochiko template shape (`# Title` + intro line + ```markdown fence + `## Usage Notes`); P13 optional fence on a verbatim body; P11 keep bare section slots. |
| **Agent `<example>` blocks** | P2/P3 add `<example>` blocks (mochiko agent-description convention; absent in HIL). |
| **REGISTRY re-file** | On finalize, re-file `authoring-requirements` + `authoring-user-stories` **plan → specify** (context.md: REGISTRY mis-files them under plan). Flip P5/P6/P7/P8/P11/P12/P13 to `[x]`; record the trace as the migration record. |

---

## H. Open flags

**NONE.** Every `flag-for-reconcile` is resolved to a concrete move in §A and homed in §B:

- P1 RF1–RF7 → §A Agenda 2/1/3 + §B (the lead absorbs the dissolving orchestration + ADDS the four gates).
- P2 (independence-confirm · dedupe · rehome-noted), P3 (pairing-record · deferred-stub) → Agenda 1/7a.
- P4 (rehome-placement · report-parse-boundary · independence-guard · strategy-coordination) → Agenda 1c/2a/2b + §B.
- P5 F1/F2/F3 → Agenda 5a/5b/5c.
- P6 (confirm-pairing · verdict-ownership · de-collision · cross-cluster) → Agenda 1a/1d/5c/7e.
- P7 + P8 (sibling overlap, distinct-vs-factor, de-collide) → Agenda 4.
- P9 + P10 (3-way dedupe · gap-classification home · cross-workflow · joint resolution) → Agenda 3.
- P11 F-1/F-2 → Agenda 6a/6b.
- P12 (edit-depth · header consistency) → Agenda 6c/6d.
- P13 F-P13-1 → Agenda 6e + §B.1.
- P14 F-CTX-1/2/3 → Agenda 2c + §B.2/B.3 + §F.

Independence holds (producer ≠ validator, disjoint skills; the absorb lands referee-only judgment on the lead). Kernel-free maintained (every rehome lands on lead / existing agent-skill / loop-discipline / contract / accepted-drop — no Python/MCP/DAG/catalog). **No new partner primitive required.** Reconcile done-condition met: **zero open flags, zero homeless responsibilities** — every responsibility of the four dissolving primitives (P4/P9/P10/P14) lands **exactly once** on the single coherent lead rehome map (§B).

---

**Reconcile version:** v1 · **Governed by:** `loop-discipline` · **Next:** Phase 2 human gate (§F) → Phase 3 `transform-recipes` (apply §D + §B + §G).
