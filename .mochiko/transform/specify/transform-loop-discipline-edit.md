# Transform (realized trace) — `loop-discipline` Gap Classification fold, cluster `specify`

Run: `transform-cluster specify` · Phase 3 (transform-recipes) · Step: **apply a finalized, human-gate-ACCEPTED EDIT to an existing shared primitive**
Producer: `transform-producer` · Skill: `mochiko:transform-recipes` (body `port-with-edits`, additive; structural = the `dedupe`/fold landing for the dissolving strategy skills)
Applied: 2026-06-27 · Authoritative inputs: `reconcile.md` §A Agenda 3b, §B.4 (Gap Classification survivor), §C (the one EDIT to a shared primitive), §D (row "`skills/loop-discipline/SKILL.md` — EDIT"), §E P10 R8/R9 + P9 (c) + P4 b4, §F gate-item 3; `assess-strategy-core.md` (R8/R9 the one additive survivor); `assess-strategy-specification.md` (c — Research-Before-User overlaps R8); HIL sources `strategy-core/SKILL.md` + `strategy-specification/SKILL.md` (taxonomy wording preserved)

> **Scope of this artifact.** This is the realized trace for the **single additive edit** to the shared doctrine skill `loop-discipline` — the canonical home into which `strategy-core`'s Gap Classification (P10 R8/R9) and `strategy-specification`'s Research-Before-User restatement (P9 c) **dedupe/fold**, so the same doctrine is never authored twice. It is NOT a new primitive (§C: zero new primitives this cluster) and NOT the router edit (separate; `transform-router-edit.md`).
>
> **Human-gate accepted.** Folding Gap Classification into `loop-discipline` (vs. keeping it as specify-lead prose) was the Phase-2 gate Q1 decision → "Fold into loop-discipline" (§F item 3). This step applies that accepted disposition; it does not re-decide it.
>
> **Not graded here.** Independence is non-negotiable: this edit + trace are handed to `verify-output` (run by a different agent) for the Phase-4 grade. No self-grade performed.

---

## TRANSFORM: loop-discipline — Gap Classification fold

```
Applied:   port-with-edits × (dedupe/fold landing) + wiring-pass(n/a — already classified/routed)
Artifacts: plugins/mochiko/skills/loop-discipline/SKILL.md (EDITED — additive only)
New partners: none (no split/promote; doctrine folded into an existing shared skill)
Wiring:    classification=unchanged (already model-invoked doctrine skill)
           router=unchanged (already registered in the Doctrine section; NOT re-registered)
           triggers=unchanged (frontmatter description left intact)
           rebinds=none (no paths/namespaces touched)
```

## What was edited — 3 additive, non-overlapping insertions

One file: `plugins/mochiko/skills/loop-discipline/SKILL.md`. No existing requirement was rewritten; the four requirements remain four. The four-requirement invariant ("You cannot rationalize your way out of any of the four") is preserved — the new content is explicitly framed as a **refinement of requirements 3 and 4, not a fifth requirement** (mirroring the existing "Briefing the agents — a guide, not a fifth gate" precedent in the same file).

1. **Pointer appended to Requirement 3.4 ("Escalate, don't silently die")** — one sentence: escalation is not uniform; route each finding by gap type; only genuine judgment calls reach the human. (Extends Req-3 prose; references the new section "below".)
2. **Pointer appended to Requirement 4's `preference-gap only` placement bullet** — a parenthetical tying that placement to the *preference gap* type and noting knowledge/scope gaps route elsewhere. (Extends Req-4 prose; references the new section "below".)
3. **New `## Routing a FAIL by gap type` section** inserted between the end of the Four Requirements (`### 4. Defined human gate`) and `## How to apply: fill in the contract` — the canonical taxonomy + the cross-wiring corollary anti-pattern.

## The exact taxonomy wording (verbatim, as written into the skill)

> ## Routing a FAIL by gap type — a refinement of requirements 3 and 4, not a fifth requirement
>
> A FAIL is not one thing. When validation returns a finding — or the loop stalls short of its done-condition — *what you do with the finding* depends on the **kind of gap** it is. Sending every finding to the human (requirement 4) spends the human on questions a machine could settle; iterating the loop on every finding (requirement 3) grinds on gaps no round will ever close. Route each finding by its type:
>
> - **Knowledge gap** — a factual unknown an investigation could settle (e.g. "which protocol does the existing system use?", "what does this artifact already contain?"). **Route to research** — a native `Explore` pass or equivalent investigation — not to the human. The loop resolves it and continues; this is the cheapest gap to close.
> - **Preference gap** — a judgment or taste call only the human can make (e.g. "opt-in or opt-out?", "what threshold is acceptable?"). **Route to the human gate.** This is the genuine-judgment escalation that requirement 4's *preference-gap only* placement is named for — research cannot manufacture an answer to "should we".
> - **Scope gap** — the work is bigger or different than it was framed (e.g. "this spans three separate concerns", "the goal conflicts with a stated constraint"). **Halt or split** — do not keep iterating. A round cap will never converge on a target the loop was never scoped to hit; surface it through requirement 3.4's escalation, with the failure context.
>
> **The corollary — never cross the wires.** A preference question sent to research comes back empty: investigation cannot answer "should we". A knowledge question sent to the human burns the scarcest validator you have on something an investigation would have settled. Routing a gap to the wrong sink is a defect, not a shortcut — the wasted round is exactly the one requirement 3's bound exists to catch.

