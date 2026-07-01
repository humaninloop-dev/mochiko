# Phase 3 Transform — P5 `testing-end-user` (implement cluster)

**Transformed:** 2026-07-01 · **Producer:** `mochiko:transform-producer` · **Skill:** `transform-recipes` (recipe: `port-with-edits` + `standalone` + grammar dedupe + wiring pass)
**Consumes:** `assess-testing-end-user.md` (trace + 2 load-bearing findings) + `reconcile.md` (Seam 5a, Seam 4, §Job 1.3, §Job 2 gate placements, §Job 3 P5 row, §Seam 6)
**Source:** `human-in-loop/plugins/humaninloop/skills/testing-end-user/` — SKILL.md (280 ln) + 4 references (`TASK-PARSING.md`, `EVIDENCE-CAPTURE.md`, `REPORT-TEMPLATES.md`, `TESTING-EVIDENCE.md`)
**Artifacts:** `plugins/mochiko/skills/testing-end-user/` — SKILL.md + `references/{TASK-PARSING,EVIDENCE-CAPTURE,REPORT-TEMPLATES,TESTING-EVIDENCE}.md`
**Grammar owner (NOT edited this run):** `plugins/mochiko/skills/patterns-vertical-tdd/references/CYCLE-STRUCTURE.md`
**Applies a decision; does not make one. Does NOT grade its own output** — that is Phase-4 `verify-output`, run by a different agent.

---

```
TRANSFORM: testing-end-user
Applied:   port-with-edits × standalone + grammar-dedupe (Seam 5a) + wiring-pass
Artifacts: plugins/mochiko/skills/testing-end-user/SKILL.md (created)
           plugins/mochiko/skills/testing-end-user/references/TASK-PARSING.md (created)
           plugins/mochiko/skills/testing-end-user/references/EVIDENCE-CAPTURE.md (created)
           plugins/mochiko/skills/testing-end-user/references/REPORT-TEMPLATES.md (created)
           plugins/mochiko/skills/testing-end-user/references/TESTING-EVIDENCE.md (created)
New partners: NONE — standalone. The producer↔validator pair already exists (staff-engineer ↔
              qa-engineer); this is qa's core validator-side skill, mounted on qa-engineer ONLY.
Wiring:    classification=model-invoked (no disable-model-invocation)
           router=DEFERRED to Wave-2 (per task scope; noted below)
           triggers=re-keyed "when the user says 'TEST:'…" → work-context RFC-2119 (MUST/SHOULD kept)
           rebinds=[ evidence filenames /tmp/claude/qa-engineer-{task}-* → /tmp/claude/verify-{task}-*
                     (role-neutral); "dispatched…implementation cycle" (L154) → role-neutral;
                     "cycle-checkpoint gate" (L200) → "the gate that consumes it"; TASK-PARSING legacy
                     "(ignored - qa-engineer handles)" → "(handled at the checkpoint, not the parser)" ]
           decouple=deny-list scan clean: qa-engineer=0, dispatch=0, cycle-checkpoint=0,
                    implementation-cycle=0, staff-engineer=0, workflow-agnostic=0
           single-source=`**TEST:**` grammar REFERENCED from patterns-vertical-tdd (CYCLE-STRUCTURE.md);
                         execution/parse semantics RETAINED; no loop-discipline inlined
Trace (realized): every assessment responsibility carries a final tag; zero silent drops.
```

---

## Body treatment applied — `port-with-edits` + the Seam-5a grammar dedupe (the load-bearing edit)

Structure, headings, and voice preserved from HIL. Only the earned edits applied: the grammar dedupe, the decouples, and the description re-key. The runtime procedure needed no rebuild (not `redesign`); it owed decoupling + dedupe (not `keep-verbatim`).

### The `**TEST:**` grammar dedupe (Seam 5a) — REFERENCE the legal set, RETAIN the semantics

`patterns-vertical-tdd`'s `CYCLE-STRUCTURE.md` canonically owns the grammar. P5 now **references** the legal set and **retains** only execution/parse semantics. The split was drawn per catalog:

| Grammar facet (HIL loc) | Realized as REFERENCE (points at) | Semantics RETAINED here |
|-------------------------|-----------------------------------|-------------------------|
| Supported-marker list (SKILL L50–55) | CYCLE-STRUCTURE § *Unified TEST: Format* + § *Legacy Format Support* — the enumerated list is **removed**; SKILL Task Detection now cedes the marker set and shows only the detection cue | *how to detect* a TEST task by its marker line + walk to its boundary (SKILL Task Detection → TASK-PARSING algorithm) |
| Action-modifier catalog (SKILL L81–86, table) | CYCLE-STRUCTURE § *Action Modifiers* — the 3-col catalog table is **replaced** by a vocabulary pointer | *how to execute* each modifier: `(background)` start-async **+ track PID**; `(timeout Ns)` enforce override vs 60s default, mark TIMEOUT on expiry; `(in path)` run in dir (SKILL Execute Actions) |
| Assert-pattern catalog (SKILL L93–99, table) | CYCLE-STRUCTURE § *Assert Patterns* — the catalog table is **replaced** by a vocabulary pointer | *how to evaluate* each assert: `Console contains` = substring vs captured stdout/stderr (`(within Ns)` = timed poll of streaming bg output); `File exists` = `test -f`; `Response status` = compare captured HTTP status; else custom→human. **Explicit pass/fail per assert; no default to PASS** (SKILL Evaluate Asserts) |
| `TASK-PARSING.md` field/modifier/assert tables | Banner + per-section pointers to CYCLE-STRUCTURE (§ *Field Definitions* / *Action Modifiers* / *Assert Patterns* / *Legacy Format Support*) cede the **vocabulary**; the tables are re-framed as **extraction mechanics** | the parse algorithm itself — boundaries, ID extraction, field-value extraction regexes, multiple-same-field handling, parsed-task structure, error handling, legacy field-mapping — RETAINED (this is the parse *how-to*, not the catalog) |

**Boundary integrity:** the `**TEST:**` grammar stays *owned* by patterns-vertical-tdd, *consumed* by P5 (contract §Boundary-integrity). `patterns-vertical-tdd` was **not** edited (task constraint). The reciprocal minor note (CYCLE-STRUCTURE L250–254 reproduces the CLI/GUI/SUBJECTIVE table "for context") is **non-blocking** and deferred — direction already correct (it cedes ownership to P5); slim it only when patterns-vertical-tdd is next touched.

### RECLAIM kept intact — the CLI/GUI/SUBJECTIVE Task-Classification table (Seam 4, procedure half)

The runtime Task-Classification table (SKILL L111–122) is P5's **own** — NOT deduped to patterns-vertical-tdd. Realized verbatim in substance at SKILL § *Task Classification*: CLI (backtick cmds + measurable asserts) ⇒ may auto-approve if 100% pass; GUI/SUBJECTIVE ⇒ always human checkpoint; **default to SUBJECTIVE if uncertain** (safe fallback). Added the conservative-disposition seam sentence ("Ambiguity is a reason to escalate to a human, never a reason to auto-approve. Any failure, on any classification, forces a checkpoint.") — the procedure half of the confidence-gate. Per Seam 4: the *procedure* lives HERE (P5); the *conservative judgment/disposition* lives in the qa-engineer persona (P3); the *placement + auto-approve/checkpoint + final-acceptance wiring* is the lead's. No gate-placement prose leaked into this skill.

### KEEP un-diluted — quality-gate execution + deterministic auto-resolution (Tier-1 ground truth)

SKILL § *Quality Gate Execution* + § *Quality Gate Auto-Resolution* retained un-softened: exit `0` = pass, non-zero = fail, **No human checkpoint for quality gate results — they are deterministic.** Added an explicit anti-dilution guard: "(This exit-code determinism is ground truth; it MUST NOT be softened into an LLM judgment call.)" The `quality_gates` YAML frontmatter shape kept. This is the qa Tier-1 determinism boundary — not diluted into LLM judgment.

## Structural move applied — `standalone`

One self-contained validator-side skill placed at `plugins/mochiko/skills/testing-end-user/` (SKILL + 4 refs). No split/merge/promote — P5 is the validator half by construction and does not self-grade; the grammar dedupe is a **reference/wiring** resolution (Seam 5a), NOT a structural split. Mounts on `qa-engineer` ONLY (independence; §Job 1.6 / Seam 2 hard rule) — never co-mounted with producer-side P4 on staff.

---

## Decouples applied (deny-list scan clean — see grep evidence in the handoff)

| HIL loc | Before | After (role-neutral) |
|---------|--------|----------------------|
| SKILL L154 | "When **dispatched as part of an implementation cycle** verification, execute quality gates…" | "When **a verification run includes quality gates**, execute them…" |
| SKILL L200 | "surfaced through the verification report to the **cycle-checkpoint gate**, which evaluates them deterministically" | "surfaced through the verification report to **the gate that consumes it**, which evaluates them deterministically" |
| SKILL L25 | "Quality gate execution … as part of **cycle** verification" | "… as part of **verification**" |
| SKILL L109 | "Human decision gates **cycle** completion" | "The human decision gates **completion**" |
| EVIDENCE-CAPTURE.md (20×) + REPORT-TEMPLATES.md (1×) | `/tmp/claude/qa-engineer-{task}-*.log` | `/tmp/claude/verify-{task}-*` (role-neutral prefix naming the *work*, not the agent) |
| TASK-PARSING.md legacy map | "`**Human confirms**:` (ignored - **qa-engineer** handles)" | "(ignored by the parser — human confirmation is handled at the checkpoint, not during parsing)" |

