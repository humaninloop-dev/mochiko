# Assessment: validation-constitution

Source: `human-in-loop/plugins/humaninloop/skills/validation-constitution/` (SKILL.md + references/{ANTI-PATTERNS.md, QUALITY-CHECKLIST.md})
Run: `setup` cluster transform — CORE scope. ASSESS ONLY.

---

## Step 1 — Branch by class

**Class: skill → PLAYS-a-role branch.** Weight the checks toward consumed-procedure-vs-emits-artifact, trigger reliability, and sibling overlap. The independence linchpin (Check 3) carries the most weight here by run-context mandate: this is the validator-side skill.

## Step 2 — Fast-path triage gate

1. **Orchestration-coupled?** **NO (content) / nuanced.** The body is a self-contained checklist procedure — no kernel, no MCP, no DAG, no catalog references. It does not *depend* on a supervisor/command to function (it can be invoked directly). The coupling here is not content-coupling; it is a wiring/independence problem surfaced below.
2. **Multi-responsibility / fans out?** **YES.** Holds checklist verification, three-part-structure check, anti-pattern scan, placeholder scan, quantification enforcement, version-bump determination, and binary-verdict emission. It also serves two producers (authoring + brownfield constitution).
3. **Emits an artifact whose correctness is NOT machine-checkable?** **YES.** Emits a `VALIDATION RESULT` (PASS/FAIL + fix list + version bump). Grading a constitution is model judgment (is enforcement *real*, is language *vague*, is the bump *semantically* right) — not a version-equality or schema assert. The validator is **real, not degenerate.**

Gates 2 and 3 trip → **full 7-check lens.**

---

## Step 3 — The lens (weighted PLAYS-a-role / skill)

### 1. Orchestration test
- **Orchestrating layer:** No kernel (HIL `setup` is a **markdown supervisor**, `commands/setup.md`). But critically, **the supervisor never invokes this skill.** Walking all of `setup.md` (Phases 0–5): the only loops are agent "Clarifications Needed" (Phase 3) and human checkpoints (Phase 2 analysis review, Phase 5 cleanup). There is **no validation phase**. The skill is reached only via (a) `principal-architect` self-invoking it (its `skills:` list) or (b) user keyword trigger.
- **Content-coupling:** none to kernel/DAG/catalog. Body is clean markdown.
- **Orchestration-coupling / responsibilities to re-home:** the **verdict-sink** and the **FAIL-loop** for this skill's PASS/FAIL — *which do not currently exist in the supervisor* — must be built and re-homed onto the mochiko supervisor/lead. This is the dry-run's strategic shape: the real transform is not "shed a kernel" but "add the independent-validation + human gate the original never had."

