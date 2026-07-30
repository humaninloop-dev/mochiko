---
description: Establish or update the project's governance on the surfaces Claude Code natively loads — an interrogation session elicits the user's declared intent (tier, type, risk, values) before anything is authored, closing on a confidence-marked synthesis that a tier-sized cold intent review stress-tests before the user ratifies it at G3 (pair / single / recorded waiver — the user rules the sizing); the ratified synthesis is then a traceable contract on a principal-architect producer teammate that authors the surface set (a marked CLAUDE.md governance region, paths-scoped rules files, skill pointers, a governance ledger), an independent validator teammate grades trace closure from the files, and the user accepts with the trace summary in hand. Greenfield | brownfield | amend; every stage user-gated, default-FAIL, bounded, kernel-free. Requires agent teams (CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS); refuses without them.
disable-model-invocation: true
---

# Setup — Governance From Interrogated Intent, On Native Surfaces

**Goal:** establish or update the project's governance so it follows the user's declared intent —
never a fixed baseline — and lives where Claude Code natively loads it. There is **no
`constitution.md`**: the deliverable is the surface set in Bindings. `$ARGUMENTS` = optional setup
request; empty is fine — detection proposes the mode.

**You are the lead** of a team-form command in the mochiko command shape: Read
`${CLAUDE_PLUGIN_ROOT}/templates/command-shape.md` (both layers) and `mochiko:loop-discipline`
first; brief every dispatch per `templates/agent-dispatch.md`; Read
`templates/sized-end-stage-review.md` at the sizing gate. This file carries only setup's
parameters. Hard-requires `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS`. **First-spawn probe:** the
producer in brownfield; otherwise the intent reviewer(s) at the synthesis review, or the producer
if it was waived.

## Goal

The surface set in Bindings exists and carries the user's ratified intent, not a baseline: the
sized intent review ran (or its waiver is recorded) with every survivor dispositioned · G3 cleared,
in every mode · `validator` returned PASS graded from the files · G4 accepted with the trace summary
in hand and every flagged proposal ruled · G2 confirmed, in brownfield · G5's landing ran where the
pinned knowledge-management copy exists. The region's semver is bumped, its stamp ratified.

**Not done:** a surface authored before G3 · an undispositioned survivor with no recorded waiver · a
validator FAIL, or a PASS read off its report · a proposal folded without the user's word · a floor
category with neither principle nor waiver · out of rounds · G4 unaccepted.

## Seats & checks

| seat | agent × skill(s) | produces / grades | spawn | peer edges |
|---|---|---|---|---|
| producer | `principal-architect` × `analysis-codebase`, `authoring-constitution` | brownfield: `codebase-analysis.md` + Essential-Floor read; all modes: the surface set + trace summary, plus flagged proposals and unresolved clarifications; never grades | standing across both jobs — at analysis in brownfield (**probe seat**), else first produce | takes each fix list from the validator directly |
| intent reviewer(s) | `devils-advocate` × `review-governance-intent` | stress-tests the frozen synthesis, lens-briefed coverage / coherence in a pair → survivors, tally, status: input you adjudicate, not the grade | cold at the synthesis review, count per the sizing ruling (**probe seat** when first) | none — messages you, never the user, never the producer or validator: a cold review stage, not in-loop traffic |
| validator | `validator` × `validation-constitution` | grades the surface set **from the files** — set + synthesis + trace summary, never the producer's report → PASS/FAIL + fix list; never authors | cold at first validation, messaged after | hands each fix list to the producer directly; the round-opening send is yours |

**Validation model:** two branches, different stages — the **sized end-stage review** of the
frozen `governance-intent.md` before G3, per `templates/sized-end-stage-review.md`, then the
produce↔validate loop, whose PASS is the authoritative grade on the surface set. Every verdict is
yours.

## Constraints

- **G1 mode-select** — evidence: `bash ${CLAUDE_PLUGIN_ROOT}/skills/analysis-codebase/scripts/detect-stack.sh .`
  (an input, never the quality gate), the source-file count, and any existing governance region in
  `CLAUDE.md` · rules: the user · decides: the mode — **greenfield | brownfield | amend** — which
  selects the stages and the producer's branch. Region present → amend; >5 source files with a
  framework → brownfield; else greenfield. Brownfield opens at analysis, the rest at the
  interrogation.
- **G2 analysis checkpoint** *(brownfield)* — evidence: the producer's summary, Essential-Floor
  table and clarifications · rules: the user · decides: **confirm** (→ interrogation) / **edit**
  (corrections, one bounded re-run) / **reject** (greenfield fallback, or abort). No machine
  validator: the analysis is an intermediate input, gated here and by the deterministic detect-stack
  baseline — the surface set is the deliverable, and that gets one.
