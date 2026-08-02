# Strip notes — `templates/workflow-contract.md`

Entry formats: `strips/README.md`. This note **opens at v0.40.0**: the template shipped from the
original library and had never been edited under the strip discipline, so there is no earlier
history to carry. Wave note: `.mochiko/strips/command-shape.md` [v0.40.0].

---

## [v0.47.0] The whole template deleted — doctrine purge wave 2
- **Disposition:** superseded → each command's own Bindings: the run-start declaration line already carries a default run, and a departing run (or one declaring non-default bounds) now writes a plain **departure record** at the same path the command already names (`*-contract.md`) with the content each command states inline — done-condition + bounds as (re-)declared · departures taken · counter state — instead of instantiating a form. Non-command loops lose the form entirely; the four soundness requirements (default-FAIL done-condition · different-agent/different-skill validator · deterministic cap · named human gate) live in each command's Goal/Seats/Bounds and, library-wide, in `skills/mochiko/SKILL.md`'s loop framing.
- **Tier failed:** n/a — supersession by ruling (ADR `.mochiko/decisions/2026-08-02-doctrine-purge-wave-2.md`; DECISIONS.md 2026-08-02 "Doctrine purge wave 2" row). Supersedes the 2026-08-01 A2 revival ("OQ-2: `workflow-contract.md` revived for departing runs") in its *form* clause only — the obligation a departing run records its departure, bounds, and counter state survives at the same paths.
- **Content:** the full template at deletion, verbatim:

```
<!--
WORKFLOW CONTRACT TEMPLATE
==========================
WHEN TO FILL THIS IN — exactly two cases:
  1. A COMMAND RUN THAT DEPARTS from its command's stated default pipeline, or that
     declares non-default bounds. A default run needs no form: the command IS its
     contract — its own Goal, Seats & checks, Constraints and Recovery — and its values
     were constant at authoring time. What a departing lead composes instead is what
     genuinely varies per run, so that is what this form carries.
  2. ANY NON-COMMAND LOOP — an agent loop, a skill's own produce → check.
A default-running command declares in ONE LINE on its deliverable and fills nothing here.

The completed contract is the inspectable proof that the loop is sound. A reviewer reads
it to confirm: (a) the done-condition defaults to FAIL, (b) the validator is a DIFFERENT
agent + DIFFERENT skill than the producer, (c) iteration is deterministically bounded,
(d) a human gate is named.

INSTRUCTIONS:
- Replace every [PLACEHOLDER]. Do not leave brackets in a finalized contract.
- Delete the HTML comments before committing the filled copy alongside the workflow.
- If you cannot fill a field truthfully, the loop is not ready — fix the loop, not the contract.
- Store the filled copy with the run (e.g. .mochiko/<workflow>/contract.md), beside the
  deliverable its departure trail lands on.
-->

# Workflow Contract — [WORKFLOW_NAME]

**Workflow:** [WORKFLOW_NAME] · **Carrier:** [commands/<name>.md supervisor | agent team] · **Filled:** [YYYY-MM-DD]
**Why this form exists for this run:** [departed from the command's stated default | declared non-default bounds | non-command loop]

## 1. Done-condition (DEFAULTS TO FAIL)

<!-- The artifact starts FAILing and only flips on real evidence. State all three parts. -->

- **Measurable end state:** [the observable fact that means done — e.g. "the validator returns PASS and every required section is present"]
- **Stated check (how it is proven):** [named in advance — e.g. "the validator agent Reads the artifact and confirms every checklist item"]
- **Constraints (must not be violated):** [e.g. "kernel-free maintained; no acceptance criterion left unverified"]
- **Initial state:** `FAIL` <!-- always FAIL until proven otherwise -->

## 2. Producer ↔ Validator (independence on two axes)

| Role | Agent | Skill(s) | Notes |
|------|-------|----------|-------|
| **Producer** | [mochiko:<agent>] | [skill, skill] | emits the artifact under review |
| **Validator** | [mochiko:<DIFFERENT agent>] | [DIFFERENT skill] | grades from the artifact itself, never the producer's say-so |

- **Independence check:** producer agent ≠ validator agent **AND** producer skills ∩ validator skill = ∅. [confirm both]
- **Validator trustworthiness tier:** [1 deterministic ground truth | 2 separate-context grounded LLM | 3 LLM-judge] — [why this is the highest the artifact allows]
- **Tamper-proofing:** [how a PASS is gated on evidence actually Read — e.g. "no PASS unless the artifact file was Read this run"]

## 3. Bounded iteration

- **Hard round cap:** [N] produce↔validate rounds, counted by the supervisor.
- **No-progress exit:** [what counts as no progress — e.g. "a round where the validator's failing items are unchanged"]
- **Budget / kill-switch:** [token/cost ceiling or out-of-band halt]
- **Declared cost range:** [e.g. "≈200–350k tokens" — a declared range IS a bound, not an estimate]
- **Counter for each bound above:** [the lead — named, because a bound nobody counts is not a bound]
- **On hitting a guard:** escalate to the human gate with failure context. [never report done on cap exhaustion]

<!-- Bound integrity (command runs): a bound rises ONLY at a user checkpoint, and every
     re-declaration is recorded in §5 — the dispatching command's Bounds state the same rule. -->

## 4. Human gate

- **Placement:** [every cycle | low validator-confidence only | preference-gap only]
- **Where it fires:** [the phase/condition — e.g. "on low validator confidence, and on any cap-exhaustion escalation"]
- **What the human decides:** [e.g. "accept/override the validator's verdict; final acceptance"]

## 5. Composed process — departures, floor, and counter state

<!-- Command runs only; a non-command loop one-lines this section as "not applicable". -->

- **Departures from the stated default:** [one line each — what the default said, what ran instead, why. This is the same trail line the deliverable carries, not a second record.]
- **Floor gates honored:** [the command's named floor gates, each ruled by the user — a departure never eats one]
- **Counted unit this run:** [what the bounds and the seat-lifecycle cadence count — named, because a composed run has no default denominator]
- **Counter state (Recovery reads this on resume):** rounds consumed [N] · bounds declared [list] · departures taken [N] · re-declarations [when, and at which user checkpoint]

---

<!-- All five sections filled with no remaining brackets = the loop is sound. -->

**Contract version:** v3 (2026-08-02)
```

