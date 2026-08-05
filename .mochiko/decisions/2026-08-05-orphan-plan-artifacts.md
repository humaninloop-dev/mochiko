# Orphan plan artifacts — `nfrs.md` minimally wired, `quickstart.md` null read deliberate

**Status:** ruled (user, 2026-08-05)
**Date:** 2026-08-05

## Context

The ponytail-concepts-integration seam scan (S2, record
`.mochiko/brainstorms/ponytail-concepts-integration/record.md`) found two plan-produced
artifacts named nowhere on the implement side: `nfrs.md` and `quickstart.md` were produced
per `commands/plan.md` but absent from `commands/implement.md`'s Design inputs, entry gate,
and every implement-side skill. Logged as a BACKLOG defect (Defects & empirical checks,
2026-08-05) independent of the ponytail session.

Fact-finding for this ruling (Explore pass, 2026-08-05):

- `nfrs.md` (authored by `authoring-technical-requirements`) carries numeric, measurable
  quality targets. It has an active plan-side consumer chain — `review-feasibility` grades
  NFR↔constraint/topology contradictions, `review-plan-artifacts` grades measurability —
  but consumption stopped at plan acceptance: no `**TEST:**` gate grammar, quality gate,
  or verification step re-checks a built system against its NFR targets, and NFR-XXX never
  threads into SC-XXX/Feature-Done.
- `quickstart.md` (authored by `patterns-api-contracts`, conditional) is "the human-facing
  integration guide over the finished contract" — for external API integrators, not a
  build input. `testing-end-user/SKILL.md:150` already sources run/build information from
  `tasks.md`'s `## Quality Gates` and `plan.md`'s build configuration; zero content
  overlap with quickstart.

## Decision

1. **`nfrs.md` — minimal wire.** `commands/implement.md` Design inputs gains `nfrs.md`
   ("the numeric quality targets the built code must respect") so builders and the
   verification seat see the targets. Pure addition, landed in the v0.53.0 wave.
2. **Runtime NFR verification — deferred to `audit` scoping, recorded as a rider.** No
   TEST-grammar NFR assert pattern exists; building one is a new gate class (n=1, collides
   the Cluster-2 meta-rule). The BACKLOG `audit` workflow scoping item carries the rider:
   NFR runtime verification (p95/availability targets vs the built system) joins
   feature-close verification scope when audit is scoped.
3. **`quickstart.md` — null implement-side read ruled deliberate.** It is an
   external-integrator deliverable over the finished contract, not a build input;
   implement's run/build needs are already sourced elsewhere with zero overlap. No wiring;
   this record is the disposition.

## Rationale

The defect asked "wire in, or rule the null read deliberate" per artifact. `nfrs.md`'s
targets are stated as binding ("no deferrals") yet were invisible to the seats building
against them — the one-line wire closes that at zero machinery cost, while the genuinely
missing half (runtime re-check) belongs to feature-close verification, which already has a
named future owner (`audit`). `quickstart.md`'s audience is outside the pipeline; wiring it
in would invent a consumer no need supports.

## Alternatives considered

- **Full wire for `nfrs.md`** (TEST-grammar NFR asserts + verification-seat NFR checks):
  rejected — new gate class on n=1, meta-rule collision, and feature-close is the natural
  altitude for runtime targets.
- **Rule `nfrs.md` null read deliberate with only the audit rider:** rejected — builders
  seeing the numeric targets is cheap and materially better than building blind to them.
- **Wire `quickstart.md` into verification:** rejected — no content overlap with
  verification's needs; its consumer is human/external.
