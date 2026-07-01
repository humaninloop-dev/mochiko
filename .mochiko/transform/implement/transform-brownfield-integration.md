# Transform (realized) — `skills/brownfield-integration/SKILL.md` (P6)

Run: `implement` cluster transform · Phase 3 (transform) · Producer: `transform-producer`
Skill applied: `mochiko:transform-recipes` (port-with-edits + convention-wiring) · Date: 2026-07-01
Consumes: `assess-brownfield-integration.md` (P6 trace; soft-vocab located L21/27/38/72/90/104/122) + `reconcile.md` (**Seam 5b** marker dedupe; §Job 1.6 P6↔P4 de-collision; §Job-3 P6 finalized trace; §Phase-5 REGISTRY delta)
HIL source: `human-in-loop/plugins/humaninloop/skills/brownfield-integration/SKILL.md` (130 ln)
Finalized disposition (reconcile §Finalized dispositions): **`port-with-edits × standalone`** — marker vocab → reference; home corrected setup→implement; zero open flags.

> ROLE NOTE: transform APPLIES the finalized decision; it does not re-decide and does not grade its own output. The independent PASS/FAIL is `verify-output`, run by a different agent.

---

## Output block

```
TRANSFORM: brownfield-integration (P6)
Applied:   port-with-edits × standalone + convention-wiring pass
Artifacts: plugins/mochiko/skills/brownfield-integration/SKILL.md   (net-new single-file skill; HIL 130 ln → 96 ln)
New partners: NONE — standalone. Emits no artifact → no split/pair; confers producer role not validator → no promote;
              reusable craft not orchestration → no absorb-into-lead; disjoint from siblings → no merge.
Wiring:    classification = model-invoked (default; no disable-model-invocation; single-file skill, no skills: list)
           router          = DEFERRED to Wave-2 (mochiko router index — Implement-cluster entry, see Wave-2 note)
           triggers        = re-anchored "when the user says…" → work-context RFC-2119 (MUST/SHOULD grammar kept);
                             de-collided with executing-tdd-cycle (P4) in the description boundary clause
           rebinds         = "the cycle report" flag-sink ×5 → role-neutral "your report / surface it";
                             one canonical reference bound caller-side to mochiko:executing-tdd-cycle (P4), no schema restated;
                             soft workflow-phase vocab (previous cycles L21 / during cycle execution L27) → intrinsic craft
           single-source   = [EXTEND]/[MODIFY] marker VOCABULARY referenced (patterns-vertical-tdd, canonical owner), not restated —
                             the HIL "Meaning" column + one-line glosses removed; interface-impact half retained
Trace (realized): every P6 responsibility → final tag (table below); no silent loss; zero drops.
```

---

## The marker dedupe as realized (Seam 5b — reference vs. retain, line-by-line)

`patterns-vertical-tdd` is the canonical **author/owner** of the `[EXTEND]`/`[MODIFY]` marker vocabulary (its Markers table, SKILL.md L146–151). P6 **consumes** those markers at implement-time. The HIL body restated their one-line meaning as if authoring it; that duplication is removed. This is **dedupe/reference, NOT merge** — the two are disjoint by altitude (design-time *tagging* vs implement-time *consumption/interface-impact*) and by cluster (tasks vs implement).

### REFERENCED (the vocabulary — ceded to `patterns-vertical-tdd`, not restated)
| HIL surface | Restatement removed | Realized |
|-------------|---------------------|----------|
| `## Core Process` → "EXTEND vs. MODIFY Semantics" table, **Meaning** column (L33–36) | `[EXTEND]` = "Add new code alongside existing code"; `[MODIFY]` = "Change existing behavior" — the one-line gloss | **Meaning column deleted.** Table lead-in now names `patterns-vertical-tdd` as owner of what the markers *mean* (SKILL L33); the section is retitled **"EXTEND vs. MODIFY: interface impact"** to signal it carries the consumption rule, not the definition. |
| Overview (HIL L10) — "When a task says `[EXTEND]`, add new code following existing patterns. When a task says `[MODIFY]`, change specific behavior…" | prose gloss reading as an authoritative definition | **Reframed to consumption** (SKILL L10): "the marker **vocabulary** is defined by `patterns-vertical-tdd`… This skill does not redefine the markers — it is the implement-time discipline of **consuming** one safely." |

Self-scan confirms zero residual authoring gloss: `grep -niE '\| *Meaning *\||Add new code alongside existing|Change existing behavior'` → **CLEAN (0)**.