- **Kept deliberately:** the departure-record obligation and its per-command path (`.mochiko/specs/<feature>/<cmd>-contract.md`, `.mochiko/memory/setup-contract.md`, `.mochiko/brainstorms/<slug>/brainstorm-contract.md`) — same filename, plain record instead of a filled form; the four soundness requirements as library doctrine in `skills/mochiko/SKILL.md`; bound-integrity (rise-only-at-user-checkpoint, recorded re-declaration) already in every command's Bounds.
- **Consumers assessed:** all six commands (Bindings run-start-declaration lines re-worded from "instantiates `templates/workflow-contract.md`" to the plain departure record) · `agent-dispatch.md` footer "Pairs with" (dies with that file this same wave) · `skills/mochiko/SKILL.md` (framing paragraph re-worded, template row removed) · ARCHITECTURE.md templates row.


---
## [v0.46.0] loop-discipline/command-shape attributions out (v3)
- **Disposition:** superseded → self-contained (the four-requirement enumeration was already inline)
- **Tier failed:** n/a — supersession by ruling (ADR `.mochiko/decisions/2026-08-02-doctrine-purge-wave-1.md`; DECISIONS.md 2026-08-02 "Doctrine purge wave 1" row)
- **Content:** "the command IS its contract (templates/command-shape.md)" → "its own Goal, Seats & checks, Constraints and Recovery"; "satisfies the four requirements of the `loop-discipline` skill" → "is sound"; the §3 bound-integrity comment's "Home: command-shape.md Layer 1, The floor, invariant 3" → "the dispatching command's Bounds state the same rule"; "(P20)"/"P18 floor gates" slot refs reworded; the closing "sound per loop-discipline" comment and footer "**Governed by:** `loop-discipline`" dropped; v2 → v3.
- **Kept deliberately:** all five sections and the four-requirement reviewer checklist (a)–(d) — the doctrine content survives here inline.
- **Consumers assessed:** 6 commands (each still instantiates it for departing runs) · router row unchanged.

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

## [v0.44.0] Contract version-history block relocated (class 2, 464 B / 5 lines)
- **Disposition:** superseded → relocated **verbatim** into this note (below). In-file residue: the
  bare stamp plus the live routing (`Governed by`), which is wiring a run consumes, not history.
- **Tier failed:** n/a — supersession by ruling (`verbosity-caveman-ops-separation` D7 + S4; the `DECISIONS.md` 2026-08-01 row above)
- **Content (verbatim, the whole block as it stood at the scrub):**
```
**Contract version:** v2 (2026-08-01 — `lead-owned-process-flexibility` OQ-2, ruled at
acceptance A2: revived as the **per-run** carrier for a departing command run or a non-command
loop, a default run declaring in one line on its deliverable instead; §3 gains the declared cost
range and the named counter; §5 added for departures, floor gates, counted unit and counter
state; v1 — the fill-in form for every workflow) · **Governed by:** `loop-discipline`
```
- **Kept deliberately:** the version *number* and its date stay in the file — a consumer still
  learns which revision it is reading; only the per-revision narrative left.

# v0.40.0 — v1 → v2, the contract revived as a **per-run** carrier