Keystone survivors kept (true of this verification professional on any job): parse / execute / classify / evidence / report / checkpoint / anti-rationalization. The dispatch/pairing and the loop-on-reject (targeted retry) are the **lead's**, not this skill's — moved-to-lead, absent from the body.

## Convention-wiring pass (all six)

1. **Classification** — `model-invoked` (no `disable-model-invocation`). Agent-consumed by the qa verifier. ✓
2. **Router registration** — **DEFERRED to Wave-2** (per task scope; entry drafted below). ⏸
3. **Trigger phrasing** — re-keyed from "when the user says 'TEST:'…" to **work-context RFC-2119** describing the verification *work* (execute a `**TEST:**` task against real infra; parse Setup/Action/Assert; run modifiers with evidence; classify CLI/GUI/SUBJECTIVE; run quality gates; present a checkpoint). MUST (primary) + SHOULD (secondary) grading kept. ✓
4. **Path rebinding** — no `.humaninloop/`, kernel, DAG, catalog, MCP, or `hil-dag` paths existed to rebind. Evidence scratch filenames rebound to role-neutral `verify-` prefix. Upstream handoff (`tasks.md` `## Quality Gates` + `plan.md` build config) kept as relative artifact names (the lead briefs the actual `.mochiko/specs/<feature>/` path — no in-body absolute rebind needed). ✓
5. **Decouple persona/skill** — the 6 rebinds above; whole-artifact deny-list scan clean (all 0). ✓
6. **Single-source / de-duplicate** — the `**TEST:**` grammar referenced from `patterns-vertical-tdd`/`CYCLE-STRUCTURE.md`, not restated (Seam 5a); no `loop-discipline` doctrine inlined. ✓

---

## Responsibility trace (realized — every assessment responsibility carries a final tag; no silent loss)

