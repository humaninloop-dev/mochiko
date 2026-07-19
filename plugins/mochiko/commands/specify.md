---
description: Create a feature specification via an independent author→critic team loop — a standing requirements-analyst seat authors spec.md across bounded rounds, a cold devils-advocate seat stress-tests it from the file, the user accepts at a named gate; sparse input is enriched first; default-FAIL, bounded, kernel-free. Requires agent teams (CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS); refuses without them.
disable-model-invocation: true
---

# Specify — Feature Specification

**Goal:** turn a feature description into an accepted `spec.md` — prioritized user stories,
FR-XXX requirements, measurable SC-XXX criteria, edge cases — authored and independently
stress-tested before the user accepts it. `$ARGUMENTS` = the feature description; empty or
sparse is handled by triage below.

**You are the lead**, and this is a **team-form command in the mochiko command shape**: Read
`${CLAUDE_PLUGIN_ROOT}/templates/command-shape.md` (both layers) before anything else — the
shape's rules bind here and are not restated; this file carries only specify's parameters.
You own the loop (round counter, verdict, escalation) and every human gate. This is a
`mochiko:loop-discipline` sound loop; the Contract section below is its authoring-time fill.

## Team-form parameters (shape Layer 2)

Hard-require `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` per the shape. The **authoritative
first-spawn probe** is the producer — always the first seat filled. Transport mechanics +
the addressability check: `templates/agent-dispatch.md` (Seat transport). The no-fallback
bet is the same `Contested` dogfood-pilot ruling as the other team-form commands.

## Session constraints

- Workspace: derive a kebab-case `<feature>` slug from the (clear) description,
  `mkdir -p .mochiko/specs/<feature>`, seed `spec.md` from `templates/spec-template.md`.
- Kill-switch: stop and escalate if `.mochiko/specs/<feature>/SPECIFY_STOP` exists — check
  before each seat send.
