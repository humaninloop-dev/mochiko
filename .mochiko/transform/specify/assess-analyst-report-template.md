# Assessment — `analyst-report-template.md` (P12)

**Run:** transform-cluster / `specify` · **Assessed by:** `transform-producer` · **Date:** 2026-06-27
**Source:** `human-in-loop/plugins/humaninloop/templates/analyst-report-template.md`
**Skill:** `mochiko:assess-primitive` (ASSESS ONLY — no edit applied)

---

## ASSESSMENT (standard format)

```
ASSESSMENT: analyst-report-template.md
Class:        template → branch artifact
Triage:       gate1=n  gate2=y  gate3=n   [full-lens]  (gate2 trips: fans out to 2 consumers)
Disposition:  port-with-edits × standalone
              (+ reconcile flags on edit-DEPTH and cross-template header consistency)
Trace:
  - Structure the analyst's self-report (producer handoff disclosure)   → kept
  - "What I Created" counts table (stories/edge-cases/FRs/entities/SC)  → kept-but-rebind (slim/demote; depth → reconcile)
  - Feed the DAG outcome-trajectory metric (count-per-pass trend)        → dropped + reason (DAG/parse-and-advance excluded kernel-free; lead reads report + spec.md directly)
  - Capture Assumptions Made (id / assumption / rationale)              → kept
  - Capture Summary (producer narrative)                                 → kept
  - Capture free-form Notes                                             → kept
  - Report identity header (feature_id / iteration / timestamp)         → kept-but-rebind ({{iteration}} → round vocabulary)
  - Be the "Report format: follow <template>" dispatch target          → kept-but-rebind (path → mochiko) + dispatch reference moved-to-lead
  - Be heading-parsed by state-analyst (parse-and-advance extraction)   → moved-to-lead (lead reads report directly; parsing layer dissolves)
Reconcile flags:
  - DEPTH of the body edit (how far to slim/reorient the counts-first shape) depends on the
    state-analyst→lead rehome (what the lead needs to read). Confirm in reconcile alongside P4 dissolution.
  - Shared header convention with advocate-report-template (P13): keep any header rebind
    ({{iteration}}→round) IDENTICAL across the matched producer/critic report pair.
```

---

## Class / branch

**template → artifact.** Inert markdown skeleton with `{{placeholder}}` slots; it is filled by a producer and read by consumers. What matters for this branch: **placeholders**, **who fills / who consumes it**, and **path coupling** — weighted exactly as the task directs.

## Triage gate

| Gate | Q | Verdict | Evidence |
|------|---|---------|----------|
| 1 | Orchestration-coupled (does the **body** need a kernel/supervisor/command/DAG to function)? | **no** | The body is an inert report skeleton — no kernel/DAG/catalog/MCP token, no path, no driver reference inside it. Consumption *was* orchestration-mediated (state-analyst referenced + parsed it), but that coupling lives in the state-analyst + catalog, not in this file. |
| 2 | Multi-responsibility / fans out? | **yes** | Fans out to **two** consumers. `specify-catalog.json` makes `analyst-report.md` a **required** consume of the `advocate-review` gate; the state-analyst (now the lead) is the other reader. |
| 3 | Emits an artifact whose correctness is NOT machine-checkable? | **no** | A template does not itself emit a gradeable artifact. The filled report's judgment content (are the assumptions sound?) is reviewed by the advocate — but that producer↔critic pairing already exists at the **agent** layer; this template introduces no new validator need. |

One "yes" (gate2) → **full 7-check lens** (correctly; the fan-out + the brain-parsing question deserve the lens).

## Placeholders (inventory)

| Placeholder | Kind | Disposition |
|-------------|------|-------------|
| `{{feature_id}}` | identity | kept (generic) |
| `{{iteration}}` | **loop coupling** | **kept-but-rebind** — "iteration" = the DAG's `pass_number` (catalog INV-004: max 5 passes). Mochiko's loop is the supervisor's bounded **round** loop (contract round-cap = 3). Rebind term to round/loop vocabulary; the concept survives. |
| `{{timestamp}}` | identity | kept (generic) |
| `{{user_story_count}}` `{{edge_case_count}}` `{{requirement_count}}` `{{entity_count}}` `{{criteria_count}}` | **metrics / parse surface** | kept-but-rebind (slim/demote — see brain-parsing verdict). Each counts a section of `spec-template.md`; redundant with spec.md, which the advocate is required to read. |
| `{{summary}}` | content | kept (primary decision-relevant content for a direct-reading lead) |
| `{{assumption}}` / `{{rationale}}` | content | kept (the advocate stress-tests assumptions; high value) |
| `{{notes}}` | content | kept |

No `.humaninloop/` paths, catalog paths, or script paths appear **inside** the body — placeholder coupling is the only body-internal coupling, and it is limited to `{{iteration}}`.

## Who fills it / who consumes it

- **Fills it (unchanged):** the `requirements-analyst` producer. Catalog `analyst-review` node `produces: ["spec.md", "analyst-report.md"]` — the analyst authors both the product (spec.md) and this self-report.
- **Consumed it in HIL:** the **state-analyst's `parse-and-advance`** — heading-based extraction (state-analyst.md L386-390: `## What I Created`→metrics, `### Summary`→summary, `## Assumptions Made`→assumptions), feeding `hil-dag record` evidence/trace JSON and the DAG outcome-trajectory metric. **Also** the `devils-advocate` (catalog: `advocate-review` requires `analyst-report.md`).
- **Consumes it in mochiko:** the **lead/referee** reads it directly (state-analyst dissolved — no parse-and-advance layer) **and** the **devils-advocate** still reads it as critic context. So the consumer set goes `{parser, advocate}` → `{lead, advocate}` — both readers are now humans-in-the-loop-style direct readers, not a machine parser.

