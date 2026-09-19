# mochiko eval evidence — 2026-09-19

Raw per-session evidence behind the primitive-evals-v2 eval reads: the persona
baselines (wave A and wave B), the four skill-slot kits, and the review-brainstorm
post-cut regression grid.

This is an archive ref. It carries evidence only — no kits, no runner, no history.
The kits that produced these runs, their pre-registrations, and the reads drawn
from them live on the main line at `evals/`.

## Why it lives here

The eval kits' own `.gitignore` ignores every `runs/` directory, because run
output is regenerated locally on every grid. Carrying it on the main line meant
force-adding each file, which silently dropped twenty-two sessions once already.
Keeping the evidence on a ref of its own means the pointers resolve for any reader
without the force-add, and the working branch stays reviewable.

## Provenance

- Source branch: `primitive-evals-v2`
- Source commit: `aa9805a52ff8a7063a6636d7bbe6eb89ffc7664c`
- Files: 795 (757 carried on the source branch, 38 previously untracked)

The 38 previously untracked files are the `evals/review-brainstorm/runs/` grids,
cited as evidence by `.mochiko/strips/review-brainstorm.md` but never committed.
That pointer resolved only on the author's machine; it resolves here now.

## Reading a pointer

Citations name a kit-relative run directory — `runs/baseline`, `runs/probe`,
`runs/pilot2-prune`. Resolve one under this ref at the citing kit's path:

    git fetch origin tag eval-evidence-2026-09-19
    git show eval-evidence-2026-09-19:evals/agents/tech-lead/runs/baseline/report.md

Or check the whole tree out somewhere scratch:

    git worktree add /tmp/eval-evidence eval-evidence-2026-09-19

## What is not here

Stream transcripts (`stream.jsonl` and the like) were never committed anywhere in
this work and are not carried here either.