**Wave context:** `lead-owned-process-flexibility`
(`.mochiko/brainstorms/lead-owned-process-flexibility/record.md`), **OQ-2** — its proposed
answer adopted **as written** at acceptance **A2**, 2026-08-01; raised as **R17** (D1 falsifies
the constant-at-authoring premise) and **R18** (a composed process living only in the lead's
context is a measured hazard: F88's resume tax, 14m32s for a one-cell edit against 3m25s
like-for-like). Body **3,272 → 5,572 B** (words 484 → 848), measured at the close.

**The revival is a narrowing, not a restoration.** v1 was to be filled "for every mochiko
workflow" — a bar that **F25** measured as met by **zero of the six commands**, and that
`command-shape.md` v5 had already superseded for commands (**F21**). v2 is filled in exactly two
cases: a command run that **departs** from its command's stated default or declares non-default
bounds, and any non-command loop. A default-running command declares in one line on its
deliverable and fills nothing here. What varies per run is what gets written down — which is the
same test the v5 prohibition applied, now returning the other answer because D1-as-amended
changed the fact it tested.

**Additions this revision** — recorded for the decision row, not as strips: the two-case
WHEN-TO-FILL block · the "Why this form exists for this run" meta line · §3's **declared cost
range** and **named counter** fields plus the bound-integrity comment (rises only at a user
checkpoint; re-declaration recorded in §5) · the whole of **§5 Composed process** (departures ·
floor gates honored · counted unit · counter state) · the version footer, which this template
did not previously carry as a dated stamp.

**§5 carries three separate rulings and is worth reading as three things:** the departure list is
**D2.4/D6(b)**'s honest trail — and it is explicitly *the same* trail line the deliverable carries
at P20, not a second record, because two records of one departure is how they drift apart. The
floor-gates line is **U1-A/P18**: a departure never eats a user gate. The counted unit is
**OQ-4**, and the counter-state row is **A2**'s own clause — *"Recovery gains a counter-state row
(rounds consumed, bounds declared, departures taken) so a resumed lead re-reads its own composed
process instead of recomposing it"* — mirrored in `command-shape.md`'s Recovery block spec, which
states the obligation while this form carries the fields.

## [v0.40.0] "Fill one of these in for every mochiko workflow" superseded
- **Disposition:** superseded → rewritten in place as the two-case WHEN-TO-FILL block at the head
  of the template comment.
- **Tier failed:** n/a — supersession by ruling (**OQ-2**, adopted at **A2**; `DECISIONS.md`
  2026-08-01 row). The paired entries are `.mochiko/strips/command-shape.md` [v0.40.0] (the
  prohibition this un-does) and `.mochiko/strips/loop-discipline.md` [v0.40.0] (the skill-side
  carrier rule).
- **Content (v1, verbatim):** "Fill one of these in for every mochiko workflow."
- **Kept deliberately:** every one of the four sections and their fill instructions · the
  reviewer's four-part reading test (done-condition defaults to FAIL · a DIFFERENT agent +
  DIFFERENT skill · deterministic bounds · a named human gate) · "If you cannot fill a field
  truthfully, the loop is not ready — fix the loop, not the contract" · the no-brackets rule ·
  the storage instruction, refined only to say the filled copy sits beside the deliverable its
  departure trail lands on.
- **Consumers assessed** (all 5 files referencing `workflow-contract`, checked this wave):
  `skills/loop-discipline/SKILL.md` — the governing skill, revised in this wave at four sites ·
  `templates/command-shape.md` — revised, its prohibition superseded and its `Pairs with:` line
  re-keyed to "conditional — the per-run carrier a **departing** run instantiates" ·
  `skills/mochiko/SKILL.md` — the router's two stale claims corrected in this wave, entry
  `.mochiko/strips/mochiko.md` [v0.40.0] · `skills/validation-command-shape/SKILL.md` — its only
  mention is the new check 22, authored this wave · `templates/agent-dispatch.md` — a bare
  `Pairs with:` pointer stating no fill rule, **unaffected and byte-verified unchanged** (OQ-4
  ruled transport unaffected). Outside that set, `templates/slices-template.md:101` lists "filled
  contracts" among per-slice artifacts — still correct under the narrowed role, since a departing
  run's contract lands beside its deliverable. **No command binds a contract** — grep across all
  six for `workflow-contract` returns zero hits — so the revival adds no obligation to any
  unconverted file, and the interim is clean.

## [v0.40.0] KEPT: §3's hard round cap, unchanged under the departable default
- **Tier-2 evidence:** the field survives verbatim, and it must. Under **D6(a)** a
  command-supervised loop's cap is carried by the command's stated default bounds, but this form
  is filled precisely when a run **departs** — so the number in this field is the composed run's
  own bound, and it is a bound in the same sense: lead-counted, rising only at a user checkpoint,
  re-declared only on the record. Recorded as a survivor because "the cap moves to the command"
  is the plausible misreading of D6(a) that would have deleted the one field a departing run most
  needs. `loop-discipline` requirement 3's amendment names the failure that would follow — a cap
  that lives only in the lead's head is the LLM-controlled exit wearing a number.