### 2. Role at two altitudes
- **Skill-role:** a **consumed procedure that emits a verdict artifact** — the grading checklist a caller runs to produce a PASS/FAIL + fix list over a constitution. It does not author the constitution (that's authoring-/brownfield-constitution); it grades it.
- **Team-role conferred:** **validator / referee.** Whoever runs this skill becomes the grader of the constitution. Per LENS Check 2, that conferred validator team-role is exactly what makes it `promote` material.
- **Reviewable artifact:** it IS the verdict half of a producer↔validator pair; the artifact it grades (the constitution) is produced upstream.

### 3. Independence — THE linchpin
- **Self-grade leak: CONFIRMED.** `principal-architect.md` line 35 `skills:` = `authoring-constitution, brownfield-constitution, validation-constitution, analysis-codebase, syncing-claude-md, authoring-roadmap`. The **same agent authors** (authoring/brownfield-constitution) **and grades** (validation-constitution) the constitution.
- **Where it hides:** in the agent's `skills:` list — invisible at `validation-constitution/SKILL.md`. (The skill body even argues *against* self-grading — "Authoring mode ≠ validation mode. Fresh review catches blind spots." — while HIL mounts it on the authoring agent, contradicting its own doctrine.)
- **Fix signal:** **split** the grading skill off `principal-architect` onto a **peer, independent validator agent** so the producer never grades its own constitution (`moved-to-validator`). Relational → flag-for-reconcile.

### 4. Verdict-sink / loop-driver
- This skill **is** a verdict primitive: Step 6 emits binary `VALIDATION RESULT: [PASS/FAIL]` + `Issues requiring fix` list + version bump. By Check 4 it is precisely the thing a loop should iterate on.
- **Consumer today:** structurally **none** — the supervisor reads no validation verdict. Consumed ad hoc by the same agent / user.
- **What loops on FAIL today:** **nothing** — there is no "validation FAIL → return to authoring → re-validate" loop in `setup.md`. **Missing loop.**
- **Re-home target:** onto the mochiko supervisor/lead as a **bounded** validation loop (round cap + no-progress guard) that returns the fix list to the producer and re-validates (`moved-to-lead`). A rehome-orchestration item for reconcile Job 2.

### 5. Sibling / overlap ("look sideways")
- **Producer↔validator pairing:** validation-constitution is the **validator partner** for BOTH `authoring-constitution` and `brownfield-constitution` (it grades what they produce). One independent validator can serve both producers (shared validator).
- **Merge?** No — validation is genuinely distinct from authoring; not a thin variant. No merge-into-sibling.
- **Trigger collision:** mostly distinct (write vs review), but generic phrases `"quality check"` / `"check quality"` risk collision with other validators. Low-risk → de-collide in the wiring pass (not structural).
- → flag-for-reconcile: pair/split onto an independent validator serving both constitution producers.

### 6. Coupling audit
- **Hardcoded paths:** body does not hardcode `.humaninloop/` paths. It references siblings by namespace — `humaninloop:authoring-constitution`, `humaninloop:brownfield-constitution` (When to Use line 18; Related Skills lines 200–202) → `kept-but-rebind` (`humaninloop:` → `mochiko:`).
- **Upstream prerequisite / handoff:** YES — assumes a constitution artifact already exists ("After drafting a constitution with…"). Handoff edge: producer writes `.mochiko/memory/constitution.md`; validator reads it. Must be an explicit contract edge.
- **Determinism boundary:** MIXED. Deterministic slices — placeholder/bracket scan (Step 4, greppable), three-part-section presence (Step 2). Model-judgment slices — vague-language detection (Step 3), semantic anti-pattern detection, version-bump determination (Step 5). The judgment bulk means the validator partner is **real, not a degenerate assert**; the deterministic placeholder scan can run as a cheap pre-assert inside the validator.

### 7. Conventions + loop placement
- **Classification:** model-invoked (graded triggers in `description`, no `disable-model-invocation`). Stays model-invoked (agent-consumed validator). 
- **Discoverability:** not yet in a router → register in `mochiko` router (as a hint).
- **Reliable model-invocation:** triggers are written in the **user-utterance idiom** ("when the user says 'review constitution'…"). For an agent-consumed validator skill these should be rephrased to **work-context** ("grade a constitution produced by the authoring producer"). Wiring fix.
- **Agent↔skill composition:** procedure correctly lives in the skill; the **validator persona must live on a NEW independent validator agent**, not `principal-architect`.
- **Producer↔validator pairing:** currently **VIOLATED** (same agent). Fix = split.
- **Sound-loop gaps:** the loop it sits in lacks (a) an **independent validation gate** (today self-graded), (b) a **done-condition / FAIL-loop** tied to the verdict, and (c) a **human gate on validated-constitution acceptance** (HIL's human gates sit at analysis-review + cleanup, not at constitution sign-off). These feed reconcile's rehome-orchestration.

---

## Step 4 — Disposition

**Body × Structural: `keep-verbatim` × `flag-for-reconcile`**

- **Body = keep-verbatim.** The content is mochiko-clean: a self-contained quality checklist, anti-pattern scan, placeholder scan, quantification table, version-bump matrix, binary-verdict format, and anti-rationalization doctrine — no kernel/DAG/catalog, no degenerate version-equality check. Every required change (trigger rephrasing to work-context; `humaninloop:` → `mochiko:` reference rebinds; router registration) is a **convention-wiring-pass item that always runs** — none is a body-prose edit the primitive *earns*. Minimalism governor → keep-verbatim, not port-with-edits. (The trace below is still mandatory; keep-verbatim pays the full floor.)
- **Structural = flag-for-reconcile** (relational; do not guess solo). Signals below.

## Step 5 — Responsibility trace (complete; no silent drops)

```
TRACE: validation-constitution
  - constitution quality-checklist verification (3-part: enforce/test/rationale)  → kept
  - anti-pattern scan (vague / missing-enforcement / placeholder / generic-threshold) → kept
  - placeholder/bracket scan (deterministic pre-assert slice)                     → kept
  - quantification enforcement (vague → measurable)                               → kept
  - binary verdict emission (PASS/FAIL) + fix list                                → kept (artifact stays;
        its CONSUMER + FAIL-loop re-home, see below)
  - version-bump determination (MAJOR/MINOR/PATCH, semantic judgment)             → kept
  - anti-rationalization / "letter = spirit" discipline (Red Flags, Loophole Closures) → kept
  - validator / referee TEAM-ROLE conferred on caller                             → moved-to-validator
        (persona → NEW independent constitution-validator agent; relational, reconcile assigns)
  - independence boundary (producer must not grade own work) — VIOLATED on principal-architect → moved-to-validator
        (split: remove skill from principal-architect.skills; mount on peer validator; relational)
  - verdict-sink + FAIL-loop + done-condition (ABSENT in setup.md supervisor)     → moved-to-lead
        (rehome-orchestration: bounded validation loop, round cap + no-progress; a MISSING gate to ADD)
  - human gate on validated-constitution acceptance (ABSENT in setup.md)          → moved-to-lead
        (supervisor's named human gate; a MISSING gate to ADD)
  - upstream prerequisite handoff (reads authored constitution artifact)          → kept-but-rebind
        (.humaninloop/memory/constitution.md → .mochiko/memory/constitution.md; explicit contract edge)
  - sibling skill references (humaninloop:authoring-/brownfield-constitution)      → kept-but-rebind
        (humaninloop: → mochiko:)
  - classification + trigger description (model-invoked, user-utterance idiom)     → kept-but-rebind
        (stays model-invoked; triggers rephrased to work-context; registered in router)
```

No responsibility dropped. Relational tags (`moved-to-validator`, `moved-to-lead`) are *flagged* here, *assigned* by reconcile-cluster.

## Reconcile flags (signals for reconcile-cluster)

1. **Self-grade leak (independence):** `principal-architect` mounts produce (authoring-/brownfield-constitution) AND grade (validation-constitution). → **split the agent**; `validation-constitution` → peer independent validator (`moved-to-validator`). Independence overrides convenience — must NOT land both on one agent.
2. **Validator team-role (promote):** the skill confers a validator role → **promote** it to the **load-bearing tool of a new independent constitution-validator agent**.
3. **Producer↔validator pairing:** it is the **shared validator** for both `authoring-constitution` and `brownfield-constitution` → **pair**; one independent validator grades both producers.
4. **Rehome-orchestration (MISSING gates to ADD):** verdict-sink, FAIL-loop/done-condition, and human-gate-on-acceptance are **absent** in HIL `setup.md` → reconcile must **add** them onto the supervisor/lead (`moved-to-lead`), not merely relocate existing wiring.
5. **Trigger de-collision (minor, wiring not structural):** generic `"quality check"/"check quality"` phrases → de-collide in the convention-wiring pass.

---

## Output block

```
ASSESSMENT: validation-constitution
Class:        skill → branch PLAYS-a-role
Triage:       gate1=n gate2=y gate3=y  [full-lens]
Disposition:  keep-verbatim × flag-for-reconcile
              signals: (a) self-grade leak → split principal-architect / moved-to-validator;
                       (b) validator team-role → promote onto NEW independent constitution-validator;
                       (c) shared validator for authoring + brownfield → pair;
                       (d) rehome MISSING verdict-sink + FAIL-loop + human-gate → moved-to-lead
Trace:        12 responsibilities tagged; 0 dropped; relational tags flagged for reconcile
Reconcile flags: independence split | validator-role promote | producer↔validator pairing
                 | rehome missing validation+human gates | minor trigger de-collision
```
