# Assessment — `authoring-requirements` (P7, specify cluster)

**Producer:** `mochiko:transform-producer` · **Skill:** `mochiko:assess-primitive` · **Date:** 2026-06-27
**Source:** `human-in-loop/plugins/humaninloop/skills/authoring-requirements/` (SKILL.md + `references/RFC-2119-KEYWORDS.md` + `references/EDGE-CASES.md` + `scripts/validate-requirements.py`)
**Status:** ASSESS ONLY — no edits made.

---

## ASSESSMENT (skill output format)

```
ASSESSMENT: authoring-requirements
Class:        skill → branch PLAYS-a-role
Triage:       gate1=n gate2=y gate3=y  [full-lens]   (≥1 yes → full 7-check lens)
Disposition:  port-with-edits × standalone
Trace:
  - FR-XXX functional-requirements authoring (RFC 2119 MUST/SHOULD/MAY, numbering, tech-agnostic) → kept
  - Edge-case identification (3-5 boundaries, 5 categories + discovery process)               → kept
  - SC-XXX success-criteria authoring (measurable, user/business outcomes)                     → kept
  - Key-entities conceptual description (optional)                                             → kept
  - Technology-agnostic / no-implementation-leakage discipline (Common Mistakes, Good/Bad)     → kept  [dedupe candidate w/ sibling — reconcile]
  - RFC-2119 keyword reference guidance (RFC-2119-KEYWORDS.md)                                  → kept
  - Deterministic format-linter (validate-requirements.py: format/sequence/RFC-presence/banned-terms/outcome-focus) → kept  [skill-local assert; determinism boundary — NOT the substantive validator]
  - Quality Checklist (pre-finalize self-check)                                                → kept
  - model-invocation trigger `description`                                                     → kept-but-rebind  [rephrase user-utterance → work-context triggers; agent-consumed]
  - "When NOT to Use" sibling cross-refs (humaninloop:patterns-api-contracts / patterns-entity-modeling / authoring-user-stories) → kept-but-rebind  [humaninloop: → mochiko:; 2 of 3 targets DEFERRED → dangling]
  - example invocation path (`path/to/spec.md`)                                                → kept-but-rebind  [→ .mochiko/ spec location]
  - classification tag (absent in HIL)                                                         → assigned model-invoked (default) via wiring pass
  - router registration (absent in HIL)                                                        → added via wiring pass
Reconcile flags: sibling-overlap/dedupe with authoring-user-stories (see below) — NOT a merge
```

---

## Step 1 — Class / branch

**skill → PLAYS-a-role.** It is one of the `requirements-analyst`'s two authoring procedures (confirmed: `agents/requirements-analyst.md` frontmatter `skills: authoring-requirements, authoring-user-stories`). Weighting per branch: consumed-procedure vs emits-artifact, trigger reliability, sibling overlap, decoupling.

## Step 2 — Fast-path triage gate

| Gate | Q | Verdict |
|------|---|---------|
| 1 | Orchestration-coupled (kernel / md-supervisor / command / DAG to function)? | **no** — pure self-contained authoring guidance + a local script; survives without the brain (ROADMAP test passes). The HIL DAG/catalog sequences the *agent*, not this skill; body has zero brain/DAG/catalog tokens. |
| 2 | Multi-responsibility / fans out? | **yes** — FR + edge-cases + SC + entities + linter; feeds the spec doc consumed by `devils-advocate` and the downstream plan cluster. |
| 3 | Emits artifact whose correctness is NOT machine-checkable? | **yes** — *format* is machine-checkable (the linter); *substance* (right requirements, genuine tech-agnosticism in meaning, high-impact edge cases, measurable outcomes) needs model judgment. |

≥1 yes → **full lens**. Gates 2 + 3 tripped.

## Step 3 — The 7-check lens (weighted for PLAYS-a-role / skill)

