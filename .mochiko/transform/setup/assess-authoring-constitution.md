# ASSESSMENT — `skills/authoring-constitution/` (P3)

Cluster: `setup` · Phase 1 (assess) · Producer-side core authoring skill
Source: `human-in-loop/plugins/humaninloop/skills/authoring-constitution/` (SKILL.md + references/{RECOMMENDED-PATTERNS, RFC-2119-KEYWORDS, SYNC-IMPACT-FORMAT})

---

## Class / branch

**Class:** skill → branch **PLAYS-a-role**.
Weight on: consumed-procedure vs emits-artifact, trigger reliability, sibling overlap.

## Triage gate

| Gate | Q | Verdict |
|------|---|---------|
| 1 | Orchestration-coupled? | **y** — orchestration-coupling (not content). The skill body is kernel-clean, but it only runs inside the `commands/setup.md` **markdown supervisor**, invoked by `principal-architect`; the produce→validate→human-gate loop around it lives outside the skill. |
| 2 | Multi-responsibility / fans out? | **y** — holds ≥5 responsibilities (Three-Part Rule, RFC-2119, SYNC-IMPACT, mandatory sections, greenfield arch patterns) and fans out to brownfield-constitution (extends it), validation-constitution (grades its output), principal-architect (invokes it). |
| 3 | Emits non-machine-checkable artifact? | **y** — emits `constitution.md`, a prose governance doc. "Enforceable / testable / justified / non-vague" is **model judgment**, not a version-equality or schema assert. Its validator is genuine, not degenerate. |

All three trip → **full 7-check lens**.

---

## Lens findings

**1 — Orchestration test.** Orchestrated by a **markdown supervisor** (`commands/setup.md`, Phase 3), *not* a kernel (setup is supervisor-driven; only specify/plan/implement are DAG). **Content-coupling: none** — `grep` confirms zero kernel/DAG/catalog/MCP refs in body; the only path strings are `.humaninloop/templates/{plan,spec,tasks}-template.md` inside SYNC-IMPACT-FORMAT.md. **Orchestration-coupling (rehome to lead):** phase sequencing, greenfield/amend mode + detected-stack input wiring, the clarification iterate loop (cap 3), the human approval gate, and the produce→validate handoff. None live in the skill; all assumed around it.

**2 — Role (two altitudes).** Skill-role: **both** — a *consumed procedure* (a caller runs it as an authoring step) that *emits a reviewable artifact* (the constitution). Team-role conferred on caller: **producer**. Emits a reviewable, non-machine-checkable artifact ⇒ **needs an independent validator partner** (validation-constitution, P5, already exists as the grader).

**3 — Independence (the leak).** Invisible at the skill; visible at the agent: `principal-architect.skills` (line 35) mounts **both** `authoring-constitution` (producer) **and** `validation-constitution` (grader) → same agent writes and grades the constitution. **Self-grade leak.** Fix signal: authoring stays producer-only; validation-constitution must move to a **peer validator agent** (`moved-to-validator`). Relational → reconcile.

**4 — Verdict-sink / loop-driver.** Consumers of `constitution.md`: validation-constitution (grades), the user (approves via supervisor `AskUserQuestion`), downstream syncing-claude-md + specify/plan (governance source). **FAIL-loop is essentially MISSING in HIL:** the supervisor loops only on *clarifications*, and never actually dispatches validation-constitution (the skill text marks validation merely "OPTIONAL"). So the validate→FAIL→revise loop and the verdict-sink are a **loop gap**, not an existing thing to relocate — they must be *added* and homed on the mochiko lead.

