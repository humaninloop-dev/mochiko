# Architect Role — Scope, Pushback, and Level of Abstraction

**Status:** open
**When:** 2026-08-12
**Topic:** Deep analysis of the scope and impact of changing the principal-architect role. The user's felt problem: the principal-architect is not doing what a principal architect should in mochiko. Wider lens: architect roles in general — are they pushing back enough, and are they ensuring the right level of abstraction?

**Session form:** brainstorm (worktree `principal-architect-role`, base a1c4bf1) — lead-run `mochiko:analysis-iterative` questioning, record written as-you-go, end-stage cold review per `mochiko:review-brainstorm` unless waived.

---

## Ground facts

- **F1 — Two architect personas exist.** `principal-architect` (`plugins/mochiko/agents/principal-architect.md`) and `system-architect` (`plugins/mochiko/agents/system-architect.md`). Both `model: opus`.
- **F2 — principal-architect's actual remit is governance + feasibility, not system design.** Skills: `authoring-constitution`, `analysis-codebase`, `review-feasibility`, `authoring-architecture`. Duties: author/amend the governance surface set, run brownfield codebase analysis, hunt cross-artifact contradictions (3-state feasible/needs-revision/infeasible verdict), maintain repo `ARCHITECTURE.md`. Its "pushback" doctrine is aimed at *standards*: the Three-Part Rule (enforcement · testability · rationale), "You are opinionated. You push back on vague requirements."
- **F3 — system-architect owns topology.** Skills: `patterns-system-design`, `patterns-technical-decisions`. Duties: container-level topology, boundaries, interaction styles, delta-from-current-state. Its rejection list includes speculative components, smallest-shape-that-works, extension-over-invention. Born at `architecture-design-primitive` session (AD-D7, `Contested`) 2026-07-30.
- **F4 — Commands never name agents.** v8 goal+harness form: the lead composes seats per run ("teammates or subagents per seat is your call"). Grep confirms: `principal-architect` / `system-architect` appear in no command file; only the router skill (`plugins/mochiko/skills/mochiko/SKILL.md`) indexes them. Role reach is therefore defined entirely by persona text + attached skills + router description.
- **F5 — Neither persona carries an explicit mandate to challenge the *user's* direction or arbitrate the design's altitude.** principal-architect pushes back on vague standards; system-architect rejects bad shapes it is asked to draw. No line in either persona (or their skills' descriptions) assigns a duty to contest scope, question whether a feature should exist, or rule on the abstraction level of artifacts produced by *other* seats.

- **F6 — The driver case: kinako FEAT-002 (preflight verification).** Feature = four launch checks (binary present · authenticated · env-var precedence · corpus writable), report each, block start until pass. Plan artifacts total **3,493 lines**; `architecture.md` alone 469 lines, **15 diagram nodes**, 4 sequence diagrams, a run-epoch protocol with bidirectional epoch comparison, subscribe-before-snapshot ordering rules, session-lifetime acceptance sets. (`~/Documents/GitHub/kinako/.mochiko/features/FEAT-002/`.)
- **F7 — The altitude bar already exists in writing.** `patterns-system-design` mandates C4 **container** level: register rows are "one line per deployable/runnable piece … the container-level register sense, not C4-level-3"; default bound ~12 rendered nodes (a *count* bound, overridable). The bound was overridden at FEAT-002 (15 nodes, reason recorded, plan lead approved).
- **F8 — FEAT-002 drifted below its own bar and every gate passed it.** Register rows "Preflight domain (Rust · no I/O)", "Preflight service (application layer)", "Engine port trait" are code-level layers inside one Rust process — C4 level-3 constructs, not containers. Signed off by the principal 2026-08-12. Partial mitigation: some structure is governance-forced (repo `ARCHITECTURE.md` committed structure: engine port, single typed IPC client, four security boundaries; IP-001 CI-reproducibility forces injectable seams).
- **F9 — The review economy is asymmetric: every grader is chartered to find *missing*, none to find *excess*.** FEAT-002 round 1 (feasibility `needs-revision`, completeness `critical-gaps`): all 8 dispositions **added** machinery — SYS-002 diagram added, a newer-epoch branch explicitly logged as "over-delivered", an acceptance-set register row added. Zero findings said "too much" or "wrong altitude".
- **F10 — The persona value exists; no skill carries it.** `principal-architect` judgment #4: "Is it necessary? If complexity isn't justified, reject it." But its plan-review skill (`review-feasibility`) hunts contradictions/buildability only — over-engineering is buildable, so the value never fires at any checkpoint. Mochiko's only minimalism machinery is build-time, on code (`patterns-code-minimalism` ladder + `review-code-minimalism` lens, advisory, qa seat) — no design-time analog.

## Decisions

*(none yet)*

## Open questions

*(populated as elicited)*

## Session trail

*(Q/A summary as the session progresses)*
