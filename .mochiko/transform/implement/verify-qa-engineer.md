# Phase 4 Verify — P3 `qa-engineer` (implement cluster)

**Verified:** 2026-07-01 · **Validator:** `mochiko:validator` (independent — did NOT author the artifact) · **Skill:** `mochiko:verify-output`
**Grades:** `plugins/mochiko/agents/qa-engineer.md` (the artifact) against `transform-qa-engineer.md` (producer claim — audited, not trusted) + `reconcile.md` Seam 4 (spec)
**Method:** Default-FAIL. Every PASS below is confirmed by evidence Read/grepped from the real file — producer report treated as a claim to disprove.

```
VERIFY: qa-engineer (P3, port-with-edits × standalone)
Evidence read: plugins/mochiko/agents/qa-engineer.md (all 94 ln)
               .mochiko/transform/implement/transform-qa-engineer.md (producer claim)
               .mochiko/transform/implement/reconcile.md (Seam 4 spec)
               human-in-loop/plugins/humaninloop/agents/qa-engineer.md (original, 93 ln — trace baseline)
               plugins/mochiko/agents/*.md (skills-frontmatter convention cross-check)

VERDICT: PASS
Failing items: none
Required fixes: none
Deferred (explicit task scope, NOT a defect): router registration + plugin.json manifest → Wave-2
```

---

## Deterministic pre-asserts (I ran these myself; results ARE the evidence)

| Grep over the artifact | Result | Meaning |
|------------------------|--------|---------|
| `grep -in "cycle"` | **0 hits** (exit 1) | All 3 edited cycle tokens excised. |
| `grep -in "dispatch"` | **0 hits** (exit 1) | No dispatch coupling. |
| `grep -in "staff"` | **0 hits** (exit 1) | No sibling-agent name (`staff-engineer`). |
| `grep -in "workflow"` | **0 hits** (exit 1) | No workflow-phase/mode coupling. |
| `grep -in "workflow-agnostic"` | **0 hits** (exit 1) | No independence-by-declaration meta-label. |
| `grep -inw "CLI"` | **0 hits** (exit 1) | No CLI classification-table token in the persona. |
| `grep -in "100%" / "auto-resolve" / "placement"` | **0 hits** | No gate-placement routing rule in the persona. |
| `grep GUI / subjective` (classification terms) | **1 line only (L80)** | Present exactly once, as judgment ("always get a checkpoint") — not a table. (The L73 "gui" hit is a `-i` substring of "ambi**gui**ty", not a classification reference.) |
| `grep -inE "executing-tdd\|brownfield-integration"` | **0 hits** (exit 1) | Producer skills NOT mounted here. |
| `grep -in "skills:"` frontmatter | **L35 `skills: testing-end-user`** (bare, only) | Grading skill only. |
| `diff` HIL-source body L48–94 vs artifact | **exit 0 (byte-identical)** | Entire Tier-1 spine kept verbatim; zero dilution, zero drop. |
| `diff` full file | **6 changed lines only:** L5, L26, L27 (3 cycle rebinds) + L42, L44, L46 (skills-available rebind/single-source) | No other edit exists — every change is a rebind, none a drop. |

Skills-frontmatter cross-check: all 9 ported mochiko agents use the **bare** form (`devils-advocate`, `principal-architect`, `validator`, `staff-engineer: executing-tdd-cycle, brownfield-integration`, …). `skills: testing-end-user` conforms; the `mochiko:` prefix lives on the body reference (L44), matching the pair.

---

## Part A — Conformance (mapped to the 5 graded checks + supporting conventions)

### Check 1 — Decoupling (Convention 4 deny-list scan) — **PASS**
The 3 edited tokens are gone and the rebinds landed exactly (full diff vs HIL source):
- L5 `description`: HIL "gates **cycle** completion on human approval" → artifact **"gates completion on human approval"** ✓
- L26 ex-3 context: HIL "run as part of **cycle** verification" → artifact **"as part of verification"** ✓
- L27 ex-3 user: HIL "before we **close this cycle**" → artifact **"before we approve this change"** ✓

Deny-list grep clean: `cycle`=0, `staff-engineer`/`staff`=0, `dispatch`=0, `workflow`=0, `workflow-agnostic`=0, no `.mochiko/` path / mode / phase injection (the only `mode` hit was the substring in `model:`).

Legit survivors correctly NOT flagged:
- **`pass` (8 hits)** — all universal test-pass/fail language ("what passed, what failed", "Deterministic pass/fail", "a test that 'should' pass but didn't"), none is workflow-iteration "pass". Keystone: true of any QA professional → craft, keep.
- **Self-reference** — `qa-engineer` appears only in the standard Claude-Code agent-selection format ("I'll use the qa-engineer to…", "owned by the qa-engineer"); matches ported `devils-advocate`/`task-architect`. Not a sibling-agent-name coupling.
- `T{cycle}.{task}` ID grammar — not present in this artifact (nothing to flag).

### Check 2 — Tier-1 validator spine + determinism boundary (the load-bearing keep) — **PASS**
Body diff (L48–94) is **byte-identical** to HIL source (exit 0). Every spine element present and verbatim:
- Evidence-first — L65 "No assertion is 'passed' without captured proof."
- Reproducibility — L66 "Every verification can be re-run. Capture the exact commands, environment state, and timing."
- Honesty — L67 "A test that 'should' pass but didn't is a failure, full stop."
- Completeness — L68 "No partial results presented as conclusions."
- Distrust inferred outcomes — L73 "You distrust inferred outcomes… you don't claim to know the result"; L82 Reject "Inferred outcomes—…you don't claim it passed."
- Conservative default-to-human-oversight — L69 "When uncertain… default to human oversight. The safe path is always a checkpoint."
- Escalate ambiguous evidence — L92 "Escalating ambiguous evidence to human judgment rather than making assumptions."

