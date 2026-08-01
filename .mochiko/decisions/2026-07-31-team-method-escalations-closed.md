# Team-method escalations closed — loop-discipline qualifier · agent-dispatch v7 · cold checkout (v0.39.0)

**Status:** ruled (execution record; underlying rulings: team-method D1/D3 + two user rulings this
date) · **Date:** 2026-07-31

## Context

The Layer-2 mesh rewrite ADR (`2026-07-30-layer-2-mesh-rewrite-executed.md`) raised three
escalations it did not rule; BACKLOG's "Team-method build items" carried them, plus the
cold-checkout gate (kinako rule 7) awaiting a placement ruling. Wave executed 2026-07-31 at
plugin **v0.39.0**. Producer: `command-architect`. Independent audits (author ≠ grader):
`validation-command-shape` on `implement.md`; a `validator` consumer-checklist grade on the two
shared primitives + strip notes. Round 1 FAIL on both (implement's not-done gap · this record not
yet on disk) → fix round → **both re-audits PASS** (implement floor 10/10 + ceiling 5/5; all four
primitive artifacts clear, the scope objection retracted on ground truth). One round-1 finding is
dispositioned here rather than fixed:
the primitive auditor read `implement.md` as outside the wave's scope because its brief enumerated
only its own half of the wave — the edit was user-ruled (ruling 3 below) and separately
`validation-command-shape`-audited, so no revert is owed.

## Decision

1. **`loop-discipline` requirement 2 qualified** — the verdict-ownership sentence gains a narrow
   structural exception for a command shape's devolved clean branch, bound by closed reference to
   `command-shape.md` Layer 2 *Clearing under the mesh* (no condition restated); the workflow's
   done-condition verdict stays the lead's. Closes escalation 1; ruling ground: team-method D3
   (DECISIONS.md 2026-07-25). All-consumer pass over the 22 referencing files: one contradiction
   found — `implement.md`, the one the amendment exists to resolve; zero follow-on edits.
2. **User ruling — ninth briefing field.** The peer-edge + hand-off-hold obligation lands as
   `agent-dispatch.md` field 9 (team-form only, omitted for a one-shot, carried by reference to
   shape Layer 2, never restated); field 6 re-routed for the mesh — peer-routed feedback is
   pointed at and the dispatch opens the round; the v3 verbatim-paste survives as the
   no-peer-edge routing. Briefing v7. Closes escalation 2. Shape-sentence-only rejected: the
   table is what callers walk; the v4 gap existed because the obligation lived only in the shape.
3. **User ruling — cold checkout lands in implement's final validation.** The final validation
   builds and runs the quality gates from a fresh clone, never only the warm working tree; the
   clone's results are part of G5's evidence, and a warm-only final validation is a named
   not-done state. Provenance: kinako rule 7 — warm-machine verification passed a
   gitignored-`build/` bug six cycles. Audit-charter placement rejected for now (audit is
   unscoped, ROADMAP Later); the step migrates if audit takes feature-close.
4. **Discharged by verification, no edit owed:** (a) the "status is input, never the gate" dedup
   rider — the sentence lives only in the shape home and the grader's key list; the six commands'
   restatements died in the v0.35.0 goal-shape rebuild; (b) the verdict-ownership triplication
   rider — `tasks` retired at the v0.32.0 merge, `plan`/`implement` rebuilt and audited at
   v0.35.0; the surviving per-command "No devolved branch" / "every verdict is yours" lines are
   the mesh wave's sanctioned negative-case declarations; (c) the mesh ADR's third escalation
   (`validation-command-shape` check-1 setup carve-out) — already discharged in the grader
   (SKILL.md check 1 carries the setup-scaffolder carve-out).

## Rationale

Doctrine and shape must agree or every audit of the devolved branch grades against a
contradiction. The qualifier is doubly closed (antecedent requires a shape to define the branch;
conditions bound by reference), so it is vacuous everywhere no shape devolves — confirmed by the
auditor's independent five-consumer spot-check. Cold checkout is the cheapest deterministic catch
for the works-warm-only bug class, placed once per run at final validation rather than per cycle.

## Alternatives considered

- **Shape-sentence-only** for the peer-edge obligation — rejected by user ruling (see 2).
- **Audit-charter placement** for cold checkout — rejected by user ruling (see 3).
- **A general devolution license** in loop-discipline — never on the table; the exception binds
  to the shape's stated conditions only.

## Observed, ruled upstream

The `agent-dispatch.md` trailer now reads v7 (2026-07-31) above v6 (2026-08-01) — the
pre-existing v6 stamp post-dates this wave's ruled date. Same anomaly exists on
`command-shape.md` v6. Recorded so the next auditor does not read it as a fresh defect.

## Superseded-mechanism note (2026-08-01)

Decision 3's **fresh clone** mechanism is superseded by the git-semantics filtered snapshot
(`git ls-files -co --exclude-standard` → copy), ruled at
`.mochiko/brainstorms/validator-worktree-isolation/record.md` D3(i)/D4 and built at v0.42.0.
Ground: executed as written against an uncommitted implementation, a clone of HEAD gates a tree
that does not contain the work under validation (that record's F22–F24).

**The ruling's intent stands** — the works-warm-only catch and the final validation's results as
part of G5's evidence, with a warm-only final validation still a named not-done state. Only the
mechanism changed: the clone gated what the repository contains, the snapshot gates what the disk
contains.
