# Persona-edit ceremony hook — an advisory `pre`/`post` grid read at every kitted persona edit

- **Status:** ruled
- **Date:** 2026-09-09
- **Context:** After two pilots and wave A, four personas carry a plan-only eval kit under
  `evals/agents/<persona>/` (rubric, goldens, pre-registration with a measured band, a baseline
  grid). Nothing in the ceremony asked for it: an edit to `plugins/mochiko/agents/<persona>.md`
  took the strip entry and the author≠grader audit and never ran the grid. The user ruled
  (2026-09-09): add the hook as recommended.
- **Decision:**
  1. **A persona edit's audit brief cites a `pre`/`post` grid read** — `agent-grid <persona>
     --arms pre,post` with `pre` at the kit's pinned `refs.pre` and `post` the working tree, judged
     and reported — whenever the edited persona has a kit under `evals/agents/`. The report's
     regression, adoption, and band lines are quoted in the strip entry or, for a pure addition,
     in the decision row's rationale.
  2. **Advisory only, never a gate** (harness D2, GI-019): the grader reads the grid as evidence
     beside the audit criteria; a regression or a noise-dominated read is a finding to disposition,
     not an exit code. A kit with no `refs.pre` pin, or a persona without a kit, takes the plain
     ceremony.
  3. **The kit moves with the edit**: the edit that lands re-pins `refs.pre` to the landed ref and
     re-runs prune only when it adds or removes claims (a reword keeps its id and its tag).
- **Rationale:** The grid is the only instrument that reads a persona edit against its own
  standards on held-out tasks; without a hook it decays into an artefact. Advisory keeps it inside
  the standing bright line — tooling that gates pipeline progress is kernel-class.
- **Alternatives considered:** a CI job (rejected — metered spend on every push, and the
  instrument is judged, not deterministic); a `plugin.json` bump gate (rejected — a gate).
