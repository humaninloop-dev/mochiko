# Transform Run Report — cluster `setup`

**Date:** 2026-06-27 · **Workflow:** `/mochiko:transform-cluster setup` · **Scope:** core-only (user-selected) · **Run state:** **DONE** (done-condition met)

## Done-condition: MET

- `verify-output` returned **PASS** for every produced primitive (8 verify dispatches, all PASS, zero required fixes).
- Every original responsibility carries a realized trace tag — trace audits PASS on all primitives, no silent drops.
- Human gate cleared all gated dispositions (Phase 2) — redesign, merge, absorb, promote, and the dropped reasons.

## Per-primitive outcome

| # | Primitive | Disposition (resolved) | Verdict | Landed as |
|---|-----------|------------------------|---------|-----------|
| P1 | setup command | redesign (loop spine) + rehome | **PASS** | `plugins/mochiko/commands/setup.md` — sound loop |
| P2 | principal-architect | port-with-edits × split (producer-only) | **PASS** | `agents/principal-architect.md` (`skills: authoring-constitution, analysis-codebase`) |
| — | constitution-validator | **net-new** (promote target of the split) | **PASS** | `agents/constitution-validator.md` (`skills: validation-constitution`) |
| P3 | authoring-constitution | port-with-edits × merge (absorber) | **PASS** | `skills/authoring-constitution/` (greenfield\|brownfield branch; +ESSENTIAL-FLOOR, +EMERGENT-CEILING refs) |
| P4 | brownfield-constitution | merge-into-sibling | **PASS** | merged into P3 — **no standalone skill** |
| P5 | validation-constitution | keep-verbatim × promote | **PASS** | `skills/validation-constitution/` (body byte-identical; promoted onto constitution-validator) |
| P6 | analysis-codebase | port-with-edits × standalone (setup slice) | **PASS** | `skills/analysis-codebase/` (+`detect-stack.sh` byte-faithful; other modes stubbed to spec/plan) |
| P7 | constitution-template | keep-verbatim × standalone | **PASS** | `templates/constitution-template.md` |
| P8 | constitution-context-template | absorb-into-lead (body drop) | **PASS** | absorbed into P1 lead — **not produced** |
| P9 | codebase-analysis-template | keep-verbatim × standalone | **PASS** | `templates/codebase-analysis-template.md` |
| P10 | codebase-inventory-schema.json | moved-to-other-cluster | **PASS** | deferred to spec/plan — **not produced** |
| — | router wiring | convention-wiring | **PASS** | `skills/mochiko/SKILL.md` — setup cluster registered |

## The core transformation (what mochiko added that HIL lacked)

HIL `setup` was a markdown supervisor with **no independent validation gate**, an **unsound done-condition** (LLM-controlled; on-cap *declared done*), and **no constitution-acceptance human gate**. The redesign installed all three:
- **Independence:** the self-grade leak (principal-architect co-mounted author + grade skills) is closed by splitting grading onto the net-new `constitution-validator`. Producer ∩ validator skills = ∅.
- **Sound loop:** default-FAIL done-condition; produce→validate→repeat with round cap 3 + no-progress exit + STOP kill-switch + **escalate-on-cap** (never done on exhaustion).
- **Human gates:** mode-select, analysis checkpoint, the NEW constitution-acceptance gate, cleanup, + escalation.

## Accepted drops (human-gated, Phase 2)

- HIL's LLM-controlled "no-clarifications OR max-3 → proceed" done-condition — **dropped** (replaced by default-FAIL). Reason: loop-discipline violation.
- `constitution-context-template` (P8) ephemeral state-carrier — **absorbed-into-lead**; state now in-session + `.mochiko/memory/`.
- `codebase-inventory-schema.json` (P10) — **moved-to-other-cluster** (orphan in setup; ports with spec/plan).
- `approved-domain-deps.md` reference — **dropped** to prose (out-of-scope template; a live path would dangle).

## Deviations & non-blocking notes (recorded, validator-PASSED)

1. **Cleanup gate (G4) — deviation from reconcile plan, safe direction.** Reconcile planned to *drop* the cleanup gate (no ephemeral file to clean); the producer instead *retained* a repurposed workspace retain/clean prompt over the real `codebase-analysis.md` (never offers to delete `constitution.md`). Validator judged it acceptable — adds a human gate, loses no capability, fully documented. **Surfaced for the record; no fix.**
2. Cosmetic wording-consistency notes (optional, no fix): `RECOMMENDED-PATTERNS.md` L82 names the dep registry in prose; cross-cutting skill re-mounts lack inline notes in `principal-architect`; `validation-constitution` assessment's "12 vs 14" trace count label; `detect-stack.sh` `set -e` portability nit; reconcile §D text says "register templates" though templates aren't router-registered (plan-text vs convention).

## Open follow-ups (for ROADMAP.md / BACKLOG.md)

- **Cross-cluster stubs to wire when their clusters port:** `syncing-claude-md` (CLAUDE.md sync) and `authoring-roadmap` (evolution-roadmap) are documented stubs in setup, not live mounts. Re-mount when those clusters land.
- **`codebase-inventory-schema.json`** belongs to the spec/plan cluster (analysis-codebase collision mode) — port it there with its real consumer.
- **`brownfield-integration`** remains deferred — confirm it is implement-time, not setup, when that cluster is scoped (REGISTRY mis-file).
- **`approved-domain-deps.md`** — decide whether to ship a registry template later or keep as project-maintained prose.
- **Memory-model backlog item** (`.mochiko/memory/` as the setup state home) — this run adopted in-session + `.mochiko/memory/`; consider closing the BACKLOG "Memory model" item with this decision.
