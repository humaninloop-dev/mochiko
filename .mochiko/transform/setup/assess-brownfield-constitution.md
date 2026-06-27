# Assessment: brownfield-constitution

Source: `human-in-loop/plugins/humaninloop/skills/brownfield-constitution/`
Files: `SKILL.md`, `references/EMERGENT-CEILING-PATTERNS.md`, `references/ESSENTIAL-FLOOR.md`
Cluster: `setup` (CORE) · Run scope: this primitive only (ASSESS ONLY)

---

## Step 1 — Branch by class

**Class: skill → branch PLAYS-a-role.** What carries weight: consumed-procedure vs emits-artifact, trigger reliability, sibling overlap. (Independence/loop-driver weighted lighter for a skill, but still relevant because of the agent that mounts it.)

This is the **brownfield variant** of constitution authoring. It is a *consumed-procedure* skill: a producer agent (HIL `principal-architect`) runs it as guidance to author a constitution for an existing codebase. It does not itself emit the artifact — it makes its *caller* a producer of a reviewable artifact (the brownfield `constitution.md`).

## Step 2 — Fast-path triage gate

1. **Orchestration-coupled?** **YES.** Hard prerequisite on `analysis-codebase` having run and left `codebase-analysis.md`; sequenced inside the `/humaninloop:setup` markdown supervisor; output feeds `validation-constitution`.
2. **Multi-responsibility / fans out?** **YES.** Holds Essential Floor doctrine, Emergent Ceiling doctrine, brownfield structure, evolution/gap-status documentation, brownfield quality checklist. Output fans to `validation-constitution` + `authoring-roadmap` + the human.
3. **Correctness NOT machine-checkable?** **YES.** A constitution is prose governance graded on enforceable/testable/justified judgment — not a version-equality or schema assert. Its validator (`validation-constitution`) is real, not degenerate.

All three gates trip → **FULL 7-CHECK LENS**.

## Step 3 — The lens (weighted PLAYS-a-role)

**1. Orchestration test.** Orchestrator is the **`/humaninloop:setup` markdown supervisor** — NOT a kernel/DAG (setup has no brain; only specify/plan/implement are DAG-based). Coupling split:
- *Content-coupling (body):* only hardcoded paths (`.humaninloop/memory/evolution-roadmap.md`, `codebase-analysis.md`), a cross-skill ref to `authoring-constitution/RECOMMENDED-PATTERNS.md`, and `${CLAUDE_PLUGIN_ROOT}/templates/approved-domain-deps.md`. No kernel/MCP/DAG/catalog concepts in the prose. → localized edits.
- *Orchestration-coupling:* assumes `analysis-codebase` ran first; supervisor sequences it; its output is looped through validation. → re-home to lead.

**2. Role at two altitudes.** Skill-role = **consumed-procedure**. Conferred team-role = **producer** (authoring). It guides toward a **reviewable artifact** → that artifact needs an *independent* validator partner. The partner already exists as a sibling skill (`validation-constitution`) — the gap is independence (Check 3), not absence.

**3. Independence.** The leak hides in the **agent's `skills:` list**, not in this SKILL.md. HIL `principal-architect` mounts the produce-side skills (`authoring-constitution`, `brownfield-constitution`) **and** `validation-constitution` → one agent writes and grades the constitution = self-grade. brownfield-constitution is one of the produce-side skills implicated. The fix lives at the agent + reconcile (split produce vs grade across two agents), but it is surfaced here so this skill is never co-mounted with `validation-constitution`.

**4. Verdict-sink / loop-driver.** Consumers of the constitution: `validation-constitution` (validator) and the human. On FAIL the HIL flow loops author → validate → revise — that loop-driving was the supervisor's. → must re-home onto the mochiko lead's bounded loop (round cap + no-progress).

**5. Sibling / overlap ("look sideways").** The defining finding:
- **Shared core + thin variant.** brownfield-constitution **extends** authoring-constitution and explicitly *defers* (does not duplicate) Three-Part Rule, RFC 2119, SYNC IMPACT. Its genuinely **variant-unique slice** = Essential Floor (4 NON-NEGOTIABLE categories w/ brownfield detail) + Emergent Ceiling (codify existing patterns from codebase analysis) + brownfield structure + Evolution/gap-status. The LENS names this case by example: "brownfield-constitution is mostly authoring-constitution." → **merge-into-sibling vs keep-separate is a RECONCILE decision.**
- **Possible shared substrate (dedupe).** "Essential Floor (I-IV)" is referenced by authoring-constitution (greenfield baseline) but its *detailed* reference (`ESSENTIAL-FLOOR.md`) lives here. If kept separate this is a substrate mounted twice → dedupe candidate; if merged it collapses naturally.
- **Trigger collision.** `create constitution` (authoring) ⊃ `create constitution for existing codebase` (brownfield); token `brownfield` fires both this skill *and* `analysis-codebase`. A brownfield user could misfire authoring or analysis. → de-collide (wiring) or merge.

