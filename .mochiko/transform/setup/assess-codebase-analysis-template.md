# Assessment — `templates/codebase-analysis-template.md` (P9)

Run: `setup` cluster transform · Phase 1 (assess) · Producer: `transform-producer`
Source (read-only): `human-in-loop/plugins/humaninloop/templates/codebase-analysis-template.md`
Consumed by: `analysis-codebase` (P6), **Setup-Brownfield mode** — *"Output: `.humaninloop/memory/codebase-analysis.md` following `codebase-analysis-template.md`"* (P6 SKILL.md L136).

---

## Step 1 — Branch by class

**Class: template → artifact branch.** Weight on placeholders, what consumes it, and path coupling.

This is the **output shape for the brownfield codebase-analysis report** — a two-part document: **Part 1 Inventory (Factual)** (project identity, directory structure, detected patterns, domain entities, external deps) and **Part 2 Assessment (Judgment)** (strengths to preserve, inconsistencies, Essential-Floor status, recommended constitution focus), plus an Appendix (Detection Method). Placeholders are `{{mustache}}` style; status enums are inlined (`present|partial|absent`, `low|medium|high`, `draft|confirmed`). Passive content, filled by P6.

## Step 2 — Fast-path triage gate

1. **Orchestration-coupled?** `gate1 = NO` — consumed-as-content by the `analysis-codebase` skill; it does not depend on a kernel, supervisor, command, or DAG to *function* as a shape. (P6 is invoked by the supervisor and bundles `detect-stack.sh`, but that is P6's coupling. The Appendix *names* `detect-stack.sh` as the detection method, which is a documentation reference, not an execution dependency of the template.) No kernel/DAG/catalog/MCP content.
2. **Multi-responsibility / fans out?** `gate2 = NO` — one template, **one direct consumer** (P6 Setup-Brownfield mode), one responsibility: define the codebase-analysis artifact shape. (The *filled* artifact then feeds brownfield-constitution + evolution-roadmap downstream, but that is the artifact's fan-out, not the template's — the template is a single shape with a single filler.)
3. **Emits a non-machine-checkable artifact?** `gate3 = YES` — the `codebase-analysis.md` this template shapes is **judgment-laden**: Part 2 is explicitly "Assessment (Judgment)" — strengths, inconsistencies + severity, Essential-Floor `present/partial/absent` calls, recommended constitution focus. Correctness is model-judgment, not a schema or version-equality assert.

gate3 trips → **full lens** (light; disposition itself is simple, but the judgment-laden output carries a cross-ref signal).

## Step 3 — The lens (weighted for artifact branch)

**Check 1 — Orchestration test.** Orchestrated as **content**. Content-coupling to kernel = **NONE**. No orchestration owned by the template body. The Appendix's `detect-stack.sh` mention is a documentation pointer (the script ships with P6, in scope). Nothing to re-home from the template.

**Check 2 — Role (two altitudes).** Artifact-shape role = **emits-the-shape-a-producer-fills**. Team-role conferred = makes its caller (`analysis-codebase` via `principal-architect`) a **producer** of the brownfield analysis. Confers no validator role.

**Check 3 — Independence.** No produce+grade leak in the template. Neutral. (The *judgment* in the filled artifact is what raises an independent-validator question for P6 — but that is P6's flag, already raised as analysis-codebase F3; this template is the structure such a validator would grade against, see Check 5.)

**Check 4 — Verdict-sink / loop-driver.** The filled artifact is read at the setup **Phase-2 human checkpoint** (Confirm/Edit/Reject), and on Edit/Reject the supervisor loops back to re-run P6. The template owns no loop/verdict; it supplies the *shape* the human reviews and the `Status: draft|confirmed` marker the checkpoint flips.

**Check 5 — Sibling / overlap.**
- **vs `codebase-inventory-schema.json` (P10):** both describe "analyze the codebase" outputs, but **different artifacts for different modes** — this template = the **Setup-Brownfield markdown** report (the setup-cluster path); the schema = the **Brownfield-mode JSON inventory** (the spec/plan collision path). Complementary, not duplicated. No dedupe owed between P9 and P10.
- **vs `analysis-codebase` SKILL.md (P6):** the skill's Essential-Floor assessment tables (Security/Testing/Error-Handling/Observability) map onto this template's "Essential Floor Status" + per-category detail blocks — **procedure (skill) vs artifact-shape (template)**, complementary; keep in sync.
- **Cross-ref (not a P9 flag):** because the output is judgment-laden (gate3), whether P6's brownfield analysis needs an **independent validator** is open — but that decision belongs to **P6 (analysis-codebase F3)**, not to this template. This template is the *done-condition surface* such a validator would check against. Surfaced as a cross-ref only.

**Check 6 — Coupling audit.**
- **No `.humaninloop/` hardcoded path in the body.** The output location (`.humaninloop/memory/codebase-analysis.md`) is bound by the *consumer* P6, not by this template — the template body carries no plugin/memory path to rebind.
- **Documentation reference:** Appendix names `detect-stack.sh` (ships with P6, in scope) — informational, no rebind needed beyond staying consistent with P6's placement.
- **Determinism boundary:** none in the template itself — it shapes both the factual inventory (Part 1, grounded by the script + scans) and the judgment assessment (Part 2). The boundary lives in P6, not here.
- **Placeholder convention:** `{{mustache}}`. Differs from `constitution-template.md`'s `[BRACKET]` style → a **cross-template convention inconsistency** at cluster level. Optional normalization in the wiring pass; not a blocker, not a body redesign.

**Check 7 — Conventions + loop placement.**
- **Classification:** inert artifact; convention-wiring floor = router/template registration + path-rebind audit (none owed in body). No trigger phrasing.
- **Loop placement:** the template supplies the **review surface** for the Phase-2 human gate and the `Status: draft→confirmed` state the gate sets. It introduces no loop gap. The independent-validation question is P6's (cross-ref above), not the template's.

## Step 4 — Disposition

**Body × Structural = `keep-verbatim` × `standalone`.**

- **Body = `keep-verbatim`.** The two-part Inventory/Assessment structure is mochiko-clean — no kernel/DAG/catalog, no `.humaninloop/` path in the body, sound `{{mustache}}` placeholders, clear status enums and usage notes. Nothing earns an edit. (Cross-template placeholder-style normalization is an *optional* cluster-wide wiring nicety, not a body change this primitive needs.) Minimalism governor → `keep-verbatim` over `port-with-edits`.
- **Structural = `standalone`.** One home (`plugins/mochiko/templates/codebase-analysis-template.md`), one filler (P6). No relational move owed. The producer↔validator question it brushes against is owned by P6, not resolved by moving this template → **no structural `flag-for-reconcile` on P9.**

## Step 5 — Responsibility trace (every responsibility tagged)

| # | Responsibility | Tag |
|---|----------------|-----|
| R1 | Define Part-1 Inventory shape: project identity, directory structure, detected patterns (architecture/naming/error/test), domain entities, external deps | **kept** (keep-verbatim) |
| R2 | Define Part-2 Assessment shape: strengths to preserve, inconsistencies + severity, Essential-Floor status table | **kept** |
| R3 | Essential-Floor detail blocks (Security / Testing / Error-Handling / Observability sub-checks) | **kept** (mirrors P6's assessment procedure; keep in sync — complementary) |
| R4 | "Recommended Constitution Focus" handoff section (feeds the constitution phase downstream) | **kept** (downstream handoff to P3/P4; preserved as artifact content) |
| R5 | Appendix: Detection Method table (names `detect-stack.sh` + pattern-matching methods) | **kept** (documentation reference; script ships with P6, in scope) |
| R6 | Status / severity / floor-status enums + Usage Notes (`draft\|confirmed`, `low\|medium\|high`, `present\|partial\|absent`) | **kept** |
| R7 | `{{mustache}}` placeholder convention | **kept** (optional cluster-wide normalization vs `[BRACKET]` style is a wiring nicety, not owed) |
| R8 | Discoverability / placement (router/template registration; path-rebind audit — none owed in body) | **kept-but-rebind** (convention-wiring floor) |

No responsibility left untagged; no silent drop.

## Reconcile flags

- **None blocking.** Disposition `keep-verbatim × standalone` is decided.
- **Cross-ref (informational, not a P9 flag):** the judgment-laden output this template shapes is the artifact that **analysis-codebase F3** (independent-validator pairing / loop placement) concerns. If reconcile pairs P6 with an independent analysis-validator, **this template is the done-condition surface** that validator grades against — no change to P9's own disposition. Also cross-refs P10: P9 (markdown, setup path) and P10 (JSON, spec/plan path) are complementary, not a dedupe.

---

### Summary block

```
ASSESSMENT: codebase-analysis-template.md (P9)
Class:        template → branch artifact (Setup-Brownfield analysis output shape; consumed by P6)
Triage:       gate1=n gate2=n gate3=y  [full-lens, light]
Disposition:  keep-verbatim × standalone
Trace:
  - R1 Part-1 Inventory shape (identity/structure/patterns/entities/deps)   → kept
  - R2 Part-2 Assessment shape (strengths/inconsistencies/floor-status)      → kept
  - R3 Essential-Floor detail blocks (Security/Testing/Errors/Observability) → kept (mirrors P6; keep in sync)
  - R4 Recommended Constitution Focus handoff section                        → kept (downstream handoff to P3/P4)
  - R5 Appendix Detection Method (names detect-stack.sh)                      → kept (script ships with P6, in scope)
  - R6 status/severity/floor enums + Usage Notes                             → kept
  - R7 {{mustache}} placeholder convention                                   → kept (cross-template normalization optional)
  - R8 discoverability/placement                                             → kept-but-rebind (wiring floor)
Reconcile flags:
  - none blocking. Cross-ref: judgment-laden output ↔ analysis-codebase F3 (this template = validator's done-condition surface);
    complementary to P10 (markdown setup path vs JSON spec/plan path) — not a dedupe.
```
