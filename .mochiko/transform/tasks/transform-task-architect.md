# TRANSFORM — `agents/task-architect.md` (+ new `templates/taskarchitect-report-template.md`)

Run: `tasks` cluster transform · **Phase 3 (transform)** · Producer: `mochiko:transform-producer` · Skill: `mochiko:transform-recipes` · Date: 2026-07-01
Consumes: `assess-task-architect.md` (P2, full trace §Step 5) · `reconcile.md` (Flag 2 report-template, Flag 3 persona↔skill split, P2 re-emitted trace L215–222, rehome map) · `contract.md` (§1 decoupling doctrine, §4 gated decisions).
Models: `agents/technical-analyst.md` (decoupled-producer gold standard) · `templates/techanalyst-report-template.md` (producer-report precedent).
Gated decisions applied as finalized: **RQ-A → Branch A** (two artifact-types) · **producer-report home → new `taskarchitect-report-template`** · self-verdict drop holds under either home.

```
TRANSFORM: task-architect (+ taskarchitect-report-template)
Applied:   port-with-edits × standalone  +  promote (new report template)  +  wiring-pass
           (heaviest port of the run — whole-section excisions, not line edits)
Artifacts: plugins/mochiko/agents/task-architect.md                     (edited → net-new mochiko form, 109 lines)
           plugins/mochiko/templates/taskarchitect-report-template.md   (created, 80 lines — promote from inline "Planner Report")
New partners: taskarchitect-report-template (template) — the producer's per-round NON-verdict self-disclosure report;
              gives P6's architect_report_path slice a home. (No new agent/skill — producer↔validator pair already exists & is independent.)
Wiring:    classification = model-invoked producer agent (agent default; no user-invoked flag — matches technical-analyst)
           router         = NOT edited (Wave-2 cluster wiring, per scope boundary) — pending item recorded below
           triggers       = agent description = work-context; 3 <example> blocks ADDED (HIL had 0), decoupled (no phase/path/sibling)
           rebinds        = .humaninloop/memory/codebase-analysis.md → .mochiko/memory/codebase-analysis.md (kept-but-rebind)
                            frontmatter `skills:` bare (patterns-vertical-tdd); body ref → mochiko:patterns-vertical-tdd
           decouple       = ALL deny-list tokens excised (scan below, zero residual)
           single-source  = literal Cycle Structure / **TEST:** grammar / slice heuristics / foundation test / marker table /
                            quality checklist → referenced to mochiko:patterns-vertical-tdd, NOT copied (folded per Flag 3)
```

---

## Convention-wiring pass — item by item

1. **Classification** — model-invoked producer agent (agents are model-invoked; no `disable-model-invocation`, matching `technical-analyst`). `skills: patterns-vertical-tdd` declared; persona-vs-procedure split honored (persona = taste; procedure → P3).
2. **Router registration** — **DEFERRED to Wave-2** per the explicit scope boundary (do NOT edit the router this call). Recorded as a pending cluster-wiring item (reconcile Flag 7.2: new "Tasks cluster" router section + a `task-architect` Agents row). Not a silent drop — owned by the Wave-2 wiring pass.
3. **Trigger phrasing** — agent description rewritten to describe **work-context** (turn planning artifacts into a story→cycle mapping / cycle-based task list), with **3 `<example>` blocks** added (HIL shipped none — the heavier-wiring axis the assessment flagged). Decoupled: no workflow-mode/path injection, no sibling-agent names.
4. **Path rebinding** — `.humaninloop/memory/codebase-analysis.md` → `.mochiko/memory/codebase-analysis.md` (`kept-but-rebind`); `evolution-roadmap.md` NOT mounted (deferred-by-reference stub — roadmap track parked, contract §1); body skill ref carries the `mochiko:` prefix, frontmatter stays bare.
5. **Decouple persona/skill** — the run's highest-risk surface. Every deny-list class excised (see "Decoupling tokens cut"). Keystone-clean craft retained (vertical slicing, TDD, foundation+parallel, traceability, brownfield markers, verification-task concept). `plugin.json` `agents` array entry = **DEFERRED to Wave-2** per scope boundary (reconcile Flag 7.3) — pending, not dropped.
6. **Single-source / de-dup** — persona now *references* `mochiko:patterns-vertical-tdd` for all literal procedure (Flag 3 fold); it restates none of it. No inline report (promoted to the template). No `loop-discipline`/contract doctrine in the persona (never had it — agent, not command).

---

## Realized responsibility trace — all 24 P2 responsibilities (assess §Step 5), no silent drops

