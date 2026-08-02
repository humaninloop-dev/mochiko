---
description: Create a feature specification via an independent author→critic team loop — a standing requirements-analyst seat authors spec.md across bounded rounds, a cold devils-advocate seat stress-tests it from the file, the user accepts at a named gate; sparse input is enriched first; default-FAIL, bounded, kernel-free. Requires agent teams (CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS); refuses without them.
disable-model-invocation: true
---

# Specify — Feature Specification

**Goal:** turn a feature description into an accepted `spec.md` — prioritized user stories,
FR-XXX requirements, measurable SC-XXX criteria, edge cases — authored and independently
stress-tested before the user accepts it. `$ARGUMENTS` = the feature description; empty or sparse
is resolved at G1.

**You are the lead**: you compose the run and own its counters, every verdict, every escalation,
every human gate, and the user-facing conversation — agents produce and review, you adjudicate.
Every dispatch carries its own brief in the spawn or send prompt — the seat's role and skill
(named as a hint, the agent decides fit), the exact inputs to Read, where the output lands
(write vs return), the bar it must clear, its peer edges and holds, and the independence
reminder that matches the seat (author: never grade your own output; grader: read the artifact
itself, default FAIL, quote evidence) — the seat owns none of this context and gets all of it
from you; on a retry, a peer-routed gap list is pointed at and the round opened, a relayed one
pasted verbatim. This file is self-contained: specify's
whole contract lives here. **First-spawn probe:** the producer — always the first seat filled.

## Goal

`.mochiko/specs/<feature>/spec.md` exists, conforming to its template with no placeholder tokens,
with the round reports for the grading that actually ran, and no blocking gap left open against it;
G3 acceptance has cleared; and the KM landing ran.

**Not done:** default **FAIL** · a blocking gap open · a departure with no trail line · out of
rounds · G3 unaccepted.

## Seats & checks

| seat | agent × skill(s) | produces / grades | spawn | peer edges |
|---|---|---|---|---|
| producer | `requirements-analyst` × `authoring-requirements`, `authoring-user-stories` | authors `spec.md` + `analyst-report.md` from their templates; never grades | one **named standing seat** across rounds; **probe seat** | hands each round's `spec.md` straight to the critic; round > 1 reaches it with the gap list already in hand |
| critic | `devils-advocate` × `review-specifications` | grades `spec.md` **from the file, never the producer's report** → `advocate-report.md`: severity-classified findings, product-framed clarifying questions, and a recommended verdict (`ready` / `needs-revision` / `critical-gaps`); never authors | cold at first critique, standing after — round > 1 re-Reads the revised spec | peer-edged with the producer |

**Validation model:** the loop's bounded in-loop critique, every round, **unsized by design**. The
critic's output is **lead-adjudicated input** (the `review-*` family boundary) and every verdict is
yours. No seat ever grades its own output.

**Team transport:** check `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` before anything else — unset →
stop and tell the user how to enable it (settings/env; Claude Code ≥ v2.1.178); the first spawn is
the authoritative probe, and there is no teamless fallback. A seat is spawned with **`name:`**
("spawn a teammate named `producer`") — a nameless spawn is a one-shot subagent, the forbidden
transport. Every later round is a `SendMessage` to that same named seat. Verify from the roster:
`~/.claude/teams/<team>/config.json` (`<team>` = `session-` + first eight chars of the session ID)
holds a `members` array — the seat's `name` not in it ⇒ kill and respawn explicitly requesting an
agent team; failing again stops the run. Teammates don't load `skills:` frontmatter — every spawn
prompt names the skill and role itself. Tell the user up front they can watch or message any
teammate; announce each seat in one line when filled; never narrate or reply to teammate
housekeeping. A peer-routed gap list is a **hand-off, not a start signal** — the producer revises
only when you open the next round, and your brief carries that hold.

**Seat lifecycle:** at each gate pause, count each standing seat's completed rounds and recycle at
~≥3 (counted, never observed — no occupancy self-reports; the user may order a recycle at any
gate). A respawn is a reset: the successor is briefed from the on-disk artifact set alone and takes
a versioned name (`producer-2`) — never reuse a dead seat's bare name. A seat whose remaining work
is zero shuts down; no ritual sends — fold confirmations into the next real dispatch.

## Constraints