Pointer text added into the requirements:
- **Req 3.4:** "Escalation is not uniform — route each finding by gap type (see *Routing a FAIL by gap type* below): only genuine judgment calls reach the human."
- **Req 4 bullet:** "(a *preference gap*; knowledge and scope gaps route elsewhere — see *Routing a FAIL by gap type* below)"

## Taxonomy provenance — wording preserved, stance hardened

The three gap types + routing + corollary are preserved 1:1 in meaning from the HIL sources; only domain-flavoring was genericized and the advisory stance was dropped.

| HIL source (verbatim concept) | Mochiko rendering | Change |
|---|---|---|
| `strategy-core` **Knowledge gaps** "resolvable through targeted research without involving the user"; ex. "What auth protocol…", "current database schema" | **Knowledge gap → route to research** (`Explore`); ex. "which protocol…", "what does this artifact already contain?" | genericized examples (drop `auth`/`database schema`); added native `Explore` per task |
| `strategy-core` **Preference gaps** "require user input — research cannot resolve what only humans can decide"; ex. opt-in/opt-out, latency threshold | **Preference gap → route to the human gate**; ex. "opt-in or opt-out?", "what threshold is acceptable?" | genericized (drop "latency"); bound to Req-4's existing *preference-gap only* placement |
| `strategy-core` **Scope gaps** "may warrant halting or splitting rather than iterating"; ex. "spans three bounded contexts", "conflicts with an architectural constraint" | **Scope gap → halt or split**; ex. "spans three separate concerns", "goal conflicts with a stated constraint" | genericized (drop "bounded contexts"/"architectural"); tied to Req-3.4 escalation |
| `strategy-core` anti-pattern **Preference-to-research** + `strategy-specification` **Research-Before-User** / **User-for-researchable** | **The corollary — never cross the wires** (both directions: preference→research empty; knowledge→human wastes the validator) | merged the sibling restatements into one corollary (P9 c dedupes into P10 R9 here) |
| `strategy-core` Rationale: "researching preferences yields nothing, asking users about codebase facts wastes their time, iterating on scope … never converges" | carried into the intro + corollary, voiced as doctrine | "codebase facts" genericized to "what an investigation would have settled" |
| `strategy-core` L8: "advisory patterns… not prescriptive… the Supervisor… may deviate" | **NOT carried** (per §E P10 R10 `dropped + reason`) | hardened to `loop-discipline`'s non-negotiable framing |

## Keystone-test confirmation — every added line is workflow-agnostic (0 specify tokens)

Keystone test applied per added line: *would this be true of the loop-discipline doctrine on ANY mochiko loop?* All added lines pass — the gap router is a general FAIL-routing rule, true of any produce→check→repeat loop.

Deny-list / specify-token grep over the whole edited file (`advocate|analyst|specif|requirements-analyst|devils|state-analyst|dispatch|mode[s]|workflow-agnostic|supervisor|may deviate|advisory|catalog|DAG|pass(es)|phase|INV-`):
- **Zero hits on any added line** for: `advocate`, `analyst`, `spec`/`specif`/`specification`, `requirements-analyst`, `devils-advocate`, `state-analyst`, `dispatch`, `mode`/`modes`, `phase`, `workflow-agnostic`, `supervisor`, `may deviate`, `advisory`, `catalog`, `DAG`, `INV-`. (Pre-existing hits for `supervisor`/`dispatch`/`workflow-agnostic` are all on untouched lines — the frontmatter, Req-3.1, and the "Briefing the agents" doctrine section.)
- **One benign hit on an added line:** the regex matched `pass` in "a native `Explore` **pass**" (Knowledge-gap bullet). This is the noun "a research run/pass," **not** the HIL DAG workflow-phase token ("pass 1 / after 5 passes" — which was deliberately excluded; the skill uses "round", and I used "round" throughout). Not a coupling token.