| # | Responsibility (assessment source) | Realized tag |
|---|-----------------------------------|--------------|
| 1 | TEST-task detection/parsing (SKILL Task Detection; TASK-PARSING algorithm) | **kept** (execution-side) — the grammar catalog it parses → **dedupe** (references CYCLE-STRUCTURE) |
| 2 | Setup execution — sequential, fail-fast, record output | **kept** |
| 3 | Action execution — modifiers, console capture, PID tracking, timeout enforcement | **kept** (execution semantics) — modifier catalog → **dedupe** (references § Action Modifiers) |
| 4 | Assert evaluation — Console contains / File exists / Response status vs evidence | **kept** (evaluation semantics) — assert-pattern catalog → **dedupe** (references § Assert Patterns) |
| 5 | Evidence capture — console/screenshot/logs/timing, storage, bg cleanup, timeout partial-output | **kept** (EVIDENCE-CAPTURE ported); evidence-path filenames embedding `qa-engineer` → **kept-but-rebind** (`verify-` role-neutral) |
| 6 | **CLI/GUI/SUBJECTIVE runtime classification — THE RECLAIM** (auto-approve vs checkpoint; default SUBJECTIVE) | **kept** (canonical runtime home; procedure half of Seam-4 confidence-gate; un-deduped, P5's own) |
| 7 | Result classification — PASS/FAIL/PARTIAL/TIMEOUT/ERROR | **kept** |
| 8 | **Quality-gate execution + deterministic auto-resolution** — exit 0=pass / non-zero=fail / no human checkpoint (Tier-1) | **kept — un-diluted** (+ anti-dilution guard added); "cycle-checkpoint gate" phrasing → **kept-but-rebind**; sequencing "as part of the cycle" → the **lead** references it |
| 9 | Report generation — minimal/rich/PARTIAL/TIMEOUT/ERROR, truncation, `quality_gates` YAML | **kept** (REPORT-TEMPLATES ported; Seam-6 keep-in-references honored) |
| 10 | Checkpoint presentation — AskUserQuestion approve/reject/retry; human gates completion | **kept** (runtime human-gate presentation) — the decision to **loop on reject** (targeted retry) → **moved-to-lead** (constraint (c)) |
| 11 | Anti-rationalization — red-flags, common-rationalizations, common-mistakes, "no exceptions" | **kept** (intrinsic craft; keystone-passes) |
| 12 | Legacy format normalization — HUMAN VERIFICATION → unified mapping | **kept** (normalization mechanics) — legacy-marker list overlaps grammar → **dedupe** (references § Legacy Format Support); the `qa-engineer` name in the map → **kept-but-rebind** |
| 13 | Skill test provenance — RED/GREEN/REFACTOR hardening record | **kept** (TESTING-EVIDENCE ported verbatim; own provenance doc) |
| 14 | Orchestration framing — "dispatched as part of an implementation cycle" (L154) | **kept-but-rebind** (phrasing → role-neutral); the actual dispatch / execute-then-verify pairing → **moved-to-lead** (constraint (b)) |
| W | Description / triggers — re-key from "when the user says 'TEST:'…" | **kept-but-rebind** (→ work-context RFC-2119; MUST/SHOULD grading kept; model-invoked) |
| W | Classification tag | **kept** (model-invoked) |
| W | Router registration | **deferred** (Wave-2) |

**Dropped: NONE.** Consistent with reconcile gated-bundle item 9 ("P3, P5, P6 — 0 drops"). Every item is kept / kept-but-rebind / dedupe-reference / (orchestration) moved-to-lead. No silent capability loss.

---

## Wave-2 wiring needed (deferred per task scope — NOT done here)

1. **Router registration** — add to the `mochiko` router under a new "Implement / execute cluster" section (alongside `executing-tdd-cycle`, `qa-engineer`, `staff-engineer`, `brownfield-integration`). Drafted entry: *"`testing-end-user` | executing a `**TEST:**` verification task against real infrastructure — parse Setup/Action/Assert, run actions with captured evidence, classify CLI/GUI/SUBJECTIVE (auto-approve vs human checkpoint), run quality gates as deterministic exit-code checks, present a verification checkpoint; the `qa-engineer` validator's core runtime skill (consumes the `**TEST:**` grammar owned by `patterns-vertical-tdd`)."* A primitive not in the router fails discoverability.
2. **`plugin.json` manifest** — ensure `skills/testing-end-user/` is picked up by the `mochiko` plugin manifest (confirm skills are auto-discovered vs. explicitly listed).
3. **Lead-side casting (P1 `implement` command)** — the execute→verify pairing (constraint (b)), the targeted-retry loop-on-reject (constraint (c)), the confidence-gate **placement** + auto-approve/checkpoint routing + named final-acceptance gate (Seam 4, intake decision 2) land on the thin lead. Not this artifact's concern; noted so the cross-primitive wiring is not lost. This skill *supplies* the verdict + checkpoint; the lead *consumes and loops*.
4. **Mount confirmation (P3 `qa-engineer`)** — already ported (`transform-qa-engineer.md`), body reference `mochiko:testing-end-user` resolves to THIS artifact. Independence holds: mounted on qa **only**, never on staff.
5. **Reciprocal grammar note (patterns-vertical-tdd)** — optionally slim CYCLE-STRUCTURE.md L250–254's CLI/GUI/SUBJECTIVE "for context" table to a pointer back to P5 when that skill is next touched. Non-blocking; direction already correct.

---

## Handoff to `verify-output` (Phase 4, independent agent)

An independent reader can check every claim from the artifact + this trace alone:
- **Grammar dedupe (Seam 5a):** the three catalog tables (marker list / modifier table / assert table) are **gone** from SKILL.md, replaced by pointers to CYCLE-STRUCTURE.md §-named sections; the execution/eval *semantics* remain (PID tracking, timeout→TIMEOUT, substring/timed/File/Status evaluation, explicit-pass-per-assert). TASK-PARSING.md carries a grammar-vs-parsing banner and cedes vocabulary while keeping the parse regexes. `patterns-vertical-tdd` untouched. `grep cycle-report` in patterns-vertical-tdd stays 0 (not this skill's concern; boundary owned there).
- **Reclaim intact:** CLI/GUI/SUBJECTIVE table + "Default to SUBJECTIVE" present (SKILL § Task Classification); it is P5's own, not deduped.
- **Exit-code determinism intact:** exit 0=pass / non-zero=fail / no-human-checkpoint verbatim in substance (SKILL § Quality Gate Auto-Resolution) + anti-dilution guard.
- **Decouples applied:** whole-dir deny-list scan clean — `qa-engineer`=0, `dispatch`=0, `cycle-checkpoint`=0, `implementation cycle`=0, `staff-engineer`=0, `workflow-agnostic`=0. Remaining "cycle" tokens are the task-ID component (`T{cycle}.{task}` grammar) + the skill's own TDD hardening cycle + the `CYCLE-STRUCTURE.md` filename — none is a workflow-phase injection.
- **Classification:** model-invoked (no `disable-model-invocation`); work-context RFC-2119 description, MUST/SHOULD graded.
- **Kernel-free:** no `.humaninloop/`, kernel, DAG, MCP, `hil-dag`, or capability-catalog references.
- **Trace complete:** every assessment responsibility carries a realized tag; zero silent drops.

**Transform version:** v1 · **Governed by:** `transform-recipes` · **Next:** Phase-4 `verify-output` grades this artifact + trace independently (different agent).
