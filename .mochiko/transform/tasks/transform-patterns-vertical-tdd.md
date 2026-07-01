# TRANSFORM — `patterns-vertical-tdd` (P3, tasks cluster)

**Producer:** `mochiko:transform-producer` · **Skill:** `mochiko:transform-recipes` · **Date:** 2026-07-01
**Consumes:** `assess-patterns-vertical-tdd.md` (P3) + `reconcile.md` (Flag 3, Flag 5a, Flag 5b, Flag 7.4/7.5, P3 trace L223–228).
**Reference model:** `plugins/mochiko/skills/patterns-entity-modeling/SKILL.md` (description shape + progressive disclosure).
**Do NOT grade this output.** Grading is `verify-output`, run by a different agent.

```
TRANSFORM: patterns-vertical-tdd
Applied:   port-with-edits × standalone + convention-wiring pass
Artifacts: plugins/mochiko/skills/patterns-vertical-tdd/SKILL.md
           plugins/mochiko/skills/patterns-vertical-tdd/references/CYCLE-STRUCTURE.md
           plugins/mochiko/skills/patterns-vertical-tdd/references/SLICE-IDENTIFICATION.md
New partners: none (standalone; producer↔validator pair already exists — task-architect ↔ devils-advocate(validation-task-artifacts))
Wiring:    classification=model-invoked (default; agent-consumed; frontmatter = name + description only)
           router=DEFERRED to Wave-2 (task scope excludes skills/mochiko/SKILL.md + plugin.json)
           triggers=user-utterance → WORK-CONTEXT graded MUST/SHOULD (models patterns-entity-modeling)
           rebinds=none (.humaninloop//.workflow/ absent; zero humaninloop: cross-refs) — verified
           new-references=tasks-template.md (single-source to P5 canonical structure, Flag 5a)
           single-source=references P5 template for the skeleton; does not restate it
```

---

## Realized responsibility trace (every section tagged)

### SKILL.md body

