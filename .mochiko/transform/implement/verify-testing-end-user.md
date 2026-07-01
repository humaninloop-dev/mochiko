# VERIFY — P5 `testing-end-user` (implement cluster)

**Verified:** 2026-07-01 · **Validator:** `mochiko:validator` (independent — did NOT author this artifact) · **Skill:** `verify-output`
**Recipe under grade:** `port-with-edits × standalone` + grammar dedupe (Seam 5a) + convention-wiring pass
**Producer's claim audited (not trusted):** `.mochiko/transform/implement/transform-testing-end-user.md`

```
VERIFY: testing-end-user (SKILL.md + 4 references)
Evidence read:
  plugins/mochiko/skills/testing-end-user/SKILL.md
  plugins/mochiko/skills/testing-end-user/references/TASK-PARSING.md
  plugins/mochiko/skills/testing-end-user/references/EVIDENCE-CAPTURE.md
  plugins/mochiko/skills/testing-end-user/references/REPORT-TEMPLATES.md
  plugins/mochiko/skills/testing-end-user/references/TESTING-EVIDENCE.md
  plugins/mochiko/skills/patterns-vertical-tdd/references/CYCLE-STRUCTURE.md   (grammar owner)
  plugins/mochiko/skills/mochiko/SKILL.md                                       (router — ground truth)
  human-in-loop/plugins/humaninloop/skills/testing-end-user/SKILL.md            (original — trace)
  .mochiko/transform/implement/transform-testing-end-user.md                    (producer claim — audited)

VERDICT: PASS
```

All greps below were run by the validator against the real files.

---

## Grade (the parent's 6 points), binary, with quoted evidence

### 1. Grammar dedupe (Seam 5a — the load-bearing edit) — **PASS**

The three authoritative catalogs are REMOVED from `SKILL.md` and replaced by references to the grammar owner; execution/parse semantics are RETAINED.

Catalogs removed (grep = absent in `SKILL.md`):
- `Supported markers` heading — **absent** (HIL had the enumerated 4-marker definitional list at L50–55).
- `| Modifier | Example | Behavior |` catalog table header — **absent**; `Run async, track PID` / `Override 60s default` catalog cells — **absent** (HIL had the 3-row table at L81–86).
- `| Pattern | Verification |` assert-catalog table header — **absent** (HIL had the table at L93–99).

References to the owner present (5 hits of `CYCLE-STRUCTURE` in `SKILL.md`):
- L16 "the `**TEST:**` construct … is authored and owned by `patterns-vertical-tdd` in `CYCLE-STRUCTURE.md`. This skill **consumes** that grammar; it does not redefine it."
- L47 field-skeleton + marker set "are defined by the grammar owner … do not re-enumerate it here."
- L63 modifier "vocabulary … is defined in `CYCLE-STRUCTURE.md` (§ *Action Modifiers*)."
- L73 assert "vocabulary … is defined in `CYCLE-STRUCTURE.md` (§ *Assert Patterns*)."
- L263 explicit "Grammar owner:" pointer.

Execution/parse semantics RETAINED (grep-confirmed in `SKILL.md`):
- `(background)` → **"track its PID"** (L65).
- `(timeout Ns)` → "kill the process, and **mark the result `TIMEOUT`**" (L66).
- `Console contains` → **"substring match against the captured stdout/stderr"**; `(within Ns)` → "timed match … poll until the text appears" (L75).
- `File exists` → **`test -f {path}`** (L76).
- `Response status` → "compare the captured HTTP status" (L77).
- **"Each assert MUST receive an explicit pass/fail evaluation. No default to PASS"** (L80).
- `TASK-PARSING.md` retains the full parse how-to (boundaries L75, ID extraction L82, field-value regexes, multiple-same-field handling, parsed-task JSON, error handling, legacy field-mapping) under an explicit grammar-vs-parsing banner (L3) that cedes vocabulary to the owner. Detection regexes = parse mechanics, not grammar re-authoring.

Dedupe direction confirmed against the owner: `CYCLE-STRUCTURE.md` canonically holds § *Unified TEST: Format* (L236), § *Field Definitions* (L258), § *Action Modifiers* table (L269–273), § *Assert Patterns* table (L277–282), § *Legacy Format Support* (L350). Consumer references, owner defines. Correct.

