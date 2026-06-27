# Transform — `analysis-iterative` (P5)

**Run:** transform-cluster / `specify` · Phase 3 (transform) · **Transformed:** 2026-06-27 · **Producer:** `mochiko:transform-producer`
**Skill used:** `mochiko:transform-recipes` (port-with-edits × standalone) · APPLY ONLY — independent grading is `verify-output` (a different agent).
**Source:** `human-in-loop/plugins/humaninloop/skills/analysis-iterative/` — **whole 5-file bundle** (directory listed first): `SKILL.md`, `SPECIFICATION-INPUT.md`, `ENRICHMENT.md`, `SYNTHESIS.md`, `references/ADAPTIVE-EXAMPLES.md`. Progressive-disclosure structure preserved.
**Inputs:** `reconcile.md` §D row P5 + §E P5 trace + §G wiring (HARD decouple hits) + Agenda 5a/5b/5c; `assess-analysis-iterative.md`; sibling-port shape `plugins/mochiko/skills/analysis-codebase/SKILL.md`; the P6 companion trace `transform-analysis-specifications.md`.
**Human-gate status:** the **6a parse-marker drop ACCEPTED** at the Phase-2 gate (reconcile §F item 5). All other edits are wiring (not §4-gated).

---

```
TRANSFORM: analysis-iterative
Applied:   port-with-edits × standalone + wiring-pass
Artifacts: plugins/mochiko/skills/analysis-iterative/SKILL.md                 (CREATE, edited)
           plugins/mochiko/skills/analysis-iterative/SPECIFICATION-INPUT.md   (CREATE, edited — HARD decouple)
           plugins/mochiko/skills/analysis-iterative/ENRICHMENT.md            (CREATE, edited — HARD decouple + 6a marker DROP)
           plugins/mochiko/skills/analysis-iterative/SYNTHESIS.md             (CREATE, verbatim — clean)
           plugins/mochiko/skills/analysis-iterative/references/ADAPTIVE-EXAMPLES.md (CREATE, verbatim — clean)
New partners: none (NOT a producer↔validator artifact — one-shot pre-loop input conditioner; no split/merge/promote)
Wiring:    classification=model-invoked (no disable-model-invocation; skill → no skills: list)
           router=NOT registered here (separate §D line: skills/mochiko/SKILL.md EDIT — out of scope; "DO NOT edit the shared router"). Description carries the "general / shared" note the router will surface.
           triggers=work-context (enrich / brainstorm / think-through / sparse description / Who-Problem-Value); de-collided from analysis-specifications (pre-spec enrichment vs post-draft gap-review)
           rebinds=[SKILL.md L25 humaninloop:analysis-specifications → mochiko:analysis-specifications;
                    SKILL.md L17 humaninloop:specify → role/work-context;
                    ENRICHMENT.md L27/L57 + SPECIFICATION-INPUT.md L3/L7/L112/L115/L117 → role;
                    SPECIFICATION-INPUT.md L9-16 mode:/missing:[…] invocation → caller-side dispatch (agent-dispatch convention)]
           drops=[ENRICHMENT.md DAG parse-markers ENRICHMENT_COMPLETE / ENRICHMENT_OUTPUT_END — kernel-free; human-gate ACCEPTED]
Trace (realized): every responsibility tagged in §D; one drop (6a, gated+accepted); no silent loss.
Dual-mode: RETAINED in one skill (two output shapes, one engine — Agenda 5b). NOT split.
```

---

## A. Body edits applied (port-with-edits — localized; structure + voice preserved)

### A.1 — HARD decouple hits (independence stated by ROLE, not agent/command names)

