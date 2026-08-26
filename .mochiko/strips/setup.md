# Strip notes — `commands/setup.md`

Entry formats: `strips/README.md`. Wave context: the D4 codification pre-shrink — the
shared team-form prose relocated into `templates/command-shape.md` (design:
`.mochiko/brainstorms/pattern-codification-and-minimalism/record.md`, D3/D4/D9).
**Stale as a standing claim:** the shape is now **v5** — see the v0.35.0 section below.
**Also stale:** the shape is **v7** as of the v0.43.0 conversion below, and setup is **v7-form** —
it carries the `<!-- shape-form: v7 -->` marker and binds P18–P20.

<!-- Wave context: the product-architecture-schema Stage-1 build wave (v0.81.0). Ruling for the
[v0.81.0] entry below: `.mochiko/brainstorms/product-architecture-schema/record.md` (D5 · D12 ·
D16) → `DECISIONS.md` 2026-08-19 product-architecture row. -->

## [v0.91.0] Greenfield baseline-seed deferral re-keyed: "the first plan run" → "the first implement run's design phase" — plan-stage retirement D1/D2

- **Disposition:** superseded → both clauses defer the greenfield baseline seed to the first
  implement run's design phase.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` D1 (implement is the single
  downstream run) and D2's absent-baseline greenfield branch, which rehomes the v0.66.0
  baseline-seed obligation into the design phase: "an absent baseline file grades its touched
  surfaces 'new (gap)'; the design phase's first duty is the seed — empty scaffolds where no code
  is delivered, reconstruct-and-confirm with the user at the design checkpoint where delivered
  code exists"; wording ruled by the wave lead 2026-08-26).
- **Content (superseded fragments, verbatim — two sites):**

  1. ```
     greenfield leaves **the baselines** to seed at the first plan run. The architecture store's
     ```
  2. ```
       from the delivered code; greenfield seeds **those baselines** at the first plan run instead.
     ```

- **Kept deliberately:** the brownfield/greenfield split whole — brownfield close still carries
  the bootstrapped product baselines and still reconstructs them from delivered code; the
  greenfield deferral is still a deferral, only its destination is renamed. The v0.81.0 narrowing
  that made **the architecture store's scaffold unconditional on both paths** (its `spine.md`
  stub and `Scope:` line written either way) is untouched and still sits outside this split.
- **Budget:** commands carry no per-primitive budget; the hard cap and the justified-exemption
  path are the only bar, and neither is engaged by a two-phrase re-key.
- **Consumers assessed:** `implement.md` (P1's rewrite owns the design phase that now performs
  the seed); the v0.66.0 baseline-seed defect close
  (`.mochiko/decisions/2026-08-11-plan-baseline-seed-enforced.md`) was homed in the retired
  `plan.md` and is rehomed by D2 — recorded here because this line is the last surviving
  pointer to that obligation from the setup side.

## [v0.81.0] Greenfield's defer-to-first-plan-run clauses narrowed — the store scaffold is unconditional

- **Disposition:** superseded → both clauses now defer **the baselines only**; the architecture
  store's `architecture/` directory and its `spine.md` stub carrying the `Scope:` line are
  written on **both** paths, greenfield included.
- **Tier failed:** n/a — supersession by ruling (record D5 — shelf scope declared at setup,
  overridable at the desk — read with D16's store-less-repo posture; `DECISIONS.md` 2026-08-19
  product-architecture row. Raised as V2 audit finding N1, the greenfield `Scope:` hole: with the
  scaffold conditional on brownfield, a greenfield project reached its first desk visit with no
  declared scope and nowhere for setup's surface read to land).
- **Content:** verbatim, two sites — (1) Goal: "brownfield close also carries the bootstrapped
  product baselines at `.mochiko/product/`; greenfield leaves them to seed at the first plan
  run."; (2) the feature-map binding's tail: "greenfield seeds the baselines at the first plan
  run instead."
- **Kept deliberately:** the greenfield/brownfield split itself survives for **the baselines** —
  `data-model.md`, `contracts/`, `constraints-and-decisions.md`, and `quickstart.md` are still
  bootstrapped from delivered code on brownfield and still deferred to the first plan run on
  greenfield, and the `Assumed` mark with its partial-baseline poisoning risk is untouched. Two
  standing claims survive **because a `Scope:`-header-only file is scaffold, not ruled content**:
  setup's own "the store's ruled content is never authored here" (now stated explicitly of both
  paths), and `commands/plan.md`'s "**The store is never seeded here**" — seeding means ruled
  content, which the stub is not. No architecture stance is taken in a setup run on either path.
- **Consumers assessed:** `commands/plan.md`'s Baseline-seed bullet (its own strip entry, same
  wave) — unaffected, per the seeding/scaffold distinction above;
  `commands/architecture.md`'s shelves bullet reads the same `Scope:` line and needed no change,
  since it never assumed the brownfield path; P1's store layout and schema need no change either
  — "written by setup's scaffold" is now true as written on both paths.
- **Follow-on within the same wave (V2 advisory A5), no separate entry owed:** making the
  scaffold unconditional created a **scaffold-only** store state that the wave's own
  bootstrap/elicit routing did not yet name — it said "store-less", which a scaffold-only store
  is not. Four spans were re-worded in place to "a store with no ruled content (scaffold-only or
  absent)": `commands/plan.md` Entry, its Architecture-store bullet, its Baseline-seed bullet
  (this fourth site was not in the advisory — same phrase class, found on sweep), and
  `commands/architecture.md`'s baseline-authoring floor. The setup scaffold additionally lays an
  empty `concerns.md` beside the `spine.md` stub so the store's layout is complete from birth.
  **No strip entry is owed for these:** every re-worded span is v0.81.0-authored text being
  refined before the version ships (`plugin.json` still reads 0.80.0 at the time of the edit) —
  the ceremony governs content that shipped, and none of this has. Logged here so the
  scaffold-only vocabulary traces back to the ruling that created the state.

## [v0.81.0] `nfrs.md` leaves the brownfield bootstrap list; the store scaffold joins it (D12/D16)

- **Disposition:** superseded → the bootstrap list drops `nfrs.md` (the file dies whole under
  D12; its `NFR-XXX` ids survive on store concern rows) and gains the architecture store's
  scaffold at `.mochiko/product/architecture/`. *(Read with the entry above: the store scaffold
  ended up **unconditional**, not a member of the brownfield-only list this entry describes —
  same wave, one round later.)*
- **Tier failed:** n/a — supersession by ruling (record D12/D16; `DECISIONS.md` 2026-08-19 row).
- **Content:** verbatim — "the brownfield analysis also bootstraps the product baselines —
  `data-model.md` · `contracts/` · `nfrs.md` · `constraints-and-decisions.md` · `quickstart.md`
  at `.mochiko/product/` (`ARCHITECTURE.md` stays repo root) — from the delivered code;
  greenfield seeds the baselines at the first plan run instead."
- **Kept deliberately:** three things, deliberately. (1) The `Assumed` mark and its named
  partial-baseline poisoning risk survive verbatim — D16 explicitly makes the brownfield derive
  *inherit* that caveat, so weakening it here would have contradicted the ruling. (2) The
  parenthetical "(`ARCHITECTURE.md` stays repo root)" survives — still true, and now doubly so:
  the root file is the store's derived index, per D4's "at the repo root, never inside the store
  directory". (3) The greenfield/brownfield split and the first-plan-run seeding fallback are
  unchanged. **Added, not superseded:** setup lays the store *scaffold* only — the ruled content
  is the desk's reconstruct-and-confirm work at the first `/mochiko:architecture` visit (D16), so
  no architecture stance is ever taken in a setup run.
- **Consumers assessed:** the never-overwrite floor covering these writes is untouched;
  `plan.md`'s Baseline-seed bullet (own strip entry, same wave) states the matching
  store-is-never-seeded-here rule; the new **Architecture scope handoff** binding and the
  `/mochiko:architecture` addition to Next step are pure additions and ride the decision row.

## [v0.78.0] Model-tiering floor line retargeted — `mochiko:explorer` superseded by native `Explore` + `model: haiku` override

- **Disposition:** superseded → the reworded floor line: locate/enumerate reads go to "a
  native `Explore` subagent spawned `model: haiku`".
- **Tier failed:** n/a — supersession by ruling (ADR
  `.mochiko/decisions/2026-08-19-explorer-retarget-native.md`; `DECISIONS.md` 2026-08-19
  row). Dogfood failure: agent-team teammates cannot spawn plugin-scoped agents.
- **Content:** verbatim superseded phrase: "the cheap explorer seat (`mochiko:explorer`)"
  (line wrap varies per command; only this phrase changed).
- **Kept deliberately:** the rest of the floor line byte-for-byte — the class-key summary,
  session-tier carve-outs, the every-seat-brief obligation, and the closing
  `mochiko:patterns-model-tiering` referenced-never-restated pointer.
- **Consumers assessed:** the same phrase edited in all six commands in the same v0.78.0
  wave (entry mirrored per command strip file); the pointed-at skill reworded in the same
  wave (`strips/patterns-model-tiering.md`).

---

## [v0.76.0] Static template read-pointers re-pointed to the CLI-render / raw-schema two-arm home
- **Disposition:** superseded → each `templates/<t>-template.md` read-pointer now names the two-arm
  guidance home: `mochiko-cli template <name>` when the binary is present, else the shipped schema
  `plugins/mochiko/schemas/<name>.yaml` Read raw — the raw Read is the D8-first-class path, not an
  error state. Templates re-pointed in this command: **governance-intent** (The synthesis binding),
  **features-index** and **feature-entry** (Feature-map binding, brownfield reconstruction shape).
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-16 "Template-schema CLI
  ruled (D1–D11 as amended at review)" row; record
  `.mochiko/brainstorms/schema-based-template-guidance/record.md`, **D1** (a single plugin CLI is
  the guidance authority, static `.md` exemplars retired) + **D8** (schemas ship as structured data
  files, the binary renders over them, raw Read is the first-class fallback); build plan §5 re-point
  inventory)
- **Content (superseded, verbatim — the read-pointers that left):**
  - The synthesis: "from `templates/governance-intent-template.md`"
  - Feature map: "(shape: `templates/features-index-template.md` + `templates/feature-entry-template.md`;"
- **Kept deliberately:** every surrounding responsibility — the synthesis binding's `GI-XXX`
  namespace, durable-amend-baseline and never-offer-to-delete clauses; the Feature-map binding's
  brownfield-derivation, `delivered`-status + reconstructed-from-code mark, and its map-machinery /
  first-touch re-verify `mochiko:authoring-feature-map` reference — all untouched. Only the
  template-source token changed. Out of the 8-template set and unchanged:
  `templates/constitution-modules/knowledge-management.md` (KM module binding) and
  `templates/output-style.md` (Register).