### RETAINED (implement-time consumption semantics — the half `patterns-vertical-tdd` does NOT carry)
The interface-impact rules per marker (the **Scope of change** + **Interface impact** columns — SKILL L37–38); the 5-step read-before-write checklist (full file / naming / error-handling / imports / tests); interface preservation (no signature/export/name changes; add alongside); conflict detection (name / import / test-file alignment / circular deps); the "flag, don't silently resolve" blocker-surfacing discipline; the code-modification-**scope** anti-rationalization spine (Common Mistakes / Rationalizations / Red Flags / "No exceptions"). All `kept`.

**Boundary integrity:** the marker grammar stays *owned* by `patterns-vertical-tdd`, *consumed* by P6 — the direct analog of the P4↔`patterns-vertical-tdd` `**TEST:**` boundary (reconcile §Seam 5a), extended to the marker grammar.

---

## The cycle-report flag-sink rebind as realized (×5 — reconcile: "reference the artifact, don't restate its schema")

The `cycle-report.md` artifact is owned by `executing-tdd-cycle` (P4). "Flag, don't silently resolve" is intrinsic craft and **stays**; the *sink* is bound caller-side. Each of the 5 HIL "cycle report" occurrences:

| # | HIL line | HIL phrasing | Realized |
|---|----------|--------------|----------|
| 1 | L38 | "flag it in the cycle report — do not silently rewrite" | **"surface it as a blocker — do not silently rewrite"** (SKILL L40) — role-neutral |
| 2 | L72 | "Flag in the cycle report (do NOT silently resolve) when:" | **the single canonical reference** — "…they belong in the cycle report the run produces (owned by `executing-tdd-cycle`), not in a quiet workaround:" (SKILL L74) — references the artifact + owner once, no schema (no `attempt`/`cycle:` frontmatter restated) |
| 3 | L90 | "The cycle report doesn't explain why unrelated files were modified." | **"Your report doesn't explain why unrelated files were modified."** (SKILL L92) — role-neutral |
| 4 | L104 | "Note the improvement opportunity in the cycle report." | **"Note the improvement opportunity in your report."** (SKILL L106) — role-neutral |
| 5 | L122 | "Flag it in the cycle report. Don't silently refactor." | **"Flag it — don't silently refactor."** (SKILL L124) — role-neutral |

Result: exactly **one** attributed reference to the cycle-report artifact (bound to P4), the other four softened to role-neutral "your report / surface it." The skill no longer reads as owning that artifact. Self-scan: `grep -niE 'cycle report'` → 1 hit (SKILL L74, the attributed reference). ✓

---

## Soft workflow-phase vocab softened (assess L21/L27 → intrinsic craft)

| HIL line | HIL phrasing | Realized (intrinsic craft) |
|----------|--------------|----------------------------|
| L21 | "When following patterns established by **previous cycles**" | **"following patterns established by prior work in the codebase"** (SKILL L21) |
| L27 | "Refactoring work (which should not happen **during cycle execution**)" | **"Refactoring work — out of scope for an extend/modify task; note the opportunity, do not act on it"** (SKILL L27) |

Self-scan: `grep -niE 'previous cycles|during cycle execution'` → **CLEAN (0)**. Keystone-clean — a professional extending existing code on ANY job follows prior patterns and doesn't refactor out of scope; only the workflow-phase *labels* were coupling.

---

## Trigger re-anchor + P4 de-collision as realized (wiring step 3)

**Re-anchor (HIL L3 → SKILL L3).** The HIL `MUST` clause led with **"when the user says 'brownfield integration', 'extend existing code', or 'modify existing file'"** (user-utterance framing — wrong for an agent-consumed producer-craft skill). Re-anchored to **work context**, grading grammar **kept**:
- `MUST` → "when implementing a task that touches existing code — safely making an `[EXTEND]` or `[MODIFY]` change to a file already on disk" (+ the four craft verbs).
- `SHOULD` → "extending an existing file, modifying existing behavior, integrating against an established interface, or following patterns set by prior work" (the HIL SHOULD substance, re-anchored; exact marker tokens preserved).