| # | Where | Before (HIL) | After (mochiko) | Tag / reason |
|---|-------|--------------|-----------------|--------------|
| 1 | **ENRICHMENT.md L27 + L57** (footer, template + example) | `**Enrichment complete.** The \`/humaninloop:specify\` supervisor should now continue to **Phase 1** using the Summary section above as the enriched input.` | Footer **removed** (it was the marker-wrapped, supervisor-continuation signal). Carry-forward guidance survives in Guidelines #6, by role: "what gets carried forward as the enriched input to whoever authors the specification next (the requirements producer)." | `kept-but-rebind` (handoff #7) + sequencing `moved-to-lead` (#6b). Sibling command name + "Phase 1" + "supervisor" all gone. Keystone-survives. |
| 2 | **SPECIFICATION-INPUT.md L3** | `Use this mode when invoked by \`humaninloop:specify\` to enrich sparse feature descriptions.` | `A focused variant of the iterative-analysis engine for conditioning a sparse feature description into a specification-ready brief. Use it when a feature description is thin on the **Who + Problem + Value** triad…` | `kept-but-rebind` — command name + "invoked by" removed; stated by **work context** (true of this analyst on any job). |
| 3 | **SPECIFICATION-INPUT.md L7** (caller-detection baked into skill) | `The specify command detects when user input lacks the **Who + Problem + Value** triad and invokes this skill to fill gaps before proceeding to the Requirements Analyst.` | `…a raw feature description is missing some of the Who / Problem / Value triad, and it needs conditioning before it reaches whoever authors the specification (the requirements producer). Whether a description is sparse enough to enrich, and which triad elements are missing, is decided by the caller before this skill runs…` | `moved-to-lead` (#5 detect-sparseness) — the *who-decides-to-enrich* logic leaves the skill; sibling-agent name → role. |
| 4 | **SPECIFICATION-INPUT.md L112/L115/L117** (Completion and Return — "supervisor" ×3) | `…the footer text indicating the **supervisor** should continue` / `markers allow the **supervisor** to parse the output` / `The **supervisor (specify command)** owns the workflow continuation.` | New `## Completion`: "Conclude — do NOT ask any follow-up questions; the enrichment is complete. The enriched description is the deliverable. Whoever requested the enrichment reads the Summary directly and carries it forward… This skill does not decide what runs next — it finishes by producing the artifact." | sequencing `moved-to-lead` (#6b); markers `dropped` (#6a). Three "supervisor" hits + the marker-parse mechanic all removed. |
| 5 | **SPECIFICATION-INPUT.md L9-16** (Invocation block — caller-side mechanism) | A `Skill(skill:"analysis-iterative", args:"mode:specification-input missing:[who,problem,value] original:\"…\"")` invocation block. | **Removed from the skill body.** Replaced by a "When this variant fits" note: "(How a caller signals 'enrich this' and passes the known-missing elements is a caller-side dispatch concern — see the agent-dispatch briefing — not something this skill parses.)" | `kept-but-rebind` (#8 mode-dispatch) → caller-side dispatch convention. See **Deviation D3**. |

### A.2 — SOFT hits + classification + de-collision

| # | Where | Before (HIL) | After (mochiko) | Tag / reason |
|---|-------|--------------|-----------------|--------------|
| 6 | **SKILL.md L3 description / triggers** | `…MUST be invoked when the user says "brainstorm", "deep analysis", "let's think through", "analyze this with me", or "help me think through". SHOULD also invoke when feature descriptions lack Who/Problem/Value clarity during specification enrichment.` | Work-context description (see §C): MUST = enrich sparse Who/Problem/Value before a spec is authored, OR run an adaptive think-through / brainstorm ending in a synthesis; SHOULD = condition a thin description before a requirements producer, or work a complex trade-off space; **"general / shared… not tied to a single workflow"**; de-collision clause vs `analysis-specifications`. | `kept-but-rebind` (#11) — agent/lead-consumed ⇒ describe **work context**, not "when the user says." Graded MUST/SHOULD retained. |
| 7 | **SKILL.md L17** When-to-Use | `- Enriching sparse feature descriptions for \`humaninloop:specify\`` | `- Enriching sparse feature descriptions before specification authoring` | `kept-but-rebind` (#9) — command name → work context. |
| 8 | **SKILL.md L25** When-NOT-to-Use | `- **Specification review** — use \`humaninloop:analysis-specifications\` instead` | `- **Specification review** — reviewing an already-drafted spec for gaps is post-draft work; use \`mochiko:analysis-specifications\` instead (this skill conditions raw input *before* a spec exists)` | `kept-but-rebind` (#9) — `humaninloop:` → `mochiko:`; de-collision boundary made explicit (Agenda 5c). |
| 9 | **SKILL.md L200-206** Modes section | `## Modes … When invoked with \`mode:specification-input\`, run a focused variant…` | `## Two output shapes, one engine` — states the questioning engine is identical; "General analysis (default)" → SYNTHESIS.md; "Specification-input enrichment" → SPECIFICATION-INPUT.md + ENRICHMENT.md; mode/missing signalling is a caller-side concern (pointer, not baked in). | `kept` (#2 dual-mode) + `kept-but-rebind` (#8) — **dual-mode retained in one skill**; `mode:` arg framing removed. |
| 10 | **SPECIFICATION-INPUT.md L22 / L69** internal agenda labels | `### Phase 1: Fill Triad Gaps` / `### Phase 2: Key Decisions` | `### Part 1: Fill triad gaps` / `### Part 2: Key Decisions` | `kept` — internal two-part agenda renamed off "Phase N" to avoid a decoupling-scan false positive. Semantics identical. See **Deviation D1**. |
| 11 | **SPECIFICATION-INPUT.md L24** | `Only ask questions for missing elements (parsed from \`missing:[]\` arg).` | `Only ask about triad elements that are actually missing — those the caller named, or those the original input leaves unclear.` | `kept-but-rebind` (#8) — `missing:[]` arg mechanic → work context. |
| 12 | **SPECIFICATION-INPUT.md L126** Key-Differences table | `\| Purpose \| … \| Enrich feature descriptions for /specify \|` | `\| Purpose \| Explore any topic \| Condition a sparse feature description before specification \|` + table renamed "Standard Mode" → "General Analysis" | `kept-but-rebind` (#9) — `/specify` → work context. |
| 13 | **ENRICHMENT.md L73** Guidelines #6 (sibling-agent name) | `**Summary**: This is what gets passed to the Requirements Analyst.` | `**Summary**: This is what gets carried forward as the enriched input to whoever authors the specification next (the requirements producer).` | `kept-but-rebind` (#7) — sibling-agent name → role. See **Deviation D2**. |

### A.3 — Dropped (the one gated item)

| # | Where | Dropped | Reason | Capability lost? |
|---|-------|---------|--------|------------------|
| 6a | **ENRICHMENT.md** template + example blocks | `<!-- ENRICHMENT_COMPLETE -->` and `<!-- ENRICHMENT_OUTPUT_END -->` (L8/L28 template, L38/L58 example) | **kernel-free** — the markers existed only so the brain could machine-lift the enriched output. In mochiko the lead/requester reads the enrichment artifact directly (output-contract change). **ACCEPTED at the Phase-2 human gate** (reconcile §F #5, §E drop table). | **No** — the enriched-input artifact (Actor/Problem/Value/Out-of-scope/Success + Summary + preserved Original Input) survives intact; only the machine-parse hooks are gone. |

**Kept verbatim (body is mochiko-clean — no kernel/DAG/catalog/MCP/`.humaninloop` content):**
- `SYNTHESIS.md` — whole file (section-inclusion guide, confidence indicators, synthesis template, guidelines). 0 coupling tokens.
- `references/ADAPTIVE-EXAMPLES.md` — whole file (3 annotated conversations). 0 coupling tokens (only legitimate craft use of "Brainstorm").
- `SKILL.md` craft core — Overview, Adaptive Flow, Discovery, Question Format Adaptation, Reading Confidence Signals, Smart Wrap-up, Output, Common Mistakes. (Edited only at L3/L17/L25/Modes.)
- `SPECIFICATION-INPUT.md` the five questions verbatim — WHO / PROBLEM / VALUE (conditional) + SCOPE / SUCCESS (always). Pure craft; untouched.

---

## B. Convention-wiring pass (all five — always runs)

| # | Convention | Action this primitive | Status |
|---|-----------|------------------------|--------|
| 1 | **Classification tag** | model-invoked → **default** (no `disable-model-invocation`). Skill (not agent) ⇒ **no `skills:` list**. Agent/lead-consumed. | done |
| 2 | **Router registration** | **Deferred to its own §D artifact line** (`skills/mochiko/SKILL.md` EDIT). Brief: "DO NOT edit the shared router." The description carries the **"general / shared (cross-cluster)"** note the router will surface (reconcile §G: mark general/shared). `verify-output` Part-A confirms registration once the router EDIT lands. | deferred (scope) |
| 3 | **Trigger phrasing** | model-invoked + agent/lead-consumed ⇒ work-context, not "when the user says." Phrases: enrich / brainstorm / think-through / sparse description / Who-Problem-Value. De-collided from `analysis-specifications` (§C). | done |
| 4 | **Path rebinding** | No `.humaninloop/` paths, no catalog/MCP/DAG paths anywhere in the bundle (assess Check 6: paths clean). Only namespace/role rebinds (A.1/A.2) + the caller-side dispatch rebind (#8). | done |
| 5 | **Decouple persona/skill** | All HARD hits → role (A.1 #1-5); SOFT hits → work context (A.2 #6-13). Keystone-tested the rest → no `dispatch`, no `workflow-agnostic`/independence-by-declaration meta-label, no sibling-agent name, no injected workflow modes/paths/phases, no DAG/catalog/brain/`state-analyst`. **Grep across the new dir: zero residual deny-list tokens.** | done |

---

## C. Triggers — de-collided from `analysis-specifications` (Agenda 5c / §G)

- **This skill (P5) = pre-spec input enrichment.** Anchor: "conditions raw input BEFORE a spec exists," "a sparse feature description… before a specification is authored," Who/Problem/Value triad. Trigger phrases: `enrich` · `brainstorm` · `think-through` · `sparse / thin description` · `Who-Problem-Value clarity`.
- **`analysis-specifications` (P6) = post-draft gap-review** (reviews an already-drafted spec for what's missing). Explicitly routed away in BOTH the description ("Distinct from analysis-specifications, which reviews an already-drafted spec for gaps") AND a When-NOT-to-Use bullet ("reviewing an already-drafted spec for gaps is post-draft work; use `mochiko:analysis-specifications` instead").
- **Lifecycle marker is the de-collision lever:** *no draft yet, enriching input* (P5) vs *a draft exists and is being reviewed* (P6). Disjoint stages ⇒ disjoint triggers. The HIL "specification" overlap (P5 F3 / P6 flag-3) resolves by anchoring P5 on "before a spec exists" — symmetric with P6's "already-drafted" anchor (see `transform-analysis-specifications.md` §C).

---

## D. Realized responsibility trace (every responsibility carries a final tag — no silent loss)

Source: assess `analysis-iterative` Step-5 trace + reconcile §E P5. Mapping is 1:1 with the assess numbering.

| # | Responsibility | Final tag | Realized where |
|---|----------------|-----------|----------------|
| 1 | Adaptive iterative-questioning procedure (opening/discovery/adaptive Q/wrap-up; format adaptation; confidence-signal reading) | `kept` | SKILL.md craft core (verbatim except L3/L17/L25/Modes) |
| 2 | Standard-mode synthesis artifact (SYNTHESIS, confidence indicators, scaled output) | `kept` | SYNTHESIS.md verbatim + SKILL.md "General analysis (default)". **Dual-mode RETAINED** (Agenda 5b) — see §E |
| 3 | Spec-input enrichment procedure (two-part Who/Problem/Value + Scope + Success agenda) | `kept` (decoupled) | SPECIFICATION-INPUT.md — five questions verbatim; wrapper decoupled (A.1) |
| 4 | Enriched-input artifact (Actor/Problem/Value/Out-of-scope/Success + Summary + preserved Original Input) | `kept-but-rebind` | ENRICHMENT.md — template kept; markers dropped (6a); reader = whoever requested it, reads directly |
| 5 | Detect "input lacks Who/Problem/Value" → decide to enrich (HIL: command computes `missing:[…]`) | `moved-to-lead` | Removed caller-detection (A.1 #3); the enrich-decision + which-missing is now caller-side (lead's Input Assessment, reconcile §B.4); skill infers if not told |
| 6a | DAG parse-markers (`ENRICHMENT_COMPLETE`/`_OUTPUT_END`) as machine-parse hooks | **`dropped + reason: kernel-free`** | ENRICHMENT.md template + example (A.3). **Human-gate ACCEPTED.** No capability lost |
| 6b | Workflow-continuation sequencing ("continue to Phase 1") | `moved-to-lead` | Removed the supervisor-continuation footer (A.1 #1/#4); "what runs next" belongs to the lead (P1), not the skill |
| 7 | Handoff enriched-input → requirements producer | `kept-but-rebind` | Restated by role ("hand to whoever authors the specification / the requirements producer"); the actual handoff edge is the lead/contract's (reconcile §B.5) |
| 8 | Mode-dispatch contract (`mode:specification-input missing:[…] original:…`) | `kept-but-rebind` | Invocation block removed from body (A.1 #5); rebound to caller-side dispatch convention; dual-mode mechanism kept as a work-context branch in one skill. See **D3** |
| 9 | Sibling cross-refs (When-NOT "use analysis-specifications"; When-to-Use "for humaninloop:specify") | `kept-but-rebind` | `humaninloop:` dropped; `analysis-specifications` → `mochiko:` + role (A.2 #7/#8); router owns discoverability |
| 10 | Progressive-disclosure reference bundle (ADAPTIVE-EXAMPLES.md — 3 annotated conversations) | `kept` | references/ADAPTIVE-EXAMPLES.md verbatim |
| 11 | Trigger phrases in `description` (graded RFC-2119) | `kept-but-rebind` | Work-context phrasing; de-collided from P6; model-invoked (A.2 #6) |

**Reconcile flags (from assess) — disposition status:**
- **F1 mount-point + orchestration rehome** (Agenda 5a) — **HONORED:** enrichment *procedure* stays in the skill; *when-to-enrich* + *which-missing* + *sequencing* are caller-side (`moved-to-lead`, #5/#6b). The skill names no caller and decides no invocation. Independence-safe (only mounting on the validator is forbidden; this skill is a pre-loop conditioner mounted nowhere here).
- **F2 dual-mode scope** (Agenda 5b) — **RESOLVED → kept whole.** Both modes in one skill over one engine; noted general/shared for the router. See §E.
- **F3 trigger de-collision** (Agenda 5c) — **DONE:** see §C.

---

## E. Dual-mode retained — one engine, two output shapes (Agenda 5b, the critical "do-not-split" call)

**Both modes live in ONE skill.** The new `## Two output shapes, one engine` section in SKILL.md states it directly: the adaptive questioning engine (Adaptive Flow → Discovery → Question Format Adaptation → Confidence Signals → Smart Wrap-up) is **identical** for both; only the agenda and the concluding artifact differ:
- **General analysis (default):** open exploration / brainstorm / trade-off think-through → concludes with `SYNTHESIS.md`.
- **Specification-input enrichment:** condition a sparse feature description (thin Who/Problem/Value) → focused two-part agenda in `SPECIFICATION-INPUT.md` → concludes with the `ENRICHMENT.md` artifact.

**Why not split (reconcile §A 5b vs the setup precedent):** setup split `analysis-codebase` because its modes were *separable procedures with distinct consumers and a JSON schema*. Here the two modes are one adaptive-questioning engine with two output templates — splitting would duplicate the engine or manufacture a shared-ref coupling. Minimalism governor ⇒ keep whole. The general brainstorm mode rides along as an inherent capability (it is **not** moved out — assess #2 `kept`, reconcile §7b), and is flagged **general / shared (cross-cluster)** in the description so the router can surface it as a shared utility (the router EDIT itself is a separate §D line, not touched here).

---

## F. Deviations / notes for the independent verifier

- **D1 — `Phase 1`/`Phase 2` agenda labels → `Part 1`/`Part 2`:** these were the skill's OWN two-part questioning agenda (triad-gaps then key-decisions), never workflow phases. Renamed defensively so the decoupling scan sees no "Phase N" token. Zero semantic change. (One lowercase "phase" remains at SKILL.md — "how long each phase lasts" — describing the conversational arc Opening→…→Conclusion; **kept**, a false positive that passes the keystone test "true of this analysis professional on any job.")
- **D2 — "Requirements Analyst" → "the requirements producer":** `requirements-analyst` is a sibling agent (P2) in this cluster; the decouple doctrine forbids sibling-agent names even where the assess line-list didn't enumerate this one. Genericized to a **role** in both ENRICHMENT.md (Guidelines #6) and SPECIFICATION-INPUT.md (L3/L7). Role-based, no capability lost; keystone-clean.
- **D3 — `mode:`/`missing:[…]` invocation NOT written into `agent-dispatch.md`:** reconcile §G phrases this "push to `agent-dispatch.md`," but §B.3 clarifies the home is **"the lead (`agent-dispatch.md` guide; not a file field)."** `templates/agent-dispatch.md` is a *generic, shared, do-not-commit caller-side checklist* whose own header says workflow knowledge "lives HERE… do not push any of it into the persona" — injecting a specify-specific `mode:specification-input missing:[who,problem,value]` example into that generic template would specialize it to one workflow (the exact coupling it is the antidote to). So this transform (a) removed the mechanic from the **skill body** (the actual requirement of trace #8), and (b) pointed readers to the caller-side dispatch convention. The concrete invocation belongs on the **P1 `specify` lead's** Task() prompt, constructed per the agent-dispatch convention — a separate primitive, transformed separately. **Flag for the verifier/lead:** confirm the P1 lead carries the enrichment dispatch (mode + known-missing elements) so #8 is realized, not silently dropped. Out of this primitive's scope; agent-dispatch.md and loop-discipline intentionally untouched (brief constraint).
- **D4 — ENRICHMENT.md template footer removed entirely (not merely rewritten):** the footer's whole job was the marker-wrapped supervisor-continuation signal (markers dropped 6a; supervisor-continuation `moved-to-lead`). The useful "the Summary is the carry-forward" guidance survives, role-based, in Guidelines #6. The template now ends at "Original Input" — pure artifact structure, nothing for a machine to parse.
- **No `redesign`, no `split/merge/promote`:** body was content-clean (assess Check 1: zero content-coupling; all deny-list tokens sat in the brain/DAG-driven spec-input wiring); it is a one-shot pre-loop conditioner with no reviewable-artifact-without-validator (assess Check 3/4). Minimalism governor ⇒ `port-with-edits × standalone`.
- **Within-cluster forward ref** `mochiko:analysis-specifications` (P6) resolves by end of Phase 3 (P6 is `CREATE` in §D; companion trace already written). Not dangling at verify time.
- **Router (wiring #2)** intentionally not edited (brief constraint + separate §D artifact line). The description holds the general/shared note; confirm the router EDIT surfaces it.

---

**Transform version:** v1 · **Governed by:** `loop-discipline` · **Disposition applied:** port-with-edits × standalone (dual-mode kept; lead-invoked pre-step) · **Next:** Phase 4 `verify-output` (independent agent) — REGISTRY flip `[ ]`→`[x]` on finalize; this trace is the migration record.
