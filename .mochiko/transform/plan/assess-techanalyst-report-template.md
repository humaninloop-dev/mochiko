# Assessment — `techanalyst-report-template.md` (P12)

**Run:** transform-cluster / `plan` · **Assessed by:** `transform-producer` · **Date:** 2026-06-30
**Source:** `human-in-loop/plugins/humaninloop/templates/techanalyst-report-template.md`
**Skill:** `mochiko:assess-primitive` (ASSESS ONLY — no edit applied, no grade)
**Precedent:** `plugins/mochiko/templates/analyst-report-template.md` (specify's ported producer-report) + its assess/transform records

> ROLE NOTE: assess/diagnose ONLY. The body treatment is proposed; relational moves (cross-report-family
> consistency, edit-depth gated on the lead-rehome, reviewer identity) are emitted as `flag-for-reconcile`,
> not resolved here.

---

## ASSESSMENT (standard format)

```
ASSESSMENT: techanalyst-report-template.md (P12)
Class:        template → branch artifact
Triage:       gate1=n  gate2=y  gate3=n   [full-lens]  (gate2 trips: fans out to lead + reviewer(s))
Disposition:  port-with-edits × standalone
              (+ reconcile flags on edit-DEPTH and cross-report-family round/header consistency [RQ1-gated])
Trace:
  - Structure the technical-analyst's self-report (producer handoff disclosure)        → kept
  - "What Was Produced" production summary (producer narrative)                          → kept (primary decision-relevant prose)
  - Report identity header (feature_id / phase / iteration / timestamp)                  → kept-but-rebind ({{iteration}}→{{round}}; {{phase}} kept as disclosure; paths→mochiko)
  - {{phase}} disclosure (analysis|design — which phase this round produced)             → kept (workflow concept; the 2-phase sequence is lead-owned [moved-to-lead], the report only discloses it)
  - Summary "Artifacts Produced" count                                                   → kept-but-rebind (demote; reviewers read the artifacts directly)
  - Summary "Completion" status ({{completion_status}})                                  → dropped + reason (self-asserted done/verdict; the done-condition is LEAD-owned, informed by reviewer reports — never self-asserted by the producer)
  - Analysis-phase count tables (TR/constraint/decision/NFR/FR-coverage)                 → kept-but-rebind (demote/slim; duplicate requirements.md / constraints-and-decisions.md / nfrs.md)
  - TR traceability snapshot table (TR ID / Source FR / Priority / Description)          → kept-but-rebind (demote; duplicates requirements.md the reviewer reads)
  - Design-phase count tables (entity/relationship/endpoint/classification/integration) → kept-but-rebind (demote/slim; duplicate data-model.md / contracts/api.yaml)
  - Feed the brain-era count/trajectory metric surface (count-per-phase parse/metric)    → dropped + reason (kernel-free; lead + reviewer read artifacts directly; no metric/trajectory feed)
  - Constitution Alignment ({{constitution_alignment}})                                  → kept (craft disclosure; constitution is a real mochiko artifact; optional light reword of the "Constitution" label, matching the technical-analyst agent port)
  - Open Questions ({{open_questions}})                                                  → kept (high value; feeds the clarification loop / gap routing; the advocate stress-tests gaps)
  - "Ready for Review" ({{ready_for_review}})                                            → kept-but-reframe (producer→reviewer handoff pointer, NOT a self-asserted ready/PASS verdict; verdict is the lead's)
  - {{mustache}} placeholder + section-heading convention                                → kept (matches the mochiko template wrapper / analyst-report precedent)
  - Be read by the prose Supervisor + reviewers as producer-disclosure context           → kept-but-rebind (consumer set rebinds: HIL prose-supervisor + architect + advocate → mochiko plan lead + reviewer(s) [RQ1]; both read prose directly — NO parser to dissolve, unlike specify)
  - Be the "Report format: follow <template>" dispatch target                            → kept-but-rebind (template path → mochiko) + dispatch reference moved-to-lead (lead points the producer at this template via agent-dispatch)
  - Output location / placement                                                          → kept-but-rebind (.workflow/techanalyst-report.md → .mochiko/specs/<feature>/techanalyst-report.md, workspace-as-state)
  - "Technical Analyst" self-reference (title)                                           → kept (role-based self-reference, like specify's "Analyst Report"; no SIBLING-agent name in body — decouple scan CLEAN)
Reconcile flags:
  - EDIT-DEPTH of the counts demotion (how far to slim the count-heavy "Key Outputs" shape, and whether to
    add a "What Changed This Round" round-delta section as specify did) depends on the lead-rehome (P11
    plan-context-template absorb → what the plan lead needs to read each round). Decide in reconcile.
  - CROSS-REPORT-FAMILY round/header consistency: keep any round rebind ({{iteration}}→{{round}}) coherent
    across the plan report family (P13 architect-report, the advocate-report). GATED ON RQ1 (which reviewer
    reports survive) AND note the header SHAPE differs (P12 = blockquote header like the analyst-report;
    P13 = YAML frontmatter `iteration:`) — alignment is a cross-template call, not solo.
```

---

## Class / branch

**template → artifact.** Inert markdown skeleton with `{{mustache}}` slots; filled by the **technical-analyst**
producer (P2) and read by consumers. For this branch what matters: **placeholders**, **who fills / who
consumes it**, and **path coupling** — weighted exactly as the task directs. (Classification/router/trigger
conventions are N/A for an inert template.)

## Triage gate

| Gate | Q | Verdict | Evidence |
|------|---|---------|----------|
| 1 | Orchestration-coupled (does the **body** need a kernel/supervisor/command/DAG to function)? | **no** | The body is an inert report skeleton — zero kernel/DAG/catalog/MCP token, zero path, zero driver reference inside it. Consumption *is* orchestration-mediated (the prose Supervisor references the template + seeds the report path; reviewers read it), but that coupling lives in `plan.md` (P1), not in this file. |
| 2 | Multi-responsibility / fans out? | **yes** | Fans out to **multiple** consumers: the **lead/Supervisor** reads it, and **both reviewers** (HIL: principal-architect + devils-advocate) read it as context (`plan.md` L357/416/550). |
| 3 | Emits an artifact whose correctness is NOT machine-checkable? | **no** | A template does not itself emit a gradeable artifact. The filled report's content is producer self-disclosure; the producer↔reviewer pairing already exists at the **agent** layer (RQ1 architecture) — this template introduces no new validator need. |

One "yes" (gate2) → **full 7-check lens** (the fan-out to lead + reviewer(s) and the brain-residue question earn it). Matches the specify analyst-report precedent (gate2 trips, full lens).

## Placeholders (inventory)

| Placeholder | Kind | Disposition |
|-------------|------|-------------|
| `{{feature_id}}` | identity | **kept** (generic; grounded by the lead's workspace naming) |
| `{{phase}}` | workflow loop concept (analysis\|design) | **kept** — the 2-phase analysis→design sequence is **lead-owned** orchestration (assess-plan-command §B, `moved-to-lead`); the report legitimately *discloses* which phase it produced this round. Not a brain token; survives in the mochiko plan loop. Appears in header + Summary table. |
| `{{iteration}}` | **loop coupling** | **kept-but-rebind → `{{round}}`** — HIL "iteration" = the brain-era pass counter; mochiko's loop is the supervisor's bounded **round** loop (contract round-cap = 3). Identical rename to the specify analyst-report port; relabel `> Iteration:` → `> Round:`. Concept survives. |
| `{{timestamp}}` | identity | **kept** (generic) |
| `{{artifacts_produced}}` (Summary "Artifacts Produced") | count metric | **kept-but-rebind** (demote — the six real artifacts are read directly by the reviewers; convenience snapshot only) |
| `{{completion_status}}` (Summary "Completion") | **self-asserted done / verdict-ish** | **dropped + reason** — the done-condition/verdict is **LEAD-owned** (informed by the architect/advocate reports), never self-asserted by the producer. A producer self-report that declares its own "Completion" leaks the verdict the independence rule reserves for the lead. (Same independence stance the precedent states: "self-disclosure, not a verdict.") Lead/human gate accepts the drop. |
| `{{production_summary}}` ("What Was Produced") | content | **kept** (primary decision-relevant prose for a direct-reading lead; analog of the precedent's `{{summary}}`) |
| `{{tr_count}}` `{{constraint_count}}` `{{decision_count}}` `{{nfr_count}}` `{{fr_coverage}}` (Analysis Phase) | **metrics / parse surface** | **kept-but-rebind** (demote/slim — each counts a section of `requirements.md` / `constraints-and-decisions.md` / `nfrs.md`, which the reviewers read directly; redundant) |
| `{{tr_id}}` `{{source_fr}}` `{{priority}}` `{{description}}` (TR traceability snapshot) | **traceability snapshot / parse surface** | **kept-but-rebind** (demote — duplicates the `requirements.md` artifact the reviewer reads; optional convenience) |
| `{{entity_count}}` `{{relationship_count}}` `{{endpoint_count}}` `{{classification_count}}` `{{integration_count}}` (Design Phase) | **metrics / parse surface** | **kept-but-rebind** (demote/slim — duplicate `data-model.md` / `contracts/api.yaml`) |
| `{{constitution_alignment}}` ("Constitution Alignment") | content / craft | **kept** — the producer discloses how its artifacts align to project governance; legitimate (constitution is a real mochiko artifact; the ported `technical-analyst` keeps this craft — assess-technical-analyst trace #11). Optional light reword of the "Constitution" label to "project principles/governance" to match the agent-side treatment. |
| `{{open_questions}}` ("Open Questions") | content | **kept** — high value; producer-disclosed unknowns feed the **clarification loop / gap routing** (the advocate stress-tests gaps). Analog of the precedent's Assumptions/Notes. |
| `{{ready_for_review}}` ("Ready for Review") | handoff / verdict-ish | **kept-but-reframe** — a producer→reviewer **handoff pointer** (what is ready for the reviewer to grade), NOT a self-asserted "I declare this ready/done" verdict. Reframe so it reads as handoff, not a done-claim; the verdict is the lead's (independence note). |

No `.humaninloop/` paths, catalog paths, or script paths appear **inside** the body — placeholder coupling
(`{{iteration}}`) and the count-metric surface are the only body-internal residues. (Identical body-internal
cleanliness to the specify analyst-report.)

## Who fills it / who consumes it

- **Fills it (unchanged):** the **technical-analyst** producer (P2). `plan.md` directs it to write
  `.workflow/techanalyst-report.md` alongside the six artifacts, with "**Report format**: Follow
  `…/techanalyst-report-template.md`" (L311/505/667/766). The analyst authors both the products and this
  self-report.
- **Consumed it in HIL:** the **prose Supervisor** (reads the report; `plan.md` is a markdown supervisor —
  "You are the Supervisor… You own the loop, manage state via files, and route based on agent outputs", L7)
  **and** both reviewers — **principal-architect** (feasibility, reads the analyst report as context, L357)
  and **devils-advocate** (completeness, L416/550). **Crucially the Supervisor does NOT extract a verdict
  from this report** — it routes on the **architect-report** (L393) and **advocate-report** (L454/605)
  verdicts. This report is pure producer self-disclosure.
- **Consumes it in mochiko:** the **plan lead/referee** (the thinned supervisor, P1) reads it directly, and
  the **reviewer(s)** read it as critic context. Consumer set rebinds `{prose-supervisor, architect,
  advocate}` → `{plan lead, reviewer(s)}` — reviewer identity is **RQ1** (one reviewer vs the two-form
  case). Both readers are direct prose readers.

## Brain-residue verdict (the run-goal headline)

**Did its structure assume brain machinery? — Partially, by *count-surface accommodation*, never by
*dependency*. And — unlike specify — there is NO parser to dissolve.**

- **No content-coupling / no dangling references.** The body contains zero references to a sibling agent,
  DAG, catalog, MCP, `parse-and-advance`, or any excluded primitive, and zero hardcoded paths. The
  decoupling-doctrine scan (path coupling + dangling refs) comes back **clean** for this file.
- **The key difference from the specify precedent.** specify's analyst-report was heading-**parsed** by the
  `state-analyst` (DAG `parse-and-advance`) → the precedent tagged "be heading-parsed → moved-to-lead
  (parse layer dissolves)". **`plan` is a prose Supervisor with no state-analyst and no DAG** (grep of
  `plan.md` for `state-analyst|hil-dag|catalog|MCP|parse-and-advance` → none). So **there is no parser to
  dissolve** here — the Supervisor and reviewers already read the report directly. The brain residue is
  therefore **the count-metric surface**, not a parser.
- **The count residue.** Two brain-era surfaces remain: (1) the **count tables** (Summary "Artifacts
  Produced"; the Analysis-phase TR/constraint/decision/NFR/FR-coverage block; the Design-phase
  entity/relationship/endpoint/classification/integration block; the TR traceability snapshot) — a
  count/metric/parse surface inherited from the brain-era report design, duplicating the six real artifacts
  the reviewers must read anyway; and (2) the **self-asserted "Completion" status**, a done-flag the producer
  must not assert (the verdict is lead-owned). For a lead + reviewer reading prose directly, the counts are
  low marginal value and "Completion" is an independence leak.
- **It survives as a valid human report.** Stable headings still help a direct reader; the production
  summary, constitution alignment, and open questions are the decision-relevant prose. This is an
  *accommodation* (demote the counts, drop the self-verdict), not a *dependency* — hence **port-with-edits**,
  not redesign. Exactly the precedent's verdict shape, with the parser concern replaced by the count/Completion
  concern.

## Lens findings (artifact-weighted)

1. **Orchestration test.** Orchestrated externally by the prose Supervisor (the "Report format: follow …"
   dispatch reference + seed/collect of the report path) and read by the reviewers. **Content-coupling:
   none** (clean inert body). **Orchestration-coupling:** the *reference to* and *consumption of* the
   template both live outside the file (in `plan.md`/P1) and rehome to the lead. The "point the analyst at
   this template" dispatch wiring → **moved-to-lead** (lead's dispatch / `agent-dispatch.md`). No DAG/parser
   responsibility to re-home (plan has none).
2. **Role (two altitudes).** Skill-role: it shapes a **producer's self-report / handoff**. Team-role:
   filling it = producer disclosure; reading it confers lead-referee context or reviewer context. It needs
   **no validator partner of its own** — the producer↔reviewer pair exists at the agent layer (RQ1).
3. **Independence.** N/A to a template, and the diagnosis is the headline edit driver: this report is the
   analyst's **self-disclosure**, not a grade. The FAIL/PASS/feasibility verdicts live in the
   **architect-report** (`{feasible|infeasible|needs-revision}`) and **advocate-report**, authored by
   different agents and read by the Supervisor for routing. **The `{{completion_status}}` and
   `{{ready_for_review}}` fields are the only independence risk** — they let the producer self-assert a
   done/ready state. → `{{completion_status}}` **dropped** (verdict is lead-owned); `{{ready_for_review}}`
   **reframed** to a handoff pointer. Note for the lead: weight the reviewer verdicts for loop control, never
   this self-report.
4. **Verdict-sink / loop-driver.** This report is **not** a loop-driver — it carries no verdict; it is
   producer-side input to the loop. The loop is driven by the **reviewer** verdicts (architect feasibility +
   advocate completeness; final reviewer shape = RQ1), owned by the lead. Consumption rehomes to the lead;
   the brain-era count/metric framing is dropped with the kernel heritage.
5. **Sibling / overlap.** Member of the **plan report family**: pairs with `architect-report-template.md`
   (P13, feasibility-reviewer report — carries a verdict) and the `advocate-report-template.md`
   (completeness-reviewer report — carries the 3-state verdict; specify-ported P15). **Not a merge** (distinct
   content + this one is the producer self-report, those are reviewer verdict reports). → **reconcile flag:**
   keep the round/header rebind coherent across the family. **Header SHAPE differs** — P12 uses a blockquote
   header (`> Feature / Phase / Iteration / Generated`, like the analyst-report) while P13 uses YAML
   frontmatter (`iteration: 1`); whichever reviewer reports survive RQ1, align the *round vocabulary*, not the
   shape. Also: this report's content overlaps in spirit with the spec/analysis artifacts (the counts mirror
   their sections) — that overlap is the *demotion* signal, **not a dedupe** (distinct purpose: producer
   disclosure vs the artifacts themselves).
6. **Coupling audit.** Body-internal hardcoded paths: **none**. External references TO it
   (`${CLAUDE_PLUGIN_ROOT}/templates/techanalyst-report-template.md`,
   `specs/{feature-id}/.workflow/techanalyst-report.md`) live in `plan.md` and rebind to the mochiko
   plugin/workspace paths → **`.mochiko/specs/<feature>/techanalyst-report.md`** (workspace-as-state; matches
   assess-plan-command §C, which keeps the round reports as per-round workspace artifacts). Prerequisite/
   handoff: companion to the six analysis/design artifacts (same producer, same round). Determinism boundary:
   entirely model-judgment content (the counts are analyst-counted, not script-emitted) → no degenerate-
   validator concern.
7. **Conventions + loop placement.** Classification/router/trigger tags are **N/A for an inert template**.
   Live wiring-pass items: **path-rebinding** (step 4) and the **decouple scan** (step 5). **Decouple scan
   CLEAN:** no SIBLING-agent name in the body (no principal-architect / devils-advocate / supervisor /
   state-analyst), no "dispatch", no "workflow-agnostic" label, no injected workflow modes/paths. The title
   **"Technical Analyst Report"** is a **role-based self-reference** (the producer naming its own profession),
   exactly parallel to the precedent keeping "**Analyst Report**" for the requirements-analyst — `kept`, not
   a coupling. **Mochiko template-wrapper precedent** (`analyst-report-template.md` /
   `codebase-analysis-template.md`): `# Title Template` + one-line "This template defines the structure for …"
   + fenced ` ```markdown ` body (filled doc) + `## Usage Notes`. Adopting this wrapper (and pushing the
   `<!-- If phase == … -->` HTML-comment directives out of the fenced body into Usage Notes) is a
   convention-alignment edit — part of why this is `port-with-edits`, not `keep-verbatim`. Producer↔validator
   pairing: exists at the agent layer (RQ1); this template owes no partner primitive. Sound-loop gates land on
   the lead (P1 rehome-orchestration), not on this artifact.

## Disposition rationale (minimalism governor)

`keep-verbatim` was considered and **rejected** — real (not cosmetic) edits are earned, the same family as the
specify analyst-report plus two P12-specific ones:
- (a) `{{iteration}}` → `{{round}}` (rebind off the brain-era pass counter) — identical to the precedent;
- (b) **demote the count-heavy "Key Outputs"/Summary count surface** (Analysis + Design count tables, TR
  traceability snapshot, "Artifacts Produced") to an optional convenience snapshot, retiring the metric
  framing — same move as the precedent's count demotion;
- (c) **drop the self-asserted `{{completion_status}}`** and **reframe `{{ready_for_review}}`** so the producer
  never asserts the verdict/done-state the lead owns (P12-specific; the precedent had no such field);
- (d) align to the mochiko template wrapper, pushing the `<!-- If phase == … -->` directives into Usage Notes.

None rise to `redesign` — the body is clean, portable, and survives the brain heritage intact (no kernel/DAG/
catalog token, no path, no parser dependency). **Structural = `standalone`**: it remains one real producer
artifact in `templates/`, filled by the analyst and read by the lead + reviewer(s). It does **not**
`absorb-into-lead` (contrast P11 plan-context-template, which IS the brain-era `.workflow/` state-carrier →
absorb) because this is a genuine producer-authored deliverable, not in-session orchestration state. No
split/merge/promote.

## Edits vs the analyst-report precedent (task-requested)

| Precedent edit (specify P12) | Applies to plan P12? | Detail |
|---|---|---|
| `{{iteration}}` → `{{round}}` (relabel `> Iteration:` → `> Round:`) | **YES, identical** | P12 header carries `> Iteration: {{iteration}}` (L5) — same rename, same blockquote-header shape. |
| Demote brain-era counts (slim to optional convenience snapshot; retire "Metric" framing) | **YES, heavier** | P12 has **far more** counts than specify: Summary "Artifacts Produced"; Analysis-phase block (TR/constraint/decision/NFR/FR-coverage); a TR traceability snapshot; Design-phase block (entity/relationship/endpoint/classification/integration). All demote; all duplicate the six real artifacts. |
| Drop the DAG/trajectory metric feed | **YES, reframed** | plan has no literal DAG, so there is no `state-analyst` trajectory feed to drop; the analog is dropping the **count/metric/parse-surface framing** (kernel-free heritage) and, P12-specific, the **self-asserted `{{completion_status}}`**. |
| Rebind paths to workspace-as-state | **YES** | Body has no internal path (like specify); the rebind is realized in Usage Notes: `.workflow/techanalyst-report.md` → `.mochiko/specs/<feature>/techanalyst-report.md`. |
| Align to mochiko template wrapper (`# Title` + intro + ```markdown body + `## Usage Notes`) | **YES** | Plus push the `<!-- If phase == … -->` HTML-comment directives out of the fenced body into Usage Notes (the precedent stripped inline HTML-comment instructions). |
| Add `## What Changed This Round` round-delta section | **RECOMMENDED — depth-gated** | specify ADDED this to support the lead's no-progress watch (LD-3.2). plan's loop also has a no-progress exit (contract §3), so the same addition fits — but the **depth** (and whether the plan lead needs it given P11's absorb) is a reconcile call, not asserted here. |
| **P12-specific (no precedent analog):** keep `{{phase}}` disclosure; keep `{{constitution_alignment}}` (optional label reword); keep `{{open_questions}}`; reframe `{{ready_for_review}}` | **NEW** | These sections do not exist in the specify analyst-report; assessed individually above (phase = lead-owned concept the report discloses; constitution-alignment + open-questions = high-value craft; ready-for-review reframed off a verdict-claim). |

## What is explicitly NOT decided here (handoff to reconcile)

- **EDIT-DEPTH** of edit (b)/(c) — how far to slim the count surface, and whether to add a "What Changed This
  Round" round-delta — is gated on the **lead-rehome** (P11 plan-context-template absorb → what the plan lead
  actually needs to read each round). Decide in reconcile alongside the P11 absorb.
- **CROSS-REPORT-FAMILY consistency** — keep the round rebind coherent across the plan report family (P13
  architect-report + the advocate-report). **Gated on RQ1** (which reviewer reports survive the reviewer-
  architecture decision) and complicated by the **differing header shape** (P12 blockquote vs P13 YAML
  frontmatter). A cross-template call for reconcile, not solo.
- All structural moves remain `standalone` here; no split/merge/promote/absorb proposed for P12.

---

## Output block

```
ASSESSMENT: techanalyst-report-template.md (P12)
Class:        template → branch artifact
Triage:       gate1=n gate2=y gate3=n  [full-lens]  (gate2: fans out to lead + reviewer(s))
Disposition:  port-with-edits × standalone
              + flag-for-reconcile: edit-depth (lead-rehome gated) · cross-report-family round/header consistency (RQ1-gated)
Trace:
  - producer self-report structure (handoff disclosure)                 → kept
  - "What Was Produced" production summary                              → kept (primary prose)
  - identity header (feature_id/phase/iteration/timestamp)              → kept-but-rebind ({{iteration}}→{{round}}; paths→mochiko)
  - {{phase}} disclosure (analysis|design)                             → kept (lead owns the 2-phase sequence; report discloses it)
  - Summary "Artifacts Produced" count                                 → kept-but-rebind (demote)
  - Summary "Completion" status                                        → dropped + reason (verdict is LEAD-owned; producer must not self-assert done)
  - Analysis-phase count tables (TR/constraint/decision/NFR/FR-cov)    → kept-but-rebind (demote; duplicate requirements/constraints/nfrs)
  - TR traceability snapshot table                                     → kept-but-rebind (demote; duplicates requirements.md)
  - Design-phase count tables (entity/rel/endpoint/class/integration)  → kept-but-rebind (demote; duplicate data-model/contracts)
  - brain-era count/metric/parse-surface framing                       → dropped + reason (kernel-free; lead+reviewer read artifacts directly)
  - Constitution Alignment                                             → kept (craft; optional light "Constitution" label reword)
  - Open Questions                                                     → kept (high value; feeds clarification loop / gap routing)
  - "Ready for Review"                                                 → kept-but-reframe (handoff pointer, not a self-asserted ready/verdict)
  - {{mustache}} + heading convention                                  → kept (matches wrapper / analyst-report precedent)
  - read by Supervisor + reviewers as producer-disclosure context      → kept-but-rebind (consumers → plan lead + reviewer(s) [RQ1]; direct prose read — NO parser to dissolve, unlike specify)
  - "Report format: follow <template>" dispatch target                 → kept-but-rebind (path → mochiko) + dispatch reference moved-to-lead (agent-dispatch)
  - output location / placement                                        → kept-but-rebind (.mochiko/specs/<feature>/techanalyst-report.md)
  - "Technical Analyst" self-reference (title)                         → kept (role-based self-reference; no SIBLING name; decouple scan CLEAN)
Reconcile flags:
  - edit-DEPTH of counts demotion + whether to add "What Changed This Round" — gated on the lead-rehome (P11 absorb)
  - cross-report-family round/header consistency (P13 architect-report + advocate-report) — GATED ON RQ1; header SHAPE differs (blockquote vs YAML frontmatter)
Decouple scan:  CLEAN — 0 sibling-agent names · 0 "dispatch" · 0 "workflow-agnostic" · 0 injected paths/modes;
                title "Technical Analyst Report" = role-based self-reference (parallels precedent "Analyst Report")
Independence:   self-disclosure, no verdict — the {{completion_status}} drop + {{ready_for_review}} reframe keep the
                producer from asserting the LEAD-owned done-state; loop-driver verdicts live in the reviewer reports
```