- **Consumers assessed:** none — commands are entry points, nothing mounts them. Co-edited this wave
  under the same ruling: the 8 `plugins/mochiko/templates/<t>.md` deletions + their supersession
  strips (P3); the sibling command re-points `specify.md` / `plan.md` / `feature.md` (own strips);
  the skill/reference re-points + D7 re-key (P5, own strips) — including this command's own
  `${CLAUDE_PLUGIN_ROOT}/skills/authoring-constitution/references/INTERROGATION-AGENDA.md` reference,
  whose `governance-intent` pointer is re-pointed by the P5 skills seat, not here. The named
  `plugins/mochiko/schemas/*.yaml` files are authored by the schemas seat (P1) this wave — the path
  is fixed contract per the approved build plan, not created here.

---

## [v0.48.0] Shape v8 goal+harness rewrite — choreography dies in place
- **Disposition:** superseded → the v8 goal+harness rewrite of this command (whole-file)
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/command-architecture-realignment/record.md` D1–D6; DECISIONS.md 2026-08-02 command-architecture row)
- **Content:** the entire v7-form file superseded — preamble dispatch-brief protocol · Seats & checks table + validation model · team-transport mandate + roster probe (D5: transport-neutral now) · seat lifecycle/recycling · every G-numbered gate, the run-start weight card, floor-gate set, counted bounds/caps/kill-switch, ordering invariants, ground-rules block · run-start declaration + departure trail + per-run contract file · KM-landing command steps · the Recovery section and resume table. Verbatim text below (pre-edit file at the v0.47.0 tree).
- **Kept deliberately:** the Goal's ratify-before-author, closed-trace, floor-accounting, accepted-set condition · lead-inline interrogation via analysis-iterative + catalog deck recommend-then-arbitrate · lead-penned synthesis always cold stress-tested or user-waived (Independence line) · card rulings/waivers/ratification/conflict-confrontation/proposal-by-proposal acceptance reserved to the user · the full surface-set, synthesis, interrogation-input, ownership-boundary (idempotent region + carve-outs + constitution.md delete-on-sight) and KM-scaffold bindings · no-git-mutation + plain-blocking-acceptance lines · output-style register pointer
- **Consumers assessed:** none — commands are entry points, nothing mounts them.

<details><summary>Verbatim superseded file (v0.47.0)</summary>

````markdown
---
description: Establish or update the project's governance on the surfaces Claude Code natively loads — an interrogation session elicits the user's declared intent (the fact profile, type, risk, values) before anything is authored, closing on a confidence-marked synthesis that a sized cold intent review (pair default) stress-tests before the user ratifies it at G3 (pair / single / recorded waiver — the lead sizes it on the user's weight card); the ratified synthesis is then a traceable contract on a principal-architect producer teammate that authors the surface set (a marked CLAUDE.md governance region, paths-scoped rules files, skill pointers, a governance ledger), an independent validator teammate grades trace closure from the files, and the user accepts with the trace summary in hand. Greenfield | brownfield | amend; every stage user-gated, default-FAIL, bounded, kernel-free. Requires agent teams (CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS); refuses without them.
disable-model-invocation: true
---

# Setup — Governance From Interrogated Intent, On Native Surfaces

**Goal:** establish or update the project's governance so it follows the user's declared intent —
never a fixed baseline — and lives where Claude Code natively loads it. There is **no
`constitution.md`**: the deliverable is the surface set in Bindings. `$ARGUMENTS` = optional setup
request; empty is fine — detection proposes the mode.

**You are the lead**: you compose the run and own its counters, every verdict, every escalation,
every human gate, and the user-facing conversation — agents produce and review, you adjudicate.
Every dispatch carries its own brief in the spawn or send prompt — the seat's role and skill
(named as a hint, the agent decides fit), the exact inputs to Read, where the output lands
(write vs return), the bar it must clear, its peer edges and holds, and the independence
reminder that matches the seat (author: never grade your own output; grader: read the files
themselves, default FAIL, quote evidence) — the seat owns none of this context and gets all of
it from you; on a retry, a peer-routed fix list is pointed at and the round opened, a relayed
one pasted verbatim. This file is self-contained: setup's
whole contract lives here. Hard-requires `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS`. **First-spawn
probe:** the producer in brownfield; otherwise the intent reviewer(s) at the synthesis review, or
the producer if it was waived.

## Goal

The surface set in Bindings exists and carries the user's ratified intent, not a baseline: the
synthesis is ratified at G3, in every mode, its Review section closed — every survivor
dispositioned, or the waiver recorded · the trace closes across the set · G4 accepted with the trace
summary in hand and every flagged proposal ruled · G2 confirmed, in brownfield · G5's landing ran
where the pinned knowledge-management copy exists. The region's semver is bumped, its stamp
ratified.

**Not done:** a surface authored before G3 · an undispositioned survivor with no recorded waiver ·
an unclosed trace · a proposal folded without the user's word · a floor category with neither
principle nor waiver · a departure with no trail line · out of rounds · G4 unaccepted.

## Seats & checks

| seat | agent × skill(s) | produces / grades | spawn | peer edges |
|---|---|---|---|---|
| producer | `principal-architect` × `analysis-codebase`, `authoring-constitution` | brownfield: `codebase-analysis.md` + Essential-Floor read; all modes: the surface set + trace summary, plus flagged proposals and unresolved clarifications; never grades | standing across both jobs — at analysis in brownfield (**probe seat**), else first produce | takes each fix list from the validator directly |
| intent reviewer(s) | `devils-advocate` × `review-governance-intent` | stress-tests the frozen synthesis, lens-briefed coverage / coherence in a pair → survivors, tally, status: input you adjudicate, not the grade | cold at the synthesis review, count per the sizing ruling (**probe seat** when first) | none — messages you, never the user, never the producer or validator: a cold review stage, not in-loop traffic |
| validator | `validator` × `validation-constitution` | grades the surface set **from the files** — set + synthesis + trace summary, never the producer's report → PASS/FAIL + fix list; never authors | cold at first validation, messaged after | hands each fix list to the producer directly; the round-opening send is yours |

**Validation model:** two branches, different stages — the **sized end-stage review** of the
frozen `governance-intent.md` before G3 (protocol in Constraints), then the
produce↔validate loop, whose PASS is the authoritative grade on the surface set. Every verdict is
yours. No seat ever grades its own output.

**Team transport:** check `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` before anything else — unset →
stop and tell the user how to enable it (settings/env; Claude Code ≥ v2.1.178); the first spawn is
the authoritative probe, and there is no teamless fallback. A seat is spawned with **`name:`** — a
nameless spawn is a one-shot subagent, the forbidden transport; every later round is a
`SendMessage` to that same named seat. Verify from the roster: the `members` array in
`~/.claude/teams/<team>/config.json` (`<team>` = `session-` + first eight chars of the session ID)
must carry the seat's `name` — absent ⇒ kill and respawn explicitly requesting an agent team;
failing again stops the run. Teammates don't load `skills:` frontmatter — every spawn prompt names
the skill and role itself. Tell the user up front they can watch or message any teammate; announce
each seat in one line when filled; never narrate or reply to teammate housekeeping. A peer-routed
fix list is a **hand-off, not a start signal** — the producer revises only when you open the next
round, and your brief carries that hold.

**Seat lifecycle:** at each gate pause, count each standing multi-unit seat's completed
produce↔validate rounds (a brownfield analysis job counts as one) and recycle at ~≥3 — counted,
never observed; the user may order a recycle at any gate. Cold end-stage reviewers are exempt: they
already arrive cold at their own stage. A respawn is a reset: briefed from the on-disk artifact set
alone, versioned successor name (`producer-2`), never the dead seat's bare name. End-of-need
shutdown; no ritual sends.

## Constraints

- **G1 mode-select** — evidence: `bash ${CLAUDE_PLUGIN_ROOT}/skills/analysis-codebase/scripts/detect-stack.sh .`
  (an input, never the quality gate), the source-file count, and any existing governance region in
  `CLAUDE.md` · rules: the user · decides: the mode — **greenfield | brownfield | amend** — which
  selects the stages and the producer's branch. Region present → amend; >5 source files with a
  framework → brownfield; else greenfield. Brownfield opens at analysis, the rest at the
  interrogation.
- **Run-start weight card** — evidence: your stated read of the four rigor factors against this
  run's scope — a first surface set, or an amend's delta: **reversibility** (rework cost if wrong)
  · **blast radius** (how much downstream work reads the governance as authoritative) ·
  **precedent** (first-of-kind, or mirroring an audit-cleared pattern) · **input confidence**
  (scored on the artifact under review; a user ruling discounts ambiguity risk only, and one
  introducing new surface raises consistency risk) — plus the process you compose from it: the
  stated default below, or your departures from it · rules: the user · decides: the run's composed
  process. Rigor scales with cost-of-being-wrong, never task size; a first constitution scores
  high on every factor and earns the full apparatus.
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
  set. **No pruning license**: every dimension is real for a deployed, operated product — only
  convergence skips, each named, never silent. Brownfield: the analysis pre-fills the
  existing-practices dimension, and detected-reality-vs-declared-intent conflicts are
  **confronted in the open**, never silently resolved. Amend: a micro-session scoped to the
  delta, superseding any legacy tier declaration (the agenda's migration clause); a fact-profile
  change (module attach/detach) or un-waive is a governance event, taking its own agenda slice
  where affected rulings are re-dealt or force-re-marked. Offer **once** each module the synthesis
  records no ruling on, recording the answer either way — a decline stands until reopened, never
  re-asked. A layered principle kept or minted runs the
  **layered-architecture beat**: module ruling + domain-dependency seed arbitration
  (`…/authoring-constitution/references/DOMAIN-DEPENDENCIES.md`). Waivers ruled per D4 — any
  asserted standard, recorded justification, legal-mandate module obligations excepted.
- **Review sizing** *(all modes, before G3)* — evidence: the synthesis's weight, stated per the
  template and purposed as input to the *elevation* of the default, never as the default ·
  rules: you, presented on the weight card · decides: the reviewer count. The default is a
  **pair**, **event-scaled on amend**: a governance event (un-waive, floor change, module
  attach/detach) takes the full pair; a lighter substantive delta recommends single; a
  wording-level delta, none-with-recorded-waiver. The gate always opens: every amend records a
  ruling or a waiver, keeping the trail audit-complete. **none** → straight to G3, waiver recorded.
- **Review protocol** — the synthesis is **frozen** from reviewer spawn until every disposition
  lands (Review section excepted). Each reviewer reads it cold, forms findings independently,
  and reports findings-formed — count only — before its counterpart is introduced; a pair then
  runs the one-shot four-message cross-exam
  (`skills/review-brainstorm/references/CROSS-EXAM.md`, the pair protocol's single source —
  owner-withdrawal only, the counterpart persuades, never vetoes). Each reviewer returns its
  own survivors (severity, concrete failure scenario, resolution path, unresolved counterpart
  objections attached) and its own tally ("N raised, M survived"; fallen retrievable on ask)
  with a recommended status — **the cross-set merge and the combined tally are yours, never a
  reviewer's**. Your own formulation is argued with the finding's owner within the Bounds' cap;
  unresolved at the cap is a deadlock → tie-break with both positions plus your
  recommendation, the user rules. A fact already routed is cited, never re-routed; an
  overruled survivor marks its element `Contested`, and nobody re-raises it. The verify pass
  is floor, not sizing — wherever a review ran it is non-discretionary; a survivor still
  blocking after review + verify escalates to the user with both positions.
- **Survivor rulings** — evidence: a survivor in user territory — deck, fact-profile or waiver rulings,
  challenged convergence-skips and scope decisions, and **user-declared facts** (team size, risk
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
- **G4 acceptance** *(only on validator PASS)* — evidence: the validated set — region version,
  floor and attached modules, principle count by home (CLAUDE.md lines / rules files / skill
  pointers), floor accounting with waivers — **with the trace summary** and **every flagged
  proposal from the producer** · rules: the user, proposal by proposal · decides: **accept**
  (done) / **amend** (changes become the fix
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
- **Floor gates:** the run-start weight card · **G2** *(brownfield)* · the **Interrogation**'s card
  rulings · **Survivor rulings** · **G3** · the **Clarification**'s answer · **G4** · **G5** ·
  **Escalation** — the user's whatever you compose, never departable. **G1** and **Review sizing**
  (yours by ruling) are not: the mode is re-ruled in the room before anything is authored. Batch
  rulings into the fewest checkpoints that respect these gates.
  **Lead-penned surface:** `governance-intent.md` — always cold-graded, non-discretionarily; its
  `none` only on a recorded user waiver at the weight card.
- **Bounds:** cap **3** produce↔validate rounds (you count) · no-progress exit on a fix list
  unchanged round-over-round · kill-switch `.mochiko/memory/SETUP_STOP`, checked before every
  producer, reviewer or validator send · review caps: one cold read per reviewer, one four-message
  cross-exam, a two-exchange lead↔reviewer cap per survivor, one verify pass, plus one bounded delta-pass
  on a material G3 edit. The interrogation is bounded instead by user-driven convergence — a
  human-attended session, not an agent loop. Out of rounds = escalate, never done. Any bound this
  run declares — including a declared cost range — has you as its named counter, **rises only at a
  user checkpoint**, and is re-declared only on the record; busting a bound escalates, never
  silently continues.
- **Workspace + hygiene:** `mkdir -p .mochiko/memory`. A `.mochiko/memory/constitution.md` on disk
  is a superseded pre-dissolution artifact — **delete it on sight**, no migration and no offer, and
  say so in one line.
- **Ownership boundary:** the governance region between `<!-- mochiko:governance:begin -->` /
  `<!-- mochiko:governance:end -->` is setup-owned and **idempotently regenerated** — re-runs and
  amends replace it in place; everything outside those markers is user content, never touched. Rules
  files, ledger and synthesis follow the same rule; the knowledge-management bundle scaffolding keeps
  its hard **never-overwrite** floor, and the template's marked carve-outs — the domain registry and
  the output-style pair (the region's switch line and its Shape-5 rules file) — are preserved
  verbatim, written default-on once and never regenerated back over the user's values.
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
- **Ground rules:** kernel-free — no brain code, no capability catalogs, no DAG-mediated
  orchestration. Suggest commits; never run git mutations, never push. No internal machinery
  vocabulary in user-facing prose — the conversation is yours and the user's, in the mochiko
  register (`templates/output-style.md`). User acceptance is plain blocking text, never a timed
  prompt. The deliverables are written as the work progresses, never reconstructed at the end; the
  lead-penned synthesis reads standalone as the review surface — review findings and dispositions
  live in its closing Review section, never interleaved, and your pen covers your own formulation
  only: nothing amends a user-ruled element, and no new element exists, without the user's word.

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
- **Run-start declaration:** one line atop `governance-intent.md` — durable by binding, and where
  Recovery already notes the resume stage — written as the file opens, carrying the card as ruled;
  a run that departs from the stated default, or declares non-default bounds, writes a
  departure record at `.mochiko/memory/setup-contract.md` beside it instead — the
  done-condition and bounds as (re-)declared, departures taken, and the counter state Recovery
  reads on resume.
  Counted unit: the **produce↔validate round** the Bounds already count, a brownfield analysis job
  counting as one against the producer's cadence.
- **Departure trail:** one line per departure from the stated default, appended under that same
  declaration as it is taken and carried into G4's evidence — never your context alone; the trail
  names the grading that actually ran. Departure is by record, never by silence.
- **KM landing:** knowledge-management adopted → scaffold it at G5 per
  `templates/constitution-modules/knowledge-management.md` (which owns adoption granularity, the
  enforcement surfaces, the never-overwrite floor and collision rulings), including the
  **project-pinned copy** at `.mochiko/memory/knowledge-management.md`, which every command resolves
  at runtime. Amend with the module attached → G5 is setup's own KM landing: run that copy's ritual
  and invariants under fix-on-sight.

## Recovery

Note the resume stage in one line atop `governance-intent.md`, or in the region stamp once it
exists, with the run's counter state — rounds consumed · bounds declared · departures taken.
Sessions and teams do not survive `/resume`, and a shared account limit can throttle the team and
the main session together — escalation then has nowhere to go but pause. Resume from workspace
evidence, never a context `phase` field, respawning only what the stage needs — a respawn is cold
by design, so recovery never costs independence.

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
````

</details>

---
## [v0.46.0] Doctrine-purge rewrite — obligated reads out, shape mechanics inlined
- **Disposition:** superseded → the command's own text
- **Tier failed:** n/a — supersession by ruling (ADR `.mochiko/decisions/2026-08-02-doctrine-purge-wave-1.md`; DECISIONS.md 2026-08-02 "Doctrine purge wave 1" row)
- **Content:** the preamble's obligated shape/loop-discipline reads and "in the mochiko command shape" framing left.
- **Kept deliberately:** all gates/bounds/bindings/recovery (incl. the sized-end-stage-review sizing-gate read) — plus inlined weight-card factors, floor rules, transport, lifecycle (cold end-stage reviewer exemption stated), mesh hold, ground rules, as-you-go lead-pen rules, counter-state recovery.
- **Consumers assessed:** none.

---
## [v0.43.0] The `<!-- shape-form: v7 -->` marker retired from the preamble
- **Disposition:** superseded → deleted. The marker was added by this same version's conversion
  entry below and retires in the same version, at the wave close.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-01 wave-close
  ratifications row, *shape-form marker retirement when the last command converts*; the trigger
  was written into the marker clause itself). **Ground and full record:**
  `.mochiko/strips/command-shape.md` [v0.43.0 wave close], entry 1 — *The form marker and its
  Conformance bullet retired* — not restated here.
- **Content (verbatim):** `<!-- shape-form: v7 -->`
- **Kept deliberately:** the entire preamble otherwise — goal line, obligated reads, probe seat —
  and every P18–P20 binding the marker used to gate. The slots bind unconditionally now; nothing
  the marker declared was lost, because the marker declared only which grading branch to take, and
  there is one branch.
- **Consumers assessed:** `validation-command-shape` check 20 was the sole grep consumer and its
  form branch retired in the same ceremony. All six commands swept together — a marker left in any
  one of them would be the only file in the library still declaring a form.
- **Measured:** `commands/setup.md` **16,919 → 16,894 B** (−25). Derived figures in this note's
  conversion section re-measured accordingly, superseded values kept inline.

# v0.43.0 — the v6→v7 conversion

**Wave context:** shape **v7** landed at v0.40.0 (`lead-owned-process-flexibility`,
`.mochiko/brainstorms/lead-owned-process-flexibility/record.md`; `DECISIONS.md` 2026-08-01 — the
lead-owned-process-flexibility row plus the shape-v7 wave-close ratification row), with **D4** ruling
**convert-on-touch** and all six commands staying v6-form. The first conversion (`implement`,
v0.43.0) is audit-cleared and is this conversion's precedent; the user **widened the wave to all six
commands on 2026-08-01**, so setup converts here rather than at a later touch. BACKLOG:
"convert-on-touch residuals".

**No new check-6 term is claimed.** The first conversion already landed both v7 ceiling terms
(`+120` on Constraints where P18 binds, `+110` on Bindings where the P19/P20 pair binds —
`.mochiko/strips/validation-command-shape.md` [v0.43.0]). setup fits inside them: its P18 binding
measures **72 w** against implement's 100 w, exactly as the calibration note predicted ("implement's
100-w P18 additionally carries a clause no other command will — the ruled verification-depth floor —
so a typical P18 should measure well under it"). **This is a conversion that merely fits, which the
skill's own text says is not a re-key case.**

**The v0.36.0 Constraints advisory is discharged, not overridden.** That entry recorded setup at
1,076/1,080 w — "the tightest margin on this surface" — and ruled that "any future addition to
setup's Constraints needs a strip first". No strip was needed here because **the ceiling moved with
the additions, not around them**: the weight-card gate line is itself a gate, taking G from 10 to 11
and raising 90·(G+2) by 90 w, and the P18 binding brings its own `+120` term. 1,080 → 1,290 against
a block that grew 1,076 → 1,205, leaving **85 w (6.6%)** — the healthiest headroom this block has
had since v0.35.0, and inside the 6–9% band the P17/P19 term calibrations produced. The advisory's
substance still stands for anything that is *not* a gate line or a P18 binding.

**Post-conversion measurement, all blocks, body-only in words** (`## Heading` lines excluded, per
check 6): preamble **123/130** (published as 127 while the 4-word form marker stood;
retired at the wave close) · Goal **130/150** · Seats & checks **227/235** (unchanged) ·
Constraints **1205/1290** · Bindings **283/326** · Recovery **208/242** (unchanged). Term derivation
as check 6 requires: **G = 11** — the ten prior gate lines plus the run-start weight card, all eleven
carrying the complete three-part `evidence:`/`rules:`/`decides:` form — so Constraints is
90·(11+2) = 1170 **plus the +120 P18 term** = 1290. **S = 3** and **R = 13**, both unchanged.
**A = 8**, unchanged from the v0.35.0 reading (the governance region · the `paths`-scoped rules
files · the skill pointers · the ledger · the trace summary · the synthesis · the brownfield
analysis · the scaffolded KM copy), so Bindings is 90 + 12·8 + 30 (KM) **plus the +110 P19/P20 term**
= 326.