**5 — Sibling / overlap ("look sideways").**
- **brownfield-constitution (P4)** explicitly *"extends authoring-constitution"* — a **thin variant over this shared core** (its prerequisites point back to this skill's Three-Part Rule, RFC-2119, SYNC-IMPACT). Variant-unique slice = Essential Floor + Emergent Ceiling + brownfield structure + Evolution Notes. → **merge-into-sibling** signal (authoring = core/absorber). 
- **validation-constitution (P5)** = the **shared validator** for authoring + brownfield outputs → producer↔validator **pairing** (ties to Check 3 independence).
- **Trigger overlap:** authoring SHOULD-fires on "governance"/"principles"; brownfield owns "brownfield/existing codebase". Generic "create a constitution" (authoring) vs "create constitution for existing codebase" (brownfield) can mis-fire → de-collide (wiring) or dissolve via merge.

**6 — Coupling audit.** Hardcoded paths: `.humaninloop/templates/{plan,spec,tasks}-template.md` in SYNC-IMPACT-FORMAT.md → `kept-but-rebind` to `.mochiko/...` (these are cross-cluster template *examples* in a list — rebind the path prefix only; don't port those templates here). No catalog/MCP/script paths. No `.humaninloop/memory/` write path in the body (the write target lives in the supervisor/templates, not the skill). Upstream prerequisite: none for greenfield (self-contained); brownfield's analysis-codebase prereq belongs to the variant, not here. **Determinism boundary:** authoring = model judgment; its validator = model judgment ⇒ **pairing is real, not a degenerate assert** (only version-bump/placeholder-scan sub-checks are semi-deterministic).

**7 — Conventions + loop placement.**
- Classification → **model-invoked** (default; agent-consumed by the producer). Assign tag.
- Discoverability → register in the `mochiko` router with when-to-reach guidance.
- Reliable model-invocation → description is currently *"when the user says 'write principles'…"*; agent-consumed skills must describe **work context**, not user utterances. Reframe (graded MUST/SHOULD kept, rephrased). Wiring fix.
- Composition → persona-vs-procedure split is **clean**: procedure (Three-Part Rule, formats, RFC-2119) in the skill, persona (governance judgment) in principal-architect. (Aside: principal-architect's *body* duplicates Three-Part Rule / Essential-Floor / RFC-2119 prose — a `dedupe` item on the **agent's** trace, not this skill's.)
- Pairing → validator exists but **independence violated** (Check 3).
- Sound-loop → loop gaps: (a) independent validation not enforced (validator never dispatched), (b) human gate exists at supervisor but not tied to a validation verdict. Feed reconcile rehome-orchestration.

---

## Disposition

**`port-with-edits` × `flag-for-reconcile`**

- **Body = `port-with-edits`** (not `keep-verbatim`, not `redesign`). Body is kernel-clean, so no redesign earned; but it is not zero-edit either — the `description` must be reframed to work-context, the SYNC-IMPACT template paths rebind, sibling-name references change, and IF reconcile merges brownfield in, this body absorbs the Essential-Floor/Emergent-Ceiling branch (additive). Minimalism governor: edits fix it, so no redesign.
- **Structural = `flag-for-reconcile`** — the placement depends on siblings (does brownfield fold into this core? is the pairing realized by splitting the validator onto a separate agent?). Not decidable solo. Likely landing: `standalone` as the producer-core (possibly the merge **absorber** for brownfield).

### Reconcile flags

- **F1 — merge:** brownfield-constitution (P4) is a thin variant over this shared core → `merge-into-sibling` with **authoring as absorber**; keep brownfield's Essential-Floor + Emergent-Ceiling slice (`moved-to-sibling-skill` from brownfield's side). Reconcile decides merge vs keep-shared-core.
- **F2 — independence/pairing:** self-grade leak in `principal-architect.skills` (authoring + validation co-mounted). Resolve as a `split`: validation-constitution → **peer validator agent**; authoring stays producer-only. Never co-mount.
- **F3 — rehome missing gate/loop:** the enforced independent-validation gate and the produce→validate→revise FAIL-loop **do not exist** in HIL. Rehome onto the mochiko lead (bounded loop: round cap + no-progress exit; human gate tied to the validation verdict).
- **F4 — trigger de-collision:** authoring↔brownfield SHOULD-trigger overlap → de-collide in the wiring pass (or dissolved by F1 merge).

---

## Responsibility trace (complete — every responsibility tagged)

```
TRACE: authoring-constitution
  - greenfield constitution authoring (Three-Part Rule: Enforcement/Testability/Rationale) → kept
  - RFC 2119 keyword guidance (references/RFC-2119-KEYWORDS.md)                            → kept
  - mandatory constitution sections spec (Principles/Tech Stack/Quality Gates/Governance/
    CLAUDE.md Sync Mandate)                                                                → kept
  - greenfield recommended architectural patterns (RECOMMENDED-PATTERNS.md: hexagonal,
    SRP, dependency discipline)                                                            → kept
  - SYNC IMPACT REPORT format (references/SYNC-IMPACT-FORMAT.md)                            → kept-but-rebind
        (.humaninloop/templates/{plan,spec,tasks}-template.md → .mochiko/...; cross-cluster
         example paths — rebind prefix only, do not port those templates here)
  - discoverability / trigger description (MUST/SHOULD phrasing)                            → kept-but-rebind
        (reframe to work-context per wiring pass; register in mochiko router; classify model-invoked)
  - shared core that brownfield-constitution extends                                       → kept  (+ F1)
        (stays the core; brownfield's unique slice may land here via merge → reconcile decides;
         from brownfield's side that is moved-to-sibling-skill)
  - pointer to validation-constitution ("validate after authoring")                        → kept-but-rebind  (+ F2)
        (reference rebinds; the ENFORCED validation gate is not this skill's — see below)
  - pointer to brownfield-constitution (When NOT to Use / Related)                          → kept-but-rebind  (+ F1)
  - pointer to syncing-claude-md (CLAUDE.md Sync Mandate execution)                         → moved-to-other-cluster
        (sync EXECUTION is cross-cutting, not ported this run; the section SPEC stays here as `kept`;
         rebind the reference name)
  - constitution quality self-checking                                                     → (none held)
        (authoring does not grade; grading is validation-constitution's — NO leak in this body.
         The leak is at principal-architect's skills list — see F2.)
  - enforced independent-validation gate + produce→validate FAIL-loop                       → moved-to-lead  (+ F3)
        (responsibility GAP surfaced here; does not exist in HIL; rehomed onto the mochiko
         supervisor/lead as a bounded loop with verdict-tied human gate)
```

No silent drops. No `dropped` tags (nothing in this body should cease to exist in mochiko).

---

## Handoff to reconcile-cluster

This primitive cannot finalize its structural move alone. Reconcile must, with full cluster context:
1. Resolve **F1** (brownfield → merge-into-this-core, or keep shared-core with branch).
2. Resolve **F2** (split validation-constitution onto a peer validator agent; authoring stays producer-only — independence non-negotiable; never co-mount on one agent).
3. Resolve **F3** (rehome the missing validation gate + FAIL-loop + verdict-tied human gate onto the lead).
4. Resolve **F4** (trigger de-collision, or dissolved by F1).

Body treatment (`port-with-edits`) is firm and may proceed once the structural move is finalized.