1. **Orchestration test.** Content-coupling: **none** (no kernel/DAG/catalog/MCP in body or refs — grep clean). Orchestration-coupling: the skill is *consumed* by the `requirements-analyst` agent via the Skill tool; in HIL the DAG sequences the agent. Nothing orchestration-bound lives *in* this skill, so nothing re-homes **from** it. Loop/sequencing belongs to the command (P1), not here.

2. **Role (two altitudes).** Skill-role = **emits-artifact** (the FR/edge-case/SC section of a spec; also run as a consumed step, but its output is a reviewable artifact). Team-role conferred = **producer** — running it makes the analyst a producer. It is *not* a checklist/grading skill (the embedded Quality Checklist is a producer self-check, not an independent grade) → **not promote material**.

3. **Independence.** The analyst mounts `authoring-requirements` + `authoring-user-stories` — **both producer skills, no grading skill**. No self-grade leak. Independence is structural at the cluster level: analyst (produce) ≠ `devils-advocate` (grade, via `analysis-specifications`). ✓

4. **Verdict-sink / loop-driver.** This skill emits **no verdict**. Verdict-sink = the advocate's review; FAIL-loop (revise-on-gaps) is driven by the command supervisor (lead). No loop owned here → nothing to rehome.

5. **Sibling / overlap.** The load-bearing check → see the dedicated section below. → **flag-for-reconcile** (dedupe signal, NOT merge).

6. **Coupling audit.**
   - Hardcoded paths: **none** `.humaninloop/`. Light rebinds only: example `path/to/spec.md` → `.mochiko/` spec location; `humaninloop:` skill cross-refs → `mochiko:`.
   - Prerequisites/handoffs: assumes a feature description exists upstream (enrichment via `analysis-iterative`) and writes into the spec doc (`spec-template`). These are command-owned handoff edges, not skill-owned.
   - **Determinism boundary:** `validate-requirements.py` is a **deterministic format linter** (FR/SC numbering + sequence, RFC-keyword *presence*, banned-substring tech scan, technical-metric regex). It is a Tier-1 self-check, **not** the substantive validator. Substance stays model-judgment, owned by the advocate. The script is skill-local (not brain/catalog/DAG/MCP) → permitted and **kept**, consistent with mochiko keeping skill-local helpers (e.g. `analysis-codebase`'s `detect-stack.sh`). (Defensible alternative = drop the linter and lean on model + advocate; minimalism + the contract's "Tier-1 sub-checks layered where they exist" favor keep. Keep is not a gated disposition, so no human gate needed.)

7. **Conventions + loop placement.**
   - Classification: **absent** → assign **model-invoked** (default; agent-consumed) via wiring.
   - Discoverability: not yet in a mochiko router → register via wiring.
   - Reliable model-invocation: current `description` is the **"when the user says …"** user-utterance pattern. For an **agent-consumed** skill, mochiko convention says describe the **work context** (avoid false auto-trigger expectations) → `port-with-edits` rephrase (keep graded MUST/SHOULD trigger phrases, anchor them to authoring work). **This is the main body edit.**
   - Agent↔skill composition / **decoupling scan: CLEAN.** No sibling-agent names, no "dispatch," no injected workflow modes/paths/phases, no "workflow-agnostic"/independence-by-declaration meta-label (grep + manual read of SKILL.md + both refs). Pure procedure; persona correctly lives in the agent, not here. Independence is stated by *role*, not agent name. ✓
   - Producer↔validator pairing: the artifact's independent validator exists at cluster level (`devils-advocate` + `analysis-specifications` — different agent, different skill, guaranteed structurally). This skill is the producer half; the linter is a degenerate deterministic self-check, not that validator.
   - Sound-loop: the enclosing specify loop's done-condition / independent-validation / human gate are the **command's** concern (P1 + contract), not this skill's. No loop gap owned here.

## Step 4 — Disposition

**`port-with-edits × standalone`.**