- **Interrogation** *(yours, inline — no seat runs it)* — evidence:
  `${CLAUDE_PLUGIN_ROOT}/skills/authoring-constitution/references/INTERROGATION-AGENDA.md`'s ten
  dimensions, worked adaptively via `mochiko:analysis-iterative`, then the catalog deck
  (`…/authoring-constitution/references/catalog/`, which owns the shelf model) · rules:
  the user, card by card (recommend-then-arbitrate) · decides: the kept / dropped / minted / waived
  set. A low-tier declaration licenses agenda pruning, **every skip named, never silent**.
  Brownfield: the analysis pre-fills the existing-practices dimension, and
  detected-reality-vs-declared-intent conflicts are **confronted in the open**, never silently
  resolved. Amend: a micro-session scoped to the delta; a tier bump or un-waive is a governance
  event, taking its own agenda slice where tier-loosened rulings are re-dealt or force-re-marked.
  Offer **once** each module the synthesis records no ruling on, recording the answer either way — a
  decline stands until reopened, never re-asked. A layered principle kept or minted runs the
  **layered-architecture beat**: module ruling + domain-dependency seed arbitration
  (`…/authoring-constitution/references/DOMAIN-DEPENDENCIES.md`). Waivers ruled where the tier
  permits.
- **Review sizing** *(all modes, before G3)* — evidence: the synthesis's weight, stated per the
  template and purposed as input to the user's *elevation* of the default, never as the default ·
  rules: the user · decides: the reviewer count. The default is tier-keyed (`poc`/`internal` →
  single, `production`/`regulated` → pair), **event-scaled on amend**: a governance event (tier bump,
  un-waive, floor or waiver change) takes that full default; lighter deltas recommend single at
  `production`/`regulated`, none-with-waiver below. The gate always opens: every amend records a
  ruling or a waiver, keeping the trail audit-complete. **none** → straight to G3, waiver recorded.
- **Survivor rulings** — evidence: a survivor in user territory — deck, tier or waiver rulings,
  challenged dimension-prunes and scope decisions, and **user-declared facts** (team size, risk
  posture, lifespan) as confirmation · rules: the user · decides: its disposition. Reality-surface
  disputes take Bindings' fact route instead, never argument.
- **G3 synthesis confirmation** *(all modes)* — evidence: the reviewed synthesis with its tally and
  dispositions, or the recorded waiver · rules: the user · decides: **confirm** (record the stamp →
  the authoring loop) / **edit** (fold corrections, re-present; a **material** post-review change
  first takes a bounded delta-pass from the still-seated reviewer) / **reject** (back to the
  interrogation, re-entering the full flow — fresh sizing and review). Nothing is authored before it
  clears, so everything ratified was stress-tested or waived.
- **Clarification** *(in-loop)* — evidence: a producer or validator question it cannot resolve from
  its inputs · rules: the user · decides: the answer, fed forward into the round — a human gate
  inside the loop, never part of the done-condition.
- **G4 acceptance** *(only on validator PASS)* — evidence: the validated set — region version and
  tier, principle count by home (CLAUDE.md lines / rules files / skill pointers), floor accounting
  with waivers — **with the trace summary** and **every flagged proposal from the producer** · rules:
  the user, proposal by proposal · decides: **accept** (done) / **amend** (changes become the fix
  list; must PASS again) / **reject** (abort; drafts stay, stamped unaccepted). A proposal folds in
  only by the user's word, then the set re-validates.
- **G5 finalize** — evidence: the accepted artifacts and the PASS + acceptance trail · rules: the
  user · decides: retain or remove the brownfield `codebase-analysis.md`, the module scaffold where
  knowledge-management was adopted, and — where the accepted set includes rules files — the probe:
  **optional, token cost stated**, `mochiko:testing-governance-injection` verifies delivery
  empirically and its findings feed an amend run. Knowledge-management adopted → its rules file joins
  the probe surfaces and the module's dogfood **gates** on it: recommend, don't merely offer.
- **Escalation** — evidence: any of the bounds below tripping · rules: the user, holding the last fix list
  and the stop reason · decides: give-guidance-and-retry / accept-with-noted-gaps / abort — the run
  stays FAIL unless the user explicitly accepts.
- **Bounds:** cap **3** produce↔validate rounds (you count) · no-progress exit on a fix list
  unchanged round-over-round · kill-switch `.mochiko/memory/SETUP_STOP`, checked before every
  producer, reviewer or validator send · review caps: one cold read per reviewer, one four-message
  cross-exam, a two-exchange lead↔reviewer cap per survivor, one verify pass, plus one bounded delta-pass
  on a material G3 edit. The interrogation is bounded instead by user-driven convergence — a
  human-attended session, not an agent loop. Out of rounds = escalate, never done.