**De-collision with `executing-tdd-cycle` (P4)** (reconcile §Job 1.6 — both legitimately co-fire on a brownfield cycle; scope each so they don't fight). The description's boundary clause scopes P6 explicitly:
> "…this is the implement-time, read-before-write craft of making that one modification safely — **NOT the execution of the cycle the task belongs to (that is executing-tdd-cycle, which co-fires on the same brownfield task and drives red/green/refactor).**"

So: **P4** = executing a cycle that *contains* an `[EXTEND]`/`[MODIFY]` task (red/green/refactor); **P6** = the read-before-write craft of safely *making* that one modification. Both fire cleanly on the same brownfield task; no merge. (`executing-tdd-cycle` is a **sibling-skill** name for boundary-setting — not on the decoupling deny-list, which targets agent names / "dispatch" / "workflow-agnostic"; naming it is the auditable de-collision, and single-sourcing requires the reference.)

---

## Disjointness vs `analysis-codebase` (confirmed — no merge, no overlap)

Re-confirmed at transform time (assess §Disjointness). Different **act**, not a shared core:

| Axis | `analysis-codebase` (setup, ported) | `brownfield-integration` (implement, this port) |
|------|-------------------------------------|--------------------------------------------------|
| Act | DETECT stack/architecture/conventions to **inform governance** | Safely **EXTEND/MODIFY** one existing file at implement-time |
| Phase | setup (before a constitution exists) | implement (writing code against existing code) |
| Output | artifact `.mochiko/memory/codebase-analysis.md` | shaped code + surfaced flags (no artifact of its own) |
| "Read existing code" | breadth — whole project once, to *characterize* | depth — the one file about to be touched, to *preserve its interface* |

Only surface overlap is the word "brownfield" + the verb "read existing code"; underneath they are opposite acts. No shared core, no merge, no trigger collision (`analysis-codebase` fires on "analyze codebase / detect stack / brownfield governance"; this skill fires on `[EXTEND]`/`[MODIFY]` / implementing against existing code). **Fully disjoint.**

---

## Decoupling self-scan (grep-anchored — the way `verify-output` will)

| Scan | Pattern class | Result |
|------|---------------|--------|
| **Deny-list** | `staff-engineer`, `qa-engineer`, `supervisor`, `state-analyst`, `dispatch`, `workflow-agnostic`, `principal-architect`, `.humaninloop`, `hil-dag`, `catalog`, `DAG`, `MCP`, `.workflow/` | **CLEAN (0)** |
| **Soft workflow-phase vocab** | `previous cycles`, `during cycle execution` | **CLEAN (0)** — softened |
| **Marker vocab restatement** | `\| Meaning \|`, "Add new code alongside existing", "Change existing behavior" | **CLEAN (0)** — deduped to `patterns-vertical-tdd` |
| **Cycle-report as own sink** | `cycle report` | **1** — the single attributed reference (owned by `executing-tdd-cycle`), by design |
| **Independence** | any mounted grading / `verify-*` / `testing-end-user` / `validation-*` | **N/A** — single-file skill, no `skills:` list; pure producer-side craft, no grading function |

Positive wiring confirmed: `patterns-vertical-tdd` (marker owner) referenced ×3; `executing-tdd-cycle` (cycle-report owner + P4 de-collision) referenced ×2; KEEP anchors present (Read-Before-Write / Interface Preservation / Conflict Detection / Common Rationalizations / "No exceptions").

---

## Realized responsibility trace (every P6 responsibility → final tag)

Mirrors the assess trace + reconcile §Job-3 P6, tags flipped to **realized**. No responsibility dropped — this port has **zero drops** (all delta is `kept` / `kept-but-rebind` / one `dedupe`, consistent with `port-with-edits × standalone`).

| # | Responsibility | Assess tag | Realized |
|---|----------------|-----------|----------|
| 1 | EXTEND/MODIFY consumption semantics (interface-impact rules per marker) | `kept` | **kept** — "EXTEND vs. MODIFY: interface impact" table, Scope + Interface-impact columns (SKILL L37–38) |
| 2 | `[EXTEND]`/`[MODIFY]` marker **vocabulary definition** (they exist + 1-line gloss) | `dedupe → patterns-vertical-tdd` | **dedupe** — Meaning column + prose gloss removed; `patterns-vertical-tdd` named as owner (SKILL L10, L33); cross-cluster single-source, NOT a merge |
| 3 | Read-before-write checklist (5 steps) | `kept` | **kept** — verbatim (SKILL L42–50) |
| 4 | Interface preservation (no signature/export/name changes; add alongside) | `kept` | **kept** — verbatim (SKILL L52–61) |
| 5 | Conflict detection (name / import / test-file alignment / circular deps) | `kept` | **kept** — verbatim (SKILL L63–70) |
| 6 | "Flag, don't silently resolve" blocker-surfacing discipline | `kept` | **kept** — "When to Flag" (SKILL L72–80); discipline intact, sink rebound caller-side |
| 7 | The flag SINK reference "the cycle report" (×5) | `kept-but-rebind` | **kept-but-rebind** — ×5 softened to role-neutral; 1 canonical reference bound to `executing-tdd-cycle` (P4), no schema restated |
| 8 | Soft workflow-phase vocab (`cycle execution`, `previous cycles`) | `kept-but-rebind` | **kept-but-rebind** — "prior work in the codebase" (L21); "out of scope for an extend/modify task" (L27) |
| 9 | Anti-rationalization spine (Common Mistakes / Rationalizations / Red Flags / "No exceptions") | `kept` | **kept** — verbatim (SKILL L82–132); **code-modification-SCOPE craft**, NOT loop-discipline's loop-gaming doctrine → NOT deduped to `loop-discipline` |
| 10 | When-to-use / when-NOT-to-use scoping (EXTEND/MODIFY tasks; not greenfield/refactor) | `kept` | **kept** — SKILL L16–27 (L21/27 soft vocab softened per #8) |
| 11 | Model-invocation triggers (graded MUST/SHOULD, exact phrases) | `kept-but-rebind` | **kept-but-rebind** — grammar graded ✓; re-anchored "when the user says…" → work-context; de-collided with P4 (SKILL L3) |
| 12 | Classification (model-invoked) + router registration | `kept` + wiring-pass | **kept** (model-invoked, default) + router **DEFERRED to Wave-2** (Implement-cluster entry) |

**No silent loss.** Zero `dropped` rows (P3/P5/P6 all had 0 drops per reconcile §gated-bundle-9). Every responsibility carries a realized tag → transform done-condition met.

*(Home cluster setup→implement is a REGISTRY re-file — metadata, not a responsibility — recorded in the Phase-5 note below.)*

---

## Phase-5 REGISTRY delta (record for the finalize step — reconcile §Phase-5, assess §Home-confirmation)

The row **moves Setup-cluster → Implement-cluster** (mis-file corrected; this skill reads as implement-time craft, confirmed by staff-engineer mounting it as its 2nd skill):
- **REMOVE** from `### Setup workflow cluster` (REGISTRY L80): `[ ] brownfield-integration — Deferred — out of setup-core scope (REGISTRY mis-file; reads as implement-time)`.
- **ADD** to `### Implement / execute cluster` (alongside `executing-tdd-cycle`, `testing-end-user`), mark `[x]`: `brownfield-integration — Ported (implement, 2026-07-01) — staff-engineer's 2nd skill (EXTEND/MODIFY interface-impact consumption + read-before-write + interface preservation + conflict detection); [EXTEND]/[MODIFY] marker vocabulary deduped → patterns-vertical-tdd (canonical owner), retains implement-time consumption/interface-impact semantics.`

---

## Wave-2 wiring needed (DEFERRED — not this pass)

1. **Router index** — register `brownfield-integration` in the `mochiko` router under a new **Implement / execute cluster** table (model-invoked, auto-reached during a `/mochiko:implement` run) with when-to-reach guidance: *the implement producer's read-before-write brownfield craft — safely making an `[EXTEND]`/`[MODIFY]` change to an existing file (interface preservation + conflict detection); consumes the marker vocabulary `patterns-vertical-tdd` owns; distinct from `executing-tdd-cycle`, which executes the cycle the task belongs to.* Deferred with the rest of the implement-cluster router entries (per landed siblings `staff-engineer` / `implement`, whose router + `plugin.json` registration are Wave-2).
2. **Cluster co-entries (noted for completeness; not this skill's move):** the same Wave-2 pass registers `executing-tdd-cycle` (P4), `testing-end-user` (P5), the `staff-engineer`/`qa-engineer` agents, and the `implement` command under the Implement cluster. This skill contributes only its own router row.

---

## Handoff

Artifact: `plugins/mochiko/skills/brownfield-integration/SKILL.md`
Trace: this file.
Next: independent grade by `verify-output` (a **different** agent) — the five conventions, sound-loop placement (good citizen inside the implement loop; owns no done-condition/validation/human-gate of its own, correctly), kernel-free, the decoupling scan, the Seam-5b dedupe (reference not restate), the ×5 cycle-report rebind, and the trace audit for silent loss (expect: zero drops).

**Transform version:** v1 · **Governed by:** `transform-recipes` · defaults to no self-grade.