**6. Coupling audit.**
- Hardcoded paths → `kept-but-rebind`: `.humaninloop/memory/codebase-analysis.md`, `.humaninloop/memory/evolution-roadmap.md`, `authoring-constitution/RECOMMENDED-PATTERNS.md`, `${CLAUDE_PLUGIN_ROOT}/templates/approved-domain-deps.md`.
- `evolution-roadmap.md` + `authoring-roadmap` belong to the **roadmap cluster NOT ported this run** → those references rebind to `moved-to-other-cluster` targets.
- `approved-domain-deps.md` template is **not in this run's in-scope template set** → coupling note: lead must confirm it ports or the ref is softened (candidate `dropped + reason` if the registry doesn't come over).
- Prerequisite handoff: requires `analysis-codebase` → `codebase-analysis.md` (an explicit handoff edge).
- Determinism boundary: this skill is **entirely model-judgment** (writing governance prose). No deterministic script here (`detect-stack.sh` belongs to `analysis-codebase`). → its validator is genuinely judgment-grounded, not a degenerate assert.

**7. Conventions + loop placement.**
- *Classification:* none present; HIL phrases it model-invoked-by-user. In mochiko it is **agent-consumed** → set model-invoked + rephrase triggers to work-context (wiring).
- *Discoverability:* not in any router → register in the mochiko router (wiring).
- *Reliable invocation:* triggers are user-phrase based and collide with siblings → graded work-context triggers + de-collision (wiring + reconcile).
- *Agent↔skill composition:* pure procedure, no persona → correct as a skill; producer agent mounts it.
- *Producer↔validator pairing:* validator exists (`validation-constitution`) but is **co-mounted** on the same agent → independence violated (Check 3).
- *Sound-loop:* done-condition present (Brownfield Quality Checklist); independent validation present-but-leaky; **human gate MISSING** (no explicit human acceptance of the constitution in HIL) → loop gap for rehome-orchestration.

## Step 4 — Disposition

**Body treatment: `port-with-edits`.** The doctrine (Essential Floor, Emergent Ceiling, brownfield structure) is mochiko-clean and worth keeping; edits are localized — path rebinds, roadmap refs to the other cluster, trigger rephrasing. Not `keep-verbatim` (real edits exist); not `redesign` (no kernel/DAG assumption to rewrite away).

**Structural move: `flag-for-reconcile`.** merge-into-sibling (`authoring-constitution`) vs keep-separate (`standalone`) cannot be decided looking at one primitive. Signal carried to reconcile below.

> `port-with-edits × flag-for-reconcile`. If reconcile resolves **merge**, the variant-unique slice is ported-with-edits *into* the sibling and its responsibilities re-tag `moved-to-sibling-skill`; if **keep-separate**, the same edits apply standalone and the trace tags below stand as written.

## Step 5 — Responsibility trace (every responsibility tagged)

Variant-unique body (held here):
- Essential Floor doctrine — 4 NON-NEGOTIABLE categories + brownfield detail (`ESSENTIAL-FLOOR.md`) → **kept** *(reconcile may re-tag → `moved-to-sibling-skill` on merge)*
- Emergent Ceiling doctrine — codify existing good patterns from codebase analysis (`EMERGENT-CEILING-PATTERNS.md`) → **kept** *(merge → `moved-to-sibling-skill`)*
- Brownfield constitution structure (Essential Floor + Emergent Ceiling layout, Evolution Notes) → **kept** *(merge → `moved-to-sibling-skill`)*
- Brownfield quality checklist (done-condition for the producer) → **kept**
- Brownfield-specific Common Mistakes guidance → **kept**

Shared-core dependencies (referenced, not held — deferred to authoring-constitution):
- Three-Part Principle Rule / RFC 2119 / SYNC IMPACT format → **kept-but-rebind** (reference to `authoring-constitution`; on merge → `dedupe`)
- "Essential Floor (I-IV)" baseline shared with greenfield → **dedupe** *(reconcile: collapse the duplicated substrate to one source)*

