# Strip notes — `templates/governance-intent-template.md`

Entry formats: `strips/README.md`. First entry created at **v0.43.0** — this primitive had no strip
note before the shape-v7 conversion wave, so there is no prior-wave history here and no protected
set recorded in this file; the protections that govern its content are the `DECISIONS.md` rows
traced in the entry below and the *Kept deliberately* fields of `.mochiko/strips/setup.md`, its only
command consumer.

**No version footer.** Unlike `command-shape.md` and `sized-end-stage-review.md`, this template
carries no version line (checked at this edit), so no footer stamp was owed and none was invented.

## [v0.76.0] Template retired — superseded by schema-based template guidance (D1/D3/D8)
- **Disposition:** superseded → plugins/mochiko/schemas/governance-intent.yaml + mochiko-cli template governance-intent
- **Tier failed:** n/a — supersession by ruling (schema-based-template-guidance D1/D3/D8; record `.mochiko/brainstorms/schema-based-template-guidance/record.md`; `DECISIONS.md` "Template-schema CLI ruled")
- **Content (superseded template, full verbatim below):**

````markdown
<!--
GOVERNANCE INTENT — the session synthesis (fill target)
========================================================
Written by the /mochiko:setup lead at the close of the interrogation session and ratified at the
synthesis-confirmation checkpoint BEFORE any authoring. Lives durably at
`.mochiko/memory/governance-intent.md`, beside the governance ledger of the surface set it
governs (there is no constitution.md — governance lands on native surfaces).

This artifact is a TRACEABLE CONTRACT on the surface-set producer, not a brief:
- Every principle in the authored surface set MUST trace to exactly one element here
  (its GI-ID), and every principle-bearing element here MUST be realized on a surface
  or surfaced as a flagged proposal at the acceptance gate.
- Producer latitude is confined to FORMULATION (wording, enforcement mechanics, three-part
  structure with real commands) — never SELECTION.
- The validator string-matches trace-IDs both ways (deterministic). Semantic fidelity of a
  stamped trace is judgment-grade residual risk, mitigated by the synthesis-confirmation
  checkpoint and the acceptance gate's trace summary.

