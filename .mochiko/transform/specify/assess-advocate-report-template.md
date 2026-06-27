# Assessment — `templates/advocate-report-template.md` (P13)

Run: `specify` cluster transform · Phase 1 (assess) · Producer: `transform-producer`
Source (read-only): `human-in-loop/plugins/humaninloop/templates/advocate-report-template.md`
Consumed by (HIL): the `devils-advocate` advocate-review node **fills** it; the **`state-analyst`** parses the filled report — extracts the `verdict` and drives the DAG gate (`agents/state-analyst.md` L156-158, L347-350), classifies the gaps for routing (L294). Output location bound by the **consumers**, not the template: `{{advocate_report_path}}` = `specs/{feature-id}/.workflow/advocate-report.md` (`commands/plan.md` L262/L435; `templates/context-template.md` L43).
Mochiko thesis: kernel-free adversarial 2-member team — the advocate (validator) fills the verdict; the **lead** (command supervisor) reads it directly to drive the bounded loop + human gate. The DAG-gate/state-analyst plumbing **dissolves to the lead**; the verdict CONTENT survives.

---

## Step 1 — Branch by class

**Class: template → artifact branch.** Weight on placeholders, what consumes it, and path coupling. (Not loop-ownership, not persona/procedure — though this template's *verdict field* is the loop's input surface, so Check 4 carries unusual weight for an artifact.)

