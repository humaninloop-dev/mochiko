---
name: testing-governance-injection
description: This skill MUST be invoked when empirically probing that an accepted governance surface set actually DELIVERS — verifying that `.claude/rules/mochiko/` files inject into agent context on the paths their `paths` frontmatter promises (and nowhere else), that the CLAUDE.md governance region reaches spawned agents, and that injected rules actually change behavior — via throwaway file stubs and disposable probe subagents, with unconditional stub cleanup. SHOULD also invoke when the work is a "governance injection probe", "injection smoke test", "verify rules injection", "probe rule delivery", building a rules-file "trigger matrix", or a post-scaffold regression check of rules delivery. Offered at a `/mochiko:setup` run's finalize; re-runnable standalone any time after (real files replace stubs as probe surfaces). Findings are observed harness behavior versioned to the run — input to an amend run, never auto-fixed. Empirical delivery testing only — static structure/trace grading is `validation-constitution`, not this skill.
---

# Testing Governance Injection — Empirical Probe of the Surface Set

## Overview

Static validation grades what the surfaces SAY; this skill tests what the harness DOES: which
rule documents actually land in an agent's context when it touches governed paths, and whether
they change what the agent will do. The two answers can diverge — the originating dogfood run
(kinako, 2026-07-19, ~220k subagent tokens, ~3 min) probed a validator-PASSed surface set and
caught two delivery defects static grading cannot see: rules injecting on **Read but not Write**
(so greenfield scaffolding never saw them), and `paths` scoped to a mechanism's home layer while
an orchestrating layer wrote governed code blind. Both are now guarded upstream
(`authoring-constitution` routing, `validation-constitution` checks) — **this probe is the check
that catches the next unknown-unknown.**

Everything this skill observes is **versioned harness behavior, never doctrine**: stamp every
finding with the Claude Code version and date, and never promote an observation into a skill or
template as a timeless fact.

## When to Use

- **After a setup run's acceptance** (the `/mochiko:setup` finalize offer) — verify the freshly
  authored rules files deliver before real work relies on them.
- **As a regression check** after real scaffolds land — real files replace stubs as probe
  surfaces; re-run cheap.
- **To settle an open empirical question** about rules delivery (e.g. comment stripping in rules
  files, fresh-session loading — `BACKLOG.md` carries the current list); design the probe case to
  answer exactly that question.

## When NOT to Use

- **As a substitute for `validation-constitution`** — trace closure, structure, and placeholder
  grading are the validator's; this skill only tests delivery.
- **To fix what it finds** — rules files and the region are setup-owned; findings route to an
  amend run (or land as `BACKLOG.md` empirical items), never hand-fixed around the ownership
  boundary.
- **Mid-authoring** — probe an accepted set; a draft's findings are noise the authoring loop will
  churn anyway.

## Build the probe plan (before spawning anything)

1. **Read every rules file's `paths` frontmatter** and build the expected-injection matrix: per
   probe surface, exactly which rules files should fire.
2. **Choose probe surfaces**: one representative path per distinct glob set, plus **one negative
   control** — a plausible project path matched by NO rules file (expected injection: none).
   Prefer real files where the project has them; otherwise create throwaway stubs, each marked
   `Throwaway probe stub — governance-injection test. Delete after.`
3. **State the cost and get the go-ahead** — default bounds: one introspection probe per distinct
   glob set + the negative control + at most two behavioral probes. Introspection probes run on a
   cheap model; behavioral probes need a capable one.

## The two probe types

Probes are **always subagents** — the lead's own context is contaminated (it read the rules files
while building the plan; a main-loop "probe" proves nothing).

- **Introspection probes** (cheap model, one per surface): touch exactly one probe surface —
  where new-file behavior matters, **Write a new file first, report, then Read it back** — and
  report raw data only: which rule documents are present in context after each step, and whether
  the CLAUDE.md governance region (with its ratified stamp) is present. The brief carries zero
  expectations — a probe told what should inject will find it.
- **Behavioral probes** (capable model, ≤2): a realistic dev task that violates a specific
  principle, with **zero mention that governance exists**. Expected: the agent refuses or
  surfaces the conflict, citing the rule. A compliant completion — or a silent "compliant"
  redesign — is a finding.

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