| Responsibility | Realized tag |
|---|---|
| Discipline banner ("Violating the letter = the spirit") | `kept` (verbatim) |
| Overview | `port-with-edits` — added the **two-artifact** framing (`task-mapping.md` source-of-truth → `tasks.md`) and the **design-time** boundary (RQ-A Branch A) |
| When to Use | `kept`; L23 "the task architect **agent**" → `kept-but-rebind` → role: "the task-structuring producer" (decouple) |
| When NOT to Use | `kept` + added an **execution-boundary** bullet (structures tasks design-time; running red/green/refactor against real infra + cycle report is downstream) — honors Flag 7.5 / B1 forward-watch by *role/phase*, names no deferred primitive |
| Core Principle 1 — Vertical over Horizontal | `kept` |
| Core Principle 2 — Test-First at Task Level | `kept`; diagram final task relabeled `Demo and validate` → `TEST — verify against real infrastructure` (internal-consistency align, Flag 5a) |
| Core Principle 3 — Foundation + Parallel | `kept` |
| Core Principle 4 — Layered Testability | `kept` |
| Identifying Vertical Slices (heuristics + boundary table + ref) | `kept` |
| **Cycle Structure — Standard Cycle Format + numbering + markers** | `dedupe`/**align → P5** (Flag 5a): Standard Cycle Format now **byte-identical to CYCLE-STRUCTURE.md Cycle Anatomy**; final task = `**TEST:**` block; **references `tasks-template.md`** as the canonical skeleton (`kept-but-rebind`, new single-source link) |
| Foundation vs Feature Cycles | `kept` |
| TDD Task Sequence (Red/Green/Refactor/verify) | `kept`; Task 4 reframed `Demo and Validate` → **`**TEST:**` verification task** (real-infra gate; verifies spec acceptance criteria) — align (Flag 5a) + producer↔validator mirror (Flag 5b) |
| Mapping Stories to Cycles (Simple/Split/Merge) | `kept` + **Branch A** framing: story→cycle decisions + rationale authored into `task-mapping.md` (SoT); `tasks.md` Story→Cycle table = **derived echo** |
| Common Rationalizations (8 rows) | `kept` (discipline core) |
| Red Flags — STOP and Restart | `kept` (discipline core) |
| Common Mistakes | `kept`; two rows' `Demo` → `TEST (verify)` for consistency with the aligned final task |
| Quality Checklist | `kept` + **mirror-aligned with P4** (Flag 5b): added `**TEST:**`-task-presence, `[EXTEND]`/`[MODIFY]` marker presence, and task-mapping.md-SoT items → producer self-check (NOT an independent grade — stays producer-side) |
| `description` (frontmatter) | `kept-but-rebind` — HIL user-utterance ("when the user says…") → **WORK-CONTEXT** graded MUST/SHOULD; names the two artifacts; states the design-time-vs-runtime boundary; retains the trigger phrases in the SHOULD clause; shaped on `patterns-entity-modeling` |
| classification tag (absent in HIL) | **assigned model-invoked** (default; agent-consumed) — wiring |
| router registration | **DEFERRED to Wave-2** (task scope excludes the router + plugin.json) — recorded, not dropped |

### references/CYCLE-STRUCTURE.md (progressive disclosure)

| Responsibility | Realized tag |
|---|---|
| Cycle anatomy / task-ID format / file-path conventions / foundation+feature examples / brownfield markers / checkpoint guidelines | `ported-with-skill` (`kept`); examples' final tasks shown as `**TEST:**` (already so in HIL; kept) |
| Worked **"Complete Example"** `tasks.md` | **align → P5** (Flag 5a): `[P]`/`[US#]`/`[EXTEND]`/`[MODIFY]` markers, `TN.X` IDs, `**Checkpoint**:` lines, `**TEST:**` blocks; added a **Story → Cycle Mapping derived-echo** table (Branch A) matching P5's L187–196 framing; added an intro pointer to `tasks-template.md` (`kept-but-rebind`) |
| **Verification Task Requirements** — the `**TEST:**` grammar (unified format · field defs · action modifiers · assert patterns · examples · legacy format) | **canonical home — COMPLETE** (Flag 3): receives P2's `folded-into-skill` `**TEST:**` grammar; grep-confirmed already resident and intact; no orphan procedure |
| `qa-engineer` runtime-classification prose (L228, L332) + "downstream verification **agent**" (L236) | `kept-but-rebind` → **role**: "a downstream verification step" / "the downstream verification step" (decouple; the deferred `qa-engineer` is out-of-scope) — 3 sibling/agent-noun tokens dropped |
| kernel-freedom | confirmed (grep: zero brain/DAG/MCP/catalog/.humaninloop/.workflow) |

### references/SLICE-IDENTIFICATION.md (progressive disclosure)

| Responsibility | Realized tag |
|---|---|
| Value-stream test / extraction-from-stories / size calibration / dependency analysis / domain examples / anti-patterns / decision matrix | `ported-with-skill` (`kept`, effectively verbatim — no coupling, no agent names) |
| Step 3 "Map Stories to Cycles" table | `kept` + one-line **Branch A** note: decisions + rationale recorded in `task-mapping.md` (SoT); `tasks.md` table is its derived echo |
| upstream `constraints-and-decisions.md` IP-XXX platform-foundation input (L40/L101/L118) | `kept` (craft handoff edge; command-owned, not skill coupling) |
| kernel-freedom | confirmed (grep clean) |

**No `dropped` responsibilities** in P3 (assessment carried none). The 3 qa-engineer/verification-agent **tokens** are `dropped` per Flag 7.4, but the *capability* (runtime classification) re-homes to implement/qa (parked) — no silent capability loss.

---

## Contracts honored (Flag 5a / 5b / 3 / 7.5)

- **Flag 3 — canonical home:** P3 is the single home of the Cycle Structure template, the `**TEST:**` verification-task grammar, the slice heuristics, the marker table, and the quality checklist. All present and complete; nothing orphaned. ✔
- **Flag 5b — producer↔validator mirror (P3↔P4):** every authoring rule P3 teaches is a check P4 grades — vertical-slice quality · TDD test-first ordering · cycle sizing/definition · **`**TEST:**` task presence** · story→cycle→task traceability · marker/file-path/task-ID presence. The Quality Checklist was tightened so "what P3 teaches" = "what P4 checks." IP-XXX/brownfield/roadmap depth stays dormant in lock-step with P4 (Flag 7.4 defer). ✔
- **Flag 7.5 / B1 — boundary vs `executing-tdd-cycle`:** P3 stays on task-**structuring** (design-time). No runtime red/green/refactor, no `cycle-report.md` (grep: zero). The new When-NOT bullet + the description's closing clause state the boundary by phase/role, coupling to no deferred primitive. Clean forward-watch. ✔
- **Flag 5a — canonical structure (P3↔P5):** markers, `TN.X`, `**Checkpoint**:`, and the derived-echo Story→Cycle framing all **conform** to the as-landed P5. **One point does NOT — see the OPEN CONFLICT below.**

---

## ⚠ OPEN ALIGNMENT CONFLICT — P3↔P5 final-task grammar (for reconcile / verify-output / lead)

**The as-landed P5 `plugins/mochiko/templates/tasks-template.md` (created 18:41, untracked, ported in parallel) uses `Demo and validate acceptance criteria` as every cycle's final task (Cycle Format L21; sample cycles L68/L86/L109/L126/L142) and contains NO `**TEST:**` block anywhere.** This skill (P3) uses the `**TEST:**` verification block as the final task.

**Why P3 was kept on `**TEST:**` (not conformed down to "Demo"):**
- Reconcile **Flag 5a** lists **"the `**TEST:**` block"** as one of the four byte-align points — presupposing the canonical structure contains one.
- Reconcile **P5 trace L240:** "cycle-format/markers/task-ID/checkpoint/**TEST block** → canonical structure; P3 conforms."
- Reconcile **Flag 3:** the `**TEST:**` grammar is `folded-into-skill` → **P3 as canonical home** (from P2). Conforming P3's cycle format down to "Demo" would **orphan** that folded grammar — CYCLE-STRUCTURE.md would teach an elaborate `**TEST:**` grammar the cycle format never uses (internal contradiction).
- The transform task's own byte-align list includes "the `**TEST:**` block grammar."
- The verification task is, per Flag 3, "what makes vertical TDD actually vertical."

**Read:** the alignment direction Flag 5a intends is **toward `**TEST:**`** (P5 adopts the `**TEST:**` final-task grammar), not toward "Demo" (which drops the canonical, folded grammar). The as-landed P5 appears to have kept HIL's "Demo and validate" final task and **not applied the Flag 5a resolution** — i.e., the divergence is on the **P5 side**.

**Scope:** editing P5 is out of this task (P3-only: the 3 files above). I did **not** touch P5. P3↔P5 alignment is a reconcile-owned relationship (Flag 5a) — surfaced here, not silently absorbed or unilaterally resolved by editing a sibling at the keyboard.

**Recommended resolution (reconcile / lead to adjudicate):** P5's cycle Format + sample cycles adopt the `**TEST:**` verification block as the final task (matching P3 + CYCLE-STRUCTURE.md), so the two are byte-conformant on the final-task grammar. A neutral bridging line was added to P3's Cycle Structure section tying the `**TEST:**` task to the template's "validate acceptance criteria" role, so the skill reads coherently either way — but the byte-drift on the final-task line stands until P5 is aligned.

---

## Convention-wiring pass (all six)

1. **Classification** — model-invoked (default; agent-consumed). Frontmatter = `name` + `description` only (matches `patterns-entity-modeling`). ✔
2. **Router registration** — **DEFERRED to Wave-2** by task instruction (router `skills/mochiko/SKILL.md` + `plugin.json` excluded). Recorded so it is not dropped; the router still has no Tasks-cluster section (Flag 7.2). ⏸
3. **Trigger phrasing** — work-context graded MUST/SHOULD; the SHOULD clause retains the exact-phrase triggers; boundary clause distinguishes design-time structuring from runtime execution. ✔
4. **Path rebinding** — none required (`.humaninloop/`/`.workflow/` absent; zero `humaninloop:` cross-refs) — grep-verified across all 3 files. New single-source links to `tasks-template.md` added (Flag 5a mechanism). ✔
5. **Decouple persona/skill** — 3 hits rebound to role (SKILL.md:23 task-architect → "the task-structuring producer"; CYCLE-STRUCTURE.md:228/332 qa-engineer + :236 "verification agent" → "the downstream verification step"). Keystone-clean survivors kept as craft: artifact refs (`task-mapping.md`/`tasks.md`/`spec.md`/`constraints-and-decisions.md`), Foundation-vs-Feature cycle taxonomy, `[EXTEND]`/`[MODIFY]` brownfield markers, IP-XXX input. No "dispatch", no "workflow-agnostic", no injected modes/paths/phases. ✔
6. **Single-source / de-duplicate** — P3 **references** `tasks-template.md` (P5) for the canonical skeleton rather than owning a rival structure; the worked example is illustration pointing at the template. ✔ (subject to the OPEN CONFLICT above)

---

## Done-condition (this transform)

- Body recipe (`port-with-edits`) applied; structure/voice preserved; edits localized. ✔
- Structural (`standalone`) — placed in `skills/patterns-vertical-tdd/`; no partner spun out (pair pre-exists, independent). ✔
- Wiring pass run (router item explicitly deferred, not skipped). ✔
- Every SKILL.md section + both reference files + description-triggers tagged; **no silent loss**. ✔
- Flag 3 (canonical home complete), Flag 5b (mirror), Flag 7.5 (boundary) honored. ✔
- Flag 5a: conformant on markers/`TN.X`/`**Checkpoint**:`/derived-echo; **one open conflict** (final-task grammar) surfaced for reconcile/verify-output — **NOT** silently absorbed. ⚠
- Not self-graded — handed to `verify-output` (different agent). ✔
