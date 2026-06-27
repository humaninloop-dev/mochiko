# Assessment — `analysis-iterative` (P5, specify cluster)

**Skill:** `mochiko:assess-primitive` · **Producer:** `transform-producer` · **Run:** specify · **Date:** 2026-06-27
**Source:** `human-in-loop/plugins/humaninloop/skills/analysis-iterative/`
(SKILL.md + SPECIFICATION-INPUT.md + ENRICHMENT.md + SYNTHESIS.md + references/ADAPTIVE-EXAMPLES.md)
**Drives HIL node:** `specify-catalog.json` → `input-enrichment` (consumes `raw-input` → produces `enriched-input`)

---

## Output (skill format)

```
ASSESSMENT: analysis-iterative
Class:        skill → branch PLAYS-a-role
Triage:       gate1=y gate2=y gate3=y  [full-lens]
Disposition:  port-with-edits × standalone   (+ 3 flag-for-reconcile, relational only)
```

---

## Step 1 — Class / branch

**SKILL → PLAYS-a-role.** Model-invoked (description carries graded RFC-2119 trigger phrases; no
`disable-model-invocation`). Weighting per the SKILL branch: consumed-procedure vs emits-artifact,
trigger reliability, sibling overlap, decoupling.

This is a **dual-mode** skill:
- **Standard mode** — general collaborative/iterative analysis ("brainstorm", "think through") →
  emits a `SYNTHESIS.md`-shaped artifact.
- **Spec-input mode** (`mode:specification-input`) — enrich a sparse feature description into
  Who/Problem/Value + Out-of-scope + Success → emits an `ENRICHMENT.md`-shaped artifact. **This is
  the mode that drives the HIL `input-enrichment` node** and the only slice specify-core strictly needs.

## Step 2 — Fast-path triage gate

| Gate | Q | Verdict |
|------|---|---------|
| 1 | Orchestration-coupled? | **YES** — the spec-input mode is catalog-node-driven (`input-enrichment`), state-analyst-dispatched, and **marker-parsed by the supervisor**. (Standard mode alone is NOT coupled; coupling is concentrated in the spec-input mode.) |
| 2 | Multi-responsibility / fans out? | **YES** — two modes, two output templates, two consumers (general user vs. the requirements producer). |
| 3 | Emits a non-machine-checkable artifact? | **YES** — both synthesis and enrichment are judgment artifacts (quality of Who/Problem/Value framing, confidence indicators). Not schema/version-equality. |

All three trip → **full 7-check lens.**

## Step 3 — The 7-check lens

**1. Orchestration test — content-coupling vs orchestration-coupling.**
- **Content-coupling: NONE.** `grep` of the whole skill bundle finds zero `brain` / `hil-dag` / `mcp`
  / `.sh` / `node_id` / `catalog` / `DAG` references in the body. The skill never reaches into the kernel.
- **Orchestration-coupling: YES.** The `specify-catalog` `input-enrichment` node decides *when* it runs
  (sparse-input detected), the `state-analyst` constructs the `mode:specification-input missing:[…]
  original:"…"` invocation, and the supervisor parses the `<!-- ENRICHMENT_COMPLETE -->` /
  `<!-- ENRICHMENT_OUTPUT_END -->` markers to lift `enriched-input` forward. **When the catalog/DAG
  dissolves, three responsibilities must rehome:** (a) the *detect-sparseness* trigger
  (SPECIFICATION-INPUT.md L3: "the specify command detects…"), (b) the *parse-and-forward* of the
  enriched output, (c) the *sequencing* ("continue to Phase 1"). Rehome target = **lead** (command supervisor).
- Net: **stands alone as a model-invoked enrichment/analysis skill; the wiring around it is what dissolves.**

**2. Role (two altitudes).**
- Skill-role: **both** consumed-procedure (carries the iterative-questioning *how*) **and**
  emits-artifact (SYNTHESIS / ENRICHMENT). Leans emits-artifact in spec-input mode.