*Observation (non-defect):* `SKILL.md` L47 names the legacy variants inline ("`TEST:VERIFY`, `TEST:CONTRACT`, `HUMAN VERIFICATION`") within the sentence that cedes their definition to the owner. This is a courtesy pointer, not a re-authored definitional catalog (no per-marker status/role assignment, no standalone authoritative list). The load-bearing requirement — remove the authoritative enumerated catalog, replace with a reference — is met.

### 2. Reclaim intact (the parked responsibility) — **PASS**

The CLI / GUI / SUBJECTIVE runtime Task-Classification table is PRESENT and is P5's own (NOT deduped to the owner):
- `SKILL.md` L97–101 table: CLI = "May auto-approve if 100% pass"; GUI = "Always human checkpoint"; SUBJECTIVE = "Always human checkpoint".
- L103 **"Default to SUBJECTIVE if uncertain — the safe fallback. Ambiguity is a reason to escalate to a human, never a reason to auto-approve. Any failure, on any classification, forces a checkpoint."** — the procedure half of the confidence-gate, canonical runtime home. (The owner's L250–254 "for context" copy correctly cedes ownership to the downstream step; boundary direction is right.)

### 3. Determinism un-diluted (Tier-1 ground truth) — **PASS**

Quality-gate deterministic auto-resolution present and **not** softened into LLM judgment:
- L145 "**Classify**: exit `0` = pass, non-zero = fail."
- L176 "Quality gates **always auto-resolve**. No human checkpoint … the answer is an exit code, not a judgment."
- L178 "**Exit `0`** = pass." · L180 "**No human checkpoint** for quality gate results — they are deterministic."
- L182 anti-dilution guard added: "**(This exit-code determinism is ground truth; it MUST NOT be softened into an LLM judgment call.)**" — hardened beyond HIL.

### 4. Decoupling + kernel-free — **PASS**

Whole-dir deny-list scan (SKILL + 4 refs), validator-run — every token **0**:
`qa-engineer`=0 · `staff-engineer`=0 · `dispatch`=0 (incl. substring `dispatched`) · `cycle-checkpoint`=0 · `implementation cycle`=0 · `workflow-agnostic`=0 · sibling names (`principal-architect`/`transform-validator`/`devils-advocate`)=0.

Kernel-free scan — every token **0**: `humaninloop:`=0 · `hil-dag`=0 · `.humaninloop`=0 · `MCP`=0 · `catalog`=0 · `brain`=0 · `capability`=0 · standalone `DAG`=0.

Legit survivors confirmed (not false-flagged): every `cycle` token is either the `CYCLE-STRUCTURE.md` filename reference, the `T{cycle}.{task}` ID grammar (`TASK-PARSING.md` L87 "Cycle number and task number", L214 `"cycle": 2`), or the skill's own TDD hardening cycle (`TESTING-EVIDENCE.md` L12) — zero workflow-phase injection. Evidence filenames role-neutral `verify-` prefix (`EVIDENCE-CAPTURE.md` L3 banner + L29/36–39/70); `qa` substring in evidence paths = **none**. The four HIL orchestration couples the producer claimed to rebind are all gone from the body (L154 "dispatched as part of an implementation cycle", L200 "cycle-checkpoint gate", L25/L109 "cycle" scoping) — verified absent.

### 5. Conventions — **PASS**

- **Classification = model-invoked:** frontmatter carries no `disable-model-invocation` (grep absent). ✓
- **Description re-keyed to work-context RFC-2119:** L3 "This skill **MUST** be invoked when executing a `**TEST:**` verification task against real infrastructure — parsing … running … evaluating … classifying …. **SHOULD** also invoke when running quality gates …." MUST=1, SHOULD=1, HIL-style "when the user says" = **0**. Exact-phrase `**TEST:**` trigger retained. ✓
- **Verification-report format kept in `references/REPORT-TEMPLATES.md`:** PASS/NEEDS REVIEW/PARTIAL/TIMEOUT/ERROR templates (L24/50/132/168/200), Checkpoint Presentation with `AskUserQuestion` (L223/230/247/266), `quality_gates` YAML shape in `SKILL.md` L152–170. ✓

*Bonus ground-truth (producer under-claimed):* the producer marked router registration "DEFERRED to Wave-2," but the router `skills/mochiko/SKILL.md` L97 **already contains** the `testing-end-user` entry with when-to-reach-it guidance, and L120 shows `qa-engineer` mounts `testing-end-user` and "mounts no producer skill." So verify-output Part A item 2 (Discoverability) and item 5 (structural producer↔validator independence) are satisfied against ground truth, not merely deferred.

### 6. Trace audit (P5 = zero drops) — **PASS**

- **Completeness:** HIL→mochiko `SKILL.md` heading inventory is a 1:1 match (same 24 sections, same order). Every one of the producer's 14 numbered + 3 wiring responsibilities carries a realized tag (`kept` / `kept-but-rebind` / `dedupe-reference` / `moved-to-lead`). No responsibility in the original is untagged.
- **Justified drops:** "**Dropped: NONE**" — consistent with reconcile gated-bundle item 9 ("P3, P5, P6 — 0 drops"). No bare or unaccepted drop.
- **No capability loss:** result-status set (5/5 PASS/FAIL/PARTIAL/TIMEOUT/ERROR), evidence types (console/screenshot/logs/timing), `quality_gates` YAML, legacy `HUMAN VERIFICATION` mapping, PID-tracking + cleanup, RED/GREEN/REFACTOR provenance — all spot-checked present. The runtime capability (detect→parse→setup→execute→evaluate→capture→classify→report→checkpoint→quality-gates) is fully retained.
- **Landing (one noted forward-dependency, not a defect):** the two orchestration responsibilities #10 (loop-on-reject / targeted retry) and #14 (dispatch / execute-then-verify pairing) are `moved-to-lead`. They are correctly ABSENT from the skill body (grep: no `execute-then-verify` / `targeted retry` / `loop on reject` / `re-dispatch`), while the skill still SUPPLIES the checkpoint choice ("approve, reject, or **retry**", L91). Orchestration is inherently caller-side and was never the skill's to keep. Its destination — the P1 `implement` command — is not yet built (Wave-2 §3), so the landing cannot be spot-checked this run; the consuming `qa-engineer` mount and router entry already exist. This is a tracked, reconcile-accepted re-home, not a silent capability loss in the graded artifact.

---

## Conformance summary (verify-output Part A)

| # | Convention | Verdict | Evidence |
|---|-----------|---------|----------|
| 1 | Classification | PASS | model-invoked; no `disable-model-invocation` |
| 2 | Discoverability | PASS | router `skills/mochiko/SKILL.md` L97 has the entry w/ when-to-reach guidance |
| 3 | Reliable model-invocation | PASS | work-context RFC-2119 MUST/SHOULD triggers, `**TEST:**` exact phrase (L3) |
| 4 | Composition & decoupling | PASS | deny-list scan all-0; qa-engineer mounts it, staff does not (L120) |
| 5 | Producer↔validator pairing | PASS | independent qa-engineer validator + machine-decidable quality gates (exit codes) |
| 6 | Sound-loop placement | PASS (skill scope) | human checkpoint gate present (L89–91); full implement-loop contract is the lead's Wave-2 artifact |
| 7 | Kernel-free | PASS | kernel scan all-0 |
| 8 | Altitude / single-source | PASS | grammar referenced from owner, not restated; no `loop-discipline` doctrine inlined |

**Trace audit:** completeness PASS · justified-drops PASS (none) · landing PASS (one tracked forward-dependency) · no-capability-loss PASS.

## Failing items
none

## Required fixes
none (artifact accepted).

## Non-blocking notes (for the lead / Wave-2 — not defects in this artifact)
1. Build the P1 `implement` command that receives the two `moved-to-lead` orchestration responsibilities (loop-on-reject, execute→verify pairing) + the confidence-gate placement/auto-approve routing/named final-acceptance gate; re-run landing spot-check then.
2. The producer's transform record still labels router registration "DEFERRED"; ground truth shows it already landed (`skills/mochiko/SKILL.md` L97). Update the record for accuracy.
3. Optional: slim the owner's `CYCLE-STRUCTURE.md` L250–254 "for context" CLI/GUI/SUBJECTIVE copy to a pointer back to P5 when that skill is next touched (direction already correct; non-blocking).
