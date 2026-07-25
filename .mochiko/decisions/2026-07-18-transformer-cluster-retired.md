# Transformer cluster retired

**Status:** ruled · **Date:** 2026-07-18
**Context:** the HIL→mochiko migration was essentially complete; the porting engine (`transform-cluster` command, `transform-producer`, assess/reconcile/transform/verify skills, `PLAYBOOK.md`) remained wired into the live library. Full detail: `.mochiko/archive/ROADMAP.md` (Key Decisions, "Transformer cluster retired").

**Decision (user-directed):** delete the cluster; keep every ported output, the run archive (`.mochiko/transform/`), and `validator`'s `validation-constitution` half. Earlier decisions naming `verify-output`'s altitude scan become point-in-time records.

**Rationale:** dogfooded scaffolding for a one-time job is dead weight and a router liability once the job is done; history stays in the archives.

**Alternatives:** keep as a dormant maintenance tool (rejected — discoverability cost with no remaining consumer).
