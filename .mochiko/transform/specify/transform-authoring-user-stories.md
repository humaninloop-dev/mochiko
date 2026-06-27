# TRANSFORM: authoring-user-stories (P8)

Run: `transform-cluster specify` · Phase 3 (transform) · Producer: `transform-producer` · Skill: `mochiko:transform-recipes`
Transformed: 2026-06-27 · Governed by `loop-discipline`
Disposition (finalized at reconcile, human-gate-accepted): **`port-with-edits` × `standalone`** (keep-distinct from P7; the P8 `flag-for-reconcile` resolved → `standalone` at reconcile §A Agenda 4a / §D row P8 / §E P8).

> **Transform applies a decision; it does not make one.** This file records applying the §D/§E/§G calls to the whole HIL skill directory. Grading is `verify-output`, run by a different agent — **not graded here.**

---

## Applied

**`port-with-edits` × `standalone` + convention-wiring pass.**

- **Body = `port-with-edits`.** The body is mochiko-clean (CLEAN decouple scan: 0 agent names, 0 "dispatch", 0 kernel tokens — confirmed by grep on the artifact). All edits are **wiring**, not content rewrites: (1) description triggers → work-context + de-collide; (2) genericize two dangling deferred design-track links. The discipline core (format, priority taxonomy, Given/When/Then, examples, anti-rationalization, validation script) is ported faithfully.
- **Structural = `standalone`.** Placed as-is in its own directory; both authoring skills stay mounted on the `requirements-analyst` producer. **Keep-distinct: the shared discipline substrate with `authoring-requirements` (P7) is NOT factored** (reconcile §A 4b — factoring would manufacture a cross-skill coupling against kernel-free minimalism). This skill keeps its own prose and its own script.
- **New partners:** none. The producer↔validator pair already exists (`requirements-analyst` ↔ `devils-advocate`); no validator is spun out for this skill (reconcile §C).

---

## Artifacts written

| Path | Action | Fidelity |
|---|---|---|
| `plugins/mochiko/skills/authoring-user-stories/SKILL.md` | CREATE (port-with-edits) | Byte-identical to HIL except 2 wiring edits (description line 3; two `When NOT to Use` links lines 29–30) — confirmed by line-diff |
| `plugins/mochiko/skills/authoring-user-stories/references/PRIORITY-DEFINITIONS.md` | CREATE (verbatim) | Byte-identical to HIL (`diff -q` IDENTICAL) |
| `plugins/mochiko/skills/authoring-user-stories/references/EXAMPLES.md` | CREATE (verbatim) | Byte-identical to HIL (`diff -q` IDENTICAL) |
| `plugins/mochiko/skills/authoring-user-stories/scripts/validate-user-stories.py` | CREATE (verbatim) | Byte-identical to HIL (`diff -q` IDENTICAL) |

**Whole-directory port confirmed:** HIL source dir held exactly these four files (SKILL.md + 2 references + 1 script); all four are ported.

---

## Convention-wiring pass (all five)

1. **Classification → model-invoked.** Frontmatter is `name` + `description` only; no `disable-model-invocation: true` (model-invoked is the mochiko default). Agent-consumed by `requirements-analyst`. Matches §G classification row (P5/P6/P7/P8 → model-invoked).
2. **Router registration → DEFERRED to the dedicated router-edit step.** Not done here by instruction ("DO NOT edit the shared router"). §D row `skills/mochiko/SKILL.md (router) — EDIT` and §G own this; the entry to add is `authoring-user-stories` (model-invoked, specify cluster). Recorded so it is not silently skipped — it is another artifact's responsibility, not a drop.
3. **Trigger phrasing → work-context + de-collide.** Description rewritten from "when the user says …" to transformation-work context. **De-collision (reconcile §A 4c / §G):** this skill KEEPS the triggers `acceptance scenarios` / `Given/When/Then` / `user story`(`ies`) / `priority` (`P1/P2/P3`) / `backlog` / `independent test`. The colliding bare phrase **"acceptance criteria" is removed from the triggers** (replaced by the precise "acceptance scenarios"); P7 `authoring-requirements` drops "acceptance criteria" and anchors on FR-/SC-/functional-requirements. Verified: the string "acceptance criteria" appears only in the Overview **body prose** (line 12, "measurable acceptance criteria") — kept as this skill's own prose; it is NOT a description trigger, so no auto-trigger collision remains.
4. **Path rebinding → no live `humaninloop:`→`mochiko:` rebind survives.** The only `humaninloop:` references in the HIL body were the two deferred design-track links (below), which are **scrubbed, not rebound**. No `.humaninloop/` paths existed. The script invocation is a clean bundle-relative path (`python scripts/validate-user-stories.py path/to/spec.md`) — kept. (Note for verify-output: do not expect a live `mochiko:` sibling link in this body — there is no surviving cross-skill link to rebind.)
5. **Decouple persona/skill → already clean; confirmed by grep.** No sibling-agent names, no "dispatch", no injected workflow modes/paths/phases, no "workflow-agnostic" label in the artifact. The one keystone-passing **false positive is intentionally kept** (see Notes).

