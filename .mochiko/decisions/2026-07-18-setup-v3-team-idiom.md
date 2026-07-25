# Setup-v3 build-time ruling — adopt the brainstorm team idiom

**Status:** ruled · **Date:** 2026-07-18
**Context:** the first dogfood setup run executed via subagents despite the hard team mandate. Side-by-side reading found setup's loop written in dispatch vocabulary (`Task()`-satisfiable) while brainstorm defines seats by messaging semantics. Full trail: `.mochiko/archive/ROADMAP.md` (Decision Trail, "setup v3").

**Decision:** setup adopts brainstorm's seat idiom wholesale — named standing teammates, results by mailbox, round > 1 a `SendMessage` to the same seat, pause posture on the deliverable.

**Rationale:** vocabulary at the decision moment selects the transport; dispatch-shaped prose documentedly resolves to subagents. (The residual set-and-ignored defect was diagnosed separately: `setup-v3-team-defect` record.)

**Alternatives:** keep dispatch vocabulary + strengthen the mandate line (rejected — the mandate was already ignored once).