- Team-role conferred: **pre-producer / input-conditioner.** It runs *upstream of* the adversarial loop
  and feeds the producer (`requirements-analyst`). It is **not** a loop member — not producer, validator,
  referee, or lead inside the produce↔grade loop. One-shot conditioner, no FAIL-loop.

**3. Independence.** No self-grade leak — it neither grades nor validates. It must not be mounted on
the validator (`devils-advocate`); mounting on the lead or the producer is independence-safe. *Which*
of those is a composition (reconcile) question, not an independence violation. **No independence risk.**

**4. Verdict-sink / loop-driver.** Consumer of its output = the **requirements-analyst** (uses the
enriched Summary as starting input). No FAIL-loop loops on it (enrichment is a one-shot pre-step, not a
graded gate). The "forward enriched-input → analyst" wiring rehomes to an explicit **handoff edge** / lead.

**5. Sibling / overlap.** Siblings: `analysis-specifications` (P6, advocate gap-analysis),
`authoring-requirements` (P7), `authoring-user-stories` (P8).
- vs `analysis-specifications`: distinct lifecycle position — analysis-iterative enriches sparse input
  **pre-spec** (interactive Who/Problem/Value); analysis-specifications reviews a **drafted spec** for gaps.
  SKILL.md L25 already states the boundary ("Specification review — use analysis-specifications instead").
  **Not a merge candidate**, but the enrichment-vs-clarification seam + the two trigger sets warrant a
  reconcile confirmation that triggers don't collide once both port (→ F3).
- The **standard brainstorm mode** is a *general-purpose, cross-cluster* capability broader than
  specify-core; only the enrichment slice is load-bearing for this cluster (→ scope flag F2).

**6. Coupling audit.**
- **Paths: clean.** No `.humaninloop/` memory/catalog/MCP/DAG paths in the body. Only relative
  progressive-disclosure bundle refs (SYNTHESIS.md, SPECIFICATION-INPUT.md, ENRICHMENT.md,
  references/ADAPTIVE-EXAMPLES.md) — fine.
