# Roadmap-v2 re-baseline — the foundational decisions

**Status:** ruled · **Date:** 2026-06-27 (confirmations through 2026-07-01)
**Context:** the 2026-06-27 synthesis refresh (sound-loop cluster, `deterministic-core-llm-shell` → adopt, the human-validator omission) forced a re-baseline of roadmap v1; the setup/specify/plan/tasks/implement ports then confirmed the empirical calls. No session record exists — this file carries the rulings; full rationale + trail: `.mochiko/archive/ROADMAP.md` (Key Decisions + Decision Trail).

## Decisions

- **Central thesis** — the human is mochiko's primary external validator, present by design in every workflow loop.
- **Kernel/code** — deferred, code-free until dogfooding (supersedes v1's "kernel-free, Confident"); payoff scoped to parallel/dependency-rich work.
- **Loop discipline** — doctrine skill (`loop-discipline`) + `workflow-contract` template, zero code.
- **Validation mechanism** — producer↔validator skill pairing at artifact granularity: different agent + different skill, structural independence.
- **Primary quality surface** — skill library + agents, not plumbing.
- **Build approach / order** — workflow-first, one at a time; `setup` then `specify`.
- **Migration tracking** — explicit REGISTRY (superseded 2026-07-25: REGISTRY retired, operating-docs-maintenance D8).
- **Human-gate placement** (2026-06-27, confirmed by ports) — gated dispositions + escalations, plus a named acceptance gate on the deliverable.
- **Memory model** (2026-06-27) — in-session + workspace-as-state under `.mochiko/`; no context-handoff file.
- **Gap classification** (2026-06-27) — FAIL-routing folded into `loop-discipline` (knowledge → research · preference → human gate · scope → halt).
- **`implement` orchestration** (2026-07-01) — sequential-first thin lead; parallelism + kernel/DAG deferred to dogfooding (shortcut-ledger entry).

**Alternatives considered:** per decision in the archive's Decision Trail (kernel-free vs defer; thesis vs open question; hooks vs skill+template; per-artifact vs per-skill pairing).
