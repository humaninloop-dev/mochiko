# Mochiko Migration Registry

Track every primitive from `human-in-loop` — check off as it lands in mochiko. Items grouped by type, then by workflow affinity. A primitive may appear in multiple workflows.

**Status key**: `[ ]` not yet ported · `[x]` ported · `[~]` ported but needs upgrade · `[-]` deliberately excluded

---

## Mochiko-native primitives (net-new, not HIL ports)

The transformer cluster — the dogfooded implementation of [`PLAYBOOK.md`](PLAYBOOK.md). It is the tool that ports everything below it. Built before any HIL primitive is ported; proven by running it on the `setup` cluster.

| Status | Name | Type | Notes |
|--------|------|------|-------|
| `[x]` | `loop-discipline` | skill | Doctrine: the four sound-loop rules |
| `[x]` | `workflow-contract` | template | Fill-in loop contract (default FAIL) |
| `[x]` | `assess-primitive` | skill | Branch-by-class + 7-check lens + responsibility trace; **+ command-altitude rule (generic discipline → `dedupe`, not `moved-to-lead`) (2026-06-30)** |
| `[x]` | `reconcile-cluster` | skill | Relational verdicts + rehome-orchestration |
| `[x]` | `transform-recipes` | skill | Per-disposition recipes + convention-wiring pass; **+ thin-command `redesign` target + wiring step 6 (single-source) (2026-06-30)** |
| `[x]` | `verify-output` | skill | Done-condition checker (independent gate); **+ altitude / single-source check (item 8) (2026-06-30)** |
| `[x]` | `mochiko` | skill | User-invoked router / library index |
| `[x]` | `transform-producer` | agent | Assess / reconcile / apply |
| `[x]` | `validator` | agent | Generic independent grader — `verify-output` + `validation-constitution` (merges the former `transform-validator` + `constitution-validator`) |
| `[x]` | `transform-cluster` | command | Supervisor / lead-referee (owns the loop + human gate) |

> Lead/referee is the `transform-cluster` command supervisor, not a separate agent — so the cluster ships **2 agents**, not 3.

---

## Workflows (Commands)