> **Two measurement judgments, recorded so the next auditor does not re-derive them.**
>
> 1. **The A term is unchanged at 8, and the conservative alternative also passes.** Check 6 bars
>    counting a **KM-landing fold target** — a doc the command folds *into* rather than produces.
>    setup does not fold into the project-pinned copy; it **scaffolds** it (G5, the KM-landing
>    binding), so it is a genuine own-output and the v0.35.0/v0.36.0 count of 8 stands. Recorded
>    because the reading is arguable: at **A = 7** the ceiling is 314 and Bindings' measured 283
>    still passes (90.1%), so nothing in this conversion turns on it.
> 2. **`setup-contract.md` is not counted in A.** P19 names it as a **departing** run's per-run
>    carrier; it is neither a deliverable nor a round report and exists only on a departing run.
>    Same disposition and same ground as the precedent's `implement-contract.md` judgment: counting
>    it would only loosen the check, so the conservative reading is the one measured.
>
> **One tight margin, and its baseline verified against git rather than carried.** The preamble
> measures **123/130 title-included**, **113 title-excluded** *(published as 127/117 while the
> 4-word form marker stood; re-measured at the wave-close sweep that retired it)*. Check 6's
> exclusion is written for a
> block's `## Heading` and the preamble has no `##` heading, so the letter of the rule leaves the
> reading unsettled — the same ambiguity `.mochiko/strips/implement.md` [v0.42.0] recorded rather
> than ruled. **Both readings clear 130.**
>
> A mid-wave warning held that this note's published v0.35.0 figure was measured title-*excluded*,
> which would put the real baseline ~8 w higher than published. **Checked and not true of setup:**
> `git show` on the goal-shape rewrite commits (`b32dd82`, `17465b7`, 2026-07-31) measures the
> preamble at **123 title-included / 113 title-excluded**, and the published figure is **123** — the
> title-*included* count, matching this wave's precedent. No hidden 8 w exists here and no trim was
> owed. Recorded so the next auditor does not re-derive it. (Every figure in this note was measured
> from the file this run, never carried from a prior wave.)
>
> At **7 w** on the title-included reading this is nonetheless the file's tightest block — but only
> just, and the margin is worth stating honestly: Seats & checks sits at 8 w (227/235), so the
> preamble leads by 1 w, not by the 5 the marker-era **3 w** figure implied. Any future preamble
> addition still needs a strip first, and so does a Seats addition.