## Brain-parsing verdict (the run-goal headline)

**Did its structure assume brain parsing? — Partially, by *accommodation*, never by *dependency*.**

- **No content-coupling / no dangling references.** The body contains zero references to state-analyst, DAG, catalog, MCP, `parse-and-advance`, or any excluded/deferred primitive, and zero hardcoded paths. The decoupling-doctrine scan (path coupling + dangling refs to excluded primitives) comes back **clean** for this file.
- **Shape accommodated the parser.** Two parser-oriented residues exist: (1) the **stable section headings** were the state-analyst's exact-match extraction anchors, and (2) the **"What I Created" counts table** existed substantially as a machine-extraction surface — it fed the parser's evidence construction and the DAG's count-per-pass trajectory. The counts duplicate `spec-template.md`'s own sections, which the advocate must read anyway; for a lead+advocate reading prose directly, the counts are low marginal value.
- **It survives the parser's removal as a valid human report.** Stable headings still help a direct reader; nothing breaks. This is an *accommodation*, not a *dependency* — distinguishing it from a rigid machine schema (contrast the enriched-input's `<!-- ENRICHMENT_COMPLETE -->` sentinel, which IS dependency-grade). Hence **port-with-edits**, not redesign, and the parse layer rehomes cleanly to the lead.

## Lens findings (artifact-weighted)

1. **Orchestration test.** Orchestrated by the DAG/state-analyst externally (NL-prompt "Report format: follow …" reference + parse-and-advance). Content-coupling: **none** (clean body). Orchestration-coupling: the *reference to* and *parsing of* the template — both live outside the file and rehome to the lead. The "point the analyst at this template" dispatch wiring → **moved-to-lead** (lead's dispatch / `agent-dispatch.md`).
2. **Role (two altitudes).** Skill-role: it shapes a **producer's self-report / handoff**. Team-role: filling it = producer disclosure; reading it confers lead-referee (drives loop) or critic-context (advocate). It needs **no validator partner of its own**.
3. **Independence.** N/A to a template, and clean: this report is the analyst's **self-disclosure**, not a grade. The FAIL/PASS verdict lives in the **advocate-report** (catalog `advocate-review.verdict_field`), authored by a different agent. Note for the lead: weight the advocate's independent verdict for loop control, never the analyst's self-summary here.
4. **Verdict-sink / loop-driver.** This report is **not** a loop-driver — it carries no verdict; it is producer-side input to the loop. The loop is driven by the advocate's verdict. Consumption rehomes to the lead; the DAG's trajectory metric (built from the counts) is a DAG responsibility dropped with the kernel.
5. **Sibling / overlap.** Matched **producer/critic report pair** with `advocate-report-template.md` (P13) — identical header block (`> Feature / Iteration / Generated`) and `---` section style. **Not a merge** (distinct content). → reconcile flag: keep header rebinds consistent across the pair. Also note `context-template.md` (P14) references the analyst-report path; when P14 is absorbed-into-lead, that path reference rehomes to the lead's handoff. And `spec-template.md` (P11) carries its own `## Assumptions` — overlaps in spirit with this report's `## Assumptions Made`, but distinct purpose (spec content vs producer authoring disclosure); **not a dedupe**.
6. **Coupling audit.** Body-internal hardcoded paths: **none**. External references TO it (`${CLAUDE_PLUGIN_ROOT}/templates/analyst-report-template.md`, `{feature_dir}/.workflow/analyst-report.md`) live in the catalog/state-analyst and rebind to the mochiko plugin/workspace paths. Prerequisite/handoff: companion to spec.md (same producer, same node). Determinism boundary: entirely model-judgment content (the counts are analyst-counted, not script-emitted) → no degenerate-validator concern.
7. **Conventions + loop placement.** Classification tags 1–3 (user/model-invoked, router, triggers) are **N/A for a template** (skill/agent-only). The live wiring-pass items are **path-rebinding** (step 4) and the **decouple scan** (step 5) — decouple comes back clean (no sibling-agent name, no "dispatch," no injected modes/paths/phases, no "workflow-agnostic" label in the body). **Mochiko template-shape precedent** (`plugins/mochiko/templates/codebase-analysis-template.md`): `# Title` + one-line "This template defines the structure for …" + fenced ```` ```markdown ```` body + `## Usage Notes`. Adopting this wrapper is a convention-alignment edit (part of why this is port-with-edits, not keep-verbatim).

## Disposition rationale (minimalism governor)

`keep-verbatim` was considered and **rejected** — three real (not cosmetic) edits are earned: (a) `{{iteration}}` rebind off DAG-pass vocabulary; (b) reorient the parser-oriented, counts-first shape for a lead that now reads the report itself (lead with Summary / Assumptions / what-changed-this-round; demote or slim the spec-duplicating counts); (c) align to the mochiko template wrapper convention. None rise to `redesign` — the body is clean, portable, and survives the parser's removal intact. **Structural = `standalone`**: it remains one real producer artifact in `templates/`, filled by the analyst and read by the advocate + lead. It does **not** absorb-into-lead (contrast P14 context-template, which is pure orchestration handoff state) because this is a genuine producer-authored artifact, not in-session orchestration state.

## What is explicitly NOT decided here (handoff to reconcile)

- The **depth** of edit (b) — how far to slim/reorient — is gated on the **state-analyst → lead rehome** (P4): what the lead actually needs to read. Decide in reconcile alongside the P4 dissolution.
- **Header consistency** with `advocate-report-template.md` (P13): rebind the shared header identically across the pair — a cross-primitive call.
- All structural moves remaining `standalone` here; no split/merge/promote/absorb proposed.