Path / reference couplings:
- `.humaninloop/memory/codebase-analysis.md` ref → **kept-but-rebind** (`.mochiko/...`)
- `.humaninloop/memory/evolution-roadmap.md` ref → **kept-but-rebind** + **moved-to-other-cluster** (roadmap cluster, not ported this run)
- `authoring-constitution/RECOMMENDED-PATTERNS.md` cross-ref → **kept-but-rebind** (in-scope sibling)
- `${CLAUDE_PLUGIN_ROOT}/templates/approved-domain-deps.md` ref → **kept-but-rebind** *(coupling note: template not in this run's scope; lead must confirm it ports, else `dropped + reason`)*
- `authoring-roadmap` "create evolution roadmap" pointer → **moved-to-other-cluster** (roadmap cluster)

Orchestration responsibilities (re-home off the dissolving supervisor):
- Prerequisite sequencing (run `analysis-codebase` first; consume `codebase-analysis.md`) → **moved-to-lead** (supervisor sequences + wires the handoff edge)
- "After authoring, run `validation-constitution`" + author→validate→revise FAIL loop → **moved-to-lead** (supervisor's bounded loop owns loop-driving + verdict sink)
- Missing human acceptance gate → **moved-to-lead** (supervisor's named human gate — a gap to ADD, not relocate)

Convention-wiring responsibilities (the floor every port pays):
- Classification tag (assign model-invoked, agent-consumed) → **kept-but-rebind** (wiring pass)
- Router registration (currently none) → **kept-but-rebind** (wiring pass)
- Trigger phrasing — user-says → graded work-context phrases → **kept-but-rebind** (wiring pass)
- Independence: never co-mount with `validation-constitution` on one agent → handled at agent assessment + reconcile (`moved-to-validator` lands on the grade-side agent, not here)

No responsibility left untagged. No silent drops (the only drop candidate — `approved-domain-deps.md` — is explicitly surfaced for the lead).

## Reconcile flags (for reconcile-cluster)

1. **Shared-core / thin-variant** with `authoring-constitution` → resolve **merge-into-sibling vs keep standalone**. Variant-unique slice to preserve on merge: Essential Floor + Emergent Ceiling + brownfield structure/Evolution Notes.
2. **Dedupe** of the Essential Floor substrate referenced by both authoring (greenfield I-IV) and brownfield (detailed `ESSENTIAL-FLOOR.md`) → collapse to one source (resolves with #1).
3. **Trigger collision**: `create constitution` (vs authoring) and `brownfield` (vs `analysis-codebase`) → de-collide in wiring, or subsumed by merge.
4. **Independence / self-grade**: HIL `principal-architect` co-mounts this produce-side skill with `validation-constitution` → reconcile must keep produce on the producer agent and `moved-to-validator` the grading onto a separate validator agent.
5. **Rehome-orchestration**: prerequisite handoff (`analysis-codebase` → `codebase-analysis.md`), the author→validate→revise loop, and the **missing human gate** → land on the `setup` lead/supervisor.

---

```
ASSESSMENT: brownfield-constitution
Class:        skill → branch PLAYS-a-role
Triage:       gate1=y gate2=y gate3=y  [full-lens]
Disposition:  port-with-edits × flag-for-reconcile (merge-into-sibling vs standalone w/ authoring-constitution)
Trace:
  - Essential Floor doctrine (4 NON-NEGOTIABLE cats + detail)   → kept (merge → moved-to-sibling-skill)
  - Emergent Ceiling doctrine (codify existing patterns)        → kept (merge → moved-to-sibling-skill)
  - Brownfield constitution structure + Evolution Notes         → kept (merge → moved-to-sibling-skill)
  - Brownfield quality checklist (producer done-condition)      → kept
  - Brownfield-specific Common Mistakes                         → kept
  - Three-Part Rule / RFC 2119 / SYNC IMPACT (deferred refs)    → kept-but-rebind (merge → dedupe)
  - Essential Floor (I-IV) substrate shared w/ greenfield       → dedupe
  - codebase-analysis.md path ref                               → kept-but-rebind (.mochiko/)
  - evolution-roadmap.md path ref                               → kept-but-rebind + moved-to-other-cluster
  - authoring-constitution/RECOMMENDED-PATTERNS.md cross-ref    → kept-but-rebind
  - templates/approved-domain-deps.md ref                       → kept-but-rebind (lead-confirm; else dropped+reason)
  - authoring-roadmap "evolution roadmap" pointer               → moved-to-other-cluster
  - Prerequisite sequencing (analysis-codebase first)          → moved-to-lead
  - run-validation + author→validate→revise FAIL loop          → moved-to-lead
  - Missing human acceptance gate                               → moved-to-lead (gap to ADD)
  - Classification tag (model-invoked, agent-consumed)         → kept-but-rebind (wiring)
  - Router registration                                         → kept-but-rebind (wiring)
  - Trigger phrasing (user-says → work-context, graded)        → kept-but-rebind (wiring)
  - Independence (never co-mount with validation-constitution)  → reconcile → moved-to-validator (grade-side agent)
Reconcile flags: shared-core/merge-vs-keep (authoring-constitution); dedupe Essential Floor substrate;
                 trigger-collision (authoring + analysis-codebase); independence/self-grade (principal-architect);
                 rehome-orchestration (analysis handoff, validate loop, missing human gate)
```
