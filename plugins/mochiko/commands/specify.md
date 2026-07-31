---
description: Create a feature specification via an independent author→critic team loop — a standing requirements-analyst seat authors spec.md across bounded rounds, a cold devils-advocate seat stress-tests it from the file, the user accepts at a named gate; sparse input is enriched first; default-FAIL, bounded, kernel-free. Requires agent teams (CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS); refuses without them.
disable-model-invocation: true
---

# Specify — Feature Specification

**Goal:** turn a feature description into an accepted `spec.md` — prioritized user stories,
FR-XXX requirements, measurable SC-XXX criteria, edge cases — authored and independently
stress-tested before the user accepts it. `$ARGUMENTS` = the feature description; empty or sparse
is resolved at G1.

**You are the lead** of a team-form command in the mochiko command shape: Read
`${CLAUDE_PLUGIN_ROOT}/templates/command-shape.md` (both layers) and `mochiko:loop-discipline`
before anything else; brief every dispatch per `templates/agent-dispatch.md`. This file carries
only specify's parameters. **First-spawn probe:** the producer — always the first seat filled.

## Goal

`.mochiko/specs/<feature>/spec.md` exists, conforming to its template with no placeholder tokens,
with both round reports written; the critic recommends `ready` grounded in the file; you Read
`spec.md` + `advocate-report.md` and confirm no blocking gap remains; G3 acceptance has cleared;
and the KM landing ran.

**Not done:** default **FAIL** · a critic status short of `ready` · a blocking gap open · the
critic's status taken as the gate without your read · out of rounds · G3 unaccepted. A G3 **amend**
re-enters the loop and must clear a verdict again.

## Seats & checks

| seat | agent × skill(s) | produces / grades | spawn | peer edges |
|---|---|---|---|---|
| producer | `requirements-analyst` × `authoring-requirements`, `authoring-user-stories` | authors `spec.md` + `analyst-report.md` from their templates; never grades | one **named standing seat** across rounds; **probe seat** | hands each round's `spec.md` straight to the critic; round > 1 reaches it with the gap list already in hand |
| critic | `devils-advocate` × `review-specifications` | grades `spec.md` **from the file, never the producer's report** → `advocate-report.md`: severity-classified findings, product-framed clarifying questions, and a recommended verdict (`ready` / `needs-revision` / `critical-gaps`); never authors | cold at first critique, standing after — round > 1 re-Reads the revised spec | peer-edged with the producer |

**Validation model:** the loop's bounded in-loop critique, every round, **unsized by design**. The
critic's output is **lead-adjudicated input** (the `review-*` family boundary) and every verdict is
yours.

## Constraints

- **G1 entry** — evidence: `$ARGUMENTS`, and `CLAUDE.md`'s governance region
  (`<!-- mochiko:governance:begin -->`) · rules: the user · decides: the resolved feature
  description and whether the run is governed. Empty `$ARGUMENTS` → ask the user to re-enter it, or
  proceed and enrich from scratch. A missing governance
  region is **surfaced** — offer `/mochiko:setup` first, or proceed ungoverned for this spec —
  never auto-resolved. Then triage: *rich* (Who / Problem / Value clear from the description or the
  governance region's domain context) → the loop; *sparse* → enrichment.
- **G2 clarification** — evidence: a producer clarification it cannot resolve, or a gap you
  classify as preference · rules: the user · decides: the answer fed forward into the next
  dispatch. **A preference gap is ruled here**; a knowledge gap routes to a native `Explore` pass,
  never to the user; a scope gap escalates. An in-loop gate, never the done-condition.
- **G3 acceptance** — evidence: your clearing verdict on `spec.md` + `advocate-report.md` (story
  and FR counts, outstanding clarifications) · rules: the user · decides: **accept** (done) /
  **amend** (the requested changes become the gap list; still bounded) / **reject** (abort; the
  draft stays under `.mochiko/specs/<feature>/`).
- **Escalation** — evidence: a cap trip, a gap set unchanged round-over-round, the kill-switch, or
  a `critical-gaps` verdict · rules: the user, on the last gap list plus the stop reason · decides:
  give-guidance-and-retry / accept-with-noted-gaps / abort — **the run stays FAIL unless the user
  explicitly accepts**.
- **Bounds:** cap **3** rounds, you count them; no-progress exit when the gap set is unchanged
  round-over-round; kill-switch — stop and escalate if `.mochiko/specs/<feature>/SPECIFY_STOP`
  exists, checked before each seat send; out of rounds = escalate, never done.
- **Enrichment** *(sparse input only)* — **yours, inline, and once.** Invoke
  `Skill(mochiko:analysis-iterative)` to surface Who / Problem / Value and the feature shape
  interactively, and carry the enriched description forward in-session into the producer's brief —
  pre-loop only; the loop's own critique drives later rounds, so never re-enrich. **It is
  lead-inline because enrich-or-not is loop-entry triage: it conditions the input, and it neither
  authors nor grades** — handing it to the producer would have that seat author from input it shaped
  itself (`KEPT:` survivor — evidence: `.mochiko/strips/specify.md`).
- **Loop invariants:** hold every revision targeted — fix the flagged gaps, don't regress passing
  sections. **No devolved branch** — the critique is a judgment grade, never all-deterministic-CLI,
  so no gate is skipped and every verdict is yours.
- **Report hygiene:** round reports are cleaned by default (their outcome stamps live in the
  deliverable); the user may ask to retain them. **Never offer to delete `spec.md`** — it is the
  deliverable.

## Bindings

- **Artifacts** under `.mochiko/specs/<feature>/`, `<feature>` a kebab-case slug derived from the
  clear description (`mkdir -p` it): `spec.md`, producer-authored from `templates/spec-template.md`
  which you seed — P1/P2/P3 stories, FR-XXX requirements, SC-XXX criteria, no placeholder tokens ·
  `analyst-report.md` from `templates/analyst-report-template.md` · `advocate-report.md` from
  `templates/advocate-report-template.md`, carrying the critic's gap IDs.
- **Uncertainty carrier:** producer-authored — the spec template's **Assumptions** and **Open
  Questions** sections, not confidence marks.
- **Fact route:** `spec.md` and the reports themselves; a knowledge gap goes to a native `Explore`
  pass.
- **Governance brief:** where the region is present, governance reaches the producer natively at
  spawn — add to its brief the one-line **obligated read** naming the `.claude/rules/mochiko/` files
  relevant to what it authors (`paths`-scoped rules do not fire for from-scratch authoring).
- **KM landing:** `.mochiko/memory/knowledge-management.md` exists → run its landing ritual +
  invariants under fix-on-sight, and mint new domain terms into `GLOSSARY.md`. No copy → skip.

## Recovery

Note the resume stage on `spec.md`'s `Status` header line; resume from workspace evidence,
respawning what the stage needs — a respawned producer re-reads `spec.md` + the gap list.

| Evidence | Resume at |
|----------|-----------|
| no `.mochiko/specs/<feature>/` | G1 triage |
| `spec.md` still the bare seeded template | loop (produce, round 1) |
| `spec.md` authored, no `advocate-report.md` this round | loop (critique) |
| `advocate-report.md` `needs-revision`/`critical-gaps`, within the cap | loop (produce) |
| `advocate-report.md` `ready`, not yet accepted | G3 |
| accepted | finalize — report the deliverable + the two reports, the round count, story / FR counts, a suggested commit (`docs: specify <feature>`), next step `/mochiko:plan` |
| `SPECIFY_STOP` present | escalate |