- **Workspace + hygiene:** `mkdir -p .mochiko/memory`. A `.mochiko/memory/constitution.md` on disk
  is a superseded pre-dissolution artifact — **delete it on sight**, no migration and no offer, and
  say so in one line.
- **Ownership boundary:** the governance region between `<!-- mochiko:governance:begin -->` /
  `<!-- mochiko:governance:end -->` is setup-owned and **idempotently regenerated** — re-runs and
  amends replace it in place; everything outside those markers is user content, never touched. Rules
  files, ledger and synthesis follow the same rule; the knowledge-management bundle scaffolding keeps
  its hard **never-overwrite** floor.
- **The synthesis is the producer's contract** (the selection-vs-formulation split and the
  flagged-proposal route are `authoring-constitution`'s); in amend it works from the current
  surfaces, preserving untouched principles and bumping the region's semver.
- **The validator's check surface is mode-parameterized every round** — brownfield adds the
  tools/versions↔`codebase-analysis.md` cross-check; an attached knowledge-management module adds the
  repo-level invariant re-audit from the project-pinned copy. Selecting it is a policy call that
  stays yours; the inbound fix list is peer-routed. **No devolved branch:** every verdict is a Tier-2
  judgment grade with deterministic sub-checks inside, never all-CLI, so no gate is skipped and no
  unit clears unread.
- **Out of scope, explicitly:** drift detection between invocations — waiver revisit triggers fire
  on re-invocation only, by design · backward compatibility with the retired `constitution.md` form.

## Bindings

- **The surface set** — the marked governance region in `CLAUDE.md` · `paths`-scoped
  `.claude/rules/mochiko/*.md` · skill pointers · `.mochiko/memory/governance-ledger.md` · the
  **trace summary** manifest. Their composition and read scope: `mochiko:authoring-constitution`.
- **The synthesis:** `.mochiko/memory/governance-intent.md`, `GI-XXX` namespace, assembled from
  `templates/governance-intent-template.md` (which owns the ID rule, the Review section and the
  delta-wise amend update). Durable (the amend baseline + traceability surface); never offer to
  delete it or the ledger.
- **Brownfield:** `.mochiko/memory/codebase-analysis.md` — the producer's `analysis-codebase`
  setup-brownfield output over detect-stack.
- **Uncertainty carrier:** the lead-penned synthesis, a confidence mark per GI element; the
  producer's own side rides its flagged proposals and clarifications.
- **Fact route:** reality-surface disputes → `codebase-analysis.md`, else a native `Explore` pass;
  user-declared facts are only confirmed, never checked.
- **Verify-pass owner:** the coherence-lens reviewer; the sole reviewer when sized down.
- **KM landing:** knowledge-management adopted → scaffold it at G5 per
  `templates/constitution-modules/knowledge-management.md` (which owns adoption granularity, the
  enforcement surfaces, the never-overwrite floor and collision rulings), including the
  **project-pinned copy** at `.mochiko/memory/knowledge-management.md`, which every command resolves
  at runtime. Amend with the module attached → G5 is setup's own KM landing: run that copy's ritual
  and invariants under fix-on-sight.

## Recovery

Note the resume stage in one line atop `governance-intent.md`, or in the region stamp once it
exists; resume from workspace evidence, respawning what the stage needs.

| Evidence in the workspace | Resume at |
|---|---|
| `.mochiko/memory/` missing or empty and no governance region in `CLAUDE.md` | G1 |
| brownfield chosen, `codebase-analysis.md` missing | analysis (produce) |
| `codebase-analysis.md` present, unconfirmed | G2 |
| mode set, `governance-intent.md` missing | interrogation |
| `governance-intent.md` present, Review section empty (no sizing ruling) | the sizing gate |
| sizing ruled, survivors undispositioned (sized-not-run, or mid-review) | the synthesis review — respawn reviewers per the ruling |
| dispositions folded, verify pass unrecorded | verify pass |
| Review section verified or waived, no confirmation stamp | G3 |
| synthesis confirmed, surface set missing or stale (no region, missing named rules files, no ledger) | loop (produce) |
| surface set present, no recorded PASS | loop (validate) |
| PASS recorded, not accepted | G4 |
| accepted | G5 — report the region, rules files, ledger, synthesis and any brownfield analysis, the PASS + acceptance trail, a suggested commit, next step `/mochiko:specify` (+ `/mochiko:brainstorm` when knowledge-management was adopted) |
| `SETUP_STOP` present | escalate |
