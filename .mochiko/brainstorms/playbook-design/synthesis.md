# Playbook Design Synthesis — Transforming Primitives into Mochiko Form

_Synthesized: 2026-06-27 from an analysis-iterative brainstorm. This is the **design of the playbook**, not the playbook itself. The playbook (`PLAYBOOK.md`) and its primitives get built from this spec._

Source of truth for "the mochiko way": [`ROADMAP.md`](../../../ROADMAP.md) + `agent-skills-research/synthesis/my-framework.md`.

---

## Problem Statement

We need a repeatable, inspectable way to turn a human-in-loop primitive (skill, agent, or workflow) into mochiko form. "Mochiko form" = kernel-free, satisfying the five skill-library conventions, and correctly placed inside a sound-loop.

The defining twist: **the playbook is dogfooded.** It is not just a doc — it is itself the first mochiko cluster (a transform workflow + transform agents + transform skills) whose *subject* is "transform a primitive." This forces the transformer to obey the sound-loop doctrine it enforces. The transformer that ports primitives must itself pass its own done-condition.

---

## Key Decisions

| # | Decision | Choice | Confidence |
|---|----------|--------|------------|
| 1 | Input scope | **HIL-only for now.** Assume inputs arrive with human-in-loop's known shape (frontmatter, `SKILL.md`, persona files, catalog refs). Non-HIL "otherwise" deferred. | Confident |
| 2 | Transformer form | **Thin workflow + skill library + agent team.** Workflow = thin orchestration (intake → assess → decompose → transform → reconcile → validate); rules/tests/recipes live in skills; work done by a **producer** (transformer) + an independent **validator** (conformance) + the **lead** as referee, with a human gate. Dogfoods the sound-loop by construction. | Confident |
| 3 | Done-condition | **Conformance + responsibility trace, default FAIL.** Output must (a) satisfy the five conventions + sound-loop placement + be kernel-free, AND (b) carry a trace mapping every original responsibility → where it lives now (`kept` / `moved-to-lead` / `folded-into-skill` / `dropped + reason`). No silent capability loss. | Confident |
| 4 | Transform unit | **Per-cluster.** One run transforms a whole workflow-cluster. Assessment runs **per-primitive inside**; a **cluster-reconciliation phase** then resolves producer↔validator pairings, `absorb-into-lead` targets, and where dropped responsibilities re-home. Matches the roadmap's "port the cluster together." | Confident |
| 5 | Deliverable artifact | **Thin doctrine + fan-out.** A short `PLAYBOOK.md` states only the *doctrine* (done-condition, disposition vocabulary, loop shape, conventions enforced) and **points to** the skills that operationalize each rule. Detailed lens/recipes/tests live *in the skills*, single-sourced (`single-source-rule-fanout`). | Confident |

---

## The Design

### Pipeline shape (v2 — the seams did NOT hold)

_Dry-Run Round 1's strongest, most convergent finding: the proposed 5 seams are wrong boundaries. `decompose` is not a peer seam — the responsibility trace **is** its output, and that trace **is** the done-condition of `assess`, so decompose folds in. `reconcile` is validated as genuinely cluster-level, but must own the **relational verdicts** (split/merge/promote/pair) and inherit **rehome-orchestration** (re-landing the dissolving supervisor's responsibilities). `validate` collided in name with the `validation-*` subject → renamed `verify-output`._

```
cluster in
   │
   ├─ triage        fast-path gate: trivial primitive? → wiring pass + verify, skip the middle
   ├─ assess        branch by primitive class; run the lens; emit either a terminal disposition
   │   (incl.        OR a non-terminal `flag-for-reconcile`; the responsibility trace is its
   │   decompose)    done-condition (decompose is a sub-step here, not a peer seam)
   ├─ reconcile     CLUSTER scope: own the relational verdicts (split / merge / promote / pair),
   │                 dedupe shared responsibilities, AND rehome-orchestration (routing, iteration
   │                 cap, the missing validation + human gates) onto the Workflow/lead
   ├─ transform     apply the disposition's recipe → mochiko-form artifact + responsibility trace
   └─ verify-output independent validator audits conformance + trace (done-condition, default FAIL)
   │
cluster out  (+ REGISTRY.md status updates)
```

### The assessment lens (v2 — reframed and branched) — the "tests / what to look for"

