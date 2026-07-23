# Run Costs — {feature}

> Per-run cost ledger for this feature's workflow runs, one row per run, newest last. Single home of
> the row format — commands reference this template, never restate it.
>
> **Manual baseline protocol** (epic ruling — `.mochiko/brainstorms/workflow-token-reduction/record.md`
> D2): at finalize the lead records the user-visible `/usage` figure (supplied by the user; write
> `not captured` when unavailable — never a blocker) plus the run shape the lead itself observed —
> seats spawned, per-phase rounds, review sizing. Deeper per-seat token attribution needs a
> transcript parse and stays an on-demand, human-initiated step. Not automated; recorded with that
> limitation stated. (Upgrade path: the one-shot OTel probe on the build list.)

| Date | Workflow | Slice | Usage (`/usage`) | Seats | Rounds | Review sizing | Notes |
|------|----------|-------|------------------|-------|--------|---------------|-------|
| YYYY-MM-DD | plan | s1 (or —) | e.g. 210k out (est.) / not captured | producer + N reviewers | P1×N · P2×N | both / completeness-only / none (waived) | anything of note |