- **Deliverable & IDs:** `spec.md`, producer-authored per its template (P1/P2/P3 stories,
  FR-XXX, SC-XXX); its uncertainty carrier is the template's **Assumptions** and **Open
  Questions** sections (the shape's producer-authored branch), not confidence marks. The
  critic's gap IDs live in `advocate-report.md`.

## The seats

- **producer** — `mochiko:requirements-analyst`, one **named standing seat** across rounds.
  Brief it to author `spec.md` + `analyst-report.md` (from
  `templates/analyst-report-template.md`) via `mochiko:authoring-requirements` +
  `mochiko:authoring-user-stories`: the feature description (enriched where applicable),
  the governance obligated-read line, the template to fill per those skills — no
  placeholder tokens. Round > 1 is a message to the same seat carrying the critic's gap
  list verbatim (fix the flagged gaps; don't regress passing sections). If it messages
  clarifications it cannot resolve, ask the user (G2) and feed the answers forward — an
  in-loop human gate, never the done-condition. It never grades.
- **critic** — `mochiko:devils-advocate`, spawned **cold at first critique**, never in
  contact with the producer. Brief it to run `mochiko:review-specifications` against
  `spec.md` — it Reads the spec file itself, never the producer's report — writing
  `advocate-report.md` (from `templates/advocate-report-template.md`): severity-bucketed
  gaps, product-framed clarifying questions, genuine strengths, and a recommended status
  (`ready` / `needs-revision` / `critical-gaps`). Round > 1 is a message to the same seat:
  re-Read the revised spec. Its output is **lead-adjudicated input** (the `review-*`
  family boundary); there is no sized end-stage review here — the bounded in-loop critique
  is this workflow's independent validation (declared in the Contract).

## The flow

**Triage** *(gate G1)* — capture `$ARGUMENTS`. Empty (the known `@`-reference drop) →
recover via G1: ask the user to re-enter, or proceed and enrich from scratch.
**Governance prerequisite:** check `CLAUDE.md` for the mochiko governance region
(`<!-- mochiko:governance:begin -->`). Present → governance reaches the producer natively
at spawn; add to its brief the one-line **obligated read** naming the
`.claude/rules/mochiko/` files relevant to what it authors (`paths`-scoped rules don't
fire for from-scratch authoring). Missing → surface it (offer `/mochiko:setup` first, or
proceed ungoverned for this spec) — never auto-resolve. Then triage: *rich* (Who /
Problem / Value clear from the description and/or the governance region's domain context)
→ the loop; *sparse* → enrichment.

**Enrichment** *(sparse input only — you, inline)* — invoke
`Skill(mochiko:analysis-iterative)` to surface Who / Problem / Value and the feature shape
interactively; carry the enriched description forward in-session. Once, pre-loop — the
loop's own critique drives later rounds; never re-enrich. Lead-inline because
enrich-or-not is loop-entry triage: it conditions the input, it neither authors nor grades.

**Spec loop** *(you own the counter; the spec is FAIL until proven otherwise)* — initialize
`round = 1` → **produce** → **critique** → **verdict (you):** Read `spec.md` +
`advocate-report.md` directly. Critic `ready` **and** you find no unresolved blocking gap
→ acceptance. Otherwise classify each gap and route it per `loop-discipline`'s gap
routing — bindings: knowledge → a native `Explore` pass · preference → G2 · scope →
escalate — then apply the bounds: cap **3** rounds · no-progress exit when the gap set is
unchanged round-over-round · kill-switch. Any trip, or `critical-gaps` → **escalate**
(present the last gap list + stop reason; give-guidance-and-retry /
accept-with-noted-gaps / abort — the run stays FAIL unless the user explicitly accepts).
Else `round += 1`, loop to produce.

**Acceptance** *(gate G3 — reachable only on your clearing verdict)* — present the
validated spec (story / FR counts, outstanding clarifications): **accept** (the
done-condition is now satisfied) / **amend** (re-enter the loop with the requested changes
as the gap list — still bounded; it must clear a verdict again) / **reject** (abort; the
draft stays under `.mochiko/specs/<feature>/`).

**Finalize** — report the artifacts (`spec.md` deliverable + the two reports), the round
count, story / FR counts, a suggested commit (`docs: specify <feature>`), and the next
step (`/mochiko:plan`). Offer a retain/clean choice for the round reports; never offer to
delete `spec.md` — it is the deliverable.

## Contract (authoring-time fill — governed by `mochiko:loop-discipline`)

- **Done-condition:** default **FAIL**; clears only when **(1)** the critic recommends
  `ready` on `spec.md` grounded in the file, **(2)** you Read `spec.md` + the advocate
  report and confirm no blocking gap remains (the critic's status is input, never the
  gate), and **(3)** G3 acceptance has cleared. Out of rounds = escalate, never done.
- **Producer ↔ validator:** `requirements-analyst` (authoring-requirements,
  authoring-user-stories) authors, never grades; `devils-advocate`
  (review-specifications) reviews from the file, never authors — disjoint agents,
  disjoint skills, structurally separated (critic cold-spawned, gap lists lead-routed, no
  producer↔critic contact). **Validation model:** the bounded in-loop critique — every
  round, unsized by design; no sized end-stage review (the shape's in-loop-critique
  branch).
- **Bounds:** ≤3 rounds (you count) · no-progress exit · kill-switch `SPECIFY_STOP` · a
  G3 amend re-enters the same bounded loop.
- **Human gates:** G1 input recovery + governance surface · G2 clarifications +
  preference-gap decisions · G3 spec acceptance · escalation on any guard trip.

## Recovery

Pause posture (per the shape): note the resume stage on `spec.md`'s `Status` header line.
Resume from workspace evidence, respawning what the stage needs — a respawned producer
re-reads `spec.md` + the gap list; a critic respawn is cold by design:

| Evidence | Resume at |
|----------|-----------|
| no `.mochiko/specs/<feature>/` | triage |
| `spec.md` still the bare seeded template | loop (produce, round 1) |
| `spec.md` authored, no `advocate-report.md` this round | loop (critique) |
| `advocate-report.md` `needs-revision`/`critical-gaps`, within the cap | loop (produce) |
| `advocate-report.md` `ready`, not yet accepted | G3 |
| accepted | finalize |
| `SPECIFY_STOP` present | escalate |

---

**What you own (not the seats):** input triage and the enrichment call; the loop, the gap
routing, and the verdict against the default-FAIL done-condition; the human gates; and
never letting producer and critic collapse into one seat. Full rules:
`mochiko:loop-discipline`.
