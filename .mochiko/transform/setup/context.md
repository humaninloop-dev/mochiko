# Transform Run Context — cluster: `setup`

## User request

> i want to transform humaninloop setup, which is constitution setting workflow, all agents and skills under it

Invoked via `/mochiko:transform-cluster setup`.

## Scope decisions (Phase 0 human gate)

- **Cluster scope:** **Core only** (user-selected). Port the 4 core skills + command + agent + 4 templates. `syncing-claude-md` and `authoring-roadmap` are **cross-cutting** — rebind references only, port later with their own clusters. `brownfield-integration` is **deferred** (REGISTRY mis-filed it under setup; agent/command never invoke it).
- **Human-gate placement:** on gated dispositions + escalations (loop-discipline default).

## Resolved primitive list (10)

Source root: `human-in-loop/plugins/humaninloop/`

| # | Type | Path | Notes |
|---|------|------|-------|
| P1 | command | `commands/setup.md` | IS-a-loop. Brownfield-aware constitution supervisor; 5 phases; dispatches principal-architect. |
| P2 | agent | `agents/principal-architect.md` | PLAYS-a-role. Declares `skills: authoring-constitution, brownfield-constitution, validation-constitution, analysis-codebase, syncing-claude-md, authoring-roadmap`. |
| P3 | skill | `skills/authoring-constitution/` | SKILL.md + refs: RECOMMENDED-PATTERNS, RFC-2119-KEYWORDS, SYNC-IMPACT-FORMAT. Core of setup. |
| P4 | skill | `skills/brownfield-constitution/` | SKILL.md + refs: EMERGENT-CEILING-PATTERNS, ESSENTIAL-FLOOR. Brownfield variant. |
| P5 | skill | `skills/validation-constitution/` | SKILL.md + refs: ANTI-PATTERNS, QUALITY-CHECKLIST. Gates setup completion. |
| P6 | skill | `skills/analysis-codebase/` | SKILL.md + refs: BROWNFIELD-ANALYSIS, CONTEXT-GATHERING + scripts/detect-stack.sh. Feeds brownfield analysis. |
| P7 | template | `templates/constitution-template.md` | |
| P8 | template | `templates/constitution-context-template.md` | |
| P9 | template | `templates/codebase-analysis-template.md` | |
| P10 | template | `templates/codebase-inventory-schema.json` | |

**Cross-cutting (referenced, NOT ported this run):** `skills/syncing-claude-md/`, `skills/authoring-roadmap/`, `templates/evolution-roadmap-template.md`.
**Deferred (out of scope):** `skills/brownfield-integration/`.
**Excluded (per REGISTRY, brain-mediated):** `scripts/setup-plan.sh`.

## Target tree

Mochiko plugin: `plugins/mochiko/` (commands/, agents/, skills/, templates/).

## Contract status

`contract.md` filled, all four sections, no open brackets. Audited against `loop-discipline` (Phase 0 step 3):
- (1) done-condition pre-declared, defaults FAIL ✓
- (2) producer ≠ validator agent, skills disjoint ✓ (Tier-2 grounded validation)
- (3) bounded: round cap 3/primitive, no-progress exit, STOP sentinel + 2×-primitive ceiling, escalate-don't-die ✓
- (4) human gate named: gated dispositions + escalations ✓

**Run state:** FAIL (default). Phase 0 complete → entering Phase 1 (assess).

---

## Phase 1 — assessment results (all 10 in)