## [v0.43.0] The Goal's end state loses its sized-review and validator-PASS clauses

- **Disposition:** superseded → rewritten in place as artifact state. The clearances the end state
  named **survive as the artifact states that carry them** — the synthesis's ratified stamp and its
  closed Review section (the review's own durable record, per
  `templates/governance-intent-template.md`), and the trace closing across the surface set.
- **Tier failed:** n/a — supersession by ruling (**D6(b)**, ratified at **A4**, 2026-08-01: *"Goal
  blocks lose process residue. Done = artifact state + floor compliance + user acceptance"*; graded
  by `validation-command-shape` check 23, v7-form only).
- **Protected content, leaving by ruling and named as such:** the second clause is
  `DECISIONS.md`-traceable — the [v0.35.0] CS-D8 ledger row for **D4** (*"the setup loop survives
  with a distributed target; **trace is the spine**"*) names the Goal's "PASS graded from the files"
  as one of that row's three homes. It is superseded by a cited ruling, not dropped, and **the row
  keeps all three homes**: the other two (Bindings' trace summary, the synthesis-is-the-contract
  invariant) are untouched, and the Goal home survives re-read as the trace's *closure* rather than
  as the seat's verdict — which is what "trace is the spine" always meant here. The first clause is
  traceable to the adversarial **D7.8** propagation row, whose four homes (done-condition, review
  caps, human gates, recovery rows) all survive: the review caps, the sizing and survivor-ruling
  gate lines, and the three Review-section Recovery rows are untouched by this wave, and the
  done-condition home survives as the Review-section clause below.
- **Content (v6, verbatim — the clauses that left):**
  ```
  the
  sized intent review ran (or its waiver is recorded) with every survivor dispositioned · G3 cleared,
  in every mode · `validator` returned PASS graded from the files
  ```
- **Replaced by (v7, verbatim):**
  ```
  the
  synthesis is ratified at G3, in every mode, its Review section closed — every survivor
  dispositioned, or the waiver recorded · the trace closes across the set
  ```
- **Kept deliberately:**
  - **The waiver escape**, in substance verbatim — "(or its waiver is recorded)" → "or the waiver
    recorded". Under **U1-B** this clause is no longer merely an escape but a *floor-compliance*
    element: `governance-intent.md` is lead-penned, so it ships uncold-read only on a recorded user
    waiver. The Goal now carries that invariant rather than a description of the review running.
  - **"G3 cleared, in every mode"** — the all-modes scope is kept word-for-word ("ratified at G3, in
    every mode"); only its position moved, into the synthesis clause it qualifies.
  - **Every survivor dispositioned** — unmoved in substance, re-sited on the artifact section that
    holds the dispositions.
  - **"never the producer's report"** — the anti-rubber-stamp half of the retired clause is **not in
    the Goal's gift to drop**: it survives verbatim in the validator seat row ("grades the surface
    set **from the files** — set + synthesis + trace summary, never the producer's report"), Read
    this run to confirm.
  - **The default pipeline itself** — both review branches stay stated in Constraints and in the
    validation-model line, exactly as written. Nothing about *what runs by default* changed; only
    the done-condition stopped naming it.
  - **The G4, G2, G5 and semver/stamp clauses** — untouched, verbatim.
- **Consumers assessed:** not a shared primitive. Four cross-file consumers checked: the grader's
  check 23 (this is the text it was written for) · `templates/governance-intent-template.md`, which
  owns the Review section and already declares it "this file's durable record of the sized pre-G3
  review (or its waiver)" — Read this run, no edit owed · `mochiko:validation-constitution`, whose
  Step 4 "Trace Closure Cross-Check (deterministic, both ways, over the manifest)" is the home the
  new clause references rather than restates — Read this run · and the other five commands, whose
  Goal blocks are each rewritten under their own note in this same wave, bound by nothing here.

## [v0.43.0] Review sizing flips user → lead (U4, executed at the conversion touch)

- **Disposition:** superseded → rewritten in place. The gate line survives whole as the stated
  default's carrier; only *who rules it* changed. The sizing decision is presented on the
  **run-start weight card**, which is user-ruled, so the user still sees every sizing call — they
  no longer own it.
- **Tier failed:** n/a — supersession by ruling (**U4**, 2026-08-01: *"review sizing passes to the
  lead by recorded supersession of the brainstorm-v2-2 ruling"*, ratified with the set at **A4**;
  executed here per `templates/sized-end-stage-review.md` **v2**'s interim note, Read this run:
  *"A command not yet converted to shape v7 still says the user rules on sizing; that stands, as
  written, in those commands **until their conversion touch**"* — setup's touch is this wave).
- **Protected content, leaving by ruling and named as such:** two `DECISIONS.md`-traceable lines
  from the [v0.35.0] CS-D8 ledger. (i) The **user-territory sizing ruling** — the ledger row
  *"Weight-statement inputs + elevation purpose … the elevation purpose **kept** on the sizing gate
  line"*. The elevation purpose is **kept, not dropped**: only its possessive left, so the clause
  now reads agnostic and the `rules:` clause settles who elevates. (ii) The **event-scaled keying**
  row is untouched entirely — it describes the *default*, not its owner.
- **Content (v6, verbatim — two sites):**
  ```
  purposed as input to the user's *elevation* of the default, never as the default ·
  rules: the user · decides: the reviewer count.
  ```
  ```
  (pair / single / recorded waiver — the user rules the sizing)
  ```
- **Replaced by (v7, verbatim):**
  ```
  purposed as input to the *elevation* of the default, never as the default ·
  rules: you, presented on the weight card · decides: the reviewer count.
  ```
  ```
  (pair / single / recorded waiver — the lead sizes it on the user's weight card)
  ```
- **Kept deliberately:**
  - **The default sizing keying, verbatim** — the pair default, the event scaling on amend
    (governance event → full pair; lighter substantive delta → single; wording-level →
    none-with-recorded-waiver). It survives as the **stated default**; what changed is that
    departing from it is now the lead's call, one P20 trail line.
  - **"The gate always opens: every amend records a ruling or a waiver, keeping the trail
    audit-complete"** — verbatim, and still literally true: a lead's sizing ruling is a ruling. The
    audit-completeness this clause buys is unchanged.
  - **"none → straight to G3, waiver recorded"** — verbatim. It is *narrowed*, not rewritten:
    **U1-B** makes `none` on setup's lead-penned synthesis not the lead's to take, and P18 carries
    that constraint rather than duplicating it into this line.
  - **The whole evidence clause's substance** — the weight statement, its template sourcing, and
    the never-as-the-default warning.
- **The frontmatter site was edited rather than held.** The v0.35.0 note holds `description:` under
  the trigger-fidelity rule, and that rule is about *not re-shrinking* it — it does not license
  shipping a sentence the body now contradicts. "the user rules the sizing" would have been simply
  false after the flip, so it is corrected in place at equal length, its trigger content (pair /
  single / recorded waiver) untouched.
- **Consumers assessed:** (i) `templates/sized-end-stage-review.md` v2 — already flipped; this
  conversion is what its interim note defers to, and after this wave its "unconverted command wins
  for its own run" carve-out no longer reaches setup. Not edited (template, out of scope). (ii)
  **`brainstorm.md`** — the other sized-review binder; its flip is being executed in this same wave
  under its own note. (iii) **`templates/governance-intent-template.md`:157** still prints
  `**user ruled:** [pair | single | none]` in the Review-section scaffold — a **live residue of
  this flip, reported not edited** (template, out of scope): the field label now names the wrong
  ruler for a v7-form setup run. Flagged for the wave lead. (iv) Recovery's row keyed on
  *"Review section empty (no sizing ruling)"* — unaffected, a ruling still gets recorded.

## [v0.43.0] The not-done state `a validator FAIL, or a PASS read off its report`

- **Disposition:** superseded → rewritten in place as `an unclosed trace`. The same state, named by
  the artifact property that is missing rather than by the seat's verdict.
- **Tier failed:** n/a — supersession by ruling (**D6(b)**, as above). It named the validator's
  verdict, which is the residue class check 23 fails by name ("the validator returned PASS").
- **Protected content, leaving by ruling:** `DECISIONS.md`-traceable to the same **D4** row as the
  entry above, and to the adversarial **D4/D5** row (the `validation-*` vs `review-*` family split).
  The [v0.35.0] ledger assigns that second row's home to the **intent-reviewer seat row** and the
  **validation-model line**, not to this Goal state — both untouched. Only the Goal echo left.
- **Content (v6, verbatim):** `a validator FAIL, or a PASS read off its report`
- **Kept deliberately:** the read-from-the-files rule and the never-from-the-report rule, both in
  the validator seat row; the authoritative-grade declaration, in the validation-model line; and
  every other not-done state, unedited — including **"a floor category with neither principle nor
  waiver"** (Setup-v2 **D1**) and **"a surface authored before G3"**, the ordering invariant G3's
  own gate line carries ("Nothing is authored before it clears").
- **Consumers assessed:** as above — not a shared primitive; grader check 23 and the five sibling
  conversions, none of which reads this line.

*Pure additions this wave, riding the decision row rather than these entries:*

- **The form marker** `<!-- shape-form: v7 -->` in the preamble — check 20's branch key.
- **The run-start weight-card gate line** (P7) — U1-A's standing user stop, in the three-part
  countable form, taking **G from 10 to 11**. Keyed to setup's own weight object: the run's scope,
  *a first surface set or an amend's delta* — the distinction the whole driver session turned on
  (a 2-element amend that drew the full apparatus).
- **`**Floor gates:**`** (P18) — the floor set (the run-start weight card · **G2** in brownfield ·
  the **Interrogation**'s card rulings · **Survivor rulings** · **G3** · the **Clarification**'s
  answer · **G4** · **G5** · **Escalation**) with the non-floor two named, so the absence is stated
  rather than inferred; and the **lead-penned surface** element bound to `governance-intent.md`
  (check 21(2)), setup's P11 being the lead-penned-record branch.
- **`**Run-start declaration:**`** (P19) and **`**Departure trail:**`** (P20) in Bindings — the
  declaration atop `governance-intent.md` for a default run, an instantiated `setup-contract.md` for
  a departing one, and the **produce↔validate round** named as the counted unit (check 22).
- **One new not-done state** — `a departure with no trail line`, the honest-trail invariant made
  visible in the Goal as floor compliance. Same addition as the precedent's.

**Four judgments made here rather than deferred, flagged for the grader.**

1. **The floor-gate set is nine of eleven, and the ground is *who rules*.** setup is the library's
   most user-ruled command — **ten of eleven** gate lines read `rules: the user` after the U4 flip
   above (Review sizing is the eleventh, now `rules: you`) — because its entire product *is* the
   user's rulings, so a large floor set is the honest reading rather than an over-marking. What the lead still composes is substantial and unaffected: whether and how large
   the intent review runs, how many produce↔validate rounds, the interrogation's pacing and
   convergence, and how few stops deliver the floor rulings.

   **Three limbs scoped narrowly, not whole gates.** Following the precedent's **G3** treatment:
   the **Interrogation** is floor on its *card rulings* — the kept/dropped/minted/waived set is the
   user's; the session's pacing and its named convergence skips are the lead's. **Survivor rulings**
   is floor where a review ran and survivors exist — it is conditional by construction, not
   departable-when-inconvenient. The **Clarification**'s *answer* is floor; when it is presented
   stays the lead's under **D3**'s consolidation authority, which is home doctrine and deliberately
   not restated in the command.

   **Why G5 is floor, since it is the least obvious member.** Its decides-clause rules on
   *deleting* the brownfield analysis and the module scaffold, and on a probe whose token cost is
   stated — and under **A3** a declared cost range is a floor bound, which rises only at a user
   checkpoint. Both are the user's. A light run where nothing needs removing and no rules files
   exist reaches the gate **vacuous, not departed**, so marking it floor costs nothing in the light
   case and protects the heavy one.

2. **Why G1 clears, on the ground that actually carries it.** G1 reads `rules: the user`, so its
   exclusion needs a structural ground, not a convenience one. The ground is that a mis-detected
   mode cannot cost a surface: **nothing is authored before G3** (G3's own gate line, floor), and
   the interrogation between them is **attended card by card** — so a wrong mode surfaces to the
   user, in the room, before any surface exists. G1's confirm is a convenience stop; the invariant's
   real carriers are G3 and the interrogation, both floor. The narrower argument — that the
   detection rules are deterministic — was **not** relied on: it says nothing about who should rule
   a borderline call.

3. **Why Review sizing clears — by construction once the U4 flip landed, not by argument.** The
   gate line now reads `rules: you`, so check 21's floor test excludes it the same way implement's
   cycle checkpoint is excluded: mechanically, on the `rules:` clause. No structural argument is
   needed or offered.

   **Recorded because the first draft of this conversion got here the hard way, and the trail should
   show it.** Before the flip was directed, this entry argued the exclusion against a line still
   reading `rules: the user` — with the gate's own "**The gate always opens**: every amend records a
   ruling or a waiver, keeping the trail audit-complete" sitting on top of it. That is exactly the
   shape of the precedent's fix-round defect (a `rules: the user` gate outside the floor set whose
   own sentence reads as never-departable), and it was surviving only on an argument. **The flip
   dissolves the tension rather than answering it.** The always-opens clause is untouched and still
   true — a lead's sizing ruling is a ruling — and a below-default sizing is one P20 trail line, so
   audit-completeness is carried whole.

   **The residual user-ruled limb is not lost — it is where P18 puts it.** Because
   `governance-intent.md` is lead-penned, a `none` sizing is *not* the lead's to take: the floor's
   second invariant gives that artifact one cold-seat grade unless the user waived it on the record
   at the weight card. P18 states exactly that, so the one part of sizing that stays the user's is
   bound to the surface it protects rather than to the gate that lost ownership.

   **One divergence from the template's interim clause, taken deliberately.**
   `sized-end-stage-review.md` v2 says a below-default sizing is recorded "where that command
   already records review outcomes — the artifact's **Review section**", adding that "P20 names that
   home explicitly at the conversion touch". setup's P20 names the **declaration head** of the same
   file, not the Review section, because P20 must be one home for *every* departure — collapsed
   stages and skipped rounds have no business in a Review section. The `none` waiver still lands in
   the Review section under the template's own rule; the two records are different things (the
   review's own artifact record vs. the departure record) and both survive.

4. **The declaration and the trail share one surface, chosen for durability.** Both land atop
   `governance-intent.md`. **`codebase-analysis.md` was rejected as a home** even though it is the
   first artifact a brownfield run writes: G5 rules on *removing* it, so a trail parked there is a
   trail the run's own last gate can delete — the same failure mode the precedent avoided when it
   rejected `cycle-report.md` for being overwritten. The synthesis is the only run artifact Bindings
   marks **durable** and never-offer-to-delete, and Recovery already keeps resume state on it, so a
   resumed lead finds declaration, departures and resume state in one place.

   **The counted unit is the produce↔validate round**, the unit the Bounds already count. The
   interrogation was rejected as the denominator: it is lead-inline with no seat running it, so it
   cannot be any governed seat's unit. One precision added because setup's producer is the library's
   only seat standing across **two jobs** — a brownfield analysis job counts as one against its
   cadence. Without that clause the producer's denominator is ambiguous in brownfield, which is
   exactly the cadence-exemption-by-missing-denominator failure **OQ-4** exists to prevent.

**Recovery left untouched, deliberately** — the same disposition as the precedent's, and for the
same reason. The shape's counter-state clause is home doctrine (v7 Recovery block); setup's pause
line already names `governance-intent.md`, the surface P19 binds. One difference from the precedent
was checked rather than assumed: setup's pause line also offers the region stamp as an alternative
resume site ("or in the region stamp once it exists"), while P19 binds the declaration to the
synthesis alone. That is a divergence without a contradiction — the *resume stage* may move once the
region exists; the declaration and its trail stay where they were written, on the durable artifact.
No edit was owed.

### R21 — a second heavy site, not the light one

`lead-owned-process-flexibility` **R21** remains **open at half**. Its residue is *the light site*,
and **setup is not it**: setup is the second-heaviest command in the library — the densest gate
surface (G = 11 after this wave, against implement's 9) and, per the record's own errata, **the
heaviest bound surface in the set by a factor of two** (F60-a, 8 individual hard bounds). These
figures corroborate the heavy band the precedent measured; they do not close R21. The next
conversion of a genuinely light command (`specify`, `slice`, or `brainstorm`) closes it.

**File growth.** `commands/setup.md` **15,331 → 16,894 B** (+1,563; words 2,103 → 2,331, +10.8%).
Attribution, each construct measured on its own text:

| construct | bytes | words |
|---|---|---|
| ~~`<!-- shape-form: v7 -->` marker~~ — added here, **retired at the wave close** | ±0 | ±0 |
| run-start weight-card gate line (P7) | +315 | +54 |
| `**Floor gates:**` — floor set + non-floor two + the lead-penned surface (P18) | +514 | +72 |
| U4 review-sizing flip — gate line (+18 B) + frontmatter (+18 B) | +36 | +7 |
| `**Run-start declaration:**` (P19) | +536 | +69 |
| `**Departure trail:**` (P20) | +162 | +26 |
| Goal block, D6(b) residue strip | ±0 | ±0 |
| **net** | **+1,563** | **+228** |

**The Goal row is a genuine zero, not an unmeasured cell.** The block is byte-identical in size
(778 → 778 B) and word-identical (130 → 130 w) across a rewrite that changed three clauses:
`wc`-verified after the last edit, and the diff is in the commit. **No offsetting saving is claimed**
— the residue strip returned what the two new Goal elements cost, and nothing more.

**Per-run read cost.** setup.md is an obligated read once per run, so this is **+1,588 B on every
setup run** — 10.4% on top of the command itself, within a hair of the precedent's 10.6% at the
heavy site. As at that site, it is not the whole delta a run pays for v7: the shape home's
always-read floor (+11,399 B/run, measured at v0.40.0) dominates, and setup additionally binds
`sized-end-stage-review.md`, whose v7 revision added +2,131 B on the two commands that read it.
Against those, the conversion is the small share of the bill.

---

# v0.36.0 — the production-only re-key (stage 4)

**Wave context:** the PO narrowing build, stage 4 of 5 — `setup.md` and `implement.md` aligned with
the constitution cluster rewritten earlier in the same wave. Scope ADR:
`.mochiko/decisions/2026-07-30-po-narrowing-build-scope.md` (rulings 4, 6, 7, 9), scoping PO-D1–D7
from `.mochiko/brainstorms/production-only-focus/record.md`. **A ruling re-key inside the existing
five blocks, not a re-shape:** shape stays **v5**, G = 10, S = 3, A and R untouched, no block
re-bounded. **Constraints measured 1,051 → 1,076 w against the 1,080 ceiling (90·(G+2)) — 4 w of
headroom**, the tightest margin on this surface; the re-keys were written at minimum length in the
ruling's own vocabulary for that reason, and any future addition to setup's Constraints needs a
strip first.

## [v0.36.0] The tier ladder retired from setup's five ruled sites
- **Disposition:** superseded → the asserted production floor + the fact profile, whose homes are
  `authoring-constitution/references/INTERROGATION-AGENDA.md` (dimension 2, the no-pruning-license
  clause, the amend migration clause), `…/references/COMPLIANCE-MODULES.md` (mechanical module
  attachment, the D4.2 legal-mandate stratum) and `templates/governance-intent-template.md` (the D4
  waiver table, the event-scaled sizing line)
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/decisions/2026-07-30-po-narrowing-build-scope.md`; PO-D1–D7)
- **Content:** the five superseded keys, old → new —
  1. **Frontmatter** — "declared intent (tier, type, risk, values)" → "(the fact profile, type,
     risk, values)"; "a tier-sized cold intent review" → "a sized cold intent review (pair
     default)".
  2. **Interrogation** — "A low-tier declaration licenses agenda pruning, **every skip named, never
     silent**." → "**No pruning license**: every dimension is real for a deployed, operated product
     — only convergence skips, each named, never silent." · "a tier bump or un-waive is a governance
     event, taking its own agenda slice where tier-loosened rulings are re-dealt or force-re-marked"
     → a fact-profile change (module attach/detach) or un-waive, **affected** rulings re-dealt, plus
     the legacy-tier-declaration supersession pointer at the amend clause · "Waivers ruled where the
     tier permits." → "Waivers ruled per D4 — any asserted standard, recorded justification,
     legal-mandate module obligations excepted."
  3. **Review sizing** — "The default is tier-keyed (`poc`/`internal` → single,
     `production`/`regulated` → pair) … a governance event (tier bump, un-waive, floor or waiver
     change) takes that full default; lighter deltas recommend single at `production`/`regulated`,
     none-with-waiver below." → "The default is a **pair** … a governance event (un-waive, floor
     change, module attach/detach) takes the full pair; a lighter substantive delta recommends
     single; a wording-level delta, none-with-recorded-waiver."
  4. **Survivor rulings** — "deck, tier or waiver rulings" → "deck, fact-profile or waiver rulings".
  5. **G4 evidence** — "region version and tier" → "region version, floor and attached modules".
- **Kept deliberately:** the sizing gate's always-opens clause with its weight/elevation evidence
  (the event-scaling changed, the gate did not) · the amend re-deal / force-re-mark mechanic, re-keyed
  to "affected" rather than deleted · **"Tier-2 judgment grade"** in the validator-check-surface
  invariant — the validator-trustworthiness sense, untouched by the governance-tier retirement and a
  deliberate survivor of ruling 9's residue grep.
- **Residue reported, not edited:** Survivor rulings still reads "challenged dimension-prunes" —
  stale vocabulary for a retired mechanism (the agenda now says *convergence skips*), outside this
  stage's ruled sites and left for the lead to rule on.
- **Lead ruling on the residue (same wave, post-report):** edited — "challenged dimension-prunes" →
  "challenged convergence-skips", the same rename the cluster audit required in
  `review-governance-intent` (its finding 5); word-count-neutral against the 4-w Constraints
  margin. Lead-authored, covered by the stage-4 delta re-audit.

---

# v0.35.0 — the goal-shape wave (CS-D10 step 4)

**Wave context:** command goal-shape rebuild, **step 4 of 4** — the remaining-five wave (design:
`.mochiko/brainstorms/command-succinctness-strip/record.md`, CS-D3/D4/D5 + D8 + D10; `DECISIONS.md`
2026-07-30; pilot precedent `.mochiko/strips/plan.md` v0.34.0). Authored against **shape v5** with
the obligated `mochiko:loop-discipline` read **retained** — the drop stays deferred to the named
live-run trigger (pilot-checkpoint ruling 5), so a v5 command omitting it is non-conformant, not
early. setup is the **heaviest of the five** and the **only one of the two sized-review consumers
that also runs a produce↔validate loop**, so it is the first command to bind
`templates/sized-end-stage-review.md` — check 1's *positive* direction, the mirror of the pilot's
first live test of the negative direction.

**Measured: 2,768 → 2,074 words (−25.1%), 20,731 → 15,102 B (−27.2%)** (`wc`, re-run after the final
trim pass). Against the pre-wave floor row of **1,629 w: +445 w (+27.3%)** — over, which is CS-D8's
safe side (materially *under* would signal dropped content), but a much larger overage than the
pilot's +8.9% and accounted below rather than waved through.

> **Wave discipline honored** (pilot block-quote): `wc` re-run and every figure in this note swept
> after the last edit landed, not at first draft. Three earlier drafts of this file were measured
> and discarded for ceiling failures before the figures above were taken.

**Blocks against the grader's ceilings** (terms as check 6 counts them — **G = 10** gate lines,
**S = 3** seat rows, **A = 8** artifacts, **R = 13** resume rows): preamble 123/130 · Goal 131/150 ·
**Seats & checks 230/235 (97.9%)** · **Constraints 1,052/1,080 (97.4%)** · Bindings 189/216 — and 189
still fits the floor row's own A = 6 value (192) · Recovery 209/242. Two blocks sit above 97%: the
Seats ceiling of `100 + 45·S` is tight for a 3-seat command whose every seat carries a
conditionality (mode-keyed probe, sizing-keyed count, cold-then-messaged), and Constraints is
discussed below.

**G = 10, not the floor row's 8 — the load-bearing call of this rewrite.** The ten gate lines are a
one-to-one translation of the *retired Contract's own Human-gates enumeration*: G1 mode-select · G2
analysis checkpoint · the interrogation itself · the in-loop clarification sub-gate · the review
sizing gate · survivor rulings on user territory · G3 · G4 · G5 · escalation on any guard trip. Each
has all three parts the shape requires (opening evidence · who rules · what it decides), so each is
a gate by the shape's own definition rather than by carrying a number. **The arithmetic matters for
the checkpoint:** at G = 8 the Constraints ceiling is 900, and setup's translated Constraints needs
1,052 — so with the floor row's assumed gate count this command could not be authored without
dropping protected content. Two readings are available and the wave lead should pick one
deliberately: (a) the gate count is honest at 10 and the formula tracks it correctly — the pilot's
"confirm the ceiling" recommendation stands; or (b) `90·(G+2)` under-serves the most gate-dense
command and the floor row's G estimates need re-keying before they are cited as targets again.
Recorded here because an author choosing its own G is exactly the quota-override the sibling wave's
D1 forbids, and the defense has to be checkable: every gate line traces to a named Contract entry.

**Where the +445 w sits, line by line.** Two extra gate lines vs the floor's model (the
interrogation, 139 w, and escalation, 35 w — the floor priced them inside the invariant set at
~50 w each): **~+75**. The floor's fitted per-gate cost of 40–55 w is contradicted by the *pilot's
measured* per-gate cost of ~74 w (plan: 7 gates, ~520 w of gate lines), and setup's eight remaining
gates carry three-option decision menus at G2/G3/G4: **~+190**. Blocks landing between the fitted
model and the ceiling — Seats 230 vs fitted 185, Bindings 189 vs 149, Recovery 209 vs 188, Goal 131
vs 100: **~+130**. Frontmatter `description:` +8 w over today's (held, per the trigger-fidelity
rule, not re-shrunk). The remainder is rounding. No block is over ceiling; nothing was padded to
reach a ceiling.

**Stale as a standing claim:** the narrowing build has now run — see the **v0.36.0 section above**;
the paragraph below records the v0.35.0-era state only.

**Production-only (PO-D1–D7) deliberately NOT pre-implemented.** The narrowing build is open
(ROADMAP *Next*; BACKLOG "Production-only narrowing"), so setup's **tier axis survives verbatim** —
the tier-keyed sizing default, the low-tier agenda-pruning license, tier-parameterized catalog
cards, and tier-permitted waivers. A future auditor reading the tier language as stale should read
this line first: it is current, and PO-D2 retires it at *that* build, not this one.

**Build note — concurrent draft.** A goal-shaped draft of this file by another agent was found in
the working tree mid-task (2,393 w; ceiling FAILs on preamble 147/130, Goal 156/150, Seats 391/235,
Constraints 990/810 — and it retained the `shape-exception` marker, dropped the clarification gate,
the escalation menu and the amend semver-bump). It was preserved, not overwritten blind, at
`…/scratchpad/setup-FOUND-draft-by-other-agent.md`; several of its phrasings survive in this
authored version. Logged so the duplication is visible rather than silent.

## [v0.35.0] The phase body and the Contract section retired into the five-block anatomy
- **Disposition:** superseded → the goal-shaped anatomy. `Detect & mode-select` → **G1** · `Brownfield
  analysis` → **G2** + the producer row · `Interrogation` → the **Interrogation** gate line · `Synthesis
  review` → **Review sizing** + **Survivor rulings** + Bindings (marks, verify-pass owner) · `Synthesis
  confirmation` → **G3** · `Authoring loop` → the seat rows + **Clarification** + **Escalation** +
  Bounds + the validator-check-surface invariant · `Acceptance` → **G4** · `Finalize` → **G5** + the
  KM-landing binding + Recovery's accepted row. The `Contract` section's clauses → **Goal**
  (done-condition + not-done states), the **Seats & checks** table (producer↔validator), **Constraints**
  (bounds · gates · out-of-scope). The `Session constraints` and `The seats` sections dissolve into
  Constraints invariants and the table.
- **Tier failed:** n/a — supersession by ruling (**CS-D3** condition-first documents · **CS-D4** the
  connective procedure is deleted and what survives is restructured · **CS-D5** the five-block anatomy
  and the Contract-as-document inversion).
- **Content:** the `## The flow` narrative (8 stage paragraphs) + `## Contract` + `## Session
  constraints` + `## The seats` + the `What you own (not the agents)` footer — 2,768 words of ordered
  procedure, appendix and duplicate ownership list. Not reproduced verbatim: every *rule* inside them
  is resolved individually in the CS-D8 ledger below, and the deleted remainder is connective
  narration (stage names as prose, "Then…", the lead's job description restated per stage).
  Recoverable in full at `git show c47684d:plugins/mochiko/commands/setup.md`.
- **Kept deliberately:** every gate, bound, routing decision, trigger, threshold, ordering rule and
  artifact binding — see the ledger.

## [v0.35.0] The `shape-exception` marker RETIRED — the surface's last one
- **Disposition:** superseded → `templates/command-shape.md` **v5 Layer 2, "Independence by
  structure"**, which now states the rule *and* its falsified converse in the file setup obligately
  reads: "Independence is carried by who fills the seats and when they arrive — disjoint agents,
  disjoint skills, no seat grading its own output, all checkable in P5's table… never a persona's
  say-so, and **for in-loop seats never a routing pattern**." The command's prose assertion goes with
  the marker; independence is now shown by the **Seats & checks** table, which check 7 grades
  mechanically. **setup carries zero exception markers, and the surface-wide inventory is now 0.**
- **Tier failed:** n/a — supersession by ruling (**CS-D8** re-grade + the checkpoint's
  re-justify-or-supersede instruction; the pilot's precedent at `strips/plan.md` v0.34.0).
- **Content (the retired marker, verbatim):** `<!-- shape-exception: setup carried the library's most
  explicit statement of the falsified routing=independence claim; the correction is stated at the seat
  so the deletion is not silent -->` — together with the sentence it protected: "Independence here is
  structural — disjoint agents, disjoint skills, cold spawn — never the routing."
- **Grounds for retirement, stated plainly because this is the wave's one contestable marker call:**
  the marker's cited ground is **not a shape section at all** — it is a historical fact about setup's
  own v0.31.0 text ("setup carried the library's most explicit statement…"). Check 9 at v5 requires
  every surviving marker to be re-justified *against the v5 home*; a ground that names a
  two-revisions-old state of the graded file itself cannot be. The non-silence the marker bought is
  now carried durably by this note's v0.31.0 entry (which quotes the falsified text verbatim), and the
  operative correction is carried by the v5 home. Retaining it would also fire check 8 twice
  (`disjoint agents, disjoint skills` **and** `structural separation` are both homed markers), leaving
  an exception whose only function is to license a restatement the anatomy no longer needs.
- **Kept deliberately:** the two *facts* the sentence carried — the validator's **cold spawn at first
  validation** (its spawn cell) and **no seat grading its own output** (the table itself). Nothing
  about independence is asserted in prose any more; it is structural and visible.

## [v0.35.0] Skill- and template-owned content stripped from the command body
- **Disposition:** relocated → the primitives that already own it. No new home written; **each home
  Read this run to confirm the text is actually there** (check 14's bar).
- **Tier failed:** 1 (altitude).
- **Content, with the home verified:**
  - The **surface-set composition** — the region's contents ("ratified stamp · principle index ·
    universal principles as short imperative lines · tech stack · quality-gates summary · module
    pointers"), the rules-file and skill-pointer roles, the ledger's contents *and its read scope*
    ("read only by setup/amend and the validator"). Home: `mochiko:authoring-constitution` — its
    description carries the set verbatim and its disclosure-tier table carries "Read only by
    setup/amend runs and the validator". Bindings now names the set's paths and points at the skill
    for composition + read scope.
  - The **trace summary's definition** ("each GI element → its primary enforceable home + companion
    entries") and its **trace classes** ("deck-kept / minted / floor-preset"). Home:
    `authoring-constitution` — "**Trace summary**: the manifest — every GI element → primary home +
    companions" and the canonical ledger key `GI-XXX (deck-kept: CARD-ID | minted | floor-preset:
    CARD-ID)`. **Kept:** *when* it is presented (G4) and *who* reads it (the validator).
  - The **validator's check enumeration** (two-way trace closure, region-marker integrity,
    index→file existence, Three-Part completeness, tier/waiver/floor accounting, anti-pattern and
    placeholder scans, the semver call). Home: `mochiko:validation-constitution` (its own numbered
    steps + verdict lines). **Kept, because the skill does NOT carry them:** the **mode-parameterized
    additions** — brownfield's tools/versions↔analysis cross-check and the KM module's repo-level
    invariant re-audit — which are the *grounds* for the lead-routed outbound leg.
  - The **lens definitions** ("coverage: missed dimensions, prune audits, the card-acceptance and
    waiver/module sweeps" / "coherence: tier↔risk↔ruling alignment, the mark/echo-rationale audit,
    reality-conflict resolutions, cross-element contradictions") and the **reviewer's read set** (the
    frozen synthesis + interrogation agenda + brownfield analysis). Home:
    `mochiko:review-governance-intent`, which states both lenses in the same words and names its own
    inputs. **Kept:** the lens *split* as a briefing parameter ("lens-briefed coverage / coherence in
    a pair") and P13's verify-pass owner.
  - The **probe mechanics** ("disposable probe subagents over throwaway stubs at the scoped paths,
    unconditional cleanup") and the **standalone regression re-run** after real scaffolds land. Home:
    `mochiko:testing-governance-injection` (its cleanup section + its "As a regression check after
    real scaffolds land" use case). **Kept:** setup's offer discipline — the rules-files trigger,
    optional-with-token-cost, findings feed an amend run, and the KM dogfood *gating* on it.
  - The **sizing gate's weight components** ("element count · mark mix · reality-surface load") and
    the **none→waiver-in-the-Review-section** landing. Home: `templates/sized-end-stage-review.md`,
    read at the sizing gate — the same stage the rule binds. **Kept:** the *elevation purpose* (weight
    informs the user's elevation, never the default), the tier-keyed + event-scaled keying, and the
    amend audit-completeness rule.
  - The **synthesis file's own rules** — GI-ID sequencing, the confidence-mark vocabulary, the Review
    section, and the delta-wise amend update ("untouched elements keep their IDs and marks"). Home:
    `templates/governance-intent-template.md`. **Kept:** the `GI-XXX` namespace (P10) and the marks as
    P11's carrier.
  - The **catalog's shelf model** ("shelves by declared type, cards filtered and parameterized by
    tier"). Home: `references/catalog/README.md` — "A project's **type**… selects which shelves are
    dealt; its **tier** filters and parameterizes the cards".
  - The **selection-vs-formulation contract** and the flagged-proposal mechanism. Home:
    `authoring-constitution` § *The synthesis contract (selection vs. formulation)*. **Kept:** the
    one-line statement that the synthesis *is* the producer's contract (D4/D7 are DECISIONS-traceable,
    so the fact survives as a reference) plus G4's ruling binding.
  - The **memory model** ("carried in-session + by workspace evidence; there is no context file") and
    the **Recovery preamble's team-survival note**. Home: `command-shape.md` Recovery ("Resume from
    **workspace evidence**, never a context `phase` field"; "Sessions and teams do not survive
    `/resume`").
  - The **`Contested` no-fallback provenance pointer** ("the same `Contested` dogfood-pilot ruling as
    `/mochiko:brainstorm`'s"). Home: `command-shape.md` Layer 2 *Hard requirement*, which at v5 states
    the bet, its `Contested` mark **and** its revisit condition. This was a v0.11.0 *Kept* item; it is
    superseded, not dropped, because the home it pointed at now carries the content it pointed to.
  - **Design-record provenance** ("Design record: `…/constitution-native-surfaces/record.md`
    (D1–D8)"). Home: this note + `DECISIONS.md`. Same disposition as the pilot's G7-provenance
    relocation: history belongs in a file that is not paid for on every run.
- **Stale pointer FIXED (not a strip):** the command pointed at "`templates/agent-dispatch.md` (Seat
  transport)" for transport mechanics + the addressability check. That content **moved to
  `command-shape.md` Layer 2 at v5**; `agent-dispatch.md` explicitly says so. The preamble now names
  the shape's both-layers read for transport and `agent-dispatch.md` for briefing only.

## [v0.35.0] CS-D8 survivor re-grade ledger — every protected line resolved

CS-D8 (extended by user ruling U4) protects two sets: `KEPT:`/*Kept deliberately* lines **and** every
line traceable to a `DECISIONS.md` row. Both were grepped and enumerated before any cut. **40 protected
lines: 37 translated (3 of those with one clause superseded), 3 superseded whole, 0 dropped.** Every
supersession names a home that was Read this run.

| protected line | source | resolved |
|---|---|---|
| Intent reviewers' no-contact clause — "they message you, never the user, and never contact the producer or validator"; a *cold review stage*, not in-loop traffic, so the mesh does not reach it | v0.31.0 *Kept deliberately* | Intent-reviewer row, peer-edge cell, in substance verbatim |
| **No devolved branch** — setup's validation is a Tier-2 judgment grade, never all-deterministic-CLI, so no gate is skipped | v0.31.0 *Added* | The validator-check-surface invariant's closing clause, with the Tier-2-with-deterministic-sub-checks standing that grounds it |
| The **producer→validator outbound leg stays lead-routed** — the check surface is mode-parameterized every round, so selecting it is a policy call | v0.31.0 *Kept deliberately, second (audit round)* | The validator-check-surface invariant (both parameterizations named) + the validator row's "the send opening a round is yours" |
| The env var, the **mode-keyed probe-seat parameter** | v0.11.0 *command keeps* | Preamble: hard-require line + the three-way probe-seat parameter |
| The `Contested` no-fallback provenance pointer | v0.11.0 *command keeps* | **Superseded** → `command-shape.md` v5 Layer 2 carries the bet + mark + revisit condition (see the strip above) |
| Weight-statement inputs **+ elevation purpose** | v0.11.0 *command keeps* | Inputs **superseded** → `sized-end-stage-review.md` (verbatim); the elevation purpose **kept** on the sizing gate line |
| Tier-keyed **+ event-scaled** sizing keying | v0.11.0 *command keeps*; adversarial D3/D6 | Review sizing, both keyings + "the gate always opens… audit-complete" |
| The **user-territory** and **user-declared-facts** routing buckets (S8, S13) | v0.11.0 *command keeps*; adversarial D7.4 | Survivor rulings: deck/tier/waiver rulings, challenged dimension-prunes and scope decisions, and the three named user-declared facts *as confirmation* |
| The **reality-surface fact route** | v0.11.0 *command keeps* | Bindings' fact route (`codebase-analysis.md`, else a native `Explore` pass) + "never argument" on the gate line |
| The **verify-pass owner** | v0.11.0 *command keeps*; adversarial D7.5 | Bindings: coherence-lens reviewer; the sole reviewer when sized down |
| The **tier-bump re-deal** rule | v0.11.0 *command keeps* | **Translated and relocated to where it fires** — the Interrogation gate's amend clause (it must act during the amend interrogation, before the template is read at assembly). Its rationale tail ("a stale `Confident` never carries a lower-tier loosening upward unexamined") **superseded** → `governance-intent-template.md`, which states it verbatim |
| Setup-specific **pause location** + the evidence table | v0.11.0 *command keeps* | Recovery: the `governance-intent.md`-or-region-stamp line + all 13 rows |
| The footer's **`agent-dispatch` briefing pointer** | v0.13.0 *command keeps* | Preamble ("brief every dispatch per `templates/agent-dispatch.md`"); the footer itself deleted |
| D1 — complete dissolution; **no `constitution.md`**; CLAUDE.md is the thin ratified core | DECISIONS row (native surfaces) | Preamble's "There is **no `constitution.md`**" + Bindings' surface set |
| D2 / D3 — governance layer splits by audience; content→surface mapping | DECISIONS rows | **Superseded** → `authoring-constitution`'s disclosure-tier table (composition + read scope), named in Bindings |
| D4 — the setup loop survives with a distributed target; **trace is the spine** | DECISIONS row | Goal (PASS graded from the files; G4 with the trace summary) + Bindings' trace summary + the synthesis-is-the-contract invariant |
| D5 — downstream briefs: native loading + a one-line governance pointer | DECISIONS row | **Superseded** → `agent-dispatch.md` field 3 (the governance obligated-read line naming `.claude/rules/mochiko/` files) + `authoring-constitution`'s disclosure table ("On matching-file reads (plus the dispatch-brief obligated read for authoring producers)"). **The wave's second contestable call:** the claim produces no setup behavior — it is a rule on *downstream* commands' briefs — and both halves are homed in files setup already references |
| D6 — no backward compatibility, no migration machinery | DECISIONS row | Workspace + hygiene invariant (**delete it on sight**, no migration, no offer, say so in one line) + Out of scope |
| D8 — setup owns a **marked** governance region; the trace summary is the validator's manifest | DECISIONS row | Ownership boundary invariant (markers, idempotent regeneration, outside-is-never-touched, same rule for rules/ledger/synthesis, the KM never-overwrite floor) + the validator row |
| Setup-v2 D1 — the floor survives, scope-tiered | DECISIONS row | G4's floor accounting incl. waivers + the Goal's not-done "a floor category with neither principle nor waiver" |
| Setup-v2 D2 — explicit waiver at low tiers, recorded with a revisit trigger | DECISIONS row | Interrogation ("Waivers ruled where the tier permits") + Out of scope (revisit triggers fire on re-invocation only) |
| Setup-v2 D3 / D4 / D5 — catalog-seeded, generation-open, interrogation-led; the agenda worked **adaptively**; all three modes, depth-proportional, with low-tier pruning | DECISIONS rows | The Interrogation gate: agenda path + adaptive working + minted intents + the catalog deck + the pruning license with **every skip named, never silent**; G1 carries the mode's stage selection |
| Setup-v2 D6 — tier axis as preset with elicited overrides | DECISIONS row | The tier-keyed sizing default + tier-parameterized cards (referenced to the catalog) — **retained verbatim pending PO-D2, see above** |
| Setup-v2 D7 — the synthesis is a **traceable contract**, selection vs. formulation | DECISIONS row | The synthesis-is-the-producer's-contract invariant (split + flagged-proposal route referenced to `authoring-constitution`) + G4's proposal rulings |
| Setup-v2 D8 — universal core + attachable modules | DECISIONS row | Interrogation's **once-only** module offer with the recorded-either-way answer + the KM-landing binding |
| Adversarial D1 / D2 — the review's target surface is the synthesis at G3; substrate is the confidence-marked synthesis | DECISIONS row | Validation model (first branch, before G3) + Bindings' uncertainty carrier |
| Adversarial D4 / D5 — a dedicated skill; the `validation-*` vs `review-*` family split (lead-adjudicated input, never the authoritative grade) | DECISIONS row | Intent-reviewer row: `review-governance-intent`, "input you adjudicate, not the grade"; the Validation-model line names which branch is authoritative |
| Adversarial D7.6 — the **G3-edit rule**: a material post-review change gets a bounded delta-pass from the still-seated reviewer; `reject` re-enters the full flow | DECISIONS row | G3's edit and reject branches, both explicit; the delta-pass also appears as a Bounds item |
| Adversarial D7.7 — the synthesis gains a **Review section**; recovery keys off its state | DECISIONS row | Bindings (the template owns the section) + three Recovery rows keyed to its state |
| Adversarial D7.8 — the propagation set: done-condition, review caps, human gates, recovery rows | DECISIONS row | Goal's done-condition; Bounds' four review caps **kept enumerated** (P8 states the bounds of every loop in the file, and the review is one); the sizing + survivor-ruling gate lines; the Review-section recovery rows |
| Team-transport legibility — `agent-dispatch` briefing + the addressability probe; no-fallback stands | DECISIONS row | Preamble's dispatch-brief line + probe seat; the mechanics ride the shape's Layer-2 read (stale pointer fixed above) |
| Setup-v3 — the brainstorm team idiom (named standing seats, messaged not respawned) | DECISIONS row | The Seats table's spawn column (standing / cold / messaged-after) |
| Domain-dependency allowlist D1–D5 | DECISIONS row | The Interrogation's **layered-architecture beat**: fires on a layered principle kept **or** minted, module ruling + domain-dependency seed arbitration against `DOMAIN-DEPENDENCIES.md`. The add-gate, signal hierarchy and rules-file-as-registry are the reference's, never setup's |
| Governance-injection probe suite | DECISIONS row | G5's probe offer (trigger, optionality + token cost, findings→amend, KM gating); mechanics superseded to the skill |
| OD-D6 / D7 / D9 / D10 + KM module v1's collision rules | DECISIONS rows | The KM-landing binding (scaffold per the module template · the **project-pinned copy** at `.mochiko/memory/knowledge-management.md` resolved at runtime · amend-with-module = setup's own KM landing under fix-on-sight) + the never-overwrite floor in the Ownership invariant. Adoption granularity, the enforcement-surface list and the collision rulings are the module template's |
| The **`>5` source files + framework** brownfield threshold and the region-present amend suggestion | current body | G1's decides clause, thresholds intact |
| **`detect-stack.sh` is an input, never the quality gate**; G2 has no machine validator | current body | G1's evidence clause + G2's stated absence with its reason |
| The **escalation menu** — last fix list + stop reason; give-guidance-and-retry / accept-with-noted-gaps / abort; the run stays FAIL unless the user explicitly accepts | current body (authoring loop) | The **Escalation** gate line, in full. Flagged because the concurrent draft dropped it outright — the compressed-evidence-clause failure class the pilot warned about, caught by this wave's grep rather than by an auditor |
| The **in-loop clarification** sub-gate — ask the user, feed answers forward, never the done-condition | current body + Contract human gates | The **Clarification** gate line. Also dropped by the concurrent draft; also recovered by the grep |
| Amend: the producer works from the **current surfaces**, preserves untouched principles, **bumps the region's semver** | current body (producer seat) | The synthesis-is-the-contract invariant's amend clause. Third recovery from the same grep |

## [v0.31.0] Fix-list routing no longer carries independence (shape v4 conforming edit)
- **Disposition:** superseded → `templates/command-shape.md` v4 (Layer 2 — "Independence by structure" + "In-loop mesh"). Rewritten in place: the validator is still cold-spawned at first validation, the producer↔validator peer edge is declared on the roster, and independence is restated as structural (disjoint agents, disjoint skills, cold spawn).
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/team-method-vs-command-shape/record.md` **D1**, scoped by **D2**), not a minimalism strip. This command carried the library's most explicit statement of the falsified claim.
- **Content (superseded, verbatim):**
  - validator seat: "**The fix list flows through you** to the producer — that routing is the independence the loop rests on."
  - validator seat: "spawned **cold at first validation**, never in the room before that and never in contact with the producer"
  - Authoring loop: "on round > 1 message it the validator's fix list verbatim for targeted revision"
  - Contract, Producer ↔ validator: "(validator cold-spawned, fix list lead-routed, no producer↔validator contact)"
- **Kept deliberately (not superseded):** the pre-G3 **intent reviewers'** no-contact clause ("they message you, never the user, and never contact the producer or validator"). Per **D2** that is a *cold review stage*, not in-loop traffic, so the mesh does not reach it — the command now says so explicitly rather than leaving it to look like an oversight.
- **Added (not a strip):** a Contract human-gates declaration that setup has **no devolved branch** — its validation is a Tier-2 judgment grade, never all-deterministic-CLI, so D3's devolution cannot apply and every verdict stays the lead's.
- **Kept deliberately, second (audit round, 2026-07-30):** the **producer→validator outbound leg stays lead-routed**. The auditor flagged the silence as the defect and left the resolution to recorded grounds; the grounds are that setup's validator has no fixed check surface — it is mode-parameterized every round (brownfield adds the tools/versions↔analysis cross-check; an attached knowledge-management module adds the repo-level invariant re-audit), so selecting it is a **policy call**, which the shape's traffic classes route to the lead, not a hand-off. This is the same reasoning the wave lead applied to plan's Phase-2 mode-selecting message. Now stated in the command rather than left silent.

## [v0.13.0] Footer ground rules (kernel-free · git)
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, Ground rules — homed at shape v2, the specify wave's S8 home revision)
- **Tier failed:** 1 (altitude — verbatim in five command footers; true of every conformant command)
- **Content:** "Stay kernel-free; … do not modify git or push." (the footer keeps its `agent-dispatch` briefing pointer)

## [v0.11.0] Hard-requirement transport mechanics
- **Disposition:** relocated → `templates/command-shape.md` (Layer 2, Hard requirement) + `templates/agent-dispatch.md` (Seat transport — already the mechanics' single source; the command's restatement removed)
- **Tier failed:** 1 (altitude — restated shape/transport prose)
- **Content:** the env-check-as-proxy prose, the no-fallback rationale body, "Running the loop on one-shot subagents is not a fallback — it is the defect this section exists to forbid," the `name:`-discriminator sentence ("a spawn without a `name:` is a one-shot subagent, the forbidden form"), and the post-spawn addressability instruction. The command keeps: the env var, its mode-keyed probe-seat parameter, and the `Contested` provenance pointer.

## [v0.11.0] Never-narrate-machinery + housekeeping constraint
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, production surface; Layer 2, seat legibility)
- **Tier failed:** 1
- **Content:** "The conversation is the production surface, and it belongs to you and the user. Never narrate machinery — no 'phase', 'round', or 'gate' talk; teammate housekeeping (idle notifications, acks) is never narrated and never replied to."

## [v0.11.0] Watch/message note + seat-announcement rule
- **Disposition:** relocated → `templates/command-shape.md` (Layer 2, seat legibility)
- **Tier failed:** 1
- **Content:** "Tell the user at the start that they can watch or message any teammate directly. Announce each seat in one line when you fill it — an unexplained teammate spawn reads as a malfunction."

## [v0.11.0] Team preamble — teammates don't load `skills:` frontmatter + message/standing-seat semantics
- **Disposition:** relocated → `templates/command-shape.md` (Layer 2, seats)
- **Tier failed:** 1
- **Content:** "Teammates do not load `skills:` frontmatter — every spawn prompt must name the skill and role itself, plus what to Read … A teammate's plain text is not visible to you: its reports arrive as messages, and your follow-ups go to the same named seat via message — that continuity is what the standing seat buys. A fresh spawn per round is the subagent anti-pattern, not a team." Seat-block echoes of the same semantics ("spawn it once, then message the same seat"; "round > 1 is a message to the same validator seat") trimmed with it.

## [v0.11.0] Synthesis-review generics (sizing-gate mechanics · freeze · withheld counterparts/cross-exam pointer · two-exchange cap · `Contested`-on-overrule · disposition landing)
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, sized end-stage review); the cross-exam protocol pointer stays single-sourced at `review-brainstorm/references/CROSS-EXAM.md` (referenced from the shape home, not the command)
- **Tier failed:** 1
- **Content:** "Open the sizing gate: state the synthesis's weight … The user rules pair / single / none"; "the synthesis is frozen from spawn until dispositions land"; "Withhold each reviewer's counterpart from its spawn prompt: findings form cold, then you introduce them for the one-shot four-message cross-exam (the single-sourced protocol in review-brainstorm's references/CROSS-EXAM.md)"; "your formulation → argue with the finding's owner, two exchanges max (you count), then tie-break"; "An overruled survivor marks its element `Contested`"; "Land every disposition in the Review section, then". The command keeps its bindings: weight-statement inputs + elevation purpose, tier-keyed + event-scaled keying, the user-territory and user-declared-facts routing buckets, the reality-surface fact route, the verify-pass owner, and the tier-bump re-deal rule.

## [v0.11.0] "No per-run contract is written" clause
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, contract section)
- **Tier failed:** 1
- **Content:** "— no per-run contract is written" (the authoring-time-fill rationale now lives with the shape's Contract clause).

## [v0.11.0] Recovery preamble
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, recovery)
- **Tier failed:** 1
- **Content:** "Teams do not survive `/resume`, and a shared account limit can throttle the team and the main session together — pause posture: …" (the setup-specific pause location and the evidence table stay).