| # | Responsibility | Realized tag | Where it landed |
|---|----------------|--------------|-----------------|
| 1 | Task-architect persona & judgment (Core Identity war-stories: oversized/misordered tasks, horizontal-slice pain, tasks-not-mapping-to-value, integration nightmares) | **kept-in-persona** | agent `## Core Identity` (4 scar→practice bullets; keystone-clean) |
| 2 | Vertical-slicing + TDD **procedure** via `patterns-vertical-tdd` (P3) | **kept-but-rebind** | frontmatter `skills: patterns-vertical-tdd` (bare); body `## Skills Available` → `mochiko:patterns-vertical-tdd`. Mount valid once P3 lands (parallel port). |
| 3 | Author a **story→cycle mapping** (artifact-type) | **kept-in-persona** (+ HOW `folded-to-P3`) | `## What You Produce` #1 `task-mapping.md` (table + type/deps + **slice rationale**). Branch A ⇒ menu lists **2** types. |
| 4 | Author a **cycle-based `tasks.md`** (artifact-type) | **kept-in-persona** (+ HOW `folded-to-P3`) | `## What You Produce` #2 `tasks.md` (foundation/feature `[P]`, file path, `[US#]`, TEST task, markers; Story→Cycle table = **derived echo**, Branch A) |
| 5 | **Phase-mode framing** ("How You Operate" / "Phase Behaviors" as injected modes) | **dropped + reason** | CUT. *Reason:* injected-workflow-mode coupling; a decoupled producer authors the artifact-type from its brief, not a self-selected phase. Content survives as the artifact-type menu (#3/#4). Zero `\bphase\b` in the port. |
| 6 | Mapping→Tasks **sequencing** the phase-mode implied | **moved-to-lead** | Not in persona. Lead sequences Phase 1→2 (reconcile rehome map). |
| 7 | **Context-file mechanism** ("reads instructions from a context file"; "Reading the Context"; `supervisor_instructions`; "always start by reading the context file") | **dropped + reason** | CUT (whole "How You Operate" + "Reading the Context" sections). *Reason:* markdown-supervisor carrier; mochiko briefs in-session via `agent-dispatch` + workspace-as-state. |
| 8 | **Injected input list** (spec/plan/constraints/data-model/contracts/constitution; `file_paths`) | **moved-to-lead** | Not self-enumerated. Lead's dispatch brief supplies inputs (`agent-dispatch`). |
| 9 | Constitution/governance **alignment** disposition | **kept-in-persona** | `## What You Embrace` → "Alignment with project governance." Principles arrive with the task (injection → moved-to-lead); the disposition to align stays. |
| 10 | Success-Criteria + Quality-Standards **quality bars** | **kept-in-persona** (taste) + **folded-to-P3** (literal) | `## Quality Standards` as taste bars (Vertical / Test-first / Verified / Traceable / File-anchored / Independently-completable / Minimally-coupled). Literal checklist → P3; review-side mirror = P4. |
| 11 | What You Reject / What You Embrace | **kept-in-persona** (taste) + light **folded-to-P3** | `## What You Reject` / `## What You Embrace` sections. |
| 12 | **Cycle Structure template** (`### Cycle N`, `TN.1→TN.4`, checkpoint) | **folded-to-P3** | CUT from persona; referenced to `mochiko:patterns-vertical-tdd` (canonical CYCLE-STRUCTURE). No orphan — P3 is the home. |
| 13 | **Verification-task concept** — a real-integration task that gates cycle completion ("makes vertical TDD actually vertical") | **kept-in-persona** (taste) + **folded-to-P3** (grammar) | Quality Standards "Verified against reality" (carries "what makes a vertical slice actually vertical") + Reject/Embrace bullets. The *concept* is taste; the grammar → P3. |
| 14 | **`**TEST:**` verification grammar** (format · field ref · action modifiers · assert patterns · examples · legacy — ~130 lines) | **folded-to-P3** | CUT entirely; referenced to `mochiko:patterns-vertical-tdd`. |
| 15 | **qa-engineer runtime-classification handoff** (auto-approve vs human checkpoint; "when to involve a human") | **moved-to-other-cluster** (implement/qa, parked) + **4 sibling-name tokens dropped** | CUT. The 4 `qa-engineer` tokens (L248/274/279/337) dropped (decouple); the runtime-classification *capability* re-homes to implement/qa (deferred, parked — NOT dropped). The design-time verification-task *concept* stays (#13). |
| 16 | Inline **"Planner Report"** self-disclosure (Summary, What Was Produced, Vertical-Slice Rationale, TDD Structure, Constitution Alignment, Open Questions) | **promoted-to-report-template** | New `templates/taskarchitect-report-template.md`. Non-verdict descriptive sections realized there (incl. tasks-specific **Vertical-Slice Rationale** + **TDD Structure**). |
| 17 | Report's **self-asserted verdict** (`Completion: complete/partial`; `Ready for Review …Devil's Advocate`) | **dropped + reason** | NOT in the template (documented in its no-self-verdict Usage Note) and NOT in the persona. *Reason:* independence — the lead owns clearing; mirrors the plan port dropping `techanalyst-report`'s `completion_status`. Also carried the A1 "Devil's Advocate" name → gone. |
| 18 | Report **path injection** | **moved-to-lead** | Template's Output-location note: `.mochiko/specs/<feature>/taskarchitect-report.md`, **seeded/collected by the lead** — not injected in the persona. |
| 19 | Brownfield **task markers** `[NEW]`/`[EXTEND]`/`[MODIFY]`/`[GAP:XXX]` + plan-marker→task-marker translation | **kept-in-persona** (craft) + light **folded-to-P3** (table) | `## Brownfield Awareness` (extension classification + plan-marker translation + gap traceability, as disposition). The literal **marker table** → P3. |
| 20 | Brownfield **codebase-analysis read** | **kept-but-rebind** | `## Brownfield Awareness` → `.mochiko/memory/codebase-analysis.md` (real mochiko artifact from setup's `analysis-codebase`), framed "when it exists" (not a mandatory boot read). |
| 21 | Brownfield **evolution-roadmap read** (the `[GAP:XXX]` source) | **deferred-by-reference** | `## Brownfield Awareness` gap-traceability note: "the evolution-roadmap track is deferred in the current mochiko core; the marker disposition stands." Path NOT mounted (stub, contract §1). |
| 22 | **PRODUCER team-role** (authors both artifacts; reviewed by `devils-advocate`+`validation-task-artifacts`) | **kept-in-persona** | description "Authors the task artifacts; **does not grade its own output**." `skills {patterns-vertical-tdd} ∩ grading = ∅` — independence structurally intact; no reviewer named. |
| 23 | Classification + router/discoverability + **`<example>` blocks** (HIL description was bare) | **kept-but-rebind** (wiring — heavier) | 3 `<example>` blocks authored (work-context). Router + `plugin.json` = **Wave-2, deferred** per scope boundary (pending items recorded above; not silent drops). |
| 24 | Agent frontmatter (`model: opus`, `color: green`) | **kept-in-persona** | frontmatter unchanged. |

**Drops that carry a reason for the human gate to accept:** #5 (phase-mode framing), #7 (context-file mechanism), #15 (4 qa-engineer sibling-name tokens — capability parked, not lost), #17 (report self-verdict fields). Each is mechanism/plumbing or a re-homed-and-parked capability — **no craft capability lost.**

---

## Decoupling tokens cut (the run's headline — coupled on the worst categories, now clean)

Verified zero residual via `grep` on the port:

- **A. Sibling-agent names (5):** "Devil's Advocate" ×1 (report handoff line) — dropped; "qa-engineer" ×4 (L248/274/279/337) — dropped. Reviewer is never named by the producer; runtime classification re-homes to implement/qa.
- **B. Phase-mode skeleton:** "How You Operate", "Phase Behaviors", "which phase (mapping/tasks)", the `{phase}` interpolation, `phase:` field — all cut. Re-expressed as a **2-item artifact-type menu** (Branch A). Zero `\bphase\b` remain.
- **C. Context-file / caller-injection:** "reads instructions from a context file", the entire "Reading the Context" section, `supervisor_instructions`, `file_paths`, "always start by reading the context file", enumerated input paths — all cut. Inputs arrive via the lead's brief (`agent-dispatch`).
- **D. Path coupling:** `.humaninloop/` → `.mochiko/` (codebase-analysis rebind); evolution-roadmap left as a documented stub, not mounted.
- **E. Self-asserted verdict:** `Completion: complete/partial`, `Ready for Review …Devil's Advocate` — dropped (independence). The only case-insensitive "completion" left is benign craft ("gates cycle completion" = the verification-task concept).

**Keystone-clean survivors (KEEP, not scrubbed):** vertical slice, TDD, test-first, real-integration verification ("what makes a vertical slice actually vertical"), foundation + parallel `[P]`, `[US#]` traceability, file-path-per-task, brownfield markers, constitution alignment — all true of any TDD architect on any job.

---

## Zero-silent-drops confirmation & handoff

- Every one of the 24 P2 responsibilities carries a realized tag above. No responsibility left untagged.
- Two deferred items (router registration, `plugin.json` agents-array entry) are **explicitly recorded as pending Wave-2 wiring** per the scope boundary — deferred with a home, not dropped.
- The `devils-advocate` re-mount is **out of scope** for this transform (Wave-2 wiring; the reviewer agent is untouched here) — recorded, not actioned.
- **Independence intact:** the produced agent mounts only `patterns-vertical-tdd` (an authoring skill); it declares "does not grade its own output"; no grading skill, no reviewer name. Produce + grade never co-reside.

**Next:** `mochiko:verify-output`, run by an **independent `mochiko:validator`** (a different agent) — grades both artifacts (five-convention conformance + decoupling scan + sound-loop placement) and audits this realized trace for silent loss. This producer does not grade its own output.