| # | Primitive | Disposition | Key signal |
|---|-----------|-------------|------------|
| P1 | setup command | **redesign** × flag-for-reconcile | loop spine unsound: no default-FAIL, no independent validation gate, on-cap declares done. Phase content ports w/ edits. **GATED (redesign).** |
| P2 | principal-architect | port-with-edits × flag: **split** | self-grade leak in `skills:` (co-mounts produce + grade); feasibility-review proc orphaned; essential-floor table dedupe |
| P3 | authoring-constitution | port-with-edits × flag | F1 absorb brownfield; F2 independence split; F3 rehome validation gate; F4 trigger de-collide |
| P4 | brownfield-constitution | port-with-edits × flag | **merge-into-sibling vs standalone** (extends authoring, doesn't duplicate); essential-floor dedupe; approved-domain-deps ref out-of-scope |
| P5 | validation-constitution | keep-verbatim × flag | **promote** onto NEW independent constitution-validator agent; shared validator for P3+P4; rehome missing verdict-sink+FAIL-loop+human-gate |
| P6 | analysis-codebase | port-with-edits × flag | mode-scoping split (setup-brownfield slice + script vs other-cluster modes); essential-floor dedupe vs P4; validator/loop placement. `detect-stack.sh` clean (keep) |
| P7 | constitution-template | **keep-verbatim × standalone** | soft: approved-domain-deps.md ref (out of scope) |
| P8 | constitution-context-template | port-with-edits × flag | **BLOCKING**: state-carrier fate (absorb-into-lead / drop / standalone); P1 inlines its own context → cluster must end with ONE state mechanism |
| P9 | codebase-analysis-template | **keep-verbatim × standalone** | clean |
| P10 | codebase-inventory-schema.json | **keep-verbatim × standalone** | orphan (referenced by nothing); other-cluster candidate; R9 zero-enforcement drop candidate |

**Convergent reconcile agenda (for Phase 2):**
1. **Independence split** (P1/P2/P5) — new `constitution-validator` agent owns `validation-constitution`; `principal-architect` becomes producer-only.
2. **Sound-loop rehome** (P1) — build default-FAIL done-condition, produce→validate FAIL-loop, escalate-on-cap, human acceptance gate on P1 lead.
3. **Merge decision** (P3/P4) — absorb brownfield-constitution into authoring-constitution, or keep standalone.
4. **Essential-floor dedupe** (P4/P6) — canonical home (require-floor vs assess-status).
5. **State carrier** (P8/P1) — one mechanism; absorb-into-lead likely.
6. **Cross-cluster rebinds** — roadmap + claude-md-sync stubs; approved-domain-deps ref drop/defer; codebase-inventory-schema other-cluster vs standalone-now.
7. **Mode-scoping** (P6) — setup port carries setup-brownfield slice + script only.

## Running log

- **Phase 0 — Intake & contract:** DONE. Scope confirmed with user (core-only). Workspace + contract + context created.
- **Phase 1 — Triage & assess:** DONE. 10 primitives assessed via 7 producer dispatches (templates grouped). All trace tags complete; relational moves held as flag-for-reconcile.
- **Phase 2 — Reconcile:** DONE. `reconcile.md` written, zero open flags. **Human gate cleared** (2026-06-27):
  - Redesign + independence split (new `constitution-validator` agent; principal-architect producer-only) → **ACCEPTED**.
  - Merge brownfield-constitution into authoring-constitution (greenfield|brownfield branch) → **ACCEPTED**.
  - Scope reductions accepted: P8 constitution-context-template absorbed-into-lead (body dropped); P10 codebase-inventory-schema.json moved-to-other-cluster; approved-domain-deps.md reference dropped → **ALL ACCEPTED**.
- **Phase 3 — Transform:** DONE. 7 producer transforms + consolidated router-wiring pass. Produced: commands/setup.md (redesign), agents/principal-architect.md (producer-only) + agents/constitution-validator.md (NET-NEW), skills/{authoring-constitution (absorbs brownfield), validation-constitution, analysis-codebase (setup slice)}, templates/{constitution-template.md, codebase-analysis-template.md}, router registered. NOT produced: brownfield-constitution (merged), constitution-context-template (absorbed), codebase-inventory-schema.json (moved). Deterministic leak sweep clean.
- **Phase 4 — Verify:** DONE. 7 validator dispatches (transform-validator), **all PASS**, zero required fixes. Trace audits PASS (no silent drops). One recorded deviation (cleanup-gate G4 retained vs reconcile's planned drop — safe direction, validator-PASSED). No FAIL → no fix-loop rounds needed (0 of 3 used).
- **Phase 5 — Finalize:** DONE. REGISTRY.md updated; report.md written. Run state → **DONE** (done-condition met).

## Run state: DONE
Done-condition met: verify-output PASS for every primitive · every responsibility traced (no silent drops) · human gate cleared all gated dispositions. See `report.md`.
- **Phase 3 — Transform:** pending.
- **Phase 4 — Verify:** pending.
- **Phase 5 — Finalize:** pending.
