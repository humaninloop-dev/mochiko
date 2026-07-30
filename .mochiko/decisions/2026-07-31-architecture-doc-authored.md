# Repo ARCHITECTURE.md authored (plugin scope)

**Status:** ruled · **Date:** 2026-07-31
**Context:** the KM pin's partial-core deviation (2026-07-25) left `ARCHITECTURE.md` deferred;
the user directed its creation in-session, ruling the scoping questions directly. No brainstorm
session — this record is the rationale home.

**Decision (user-directed):** author top-level `ARCHITECTURE.md` as the current-state map of
the shipped plugin only (`plugins/mochiko/`), organized both ways — layer model (commands /
agents / skills / templates + boundaries) first, then the workflow-cluster map — with a mermaid
diagram per cluster plus an overview, and graded by a cold reviewer **pair** (accuracy-vs-code
/ doc-fitness lenses; author ≠ grader). The repo-side knowledge plane stays documented by
`CLAUDE.md`, not this doc.

**Rationale:** the plugin is the shipped artifact users install — its system view is the one a
maintainer needs in one sitting; the doc references single-sourced homes (router,
`command-shape.md`) rather than restating them. Review outcome: both reviewers returned FAIL
(1 Important accuracy finding — template overcount; 3 Important fitness findings — home
restated past pointer depth, diagram/table seat mismatches, unglossed `architecture.md` vs
`ARCHITECTURE.md` collision — plus 7 Minors); all folded except one fitness Minor folded
partially (boundaries bullets retained for one-read placeability), verify pass confirmed the
folds. GLOSSARY.md stays deferred — this landing narrows the deviation, not closes it.

**Alternatives:** whole-repo scope (rejected by the user — plugin only); prose-only or
single-diagram form (rejected — diagram per cluster ruled); single reviewer / waiver (rejected
— pair ruled).
