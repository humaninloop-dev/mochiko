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

## Open Questions (live)

Resolved this rewrite and removed: prose-vs-gate global allocation (→ deferred with the kernel), loop-discipline carrier (→ skill+template), human-as-validator (→ thesis).

Still genuinely open:

1. **Human-gate placement per workflow** — ~~every cycle / low validator-confidence only / preference-gaps only?~~ **RESOLVED (2026-06-27, setup + specify):** *on gated dispositions + escalations*, plus a named human acceptance gate on the deliverable. Promoted to Key Decisions.
2. **Claude-Code portability** — `rfc2119-invocation-trigger` and the `disable-model-invocation` flag are CC-specific. Adopt-and-bind, or abstract? Surfaces when the router skill is built.
3. **Memory model** — ~~how does `stateful-workspace-as-memory` relate to the spec/plan/task artifact layout?~~ **RESOLVED (2026-06-27, setup + specify):** in-session + workspace-as-state under `.mochiko/` (`.mochiko/memory/` durable governance; `.mochiko/specs/<feature>/` per-feature loop state); state recovery reads workspace evidence, not a context `phase` field; the HIL context-handoff template is absorbed into the lead. Promoted to Key Decisions.
4. **Intensity modes** — global `lite/full/ultra/off` dial vs per-rule? Defer until two workflows exist and the pattern is clear.
5. **`implement` orchestration + the deferred kernel** — do native Workflow `pipeline()` calls suffice for parallel TDD slices, or does `implement` re-open the kernel/hook question? The one place the deferred code-decision is most likely to come due.

---

## Recommended Next Steps

1. **Author the doctrine primitives first** (net-new, not ported): the `loop-discipline` model-invoked skill and the `workflow-contract` template. Everything else consumes them.
2. **Build `setup`** as the first workflow under the playbook: port + upgrade `principal-architect` and the constitution cluster (`authoring-constitution`, `brownfield-constitution`, `validation-constitution`, `analysis-codebase`, `syncing-claude-md`), shedding the brain-mediated DAG invocation, and instantiate its `workflow-contract` (done-condition, `validation-constitution` as the independent validator, round cap, human gate).
3. **Build `specify`** as a 2-member agent team (analyst producer + advocate critic, lead as referee), keeping the three `adversarial-pair-convergence-loop` invariants and the human-escalation gate.
4. **Dogfood, then generalize.** After `setup`+`specify` run for real, extract the crystallized conventions into the playbook and CLAUDE.md, and re-evaluate the deferred-kernel ledger entry.

See [`REGISTRY.md`](REGISTRY.md) for the migration inventory.