- **G1 entry** — evidence: `$ARGUMENTS`, and `CLAUDE.md`'s governance region
  (`<!-- mochiko:governance:begin -->`) · rules: the user · decides: the resolved feature
  description and whether the run is governed. Empty `$ARGUMENTS` → ask the user to re-enter it, or
  proceed and enrich from scratch. A missing governance
  region is **surfaced** — offer `/mochiko:setup` first, or proceed ungoverned for this spec —
  never auto-resolved. Then triage: *rich* (Who / Problem / Value clear from the description or the
  governance region's domain context) → the loop; *sparse* → enrichment.
- **Run-start weight card** — evidence: your stated read of the four rigor factors against this
  feature description — **reversibility** (how expensive is rework if this is wrong) · **blast
  radius** (how much downstream work reads the spec as authoritative) · **precedent**
  (first-of-kind, or mirroring an audit-cleared pattern) · **input confidence** (scored on the
  artifact under review; a user ruling discounts ambiguity risk only, and one that introduces new
  surface raises consistency risk) — plus the process you compose from it — the stated default
  below, or your departures from it · rules: the user · decides: the run's composed process. Rigor
  scales with cost-of-being-wrong, never task size; diff size is at most a hint.
- **G2 clarification** — evidence: a producer clarification it cannot resolve, or a finding you
  judge only the user can settle · rules: the user · decides: the answer fed forward into the next
  dispatch. You route each finding by judgment: answerable by investigation → a native `Explore`
  pass, never the user; a genuine judgment call → ruled here; bigger than the run was framed →
  escalate. An in-loop gate, never the done-condition.
- **G3 acceptance** — evidence: your clearing verdict on `spec.md` + `advocate-report.md` (story
  and FR counts, outstanding clarifications) · rules: the user · decides: **accept** (done) /
  **amend** (the requested changes become the gap list; still bounded, and clear a verdict again) /
  **reject** (abort; the draft stays under `.mochiko/specs/<feature>/`).
- **Escalation** — evidence: a cap trip, a gap set unchanged round-over-round, the kill-switch, or
  a `critical-gaps` verdict · rules: the user, on the last gap list plus the stop reason · decides:
  give-guidance-and-retry / accept-with-noted-gaps / abort — **the run stays FAIL unless the user
  explicitly accepts**.
- **Floor gates:** the run-start weight card · **G2**'s user ruling · **G3** · **Escalation**
  · and **G1** on its two user-ruled limbs — the empty-`$ARGUMENTS` ask and the governance-absence
  surfacing, never auto-resolved — the user's whatever you compose, never departable. G1's
  rich/sparse triage is yours, and departable. Batch rulings into the fewest checkpoints that
  respect these gates. No lead-penned surface takes a standing cold grade here: the uncertainty
  carrier is producer-authored, and enrichment lands in the producer's brief rather than in an
  artifact of yours — were you to pen a deliverable surface, it would take one cold-seat grade
  non-discretionarily, waivable only by recorded user waiver at the weight card.
- **Bounds:** cap **3** rounds, you count them; no-progress exit when the gap set is unchanged
  round-over-round; kill-switch — stop and escalate if `.mochiko/specs/<feature>/SPECIFY_STOP`
  exists, checked before each seat send; out of rounds = escalate, never done. Any bound this run
  declares — including a declared cost range — has you as its named counter, **rises only at a
  user checkpoint**, and is re-declared only on the record; busting a bound escalates, never
  silently continues.
- **Enrichment** *(sparse input only)* — **yours, inline, and once.** Invoke
  `Skill(mochiko:analysis-iterative)` to surface Who / Problem / Value and the feature shape
  interactively, and carry the enriched description forward in-session into the producer's brief —
  pre-loop only; the loop's own critique drives later rounds, so never re-enrich. **It is
  lead-inline because enrich-or-not is loop-entry triage: it conditions the input, and it neither
  authors nor grades** — handing it to the producer would have that seat author from input it shaped
  itself (`KEPT:` survivor).
- **Loop invariants:** hold every revision targeted — fix the flagged gaps, don't regress passing
  sections. **No devolved branch** — the critique is a judgment grade, never all-deterministic-CLI,
  so no gate is skipped and every verdict is yours.
- **Report hygiene:** round reports are cleaned by default (their outcome stamps live in the
  deliverable); the user may ask to retain them. **Never offer to delete `spec.md`** — it is the
  deliverable.
- **Ground rules:** kernel-free — no brain code, no capability catalogs, no DAG-mediated
  orchestration. Suggest commits; never run git mutations, never push. No internal machinery
  vocabulary in user-facing prose — the conversation is yours and the user's, in the mochiko
  register (`templates/output-style.md`). User acceptance is plain blocking text, never a timed
  prompt. The deliverable is written as the work progresses, never reconstructed at the end.

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
- **Run-start declaration:** one line on `spec.md`'s `Status` header — where Recovery already notes
  the resume stage — for a default run; a run that departs from the stated default, or declares
  non-default bounds, writes a departure record at
  `.mochiko/specs/<feature>/specify-contract.md` beside the reports instead — the
  done-condition and bounds as (re-)declared, departures taken, and the counter state Recovery
  reads on resume. Counted unit: the
  **round**, the unit the Bounds already count.
- **Departure trail:** one line per departure from the stated default, appended under that same
  `Status`-header declaration as it is taken and carried into G3's evidence — never your context
  alone; the trail names the grading that actually ran. Departure is by record, never by silence.
- **KM landing:** `.mochiko/memory/knowledge-management.md` exists → run its landing ritual +
  invariants under fix-on-sight, and mint new domain terms into `GLOSSARY.md`. No copy → skip.

## Recovery

Note the resume stage on `spec.md`'s `Status` header line, with the run's counter state — rounds
consumed · bounds declared · departures taken. Sessions and teams do not survive `/resume`, and a
shared account limit can throttle the team and the main session together — escalation then has
nowhere to go but pause. Resume from workspace evidence, never a context `phase` field, respawning
only what the stage needs — a respawned producer re-reads `spec.md` + the gap list, and a respawn
is cold by design.

| Evidence | Resume at |
|----------|-----------|
| no `.mochiko/specs/<feature>/` | G1 triage |
| `spec.md` still the bare seeded template | loop (produce, round 1) |
| `spec.md` authored, no `advocate-report.md` this round | loop (critique) |
| `advocate-report.md` `needs-revision`/`critical-gaps`, within the cap | loop (produce) |
| `advocate-report.md` `ready`, not yet accepted | G3 |
| accepted | finalize — report the deliverable + the two reports, the round count, story / FR counts, a suggested commit (`docs: specify <feature>`), next step `/mochiko:plan` |
| `SPECIFY_STOP` present | escalate |