_Revised by Dry-Run Round 1. The original 5 checks mis-fired three ways: the Brain test assumed a kernel the `setup` cluster never had; checks 2/5/conv-5 triple-counted one finding (self-grade); and all checks were blind to siblings._

**Branch by primitive class first** — a command/workflow *IS* a loop; an agent/skill *PLAYS a role in* a loop. Running identical checks on both is lossy (classification is a category error against an agent; loop-ownership sits on the Workflow, not the agent).

**Fast-path triage gate** (top of assess): (1) orchestration-coupled? (2) multi-responsibility / fans out? (3) emits an artifact whose correctness is NOT machine-checkable? All "no" → skip to the wiring pass + verify. Reserve the full lens for orchestration-coupled or genuine producer primitives.

| Check | Question |
|---|---|
| 1. **Orchestration test** (was: Brain test) | What layer orchestrates this — Python/DAG kernel, *markdown supervisor*, or command — and who inherits each responsibility when it dissolves? Separate content-coupling from orchestration-coupling. |
| 2. **Role (two altitudes)** | Skill-role (consumed-procedure / emits-artifact) AND the team-role it confers on its caller (producer / validator / referee / lead). A consumed-skill can make its *caller* a producer. |
| 3. **Independence** (merges old 2+5+conv-5) | Does one agent both produce and grade? Cluster-scoped — the leak is usually in the agent's `skills:` list, invisible at the skill. |
| 4. **Verdict-sink / loop-driver** | Who consumes this primitive's output, and what loops on FAIL? (The biggest thing the kernel/supervisor owned.) |
| 5. **Sibling / overlap** | Shared core, shared validator, trigger collisions with siblings → emits a merge/split signal. The "look sideways" check. |
| 6. **Coupling audit** | Hardcoded paths, upstream prerequisites/handoffs, and the determinism boundary (deterministic script vs model judgment). |
| 7. **Conventions + loop placement** | The five conventions + does it supply a done-condition / independent validation / human gate. |

### Disposition vocabulary (v2 — two orthogonal axes + a mandatory wiring pass)

_Revised by Dry-Run Round 1: a single 5-word list cannot express the moves mochiko actually makes. The important ones — split a producer from its validator, merge a variant into its sibling, promote a check into an independent validator's tool — are **relations between primitives**, not single-primitive verbs._

A disposition is **one body value × one structural value**, and a wiring pass that *always* runs:

- **Body treatment** (what happens to the content): `keep-verbatim` · `port-with-edits` · `redesign` · `drop`
- **Structural move** (how it sits in the cluster): `standalone` · `split` (1→N, esp. producer↔validator) · `merge-into-sibling` (N→1 variant consolidation) · `promote` (role elevation, e.g. gate-check → independent-validator's tool) · `absorb-into-lead` · `rewire-cluster` (body fine, fix the agent/workflow around it)
- **Convention-wiring pass** — ALWAYS runs, never a disposition: classification tag, router registration, RFC2119 triggers, path rebinding. This is why `keep-as-is` was a lie — even the most trivial utility pays this. The floor is never zero-work.

Examples from the dry-run: `syncing-claude-md` = keep-verbatim × standalone (still pays wiring); `validation-constitution` = port-with-edits × **promote**; `brownfield-constitution` = redesign × **merge-into-sibling**; `principal-architect` = port-with-edits × **split** (producer + new independent validator agent). The relational moves (`split`/`merge`/`promote`) are decided at **reconcile** (cluster scope), not at per-primitive assess.

### Done-condition detail (conformance + responsibility trace) — v2 tags

The done-condition itself held up well — forcing a responsibility trace surfaced real issues in every run. But the 4 tags were lead-centric and sibling-blind. **Expanded tag set:**

`kept` · `kept-but-rebind` (path/grounding changes in place) · `folded-into-skill` · `moved-to-lead` · `moved-to-validator` (a *peer* agent, not the lead) · `moved-to-sibling-skill` · `moved-to-other-cluster` (a peer workflow) · `dedupe` (legitimately duplicated across producer/validator — collapse) · `dropped + reason`

The producer emits the trace alongside the transformed artifact. The independent validator's checklist:
1. **Conformance** — five conventions present, sound-loop placement correct, kernel-free (artifact-checkable).
2. **Trace audit** — every original responsibility is accounted for; no silent drops; every `dropped` carries a reason the lead accepts.

The trace doubles as the `REGISTRY.md` migration record. Default is FAIL until both pass.

### How it dogfoods the sound-loop

| Sound-loop requirement | How the transformer satisfies it |
|---|---|
| Pre-declared done-condition (default FAIL) | Decision 3 — conformance + trace, written before the run |
| External, independent validation | The conformance-validator is a *different agent* running a *different skill* than the producer |
| Bounded iteration | Round cap on transform↔validate; no-progress exit; escalation to human |
| Defined human gate | Lead presents disposition + trace at a placement TBD (see Open Questions) |

### Primitive decomposition (v2 — revised by the dry-run)

**Skills (model-invoked — the rules/tests/recipes):**
- `assess-primitive` — branch-by-class + fast-path triage + the 7-check lens; emits disposition (or `flag-for-reconcile`) + responsibility trace. **Absorbs the former `decompose-primitive`** (decompose is its done-condition, not a peer).
- `reconcile-cluster` — cluster-scope: relational verdicts (split / merge / promote / pair), dedupe, **and rehome-orchestration** (re-land the dissolving supervisor's responsibilities).
- `transform-recipes` — per-disposition recipes (the "how to change it"), keyed to the two-axis vocabulary.
- `verify-output` — the done-condition checker (the validator's tool). Renamed from `validate-conformance` to avoid colliding with the `validation-*` subject.

(Dropped: `decompose-primitive` as a standalone skill.)

**Agents:**
- transform-producer (persona: senior framework migrator) — `skills:` assess-primitive / reconcile-cluster / transform-recipes
- **transform-validator** (persona: skeptical reviewer, independent) — `skills:` verify-output. **Non-negotiable per the dry-run's meta-finding:** the transformer must pair producer+validator or it commits the self-grade sin it exists to catch.
- lead/referee — owns the verdict, owns reconciliation arbitration, owns the human gate.

**Doc:** `PLAYBOOK.md` — thin doctrine + pointers to the above (Decision 5).

These seams are now **dry-run-tested** against the `setup` cluster (Round 1), not merely assumed — see the findings log below. A second round (with the producer↔validator partner *discovered* rather than handed in) is the recommended next validation.

---

## Dry-Run Findings (Round 1 — 2026-06-27)

Six transform-producer subagents ran the lens on the `setup` cluster — chosen to exercise every disposition path: an agent (`principal-architect`), a producer skill (`authoring-constitution`), its validator partner (`validation-constitution`), a feeder (`analysis-codebase`), a variant (`brownfield-constitution`), a utility (`syncing-claude-md`).

**All five headline decisions survived; the internal mechanics (lens, dispositions, tags, seams) were revised** — captured in the v2 sections above. The convergent flaws that drove the revisions:

1. **Orchestration ≠ kernel.** `setup` was never kernel-driven — it's a markdown supervisor (`commands/setup.md`); only `specify`/`plan` are DAG-based. The "brain test" returned "nothing breaks" and hid the real work (re-homing the supervisor). → Check 1 reframed.
2. **Dispositions are single-primitive; the real moves are relational.** Missing `split`, `merge-into-sibling`, `promote`, `rewire-cluster`; `keep-as-is` hides mandatory wiring. → two-axis vocabulary.
3. **Trace tags are lead-centric / sibling-blind.** → expanded tag set.
4. **Seams wrong:** decompose folds into assess; reconcile owns relational verdicts + rehome-orchestration; validate→verify-output; fast-path for trivial primitives. → pipeline v2.
5. **Meta — the pipeline must eat its own dogfood:** this round used producers with no independent validator (the exact self-grade sin). And the prompts pre-biased cluster checks by *handing* agents their partner; Round 2 should let `assess` flag and `reconcile` discover.

**Per-primitive dispositions produced (the bonus output, in v2 vocabulary):**

| Primitive | Body × Structural | Note |
|---|---|---|
| `principal-architect` | port-with-edits × **split** | Spin out an independent `constitution-validator`/`governance-reviewer`; extract the orphaned feasibility-review procedure into a skill; trim duplicated procedure from the persona. |
| `authoring-constitution` | keep-verbatim × **rewire-cluster** | Skill is near-clean; the fixes (remove `validation-constitution` from the producer agent, wire the gate) live in the *agent/workflow*, not `SKILL.md`. |
| `validation-constitution` | port-with-edits × **promote** | Becomes the load-bearing tool of the independent validator — the concrete instance of `external-grounded-validation`. De-collide its generic triggers from the other `validation-*` skills. |
| `analysis-codebase` | port-with-edits × **split** | Fans out to 3 consumers/modes (shared deterministic substrate / setup-producer / planning collision-feeder); needs a `validation-codebase-analysis` partner that checks claims against real files. |
| `brownfield-constitution` | redesign × **merge-into-sibling** | Consolidate with `authoring-constitution` into one skill with greenfield/brownfield branches; only Emergent Ceiling + Evolution Notes + the `analysis-codebase` prerequisite are brownfield-unique. |
| `syncing-claude-md` | keep-verbatim × standalone | No validator partner — its correctness is machine-decidable (version equality + table diff), so the validator degenerates to a deterministic assert. Still pays the wiring pass. |

**Strategic finding for `setup` (feeds ROADMAP):** the HIL `setup` loop is *missing external validation entirely* — `validation-constitution` exists but is never invoked, and is co-mounted on the producer (self-grade). So `setup`'s real transformation isn't "shed a kernel"; it's "re-home the markdown supervisor onto a native Workflow **and add the independent-validation gate + human gate HIL never had**."

---

## Decision Trail

- **Scope** opened as HIL-first vs truly-arbitrary vs HIL-with-deferred-normalization-seam → user chose HIL-only, collapsing intake to a known shape.
- **Done-condition** opened as conformance-only vs +behavioral-preservation vs +responsibility-trace → trace chosen: real preservation rigor without pretending to execution-test prose; falls out as a natural producer↔validator pair.
- **Unit** opened as per-primitive vs per-cluster vs escalating-hybrid → per-cluster chosen: every mochiko composition decision (pairing, absorb, dedupe) is inherently cluster-scoped, so per-primitive only defers the hard calls.
- **Artifact form** opened as doc-first vs primitives-first vs thin-doctrine+fan-out → thin-doctrine chosen: it is the playbook's own thesis applied to itself, and the only option that doesn't duplicate rules across a doc and the skills. (Resolves the earlier deleted-`PLAYBOOK.md` tension: that doc was build-specific prose; this one is doctrine + pointers.)

---

## Open Questions (deferred)

1. **Human-gate placement in the transform loop** — every primitive / only on `redesign`+`absorb` / only on low validator-confidence? Resolve empirically when the transformer first runs on the `setup` cluster (mirrors ROADMAP OQ#1). _Deferred._
2. **Non-HIL normalization seam** — when "otherwise" inputs arrive, a reverse-engineering front-end is needed before the lens can run. Named as a `deliberate-shortcut-ledger` entry; not built now. _Deferred._
3. **Disposition short-circuiting** — do `drop` / `absorb` skip the transform→validate loop, or still produce a (degenerate) trace for the audit? Lean: still produce a trace (the dropped/absorbed responsibilities are exactly what must be audited). _Unsure._
4. **Exact skill boundaries** — _largely resolved by Dry-Run Round 1:_ decompose folds into `assess-primitive`; seams are `triage → assess → reconcile → transform → verify-output`. Remaining: whether `transform-recipes` is one skill with branches or one per disposition. _Resolve when built._
5. **Round-2 dry-run (unbiased)** — re-run with the producer↔validator partner *discovered* by `reconcile`, not handed in the prompt, AND with an independent transform-validator in the loop, to prove the seam boundaries rather than assume them. _Open._

---

## Next Steps

1. **Ratify this design** (or amend), then write the thin `PLAYBOOK.md` doctrine doc per Decision 5.
2. **Build the transformer cluster** as the first dogfood: the five skills + three agent roles, instantiating its own `workflow-contract` (done-condition = conformance + trace, conformance-validator as the independent validator, round cap, human gate).
3. **Run it on the `setup` cluster** as the proving ground — it transforms `principal-architect` + the constitution skills + templates, and the run teaches the real skill boundaries (Open Q4) and the human-gate placement (Open Q1).
4. **Feed results back** — update `REGISTRY.md` from the responsibility traces; record any crystallized convention in `ROADMAP.md`; close the relevant `BACKLOG.md` items.
