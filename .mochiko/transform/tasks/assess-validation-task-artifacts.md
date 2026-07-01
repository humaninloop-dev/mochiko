# ASSESSMENT — P4 `validation-task-artifacts` (tasks cluster)

**Run:** `/mochiko:transform-cluster tasks` · core-only · assess-only (no transform)
**HIL source:** `human-in-loop/plugins/humaninloop/skills/validation-task-artifacts/` — `SKILL.md` + `references/ISSUE-TEMPLATES.md` + `references/PHASE-CHECKLISTS.md`
**Gold-standard sibling:** `plugins/mochiko/skills/validation-plan-artifacts/` (P9 of the plan port — matched closely)

---

## Class / branch + triage

**Class:** skill (model-invoked) → **branch: PLAYS-a-role.**
For a skill the weight sits on: consumed-procedure vs emits-artifact, trigger reliability, sibling overlap, decoupling (independence by *role*, no agent names).

**Triage gate:** `gate1=y` `gate2=y` `gate3=y` → **full lens.**
- **gate1 (orchestration-coupled?) = YES** — body carries phase-injection ("before proceeding to the next phase", "Review Focus by Phase", mapping|tasks split driven by the HIL DAG) and names its caller (Devil's Advocate) + its verdict-sink (Return to Task Architect). Coupling is to the dissolving supervisor, not a kernel.
- **gate2 (multi-responsibility / fans out?) = YES (loosely)** — two review modes (mapping / tasks) + severity + verdict + report templates; single consumer.
- **gate3 (emits a NOT-machine-checkable artifact?) = YES** — emits a gap report + a 3-state verdict from model judgment (is this a true vertical slice? is the checkpoint observable vs test-only?), not a schema/version-equality assert. The validator partner is **real, not degenerate.**

---

## The 7-check lens

**1 · Orchestration test.** Orchestrator = the HIL DAG/`tasks` supervisor (the `humaninloop:tasks` command is DAG-based), which injected the review "phase" and looped on the verdict. Split:
- *Content-coupling* (decouplable by edits): "Devil's Advocate" / "Task Architect" names, the mapping|tasks **phase** framing, "next phase" / "before implementation begins". → `port-with-edits`.
- *Orchestration-coupling* (re-homes): phase sequencing (which artifact set is in scope) + the FAIL-loop → the `/mochiko:tasks` **lead** (P1, ported this same run), exactly as `validation-plan-artifacts` lets "the caller supply which artifacts are in scope". The **body is fundamentally clean** — a mirror-checklist gap-finder, no kernel/DAG mechanism baked in.

**2 · Role (two altitudes).** Skill-role = a **mirror-checklist** (fixed checks · fixed questions · fixed severities · verdict derived mechanically from issue counts). Team-role conferred = **validator / completeness-reviewer**. It emits a reviewable output, but this IS the validator-side skill of the producer↔validator pair (producer = `task-architect` P2 authoring via `patterns-vertical-tdd` P3) — it does not itself need a downstream validator. Structural twin of `validation-plan-artifacts`.

**3 · Independence — CLEAN (explicit).** Producer of task artifacts = `task-architect` (P2); grader = `devils-advocate` (mounting this skill) — **different agents**. The re-mount target is the **reviewer** agent, never the producer. `devils-advocate` mounts only reviewer skills (`analysis-specifications`, `validation-plan-artifacts`, +this) — no task-authoring skill, so **no self-grade leak**. Independence guaranteed structurally. *Guard for transform/reconcile:* `validation-task-artifacts` must NOT be mounted on `task-architect`.

**4 · Verdict-sink / loop-driver — VERDICT-OWNERSHIP (explicit).** The skill emits a **recommended 3-state** `ready / needs-revision / critical-gaps`, **NOT a clearing PASS/FAIL**. Consumer of the report + FAIL-loop driver = the `/mochiko:tasks` **lead** (P1), who owns the clearing verdict and the bounded revision loop (cap 3, settled precedent). "Return to Task Architect" is a routing directive that **moves to the lead** — the reviewer *recommends*, the lead *routes*. This is the identical doctrine the plan port applied to `validation-plan-artifacts` ("Return to the responsible producer"), and matches this run's stated contract.

**5 · Sibling / overlap — DEDUP-vs-`validation-plan-artifacts` (explicit).** Two sideways relationships:
- **vs `validation-plan-artifacts` (same agent, same 3-state, same Critical/Important/Minor):** artifacts reviewed are **disjoint** (PLAN: requirements · constraints-and-decisions · NFRs · data-model · contracts · quickstart — vs TASK: task-mapping.md · tasks.md); checks are **disjoint** (design coverage/measurability/schema-consistency vs vertical-slice quality · TDD/test-first ordering · TEST-task presence · story→cycle→task traceability · task-ID/file-path/marker presence). The **only** shared surface is scaffolding — the severity taxonomy, the 3-state verdict, the issue-doc format, the report shape — all already single-sourced (severity + 3-state are the mochiko validation-skill convention; the assembled deliverable is `advocate-report-template`). **Verdict: cleanly complementary, NO structural merge.** One convention-wiring **dedupe** falls out: the target's `ISSUE-TEMPLATES.md` embeds two full report templates ("Mapping Review Report" / "Tasks Review Report") — the ported sibling instead *references* `advocate-report-template` ("do not restate that template here"). → dedupe the embedded templates, keep the task-specific "Checks Executed" tables.
- **vs `patterns-vertical-tdd` (P3, producer skill, same cluster):** this is the **producer↔validator mirror** — the review checklist mirrors the authoring criteria (vertical slice · TDD structure · cycle definition). Not a merge; the healthy split. Reconcile should keep the two aligned (the reviewer must check what the producer is told to author).

**6 · Coupling audit.** No `.humaninloop/` directory paths in the body; it names the artifacts it reviews (legit). Prerequisite reads (task-mapping.md/tasks.md, spec stories, constraints-and-decisions.md for IP-XXX) rebind to `.mochiko/specs/<feature>/`. **Determinism boundary:** greppable checks (task-ID `TN.X` format, file-path presence, `TEST:` task presence, `[P]`/`[EXTEND]`/`[MODIFY]` marker presence, traceability presence) vs model-judgment checks (true vertical slice? slice sizing? checkpoint observable vs test-only?). The sibling added a Tier-1 pre-assert (`check-artifacts.py`) for its greppable slice; **the target has none, and none is built here** (minimalism; the sibling's script is plan-specific). Noted as an optional future parity item, not a port requirement — the core is model judgment, so gate3 stands.

**7 · Conventions + loop placement.**
- Classification: model-invoked — **keep** (no `disable-model-invocation`).
- Discoverability: **NOT in the router** — no "Tasks cluster" section exists yet. → register (convention-wiring).
- Triggers: description is HIL "when the user says…" style; mochiko convention (per sibling) is a **work-context** description for an agent-consumed skill. → rewrite.
- Decoupling scan — **deny-list hits:** "for the **Devil's Advocate**" (SKILL L10), "Return to **Task Architect**" (SKILL L51; ISSUE-TEMPLATES L20), "Wait for **Task Architect**…" (SKILL L26, L155), "Used by the **Devil's Advocate**" (PHASE-CHECKLISTS L3), and the **phase-injection** framing (mapping|tasks *phase*, "next phase"). No "dispatch" / "workflow-agnostic" labels. → decouple to role ("the responsible producer") + reframe phase→**artifact-type** ("the caller supplies which artifacts are in scope"), matching the sibling's "Review Focus by Artifact Type".
- Producer↔validator pairing: guaranteed structurally (different agent + different skill). ✓
- Sound-loop placement: the loop's done-condition + human gate + FAIL-drive live on the lead (P1); this skill supplies the **independent-validation input**. ✓

**Re-mount (explicit).** `devils-advocate` (`plugins/mochiko/agents/devils-advocate.md`) carries the deferred stub at frontmatter L25–28: *"Deferred — NOT dropped — re-mount here when its cluster ports: validation-task-artifacts (tasks cluster)."* This run's wiring makes it **live**: add `validation-task-artifacts` to the `skills:` list (L28), add a Skills-Available bullet (after L38), delete the deferred comment. Traced `kept-but-rebind` (stub→live). Independence preserved (mounted on the reviewer, per Check 3).

---

## Disposition

**`port-with-edits` × `standalone`** (skill lands standalone in its own directory; both reference files travel with it; the devils-advocate re-mount + router + `plugin.json` are convention-wiring, not a structural move of the skill)
**+ flag-for-reconcile: RQ-A artifact-shape** — the mapping|tasks review split assumes **two** task artifacts. If RQ-A collapses mapping into `tasks.md` (one artifact), the artifact-type review folds to one pass. Do NOT resolve here (cluster-scope call). *Note:* reframing phase→artifact-type is robust to either RQ-A outcome (one row or two) — the body edit is safe to apply; only the row-count waits on the flag.

Body treatment rationale: **not `keep-verbatim`** (needs decouple + description + verdict-ownership + dedupe edits); **not `redesign`** (no kernel/DAG mechanism in the body — the phase language is decouplable content). Matches the sibling's `port-with-edits × standalone`.

---

## Responsibility trace (complete — SKILL.md + both references + triggers + re-mount + router)

**SKILL.md body**
1. Task-artifact gap-finding procedure (vertical slice · TDD structure · traceability over task-mapping.md/tasks.md) → **kept** (the skill's reason to exist)
2. Severity classification (Critical / Important / Minor) → **kept**
3. 3-state verdict *derivation* from issue counts (ready / needs-revision / critical-gaps) → **kept**
4. Verdict as a *clearing decision* + "Return to Task Architect" routing → **moved-to-lead** (lead owns the clearing verdict + FAIL-loop; reviewer recommends)
5. "provides phase-specific review criteria for the **Devil's Advocate**" (naming caller agent) → **dropped + reason:** decouple — independence stated by role, not by naming the reviewer agent (sibling names none)
6. Phase split (Mapping-phase vs Tasks-phase criteria) → **kept-but-rebind** (reframe phase→artifact-type; "caller supplies which artifacts are in scope") — *set-count pending RQ-A → flag-for-reconcile*
7. Review process (gather → execute → cross-reference → report) → **kept**
8. Cross-artifact traceability checks (story→cycle→task) → **kept**
9. Key principles to validate (vertical slicing · TDD structure · traceability) → **kept**
10. Quality checklist (reviewer self-check) → **kept**
11. Common mistakes → **kept**
12. "Wait for Task Architect to complete/signal completion" (When-NOT / mistakes) → **kept-but-rebind** ("during active drafting — wait for artifact completion"; agent-name dropped)
13. When-NOT pointers to sibling skills (`validation-plan-artifacts`, `analysis-specifications`) → **kept-but-rebind** (`humaninloop:`→`mochiko:`; the boundary pointers establish the clean division of labor)

**Description / triggers**
14. Trigger phrases ("MUST be invoked when the user says 'review task mapping'…") → **kept-but-rebind** (rewrite to mochiko work-context convention: grade a producer's task artifacts → severity-classified gap report + 3-state verdict; the task-artifact half of the producer↔validator pair; defaults to FAIL; independent validator, never the author; explicit boundary vs `validation-plan-artifacts`)

**ISSUE-TEMPLATES.md**
15. Severity classification rules (categories + examples) → **kept**
16. Issue-documentation format (standard template + examples) → **kept**
17. Embedded report templates (Mapping Review Report · Tasks Review Report) → **dedupe** (reference `mochiko:advocate-report-template`; keep only the task-specific "Checks Executed" tables) → the retained Checks-Executed tables **kept**
18. Verdict decision tree (mechanical derivation) → **kept**
19. Issue-ID conventions (TM-/TT-/TX-) → **kept**
20. "Return to **Task Architect**" action label → **kept-but-rebind** ("Return to the responsible producer"; agent-name dropped)

**PHASE-CHECKLISTS.md**
21. Mapping checklist (story coverage · vertical slice · foundation · sizing · traceability) → **kept**
22. Tasks checklist (cycle coverage · TDD/test-first · file paths · `TEST:` verification tasks · task IDs · story labels · markers · checkpoints · dependencies) → **kept**
23. Cross-artifact review checklist (mapping↔tasks alignment · story traceability · consistency) → **kept**
24. Vertical-slice / TDD-structure / verification-task validation guidance (good vs bad examples) → **kept**
25. "Used by the **Devil's Advocate**" (L3) → **dropped + reason:** decouple — agent-name
26. IP-XXX coverage · platform-app ordering · deployment cycle · `[EXTEND]`/`[MODIFY]` brownfield markers · constraint-task traceability → **kept (dormant/parked)** — feeders (IP-XXX from constraints-and-decisions, evolution-roadmap, `[GAP:XXX]`) are the roadmap/brownfield/IP track that context.md marks **DEFERRED** (documented stub). Parked in reference content, not dropped — mirrors the plan port's parked brownfield-discovery review; activates when that track ports. **Not a silent drop.**
27. Common-issues tables (mapping / tasks) → **kept**
28. Example review report (embedded, illustrative) → **kept**

**Cluster wiring**
29. Devils-advocate re-mount (stub L25–28 → live: `skills:` entry + Skills-Available bullet, delete comment) → **kept-but-rebind** (stub→live; independence preserved — reviewer agent)
30. Router registration (new "Tasks cluster" section in the `mochiko` router, with when-to-reach-it + boundary vs `validation-plan-artifacts`) → **kept-but-rebind** (discoverability; currently absent)
31. `plugin.json` manifest registration (per context.md: "register P2–P5 in the router + plugin.json") → **kept-but-rebind** (packaging)

*Silent-drop check:* every SKILL.md / ISSUE-TEMPLATES.md / PHASE-CHECKLISTS.md section, the triggers, the re-mount, and the router are tagged. The only `dropped` items (5, 25) are decouple-of-agent-name, each with a reason for lead acceptance.

---

## Reconcile flags (for `reconcile-cluster`)

1. **RQ-A artifact-shape (primary).** Two-phase split (task-mapping.md → tasks.md) vs collapsed single `tasks.md`. Resolve RQ-A first; then this skill's artifact-type section reflects one set or two. Body edit (phase→artifact-type) is safe regardless; only the row-count depends on the flag. **Do not resolve in assess.**
2. **Boundary vs `validation-plan-artifacts` — confirmed clean.** Disjoint artifacts + disjoint checks; complementary, not overlapping. Reconcile: record the mutual boundary in both skills' descriptions/When-NOT (the sibling already points here at its L45). No merge.
3. **Scaffolding dedupe.** Embedded report templates in `ISSUE-TEMPLATES.md` → reference `advocate-report-template` (convention-wiring dedupe; matches sibling). Hand to `transform-recipes`.
4. **Mirror-alignment with `patterns-vertical-tdd` (P3).** Keep the reviewer's checks aligned with the producer's authoring criteria (producer↔validator mirror). Not a merge.
5. **Parked/dormant checks (IP-XXX / brownfield / roadmap).** Confirm these stay as parked reference content (not dropped, not live-required) for the core-only tasks loop; activate with the roadmap/brownfield track.
6. **Optional: determinism pre-assert parity.** A Tier-1 greppable pre-assert (task-ID/file-path/TEST-task/marker/traceability presence) is a possible future parity item with the sibling's `check-artifacts.py`; not built in this core port (minimalism). Non-blocking / backlog.