- **Prerequisites/handoffs:** consumes `raw-input` + a `missing:[…]` list (the latter computed
  upstream by the command's triad-detection — an upstream dependency owned by the command); produces
  `enriched-input` for the analyst.
- **Determinism boundary:** entirely model-judgment (adaptive conversation). No deterministic script →
  no validator-degeneration; genuinely a judgment skill.

**7. Conventions + loop placement + DECOUPLING SCAN (the run's keystone test).**
- Classification: model-invoked ✓ (keep). Router registration: due at transform (wiring). Trigger
  phrasing: graded RFC-2119, work-context — **good reliability**; the SHOULD spec-enrichment trigger is
  the cluster's load-bearing one. Path rebinding: minimal (no kernel paths).
- Loop placement: supplies its own done-condition (convergence / enrichment-complete) and a **natural
  human gate** (the whole skill is human-interactive Q&A). Not a graded loop → needs no independent
  validator. Correct as a **pre-loop conditioner.**
- **DECOUPLING SCAN — keystone test applied per line** (true of this professional on *any* job → keep;
  names a sibling/dispatch/workflow-mode/path/phase → flag):

  | Loc | Token | Severity |
  |-----|-------|----------|
  | ENRICHMENT.md L27 & L57 | "The `/humaninloop:specify` supervisor should now continue to **Phase 1** …" | **HARD** — sibling command name + injected workflow **phase** |
  | SPECIFICATION-INPUT.md L1, L3 | "invoked by `humaninloop:specify`" / "the **specify command** detects when input lacks the … triad" | **HARD** — sibling command name + caller detection logic baked into skill |
  | SPECIFICATION-INPUT.md L112, L115, L117 | "the **supervisor** should continue" / "markers allow the **supervisor** to parse" / "the **supervisor (specify command)** owns the workflow continuation" | **HARD** — supervisor/dispatch coupling ×3 |
  | ENRICHMENT.md markers `<!-- ENRICHMENT_COMPLETE/…_END -->` + SPECIFICATION-INPUT.md L9–16 invocation incl. `missing:[…]` | DAG parse-markers (exist only so the brain can machine-lift output) + caller-side dispatch args | **MECHANISM / caller-side** |
  | SKILL.md L17 | "Enriching sparse feature descriptions for `humaninloop:specify`" | **SOFT** — sibling command name |
  | SKILL.md L25 | "use `humaninloop:analysis-specifications` instead" | **SOFT** — sibling-skill `humaninloop:` prefix (rebind) |
  | SKILL.md core + SYNTHESIS.md + references/ADAPTIVE-EXAMPLES.md | — | **CLEAN** — passes keystone ("true of this analysis professional on any job") |

  **Doctrine finding for this run:** the deny-list tokens are real and **concentrated exactly in the
  brain/DAG-driven mode** (spec-input), while the pure-craft core (standard mode, examples, synthesis
  template) is clean. Clean empirical confirmation: **coupling tracks orchestration, not body.**

## Step 4 — Disposition

**`port-with-edits × standalone`** (hypothesis confirmed).

- **Body = port-with-edits** (NOT keep-verbatim, NOT redesign): the adaptive-questioning + enrichment
  *approach* is sound for mochiko — only the supervisor-coupling wiring needs localized fixes. Edits:
  strip the HARD/SOFT decoupling tokens above (state independence by *role*: "hand the enriched
  description to the requirements producer" — no command name, no "Phase 1", no "supervisor"); push
  the `missing:[…]`/detect-sparseness logic and the `mode:` invocation example to **caller-side**
  dispatch guidance (`agent-dispatch.md`), not the skill; drop the DAG parse-markers in favor of a
  plain artifact the lead reads directly (mild output-contract edit, not a wholesale redesign).
- **Structural = standalone**: stays one model-invoked skill in its own directory with its bundle. It
  does **not** split (not produce+grade; its output is consumed as input, not graded), **not**
  merge (analysis-specifications is a different lifecycle stage), **not** promote (no validator role),
  **not** absorb-into-lead (it has a real reusable body). The skill-as-primitive is standalone.

> Per assess discipline, the **relational** decisions below are NOT guessed here — emitted as
> `flag-for-reconcile`. The body treatment stands regardless of how they resolve.

## Step 5 — Responsibility trace (every responsibility tagged — no silent loss)

| # | Responsibility | Tag |
|---|----------------|-----|
| 1 | Adaptive iterative-questioning procedure (opening/discovery/adaptive Q/wrap-up; format adaptation; confidence-signal reading) | **kept** — mochiko-clean core |
| 2 | Standard-mode synthesis artifact (SYNTHESIS.md, confidence indicators, scaled output) | **kept** *(scope-flagged F2: may become `moved-to-other-cluster` if the general brainstorm mode is rehomed out of specify-core — reconcile decides)* |
| 3 | Spec-input enrichment procedure (two-phase Who/Problem/Value + Scope + Success agenda) | **kept** (decoupled) — the slice specify-core needs |
| 4 | Enriched-input artifact (Actor/Problem/Value/Out-of-scope/Success + Summary + preserved Original Input) | **kept-but-rebind** — keep artifact; drop HIL parse-markers, rebind handoff to "lead/producer reads artifact directly" |
| 5 | Detect "input lacks Who/Problem/Value" → decide to enrich (HIL: command computes `missing:[…]`) | **moved-to-lead** — orchestration the dissolving command/brain owned (reconcile rehome F1) |
| 6a | DAG parse-markers (`ENRICHMENT_COMPLETE`/`_OUTPUT_END`) as machine-parse hooks | **dropped + reason:** kernel-free — markers exist only so the brain can lift output; in mochiko the lead reads the artifact directly *(human-gate must accept: output-contract change)* |
| 6b | Workflow-continuation sequencing ("continue to Phase 1") | **moved-to-lead** — "what runs next" belongs to the lead, not the skill |
| 7 | Handoff enriched-input → requirements producer | **kept-but-rebind** — from catalog `produces`/supervisor-forward to an explicit contract **handoff edge** |
| 8 | Mode-dispatch contract (`mode:specification-input missing:[…] original:…`) | **kept-but-rebind** — invocation example → caller-side `agent-dispatch.md`; dual-mode mechanism kept if reconcile keeps both modes in one skill (F2) |
| 9 | Sibling cross-refs (When-NOT "use analysis-specifications"; When-to-Use "for humaninloop:specify") | **kept-but-rebind** — drop `humaninloop:` prefixes / state by role; router owns discoverability |
| 10 | Progressive-disclosure reference bundle (ADAPTIVE-EXAMPLES.md — 3 annotated conversations) | **kept** — clean pure-craft examples |
| 11 | Trigger phrases in `description` (graded RFC-2119) | **kept-but-rebind** — keep graded triggers describing work-context; spec-enrichment SHOULD-trigger is the load-bearing one |

**Only `drop`:** the DAG parse-markers (6a), with explicit reason → routed to the human gate for acceptance.

## Reconcile flags (relational — for `reconcile-cluster`)

- **F1 — Mount-point + orchestration rehome.** Where does enrichment mount, and who inherits the
  command/state-analyst orchestration ("detect sparse input → invoke enrichment → forward
  enriched-input → analyst")? Candidates: **lead invokes it as a model-invoked pre-step** (rehome
  detection+sequencing to the lead) **vs. the `requirements-analyst` owns enrichment as its first
  move.** Touches P1 (`specify` command), P2 (`requirements-analyst` `skills:`), P4 (`state-analyst`
  dissolution). Independence-safe either way; pick on minimalism + clean handoff.
- **F2 — Scope of the standard brainstorm mode.** specify-core strictly needs only the enrichment
  slice; the general iterative-analysis/brainstorm mode is a cross-cluster utility. Keep dual-mode in
  one skill, or split the general mode out (`moved-to-other-cluster` / general skills)? Touches cluster
  scope + REGISTRY filing.
- **F3 — Sibling boundary / trigger de-collision with `analysis-specifications` (P6).** Confirm the
  enrichment (pre-spec) vs gap-review (post-draft) boundary holds and the two trigger sets don't
  collide once both port (convention-wiring de-collision, but reconcile confirms the boundary).

## Decoupling-scan hits (run goal #1 — empirical doctrine test)

HARD: ENRICHMENT.md L27/L57 (`/humaninloop:specify` + "Phase 1"); SPECIFICATION-INPUT.md L1/L3
(command name + caller detection logic); SPECIFICATION-INPUT.md L112/L115/L117 ("supervisor" ×3).
MECHANISM/caller-side: ENRICHMENT.md parse-markers + SPECIFICATION-INPUT.md L9–16 invocation/`missing:[]`.
SOFT: SKILL.md L17 (command name), L25 (`humaninloop:` sibling-skill prefix).
CLEAN: SKILL.md core, SYNTHESIS.md, references/ADAPTIVE-EXAMPLES.md.

**Verdict:** decoupling doctrine **holds and is useful** for this primitive — every deny-list token
sits in the brain/DAG-driven spec-input mode; the pure-craft body is clean. No content-coupling to the
kernel; coupling is orchestration-only and rehomes to the lead. All hits are fixable as
`port-with-edits` decouple actions in the wiring pass; `verify-output`'s decoupling scan must confirm
zero remain post-transform.
```
```

---

**Reconcile flags:** F1 (mount-point + orchestration rehome), F2 (standard-mode scope), F3 (sibling/trigger de-collision with analysis-specifications). All relational — deferred to `reconcile-cluster`.