The body is a fill-in advocate-review report with five blocks: **header** (`{{feature_id}}` / `{{iteration}}` / `{{timestamp}}`), **Gaps Found** (table: ID/type/description/severity + inlined `Severity`/`Gap type` taxonomies), **Clarifications Needed** (C#: title, related gap, question, three options, impact), **Verdict** (`Status: {{verdict}}` + the 3-state vocabulary `ready | needs-revision | critical-gaps` + `{{verdict_rationale}}`), and **What's Strong** (`{{strengths}}`). Placeholders are `{{mustache}}` style — same convention as the already-ported `codebase-analysis-template.md`. Passive content, filled by the advocate, never executed.

## Step 2 — Fast-path triage gate

1. **Orchestration-coupled?** `gate1 = NO` — the template is an inert artifact-shape; it does **not** depend on a kernel, markdown supervisor, command, or DAG to *function* as a shape (a human or agent can fill it standalone). The dependency direction is *inverted*: the DAG/state-analyst depends on **reading** the template's verdict, not the template on the DAG. That orchestration entanglement is captured as orchestration-coupling in Check 1/Check 4 (rehome to lead), not as a functional dependency of the body. Consistent with the setup precedent (`codebase-analysis-template` `Status: draft|confirmed` flipped by the human gate, scored gate1=NO).
2. **Multi-responsibility / fans out?** `gate2 = YES` — five distinct slot-responsibilities, and it **fans out to three consumers**: (a) the **lead** reads the *verdict* to converge/revise/escalate; (b) the **human gate** consumes the *clarifications*; (c) the **producer** (`requirements-analyst`) reads the *gaps* to revise on a `needs-revision` round.
3. **Emits a non-machine-checkable artifact?** `gate3 = YES` — the filled advocate report is judgment-laden: whether the gaps are real, whether the verdict is justified by the gaps, whether the clarifications are well-posed — all model-judgment, not a schema/version-equality assert.

gate2 + gate3 trip → **full lens** (light; disposition is simple, but the verdict makes Check 4 load-bearing).

## Step 3 — The lens (weighted for artifact branch)

**Check 1 — Orchestration test.** Orchestrated as **content**, not driven.
- **Content-coupling to kernel/DAG/catalog = NONE.** Empirically confirmed: a deny-list scan of the body (`dispatch | workflow-agnostic | state-analyst | devils-advocate | requirements-analyst | DAG | catalog | hil-dag | brain | .humaninloop | verdict_field | gate-node | pass_number`) returns **CLEAN — zero hits**. The template never names its filler agent or its DAG consumer. It is already fully decoupled. (Positive empirical data point for the run's decoupling-doctrine test.)
- **Orchestration-coupling = the consumers, which dissolve to the lead.** In HIL, the `state-analyst` extracted the verdict and ran the gate (L347-350, L156-158); the commands bound the output path. None of that lives in the template body — so there is **nothing in the body to soften**. The rehome of the *consumption* is owned by **P1 (`specify` command, redesign)** + **P4 (`state-analyst`, dissolve)**, surfaced here as F-P13-1 so the chain is not silently broken.

**Check 2 — Role (two altitudes).** Artifact-shape role = **emits-the-shape-the-critic-fills**. Team-role conferred = it makes its caller a **validator/critic** (the adversarial reviewer) — and uniquely, it carries the **loop-driver field** the lead-referee consumes. It is the validator-side analog of `analyst-report-template` (producer-side). The producer↔validator pairing it participates in already exists structurally in the cluster (analyst produces / advocate critiques / lead refereed), so no pairing is owed by this template.

**Check 3 — Independence.** No produce+grade leak in the template (it is neutral content). The independence concern lives one level up — that the *advocate* agent (P3) that fills this is a different agent from the *analyst* producer (P2), which the cluster already enforces. Template is neutral.

**Check 4 — Verdict-sink / loop-driver (load-bearing).** This is the template's most important fact.
- **The verdict is THE loop driver.** HIL mapping (evidence, `state-analyst.md`): `verdict: ready` → freeze + completion (converge) (L156); `verdict: needs-revision` → new pass (loop) (L157); `verdict: critical-gaps` → escalate (L158); verdict extraction at L347-350; gap classification (knowledge/preference/scope) drives routing at L294.
- **Mochiko rehome (1:1):** `ready` → lead converges → human acceptance gate; `needs-revision` → lead opens a new bounded round (contract cap 3); `critical-gaps` → lead escalates to the human gate. The **DAG gate node + state-analyst parse machinery dissolve**; the lead reads the verdict (and the gap set) **directly** from the filled report. → `moved-to-lead` (R5). The clarifications likewise feed the lead's human gate (R3).
- The template **owns the verdict field** (the input surface) but **not** the loop that consumes it. Naming this prevents the silent drop the contract guards against (state-analyst's report-parsing / convergence-watching → rehome to lead).

**Check 5 — Sibling / overlap ("look sideways").**
- **vs `analyst-report-template.md` (P12):** both are domain-agent report shapes sharing a header convention (`feature_id`/`iteration`/`timestamp`), but they are **complementary, not duplicated** — P12 = the *producer's* output report; P13 = the *critic's* adversarial review **with the verdict loop-driver** P12 lacks. Different fillers, different team-roles. **No merge owed** (cf. setup P9/P10 "complementary, not a dedupe"). Reconcile *may* confirm a shared header/metadata convention across P12/P13 — a wiring nicety, not a structural move.
- **Cross-cluster reuse:** the same template is referenced by `plan` / `techspec` / `tasks` (`commands/plan.md` L435, `commands/tasks.md`, `commands/techspec.md`) — it is a **shared substrate, mounted once, filled in multiple workflows** (correct reuse, like `constitution-template` shared by setup P3+P4; like `context-template` listed for 4 workflows). Port once for specify; later clusters **reuse, do not re-port**. Informational cross-ref, not a dedupe.

**Check 6 — Coupling audit.**
- **Hardcoded paths in the body: NONE.** The output location `specs/{feature-id}/.workflow/advocate-report.md` and the `${CLAUDE_PLUGIN_ROOT}/templates/...` reference live in the **consumers** (P1 command, P4 state-analyst L465, P14 context-template L43), not in this template. → the template body carries **no path to rebind** (same finding as setup `constitution-template`/`codebase-analysis-template`: "output location bound by the consumer"). The `specs/.../.workflow/` → `.mochiko/` workspace rebind is owed by **P1/P14**, surfaced as a cross-ref so it is not lost.
- **Grounding rebinds (who *populates* the fields, not the body):**
  - `{{feature_id}}` — in HIL populated by `create-new-feature.sh` / branch naming (an **EXCLUDED brain script**). Kernel-free → the **lead's** mochiko feature/workspace naming. → `kept-but-rebind` (R1a).
  - `{{iteration}}` — in HIL the DAG `pass_number`. Kernel-free → the **lead's** bounded-loop round counter (cap 3). → `kept-but-rebind` (R1b).
  - These are *grounding* rehomes executed by the lead/wiring pass; **neither requires a body edit** (the field text is unchanged), exactly as `constitution-template`'s path ref was `kept-but-rebind` while the body stayed `keep-verbatim`.
- **Determinism boundary:** none in the template — pure document shape; all model-judgment to fill.
- **Placeholder convention:** `{{mustache}}` — matches the ported `codebase-analysis-template.md`; **no normalization owed** (setup established that placeholder style is per-template, not cluster-normalized).

**Check 7 — Conventions + loop placement.**
- **Classification:** inert artifact — no `disable-model-invocation`, no trigger phrasing. Convention-wiring floor = **router/template registration** (hinted entry) + path-rebind audit (**none owed in body**). Optional packaging to match the sibling `codebase-analysis-template` (title + intro line + ```markdown fence) is a wiring/presentation choice applied to a verbatim body — *not* a body treatment (that sibling was assessed `keep-verbatim` and still got the fence).
- **Decoupling scan:** CLEAN (Check 1) — zero deny-list tokens; nothing to scrub.
- **Loop placement:** the template supplies the **verdict surface** (the loop's input) and the **clarification surface** (the human gate's input). It introduces no loop gap; the done-condition / independent-validation / human-gate live on the **lead** + the contract. The verdict-consumption rehome (R5) is what makes the mochiko sound-loop work — owned by reconcile (F-P13-1), not the template.

## Step 4 — Disposition

**Body × Structural = `keep-verbatim` × `standalone`.**

> **Departure from the run hypothesis (`port-with-edits`), stated for the lead to check.** The hypothesis was "soften DAG-gate coupling; keep the verdict." Investigation shows there is **no DAG-gate coupling in the body to soften** — the deny-list scan is CLEAN and there are no paths in the body. The coupling lives entirely in the *consumers* (state-analyst parse logic, command output paths), which rehome to the lead via P1/P4 — that is reconcile/rehome work, not a P13 body edit. Every candidate body change is *optional packaging* (placeholder-style normalization, `iteration→round` rename, a fence wrapper, a Usage-Notes block) which the **setup precedent explicitly classified as wiring nicety, not a body treatment** (`codebase-analysis-template` = `{{mustache}}` + clean → `keep-verbatim`; `constitution-template` = one `kept-but-rebind` ref → `keep-verbatim`). Adding loop-consumption prose to the template would *re-couple* it and is rejected by decoupling-doctrine. The minimalism governor (`keep-verbatim` > `port-with-edits`) therefore selects `keep-verbatim`: I cannot name a single body line that *must* change for mochiko correctness.

- **Body = `keep-verbatim`.** The body is mochiko-clean in substance and form: no kernel/DAG/catalog/MCP token (grep-confirmed), no path, no agent name, sound `{{mustache}}` placeholders, the 3-state verdict vocabulary is the loop-driver content that **survives unchanged**. The grounding rebinds (`feature_id`, `iteration`) and the verdict/clarification consumption are handled by the wiring pass + reconcile rehome (`kept-but-rebind` / `moved-to-lead`), **not** by editing the body.
- **Structural = `standalone`.** One home (`plugins/mochiko/templates/advocate-report-template.md`), filled by the advocate, read by the lead. No merge with P12 (complementary). Cross-cluster reuse (plan/tasks/techspec) is correct single-source reuse, not a split/dedupe. No relational move owed → **no structural `flag-for-reconcile`** on P13.

## Step 5 — Responsibility trace (every responsibility tagged)

| # | Responsibility | Tag |
|---|----------------|-----|
| R1a | Header field `{{feature_id}}` | **kept-but-rebind** — grounding moves from EXCLUDED `create-new-feature.sh`/branch naming → **lead's** mochiko feature/workspace naming (no body edit) |
| R1b | Header field `{{iteration}}` | **kept-but-rebind** — grounding moves from DAG `pass_number` → **lead's** bounded-loop round counter (cap 3); optional text rename `iteration→round` is a wiring nicety, not owed |
| R1c | Header field `{{timestamp}}` | **kept** (framework-agnostic) |
| R2 | Gaps Found shape: table (ID/type/description/severity) + `Severity` (Critical\|Important\|Minor) + `Gap type` (Missing\|Ambiguous\|Edge Case\|Assumption\|Contradiction) taxonomies | **kept** (content); the *consumption* of gaps on a `needs-revision` round (analyst revises; HIL gap-routing classification, state-analyst L294) → **moved-to-lead** (lead sequences the round + reads gaps to route) |
| R3 | Clarifications Needed shape (C#: title, related gap, question, 3 options, impact) | **kept** (content); its *consumption* as **human-gate input** rebinds from the HIL human-clarification DAG node → **moved-to-lead** (lead's `AskUserQuestion` human gate) |
| R4 | Verdict slot: `Status {{verdict}}` + 3-state vocabulary `ready\|needs-revision\|critical-gaps` + `{{verdict_rationale}}` — the loop-driver **field** | **kept** (the verdict CONTENT survives unchanged — the load-bearing item) |
| R5 | Verdict→loop-driver **consumption** (ready→converge / needs-revision→new round / critical-gaps→escalate) | **moved-to-lead** — DAG gate + state-analyst parse machinery (L156-158, L347-350) dissolve; the lead reads the verdict directly to drive the bounded loop + human gate. Rehome executed in reconcile via P1/P4 → **F-P13-1** ensures it lands. Template is the origin surface only |
| R6 | "What's Strong" / `{{strengths}}` slot | **kept** (positive-feedback content; no loop role) |
| R7 | `{{mustache}}` placeholder convention | **kept** (matches `codebase-analysis-template`; no normalization owed) |
| R8 | Discoverability / placement (router template-registration as hinted entry; output-path audit) | **kept-but-rebind** (convention-wiring floor; the `specs/.../.workflow/` → `.mochiko/` output-path rebind lives in the **consumers** P1/P14, not this body) |
| R9 | Cross-cluster availability (also filled in plan/techspec/tasks) | **kept** for specify; single-source **reuse** by later clusters (not a re-port, not a dedupe) |

No responsibility left untagged; no silent drop.

## Reconcile flags

- **F-P13-1 (soft — chain-integrity / rehome-orchestration).** The **verdict→loop-driver consumption** (R5) + the **clarification→human-gate** feed (R3) + the **gap→revision/routing** sequencing (R2) must **rehome onto the LEAD** during reconcile (as part of P1 `specify`-command redesign + P4 `state-analyst` dissolution). The template is the *origin surface* for all three; if the lead does not pick up the verdict consumption, the bounded loop is silently broken. Evidence the chain exists in HIL: `state-analyst.md` L156-158 (verdict→completion/new_pass/escalate), L347-350 (verdict extraction), L294 (gap classification→routing). **The template's own disposition (`keep-verbatim × standalone`) stands regardless** — this is a rehome guarantee for reconcile, not a structural move on P13. Directly serves the run goal "trace the verdict→loop-driver responsibility to the lead."

**Cross-refs (informational — not flags, not blocking):**
- **vs P12 `analyst-report-template`:** complementary report shapes (producer report vs critic-review-with-verdict), different fillers — **no merge**. Reconcile MAY confirm a shared header/metadata convention across P12/P13 (wiring nicety).
- **Cross-cluster:** P13 is a **shared artifact** referenced by plan/techspec/tasks; reconcile/lead should mark it shared so later cluster ports **reuse** rather than re-port.
- **Output-path rebind** (`specs/{feature-id}/.workflow/advocate-report.md` → `.mochiko/` workspace) lives in the **consumers** (P1 command, P14 context-template), not this body — surfaced so the rebind is not lost when those primitives port.
- **Decoupling-doctrine data point:** body deny-list scan is **CLEAN** (zero tokens) — already fully decoupled; positive evidence for the run's empirical decoupling test.

No blocking structural flag. Disposition is decided.

---

### Summary block

```
ASSESSMENT: advocate-report-template.md (P13)
Class:        template → branch artifact (advocate-review report shape; verdict = loop-driver surface)
Triage:       gate1=n gate2=y gate3=y  [full-lens, light; Check 4 load-bearing]
Disposition:  keep-verbatim × standalone   (DEPARTS from port-with-edits hypothesis — see Step 4:
              no DAG-gate coupling in the body to soften; deny-list scan CLEAN; coupling lives in
              the consumers, which rehome to the lead via P1/P4, not a P13 body edit)
Trace:
  - R1a header {{feature_id}}                          → kept-but-rebind (EXCLUDED create-new-feature.sh → lead workspace naming)
  - R1b header {{iteration}}                           → kept-but-rebind (DAG pass_number → lead round counter, cap 3)
  - R1c header {{timestamp}}                           → kept
  - R2  Gaps Found shape + severity/type taxonomies    → kept (content); gap consumption/routing → moved-to-lead
  - R3  Clarifications Needed shape                     → kept (content); consumption → moved-to-lead (human gate)
  - R4  Verdict slot (ready|needs-revision|critical-gaps + rationale)  → kept (verdict CONTENT survives — load-bearing)
  - R5  verdict→loop-driver CONSUMPTION                 → moved-to-lead (DAG gate + state-analyst dissolve; reconcile via P1/P4 — F-P13-1)
  - R6  "What's Strong"/strengths slot                 → kept
  - R7  {{mustache}} placeholder convention            → kept
  - R8  discoverability/placement (output-path in consumers, not body)  → kept-but-rebind (wiring floor)
  - R9  cross-cluster availability (plan/techspec/tasks) → kept for specify; reuse later, no re-port
Reconcile flags:
  - F-P13-1 (soft) verdict→loop-driver + clarification→human-gate + gap→revision consumption must
    rehome onto the LEAD (reconcile rehome via P1 command + P4 state-analyst dissolution); template is
    origin surface only; disposition stands regardless.
  - cross-refs (not flags): complementary to P12 (no merge); shared cross-cluster artifact (reuse, no re-port);
    output-path rebind owned by P1/P14; body deny-list scan CLEAN (decoupling-doctrine positive).
```