AMEND RUNS UPDATE THIS FILE DELTA-WISE: touched elements are edited or superseded (never
renumbered), new elements take fresh IDs, untouched elements keep their IDs, and the Amendment
Log records each delta. The traceability cross-check always runs against this persisted,
updated file. LEGACY MIGRATION: an amend run meeting a tier declaration (the retired axis)
supersedes it — never renumbered — elicits the fact profile once, and re-records existing
waivers under the D4 model. An existing single-floor synthesis that carries NO depth-level
declaration defaults to `high` on its next amend (adaptive-depth 2026-08-11, review fold #7) —
recorded as the declared level without a flip ceremony, since it never operated at `low`.

GI-ID rule: sequential GI-001, GI-002, … — unique forever within this file; never reuse a
retired ID.

CONFIDENCE MARKS: every GI element carries a mark from
`Confident / Assumed / Contested / Unsure / Deferred`, assigned by the lead at assembly from the
session's own indicators — updated DELTA-WISE on amend (untouched elements keep their marks).
The marks are the G3 intent review's prioritization substrate; they are lead-self-reported, so
the review audits them rather than trusting them. `Contested` = the user overruled a recorded
challenge with the steelman in view — the recorded basis is required, or the mark is a defect.

THE REVIEW SECTION is this file's durable record of the sized pre-G3 review (or its waiver) —
recovery keys off its state. The reviewers produce lead-adjudicated input; the Tier-2 validator
downstream issues the authoritative grade on the authored surface set (both still human-gated).
-->

# Governance Intent — [PROJECT_NAME]

**Session date:** [YYYY-MM-DD] · **Mode:** [greenfield | brownfield | amend]
**Confirmed at synthesis checkpoint:** [YYYY-MM-DD] by [user]
**Governs:** the governance surface set v[X.Y.Z] (CLAUDE.md governance region · `.claude/rules/mochiko/` · `.mochiko/memory/governance-ledger.md`)

## Fact profile

The module-driving facts (interrogation dimension 2), each with its consequence-stated
confirmation — negatives are recorded facts too (the S4 fail-safe, per
`authoring-constitution/references/COMPLIANCE-MODULES.md`):

- **GI-001 — Facts:** industry: [x] · data classes: [x] · jurisdictions/markets: [x] ·
  contractual commitments: [x] · **Mark:** [Confident | Assumed | Contested | Unsure | Deferred]
- **Modules triggered (mechanical):** [module list with trigger fact each, or "none — negatives
  confirmed: (each negative + its stated consequence)"]
- **Brownfield cross-check:** [declared facts vs `codebase-analysis.md` / DS-XXX / detected
  integrations — "consistent" or the confrontation's ruling] *(brownfield only)*

## Project identity & type

- **GI-002 — Type:** [frontend | backend | fullstack | service | mobile | desktop] → shelves
  dealt: [list] · **Mark:** [Confident | Assumed | Contested | Unsure | Deferred]
- **Identity:** [what's being built, for whom, lifespan — 2-3 sentences]
- **Risk surface:** [what failure costs — honest context for the floor's expression]
- **Team reality:** [solo/team, review culture — what enforcement can lean on]

## Depth level declaration

The floor's single depth dial (adaptive-depth, 2026-08-11) — **one project-wide declaration, not
a per-check negotiation**. Breadth is invariant: every floor category is present at both levels;
the level tunes only how strictly the asserted *code* standards are authored (process rigor is
uniform — D8), and compliance modules are level-blind (full strength at either level — D7). The
level is the user's declaration alone — never fact-derived (D1); setup recommends, the user rules.

- **GI-0XX — Declared level:** [low | high] · **Declared:** [YYYY-MM-DD] by [user] ·
  **Mark:** [Confident | Assumed | Contested | Unsure | Deferred]
- **Rationale:** [why this level — setup recommends `low` for greenfield, `high` where reality
  warrants it; the user's ruling, recorded]
- **Ratchet:** one-way — `high` is terminal; a later `low`→`high` move happens only through a
  flip ceremony (a conscious `/mochiko:setup` rerun in high mode), never silently, never reversed.

## Convergence skips

[Named skips only — a dimension left unasked because an earlier answer settled it ("dimension 5
settled by dimension 1: solo founder"). A skip is convergence bookkeeping, never a scope ruling
— there is no pruning license. Or "none".]

## Real commands (dimension 6/8 → the validator's placeholder bar)

| Purpose | Command | Source |
|---------|---------|--------|
| Lint | `[actual command]` | [detected / declared] |
| Test | `[actual command]` | [detected / declared] |
| [etc.] | | |

## Floor expression & deck rulings

Floor cards enter asserted — their rows record *expression* (type translation), never a level
ruling; arbitrated cards (architecture-opinion and kin) record the user's ruling. **Dropped
arbitrated cards are rulings too:**

| GI-ID | Card | Layer | Ruling / Expression | Mark |
|-------|------|-------|---------------------|------|
| GI-0XX | FLOOR-SEC | floor-asserted | [expression shaped how, or "at floor level"] | [Confident] |
| GI-0XX | BE-HEX | arbitrated | [kept/tightened/dropped + user's stated reason] | [Confident] |
| … | | | | |

*(On a fact-profile or un-waive amend, affected rulings are re-dealt on the event's agenda
slice or force-re-marked — a stale `Confident` never carries an unexamined ruling forward.)*

## Minted principle intents

One element per minted intent — traced to the elicited answer, never to prompting:

- **GI-0XX — [Intent name]:** [what to enforce, how strictly — the user's words distilled]
  · **Mark:** [Confident | Assumed | Contested | Unsure | Deferred]
  *Elicited from:* [dimension 9 answer, quoted or closely paraphrased]

## Waivers

Any asserted standard may be waived with a recorded justification (D4) — **except legal-mandate
module obligations (D4.2)**. Waivers are permanent pending the D4.1 revisit; a revisit trigger
is the user's option, never a default.

**Interim transition delta (D6).** After a `low`→`high` flip, a category the project does not yet
meet at the high level rides a waiver as a **bounded transition exception** — transition-scoped,
naming its delta item, and dying when that item conforms. It is a staged-adoption record, never a
depth valve: it does not lower the declared level, and the level stays `high`.

Every waiver is auditable:

| GI-ID | Standard (floor category / card / non-legal module obligation) | Justification | Revisit trigger (optional) | Mark |
|-------|---------------------------------------------------------------|---------------|---------------------------|------|
| GI-0XX | [e.g. FLOOR-TEST coverage gate] | [recorded reason] | [or "permanent (D4.1 pending)"] | [Confident] |
| GI-0XX | [e.g. FLOOR-OBS SLO formalization] | interim transition delta (high declared [date]) | transition — dies when the item conforms | [Confident] |

## Module selections

**Template modules** (the operating-docs layer — knowledge-management, layer-rules,
release-gates), ruled in session; *compliance modules attach mechanically in the Fact profile
above, never here.* **Declines are rulings too** (recorded, durable: amend runs offer only
modules with *no* recorded ruling here; a recorded decline is never re-asked until the user
reopens it). The validator checks core + exactly the adopted modules:

| GI-ID | Module | Ruling | Because | Mark |
|-------|--------|--------|---------|------|
| GI-0XX | [layer-rules] | adopted | [BE-HEX kept \| layered intent minted] | [Confident] |
| GI-0XX | [knowledge-management] | [adopted \| declined] | [offered default-on at dimension 7; a decline also memorializes in dimension 10] | [Confident] |
| … | | | | |

## Domain-dependency seeds (only when `layer-rules` is adopted)

Session-arbitrated registry seeds per `authoring-constitution/references/DOMAIN-DEPENDENCIES.md`
— domain-relevance filtered, trust-ranked, each level-cited; rejected seeds are rulings too:

| GI-ID | Dependency | Signal level | Ruling | Mark |
|-------|------------|--------------|--------|------|
| GI-0XX | [equatable] | [1 — Flutter Favorites, live-verified] | [kept \| dropped] | [Confident] |
| … | | | | |

## Deliberate exclusions (dimension 10)

- **GI-0XX:** [what governance will not cover + why — never a floor category (the floor leaves
  only by recorded waiver, above)]
  · **Mark:** [Confident | Assumed | Contested | Unsure | Deferred]

## Review

<!-- The durable record of the sized pre-G3 intent review — one block per run (amend runs append
a new block, event-scaled). Recovery keys off this section's state: no sizing ruling → open the
gate; sized but survivors undispositioned → respawn/continue the review; folded but unverified →
verify pass; waived or verified → G3. -->

**[YYYY-MM-DD] — [first ratification | amend]**

- **Sizing:** lead stated weight [element count · mark mix · reality-surface load]; the default
  [pair on first ratification | event-scaled on amend: governance event → pair · lighter
  substantive delta → single · wording-level → none-with-recorded-waiver]; **lead sized:**
  [pair | single | none], the sizing composed by the lead in its run plan — below the default
  also takes a departure-trail line.
- *(if none)* **Waiver:** this record is lead-penned, so `none` is not the lead's to take —
  waived by [user] as an explicit reserved ruling, [stated reason]. The synthesis proceeds to
  G3 unreviewed; this record is the audit trail.
- *(otherwise)* **Review:** reviewer(s) [coverage / coherence lenses | solo]; **tally** [N raised
  → M merged survivors]; recommended status [ready | needs-revision | critical-gaps]
- **Survivor dispositions** (every survivor carries one):

  | # | Sev | GI element(s) | Finding | Disposition |
  |---|-----|---------------|---------|-------------|
  | S1 | [Critical] | [GI-0XX] | [one line] | [resolved / user-ruled / recorded-open — overruled → element marked `Contested`] |

- **Verify pass:** [PASS — folds confirmed by the coherence-lens/sole reviewer | pending]
- *(if any)* **G3-edit delta-pass:** [edited elements + the still-seated reviewer's outcome]

## Amendment Log

[Empty on first ratification. Amend runs append one entry per delta:]
- [YYYY-MM-DD] — [GI-IDs added/edited/superseded] — [one-line rationale] — re-confirmed at
  synthesis checkpoint [date]
````
- **Kept deliberately:** Doctrine-dense, multi-shape canonical reference — every operative line is protected / `DECISIONS.md`-traceable governance doctrine. All of it was carried **verbatim** into `plugins/mochiko/schemas/governance-intent.yaml` (shape-blocks preserved over uniform per-section fields, per plan §3 I3) and renders through `mochiko-cli template governance-intent`; the `.yaml` ships in the plugin as the raw-Read first-class degraded path (D8, GI-020, no install regression). Net-new per-section `check` lines were authored under D7 (disclosed, not lifted). V2 confirmed **no doctrine dropped** — nothing removed.
- **Consumers assessed:** `commands/setup.md` (re-pointed by P4) · `skills/authoring-constitution/references/INTERROGATION-AGENDA.md` (re-pointed by P5). V2 fidelity PASS 2026-08-16 (schema graded 8/8 at the M3 gate).

## [v0.53.0] Review-block sizing/waiver re-keyed off the deleted weight card
- **Disposition:** superseded → the same two Review-section lines re-keyed to the v8 reality: sizing "composed by the lead in its run plan"; the `none` waiver "waived by [user] as an explicit reserved ruling".
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-02 command-architecture-realignment D2 — the weight card was deleted at the v8 rebuild, v0.48.0; this template's lines 157/160 were the stale remainder, logged as a BACKLOG defect at the 2026-08-04 groom and closed in the v0.53.0 wave).
- **Content (verbatim, the two superseded fragments):**
  ```
  stated on the user's run-start weight card — below the default also
  takes a departure-trail line.
  ```
  ```
  waived by [user] at the **weight card**, [stated reason].
  ```
- **Kept deliberately:** the sizing defaults (pair on first ratification · event-scaled on amend), the departure-trail obligation below the default, the `none`-is-not-the-lead's-to-take rule, and the audit-trail sentence — all verbatim.
- **Consumers assessed:** grep-verified across `plugins/` this run (`grep -rln "governance-intent-template" plugins/` → two literal consumers): setup (binds the template; its harness already reserves waivers to the user — now consistent) · `authoring-constitution/references/INTERROGATION-AGENDA.md` (re-checked, unaffected — its pointer text "see `templates/governance-intent-template.md`" doesn't restate the Review section or the sizing/waiver lines). Non-literal consumer also assessed: review-governance-intent (reads the Review block's state for recovery; state vocabulary unchanged).

---

**Wave context (v0.44.0 — the D7 leakage scrub).** `verbosity-caveman-ops-separation` D7 as
folded at review (S4): **full scrub** of ops leakage from the shipped tree, with no
changelog-worthy detail lost — every removed block is preserved verbatim below. Ruling:
`DECISIONS.md` 2026-08-01 "Output verbosity, caveman & ops separation ruled" row.

**The leak test this wave used, recorded so a future sweep inherits it: *whose artifact does the
pointer name?*** Mochiko's own ops records — `.mochiko/strips/`, `.mochiko/brainstorms/`,
`.mochiko/decisions/`, `.mochiko/archive/` — are leaks: they resolve to nothing in an installed
plugin. Adopter runtime paths (`.mochiko/specs/`, `.mochiko/memory/`) and the KM module's
document contracts are the **user's** artifacts and are untouchable. A prefix-based sweep on
`.mochiko/` would gut the KM module and the brainstorm command; 101 of this tree's 146
`.mochiko/` references were correctly left alone on that test.

## [v0.44.0] Confidence-marks session-slug citation
- **Disposition:** superseded → deleted from the shipped file; preserved verbatim here.
- **Tier failed:** n/a — supersession by ruling (`verbosity-caveman-ops-separation` D7 + S4; the `DECISIONS.md` 2026-08-01 row above)
- **Content (verbatim):**
```
(setup-adversarial-review D2)
```
- **Kept deliberately:** the confidence-mark machinery whole — the five-value vocabulary, lead-assigned at assembly from the session's own indicators, updated delta-wise on amend.

# v0.43.0 — the v6→v7 conversion wave

**Wave context:** the shape-v7 conversion wave (`lead-owned-process-flexibility`,
`.mochiko/brainstorms/lead-owned-process-flexibility/record.md`; `DECISIONS.md` 2026-08-01), all six
commands converting after the user widened the wave on 2026-08-01. This template is not a command
and did not convert; it is edited here because **U4**'s sizing-ownership flip, executed in
`commands/setup.md` at its conversion touch, left this scaffold printing the superseded ruler.
Surfaced by the setup conversion and routed to a scope extension by the wave lead.

## [v0.43.0] The Review-section sizing scaffold flips user → lead (U4), and its waiver moves to the weight card (U1-B)

- **Disposition:** superseded → rewritten in place. The Review section, its recovery keying, and
  every fill slot survive; the *ruler* named by the sizing sub-line and the *gate* named by the
  waiver sub-line changed.
- **Tier failed:** n/a — supersession by ruling (**U4**, 2026-08-01: *"review sizing passes to the
  lead by recorded supersession of the brainstorm-v2-2 ruling"*, ratified with the set at **A4**;
  and **U1-B**: *"a lead-penned deliverable ships with zero cold reads only by recorded user waiver
  at the weight card"*). Doctrine home already flipped:
  `templates/sized-end-stage-review.md` **v2** — *"The sizing gate is the lead's"* and *"On a
  **lead-penned** artifact `none` is not the lead's to take"*. This edit brings the scaffold into
  line with the home it serves.
- **Content (verbatim, the two bullets that were rewritten):**
  ```
  - **Sizing:** lead stated weight [element count · mark mix · reality-surface load]; recommended
    [pair | single | none] ([pair default on first ratification | event-scaled on amend:
    governance event → pair · lighter substantive delta → single · wording-level →
    none-with-recorded-waiver]); **user ruled:** [pair | single | none]
  - *(if none)* **Waiver:** waived by [user] at the sizing gate — [stated reason]. The synthesis
    proceeds to G3 unreviewed; this record is the audit trail.
  ```
- **Replaced by (verbatim):**
  ```
  - **Sizing:** lead stated weight [element count · mark mix · reality-surface load]; the default
    [pair on first ratification | event-scaled on amend: governance event → pair · lighter
    substantive delta → single · wording-level → none-with-recorded-waiver]; **lead sized:**
    [pair | single | none], stated on the user's run-start weight card — below the default also
    takes a departure-trail line.
  - *(if none)* **Waiver:** this record is lead-penned, so `none` is not the lead's to take —
    waived by [user] at the **weight card**, [stated reason]. The synthesis proceeds to G3
    unreviewed; this record is the audit trail.
  ```
- **The waiver half is a correction, not a rename — and it is why this residue was worth fixing
  rather than tolerating.** The old line let the waiver be recorded *"by [user] at the sizing
  gate"*. After U4 the lead owns that gate, so the scaffold as written would have invited a lead to
  size to `none` and record the waiver at a gate it now rules — the lead waiving its own cold
  grade. That is exactly the meta-self-grading hole **R3** identified and **U1-B** closes. Moving
  the waiver to the **weight card** (the user-ruled stop, U1-A) restores the separation, and states
  the lead-penned ground so the constraint reads as ruled rather than arbitrary.
- **Kept deliberately:**
  - **The Review section itself, whole, and its recovery keying** — `DECISIONS.md`-traceable to
    adversarial **D7.7** (*"the synthesis gains a Review section; recovery keys off its state"*).
    A sizing ruling is still recorded, only by a different ruler, so the section's own comment
    (*"no sizing ruling → open the gate"*) and setup's three Review-section Recovery rows still key
    exactly as before — **verified against `commands/setup.md` this run** (rows at *"Review section
    empty (no sizing ruling)"*, *"sizing ruled, survivors undispositioned"*, *"Review section
    verified or waived"*).
  - **The default keying, verbatim in content** — pair on first ratification; event-scaled on
    amend with all three limbs (governance event → pair · lighter substantive delta → single ·
    wording-level → none-with-recorded-waiver). U4 moved who rules the sizing, never what the
    default is.
  - **The weight-statement inputs** — element count · mark mix · reality-surface load, unchanged.
  - **The waiver's audit-trail purpose and both fill slots** — `[user]`, `[stated reason]`, and
    *"The synthesis proceeds to G3 unreviewed; this record is the audit trail"* verbatim.
  - **Every other slot in the block** — the Review line with its lens/tally/status fills, the
    survivor-disposition table, the verify pass, and the G3-edit delta-pass: untouched.
- **Added (not a supersession):** *"below the default also takes a departure-trail line"* — P20,
  making the flipped doctrine's departure obligation visible where the sizing is recorded.
- **Consumers assessed** — grep-verified across `plugins/` this run, four consumers, one
  disposition each:
  - **`plugins/mochiko/commands/setup.md`** — the **only command** consumer
    (`grep -rln 'governance-intent-template' plugins/mochiko/commands/` returns setup alone).
    Flipped in this same wave under `.mochiko/strips/setup.md` [v0.43.0]; this scaffold edit is the
    consumer-side completion of that flip. Recovery keying re-verified, above.
  - **`plugins/mochiko/skills/authoring-constitution/references/INTERROGATION-AGENDA.md`** — a
    second consumer in `plugins/`, found by the same grep and named here rather than left silent.
    It cites this template **only as a pointer** to where the synthesis output lives (*"Its output
    is the session synthesis at `.mochiko/memory/governance-intent.md` (see
    `templates/governance-intent-template.md`)"*); it restates neither the Review section nor the
    sizing line. **Unaffected, not edited.**
  - **`commands/brainstorm.md`** — **the wave lead's claim, verified before writing it and it
    holds.** brainstorm's `record.md` has **no equivalent scaffold**: there is no record template in
    `templates/` at all, and the only `user ruled` sizing scaffold anywhere in `plugins/` was the
    line superseded here. brainstorm's Review-section shape is described in the command body, and
    its own sizing construction is already flipped in this wave (*"Review sizing is the one
    lead-ruled gate here"*). Nothing to edit.
  - **`templates/sized-end-stage-review.md`** — the doctrine home, already flipped at **v2** in the
    v0.40.0 doctrine wave. Not edited; it is what this scaffold was brought into line with.
