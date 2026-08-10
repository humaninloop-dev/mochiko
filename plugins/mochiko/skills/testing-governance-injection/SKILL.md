---
name: testing-governance-injection
description: This skill MUST be invoked when empirically probing that an accepted governance surface set actually DELIVERS — that `.claude/rules/mochiko/` files inject on their promised paths, that the CLAUDE.md governance region reaches spawned agents, and that injected rules change behavior. SHOULD also invoke for a 'governance injection probe' or a post-scaffold regression check. Empirical delivery testing only — static structure/trace grading is `validation-constitution`, not this skill.
---

# Testing Governance Injection — Empirical Probe of the Surface Set

## Overview

Static validation grades what the surfaces SAY; this skill tests what the harness DOES: which
rule documents actually land in an agent's context when it touches governed paths, and whether
they change what the agent will do. The two answers can diverge — the originating dogfood run
(kinako, 2026-07-19) probed a validator-PASSed surface set and caught two delivery defects
static grading cannot see, both now guarded upstream — **this probe is the check that catches
the next unknown-unknown.**

Everything this skill observes is **versioned harness behavior, never doctrine**: stamp every
finding with the Claude Code version and date, and never promote an observation into a skill or
template as a timeless fact.

## When NOT to Use

- **As a substitute for `validation-constitution`** — trace closure, structure, and placeholder
  grading are the validator's; this skill only tests delivery.
- **To fix what it finds** — rules files and the region are setup-owned; findings route to an
  amend run (or land as `BACKLOG.md` empirical items), never hand-fixed around the ownership
  boundary.
- **Mid-authoring** — probe an accepted set; a draft's findings are noise the authoring loop will
  churn anyway.

## The matrix and the findings

Per probe: expected (from `paths`) vs observed. **Both directions are findings** — a missing rule
(under-injection: the scoped code can be written blind) and an unexpected rule (over-injection:
the always-on budget leaks). Known observed behavior the matrix should verify rather than
re-discover: injection fires on **Read of a matching file, not on Write** (kinako 2026-07-19) —
the region's standing new-file read line is the authored mitigation; confirm the line is present
and that the read-back it instructs actually injects.

## Cleanup (unconditional)

Delete every stub the run created — **even when probes fail or the run is interrupted** — then
verify with `git status` that the tree carries only intended changes. A leftover probe stub is a
defect in the run, not a finding about the project.

## Report

The trigger matrix, each finding (expected vs observed, severity, the probe that caught it), the
harness version probed (Claude Code version + date), and the token cost. Routing: `paths` or
region changes → a `/mochiko:setup` amend run; harness-behavior surprises → `BACKLOG.md` as
empirical items; behavioral-probe failures → both (the principle's enforcement is not landing).

## Common Mistakes

| Mistake | Problem | Fix |
|---------|---------|-----|
| Probing from the lead's context | The lead already read the rules files — guaranteed false positive | Probes are always fresh subagents |
| Read-only probing | Misses the Write gap that motivated this skill | New-file probes Write first, report, then Read back |
| Briefing a probe with expectations | An agent told what should inject will report it | Introspection briefs ask for raw context contents only |
| Skipping the negative control | Over-injection goes unmeasured; "everything fired" looks like success | Always one surface expected to inject nothing |
| Leaving stubs behind | Pollutes the tree and the next session's context | Unconditional cleanup + `git status` verification |
| Treating findings as doctrine | Harness delivery behavior changes across versions | Version + date stamp on every finding |
| Fixing rules files inline | They are setup-owned; regeneration silently reverts hand-edits | Route through the amend run |