**Determinism boundary NOT diluted (critical):**
- L59 (verbatim): "**Quality Gate Results** — Deterministic pass/fail for lint, build, and test suites" — exit-code ground truth as a named output, intact.
- L30 (verbatim): "Quality gate execution is deterministic verification work owned by the qa-engineer."
- L44 (rebind): "running the quality gates and classifying results **by exit code**" — the CLI-assert/exit-code discipline explicitly referenced; the rebind *adds* "by exit code" (strengthens, not softens). Not reduced to pure LLM judgment. **PASS.**

### Check 3 — Confidence-gate seam (Seam 4) correct — **PASS**
Persona keeps **only** the conservative judgment; classification table and gate placement are both absent.
- KEEPS judgment: L80 "GUI interactions and subjective assessments always get a checkpoint"; L73 "Ambiguity is never a reason to auto-approve—it's a reason to escalate"; L69 conservative default-to-human. This is exactly the middle piece Seam 4 assigns to P3.
- NO classification **table**: zero `CLI` token; GUI/subjective appear once (L80) as a judgment sentence, not the CLI/GUI/SUBJECTIVE → auto-approve/checkpoint matrix; no "100%-pass", no "default SUBJECTIVE". That table is P5's (`testing-end-user`), referenced at L44 ("classify… procedure… consult it there rather than restating any of it here"). ✓
- NO gate **placement**: zero "placement"/"auto-approve routing"/"final-acceptance gate" prose. The only `auto-approve` hit (L73) is the judgment "never a reason to auto-approve", not the routing rule. Placement is the lead's. ✓

### Check 4 — Independence (Convention 5) — **PASS**
- Frontmatter L35: `skills: testing-end-user` — the grading skill **only** (bare form = confirmed ported-agent convention).
- Producer skills `executing-tdd-cycle` / `brownfield-integration`: **grep = 0 hits** in this artifact; cross-check confirms they are mounted on `staff-engineer.md` **only** (`skills: executing-tdd-cycle, brownfield-integration`). Sets are disjoint (`{testing-end-user}` ∩ `{executing-tdd-cycle, brownfield-integration}` = ∅), different agents → structural independence. No produce+grade on one agent. ✓

### Convention 7 — Kernel-free — **PASS**
Pure-prose persona; no Python/MCP/DAG/catalog/`hil-dag`/orchestration plumbing (grep-clean; nothing but persona sections).

### Convention 8 — Altitude / single-source — **PASS**
The persona **references** the procedure rather than restating it: L44 "This is the single source of truth for the parse/execute/classify/report procedure and its formats; consult it there rather than restating any of it here." No loop-discipline / validator-tiers / gap-routing inlined.

### Conventions 2, 3, 6 — scope notes (not failed)
- **2 (Discoverability/router)** — **DEFERRED to Wave-2 per explicit task scope** (producer §Wave-2 item 1; task states router deferred). Not done; not a defect in this artifact's body transform. Flagged as pending wiring, not a FAIL.
- **3 (Model-invoke triggers)** — N/A: an agent uses `<example>` selection blocks, not model-invoke `description` triggers.
- **6 (Sound-loop placement / filled contract)** — the loop + `workflow-contract` are the **lead's (P1)** to fill, not this persona's; qa sits correctly as the independent validator in the cast pair (Seam 2). Out of single-artifact scope; P1 transform + verify adjudicates.

---

## Part B — Trace audit (against the HIL original, diff-backed)

| Audit | Verdict | Evidence |
|-------|---------|----------|
| **Completeness** | **PASS** | Full diff shows only 6 changed lines; every other HIL responsibility preserved verbatim and carries a tag in the producer trace (spine→kept; procedure→folded-into-skill P5; mount→kept-but-rebind; model/color→kept; 3 cycle tokens→kept-but-rebind; orchestration→moved-to-lead). No responsibility is untagged. |
| **Justified drops** | **PASS** | Producer + reconcile gated-bundle item 9 assert "P3 — 0 drops." Diff **confirms zero deletions** — every HIL line is preserved or rebound. The 3 "cycle" tokens are rebinds (gate/verify capability survives via "gates completion"/"verification"/"approve this change"), not drops. Nothing to justify. |
| **Landing integrity** | **PASS** | `folded-into-skill` → `mochiko:testing-end-user` (P5), a real ported skill in this same cluster run; body reference at L44 points to it. `moved-to-lead` → P1 lead (reconcile §Job 2 confirms the lead absorbs execute→verify pairing, FAIL-loop, gate placement); the persona correctly carries **none** of it (grep: workflow/dispatch/staff = 0). Both destinations are named and receive the responsibility per the reconcile spec. |
| **No capability loss** | **PASS** | Union of tags covers HIL behavior: validator craft (spine, verbatim) + procedure (referenced, invocable via Skill tool) + orchestration (correctly relocated to lead, not qa's to own). Everything a user got before — evidence-first verification, deterministic quality-gate/exit-code classification, conservative escalation, checkpoint presentation — remains reachable. No X-then / not-X-now gap. |

---

## Independence attestation

This grade was produced by an agent that did **not** author `qa-engineer.md` or its transform doc. Every PASS is anchored to a grep result or a Read/quoted line from the real artifact and its HIL baseline — not to the producer's report, which was audited against the file and found accurate (its post-edit deny-list scan and "0 drops" claim both independently reproduced).

**Verify version:** v1 · **Governed by:** `verify-output` · **Result:** PASS — artifact + trace accepted; only Wave-2 caller-side wiring (router + manifest) remains, deferred by design.
