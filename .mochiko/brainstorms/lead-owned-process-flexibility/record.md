# Lead-Owned Process Flexibility — command rigor becomes the lead's judgment under a non-waivable floor

**Status:** **accepted (2026-08-01)** — pair-reviewed: 40 raised → 39 survived cross-exam → 31 lead-merged → **31/31 dispositioned** (U1–U4 user batch + lead folds); verify NOT CLEAN ×2 → 14 repairs across three rounds → **CLEAN at round 3** (one verify finding withdrawn by its owner, B9); acceptance **A1–A4 ruled** (D6(a) ratified · OQ-2 contract-carrier · B7 cost-as-bound ratified · record accepted); landed per the KM ritual (DECISIONS.md row 2026-08-01 · BACKLOG shape-v7 build item · ROADMAP Now). Build **landed 2026-08-01 at v0.40.0** (shape v7 + doctrine wave; convert-on-touch residuals open).
**When:** 2026-07-31
**Mode:** carried-over session. Opened as a direct `mochiko:analysis-iterative` invocation; graduated to `/mochiko:brainstorm` at convergence on the user's instruction ("with the context and decision above, take over"). Five rulings (D1–D5) were made in that questioning session — one question per turn, card-arbitrated — and are restated here with their full alternatives for the cold review; **D6 is a lead-proposed consequence set carried for ratification at acceptance**, per its own mark (Mode line qualified at fold, R29). The ruling-bearing card set is persisted at [`inputs/2026-07-31-questioning-session-rulings.md`](inputs/2026-07-31-questioning-session-rulings.md) (fold R24); the driver evidence likewise (below).

## Driver

The user's words, verbatim: *"i want to provide much greater flexiblity in mochiko commands to reach to its goal in terms of how it reaches there. It feels like a small change forced a long review process. Lead should have more flexility."*

**Evidence:** a user-pasted transcript of a 2026-07-31 `/mochiko:setup` amend run in mochiko-app (plugin v0.36.0), excerpt persisted at [`inputs/2026-07-31-mochiko-app-setup-amend-run.txt`](inputs/2026-07-31-mochiko-app-setup-amend-run.txt). Lead-derived cost anatomy — each figure checkable against the excerpt:

- The substantive delta: **2 governance elements** (GI-002 edited + GI-032 minted). All four stack decisions were user-ruled by card within the run's first minutes; the sizing gate itself classified the delta light ("recommended single (event-scaled: lighter substantive delta, no governance event)").
- Process consumed after those rulings: cold intent review (**17 raised → 10 survived**) · verify pass iterating **3 times** (folds-incomplete → 3 lead-pen defects → 1 follow-on overclaim needing a user ruling → PASS) · G3 confirm · **3-round** producer↔validator loop (FAIL 5 fixes → FAIL 2 defects → PASS) · **4** G4 proposal rulings · fold + bounded re-read · landing ritual · an 8-agent injection probe (222.4k tokens, offered-and-accepted). *(Split at fold R13 — machinery-mandated: the weight-blind validator branch and the gate ceremony; user-elected: the probe (accepted for a future amend's findings, mandated by no command, and D1 removes none of it) and the intent review's size (existence mandated — "the gate always opens" — size user-scaled to single). The probe was also offered at "~30–80k tokens" and consumed 2.8× that silently — behavioural counter-evidence carried at D2.3.)*
- **18 user rulings on 10 card stops** across the run (tally in the excerpt's editorial note).
- Composition of the review traffic *(recomputed at fold, R9 — the original claim used a partial denominator and carried a false attribution)*: of the **20** enumerated review items (S1–S10 · 3 verify-pass defects · 5 round-1 + 2 round-2 validator fixes), **7 are named bookkeeping** — version stamps, ledger arithmetic, trace counts, flagged-proposal counts, "basis not recorded" (S3, S6, S10, both round-2 defects, 2 of 3 verify-pass defects) — and **≥8 are consequential**: S1 (Critical), S2, S4, S5, S7, the graduation rule, and round 1's substantive catches (a test criterion contradicting its own carve-out; "materially edited" undefined while load-bearing; the components.json carrier concern, which produced a G4 user ruling); the residual four (S8, S9, and two of round-1's five fixes) remain unclassified — mixed record-scope corrections, named so the 20-item denominator stays honest (verify N1). The consequential catches attached to **both** halves of the delta — S1/S2/S4 to the copy-in, **S5 to pre-existing domain purity, S7 to the state-management mint** — so the earlier "all attached to a small-diff choice" is retracted as false on the run's own survivor table. What this composition honestly grounds is narrower than first written: the run **mixed** real catching with self-generated bookkeeping traffic in comparable shares; it does not show bookkeeping dominated, and D1/D5 rest on the corrected reading (the waste concentrated in the weight-blind branch and the bookkeeping class, not in reviewing per se).

**Scope note:** the transcript is evidence about a run in another repo at plugin v0.36.0; the current shape home is v6 (post-`plan-run-transport-forensics`). The fact map below grounds every claim about what *current* doctrine mandates.

## Fact map

*(fact-checker authored — landed verbatim 2026-07-31; the lead writes around it and never restates it)*

# Reality Map — mochiko primitive set, process-prescription surface
**Fact-checker seat · 2026-07-31 · 97 facts (F1–F97)**

Scope: `plugins/mochiko/skills/loop-discipline/SKILL.md` · `templates/command-shape.md` (v6) · `templates/sized-end-stage-review.md` (v1) · `templates/agent-dispatch.md` (v7) · `templates/workflow-contract.md` · the six `commands/*.md` · `skills/validation-command-shape/SKILL.md` · `.mochiko/brainstorms/index.md`. All paths relative to `/Users/deepeshadmin/Documents/GitHub/mochiko/plugins/mochiko/` unless stated. Derived counts show the lines they were counted from. Facts marked **[cuts against]** / **[cuts for]** relative to the session's stated direction, volunteered on the same terms.

---

### 1 · What loop-discipline mandates today

**F1.** The skill's framing sentence, `skills/loop-discipline/SKILL.md:10`: "Every mochiko workflow is a **constrained loop**, never freeform generation. Mochiko has no kernel to enforce this — the discipline is carried by this skill (the rules) plus a [`workflow-contract`]… A loop that does not satisfy all four requirements below is not a mochiko workflow; it is unbounded generation wearing a loop costume."

**F2. [cuts against]** `SKILL.md:12` names lead-judgment substitution as the target failure verbatim: "**Violating the letter of these rules is violating the spirit of them.** \"The loop basically validates\" or \"the lead will stop when it's clearly done\" is not loop discipline — it is the exact failure this skill exists to prevent."

**F3.** The four-line box, `SKILL.md:15–21`, quoted whole:
```
A LOOP IS ONLY SOUND WHEN:
  its done-condition was written BEFORE the loop ran and DEFAULTS TO FAIL,
  its validator is a DIFFERENT agent running a DIFFERENT skill than the producer,
  its iteration is BOUNDED by a deterministic cap, not by the model feeling done,
  and a HUMAN GATE is named in its contract.
You cannot rationalize your way out of any of the four.
```

**F4.** Requirement 1 (`SKILL.md:40–52`): "The success condition is written **before** the loop runs, and the result artifact **starts in the failing state**. Absence of proof reads as *not done* — never as done." Three parts tabled at `:46–50` (measurable end state · stated check · constraints). `:52`: "An LLM-controlled exit (\"I think that's done\") is not a done-condition."

**F5.** Requirement 2's independence mandate, `SKILL.md:56` verbatim: "**Never let the producer grade its own output.** Validation is run by a **different agent** using a **different skill**, working from the artifact itself — not from the producer's say-so."

**F6.** The one structural exception is already carved into requirement 2, same line (`:56`): "one structural exception: where a command shape defines a **devolved clean branch**, a unit clears on the verifying seat's PASS-with-evidence without a lead verdict, exactly on that shape's stated conditions ([`command-shape`], Layer 2 *Clearing under the mesh*); the workflow's done-condition verdict is the lead's either way." **[cuts for]** — the doctrine already admits a shape-defined waiver of the lead-verdict step.

**F7.** Validator trust ranking, `SKILL.md:58–62`: (1) deterministic ground truth "Strongest" · (2) separate-context grounded LLM "Middle" · (3) LLM-as-judge "Weakest; a fallback, never the only gate where (1) or (2) is possible."

**F8.** Tamper-proofing, `SKILL.md:64`: "A PASS is invalid unless the evidence was actually Read from the real artifact. \"It looks like it passes\" is not a PASS."

**F9.** The "cheapest external ground truth" line, `SKILL.md:66` verbatim: "When **nothing is machine-checkable**, the human is the cheapest external ground truth — which is requirement 4, not an excuse to let the producer self-grade."

**F10.** Requirement 3's opening, `SKILL.md:70`: "An LLM-controlled exit can run forever — burning budget, repeating failed actions, or grinding the validator until it games it. Every loop ships **all four** guards:"

**F11. [cuts against]** The static/deterministic cap language, `SKILL.md:72` verbatim: "**Hard round cap** — a deterministic ceiling counted by the supervisor (e.g. `max 3 produce↔validate rounds`), not judged by the model. The model's \"I'll stop now\" is itself LLM-controlled; the cap is the backstop."

**F12.** Guard 2, `:73`: "**No-progress exit** — stop when a round changes nothing, so the loop doesn't spin on stuck state." Guard 3, `:74`: "**Budget / kill-switch** — stop when the token or cost ceiling is hit; provide an out-of-band halt."

**F13.** Guard 4, `:75`: "**Escalate, don't silently die** — on hitting a guard instead of the goal, hand off to the human gate with failure context. Never report \"done\" because you ran out of rounds. Escalation is not uniform — route each finding by gap type … only genuine judgment calls reach the human."

**F14.** Requirement 4, `SKILL.md:79`: "Every contract **names where the human validates**. *Presence* is non-negotiable; *placement* is a per-workflow call:" — three named placements at `:81–83` (every cycle · low validator-confidence only · preference-gap only). **[cuts for]** — placement is already declared a per-workflow parameter.

**F15.** `SKILL.md:85`: "A workflow with no named human gate is incomplete, even if it never fires."

**F16.** The gap-routing section is titled as a *refinement*, not a fifth rule — `SKILL.md:87`: "## Routing a FAIL by gap type — a refinement of requirements 3 and 4, not a fifth requirement".

**F17.** Its stated economic rationale, `SKILL.md:89`: "Sending every finding to the human (requirement 4) spends the human on questions a machine could settle; iterating the loop on every finding (requirement 3) grinds on gaps no round will ever close." **[cuts for]** — the skill already prices over-gating as a defect.

**F18.** The three routes, `:91–93`: knowledge gap → "**Route to research** — a native `Explore` pass or equivalent investigation — not to the human"; preference gap → "**Route to the human gate**"; scope gap → "**Halt or split** — do not keep iterating. A round cap will never converge on a target the loop was never scoped to hit."

**F19.** The corollary, `:95`: "Routing a gap to the wrong sink is a defect, not a shortcut — the wasted round is exactly the one requirement 3's bound exists to catch."

**F20.** The devolved-clean-branch exception appears in loop-discipline **once only**, at `:56` (F6); no second mention. Grep-verified: `devolved` occurs at `SKILL.md:56` alone.

**F21.** `SKILL.md:99`: "A workflow without a filled contract has not met this skill's bar." — but `templates/command-shape.md:24–27` supersedes this for commands (see F30).

**F22.** Briefing is explicitly *not* a gate, `SKILL.md:103`: "a mochiko agent is a self-contained professional that **degrades gracefully** — a thin brief yields a worse result, not a broken one. So [`agent-dispatch`] is a caller-side *briefing guide* … it is **not** a fifth requirement, and an under-filled brief does not by itself make a loop unsound." **[cuts for]** — a live precedent for quality-vs-soundness tiering.

**F23. [cuts against]** The Red Flags list, `SKILL.md:111–121`, includes four entries that a lead-composed-process design would need to answer directly: "We'll add the validator later / a round cap later" · "It'll obviously stop when it's done" · "No need for a human gate, the validator is good enough" · "The done-condition is implied". Closing line `:121`: "**All of these mean:** you are rationalizing away one of the four requirements. None are optional."

**F24. [cuts against]** The Rationalizations table, `SKILL.md:126–132`, prices two candidate arguments: "Separate agent is overkill here" → "If an artifact is reviewable, independence is cheap insurance against confident-wrong output"; "The model will stop when it's done" → "LLM-controlled exits never reliably fire. The deterministic cap is the only backstop."

**F25.** `templates/workflow-contract.md` remains the fill-in form; §3 at `:43–46` requires "**Hard round cap:** [N] produce↔validate rounds, counted by the supervisor", a no-progress exit, a budget/kill-switch, and escalation. `command-shape.md:26–27` scopes it: "`workflow-contract.md` stays the form for loops whose values genuinely vary per run." No command in the six writes one.

---

### 2 · What command-shape.md v6 mandates — and every flexibility lever already in it

#### 2a · Process obligations encoded in the anatomy

**F26.** The obligated read, `templates/command-shape.md:5–7`: "**How a command consumes this:** an obligated read — the lead Reads this file up front; the command states only its per-workflow parameters."

**F27.** The fixed five-block structure, `:19–21`: "frontmatter · one-line goal + obligated reads + probe seat · **Goal** · **Seats & checks** · **Constraints** · **Bindings** · **Recovery**". Preceded at `:17–18` by "A command is **condition-first** … It is not a procedure to walk."

**F28.** The Constraints gate grammar, `:84–86` verbatim: "The gates, **in order**, one bullet each, in this exact form so the set is countable — `- **<label>** — evidence: … · rules: … · decides: …` [PARAM: the gate lines; a review-sizing gate's line carries its default keying, e.g. heavyweight→pair, or tier-keyed]."

**F29.** The bounds obligation, `:88–91`: "Then the bounds, stated once for every loop in the file [PARAM: the caps, the no-progress exit, the kill-switch path] — the lead counts them, and out of rounds = escalate, never done."

**F30.** The contract-is-the-document ruling, `:22–27`: "`loop-discipline`'s four requirements are this file's skeleton: done-condition = **Goal** · producer↔validator = the **Seats & checks** table · bounds and human gates = **Constraints**. There is no `Contract` section, and no per-run contract file is written — a per-run form whose values are constant at authoring time is ritual, not proof."

**F31.** Slot index, `:113–120` — the 17 parameters. P7 = gate lines, P8 = bounds. Both are unconditional for every command form (only P2/P13/P14/P17 carry form or branch conditions).

**F32.** Goal-block obligations, `:66–73`: "One measurable end state … plus the states that read as *not* done … Initial state is **FAIL**. User acceptance of the deliverable is part of the end state — plain blocking text, never a timed prompt." **[cuts against]** — user acceptance is a shape-level, non-parameterized obligation.

**F33.** The Seats & checks table is declared the independence proof, `:76–79`: "The table **is** the producer↔validator proof: independence is visible in it, and **no row grades its own output**." Beneath it, P6 names "which validation branch this workflow runs … the loop's bounded in-loop critique, or the sized end-stage review of a named artifact."

**F34.** Obligated reads enumerated at `:49–52`: "this file, both layers for team-form · `mochiko:loop-discipline` · `templates/agent-dispatch.md` for briefing — plus, where P6 binds a sized end-stage review, `templates/sized-end-stage-review.md`, read at the sizing gate rather than up front."

**F35.** The conditional sized-review read, Layer 1 `:145–149`: "Where P6 binds a sized review of a judgment-heavy record or synthesis, that branch's doctrine is single-sourced at `templates/sized-end-stage-review.md`: Read it at the sizing gate, not up front. Where P6 binds the loop's bounded in-loop critique instead, that file is never loaded — the in-loop branch satisfies producer↔validator on its own."

**F36.** One-lead rule, `:127–131`: "The body addresses a single lead, who owns the loop's counters, every verdict, every escalation, every human gate, and the user-facing conversation. Agents produce and review; the lead adjudicates."

**F37.** The v5 transition note (the loop-discipline read-drop deferral), `:57–64`, quoted verbatim as requested:

> **Transition note (v5).** The obligated `mochiko:loop-discipline` read **stays, and its drop is deferred — not pending a ceremony.** The pilot checkpoint ruled that *authoring-loop* evidence cannot settle it: whether a goal-shaped command holds its gates without the read is answerable only by a run. **Named trigger for reopening:** the first live dogfooded run of a rebuilt command in which the gates were not rationalized and the bounds held. Until that evidence exists, a command that omits the read is non-conformant, not early (ADR `.mochiko/decisions/2026-07-30-goal-shape-pilot-checkpoint.md`; `command-succinctness-strip` D7 + D10).

**F38. [cuts against]** The trigger in F37 is phrased as "the gates were not rationalized and the bounds held" — i.e. the named reopening condition presupposes that gates and bounds exist to hold.

**F39.** The preservation standard for any rewrite, `:40–44`: "every routing decision and every trigger survives; the narration around them dies … A line traceable to a `DECISIONS.md` row or carrying `KEPT:` Tier-2 evidence is translated into the new anatomy or superseded by a logged ruling; it is never dropped in the rewrite."

#### 2b · Flexibility levers ALREADY in the shape (full inventory)

**F40.** **Block-absence rule**, `:31–34`: "Every block present **per its binding, not per its heading.** A block whose parameters are all vacuous for this workflow is one-lined or omitted, and the absence is *stated* (\"no gates\", \"no resume table\") — never left to inference. A session command with no numbered gates and no recovery rows is the worked case (`commands/brainstorm.md`)." **[cuts for]** — "no gates" is explicitly a conformant state.

**F41.** **P17 override-only**, `:118–120`: "**P17** lifecycle override (team-form, **override only** — an unbound P17 states nothing at all: the block-absence rule above does not reach it, because Layer 2's default is what governs the silence)." **[cuts for]** — a precedent for a slot whose *silence* is the conformant default.

**F42.** **Event-scaled sizing keying**, `:85–86`: P7's parameter note explicitly admits per-run default keying — "a review-sizing gate's line carries its default keying, e.g. heavyweight→pair, or tier-keyed".

**F43.** **The devolved clean branch**, Layer 2 `:232–240`: "One unit of work (P14) advances on the verifying seat's **PASS-with-evidence, unread by the lead**, when *all* of: every verification in it is a deterministic CLI check that passed 100% · no deviation was reported · no domain-registry addition was made. **Everything else returns to the lead** … The devolved branch is *exactly* the deterministic-and-clean one, and that exactness is the guard: wherever judgment exists, the verifying seat's status is **input, never the gate**."

**F44.** **Sizing-gate ownership devolved to the user** — the whole `sized-end-stage-review.md` branch (§5 below) is a lever the shape delegates rather than fixes.

**F45.** **Counted-not-observed lifecycle cadence**, Layer 2 `:266–271`: "the lead **counts** each governed seat's completed loop units … and recycles at **~≥3**, cache warmth composing on the same trigger … The user, who can see the panes, may order a recycle at any gate. A command writes a lifecycle line **only to override**." **[cuts for]** — a live case of a doctrine default plus per-command override plus user override, all three.

**F46.** **Marked shape-exception escape**, `:8–10`: "A command line that must restate shape content (rare) carries an inline marked exception — `<!-- shape-exception: why -->` — the audit's deterministic floor keys on that marker."

**F47.** **Cadence exemption by absent denominator**, `:260–262`: "any governed seat with **no countable unit**, cadence-exempt for want of a denominator and covered by the user override below." (Bound by `brainstorm.md:43–44`.)

**F48.** **Form-scoped conformance**, `:11–12`: "A one-shot command conforms to Layer 1 alone; a team-form command to both layers."

**F49.** **Conditional slots** P13 (verify-pass owner, sized review only) and P14 (clearing unit, devolved branch only), `:117–118` — two slots that legitimately do not bind.

**F50.** Grep-verified: the levers above are the complete set in `command-shape.md`. No lever permits omitting P7 gate lines *and* P8 bounds while a loop runs; the block-absence rule (F40) reaches the Constraints **block**, and check 3 of the validator (F63) makes a "no gates" claim a FAIL "in a file whose Constraints rule gates".

**F51.** Layer 1 `:151–153` — the kernel-free ground rule and "A command suggests commits; it never runs git mutations and never pushes" — are unrelated to process prescription and unaffected either way.

---

### 3 · Per-command inventory (all six)

Method note for the derived counts: `G` was counted bullet-aware (a gate bullet wraps across physical lines), matching the validator's own definition at `validation-command-shape/SKILL.md:86–89` — a bullet matching `^- \*\*` containing **all three** of `evidence:` · `rules:` · `decides:`. My counts reproduce the validator's published measurement at `:95–96` exactly: **4 · 4 · 4 · 10 · 8 · 7**.

| command | gate lines (G) | user-ruled | lead-ruled | hard bounds | kill-switch | P6 review branch mandated in Goal |
|---|---|---|---|---|---|---|
| brainstorm | 4 | 4 | 0 | 4 (review caps) | **none** (stated) | sized end-stage review, waivable |
| specify | 4 | 4 | 0 | 3 | `SPECIFY_STOP` | in-loop critique, unsized |
| slice | 4 | 4 | 0 | 3 | `SLICE_STOP` | in-loop critique, single reviewer |
| plan | 7 | 7 | 0 | 3 (per stage ×5) | `PLAN_STOP` | in-loop critique, two reviewer seats |
| implement | 8 | 7 | 1 | 4 | `IMPLEMENT_STOP` | in-loop critique + final validation |
| setup | 10 | 10 | 0 | 4 | `SETUP_STOP` | **both** — sized review + produce↔validate loop |
| **total** | **37** | **36** | **1** | — | 5 of 6 | — |

**F52. brainstorm.md** — gates at `:48` Review sizing · `:54` Survivor rulings · `:57` Tie-break · `:60` Acceptance. All four `rules: the user`. Bounds `:64–67` verbatim: "per reviewer one cold read, plus (pair only) the one-shot four-message cross-exam, plus one verify pass; lead↔reviewer argument **max two exchanges per survivor**, you count them; one fact-checker dispatch per fact. **No kill-switch and no no-progress exit** — the human-attended session is the escalation surface, not a substitute for the caps." Goal `:23–27` mandates the sized review **or** a recorded waiver. Not-done `:30–31` names "an unreviewed record with no recorded waiver". P17 line at `:43–44`.

**F53. specify.md** — gates at `:42` G1 entry · `:49` G2 clarification · `:53` G3 acceptance · `:57` Escalation; all `rules: the user`. Bounds `:61–63` verbatim: "cap **3** rounds, you count them; no-progress exit when the gap set is unchanged round-over-round; kill-switch — stop and escalate if `.mochiko/specs/<feature>/SPECIFY_STOP` exists, checked before each seat send; out of rounds = escalate, never done." Goal `:20–22` mandates the critic branch: "the critic recommends `ready` grounded in the file; you Read `spec.md` + `advocate-report.md` and confirm no blocking gap remains". Validation model `:36` declares it "**unsized by design**". `:71–73` "**No devolved branch**".

**F54. slice.md** — gates at `:47` G1 entry · `:58` G2 clarification · `:63` G3 escalation · `:67` G4 acceptance; all `rules: the user`. Bounds `:74–77`: "cap **3** rounds, you count them; no-progress exit … kill-switch — stop and escalate if `.mochiko/specs/<feature>/SLICE_STOP` exists, checked before each seat send; a G4 amend or override re-enters the same bounded loop; out of rounds = escalate, never done." Goal `:26–28` mandates the reviewer branch. `:41–42` "from a **single reviewer** — unsized by design". `:79–80` "**No devolved branch**".

**F55. plan.md** — gates at `:54` G1 entry · `:63` G2 baseline (bootstrap only) · `:68` G3 architecture sign-off (**always-on**) · `:77` G4 feasibility/governance rejection · `:82` G5 clarification · `:86` G6 exit-early/escalation · `:90` G7 package acceptance; all seven `rules: the user`. Bounds `:94–97` verbatim: "cap **3** produce↔review rounds **per stage** (analysis · architecture · detailed design · mapping · tasks), you count each; no-progress exit on a gap set unchanged round-over-round; kill-switch `PLAN_STOP` checked before each seat send; out of rounds = escalate, never done." — i.e. **15 capped rounds across the run**, from the five named stages.

**F56.** plan's Goal `:21–26` hard-codes **six** distinct reviewer clearances into the done-condition: "`principal-architect` returned `feasible` on the analysis **and** on the architecture pass; `devils-advocate` returned `ready` on the analysis, the architecture coverage, the detailed design, the mapping **and** the tasks, each grounded in the files; the architecture sign-off (G3) cleared". **[cuts for]** — this is the densest mandated-review-branch surface in the set.

**F57.** plan carries one flexibility lever inside a gate: G3's degrade-with-record, `:73–76`: "With none of those render surfaces in an attended session the gate **degrades with record**: present the diagram source + component table and record \"presented un-rendered\" on the artifact. Plan is never hard-blocked by rendering." And a non-gate courtesy, `:115–116`: "**Optional design checkpoint** — on request in a judgment-heavy run … a courtesy, never a standing gate." **[cuts for]**

**F58. implement.md** — gates at `:56` G1 entry · `:59` Package gate · `:62` Governance surface · `:67` Cycle checkpoint (**the one lead-ruled gate in the set** — "rules: you, except on the devolved branch") · `:75` Architecture deviation · `:80` G3 clarification · `:84` G4 exit-early/escalation · `:88` G5 final acceptance. Bounds `:94–99` verbatim: "**targeted retry** — trace a checkpoint failure to its tasks and re-open only those, **max 3 attempts per cycle**, never regressing passing code; **fix pass** — failure-scoped after a final-validation failure, **max 3 passes**; **convergence stall** — the same failure pattern across **2+ rounds** surfaces rather than silently continuing, no-progress being an unchanged failing set; kill-switch `.mochiko/specs/<feature>/IMPLEMENT_STOP`, checked before each seat send. You count every round." — the retry cap is **per cycle**, so total capped attempts scale with cycle count.

**F59.** implement is the only command binding the devolved branch (`:67–74`, P14 at `:135–137`); its de-devolving conditions are command-specific: "a surfaced architecture deviation **de-devolves** it, and a non-empty `domain_deps_added` **always** forces the escalated human checkpoint — never auto-approved, no stamp read." **[cuts for]** — an existing worked example of a command parameterizing when a gate fires at all.

**F60. setup.md** — 10 gates: `:48` G1 mode-select · `:54` G2 analysis checkpoint (brownfield) · `:59` Interrogation · `:76` Review sizing · `:83` Survivor rulings · `:87` G3 synthesis confirmation · `:93` Clarification · `:96` G4 acceptance · `:103` G5 finalize · `:109` Escalation. All ten `rules: the user`. Bounds `:112–117`: "cap **3** produce↔validate rounds (you count) · no-progress exit on a fix list unchanged round-over-round · kill-switch `.mochiko/memory/SETUP_STOP`, checked before every producer, reviewer or validator send · review caps: one cold read per reviewer, one four-message cross-exam, a two-exchange lead↔reviewer cap per survivor, one verify pass, plus one bounded delta-pass on a material G3 edit."

**F61.** setup carries two context-scaled bound precedents. `:116–117`: "The interrogation is bounded instead by user-driven convergence — a human-attended session, not an agent loop." And `:63–65`: "**No pruning license**: every dimension is real for a deployed, operated product — only convergence skips, each named, never silent." **[cuts for]** — a floor-plus-named-skips construction already shipped, matching the session's "honest trail of skips" invariant.

**F62.** Measured Constraints-block body word counts (heading excluded, per `validation-command-shape/SKILL.md:123`): brainstorm 511 · specify 436 · slice 458 · plan 788 · implement 817 · setup 1076. Against the current G-keyed ceiling `90·(G+2)`: 540 · 540 · 540 · 810 · 900 · 1080. Headroom is thinnest at setup (4 words) and plan (22 words).

---

### 4 · What validation-command-shape's deterministic floor actually greps for

**F63. Check 3 (floor) — blocks present per bindings**, `validation-command-shape/SKILL.md:68–74`: "A block whose parameters are all vacuous for this workflow may be one-lined or omitted **only with the absence stated in the file** (\"no gates\", \"no resume table\"). A block that is simply missing — no heading, no stated absence — is a floor FAIL, **as is a stated absence that contradicts the body (a \"no gates\" claim in a file whose Constraints rule gates)**." → A command that stops carrying gate lines but still describes any ruling passes only if the absence is stated *and* the body genuinely rules nothing.

**F64. Check 6 (floor) — the Constraints ceiling is arithmetically keyed to the gate count.** `:86–89` defines the term: "`G` = **gate lines in Constraints** = bullets matching `^- \*\*` that contain **all three of `evidence:` · `rules:` · `decides:`** — the complete three-part form the shape mandates for P7." Ceiling at `:109–110`: "Constraints ≤ 90·(G+2)".

**F65. [cuts against] — the load-bearing consequence.** With G = 0 the Constraints ceiling becomes 180 words. Measured against F62's current bodies, all six commands would be over that floor: brainstorm +331 · specify +256 · slice +278 · plan +608 · implement +637 · setup +896. The check does not merely stop working — it **fires a floor FAIL on every command in the set** unless the Constraints block shrinks below 180 words or the ceiling formula is re-keyed. `:111–112`: "Over a ceiling is a floor FAIL — name the block, the count, the term values, and the bound."

**F66.** Check 6 also carries a calibration warning against loosening `G` by miscounting, `:88–93`: "Keying on `evidence:` alone **over-counts**: an invariant or bounds bullet may cite evidence without being a gate, and two live instances were found this way (`brainstorm`'s Invariants, `specify`'s Enrichment), each inflating `G` by one and so loosening the Constraints ceiling by 90 w per false hit."

**F67.** Check 6 explicitly decouples gates from numbering, `:92–94`: "**`G` is not \"numbered gates\"** either: a workflow that numbers nothing still has gates (`brainstorm` carries **G = 4** — sizing, survivor rulings, tie-break, acceptance — while the checker map correctly records it as having 0 *numbered* gates)." **[cuts for]** — the floor already grades gate *substance*, not gate ceremony.

**F68. Check 1 (floor) — the review-branch reference test is keyed to P6**, `:59–62`: "A file whose validation model (P6) binds a **sized end-stage review** also contains `sized-end-stage-review` — the conditional read for that branch; a file declaring the in-loop-critique branch must **not** contain it (loading it there is the sham-read the split exists to prevent)." Grep-verified present count per command: brainstorm 1 · setup 2 · specify 0 · plan 0 · slice 0 · implement 0. → If commands stopped declaring a mandated review branch, this check has no key to grade against in either direction.

**F69. Check 1 (floor) — the loop-discipline reference is currently a FAIL-on-absence**, `:63–66`: "**Re-keyed by ruling, disposition pending:** the `loop-discipline` member of this set is retained by shape v5's transition note, and whether it leaves is ruled at the pilot checkpoint … Until that ruling lands, a missing `loop-discipline` reference is a FAIL. Never treat its absence as anticipated."

**F70. Check 5 (floor)**, `:78–80`: "within the Constraints block, no line matches `^\s*\d+\.\s` and no `step \d` cross-reference appears. Gates are ordered constraints; an ordinal list is the ordering narrative returning under a permitted heading." — unaffected by dropping gates; it only bans procedural form.

**F71. Check 7 (floor)**, `:134–138`: "No row's produces/grades cell claims both authorship and grading of the same artifact, and no agent × skill pair appears as both the producer and the grader of one artifact." — this is the author≠grader floor and it keys on the **Seats & checks roster**, not on gate lines. **[cuts for]** — a goal + capability-roster command retains this check intact.

**F72. [answering the question directly] Bounds are NOT in the deterministic floor.** Checks 1–10 contain no bounds term: check 6's four counted terms are `G` (gate lines), `S` (seat rows), `A` (artifacts), `R` (recovery rows) — `:86–106`. Bounds appear only in the judgment ceiling: check 12 (`:183–184`, P8 in the parameter set) and check 13 (`:197`, "every bound has the lead as its counter").

**F73. Check 12 (ceiling) — parameter completeness**, `:186–191`: "An unbound parameter is a gap, not a style choice; a conditional slot that does not bind carries its stated absence (check 3's rule) — **P17 the one exception**: an unbound P17 states nothing at all … (TC-D6 rejected forced per-command explicitness, so a \"no lifecycle override\" line would be the defect, not the conformance)." → P7 and P8 unbound are graded gaps today; P17 is the sole precedent for a slot whose silence is conformant.

**F74. Check 13 (ceiling) — the gate/bounds soundness test**, `:194–198` verbatim: "the Goal's end state is measurable and its not-done states are real states of this workflow, not generic FAIL prose; every gate the body relies on appears as a Constraints line carrying all three of opening evidence · who rules · what it decides; every bound has the lead as its counter; and the Goal names no check that no Seats row produces."

**F75.** Summary of what breaks if commands stop carrying gate lines, caps, and mandated review branches: **check 6 fails all six commands on arithmetic** (F65) · **check 3** requires a stated absence that must not contradict the body (F63) · **check 1's** sized-review key becomes unkeyed (F68) · **checks 12 and 13** register P7/P8 as unbound gaps (F73, F74). Surviving unchanged: checks 2 (frontmatter), 4 (forbidden headings), 5 (ordinal steps), 7 (self-grading rows), 8 (restated shape prose), 9 (marked exceptions), 10 (strip stamps), 11 (altitude), 14 (preserved responsibilities), 15 (strip-note quality).

**F76.** The floor's stated purpose, `:34–37`: "the **deterministic floor** runs first and its results are recorded as the evidence (it is grep — it cannot be rationalized past); the **judgment ceiling** does the work grep cannot. A floor failure is a FAIL regardless of how good the prose reads. (The residual risk that the judgment layer rationalizes is recorded as accepted — the floor is the backstop.)" **[cuts against]** — moving process obligations from floor-graded to judgment-graded surface moves them onto the layer whose rationalization risk the skill has already booked as accepted.

**F77.** Any change to the shape home triggers checks 16–19 (`:214–234`), including `:221–222` "the version line is bumped with date + ruling source" and `:230–234` "the handoff names every conformant command the revision affects — an unnamed affected command is a gap."

---

### 5 · sized-end-stage-review.md's obligations

**F78.** Scope line, `:3–5`: "Conditional shape doctrine, read at the sizing gate when P6 binds this branch — never loaded by a command running the in-loop critique instead." Version v1, `:40–42`.

**F79. The none-with-waiver lever**, `:8–13`, quoted verbatim: "**The sizing gate is the user's.** At convergence the lead states the artifact's weight (element count, confidence-mark mix, reality-surface load) and the estimated review cost, recommends **pair / single / none** against the declared default (P7 carries that keying), and the user rules. **None** → a review waiver in the artifact's Review section (who waived, at which gate, why): the validator seat passes to the user alone, deliberately and auditably."

**F80.** Cold-and-withheld obligation, `:15–18`: "Each spawns at convergence … reads the frozen artifact cold, forms findings independently, and reports findings-formed — count only — before its counterpart is introduced. The artifact is **frozen** from reviewer spawn until every disposition lands (Review section excepted)."

**F81.** Cross-exam bound, `:19–21`: "**A pair cross-examines once:** the one-shot four-message protocol … owner-withdrawal only, the counterpart persuades, never vetoes."

**F82.** Merge ownership, `:22–25`: "**The cross-set merge and the combined tally are the lead's, never a reviewer's.**"

**F83.** Survivor routing and the argument cap, `:26–31`: "Challenges to user rulings, and user-declared facts as confirmation → the user directly; theirs to answer, not a tie-break. The lead's own formulation → argued with the finding's owner, **max two exchanges (lead-counted)**; unresolved at the cap is a deadlock → tie-break with both positions + a recommendation, the user rules. Facts → checked at P12's route, never argued … An overruled survivor marks its element `Contested`; nobody re-raises it."

**F84.** The bound statement, `:32–36`: "**One disposition per survivor** — resolved / user-ruled / recorded-open — then one **verify pass** (P13's owner; a solo reviewer verifies the lead's folds, grading the repairs, not its own findings), quoting the evidence the folds landed. … **Review + verify is the bound:** a survivor still blocking after that escalates to the user with both positions — out of bounds is never silently done."

---

### 6 · History facts from the brainstorms index (index claims — underlying records not verified)

#### 6a · Where machinery cost dominated **[cuts for]**

**F85.** `index.md:167` — brainstorm-v2-revision: "revision of v2's team engagement after transcript forensics on its first run measured **3:1 machine-to-user traffic** and consent-free folds into user-ruled decisions. Retired the standing episodic advocate; all adversarial pressure moved to convergence".

**F86.** `index.md:149` — brainstorm-v2-2-revision: "token-efficiency revision of the v2.1 end-stage review after its first completed run measured **≈654k out**, dominated by the review pair triple-reading a reality surface the fact-checker had already mapped." The sizing gate itself was born here: "review sizing becomes a named human gate (pair / single / none with waiver)".

**F87.** `index.md:107` — workflow-token-reduction: "Fact-grounded headline: **the reporting layer outweighs the design layer (31 reports ~102k tok est., never read by the user, sole consumer the lead's verdict)**." Ruled outcome included "sizing gates generalized with **verification depth floored never-zero**" — the same floor-plus-flex construction the session is proposing.

**F88.** `index.md:19` — plan-run-transport-forensics, the headline figures: "the user-labelled \"v0.38.0\" plan run … measured at **~285 min agent runtime + ~74 min lead turns across 44 seat tasks, unfinished at capture**." Compounding costs named: "completeness seat **836k-token pane** reading at 14+ passes"; "the resume tax (**14m32s for a one-cell edit vs 3m25s like-for-like**)"; "ritual waste (**seven no-op compact resumes**)".

**F89.** `index.md:14` — the session's own driver, as the index records it: "a 2-element mochiko-app setup amend consumed **two review branches, ~18 user rulings, and a 3-round validator loop**."

**F90.** `index.md:189` — command-altitude: precedent for very large command reductions holding up — "recipe fixed, `specify` (**329→66**) and `setup` (**385→78**) retrofitted, both independently verified PASS".

**F91.** `index.md:183` — brainstorm-command (v1), superseded: "adversarial substance kept, **phase/gate ceremony killed**" — a precedent for cutting ceremony while retaining the substance.

#### 6b · Where caps, gates, and reviews demonstrably caught defects **[cuts against]**

**F92.** `index.md:19` — the same forensic run's verdict on the machinery that held: "**Caps, author≠grader, G3 degrade-with-record, batched rulings, and feasibility re-fire discipline all held.**" The failures were in the *un-doctrined* surfaces: "the team never existed — every seat was a harness-classified background subagent (the forbidden transport), undetected because shape v5's probe discriminator … no longer discriminates"; "no context lifecycle in the shipped shape"; "Conduct breaches independent of transport: the Layer-1 machinery-vocabulary ban and housekeeping-narration ban, **both pervasive**."

**F93.** `index.md:131` — setup-v3-team-defect: a hard, explicitly-stated mandate was set aside by a lead composing its own process — "the post-v3 dogfood run of `/mochiko:setup` (kinako) executed its producer as a one-shot subagent **despite the hard team requirement** — fork **B, set-and-ignored**: the lead probed `AGENT_TEAMS=1` itself, followed every surrounding instruction, then dispatched via the Agent tool."

**F94.** Verify passes caught blocking defects on **six** separate accepted sessions: `index.md:23` team-lead-strategic-compaction "verify round 1 **NOT CLEAN — 5 blocking + 5 non-blocking**, all repaired same round → round 2 CLEAN" · `:29` ops-observability-hardening "verify round 1 **NOT CLEAN — 3 defects incl. a ninth enumeration file invisible to every grep**" · `:35` security-depth-scoping "verify **NOT CLEAN×2** → 9 repairs + 1 tidy penned … → CLEAN at round 3" · `:41` production-only-focus "verify pass caught **2 fold-contradictions**" · `:53` architecture-design-primitive "verify pass CLEAN after **one blocking repair (B1, R4-propagation into D7)**" · `:59` operating-docs-maintenance "verify pass CLEAN after **one blocking-collision repair round** + bounded delta".

**F95.** Cold reviews killed or reversed load-bearing premises on **four** sessions: `index.md:85` skill-succinctness-strip "**Headline review find (C1): the \"descriptions load fully into every session\" premise is false**" · `:49` command-succinctness-strip "the map's derived aggregates were wrong — **param-slot sections are the majority of every command**, \"flow dominance\" retracted" · `:125` setup-adversarial-review "**re-ruled on corrected facts after the review's Critical finding (S1)**" · `:31` ops-observability-hardening "the \"mostly promoting electives\" premise was false".

**F96.** No reviewed session in the index returned zero survivors. Raised→merged tallies (index lines `:23, :29, :35, :41, :47, :53, :59, :71, :93, :99, :105, :111, :117, :123, :135, :141, :153, :171`): 35→29 · 29→20 · 25→20 · 24→18 · 18→13 · 18→13 · 16→13 · 16→8 · 15→11 · 14+18→21+3 · 14→13 · 14→11 · 13→10 · 12→9 · 11→10 · 9→7-confirmed · 1C/8I/3M. Merge attrition is real (e.g. 35→29, 29→20, 24→18), but the surviving count was never 0.

**F97. [cuts both ways]** Review sizing is already exercised across its full range in practice: **13** sessions pair-reviewed; **4** sized to a solo/single reviewer (`index.md:71` skill-succinctness "solo-cold-reviewed + lead citation-verified"; `:111` domain-dependency-allowlist "user-sized solo cold subagent + lead pressure-test of every finding against cited files: 9 raised → **7 confirmed / 1 weakened / 1 reframed / 0 refuted**"; `:141` setup-operating-docs-scaffolding "single reviewer per sizing gate"; `:23` team-lead-strategic-compaction "sized single, **user-reversed to pair pre-spawn**"); and **6** run bare/un-reviewed (`:17` plan-run-transport-forensics, `:65` team-method-vs-command-shape, `:129` setup-v3-team-defect, `:147` brainstorm-v2-2-revision, `:159` fact-checker-seat, `:165` brainstorm-v2-revision). Two of the six bare sessions (`:65` team-method, `:129` setup-v3-defect) shipped rulings that later needed correction or reopening — `:68` records team-method's escalations closing only on 2026-07-31 at v0.39.0, and `:132` records setup-v3-defect's "Verification pending".

---

### 7 · Claim checks

#### C-a — "setup.md's review sizing gate covers only the intent-review branch; nothing sizes the producer↔validator authoring loop."
**CONFIRMED, with one nuance volunteered.**
- The gate's own scope, `commands/setup.md:76–82`: "**Review sizing** *(all modes, before G3)* — evidence: the synthesis's weight … rules: the user · **decides: the reviewer count**." Its object is the intent reviewer(s) only.
- The two branches are declared separately at `:41–44`: "two branches, different stages — the **sized end-stage review** of the frozen `governance-intent.md` before G3 … **then the produce↔validate loop**, whose PASS is the authoritative grade on the surface set."
- The authoring loop's only quantity is a fixed cap, `:112`: "cap **3** produce↔validate rounds (you count)". No sizing gate, no waiver, no skip path anywhere in the file.
- **Nuance (volunteered, cuts for the session):** the authoring loop's *check surface* is already scaled — `:129–132`: "**The validator's check surface is mode-parameterized every round** — brownfield adds the tools/versions↔`codebase-analysis.md` cross-check; an attached knowledge-management module adds the repo-level invariant re-audit … Selecting it is a policy call that stays yours." So a lead-composed-depth precedent exists inside the loop; what does not exist is any lever over whether the loop runs or how many rounds it may take.

#### C-b — "setup.md's Goal block hard-codes both review branches into the done-condition regardless of delta weight."
**NUANCED.** Both readings shown.
- **Supporting the claim:** both branches are in the done-condition, `:23–27`: "the sized intent review ran (or its waiver is recorded) with every survivor dispositioned · G3 cleared, **in every mode** · `validator` returned PASS graded from the files · G4 accepted …". Not-done at `:30–31` re-states both: "an undispositioned survivor with no recorded waiver · **a validator FAIL, or a PASS read off its report**". The validator-PASS branch carries **no** alternative, no waiver, and no weight keying anywhere in the file — it is unconditional at every delta weight, including a wording-level amend.
- **Cutting against the claim:** the intent-review branch is *not* weight-blind. Its done-condition clause carries an explicit escape ("or its waiver is recorded"), and its default is delta-scaled at `:78–82`: "The default is a **pair**, **event-scaled on amend**: a governance event (un-waive, floor change, module attach/detach) takes the full pair; a lighter substantive delta recommends single; a wording-level delta, none-with-recorded-waiver. **The gate always opens**: every amend records a ruling or a waiver, keeping the trail audit-complete."
- **Net:** "both branches in the done-condition" is confirmed; "regardless of delta weight" holds for the validator branch and is refuted for the intent-review branch.

#### C-c — "validation-command-shape's deterministic floor greps for gate lines and bounds."
**NUANCED — half confirmed, half refuted.**
- **Gate lines: CONFIRMED, and more strongly than the claim states.** Floor check 6 counts them mechanically (`validation-command-shape/SKILL.md:86–89`) and the Constraints ceiling is arithmetically keyed to the count, `:109–110`: "Constraints ≤ 90·(G+2)". Floor check 3 additionally grades a stated gate-absence against the body (`:72–74`). The consequence is quantified at F65: at G = 0 all six commands blow the 180-word ceiling.
- **Bounds: REFUTED for the floor.** Checks 1–10 contain no bounds term; check 6's counted terms are `G`, `S`, `A`, `R` only (`:86–106`). Bounds are graded exclusively in the judgment ceiling — check 12's P8 (`:183–184`) and check 13's "every bound has the lead as its counter" (`:197`).

#### C-d — "the sized-end-stage-review's 'none' ruling already passes the validator seat to the user alone with a recorded waiver."
**CONFIRMED verbatim, with a scope nuance volunteered.**
- `templates/sized-end-stage-review.md:11–13`: "**None** → a review waiver in the artifact's Review section (who waived, at which gate, why): **the validator seat passes to the user alone, deliberately and auditably**."
- **Nuance:** the lever's reach is not uniform across the two commands that bind it. In `brainstorm.md` there is no other validation seat — `:40–41`: "the sized end-stage review of `record.md`; **there is no in-loop critique seat**" — so a `none` ruling genuinely leaves the user as the run's sole validator. In `setup.md` the waiver removes only the intent-review branch; the produce↔validate loop's agent validator still runs unconditionally (see C-a/C-b). So the claim is true of the branch, and true of a whole run only in brainstorm.
- Second-order fact, `brainstorm.md:83–84`: the waiver's downstream consequence is itself recorded — a synthesis produced under a waiver "is stamped **\"derived, unchecked\"** instead — the same recorded-absence discipline as the waiver."

#### C-e — "brainstorm.md has no kill-switch and no no-progress exit, with a stated rationale that the human-attended session is the escalation surface."
**CONFIRMED, with the rest of the sentence volunteered.**
- `commands/brainstorm.md:66–67` verbatim: "**No kill-switch and no no-progress exit** — the human-attended session is the escalation surface, **not a substitute for the caps**."
- Grep-verified: brainstorm is the only one of the six with no `*_STOP` file (the other five carry `SETUP_STOP` `:113` · `SPECIFY_STOP` `:62` · `SLICE_STOP` `:75` · `PLAN_STOP` `:96` · `IMPLEMENT_STOP` `:98`).
- **Volunteered, cuts against reading this as a bound-free precedent:** the same bullet's trailing clause keeps four caps in force — `:64–66`: "per reviewer one cold read, plus (pair only) the one-shot four-message cross-exam, plus one verify pass; lead↔reviewer argument **max two exchanges per survivor**, you count them; one fact-checker dispatch per fact." So brainstorm drops two of loop-discipline's four guards (F10–F12) on a stated rationale while explicitly retaining the other two. It is a precedent for **context-scaled selection among the four guards with the omission stated**, not for a command carrying no bounds.
- Conformance note: this passes today because of the block-absence rule (F40) plus check 3 (F63) plus check 12's P8 treated as bound-with-stated-absence — the same construction the session would need to generalize.

---

**Standing.** The checker remains available for reviewer fact disputes routed through the lead. Two derived aggregates above are the checker's own and re-checkable on demand: the G-per-command counts (F52–F60, reproducing `validation-command-shape/SKILL.md:95–96`) and the ceiling arithmetic at G = 0 (F62, F65).

### Map errata (checker-authored, 2026-08-01 — fact-route from the record-integrity reviewer, resolved option (b): corrected figures)

**The §3 `hard bounds` column is corrected.** No counting rule yields the column as published; two incompatible rules were applied across rows (setup's five-member `review caps:` sub-list collapsed to one clause; brainstorm matching neither rule). The rule that should have been declared: **a hard bound = one distinct guard with its own trigger and its own ceiling, counted individually — sub-lists counted as their members, not their label; stated absences not counted; the escalation rule (loop-discipline guard 4) not a separate bound.** Corrected: **brainstorm 4→5** (cold read per reviewer · four-message cross-exam · verify pass · two-exchange cap per survivor · one checker dispatch per fact) · **setup 4→8** (3-round cap · no-progress exit · `SETUP_STOP` · cold read per reviewer · cross-exam · two-exchange cap · verify pass · bounded delta-pass) · specify 3 · slice 3 · plan 3 (the cap clause instantiates once per named stage — ×5 multiplies instantiations, never clause count) · implement 4 (retry cap instantiates per cycle). **Corrected total: 26 individual hard bounds** (published column summed 21). **C-e correction:** "four caps in force" → **five**; C-e's verdict (CONFIRMED) and conclusion unaffected.

**F60-a [cuts against].** Under the corrected rule setup carries **8** individual hard bounds — the heaviest bound surface in the set by a factor of two over the next command, double the published figure.

**F52-a [cuts against reading brainstorm as a light-bound precedent].** At 5, brainstorm has *more* individual caps than specify, slice, or plan (3 each), despite being the one command with no kill-switch and no no-progress exit. Brainstorm is a precedent for *substituting* guard kinds — dropping two of loop-discipline's four while carrying the densest per-loop cap set in the library — not for carrying fewer bounds.

### Map errata 2 (checker-authored, 2026-08-01 — second fact-route, F96 reconciliation)

**The missing line ref is `:117`** — `pattern-codification-and-minimalism`, tally `17 raised → 13 merged`. Corrected count: **18 tallies for 18 line refs**, one-to-one. Two further defects volunteered from the same enumeration, both citing an intermediate figure where the column's axis is raised→merged: `:41` production-only-focus published 14→13, **corrected 14→9** ("9 lead-merged survivors") · `:99` standing-seat-lifecycle published 16→13, **corrected 16→11** ("11 merged survivors"). Remaining fifteen verified correct on the merged axis; three carry a different shape by construction and are labelled, not errors (`:71` formed→reported · `:111` confirmation axis · `:171` severity split only).

**Corrected F96 enumeration (18 entries, descending by raised):** 35→29 (`:59`) · 29→20 (`:35`) · 25→20 (`:29`) · 24→18 (`:47`) · 18→13 (`:123`) · 18→13 (`:71`, formed→reported) · **17→13 (`:117`)** · **16→11 (`:99`)** · 16→8 (`:135`) · 15→11 (`:153`) · 14+18→21+3 (`:23`) · 14→11 (`:105`) · **14→9 (`:41`)** · 13→10 (`:93`) · 12→9 (`:141`) · 11→10 (`:53`) · 9→7-confirmed (`:111`) · 1C/8I/3M (`:171`).

**Effect on the dependent finding:** F96's claim holds, now at 18/18 — no reviewed session ever returned zero survivors. **One thing the corrections move, and it cuts *for* the session's direction:** merge attrition is larger than published at both corrected sites (36% and 31%, not 7% and 19%); across the twelve entries with a clean raised→merged pair, the lead's merge discards roughly a quarter to a third of what cold reviewers raise. Anything leaning on F96 should lean on the corrected figures.

**Checker's note on its own error class:** all three defects across the two fact routes are **derived aggregates**; every primary measurement and verbatim quotation has held under challenge — the same failure signature the index records at `command-succinctness-strip` (`:49`). The checker recommends reviewers keep aiming at computed columns, stated totals, and per-item attributions rather than re-sampling quotations, and will re-derive any other aggregate on request.

## Decisions

### D1 — Commands become goal + toolbox; the lead composes the process

**Statement** *(as amended at U2 — superseded clauses struck in place, verify B2)*: A mochiko command carries its goal (done-condition + not-done states), its capability roster (the seats, agents, and skills the lead may deploy), its bindings, the floor (D2), and — per U2 — its **stated default pipeline**. ~~not a prescribed pipeline~~ → the pipeline is a **default, never an obligation**: the lead composes the process per run — whether the default's reviews run, how many rounds, which stages collapse or merge — **by recorded departure, never silent omission**. ~~No static round counts in commands~~ → the default's round counts stand in the command as departable defaults. No codified weight-tier ladder anywhere (unchanged).

**User's ruling, verbatim** (rejecting both the lead's tier-ladder framing and all three dealt collapse options): *"I want the lead to be incharge of it. I dont want static number of rounds of paths. The lead owns the goal and 'tools' it has access too. When i say tools, it is basically agent teams and their capabilites"*.

**Rationale:** (a) The driver run shows the weight-blind validator branch running unconditionally on a delta whose decisions were already user-ruled *("full strength" retracted at fold R13 — the intent-review branch was user-scaled down and still yielded 17 findings; the indictment belongs to the branch without a lever)* — the existing flexibility lever (review sizing, pair/single/none) covers only the intent-review branch: the authoring loop has no sizing gate, no waiver, no skip path, and its validator-PASS clause is unconditional at every delta weight (map C-a CONFIRMED, C-b's supporting half). *Lead correction at map reconciliation:* my original phrasing — "both branches hard-coded regardless of weight" — was half wrong: the intent-review branch **is** weight-scaled (waiver escape + event-scaled default, map C-b refuting half); the weight-blindness is real but belongs to the validator branch alone. (b) Mochiko's founding bet — "engineering discipline lives in the quality of the skill library, not in a deterministic kernel" — read seriously: a prescribed pipeline inside a command is a soft kernel. (c) The counter-evidence was stated in the open before the ruling: lead judgment drifts under context pressure (the v0.34.0 pilot checkpoint deferred even the `loop-discipline` read-drop pending live-run evidence; the driver run's lead double-spawned a producer with full rails in place). The floor (D2) is the designed answer to that counter-evidence, not a denial of it.

**Alternatives rejected:** static weight-tier ladder (the lead's initial framing — withdrawn on the user's redirect) · per-gate skip-negotiation (every run renegotiates process) · cost-driver-only surgery (lighter record formats, merged gates — keeps the structural rigidity) · one-cold-read-post-author as a codified light path · sizing-both-branches-down as a codified light path · user-as-sole-grader as a codified light path (all three subsumed: any of them is now a composition the lead may *choose*, none is prescribed).

**Mark:** `Confident` — user-declared direction, restated back and confirmed across three subsequent rulings.

**Amended at review (2026-08-01, U2 — R8's never-dealt option, dealt and adopted):** a command **states its default pipeline** — its current gate lines, seats, and bounds survive as that stated default — and the lead **departs from it at will**, each departure one recorded trail line; nothing obliges the default to run. The ruled core is preserved (the lead owns the how; no obligation survives that the lead cannot depart from) while the on-disk baseline returns for everything that needs one: skip-trails (R10), the interim audit (R6), recovery (R18), the lifecycle denominator (R23). **Counter-evidence engaged (R1):** F96's 18/18 never-zero base rate and F94's six verify-pass catches are answered structurally, not predicted away — reviews remain every command's *default*, and a lead composing one out does it past two floor stops (the U1-A weight card and the recorded departure), never silently. F93's set-and-ignored precedent is answered by the same pair: a stated default cannot be silently absent, and its absence is a user-ruled card, not a lead inference. F76 is answered at the amended D6(d): the deterministic audit keys survive because the default's gate lines survive. F92's conduct-breach evidence stands un-neutralized and is priced into U1-B — the non-discretionary cold grade on the lead's own pen. F52-a/F60-a consumed: the bound surface being made departable is larger than first inventoried, which is why departures are floor-recorded rather than free. F95 (cold reviews reversed load-bearing premises on four sessions — and on this one: the composition claim and the input-confidence factor were both reversed by this record's own review, R9/R4) is engaged the same way: premise-reversal capacity lives in the default reviews and U1-B's non-discretionary cold grade, which survive every composition except one the user explicitly waived. Mark unchanged: `Confident` — amended by the user's own card.

### D2 — The non-waivable floor: four invariants

**Statement:** Four invariants sit beyond the lead's authority to waive, at any weight:

1. **User gates** — what is the user's to rule is ruled by the user; the lead never self-accepts a deliverable. (Count and shape of the stops: D3.)
2. **Author≠grader when reviewing** — *whether* a review runs is the lead's call; but a recorded PASS never comes from the artifact's own author. If it grades, a cold seat grades. *(The conditional is superseded at U1-B — amendment item 2 below; the card text itself is preserved unedited per the persisted Q3 ruling.)*
3. **Self-declared bounds** — no static caps in commands; instead the lead declares its plan and bounds at run start, user-visible. Busting a self-declared bound escalates — never silently continues. *(The "no static caps in commands" clause is superseded at U2 — caps survive as stated, departable defaults; hardened at U1-D, amendment item 3 below; card text preserved unedited.)*
4. **Honest trail of skips** — every stage the lead collapses or skips gets a one-line record on the artifact. Flexibility never costs auditability.

**Rationale:** each invariant guards a distinct failure mode — self-acceptance, self-grading, runaway loops, invisible corner-cutting — at near-zero token cost; jointly they are what keep a lead-composed run auditable and make the D1 bet reversible (a bad run leaves a trail that shows exactly which skips hurt).

**Alternatives rejected:** none — all four adopted; the alternative of a larger floor (mandatory machine validation per artifact) is exactly what D1 removes, and a smaller floor (dropping any of the four) was not argued by either side.

**Mark:** `Confident` — adopted as recommended, all four. *(Superseded by the amendment below — mark re-earned card-by-card at U1, closing R14's streak finding for this element.)*

**Amended at review (2026-08-01, U1 — three hardenings adopted, one declined):**

1. **User gates** — unchanged, plus the **run-start weight card** (U1-A): the lead's D5 weight read and composed process (default, or departures from it) is a standing user-ruled card at run start. Partially amends D3. **R11 folded:** this invariant is an *accountability* guarantee, not a detection one — the driver evidence shows 18/18 user↔lead concordance on recommendation-bearing cards; detection lives in invariant 2's cold grade and the default reviews, never in the user's eyes alone.
2. **Author≠grader** — hardened (U1-B, closes R3): the lead's own folds and any lead-penned record get **one cold-seat grade, non-discretionarily** wherever a review ran; a lead-penned deliverable ships with zero cold reads only by recorded user waiver at the weight card. The conditional that made D2.2 vacuously satisfiable is gone.
3. **Self-declared bounds** — hardened (U1-D, closes the R16/R20 cluster): every declared bound has the lead as its named counter · a bound **rises only at a user checkpoint** · re-declaration is itself recorded. The driver run's 2.8× silent probe overrun (offered ~30–80k, consumed 222.4k) stands in the trail as the behavioural counter-evidence this rule answers. Whether cost ranges are part of the declaration: **lead-inferred yes — pending ratification, NOT settled here** (verify B7: the card of record carries three mechanisms and no cost clause, so this reading is the lead's, not the user's). The inference's ground, stated for the ratification card: if cost ranges are not bounds, U1-D's three rules never touch the one behaviour the session actually observed — the 2.8× silent overrun. Live instance if ratified: this record's own sizing declaration ("pair ≈ 200–350k tokens") was a bound on this run, with a counter nobody kept. **Takes its own line on the acceptance card.** **→ Ratified at acceptance (A3, 2026-08-01, user card): cost ranges ARE floor bounds.** The pending-ratification marker above stands as provenance — who inferred, who ruled.
4. **Honest trail** — re-anchored by U2 (closes R10, and R15's contradiction): with a stated default, a *skip* is well-defined again. The trail records **departures from the default** and names the grading that actually ran — one reading; D6(b) now says the same thing.

**Declined (R5, user-ruled — element marks `Contested`):** elevating F7's validator-tier ranking to floor status plus a bookkeeping-linter obligation. Validator-tier selection stays lead judgment under the floor above; the linter idea is recorded as a non-blocking build suggestion (it would have deleted the driver run's round 2 — both round-2 defects were trace arithmetic).

### D3 — Checkpoint consolidation is the lead's

**Statement:** The floor guarantees **what** is the user's to rule, never **how many stops** deliver it. The lead batches rulings into the fewest checkpoints that respect them — the driver run's 18 rulings over 10 stops could have been ≈6 stops *(corrected in place at fold R12, verify B3 — the original "~4" is superseded: reaching it requires deleting a ruling-generating stage)*. Whether to ratify the run plan up front is itself lead judgment, scaled to stakes; the user can always interrupt a live session.

**Alternatives rejected:** plan-ratified-up-front-always (a standing steering card on every run, even trivial ones) · named-gates-stay-fixed (flexibility confined to agent-side machinery).

**Mark:** `Confident` — adopted as recommended. *(Re-ratified via U1-A's explicit card, closing R14 for this element.)*

**Amended at review (2026-08-01):** U1-A makes the run-start weight/process card a **standing user stop**; consolidation authority governs everything else. **Rationale (added at fold, R30 — this decision shipped without one):** the floor guarantees *what* is the user's to rule, never the stop count; the user can always interrupt a live session; the driver run's cost was in stop shape and self-generated traffic, not in the count of things genuinely theirs to rule. **Stop arithmetic corrected (R12):** 9 of the driver's 18 rulings existed only because a stage generated them; batching everything batchable without deleting a ruling-generating stage floors at **≈6 stops, not ~4** (and one of the ten stops was an interrupt artifact — designed baseline nine). Below ≈6, fewer stops means fewer *rulings* — a different trade, dealt to the user if ever proposed, never folded into "consolidation."

### D4 — Landing: shape v7 now, commands convert on next touch — `Contested`

**Statement:** The doctrine revision lands now — `command-shape.md` v7 plus the D6 consequence set — and each of the six commands converts **when next touched or needed**. The library runs mixed-form in the interim (converted commands lead-composed; unconverted ones still prescriptive until their turn).

**Rationale (user's side):** immediacy — the flexibility is wanted for real upcoming runs; evidence accrues per command as each converts, so every conversion's first live run is its own checkpoint.

**The recommendation it overrode:** pilot-one-command-first with a named confirm-or-revert checkpoint, per the repo's own precedent — the v0.34.0 pilot checkpoint ruled that even the `loop-discipline` read-drop could not be justified on authoring evidence alone and deferred it to a named live-run trigger; and the combine-precedented-waves norm requires an audit-cleared precedent before a combined ceremony, which does not exist for this form. Wholesale-wave-now was rejected by both sides (six-command revert risk).

**Mark:** `Contested` — the user chose against the lead's recommendation with the precedent stated in the open.

**Review folds (2026-08-01 — the ruling itself unchanged, `Contested` stands):** *(R6/R26)* Under U2, unconverted commands remain conformant during the interim — their gate lines and caps **are** stated defaults — so the mixed-form problem shrinks to an **additive** audit delta: OQ-1 owns the dual-form branch (floor/declaration checks added; the existing G-keyed checks survive), and F77's version-line requirement now has its ruling source — this record's U1/U2/U4 rulings. *(R16)* The v0.34.0 trigger's terms survive, re-keyed: "the gates were not rationalized" is measurable against the stated default plus recorded departures; "the bounds held" against declared bounds under U1-D's counter-and-no-silent-re-declaration rule; re-specification is owned by the v7 authoring item. *(R24's worked case, iM3)* The rationale paragraph above is **lead-reconstructed** — the user selected the option labeled "Shape v7, convert on touch" with the pilot recommendation and the v0.34.0 precedent stated in the open; no verbatim user words exist for this ruling.

### D5 — Rigor scales with cost-of-being-wrong, never task size

**Statement:** The doctrine that replaces the rails names **four factors** the lead weighs, and states its read of them in the run-start declaration (D2.3): **reversibility** (how expensive is rework if this is wrong) · **blast radius** (how much downstream work reads this artifact as authoritative) · **precedent** (first-of-kind vs mirroring an audit-cleared pattern) · **input confidence** (user-ruled inputs vs lead inference). Task/diff size is at most a hint inside these factors, never the dial.

**Rationale:** the driver run falsifies size in both directions — the delta was small and the process was huge (the waste), and the run's one Critical catch attached to a four-word card choice whose consequences were large (the value). Worked example: *the version originally recorded here — "reversible / moderate radius / precedented form / all-user-ruled inputs → one cold read over finished surfaces" — is **superseded in place** (verify B4) by the re-derived example in the U3 amendment below, which is the binding one: the original scored inputs on the upstream cards and prescribed a post-authoring read, both corrected at U3/R19.* The greenfield contrast stands: a first constitution scores high on every axis → earns the full apparatus. Same doctrine, both outcomes, no ladder.

**Alternatives rejected:** task-size dial (misfires on small-and-consequential — the shadcn copy-in would have gone unreviewed) · no-doctrine pure judgment (nothing to audit the declaration against; nothing teaches the craft to a weaker model in the seat).

**Mark:** `Confident` — adopted as recommended. *(Re-earned at U3's explicit card, closing R14 for this element.)*

**Amended at review (2026-08-01, U3 — R4 + R19):** the input-confidence factor **splits**: a user ruling discounts *ambiguity* risk only; a ruling that introduces new surface (the shadcn copy-in) **raises consistency risk**; and the factor is scored on **the artifact under review** — the lead-authored delta — never the upstream cards. **Worked example re-derived:** the driver amend now scores reversible · moderate radius · precedented form · inputs user-ruled *but surface-introducing* · artifact lead-authored → **a cold review of the authored delta was earned**, and the intent review did catch S1. What the example teaches after the amendment: the waste was never the review existing — it was the weight-blind second branch and the self-generated bookkeeping rounds. **Example status (R19):** binding guidance — a lead owes its run-start declaration a stated read against these factors; an illustration that obliged no one taught no one.

### D6 — Derived-consequence set (proposed by the lead, carried into this record for ratification at acceptance)

**Statement:** The consequence edits D1–D5 force, named so the review can probe them and the user ratifies them as a set at acceptance rather than card-by-card:

- **(a) `mochiko:loop-discipline` rewrite** *(amended at folds; scoped per R28 to command-supervised loops — non-command loops keep the four requirements exactly as written today)*. Requirement 1 (pre-declared done-condition, default FAIL) and requirement 4 (a named human gate) stay mandatory. Requirement 2 keeps author≠grader absolute and **gains the U1-B floor clause** (the lead's folds/record always cold-graded); whether other agent reviews run beyond the stated default is departure judgment under D1-as-amended. *The earlier F9 citation is withdrawn (R5/dC5): F9's antecedent fails for machine-checkable artifacts and its closing clause forecloses the inference — validator-tier selection is now honestly recorded as lead judgment, ruled `Contested` at U1-C's decline, not doctrine-licensed.* Requirement 3's carrier: the command's **stated default bounds** plus recorded departures plus U1-D's counter/no-silent-re-declaration rules — not command-static-only, and never declaration-only.
- **(b) Goal blocks lose process residue.** Done = artifact state + floor compliance + user acceptance. "The validator returned PASS" and "the sized review ran" leave the done-condition — the default pipeline stays **stated in the command** (U2) but a departure-run can still reach done. The trail records departures-from-default and names the grading that ran — one reading, shared with D2.4 (R15 resolved; the earlier two-artifact ambiguity is gone).
- **(c) `templates/sized-end-stage-review.md` survives as the stated default** review pattern wherever P6 binds it today — *"demotes to optional playbook" is withdrawn* (it created R3's self-grading hole and R7's silent supersession). Two ruled changes to it: sizing ownership passes to the lead **by recorded supersession** (U4 — logged at the landing under the primitive-edit ceremony), and the verify-pass clause **hardens to floor** (U1-B). Departing from the pattern is a recorded departure like any other.
- **(d) `mochiko:validation-command-shape` re-keys additively** *(amended at U2 — F65's forcing dissolves: gate lines and caps survive as stated defaults, so the G-keyed arithmetic and check 6 survive intact, as do checks 2/4/5/7/8/9/10/11/14/15 per F71/F75)*. Added: floor-presence checks (the amended D2 invariants reachable), the declaration/trail carriers named, departure-license grading (a departure without its trail line is the new deterministic FAIL), and the **dual-form interim branch** keyed to a form declaration in the file (R6); the version-line's ruling source is this record (R26). This answers F76 on its own terms — the deterministic floor stays the backstop; nothing moves wholesale onto the judgment layer.
- **(e) — dissolved into OQ-2** (R31): the run-start declaration's home and form are ruled there, at acceptance, on the rewritten OQ-2's proposed answer. An open question does not ride a ratification set.

**Mark:** `Assumed` — lead-proposed, never individually user-ruled; **ratification restructured at fold (R31):** (a) takes **its own card** at acceptance (doctrine surgery on the skill that carries mochiko's entire loop discipline deserves its own yes) · (d) is noted consequence-forced in its amended, additive form · (e) has dissolved into OQ-2's ruling · (b)/(c) ride the set. All five amended at the U-batch folds as written above. The set carries one **`Contested` element inside it** — (a)'s validator-tier decline (U1-C) — which the acceptance card must show alongside the `Assumed` mark (verify N4). **Ratified at acceptance (2026-08-01):** (a) by its own card (A1, Contested element shown) · (b)/(c)/(d) with the accepted set (A4) · (e) closed by A2's OQ-2 ruling. The `Assumed` mark is discharged.

## Open questions *(rewritten at the fold pass, 2026-08-01)*

- **OQ-1** *(now a scoped work order, not an open design question — owned by the v7 authoring item)*: encode U2 — **P7/P8 survive as default carriers**; add slots for the floor invariants, the run-start declaration, and the departure trail; author the audit's dual-form interim branch (R6); re-specify the v0.34.0 trigger's terms against defaults + declared bounds (R16); and carry R21's recorded-open obligation — a measured cost estimate for declaration + trail + composition on one light and one heavy run (verify N3). Ruling source for F77's version line: this record.
- **OQ-2** *(carries R17/R18 and absorbed D6(e); ruled at acceptance)*: the run-start declaration's durable home. **Proposed answer:** `workflow-contract.md` revived as the per-run carrier — D1-as-amended makes its values genuinely vary per run, which is F25's surviving scope and un-does F30's premise for departing runs — instantiated **only when a run departs from the default or declares non-default bounds**; a default-running run's declaration is one line on the deliverable. Either way **Recovery gains a counter-state row** (rounds consumed, bounds declared, departures taken) so a resumed lead re-reads its own composed process instead of recomposing it (F88's resume tax is the measured hazard). **Ruled at acceptance (A2, 2026-08-01): the proposed answer adopted as written.**
- **OQ-3** *(replaced at fold, R27 — the presumption was wrong)*: the landing owes the **primitive-edit ceremony**: version-stamped `.mochiko/strips/` entries + independent author≠grader audits for `loop-discipline` · `command-shape.md` · `sized-end-stage-review.md` · `validation-command-shape` at the doctrine wave, and per command at each conversion; **supersessions to log by ruling:** the sizing-gate ownership transfer (U4) and any protected/`KEPT:` line touched — F39's preservation standard governs every rewrite. KM close rituals themselves: unaffected, confirmed.
- **OQ-4** *(split at fold, R23)*: **transport — unaffected, confirmed** (roster probe, no-fallback bet untouched). **Lifecycle — affected and now answered structurally:** a default-running run keeps the default's counted units as its recycle denominator; a departing run's declaration **must name its counted unit**, so F47's cadence exemption stays the exception rather than becoming the default state of composed runs. Encoding rides the v7 authoring item.

## Review

**Sizing (2026-08-01):** lead stated weight — 6 decisions (4 `Confident` · 1 `Contested` D4 · 1 `Assumed` D6) · reality load high (97-fact map over ten doctrine files; load-bearing derived aggregates: per-command G counts, G=0 ceiling arithmetic, driver cost anatomy from the persisted transcript) · blast radius maximal by D5's own factors (first-of-kind, whole command library + loop-discipline + audit skill) — and estimated cost (pair ≈ 200–350k tokens, single ≈ half, none = waiver). Recommended **pair** against the heavyweight default; **user ruled: pair.** Lens split: decision-quality + record-integrity; verify-pass owner: the record-integrity reviewer. The record is frozen from reviewer spawn until every disposition lands (Review section excepted).

**Cold reads (2026-08-01):** both formed independently, mutually withheld — decision-quality **23 raised** (8C/13I/2M) · record-integrity **18 raised** (3C/10I/4M + 1 fact-route). Transport fault at cross-exam open: decision-reviewer's message 1 was emitted as plain text and never crossed the seat boundary; integrity-reviewer, seeing nothing, delivered its 18 to the lead undebated (held to the solo bar, sequestration unbroken). Repair: message 1 re-sent, the one-shot four-message exchange proceeding — a retry of a lost send, not a second exchange. *Observed transport evidence (2026-08-01, logged for the trail, corrected same day): the "lost" send was a real `SendMessage` call, receipt success, correct routing — and it was **delayed, not lost**: it landed in the recipient's inbox a full nudge-cycle later, in the same turn as the lead's probe. Corrected conclusion: sender receipts prove neither delivery nor timeliness; seat-to-seat hand-offs in this run are confirmed by the recipient's next message. The resend produced a benign duplicate, flagged as such by its sender.* The fact-route item (the map's bounds-column counting rule, brainstorm/setup rows) went to the checker in parallel — one dispatch, one fact — and **resolved 2026-08-01: checker error owned, corrected figures landed as Map errata** (brainstorm 5 · setup 8 · total 26; C-e "four caps"→five; two new [cuts against] facts F52-a/F60-a volunteered). Survivors + per-reviewer tallies land at exchange close; the cross-set merge is the lead's.

**Exchange trail:** msg 1 delayed-then-delivered (resend benign-duplicate) · msg 2 sent, defensively resent with errata addendum · msg 3 delivered (six attacks on the integrity set: M4 withdrawn under attack; six decision-set findings amended, two sub-limbs withdrawn, one severity raised M2→I14) · msg 4 closed the exchange (1 withdrawal, 6 amendments). **Decision-quality set: 23 raised, 23 survived (8C/14I/1M), zero refuted, `critical-gaps`.** **Record-integrity set: 17 findings + 1 fact-route (own accounting correction from "18 raised"); M4 withdrawn, route resolved via errata; 16 survived (3C/10I/3M), `critical-gaps`.** Zero unresolved counterpart objections on either side; the one severity split self-resolved in debate.

**Lead merge (2026-08-01): 40 findings raised → 39 survived cross-exam → 31 lead-merged survivors (10C / 17I / 4M).** Seven agreed pairs merged (dC1+iC1 · dC4+iI10 · iC3+dI3 · dI4+iI4 · dI5+iI1 · dI11+iI5 · dI14+iI7); two lead merge-judgments: iM3 folded into iI2 as its worked case, dI12 demoted to Minor on its owner's own softest-in-set flag. Fallen items retrievable from either reviewer on ask.

| R# | Sev | From | Hits | Finding (compressed) | Disposition |
|---|---|---|---|---|---|
| R1 | C | dC1+iC1 | D1 D2 D5 D6(d) | commissioned counter-evidence unengaged: F76/F92–F96 in no rationale; census 5 map-cites across 6 decisions; against-set grew post-hoc (F52-a/F60-a); F96 base rate = 18/18 reviews found something, D1 lets a lead predict the never-observed zero case | *pending — folds + U-batch outcomes* |
| R2 | C | dC2 | D2 | all four floor invariants are presence-checks; worked driver-replay passes the floor with S1 never surfacing | **user → U1-A** |
| R3 | C | dC3 | D6(c) D2.2 | the check on the lead's own folds becomes lead-discretionary; D2.2's conditional binding is vacuously satisfiable — meta-self-grading | **user → U1-B** |
| R4 | C | dC4+iI10 | D5 | input-confidence inverted: the driver run's only Critical was created by a user-ruled input; factor scored on the upstream cards, not the graded artifact | **user → U3** |
| R5 | C | dC5 | D1 D6(a) | tier-3 composer may drop tier-1/2 validators; F9 cited against its own antecedent; machine-checkable bookkeeping linter never considered as the cheap first answer | **user → U1-C** |
| R6 | C | dC6 | D4 D6(d) OQ-1 | mixed-form interim has no valid grader in either ordering; audit needs an explicit dual-form branch keyed to a form declaration | lead fold → OQ-1 owns the dual-form branch |
| R7 | C | dC7 | D6(c) | a user-owned gate (review sizing, F79/F86) moves to lead discretion with no recorded supersession | **user → U4** |
| R8 | C | dC8 | D1 | strongest option never dealt: prescribed default + recorded deviation | **user → U2** |
| R9 | C | iC2 | Driver D1 D5 | composition claim uncomputed (8/20 items unclassified incl. the substantive round-1 FAIL); "all attached to a small-diff choice" false on the survivor table (S5, S7) | lead fold: classify all 20 or retract; restate D1's grounding |
| R10 | C | iC3+dI3 | D2.4 D6(b) | skip-trail has no baseline once D1 lands; D6(b) silently re-reads the invariant as "what ran" | **user → U2 outcome + reading ruling at folds** |
| R11 | I | dI1 | D2.1 D6(a) | the elevated user gate went 18/18 concordant in the driver evidence — an accountability invariant, not a detection one; carries the iC7-relocation note | lead fold: restate D2.1's claim honestly |
| R12 | I | dI2+iM2(num) | D3 | "~4 stops" requires deleting a ruling-generating stage; consolidation floor is ≈6 on the driver's own arithmetic | lead fold: restate or drop the numeral |
| R13 | I | dI4+iI4 | Driver D1(a) | electives billed to machinery (probe = the clean elective); "fixed machinery firing at full strength" survives its own correction note | lead fold: split the anatomy; retract "full strength" |
| R14 | I | dI5+iI1 | D2 D3 D5 | three `Confident` marks on unrecorded assent inside an unflagged adoption streak; M16-precedent repair | resolved structurally: U1–U4 re-open D2/D3/D5 content individually — each mark re-earned or downgraded at fold |
| R15 | I | dI6 | D6(a)↔D6(b) | absence-of-proof inverted for the grading dimension by the set claiming to preserve it | lead fold with R10's reading ruling |
| R16 | I | dI7 | D1 D4 | v0.34.0 trigger's terms voided, re-specification unowned; "bounds held" near-unfalsifiable against silently re-declarable bounds | lead fold → D4/OQ-1 obligation + R20's counter rule |
| R17 | I | dI8 | OQ-2 | D1 falsifies F30's constant-at-authoring premise; `workflow-contract.md` (F25's surviving scope) is the designed carrier for the declaration | lead fold → OQ-2 proposed answer |
| R18 | I | dI9 | OQ-2 D2.3 | composed process + bounds live only in lead context; F88's resume tax makes that a measured hazard; Recovery needs counter state | lead fold with R17 |
| R19 | I | dI10 | D5 | worked example binds nobody yet is the teaching instrument; relocates the driver's review post-authoring, never testing whether it still catches S1 | lead fold: state example status + the honest unknown |
| R20 | I | dI11+iI5 | D2.3 D6(e) | re-declaration loophole (raise the bound before busting it); no counter designed; the 2.8× silent probe overrun as behavioural counter-evidence; whether cost is part of the declaration — unruled | **user → U1-D** |
| R21 | M | dI12 (demoted) | D2 D5 | "near-zero cost" asserted unmeasured; composition cost lands in the longest-lived pane | lead fold: recorded-open estimate obligation |
| R22 | I | dI13 | D1 D5 | floor soundness becomes model-dependent; mochiko distributes to leads the record never reasons about | **user → U2 (audience-split surface)** |
| R23 | I | dI14+iI7 | OQ-4 | composed runs dissolve the lifecycle denominator; F47's exemption fires by default; re-opens the 836k-pane failure with doctrinal blessing | lead fold: split OQ-4; re-key obligation for v7 |
| R24 | I | iI2+iM3 | provenance | the ruling-bearing transcript is unpersisted while supporting evidence is; D4's user rationale is unquoted (the worked case) | lead fold: persist the ruling excerpt to `inputs/`; quote or mark D4's rationale |
| R25 | I | iI3 | KM | index entry and record status line both contradicted review state | **resolved — folded 2026-08-01 (both edits landed)** |
| R26 | I | iI6 | D4 OQ-1 | F77's version-line requires "date + ruling source"; for P7/P8 no ruling source exists because OQ-1 declines to make one | lead fold with R6 |
| R27 | I | iI8 | OQ-3 | primitive-edit ceremony never named for edits to 4 shipped primitives + 6 commands; F39 consumed by nothing | lead fold: replace OQ-3 with the named obligation set |
| R28 | M | dM1 | D6(a) | jurisdiction exceeds evidence: loop-discipline governs non-command loops D1–D5 never reason about | lead fold: scope D6(a) |
| R29 | M | iM1 | Mode line | "the six rulings … card-arbitrated" contradicts D6's own not-user-ruled mark | lead fold: qualify to five + one carried |
| R30 | M | iM2 | D3 | no Rationale section at all — unattackable by construction | lead fold: add the quoted basis |
| R31 | I | iI9 | D6 | ratification set heterogeneous: (a) doctrine surgery · (d) arithmetically forced · (e) an open question restated | lead fold: (a) gets its own card at acceptance; (e) moves wholly into OQ-2 |

**Bound-integrity cluster note (both reviewers, agreed):** R16 + R20 + iI5 share one repair — a counter, a rise-only-at-user-checkpoint rule, and re-declaration itself recorded. One fix closes the cluster.

**U-batch rulings (2026-08-01, user, four cards):**

- **U1 (floor hardenings, multiSelect)** — adopted **A** (the lead's weight read + composed process is a run-start user-ruled card), **B** (the lead's folds / lead-penned record always get one cold-seat grade; verify pass non-discretionary where a review ran; zero-cold-read shipping only by recorded waiver at the weight card), **D** (bound integrity: lead-counted counter · a bound rises only at a user checkpoint · re-declaration itself recorded). **Declined C** (F7 tier-ranking as floor + linter obligation): validator-tier selection stays lead judgment; the bookkeeping linter is recorded as a non-blocking build suggestion. → R2, R3, R20 user-ruled/adopted; **R5 user-ruled/declined — the D6(a) tier element marks `Contested`** per protocol.
- **U2 (D1 shape)** — **default + recorded deviation**: commands state a default pipeline (today's gate lines, seats, bounds survive as that default); the lead departs at will, each departure one trail line; nothing obliges the default to run. → R8 dealt-and-adopted; restores the baseline R6, R10, R18, R23 key on. **R22 dispositioned user-ruled-via-U2** *(landed at verify B1)*: the audience-split option was dealt on this same card and not chosen — the uniform stated default is itself the answer for distributed/weaker leads: an external run gets the full default pipeline unless its lead deliberately departs past the weight card and the recorded-departure stop. Named watch item: the first external dogfood run grades this in practice.
- **U3 (D5)** — split the input-confidence factor (ambiguity discount ≠ consistency risk; scored on the artifact under review); worked example re-derived. → R4 user-ruled.
- **U4 (sizing gate)** — review sizing passes to the lead **by recorded supersession** of the brainstorm-v2-2 ruling; the supersession is logged at the landing under the primitive-edit ceremony (R27's set). → R7 user-ruled.

**Fold pass (2026-08-01, lead pen — the verify pass grades these):** R1 engaged in D1's amendment · R9 + R13 corrected in the Driver · R11 folded into D2 · R12 + R30 into D3 · R6/R16/R26 + M3-provenance into D4's fold block · R19 into D5's amendment · R15 resolved by U2's unified trail reading (D2.4 ≡ D6(b)) · R17/R18/R31(e) into the rewritten OQ-2 · R23 into the rewritten OQ-4 · R27 replaces OQ-3 · R28 scopes D6(a) · R29 qualifies the Mode line · R24 persisted (`inputs/2026-07-31-questioning-session-rulings.md`) · R21 recorded-open: a cost estimate for declaration + trail + composition rides the v7 build item — *scope contingency (N6, post-B7): if the cost-as-bound clause ratifies at acceptance, R21's residue is only the unmeasured estimate; if it does not, the cost dimension carries no floor invariant at all and R21's residue includes that gap* · R25 already landed · R31 restructures D6's ratification (below). **R14 disposition:** satisfied structurally — U1–U4 re-opened D2/D3/D5 content individually; each mark is re-earned by its own card, not by the original streak.

**Verify pass:** round 1 (2026-08-01) **NOT CLEAN — closing figures 7 blocking (B1–B7) · 6 non-blocking (N1–N6)**, the addendum grade included (B8 corrected these head figures — they briefly read 6+5, understating the round this block exists to record; same defect class as R25, caught recurring inside the block written to prevent it). 12+ folds verified clean with quoted evidence. Blocking pattern: amendments appended while superseded Statements stood (B2 D1 · B3 D3 · B4 D5 — repaired with in-place strike/supersession markers per the R13 inline pattern the verify itself named as the model) · B1 R22 had no landed disposition (now user-ruled-via-U2, audience-split dealt-and-not-chosen, external-dogfood watch item) · B5 F95 unengaged in R1's fold (clause added — premise-reversal capacity survives via default reviews + U1-B) · B6 the folds re-broke the index About/Artifacts (propagated). Non-blocking N1–N5 landed same round (residual-four named · D2 inline markers, card text preserved · R21 obligation surfaced in OQ-1 · D6 mark names its internal `Contested` element · the verbatim quote's terminal period moved outside the quote); **N6 deferred by design** pending B7's ratification routing, then landed as the R21 scope contingency; **B7 repaired via branch (b)** — the cost-as-bound clause demoted to lead-inferred-pending-ratification with its own acceptance-card line. **Round-1 addendum (crossed the repairs in flight):** the verify owner graded the lead's four exchange-close deltas — two confirmed clean (the C7(ii) relocation defence nowhere in the file, "the ever-present external check" fully propagated out; the bound cluster one-fix-three-consumers with no drift) and one new blocking item: **B7 — the cost-as-bound settlement was lead-inferred presented as user-adopted** (the U1-D card of record carries no cost clause; R20's own row says "unruled"; the D2 fold said "settled here") — the provenance pattern R14 exists to catch, inside the fold-set that closed R14. **Repaired via branch (b):** the clause demoted to lead-inferred-pending-ratification with its ground and live instance stated; it takes its own acceptance-card line. N6 (contingent) landed post-B7 on R21's scope. **Round 2 (re-grade):** NOT CLEAN — all eleven B1–B6/N1–N5 repairs **verified clean with quoted evidence** (B2, B5, N2 graded better-than-specified); **B7 verified carried-unrepaired against the file as read (quoted), B8/N7 raised.** *(B9 — raised, then withdrawn by its owner at round-3 close. The reconciled account, in the reviewer's own suggested phrasing: the branch-(b) repair and N6's contingency, made in response to the round-1 addendum, landed in the window between the reviewer's greps and its verdict's delivery — already on disk when the round-2 verdict arrived, the reviewer's read predating the repair. Both accounts are true of their own moment; the withdrawal was the reviewer's, on the ground that its evidence proved read-time state, not verdict-time state.)* B8 (this block's head figures) and N7 (the status line trailing the index) repaired at round-2 close.

**Round 3 (bounded re-read):** every substantive item CLEAN — B7's branch-(b) text verified fully ("the strongest form the repair could take"), B8 both halves, N6 correctly sequenced, N7 agreeing on all three surfaces, zero regressions across B1–B6/N1–N5. **Verify pass CLOSED: CLEAN (round 3, 2026-08-01).** B9 withdrawn by its owner, so the fallibility ledger reads symmetrically: two lead bookkeeping overstatements (R25 · B8) and one reviewer over-reach (B9), **every one caught by the other side's read, none by its author** — the session's own trail now carries the two-sided case for U1-B's non-discretionary cold grade and for D2.4's trail needing a grader, not only a writer. Across the pass: 31/31 dispositions verified landed · 7 blocking + 6 non-blocking repaired or routed · one cross-exam withdrawal (M4) · one verify withdrawal (B9) · one fact-route resolved with the checker owning the error · three repairs graded better-than-specified (B2, B5, N2). **One item travels to acceptance and is not the review's to settle: B7's cost-as-bound clause, lead-inferred, its own card.**

**Acceptance (2026-08-01, user, four cards A1–A4):** **A1 — D6(a) ratified as amended** (own card per R31, the internal `Contested` tier-decline shown) · **A2 — OQ-2 ruled: the proposed answer adopted** — `workflow-contract.md` revived as the per-run carrier, instantiated only on departure or non-default bounds; default runs declare in one line on the deliverable; Recovery gains the counter-state row · **A3 — B7 ratified: declared cost ranges ARE floor bounds** (counter, rise-only-at-user-checkpoint); N6's contingency resolves to the narrow branch — R21's residue is the unmeasured estimate only · **A4 — record ACCEPTED**; D6(b)/(c)/(d) ratified with the set. Landing ritual run same day: DECISIONS.md row · BACKLOG "Lead-owned process flexibility build (shape v7)" opened, absorbing the goal-shape read-drop residual per R16 · ROADMAP Now swap · index closed. Synthesis: on request only; the verify owner samples its fidelity before it ships.