---

## The two body edits (detail)

### Edit 1 — description triggers → work-context + de-collide
- **Before:** `This skill MUST be invoked when the user says "write user stories", "define acceptance criteria", "prioritize features", "user story", "acceptance scenario", or "Given When Then". SHOULD also invoke when user mentions "priority", "P1", "P2", "P3", or "backlog". …`
- **After:** `This skill MUST be invoked when transforming a feature description into prioritized user stories — assigning P1/P2/P3 priority levels, authoring Given/When/Then acceptance scenarios, and specifying an independent test for each story. SHOULD also invoke when the work involves a user story, story prioritization (P1/P2/P3), acceptance scenarios, a feature backlog of independently testable stories, or breaking a large feature into separate, independently testable user journeys. Produces prioritized user stories with independently testable acceptance scenarios.`
- **Why:** agent-consumed skill → describe the transformation work, not "when the user says …" (avoids false auto-trigger expectations). De-collides the "acceptance criteria" overlap with P7 while keeping ownership of the acceptance concept via the precise "acceptance scenarios".

### Edit 2 — dangling deferred-cluster links scrubbed (genericized to prose)
The two design-track sibling skills are **DEFERRED this run** (context.md scope: design track deferred; §G "Deferred / dangling cross-refs to scrub"). Rebinding `humaninloop:`→`mochiko:` would point at unported skills (a dangling link → `verify-output` Tier-1 FAIL), so they are **genericized to prose** (the "when NOT to use" guidance itself is valid for any user-story job — keystone-passes — so the guidance is kept, only the dead links are removed). The two `When NOT to Use` bullets changed as follows:

```
BEFORE (HIL, lines 29–30):
- **Architecture decisions** - Use [humaninloop:patterns-technical-decisions] instead
- **API contract design** - Use [humaninloop:patterns-api-contracts] instead

AFTER (mochiko):
- **Architecture decisions** - Capture technical-decision rationale in the design/plan track
  instead; this skill authors user stories, not technical choices
- **API contract design** - Define endpoints and schemas in the design/plan track
  instead; this skill authors user stories, not interface contracts
```
(`[...]` above stands in for the inline-code link that was removed.)

- **Reason (for the lead, already accepted at the §F gate via the dropped-link reasons):** `patterns-technical-decisions` + `patterns-api-contracts` are deferred design/plan-track skills, out of specify-core scope this run. Genericized rather than dropped so the out-of-scope guidance survives; no domain capability lost.

---

## Realized responsibility trace (every responsibility carries a final tag)

Flipped from the assess trace (`assess-authoring-user-stories.md`) and reconcile §E P8. **No silent drops.**

