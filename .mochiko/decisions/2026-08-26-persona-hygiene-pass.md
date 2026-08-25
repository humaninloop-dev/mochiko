# ADR — Persona hygiene pass: requirements-analyst skills section, devils-advocate description

- **Date:** 2026-08-26
- **Status:** ruled (user, in-session) + built
- **Ships:** v0.86.0
- **Primitives:** `plugins/mochiko/agents/requirements-analyst.md` ·
  `plugins/mochiko/agents/devils-advocate.md`
- **Strip entries:** `.mochiko/strips/requirements-analyst.md` [v0.86.0] ·
  `.mochiko/strips/devils-advocate.md` [v0.86.0]

## Context

After the v0.84.0/v0.85.0 validator re-index, the user asked whether other personas carry
similar defects (restated skill internals, stale wording, scope-misread shape,
maintainer-facing prose). A full read of the other nine personas found two actionable
findings and one declined:

1. **`requirements-analyst`** — its `## Skills Available` section predated the house pattern:
   bullets restated skill internals (FR-XXX / RFC 2119 / SC-XXX; P1-P3 / Given-When-Then)
   with no single-source framing, while the same file's Quality Standards section already
   named the same templates with a correct "consult them there rather than a copy" pointer.
   Same content twice, once at the wrong altitude.
2. **`devils-advocate`** — its frontmatter `description:` (the routing/staffing surface) said
   "stress-tests specifications" only, while the actual remit spans five review targets:
   specs, plan packages, brainstorm records, governance intent, and the runtime blind
   gap-finding pass against built systems. The description lagged the remit growth (the
   `review-brainstorm`, `review-governance-intent`, and `testing-gap-finding` mounts arrived
   after it was written).
3. **`tech-lead` Three-Part Rule restatement — declined.** The inline
   enforcement/testability/rationale triad borders on restating `authoring-constitution`
   canon, but it is three lines, drives the persona's own Judgment section, and reads as
   carried judgment rather than copied procedure. User ruled: skip.

## Decision

**requirements-analyst:** the Skills Available section is rewritten to the ruled precedent
form (single-source framing + one routing line per mount — the same form the
devils-advocate [v0.25.0] Tier-1 strip established and every other persona uses). Skill
internals live only in the mounted skills. Body-only edit; the `description:` (303 chars,
budget 379) and everything else untouched.

**devils-advocate:** the `description:` value is rewritten to cover the full remit —
"stress-tests finished artifacts — specifications, plan packages, brainstorm records,
governance intent — and probes built systems in the blind gap-finding pass," with the
what-if/severity/recommended-verdict framing kept. The output-shape tail is scoped
"severity-ranked **document** findings" (audit fix round 1): the runtime gap-finding pass
splits findings by kind, never by severity, per `testing-gap-finding` — an unscoped tail
would contradict that skill. New parsed value 384 chars against the 395 budget (a current
measurement, not a re-derivation base — the budget stays 395; ledger row annotated). This
supersedes part of the [v0.63.0] entry's `Kept deliberately` description prose — by this
recorded ruling, per the protected-content rule. Body untouched.

## Consumers assessed

- Router `skills/mochiko/SKILL.md`: the agents-table row describes devils-advocate as
  cross-workflow (four duties in prose, the gap-finding duty carried by its `skills:` list
  and detailed at the implement-cluster `testing-gap-finding` row) — the description now
  agrees with the router instead of lagging it; the requirements-analyst row is
  body-agnostic. No router edit.
- Commands: `plugins/mochiko/commands/implement.md:115` names `devils-advocate` (the
  gap-finding seat, "a fresh `devils-advocate`, dispatched blind per run") — it names the
  agent, never quotes the description; the staffing contract is untouched and the widened
  description now covers exactly that dispatch (audit fix round 1 — an earlier draft of this
  ADR claimed no command named either agent, which was false for devils-advocate). No
  command names `requirements-analyst`.
- Staffing/routing: agent names unchanged; the devils-advocate description's core framing
  (adversarial, what-if, severity-ranked, recommended verdict) survives, so the v0.63.0
  benchmark's routing result is not invalidated — widened, not re-aimed.
- Budgets ledger `.mochiko/memory/primitive-cost-budgets.md`: devils-advocate row annotated
  with the v0.86.0 re-measurement (384; budget unchanged 395).