Per-element universality check:
- **Gap types** (knowledge / preference / scope) — universal: any loop can produce any of the three finding types.
- **Routing sinks** (research / human gate / halt-or-split) — universal: research = native `Explore` (a platform-native capability available to *every* mochiko loop, the opposite of a workflow-specific sibling agent — and the task-named example); human gate = Req-4 (already universal); halt/split = Req-3 escalation (already universal).
- **Examples** — all generic decision/fact/scope illustrations; none names a spec artifact, agent, mode, path, or phase.
- **Cross-references** — only to "requirement 3"/"requirement 3.4"/"requirement 4" and "round"/"loop"/"done-condition"/"validation"/"human gate"/"validator" — all existing universal loop-discipline vocabulary.
- **No agent names**, **no "dispatch"**, **no modes/paths/phases**, **no "workflow-agnostic"/declaration meta-label**, **no "advisory/may-deviate" stance**, **no kernel/DAG/catalog/MCP**.

Voice + anti-rationalization framing preserved: doctrine imperative ("Route each finding by its type", "do not keep iterating"), and the corollary closes in the skill's established cadence ("a defect, not a shortcut" ≈ "Speed is not soundness").

## Trace (realized) — the dedupe/fold tags this edit lands

This edit is the **landing site** for the relational tags assigned in `reconcile.md` §E. Realized:

- **P10 `strategy-core` R8** — Gap Classification taxonomy (knowledge→research / preference→user/human-gate / scope→halt-or-split) → **folded-into `loop-discipline`** (additive). ✔ landed (the new section).
- **P10 `strategy-core` R9** — anti-pattern Preference-to-research (corollary) → **folded-into `loop-discipline`** (the "never cross the wires" corollary, both directions). ✔ landed.
- **P9 `strategy-specification` (c)** — "Research Before User" + Uniform-gap-treatment / User-for-researchable → **dedupe** into the same taxonomy (not authored twice). ✔ landed (subsumed by the one corollary + the three-type routing).
- **P4 `state-analyst` b4** — gap-classification judgment → **moved-to-lead + dedupe** (the doctrine lives here in `loop-discipline`; the specify lead *applies it by consuming `loop-discipline`*, which it already does). ✔ doctrine home realized here; lead-side application is P1's body port (separate).
- **P10 R10** ("advisory / may deviate" stance) — `dropped + reason` (superseded by non-negotiable framing) → **honored**: not carried into the fold. ✔
- **P10 R11 / P9 framing** ("consumed by State Analyst → Supervisor briefings") — `dropped + reason` (kernel-era consumer dissolves) → **honored**: zero consumer/agent framing in the added text. ✔

No responsibility from the §E rows above is left homeless; no new responsibility was invented.

## Deviations / notes (for the grader)

1. **New `##` section vs inlining into the requirements.** The taxonomy is ~6 lines + a corollary; inlining it would have *rewritten* Req 4 (forbidden by the task) and split the corollary awkwardly across two requirements, risking authoring it twice. A single canonical section, referenced by two minimal pointers, keeps the four requirements pristine and authors the doctrine once. Adding such a refinement section is additive, not restructuring — and has direct in-file precedent ("Briefing the agents — a guide, not a fifth gate").
2. **Scope deliberately confined to Req-3/Req-4.** The task scoped the extension to "the existing Req-3/Req-4 prose." I therefore did **not** add rows to *Red Flags* or *Common Rationalizations*, even though a cross-wiring red flag would fit the skill's idiom — that would extend other sections beyond the instructed scope. The anti-rationalization framing is instead carried *inside* the new section's corollary paragraph.
3. **`Explore` named once, as an example of the universal category "research/investigation".** Phrased "a native `Explore` pass or equivalent investigation" so the rule does not hard-depend on one tool — `Explore` is the task-named, platform-native illustration, not a workflow sibling. Keystone-passes (any mochiko loop can route a knowledge gap to Explore).
4. **"round" not "pass".** HIL's gap doctrine lived among "pass 1 / after 5 passes" phase vocabulary; I used `loop-discipline`'s existing "round" term throughout, so no DAG-phase token entered the skill.

## Scope guard

- Only `plugins/mochiko/skills/loop-discipline/SKILL.md` was edited. The router and every other file were left untouched (per task).
- Additive only: no existing requirement rewritten; the four-requirement count and the "you cannot rationalize your way out of any of the four" invariant are intact.
- No kernel/DAG/catalog/MCP path or vocabulary introduced; no "advisory/may-deviate" stance carried.
- This artifact + the verbatim taxonomy + the keystone scan stand alone for `verify-output` (decoupling scan + trace audit) without needing to consult the producer.

---

**Trace version:** v1 · **Governed by:** `loop-discipline` (the skill being extended) · **Next:** `verify-output` (independent grade, different agent) — decoupling scan (0 specify tokens) + responsibility-trace audit (R8/R9/c/b4 landed, R10/R11 honored as drops).