| Status | Name | v3 Notes |
|--------|------|----------|
| `[x]` | `setup` | Ported 2026-06-27 — redesigned into a sound loop (default-FAIL done-condition, independent `validator`, escalate-on-cap, NEW constitution-acceptance human gate). Core-only scope. **Re-transformed to thin/altitude shape 2026-06-30 (385→78 lines; independently verified PASS).** **REWRITTEN v2 2026-07-16 (constitution flexibility + deep elicitation, per the accepted `setup-constitution-flexibility` record):** an **interrogation session** (lead-inline via `analysis-iterative`; nine-dimension agenda, low-tier pruning, deck arbitration, waiver rulings) now precedes and governs authoring; closes on a ratified synthesis (`.mochiko/memory/governance-intent.md`, durable, delta-updated on amend) that is a **traceable contract** on the producer (selection = session's, formulation = producer's; deterministic GI trace-IDs; flagged proposals ruled at acceptance); floor scope-tiered (`poc`/`internal`/`production`/`regulated`) with recorded waivers at low tiers; principle content from the tier/type-tagged **catalog** + session-minted intents; template split into **universal core + attachable modules**; **team-form** (the second agent-team command after brainstorm — standing producer seat, cold validator seat, lead-routed fix list; hard-requires `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS`); authoring-time contract (per-run `contract.md` fill retired). Frontend catalog shelf = stage 2 (BACKLOG). **KM wiring 2026-07-17 (per the accepted `setup-operating-docs-scaffolding` record):** the agenda is now **ten dimensions** (knowledge-management at #7 — default-on module offer with the brownfield collision beat); G5 scaffolds the adopted operating-docs bundle on a hard never-overwrite floor (setup's first repo-root writes, deliberate) and suggests `/mochiko:brainstorm` alongside `/mochiko:specify`; amend offers unruled modules **once**, the answer recorded in `governance-intent.md` either way. **REWRITTEN v3 2026-07-18 (constitution dissolution, per the accepted `constitution-native-surfaces` record, D1–D8):** no `constitution.md` — the deliverable is the governance **surface set** (marked CLAUDE.md governance region · `paths`-scoped `.claude/rules/mochiko/` files · skill pointers · `.mochiko/memory/governance-ledger.md`) plus the trace summary manifest, graded by **trace closure**; a superseded `constitution.md` is deleted on sight (D6, no backcompat); the region regenerates idempotently, user content never touched (D8); the team section rewritten in brainstorm's **seat/messaging idiom** (build-time ruling after the D7 subagents-not-teams dogfood defect — dispatch vocabulary removed). |
| `[x]` | `specify` | Ported 2026-06-27 — adversarial 2-member team (analyst↔advocate), kernel-free; default-FAIL loop + NEW human acceptance gate; `state-analyst` dissolved into the lead; strategy skills deduped into `loop-discipline`. Core-only scope. **Re-transformed to thin/altitude shape 2026-06-30 (329→66 lines; independently verified PASS) — see `.mochiko/brainstorms/command-altitude/synthesis.md`.** |
| `[x]` | `plan` | Ported 2026-07-01 — thin/altitude analysis→design loop (`plan.md`, 82 lines; independently verified PASS). **First cluster to run convention-5's two-form** (two distinct validators: `principal-architect`+`validation-feasibility` feasibility × `devils-advocate`+`validation-plan-artifacts` completeness), and the **first net-new command since the altitude fix** — thin by construction. Producer `technical-analyst`; lead owns verdict + 4 gates HIL lacked. Full run record: `.mochiko/transform/plan/`. |
| `[x]` | `tasks` | Ported 2026-07-01 — thin/altitude Mapping→Tasks loop (`tasks.md`, 77 lines; independently verified PASS, single round). **RQ-A resolved = keep two artifacts** (`task-mapping.md` source of truth, `tasks.md` table derived echo). Single-reviewer **specify shape** (`task-architect` produces ↔ `devils-advocate`+`validation-task-artifacts` grades); lead owns verdict + 4 gates HIL lacked (default-FAIL, lead-owned verdict, cap+`TASKS_STOP`, G5 acceptance). **Decoupling doctrine held on the run's hardest surface** (task-architect: 5 sibling-names + phase-mode + context-file all cut, zero residual). Full run record: `.mochiko/transform/tasks/`. |
| `[x]` | `implement` | Ported 2026-07-01 — thin **sequential** sound-loop (`implement.md`, 80 lines; independently verified PASS, single round). **Sequential-first per the orchestration decision** — `Task`-subagent dispatch under a thin lead (dependency-ordered cycles, as HIL runs); native `pipeline()`/`parallel()` + any kernel/DAG deferred to dogfooding (a `deliberate-shortcut-ledger` entry, not a silent drop). Producer `staff-engineer` ↔ independent **Tier-1** validator `qa-engineer` (real-infra + exit codes); lead owns verdict + confidence-based per-cycle gate + named final-acceptance gate. **5th/final command port — altitude rollout complete.** Full run record: `.mochiko/transform/implement/`. |
| `[x]` | `brainstorm` | **NEW — net-new, not a HIL port (2026-07-02). REWRITTEN v2 as the agent-team pilot (2026-07-04). REVISED v2.1 to the end-stage review pair (2026-07-05)** after transcript forensics on the first v2 run: the standing episodic advocate generated 3:1 machine-to-user traffic (half of it bookkeeping wakes, each a full teammate turn), folded amendments into user-ruled decisions behind unanswered objection windows, used teammate↔teammate messaging zero times, and measured more expensive than v1's dispatches (I-4 closed). v2.1 keeps the frame — **native agent teams** (hard-requires `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` — `Contested`, no fallback transport); lead runs `analysis-iterative` inline and writes `record.md` as-you-go (one ID namespace; **the lead's pen covers only its own formulation** — nothing amends a user-ruled decision without the user's word); **fact-checker** (né *grounder* — renamed 2026-07-05 after the name misread as a findings-tester at its first fresh-run spawn) = conditional seat, spawned at session start when the topic has a reality surface (neutral: reports what IS; settles the reviewers' fact disputes) — and moves ALL adversarial pressure to convergence: a **review pair** (two cold reviewers, each's counterpart withheld until its independent findings are formed) cross-examines each other — owner-withdrawal only, never veto — and returns survivors + a tally ("N raised, M survived"; fallen retrievable on ask). Survivor routing (user decisions → user · lead reasoning → two exchanges → tie-break · facts → fact-checker) → dispositions (resolved / user-ruled / recorded-open) → one verify pass → plain-text acceptance; done = every survivor dispositioned + user accepts (reachable, default-FAIL); contract embedded authoring-time; the record is the deliverable, pipeline entry an offer. **RE-SIZED v2.2 to the sized lens-split review (2026-07-16)** after transcript forensics on the first *completed* run (≈654k out, level with both baselines; the pair's 270k dominated by a triple-read of the reality surface the seated fact-checker had already mapped; the two same-briefed reviewers specialized on their own; headline finding M1 = lead-paraphrase relay infidelity over the checker's map): review sizing = **named human gate at convergence** (pair / single / none — "none" writes a review waiver into the Review section; single mode names the sole reviewer as verify owner over the lead's folds); pair spawns with **lens briefs** (decision-quality / record-integrity — the skill stays one document); the fact-checker's map lands **verbatim** as a checker-authored record section and is the review's fact substrate (integrity lens sample-audits it against files); cross-exam is **one-shot** (four messages, fact-route dedup); cross-set merge/tally lead-owned; verify on the integrity reviewer only; `synthesis.md` sanctioned on-request post-acceptance, stamped derived + fidelity-sampled by the verify reviewer ("derived, unchecked" when review was waived). Design records: `.mochiko/brainstorms/brainstorm-command-rewrite/record.md` (v2) · `.mochiko/brainstorms/brainstorm-v2-revision/record.md` (v2.1) · `.mochiko/brainstorms/brainstorm-v2-2-revision/record.md` (v2.2). **Index bookkeeping added 2026-07-17 (the knowledge-management module's command carrier):** read `.mochiko/brainstorms/index.md` before opening · entry on open · status update at acceptance/supersession naming where the outcome landed · the module's invariants run at session open/close under fix-on-sight; graceful no-module branch when the layer was declined. |
| `[x]` | `slice` | **NEW — net-new, not a HIL port (2026-07-02).** Thin sound-loop (`slice.md`) decomposing an accepted spec into **graduation slices** — ordered story groups that then run `plan → tasks → implement` **per slice** instead of whole-spec (vertical graduation substrate; the `/mochiko:graduate` super-command was deliberately deferred to dogfooding). `task-architect` (`authoring-slices`) authors the `slices.md` overlay ↔ `devils-advocate` (`validation-slices`) grades it; lead owns verdict + G5 acceptance; **null-exit-aware** (small specs → reviewed whole-spec recommendation, no file). The overlay carries the **Graduation contract** (staleness stamp · shared-accumulating vs per-slice artifact layout · extend-mode · graded amendment) and the **Feature-Done declaration** (SC coverage map + seams — executed at feature-close, deferred to `audit`'s scoping). Slice-scoped entry variants landed on `plan`/`tasks`/`implement`. Design record: `.mochiko/brainstorms/vertical-graduation/synthesis.md`. |
| `[ ]` | `audit` | TBD — **charter acquired (2026-07-02):** feature-close verification of `slices.md`'s Feature-Done section (SC coverage + cross-slice seams against real infra); audit's scoping decides whole-workflow vs branch |
| `[-]` | `techspec` | Deprecated in HIL (merged into `plan`); **confirmed excluded (plan port, 2026-07-01)** — `plan` IS the merged form; techspec's `integrations.md`/`data-sensitivity.md` fold into `data-model.md` (sensitivity) + `contracts/api.yaml` (x-integration) |

---

## Agents

| Status | Name | Workflow Affinity |
|--------|------|-------------------|
| `[x]` | `principal-architect` | `setup` producer (authoring-constitution, analysis-codebase) **+ `plan` feasibility reviewer** (validation-feasibility, re-broadened 2026-07-01) — produce-in-setup / review-in-plan (decoupling-legit); **G1: never re-mounts `validation-constitution`** (no constitution self-grade) |
| `[x]` | `validator` | mochiko-native generic grader (see top table) — one independent grader serves `setup` (validation-constitution) + transform (verify-output); replaces the setup-specific `constitution-validator` born from the principal-architect split |
| `[x]` | `devils-advocate` | **cross-workflow reviewer** — `specify` critic (`analysis-specifications`) + `plan` completeness reviewer (`validation-plan-artifacts`) + `tasks` task-artifact reviewer (`validation-task-artifacts` **re-mounted live 2026-07-01** — the last stubbed mount now closed) + `brainstorm` **end-stage reviewer(s)** (`validation-brainstorm`, net-new mount 2026-07-02; v2.1 2026-07-05; v2.2 2026-07-16 — spawned cold at convergence per the user's sizing ruling: a lens-split pair by default (decision-quality / record-integrity), single for a lean record; counterpart withheld until independent findings are formed, then a one-shot four-message cross-examination → survivors only; teammates ignore `skills:` frontmatter, so the spawn prompt names the skill + role + lens) + `slice` decomposition reviewer (`validation-slices`, net-new mount 2026-07-02 — coverage / closure / foundation legitimacy / Feature-Done / depth) |
| `[x]` | `requirements-analyst` | `specify` — producer (skills: `authoring-requirements`, `authoring-user-stories`) |
| `[x]` | `technical-analyst` | `plan` — producer (skills: authoring-technical-requirements, patterns-technical-decisions, patterns-entity-modeling, patterns-api-contracts); authors the 6 analysis+design artifacts, never grades |
| `[x]` | `task-architect` | **cross-workflow producer** — `tasks` (skills: `patterns-vertical-tdd`; authors `task-mapping.md` + `tasks.md`) **+ `slice` (2026-07-02:** `authoring-slices`; authors the `slices.md` graduation-slice overlay — the same slicing judgment one level up), never grades. Ported 2026-07-01; report → `taskarchitect-report-template` / `slicer-report-template` (no self-verdict). The tasks run's heaviest port + hardest decoupling surface (zero residual deny-list tokens) |
| `[x]` | `staff-engineer` | `implement` producer (skills: `executing-tdd-cycle`, `brownfield-integration`) — writes code via TDD, emits `cycle-report.md`, never grades. Ported 2026-07-01; fix-mode decoupled (routing/trigger/max-passes → lead; intrinsic craft kept). Decoupling held on the run's hot surface ("Two Execution Modes" dissolved, zero residual) |
| `[x]` | `qa-engineer` | `implement` **independent Tier-1 validator** (skill: `testing-end-user`) — verifies against real infra + quality-gate exit codes (deterministic), emits verification report + checkpoint, never produces. Ported 2026-07-01; the strongest (Tier-1 deterministic) validator of any cluster. **Also serves `audit`** (affinity) — reclaim when that cluster ports |
| `[-]` | `state-analyst` | **Dissolved (kernel-free)** — specify slice absorbed into the `specify` command lead; non-kernel judgment deduped into `loop-discipline`; no standalone agent. plan/tasks/implement orchestration handled when those clusters port. |
| `[ ]` | `ui-designer` | `specify`, `plan` — design-track; **deferred** from specify-core (catalog never invokes it) |

---

## Skills

### Meta / foundational
| Status | Name | Notes |
|--------|------|-------|
| `[-]` | `strategy-core` | **Dissolved** — near-total dedupe into `loop-discipline` (specify run); **Gap Classification** taxonomy folded into `loop-discipline` |
| `[-]` | `strategy-specification` | **Dissolved** — dedupe into `loop-discipline`; survivors (Input Assessment, targeted-revision, spec done-condition) → specify lead |
| `[-]` | `strategy-implementation` | **Dissolved** into `loop-discipline` (doctrine) + the implement lead (params) (implement port, 2026-07-01) — 3rd/final strategy dissolution (after `strategy-core` + `strategy-specification` in specify); zero residual, no `loop-discipline` edit needed. **Strategy family now fully dissolved** — no `strategy-*` skill survives for any workflow |
| `[ ]` | `syncing-claude-md` | Cross-cutting — referenced by `setup` as a documented stub (not yet ported); port with its own cluster |
| `[ ]` | `using-git-worktrees` | Cross-cutting utility |
| `[ ]` | `using-github-issues` | Cross-cutting utility |

### Setup workflow cluster
| Status | Name | Notes |
|--------|------|-------|
| `[x]` | `authoring-constitution` | Core of setup — **absorbed `brownfield-constitution`** as a greenfield\|brownfield branch; canonical `ESSENTIAL-FLOOR.md` home. **Rewritten 2026-07-16 (setup v2):** authors FROM the ratified synthesis as traceable contract (formulation-only latitude; trace stamps; flagged-proposals channel incl. can't-formulate intent); module assembly (core + selected modules); floor retiered (principle **or recorded waiver** per category); greenfield fixed V–VII defaults dissolved into the **catalog** (`references/catalog/` — README + universal-floor + backend-service shelves; `RECOMMENDED-PATTERNS.md` retired into the backend/service shelf); `references/INTERROGATION-AGENDA.md` added (the lead's nine-dimension session agenda). **2026-07-17:** the agenda grows to **ten dimensions** (knowledge-management at #7, with the brownfield collision beat); module assembly gains `knowledge-management` (default-on, adopted or declined whole; a recorded decline is durable). **Rewritten v3 2026-07-18 (dissolution):** authors the **surface set** with **surface routing** (universal → CLAUDE.md region lines; scope-bound → `paths`-scoped rules files; procedure → skill pointers; Three-Part metadata → ledger keyed by GI-ID) + the trace summary manifest; module content routes by surface; the CLAUDE.md-Synchronization section and `syncing-claude-md` stub retired — governance lives ON CLAUDE.md, nothing left to sync |
| `[x]` | `brownfield-constitution` | **Merged into `authoring-constitution`** (greenfield\|brownfield branch) — no standalone skill |
| `[x]` | `validation-constitution` | Body kept verbatim; **promoted** onto the independent generic `validator` agent. **Extended 2026-07-16 (setup v2):** deterministic trace-ID cross-check vs `governance-intent.md` (both directions; semantic fidelity recorded as judgment-grade residual risk, advisory only); tier + waiver-format + floor-accounting checks (principle-or-waiver, all modes — the F5 fix); module-parameterized section checklist (core + selected modules' embedded fragments). **Extended v3 2026-07-18 (dissolution):** grades the surface **set** — region/surface integrity (markers, index→home resolution, `paths` scoping, universal-in-rules = FAIL), two-way trace **closure** over the manifest (primary home + companion entries), three-part structure in the ledger |
| `[x]` | `analysis-codebase` | Setup-brownfield slice + `detect-stack.sh` (verbatim); collision/spec-plan modes deferred to spec/plan cluster; Essential-Floor deduped to authoring's canonical ref. **2026-07-16 (setup v2):** assessment declared **tier-blind/waiver-blind** — reports what IS; tier interpretation (confrontation vs waiver) belongs to the session downstream |

### Specify workflow cluster
| Status | Name | Notes |
|--------|------|-------|
| `[x]` | `analysis-specifications` | Ported — the advocate's gap-analysis procedure (stays a gap-finder; verdict owned by the lead) |
| `[x]` | `analysis-iterative` | Ported — enrichment + general brainstorm (dual-mode kept; marked general/shared in router); `/mochiko:brainstorm`'s questioning engine (2026-07-02). **+ 4th question format `Recommend-then-Arbitrate` (2026-07-04, dogfood R10):** when the user lacks domain expertise, supply the domain content and let them sort/arbitrate — distinct from structured-options / open-probe / confirmation |
| `[-]` | `strategy-specification` | Dissolved (see Meta/foundational section) |

### Plan / technical design cluster
| Status | Name | Notes |
|--------|------|-------|
| `[x]` | `authoring-requirements` | Ported **via specify** (re-filed plan→specify) — analyst's FR-XXX / SC-XXX authoring |
| `[x]` | `authoring-technical-requirements` | Ported (plan, 2026-07-01) — owns TR/C/NFR/IP + `constraints-and-decisions.md` + C↔D/IP traceability; DS→`patterns-entity-modeling`, x-integration→`patterns-api-contracts`, technique→`patterns-technical-decisions` (thin declarations + refs) |
| `[x]` | `authoring-user-stories` | Ported **via specify** (re-filed plan→specify) — analyst's P1/P2/P3 + Given/When/Then |
| `[x]` | `patterns-api-contracts` | Ported (plan) — API contracts (OpenAPI) + **x-integration boundary authoring** (added — canonical home; repaired the dangling authoring-technical-requirements ref) |
| `[x]` | `patterns-entity-modeling` | Ported (plan) — data-model + **canonical data-sensitivity 4-level taxonomy** (absorbed from authoring-technical-requirements; single `data-model.md` template) |
| `[x]` | `patterns-technical-decisions` | Ported (plan) — decision technique / ADR; references authoring-technical-requirements for the `constraints-and-decisions.md` artifact (boundary + dedupe) |
| `[x]` | `validation-feasibility` | **NEW — net-new, not a HIL port (plan, 2026-07-01).** Feasibility reviewer's adversarial-critique skill on `principal-architect`; cross-artifact contradiction / impossibility / buildability → 3-state incl. `infeasible`; the convention-5 **two-form** partner of `validation-plan-artifacts` (homes the feasibility review the setup PA port disclaimed) |
| `[ ]` | `authoring-roadmap` | Roadmap authoring — **deferred** (setup-brownfield track, not plan-core) |
| `[ ]` | `patterns-flow-mapping` | Flow design — **deferred** (design track / ui-designer, not plan-core) |
| `[ ]` | `patterns-interface-design` | UI patterns — **deferred** (design track / ui-designer, not plan-core) |
| `[x]` | `patterns-vertical-tdd` | Ported (tasks, 2026-07-01) — the producer's vertical-slice + TDD structuring skill; **canonical home of the `**TEST:**` verification-task grammar** (folded from the task-architect persona); conforms to `tasks-template` as the canonical `tasks.md` structure; design-time boundary vs deferred `executing-tdd-cycle` (implement) held |

### Implement / execute cluster
| Status | Name | Notes |
|--------|------|-------|
| `[x]` | `executing-tdd-cycle` | Ported (implement, 2026-07-01) — `staff-engineer`'s core: runtime red/green/refactor + retry/fix HOW-craft + `cycle-report.md` format; the **runtime** side of the design↔runtime boundary vs `patterns-vertical-tdd` (design-time structuring) — held clean (`**TEST:**`=0, no duplicated substance); retry/fix routing deduped → `loop-discipline`, params → lead |
| `[x]` | `validation-plan-artifacts` | Ported **via plan** (2026-07-01; re-filed implement→plan-cluster) — completeness mirror-checklist on `devils-advocate`; **absorbed `cross-artifact-checklist.md`**; feasibility checks deduped → `validation-feasibility` |
| `[x]` | `validation-task-artifacts` | Ported (tasks, 2026-07-01) — the reviewer's task-artifact gap-finder on `devils-advocate`; gap report + recommended 3-state (**verdict lead-owned**, not clearing PASS/FAIL); embedded report templates deduped → `advocate-report-template`; disjoint boundary vs `validation-plan-artifacts` (task vs plan artifacts); two-phase Mapping/Tasks/Cross-Artifact checklists kept (Branch A); mirrors `patterns-vertical-tdd` |
| `[x]` | `testing-end-user` | Ported (implement, 2026-07-01) — `qa-engineer`'s core (independent **Tier-1** validator): detect/parse/execute `**TEST:**` tasks against real infra, evidence capture, result classification, deterministic quality-gate exit codes; **reclaims the parked CLI/GUI/SUBJECTIVE runtime-classification** (auto-approve vs human-checkpoint) from the tasks port; `**TEST:**` grammar deduped → `patterns-vertical-tdd` (canonical owner), keeps execution/parse semantics |
| `[x]` | `brownfield-integration` | Ported (implement, 2026-07-01) — **mis-file corrected — home confirmed implement** (staff-engineer's 2nd skill); EXTEND/MODIFY consumption + read-before-write + interface preservation + conflict detection; `[EXTEND]`/`[MODIFY]` marker vocabulary deduped → `patterns-vertical-tdd` (canonical owner), keeps implement-time consumption/interface-impact semantics; 0 drops |

### Brainstorm workflow cluster
| Status | Name | Notes |
|--------|------|-------|
| `[x]` | `validation-brainstorm` | **NEW — net-new (brainstorm, 2026-07-02); REWRITTEN v2.1 for the end-stage review pair (2026-07-05; v2's two live-team roles retired with the standing advocate); REVISED v2.2 for the sized lens-split review (2026-07-16).** One role, sized instances (lens-briefed pair by default, or solo): **end-stage reviewer** — independent cold read FIRST (scenario stress per decision, the five hunt classes, reality-grounding against the record's fact-checker map — sample-audited against actual files on the record-integrity lens; no map → the files directly — and the `references/RECORD-FITNESS.md` standalone-record checklist) with the counterpart withheld until findings are formed; the lens (decision-quality vs record-integrity) is dispatch-level — the skill stays one document, the brief sets depth, not jurisdiction; then the **one-shot four-message cross-examination** (attack the counterpart's findings, defend or **owner-withdraw** yours — persuade, never veto; fact disputes → fact-checker, a fact the counterpart already routed is cited, never re-routed; unresolved disagreements survive with the objection attached); returns **own** survivors + tally ("N raised, M survived"; cross-set merge/combined tally lead-owned) + recommended 3-state (verdict lead-owned); the verify pass over the folds runs when assigned (integrity lens in a pair, automatic when solo) plus the fidelity sample of an on-request `synthesis.md`. Findings return as messages (no report files). The four destination checklists (`READINESS-CHECKLISTS.md`) retired with v1's ramps |

### Slice workflow cluster
| Status | Name | Notes |
|--------|------|-------|
| `[x]` | `authoring-slices` | **NEW — net-new, not a HIL port (slice, 2026-07-02).** The producer's graduation-slice decomposition craft on `task-architect`: the slicing invariants (exactly-one-home · dependency closure · foundation legitimacy · ordering · coverage · overlay purity), sizing, cross-cutting earliest-home + extend obligations (un-homeable → spec-amendment finding), the null exit, the Feature-Done declaration (SC coverage map + seams), and the spec stamp; conforms to `slices-template` (canonical `slices.md` structure). Explicit two-level vocabulary boundary vs `patterns-vertical-tdd` (graduation slice = spec-level pipeline unit; vertical slice = cycle) |
| `[x]` | `validation-slices` | **NEW — net-new, not a HIL port (slice, 2026-07-02).** The reviewer's decomposition gap-finder on `devils-advocate` — 13 named checks (coverage, closure, foundation legitimacy, sizing, journey coherence, obligations, SC coverage, seams, stamp, purity, depth-both-directions) mirroring what `authoring-slices` teaches the producer; reads BOTH `slices.md` and `spec.md` (set comparisons, not impressions); grades null-exit depth calls too; carries the same explicit two-level vocabulary boundary (graduation slice ≠ vertical-slice cycle) as its producer mirror; severity-classified gaps + recommended 3-state (**verdict lead-owned**); reuses `advocate-report-template` |

### Analysis / design cluster
| Status | Name | Notes |
|--------|------|-------|
| `[ ]` | `analysis-screenshot` | Screenshot / design extraction |
| `[ ]` | `authoring-design-system` | Design system authoring |

---

## Templates

| Status | Name | Workflow Affinity |
|--------|------|-------------------|
| `[x]` | `constitution-template.md` | `setup` — ported (`approved-domain-deps.md` ref softened to prose). **Split 2026-07-16 (setup v2)** into the **universal core** (+ Governance Tier section w/ waiver records + trace stamps + tier-parameterized Quality Gates) with type/tier sections moved to `constitution-modules/` (`layer-rules`, `release-gates`, `evolution-notes` — each carrying its own validator checklist fragment); the backend-flavored conditional sections (Project Structure / Layer Import Rules) now live only in the `layer-rules` module. **2026-07-17:** Mandatory Sync Artifacts table gains the knowledge-management rows (conditional on adoption; stub-backed until `syncing-claude-md` ports). **RETIRED 2026-07-18 (setup v3, dissolution):** replaced by `governance-surfaces-template.md` (region block · rules-file · ledger · manifest shapes); `constitution-modules/` survive as content sources, routed by surface per the authoring skill's routing table |
| `[x]` | `governance-intent-template.md` | **NEW — net-new, not a HIL port (setup v2, 2026-07-16).** The session synthesis fill-target: tier declaration, deck rulings, minted intents, waivers, module selections, real commands — all GI-ID'd; persisted durably at `.mochiko/memory/governance-intent.md`, delta-updated on amend (Amendment Log). **2026-07-17:** module selections record **declines** too (durable rulings — amend offers only unruled modules, once); dimension references renumbered for the ten-dimension agenda |
| `[x]` | `governance-surfaces-template.md` | **NEW — net-new, not a HIL port (constitution dissolution, 2026-07-18).** The canonical shapes of the dissolved constitution: the marked CLAUDE.md governance-region block (stamp · index · universal principles · stack · gates summary · pointers), the `paths`-scoped rules-file shape, the governance-ledger shape (Three-Part metadata keyed by GI-ID · waivers · amendment policy · exception registry · amendment log), and the trace-summary manifest. Design: `.mochiko/brainstorms/constitution-native-surfaces/record.md` |
| `[x]` | `constitution-modules/knowledge-management.md` | **NEW — net-new, not a HIL port (operating-docs scaffolding, 2026-07-17).** Elective **default-on** constitution module: the four-part operating-docs bundle (`.mochiko/brainstorms/<slug>/` session org + `index.md` with the maintenance contract, repo-root `BACKLOG.md` + `ROADMAP.md` with circulation rules) adopted or declined **whole**; three enforcement carriers (command index-bookkeeping + open/close invariants · CLAUDE.md sync rows · setup/amend re-audit); mechanical invariants with the content-quality exemption declared; never-overwrite scaffolding floor + collision-rulings slot; explicitly disambiguated from `.mochiko/memory/evolution-roadmap.md`; carries its own validator checklist fragment. Design: `.mochiko/brainstorms/setup-operating-docs-scaffolding/record.md` |
| `[-]` | `constitution-context-template.md` | `setup` — **absorbed into the setup lead** (in-session + `.mochiko/memory/` state); not a standalone artifact |
| `[x]` | `codebase-analysis-template.md` | `setup` — ported verbatim |
| `[ ]` | `codebase-inventory-schema.json` | **Deferred to spec/plan cluster** — orphan in setup (no consumer after mode-scoping) |
| `[x]` | `spec-template.md` | `specify` — ported (8 body slots verbatim; header rebound to workspace-as-state) |
| `[x]` | `analyst-report-template.md` | `specify` — ported (iteration→round; counts demoted; DAG trajectory dropped) |
| `[x]` | `advocate-report-template.md` | **shared (specify + plan)** — ported via specify (keep-verbatim; 3-state verdict); **reused as-is by plan's completeness review** (RQ4, 2026-07-01 — no new file) |
| `[-]` | `context-template.md` | **Absorbed into the lead** (in-session + `.mochiko/specs/<feature>/` workspace-as-state); peers inherit — not a standalone artifact |
| `[x]` | `plan-template.md` | Ported (plan, 2026-07-01) — the `plan.md` deliverable; the lead's fill-target; `[...]` model-fill style (not `{{}}`) |
| `[-]` | `plan-context-template.md` | **Absorbed into the plan lead** (memory-model — 3rd confirmation; in-session + `.mochiko/specs/<feature>/` workspace-as-state; state recovery reads workspace evidence) — not a standalone artifact |
| `[x]` | `tasks-template.md` | Ported (tasks, 2026-07-01) — the `tasks.md` deliverable skeleton; **the canonical `tasks.md` structure** (`patterns-vertical-tdd` conforms to it); `[...]` model-fill, frontmatter dropped; each cycle ends in a `**TEST:**` real-infra verification task (P3↔P5 alignment fixed this run); Story→Cycle table = derived echo of `task-mapping.md` |
| `[-]` | `tasks-context-template.md` | **Absorbed into the tasks lead** (memory-model, 4th confirmation — in-session + `.mochiko/specs/<feature>/` workspace-as-state; state recovery reads workspace evidence; the cross-workflow plan-complete entry gate rebound to `plan.md` presence) — not a standalone artifact |
| `[x]` | `taskarchitect-report-template.md` | **NEW (tasks, 2026-07-01)** — the `task-architect` producer's per-round non-verdict self-disclosure report (mirrors `techanalyst-report-template` + tasks-specific Vertical-Slice-Rationale / TDD-Structure sections); **no `Completion`/`Ready-for-Review` field** (lead owns the verdict) |
| `[ ]` | `evolution-roadmap-template.md` | `plan`-filed but **deferred** (roadmap track, not plan-core) |
| `[x]` | `architect-report-template.md` | Ported (plan) — **renamed `feasibility-report-template.md`**; feasibility reviewer's report; 3-state incl. `infeasible`; Next-Steps routing lifted to lead; `type:` DAG frontmatter dropped |
| `[x]` | `techanalyst-report-template.md` | Ported (plan) — producer self-disclosure; **`completion_status` dropped** (independence — verdict is lead-owned); iteration→round; brain-era counts demoted |
| `[-]` | `cross-artifact-checklist.md` | **Folded into `validation-plan-artifacts`** (plan; orphan — no consumer + near-total dup; the tasks validator self-contains its own) |
| `[ ]` | `approved-domain-deps.md` | `implement` — deferred (governance/constitution track; reference-stub) — confirmed out of implement-core scope (implement port, 2026-07-01) |
| `[-]` | `handoff-brief-template.md` | **RETIRED (brainstorm v2, 2026-07-04)** — was the v1 tasks/direct-execution ramp deliverable; v2 removed the ramps (record is the deliverable, pipeline entry an offer), so the template and its consumer (the `/mochiko:tasks` stamped design-light entry variant) were both removed — no orphan left behind |
| `[x]` | `slices-template.md` | **NEW (slice, 2026-07-02)** — the `slices.md` deliverable skeleton; **canonical `slices.md` structure** (`authoring-slices` conforms, `validation-slices` grades against it); `[...]` model-fill; carries the Spec stamp (staleness guard), the Feature-Done section (declared at decomposition), and the **Graduation contract** — the single source of the downstream consumption rules the three slice-scoped entry variants honor; no status/stage fields (workspace-as-state) |
| `[x]` | `slicer-report-template.md` | **NEW (slice, 2026-07-02)** — the slice producer's per-round non-verdict self-disclosure (mirrors `taskarchitect-report-template`; slicing-specific Slicing-Rationale / Feature-Done-Coverage sections; the null-exit outcome is disclosed here — no `slices.md` written); **no `Completion`/`Ready-for-Review` field** (lead owns the verdict) |

---

## Catalogs

| Status | Name | Notes |
|--------|------|-------|
| `[-]` | `specify-catalog.json` | Brain-mediated DAG — excluded (kernel-free) |
| `[-]` | `implement-catalog.json` | Brain-mediated DAG — excluded (kernel-free) |

---

## Scripts

| Status | Name | Notes |
|--------|------|-------|
| `[-]` | `check-prerequisites.sh` | Brain/MCP prerequisites — excluded |
| `[-]` | `common.sh` | Brain/MCP common — excluded |
| `[-]` | `create-new-feature.sh` | Brain-mediated — excluded |
| `[-]` | `setup-plan.sh` | Brain-mediated — excluded |

---

_Update this file as primitives are ported. Change `[ ]` to `[x]` when landed, `[~]` if ported but needs a v3 upgrade pass, `[-]` if deliberately excluded._