### Owned by this skill
| Responsibility | Final tag | Realized landing |
|---|---|---|
| Transform feature descriptions → prioritized user stories (P1/P2/P3) — core procedure | **kept** | SKILL.md Overview + User Story Format (verbatim) |
| Priority taxonomy + assignment guidance (defns, decision tree, distribution) | **kept** | `references/PRIORITY-DEFINITIONS.md` (verbatim) |
| Given/When/Then acceptance-scenario grammar | **kept** | SKILL.md Acceptance Scenario Guidelines (verbatim) |
| Independent-test-per-story requirement | **kept** | SKILL.md Independent Test Requirement (verbatim) |
| Worked user-story examples | **kept** | `references/EXAMPLES.md` (verbatim) |
| Anti-rationalization discipline (Red Flags / Common Rationalizations / Common Mistakes) | **kept** | SKILL.md (verbatim) |
| Technology-agnostic + observable/measurable-outcome discipline (shared with P7) | **kept** — **NOT factored** (keep-distinct, reconcile §A 4b) | This skill's own prose; no shared-substrate file created |
| Deterministic structural format validation (Tier-1 self-check, kernel-free) | **kept** | `scripts/validate-user-stories.py` (verbatim, byte-identical; imports `json/re/sys/pathlib` only — kernel-free) |
| Script-invocation path | **kept** (no rebind needed — already bundle-relative) | SKILL.md Validation Script (`python scripts/validate-user-stories.py path/to/spec.md`) |
| Scope-partition cross-refs to deferred design-track skills | **kept-but-rebind** → resolved as **scrub-with-reason** | Genericized to prose (Edit 2); reason = deferred design/plan-track, out of scope this run |
| Classification + trigger reliability ("when user says" model-invoked) | **kept-but-rebind** | Re-tagged agent-consumed model-invoked; description → work-context; "acceptance criteria" de-collided vs P7 (Edit 1) |
| Router discoverability (none in HIL) | **kept-but-rebind** (new convention floor) | Deferred to the router-edit step (§D/§G) — entry: `authoring-user-stories`, model-invoked |

### Orchestration the skill is SUBJECT TO but does NOT own (rehomed via P1 lead / P4 absorb — not this skill's burden)
| Responsibility | Final tag | Realized landing |
|---|---|---|
| Sequencing (after input-enrichment, before advocate-review) | **moved-to-lead** | `specify` command lead (rehome map §B.1) |
| Loop-driving on advocate FAIL (re-brief analyst to revise stories) | **moved-to-lead** | lead's bounded produce→critique→revise loop (§B.1) |
| Input handoff (enriched feature description) + output sink (`## User Stories` section of spec.md) | **moved-to-lead** | explicit handoff edges in the contract (§B.5) |
| Independent validation of story *correctness* | **kept** at cluster level (no new validator) | provided by `devils-advocate`/`analysis-specifications` at the agent level (reconcile §C) |

**Drops:** none. The only removals are the two dead cross-links to deferred design-track skills, each carrying a reason (Edit 2). No domain capability lost.

---

## Notes for `verify-output` (so the scan does not trip)

- **Kept false positive (intentional).** `references/PRIORITY-DEFINITIONS.md:97` — "Many P3 stories: Consider deferring to a future phase" — is **product delivery phasing** of P3 stories (true of any spec job → keystone-passes), **not** an injected workflow phase. KEEP. The decoupling deny-list grep for "phase" will hit this line; it is an accepted false positive (assess §7 + this run's instruction).
- **`acceptance criteria` at SKILL.md:12** is **body prose** ("measurable acceptance criteria"), kept as this skill's own prose. It is not a frontmatter trigger, so it does not re-introduce the P7/P8 trigger collision. Not a defect.
- **No live `mochiko:` sibling link in this body** — by design (Edit 2 scrubbed the only cross-refs). Do not flag a missing rebind.
- **Router + loop-discipline edits are NOT in this artifact** by instruction; they are separate §D artifacts. Router registration of this skill is owed by the router-edit step, not skipped.

## Deviations

- **None from the finalized disposition.** Body × structural applied exactly as §D row P8 specifies. The single judgment within "rebind `humaninloop:`→`mochiko:`": the only `humaninloop:` refs were the two deferred links, so the rebind resolves to a **scrub** (genericize-with-reason) rather than a live re-point — this is exactly what §G's "Deferred / dangling cross-refs to scrub" row directs for P8, not a deviation.

---

## Output (transform-recipes format)

```
TRANSFORM: authoring-user-stories
Applied:   port-with-edits × standalone + wiring-pass
Artifacts: plugins/mochiko/skills/authoring-user-stories/{SKILL.md, references/PRIORITY-DEFINITIONS.md, references/EXAMPLES.md, scripts/validate-user-stories.py}
New partners: none (producer↔validator pair already exists: requirements-analyst ↔ devils-advocate)
Wiring:    classification=model-invoked  router=deferred-to-router-edit-step  triggers=work-context+de-collided("acceptance scenarios" kept; "acceptance criteria" removed from triggers)  rebinds=[2 deferred design-track links scrubbed-with-reason; no live mochiko: link survives]
Trace (realized): all kept / kept-but-rebind / moved-to-lead — 0 drops; shared discipline substrate NOT factored (keep-distinct); validate-user-stories.py ported verbatim (kernel-free)
```