- **Body = port-with-edits** (not keep-verbatim, not redesign): the body is high-quality and mochiko-clean in substance (FR format, RFC 2119, edge-case taxonomy, SC rules, entities, checklists, Common Mistakes — all kept). Edits are **localized**: (a) rephrase the `description` from user-utterance triggers to work-context triggers; (b) rebind `humaninloop:` → `mochiko:` namespaces; (c) rebind the example spec path. Not `keep-verbatim` because those fixes are required; not `redesign` because the approach assumes no kernel/DAG/catalog and is right for mochiko.
- **Structural = standalone**: this is the analyst's distinct FR/SC/edge-case producer procedure, mounted on `requirements-analyst`. Its placement does **not** depend on the sibling — only the *optional* substrate dedupe does (flagged, below).

## Step 5 — Responsibility trace

Complete trace in the ASSESSMENT block above. **No `dropped` responsibilities** → no silent loss, nothing requiring a lead-accepted drop reason. Three `kept-but-rebind` (description triggers, sibling namespaces, example path) and two new wiring assignments (classification, router); everything else `kept`. One `kept` (tech-agnostic discipline) carries a cross-skill `dedupe` candidate that only reconcile can assign.

---

## SIBLING-OVERLAP FINDING → flag-for-reconcile (the weighted question)

**Pair:** `authoring-requirements` (P7) ↔ `authoring-user-stories` (P8) — co-mounted producer skills on `requirements-analyst`.

**What they genuinely share (dedupe signals):**
1. A **technology-agnostic / no-implementation-leakage discipline substrate** — duplicated prose + Good/Bad examples in both ("describe WHAT not HOW", observable/measurable outcomes, no tech references).
2. A **near-identical deterministic validation-script scaffold** — `validate-requirements.py` and `validate-user-stories.py` share ~80% structure (find→check pattern, `checks`/`summary` JSON, identical `main()` + exit-code shape) but check **different** artifacts.

**What they do NOT share (the reason this is NOT a merge):**
- **Distinct core artifacts:** FR-XXX system-capability statements vs P#/Given-When-Then actor-journey stories — different spec sections, different formats, different numbering, different scripts.
- **HIL deliberately separated them:** `authoring-requirements`'s own "When NOT to Use" redirects user-story work to `authoring-user-stories` ("this skill focuses on the underlying requirements"); the sibling's Common Mistakes draw the same boundary.

**Therefore:** **NOT** `merge-into-sibling` (no thin-variant-over-shared-core). Recommended resolution = **`standalone` for both**, with an **optional** `dedupe` of the shared discipline substrate *only if* reconcile judges a shared reference worth it. My lean: **keep distinct** for both the discipline prose and the two scripts — factoring them manufactures cross-skill coupling for marginal benefit, against kernel-free minimalism. Reconcile owns the final call (it needs P8's assessment in hand).

**Flag payload for `reconcile-cluster`:** `authoring-requirements ↔ authoring-user-stories : shared tech-agnostic discipline substrate + shared validation-script scaffold → dedupe candidate; resolve standalone-vs-factor-shared-substrate; NOT a merge.`

## Decoupling-scan hits (run-goal: empirical decoupling-doctrine test)

**Zero deny-list hits.** Grep + manual read of SKILL.md and both reference files found **no** sibling-agent names, **no** "dispatch," **no** injected workflow modes/paths/phases, **no** "workflow-agnostic"/independence-by-declaration meta-label. The skill is already a clean, role-stated, decoupled procedure — it states its job by *work*, not by naming the analyst or the workflow. The only caller-coupling-adjacent items are the `humaninloop:` skill cross-references in "When NOT to Use" (a namespace `kept-but-rebind`, not a coupling violation), of which `patterns-api-contracts` and `patterns-entity-modeling` point at **DEFERRED** (out-of-specify-core) skills and will **dangle** until the plan/design track ports — hand to the wiring pass / note for reconcile.

---

**Done-condition (this assessment):** every responsibility tagged ✓ · no untagged responsibility ✓ · relational move flagged not guessed ✓ · ASSESS-only (no edits) ✓.
