# Mochiko Roadmap — v2 (Re-baselined)

_Rewritten: 2026-06-27. Supersedes the 2026-06-26 brainstorm synthesis (kept in git history)._

**Why this is a rewrite, not an amendment:** `agent-skills-research/synthesis/my-framework.md` was refreshed on 2026-06-27 — one day after roadmap v1 — with foundational changes to *how skills, agents, and workflows must work*:

- a new **sound-loop technique cluster** (`external-grounded-validation`, `pre-declared-done-condition`, `bounded-agent-loop`) that imposes structural requirements on every workflow;
- three `maybe`→`adopt` upgrades, including **`deterministic-core-llm-shell`**, which makes v1's *"Confident: kernel-free"* decision no longer honest as written;
- a new **open question #11** — *is the human a first-class external validator?* — flagged by the synthesis red-team as "the glaring omission for a framework literally named human-in-loop."

These touch the thesis and a Confident decision, so they can't be patched in as caveats. This document re-baselines around them.

---

## What changed since v1

| Disposition | Item |
|---|---|
| **Survives intact** | Discipline lives in the skill library, not a kernel · skills/agents are the primary building block · workflow-first build order · `setup` then `specify` |
| **Net-new doctrine** | The sound-loop cluster — every workflow is now a *constrained loop* with a pre-declared done-condition, external/independent validation, and a bounded iteration cap |
| **Re-centered** | The human is promoted from absent to **central thesis**: mochiko's primary external ground-truth validator |
| **New convention** | **Producer↔validator skill pairing** at artifact granularity (a fifth skill-library axis) |
| **Superseded** | v1's *"Kernel architecture: kernel-free — Confident"* → reframed as **deferred, code-free until dogfooding** (reasoning preserved below, not deleted) |

---

## Thesis (re-centered)

Mochiko is the v3 successor to [human-in-loop](human-in-loop/). It delivers human-in-loop's engineering discipline **without its Python/MCP deterministic kernel** — discipline lives in the quality of the skill library, and native Claude Code agent teams + Workflows handle orchestration. Skills and agents are the primary building block; orchestration is the layer on top, not the enforcer.

The re-centering, drawn from `external-grounded-validation` and open question #11:

> **A loop only improves an output when its validator is external to and independent of the agent being checked. Where nothing is machine-checkable, the human is the cheapest external ground truth — so in mochiko, the human is the framework's primary external validator, present by design in every workflow loop.**

The name finally means something structural, not just lineage: *human-in-loop* = the human sits inside the validation loop as ground truth.

---

## The sound-loop doctrine (the foundational layer)

Every mochiko workflow is a **constrained loop**, not freeform generation. Without a kernel to enforce this, the discipline is carried by a `loop-discipline` doctrine skill (the rules + anti-rationalization prose) and a `workflow-contract` markdown template that each workflow instantiates. Every workflow MUST satisfy four things:

1. **Pre-declared done-condition** (`pre-declared-done-condition`) — a machine- or human-checkable success condition written *before* the loop runs, **defaulting to FAIL**. "Done" cannot drift to "the agent got tired."
2. **External, independent validation** (`external-grounded-validation`) — the agent that produced the work never grades its own output. Validation is run by a *different agent* using a *different skill* (see the producer↔validator convention). The lead/referee owns the verdict.
3. **Bounded iteration** (`bounded-agent-loop`) — a deterministic round cap, a no-progress exit, and an escalation path. No LLM-judged "I'll stop when it's good."
4. **A defined human gate** (open question #11) — every workflow's contract names *where the human validates* (every cycle / only on low validator-confidence / only on preference-gaps). Placement is a per-workflow call; *presence* is non-negotiable.

These are inspectable: a workflow's filled-in contract shows whether its validator is genuinely independent and where its human gate sits.

---

## Key Decisions

| Decision | Choice | Confidence | Rationale |
|----------|--------|------------|-----------|
| Central thesis | Human as primary external validator | Confident | Resolves OQ#11; gives the successor-to-human-in-loop a structural spine, not just a name |
| Kernel / code | **Deferred — code-free until dogfooding** | Confident | `deterministic-core-llm-shell` is now `adopt` *and heavy*; the synthesis says it earns its weight only where work is genuinely parallel/dependency-rich (`implement`). Build kernel-free first, dogfood `setup`+`specify`, let real drift — not theory — trigger any code. (Supersedes v1's "kernel-free, Confident.") |
| Loop discipline | Doctrine **skill** + workflow-contract **template**, zero code | Confident | Skill-library-native; composes with `single-source-rule-fanout`, `enriched-instruction-contract`, `anti-rationalization-scaffolding`; keeps the kernel out while staying inspectable |
| Validation mechanism | **Producer↔validator skill pairing** at artifact granularity | Confident | Concrete kernel-free form of `external-grounded-validation` — independence on two axes (different agent + different skill); already human-in-loop's latent pattern (`authoring-X` ↔ `validation-X`) |
| Agent↔workflow decoupling | Personas are workflow-agnostic by **absence** (self-contained professional; no sibling/dispatch/workflow vocab), not by declaration; independence is structural; one generic `validator`; caller-side context in `agent-dispatch.md` | Confident | A persona that *declares* independence but carries workflow-shaped inputs re-couples in disguise. Decoupling-by-absence + the keystone test make it grep-checkable. See `AGENT-DECOUPLING-SYNTHESIS.md` (D1–D4, O1–O5) |
| Primary quality surface | Skill library + agents | Confident | Discipline injected through quality primitives, not plumbing (unchanged from v1) |
| Build approach | Workflow-first, one at a time | Confident | Each workflow teaches what the conventions actually need (unchanged from v1) |
| Build order | `setup` first, then `specify` | Confident | Setup establishes the constitution (standards root); specify proves the adversarial pair (unchanged from v1) |
| Migration tracking | Explicit registry with ported / not-yet status | Confident | Nothing falls through the gap (unchanged from v1) |
| Human-gate placement | On gated dispositions + escalations, plus a named acceptance gate on the deliverable | Confident | Resolves OQ#1; ran cleanly across `setup` + `specify` — routine PASSes run unattended, gated dispositions/escalations stop. (2026-06-27) |
| Memory model | In-session + workspace-as-state under `.mochiko/` (no separate context-handoff file) | Confident | Resolves OQ#3; `setup` + `specify` both dissolve the HIL context carrier into the lead; state recovery reads workspace evidence, not a context `phase` field. (2026-06-27) |
| Gap Classification (FAIL-routing) | Folded into `loop-discipline` (knowledge→research / preference→human gate / scope→halt) | Confident | Universal FAIL-routing a real workflow (`specify`) needed now; hosting it per-workflow would re-author the doctrine; keystone-tested workflow-agnostic. (2026-06-27) |
| Command altitude / single-sourcing | A command stitches a team to a goal under a contract — it *references* shared doctrine, never restates it; enforced by `verify-output`'s altitude scan | Confident | The first ports re-inlined `loop-discipline` (a `single-source-rule-fanout` violation); fixed at the recipe (assess→`dedupe`, transform→thin-redesign + wiring step 6, verify→altitude gate) and retrofitted `specify` (329→66) + `setup` (385→78), both independently verified. See `COMMAND-ALTITUDE-SYNTHESIS.md` (2026-06-30) |
| `implement` orchestration + deferred kernel | Sequential-first, thin lead (Task-subagent dispatch); parallel TDD slices + any kernel/DAG deferred to dogfooding | Confident | Resolves OQ#5; HIL itself is sequential, parallelism was never realized; consistent with 4 prior thin ports + the "code-free until dogfooding" decision; deferral logged as a shortcut-ledger entry. (2026-07-01) |
| `brainstorm` workflow (first net-new) | ~~Two artifact-grounded advocate gates + emergent exit ramp with per-ramp handoff~~ **Superseded by brainstorm v2 (2026-07-04, next row)** after the first dogfood run: the adversarial substance earned its cost; the freeze/dispatch/gate ceremony made the session illegible to the human it exists for | Superseded | Original rationale preserved in git history + `.mochiko/brainstorms/brainstorm-command/synthesis.md`; behavioral evidence for the supersession in `.mochiko/brainstorms/setup-workflow-rewrite/dogfood-handover.md` (branch `brainstorm-dogfood-setup-rewrite`). (2026-07-02 → 2026-07-04) |
| `brainstorm` v2 — the agent-team pilot (sound loop in team form) | A minimal framework on **native agent teams** (hard-require, `Contested`, no fallback transport): lead runs the questioning inline and writes `record.md` as-you-go (challenge trail + confidence marks); an **advocate** teammate challenges episodically at decision moments; a **grounder** teammate is a conditional seat (topic has a reality surface; reports what IS, never what SHOULD BE); a **pressure-tester** spawns cold at convergence and audits the record *and the advocate* (coverage, losses) with a sequestered cold-read-first protocol, then one verify pass. Human = answer-owner routing + tie-breaks (the preference-gap placement) + final acceptance. The four requirements map natively: done = every PT finding dispositioned + user accepts (reachable, default-FAIL); validator = the cold PT (never in the room — stronger than file-freeze independence); bounds = review+verify, no-progress→tie-break; contract filled once at authoring time in the command (a per-run form whose values are constant is ritual, not proof). Deliverable = the record; pipeline entry is an offer, never a ramp | Confident *(engagement model revised → v2.1, next row)* | The pilot for team-form workflows: if the v2 dogfood succeeds, specify/plan/tasks/implement inherit the mapping (their producers/validators as teammates under the same four-requirement translation); if it fails, the substrate item closes toward `Task`-subagents with evidence. Kills the dogfooded failure (ceremony: 7 phases, 5 gates, per-run contract, freeze artifacts, 4 ramps, five ID namespaces, an unreachable done-condition) while keeping the dogfooded value (adversarial challenge while thinking is fluid + cold independent review + confidence-mark vocabulary). Edit surface: 1 command rewrite + 1 skill rewrite + 1 checklist swap + 1 `analysis-iterative` format + retirements (tasks entry variant, handoff-brief template). Design record: `.mochiko/brainstorms/brainstorm-command-rewrite/record.md`. (2026-07-04) |
| `brainstorm` v2.1 — the end-stage review pair (first-run verdict folded in) | The session is the lead + the user (+ conditional grounder); **all adversarial pressure moves to convergence**: two cold reviewers (`devils-advocate` × `validation-brainstorm`, each's counterpart withheld until its independent findings are formed) cross-examine each other and return only the **survivors** with a tally ("N raised, M survived"); survivor routing (user territory → user · lead reasoning → two-exchange cap → tie-break · facts → grounder) → dispositions → one verify pass → plain-text acceptance. The lead's pen covers only its own formulation — nothing amends a user-ruled decision without the user's word. Retired: the standing episodic advocate, its engagement/liveness machinery, and the coverage/losses audit (the pair audits each other by construction) | Confident (user-ruled; mechanism un-exercised) | The first v2 run failed the user's-seat check — transcript forensics (not the run's self-report) measured 3:1 machine-to-user traffic with half the lead→advocate messages pure bookkeeping (each a full teammate turn), consent-free folds into user-ruled decisions behind unanswered objection windows, teammate↔teammate messaging used **zero** times, and cost above v1's ≈620k (I-4 closed; numbers in `BACKLOG.md`). The revision trades before-hardening catches (eyes open — v2's 7/7 engagement yield now arrives at review instead) for a session the human can follow, and is the first shape to exercise inter-teammate debate — the one capability distinguishing teams from `Task` dispatch. Record: `.mochiko/brainstorms/brainstorm-v2-revision/record.md`. (2026-07-05) |
| `slice` workflow / vertical graduation (substrate, not super-command) | Post-spec decomposition command (`/mochiko:slice` → a reviewed `slices.md` overlay; `spec.md` stays whole) + slice-scoped entry variants on `plan`/`tasks`/`implement`; foundation slice + extend-mode over single accumulating design artifacts; graded amendment for breaking changes; feature-done **declared at decomposition** (SC coverage map + seams), verified at feature-close (= the `audit` charter); `/mochiko:graduate` wrapper deferred to dogfooding | Confident | Both drivers (time-to-working-code, artifact focus) are served by the substrate alone — a graduation super-command adds only ergonomics (explicitly not a driver) and is the kernel-shaped commands-invoking-commands risk, so it waits for demonstrated shepherding pain. Axis-4 composition reuses both cross-workflow agents (`task-architect` gains `authoring-slices` — its cycle-slicing judgment one level up; `devils-advocate` gains `validation-slices`, its 5th mount), not a new agent; axis-5 keeps the pair independent (producer skill on task-architect, reviewer skill on the advocate). The downstream consumption rules live in the artifact's own **Graduation contract** (single-source; the three entry variants reference it, never restate). Null exit keeps small specs whole-spec. Edit surface: 1 command + 2 skills + 2 templates + 3 entry variants. Design record: `.mochiko/brainstorms/vertical-graduation/synthesis.md`. (2026-07-02) |

---

## Skill-library conventions (five axes)

Four structural axes carry over from the synthesis; the fifth is promoted from human-in-loop's latent pattern this rewrite.

1. **Classification** (`invocation-axis-taxonomy`) — every skill declares `user-invoked` or `model-invoked`; user-invoked may call model-invoked, never each other.
2. **Discoverability** (`router-skill`) — one user-invoked router indexes the rest with when-to-reach-each guidance.
3. **Reliable model-invocation** (`rfc2119-invocation-trigger`) — model-invoked skills encode graded MUST/SHOULD + exact trigger phrases in their `description`.
4. **Agent↔skill composition** (`skill-augmented-agent`) — agents declare a `skills:` list and lean on those skills when the work is in their domain. The **persona** carries self-sufficient method + judgment + standards + output-shape (competent even when no skill fits); the **skill** carries the exact, repeatable procedure. A persona contains **no trace of any workflow** — no sibling-agent names, no "dispatch," no modes/paths/phases, no "workflow-agnostic" meta-labels; decoupling is proven by that *absence*, enforced by the keystone test (*true of this professional on any job → keep; only-in-this-workflow → cut*). Caller-side context lives in `agent-dispatch.md`, never the persona. Details in [`AGENT-DECOUPLING-SYNTHESIS.md`](AGENT-DECOUPLING-SYNTHESIS.md).
5. **Producer↔validator pairing** *(refined)* — every reviewable artifact is graded by an independent validator, and independence is **structural**: a *different agent* running a *different skill* than produced it, guaranteed by the loop — never asserted by a persona declaring itself independent. **Form by artifact type:** a *mirror checklist* skill for objective acceptance criteria (e.g. `validation-constitution` checks Enforcement/Testability/Rationale); an *adversarial-critique* skill for judgment-based artifacts (e.g. `specify`'s advocate). Utility and inside-an-authoring-skill pattern skills get no partner (per `minimalism-decision-ladder`).

---

## Decision Trail

### Kernel-free → deferred (superseded, not deleted)

- **v1 chose:** shed the Python/MCP kernel entirely — "kernel-free, Confident."
- **v1 reasoning (preserved):** accidental complexity for sequential workflows; Claude agent teams solve the isolation problem at the platform layer.
- **What changed:** the synthesis upgraded `deterministic-core-llm-shell` to `adopt` (corroborated by Anthropic's verification hierarchy), so a blanket "kernel-free" is no longer accurate.
- **v2 chooses:** **defer**, don't reject. Build code-free now; dogfood; let concrete drift the lead can't catch by reading a contract be the upgrade trigger. The synthesis itself scopes the kernel's payoff to parallel/dependency-rich work (`implement`), so no kernel is warranted before `setup`/`specify` are real. Recorded as a `deliberate-shortcut-ledger` entry (see `BACKLOG.md`).

### Human-validator promoted to thesis

- **Considered:** central thesis / one belief among several / leave as open question.
- **Chosen:** central thesis, with gate *placement* left per-workflow.
- **Reasoning:** the synthesis red-team named the human's absence the glaring omission for this framework specifically. Promoting it resolves OQ#11 and gives mochiko a spine v1 lacked; placement stays a `setup`/`specify` discovery, consistent with workflow-first.

### Loop-discipline mechanism

- **Considered:** doctrine skill + contract template (hybrid) / skill only / hooks as hard gates.
- **Chosen:** hybrid skill + template, zero code.
- **Reasoning:** skill-library-native, inspectable, and composes with four already-adopted techniques. Hooks reintroduce the harness-specific infrastructure just shed — reconsidered only for `implement`, alongside the kernel question, after dogfooding.

### Producer↔validator pairing convention

- **Considered:** per produced artifact / literally every skill / two mechanisms by artifact type.
- **Chosen:** per artifact, with the two-form nuance folded in.
- **Reasoning:** gives independence on two axes (agent + skill); minimal (no validation skills that check nothing); already human-in-loop's latent pattern, now named as a law.

### Agent↔workflow decoupling — by absence, not declaration

- **First attempt (wrong):** thin the personas, then hand each a "workflow-agnostic" label and an "Inputs I require" interface. Implementing it, that interface filled with setup-vocabulary (`greenfield|brownfield|amend`, `.mochiko/memory/…`, a grader's fix-list) — hidden coupling, worse-disguised than the visible kind it replaced.
- **Chosen:** decoupling is proven by the *absence* of coupling. A persona carries only craft + the skills it leans on, names no sibling/dispatch/workflow vocab, and **degrades gracefully** (competent with a thin brief or an ill-fitting skill; asks for what it needs rather than inventing it). The caller (`agent-dispatch.md`) holds workflow knowledge; independence is structural (different agent + disjoint skills), not a persona trait. Skills get the same treatment (no agent names; independence stated by *role*) but stay self-sufficient since they run agent-less. The keystone test governs every line; the deny-list is grep-checkable and enforced through `assess-primitive` → `transform-recipes` → `verify-output`.
- **Full record:** [`AGENT-DECOUPLING-SYNTHESIS.md`](AGENT-DECOUPLING-SYNTHESIS.md).

### `specify` confirms the empirical calls + folds Gap Classification (2026-06-27)

- **Ported `specify`** as the second workflow (`/mochiko:transform-cluster specify`) — rebuilt kernel-free as a 2-member adversarial team (analyst ↔ advocate) with the command as lead/referee; the HIL `state-analyst` + both strategy skills + the context carrier dissolved onto that lead via one rehome map. All 14 primitives PASSed independent verification in a single round. Full run record: `.mochiko/transform/specify/` (`report.md`, `reconcile.md`, per-primitive assessments).
- **The inverse of `setup`:** specify already shipped an independent producer↔validator pair (analyst ↔ advocate, disjoint skills), so **no new primitive was constructed**. The work was re-landing the dissolving DAG orchestration onto the lead and ADDING the four `loop-discipline` gates HIL lacked (default-FAIL done-condition, lead-owned verdict, hard bound + kill-switch, and the NEW human acceptance gate).
- **Human-gate placement (OQ#1) → confirmed.** *Gated dispositions + escalations* + a named acceptance gate on the deliverable worked for both ports. Promoted to Key Decisions.
- **Memory model (OQ#3) → confirmed.** In-session + workspace-as-state under `.mochiko/`; context-handoff absorbed into the lead. specify's `context-template` absorb is a *stronger* confirmation than setup's twin (no drift; more state, still dissolves). Promoted to Key Decisions.
- **Decoupling doctrine → held empirically.** The decoupling-by-absence rules survived untouched: agent/skill bodies were clean by absence, coupling lived in the dissolving orchestration, and the only genuine deny-list hit was the canonical grep-catchable `analysis-specifications` "Devil's Advocate" case (decoupled to role). `verify-output`'s scan + the keystone test caught every case; **no deny-list refinement was needed** — the doctrine is proven.
- **Gap Classification folded into `loop-discipline`.** strategy-core's FAIL-routing taxonomy is universal (it lived in the cross-workflow skill), specify needed it now, and hosting it per-workflow would re-author the doctrine — so it was added to the shared skill (human-gate-accepted), keystone-tested to stay workflow-agnostic. The run's one edit to a foundational shared primitive.

---

### Command altitude — references, not restatements (2026-06-30)

- **Observed:** the first two ports (`setup` 385 lines, `specify` 329) read like the deleted kernel transliterated into prose — they re-stated `loop-discipline` up to three times (intro + an inlined filled contract + a "Supervisor behaviors" footer), inlined a filled `workflow-contract`, and transliterated every `Task()`/`AskUserQuestion()` payload.
- **Diagnosis:** not a style problem — a `single-source-rule-fanout` violation. The discipline already lives single-sourced in `loop-discipline` + `workflow-contract`; the commands duplicated it. The transform recipe caused it: `absorb-into-lead` swept *generic* discipline into the command body and no step single-sourced it back out.
- **Chosen:** a command's irreducible job is to stitch a team to a goal under a contract — declare the team, state the per-workflow contract parameters, reference shared doctrine, name the human gates. Fixed at the recipe (assess `dedupe` rule; transform thin-redesign + wiring step 6; `verify-output` altitude scan = grep floor + keystone ceiling), then retrofitted `specify` (329→66) and `setup` (385→78), both independently verified PASS via the standard producer↔validator pass. **Substrate** (native agent teams vs. `Task`-subagents) deferred until a thin command exists to evaluate against — it now does. Full record: [`COMMAND-ALTITUDE-SYNTHESIS.md`](COMMAND-ALTITUDE-SYNTHESIS.md).

### `plan` confirms the altitude recipe + runs convention-5's two-form (2026-07-01)

- **Ported `plan`** as the third workflow (`/mochiko:transform-cluster plan`) — the unified analysis→design planning loop (15 primitives, plan-core). Producer `technical-analyst` graded by **two independent reviewers**; all 15 primitives + the wiring PASSed independent verification in a single round, zero required fixes. Full run record: `.mochiko/transform/plan/`.
- **First net-new command since the altitude fix → the recipe produces thin by construction.** `plan.md` came out at **82 lines** (specify 66, setup 78) — the delta is genuinely workflow-specific (a second phase, feasibility-once-then-completeness ordering, skip-architect routing, incremental-review mode, a fifth gate), not restated doctrine. The `verify-output` altitude scan PASSed on the first try; **no retrofit was needed** (unlike specify/setup, which were re-transformed). The 2026-06-30 recipe fix is confirmed working.
- **First cluster to exercise convention-5's two-form** (axis 5). Plan legitimately needs **two distinct validators** on one producer: a *mirror-checklist* (`validation-plan-artifacts`, completeness) **and** an *adversarial-critique* (`validation-feasibility`, feasibility) — the two forms the convention names, realized as two skills on two agents. The minimalism counter (P9 already overlaps feasibility, so folding to one reviewer was cheaper) was weighed at the human gate and rejected: minimalism governs *body treatment*, not *collapsing distinct responsibilities* (buildability ≠ completeness; the distinct `infeasible`→business-escalation would flatten). Human-gate-accepted (RQ1 option i). `validation-feasibility` is the run's one net-new primitive (zero new agents).
- **Produce-here / review-there is decoupling-legitimate.** `principal-architect` produces in setup (authors the constitution) and reviews in plan (feasibility) — one workflow-agnostic persona, different artifacts in different workflows, never produce+grade the same artifact. The setup port had *narrowed* it to producer-only and disclaimed feasibility; re-broadening restored one coherent professional with **G1** intact (`validation-constitution` never re-mounted → no constitution self-grade recreated).
- **Decoupling doctrine held again (3rd empirical pass).** The richest persona yet (`technical-analyst`, 6 artifacts) had only 3 deny-list hits, all one caller-injection class, zero dangerous tokens; skills clean or near-clean. `verify-output`'s scan + the keystone test caught every case; no deny-list refinement needed. The one adjudicated edge (`principal-architect` L126 "its own validator") resolved as legitimate by-role independence language, not coupling.
- **`techspec` merge (BACKLOG) resolved → excluded.** HIL merged techspec into plan (ADR-008); the ported `plan` IS the merged form. techspec stays `[-]`.

### `implement` closes the command ports — sequential-first, decoupling's 5th pass (2026-07-01)

- **Ported `implement`** as the fifth and final workflow (`/mochiko:transform-cluster implement`) — the cycle-execution loop (7 primitives, implement-core), and the **DAG-heaviest** workflow (HIL ran it on a brain-mediated `implement-catalog.json`). All 7 primitives + the wiring PASSed independent verification in a single round, zero required fixes. Full run record: `.mochiko/transform/implement/`.
- **OQ#5 resolved → sequential-first.** The migration's one open *blocking* decision. Cycles execute in dependency order (foundation-before-feature) under a thin prose lead with `Task`-subagent dispatch — as HIL itself runs; parallelism was never realized. Native `pipeline()`/`parallel()` + any kernel/DAG are **deferred to dogfooding**, logged as a `deliberate-shortcut-ledger` entry (not a silent drop), consistent with the "code-free until dogfooding" kernel decision. Promoted to Key Decisions.
- **The inverse of setup (strongest example).** Like `specify`, the independent producer↔validator pair **already existed** — `staff-engineer` produces (skills `executing-tdd-cycle` + `brownfield-integration`), a *different* agent `qa-engineer` grades (`testing-end-user`), disjoint skills. **No new primitive was constructed.** The rehome instead ADDED the gates HIL lacked at the mochiko bar (default-FAIL done-condition, the confidence-based per-cycle gate made explicit, the NEW named final-acceptance gate, the overall round-cap / no-progress / `STOP` kill-switch) and MOVED verdict-ownership from the dissolved State-Analyst onto the lead. The validator is the strongest of any cluster — **Tier-1 deterministic** (real infrastructure + quality-gate exit codes), not a judgment grade.
- **Decoupling doctrine held (5th empirical pass) on the run's hot surface.** `staff-engineer`'s "Two Execution Modes" (Cycle/Fix mode-skeleton + trigger + `context.md` examples) was the coupled surface; the transform dissolved it to **zero residual** — mode routing/trigger/max-passes rehomed to the lead, only intrinsic fix-craft kept in the persona. `verify-output`'s decoupling scan + the keystone test caught every case; no deny-list refinement needed.
- **Altitude rollout now complete.** `implement.md` came out at **80 lines** (tasks 77, plan 82) thin-by-construction; the altitude scan PASSed first try, no retrofit. `implement` is the **last command** — every mochiko command is now thin, so the 2026-06-30 altitude-recipe rollout (recipe fix → retrofit specify/setup → thin-by-construction plan/tasks/implement) is finished.
- **Strategy family fully dissolved.** `strategy-implementation` was the **3rd/final** strategy dissolution (after `strategy-core` + `strategy-specification` in specify) — its doctrine deduped into `loop-discipline` (all present, **no edit**), its workflow params merged into the lead's rehome (no double-home), **zero residual skill**. No `strategy-*` skill survives for any workflow.

### `brainstorm` v2 — ceremony out, live team in (2026-07-04)

- **Observed (first behavioral dogfood of any workflow):** `/mochiko:brainstorm` v1 ran end-to-end on a real design question and split cleanly down the middle — the adversarial *substance* caught defects the room couldn't see (including a contradiction introduced by a mid-loop fix), while the *ceremony* (freeze→dispatch→report→fold→ratify, phase narration, five ID namespaces, a done-condition unreachable by construction, ≈620k tokens of cold re-reads) made the session illegible to the human it exists for.
- **Chosen:** rewrite as a minimal framework on native agent teams — the substance carriers survive (episodic challenge while thinking is fluid; cold independent review; the confidence-mark vocabulary; a verify pass), everything else dies. The human moves from gate-approver to **answer-owner + tie-breaker**, which is `loop-discipline`'s own preference-gap placement. Sound-loop status is claimed natively, not carved out — this is the pilot the other workflows inherit if dogfooding succeeds.
- **Deliberate bets, eyes open:** hard-require the experimental teams flag with no fallback transport (`Contested` — dogfood fidelity over reach; revisit at distribution); the pressure-tester's independence is *structural sequestration* (spawned cold at convergence, reads the record before any teammate contact) rather than v1's file-freeze.
- **Full record:** `.mochiko/brainstorms/brainstorm-command-rewrite/record.md` (D1–D9 with challenge trail); dogfood evidence: `.mochiko/brainstorms/setup-workflow-rewrite/dogfood-handover.md` (branch `brainstorm-dogfood-setup-rewrite`).

### `brainstorm` v2.1 — the end-stage review pair (2026-07-05)

- **Observed (first v2 run, judged from its transcripts, not its self-report):** the run produced real content — 11 decisions, 7/7 advocate engagements yielding surviving folds, and the standing grounder closed I-6 by steering decisions before they formed — but the pre-registered user's-seat check **failed**: "a continuous loop without my input." Forensics on the lead + teammate transcripts measured 8 user inputs against 14 lead→advocate messages (7 engagements, 7 bookkeeping wakes — every message costs a full teammate turn), ~10 advocate turns with 6 record re-reads, 3 message crossings, lead-folds amending user-ruled decisions behind two never-answered objection windows, teammate↔teammate messaging used **zero** times, and total cost above v1's ≈620k (I-4 closed; numbers in `BACKLOG.md`). The run died at the shared account limit pre-convergence — the pressure-test tail never ran.
- **Chosen (three user rulings, converging):** challenge-everything turn-discipline → on-demand triggering → **end-stage review pair** — each step in the same direction; the final shape ranks session legibility above early catches (the accepted trade, recorded eyes-open). The advocate and pressure-tester seats merge into two symmetric cold reviewers who audit each other by cross-examination: sequestration generalized (counterpart withheld until findings formed), owner-withdrawal only, survivors + tally to the lead.
- **Kept from v2:** record-as-you-go with confidence marks, the conditional grounder (now also the debate's fact arbiter), hard-require teams (`Contested`, unchanged), reachable default-FAIL done-condition, dispositions + verify pass + plain-text acceptance, pipeline entry as offer. Companion edit: the ratification-streak check landed in `analysis-iterative` (third session validating it).
- **Full record:** `.mochiko/brainstorms/brainstorm-v2-revision/record.md` (decision + supersession trail, the transcript-forensics method, measured costs).

## Open Questions (live)

Resolved this rewrite and removed: prose-vs-gate global allocation (→ deferred with the kernel), loop-discipline carrier (→ skill+template), human-as-validator (→ thesis).

Still genuinely open:

1. **Human-gate placement per workflow** — ~~every cycle / low validator-confidence only / preference-gaps only?~~ **RESOLVED (2026-06-27, setup + specify):** *on gated dispositions + escalations*, plus a named human acceptance gate on the deliverable. Promoted to Key Decisions.
2. **Claude-Code portability** — `rfc2119-invocation-trigger` and the `disable-model-invocation` flag are CC-specific. Adopt-and-bind, or abstract? Surfaces when the router skill is built.
3. **Memory model** — ~~how does `stateful-workspace-as-memory` relate to the spec/plan/task artifact layout?~~ **RESOLVED (2026-06-27, setup + specify):** in-session + workspace-as-state under `.mochiko/` (`.mochiko/memory/` durable governance; `.mochiko/specs/<feature>/` per-feature loop state); state recovery reads workspace evidence, not a context `phase` field; the HIL context-handoff template is absorbed into the lead. Promoted to Key Decisions.
4. **Intensity modes** — global `lite/full/ultra/off` dial vs per-rule? Defer until two workflows exist and the pattern is clear.
5. **`implement` orchestration + the deferred kernel** — ~~do native Workflow `pipeline()` calls suffice for parallel TDD slices, or does `implement` re-open the kernel/hook question?~~ **RESOLVED (2026-07-01, implement):** sequential-first; native `pipeline()`/`parallel()` + kernel deferred pending dogfooding. Promoted to Key Decisions.

---

## Recommended Next Steps

1. **Author the doctrine primitives first** (net-new, not ported): the `loop-discipline` model-invoked skill and the `workflow-contract` template. Everything else consumes them.
2. **Build `setup`** as the first workflow under the playbook: port + upgrade `principal-architect` and the constitution cluster (`authoring-constitution`, `brownfield-constitution`, `validation-constitution`, `analysis-codebase`, `syncing-claude-md`), shedding the brain-mediated DAG invocation, and instantiate its `workflow-contract` (done-condition, `validation-constitution` as the independent validator, round cap, human gate).
3. **Build `specify`** as a 2-member agent team (analyst producer + advocate critic, lead as referee), keeping the three `adversarial-pair-convergence-loop` invariants and the human-escalation gate.
4. **Dogfood, then generalize.** After `setup`+`specify` run for real, extract the crystallized conventions into the playbook and CLAUDE.md, and re-evaluate the deferred-kernel ledger entry.

See [`REGISTRY.md`](REGISTRY.md) for the migration inventory.
