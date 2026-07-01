# VERIFY-OUTPUT — `skills/brownfield-integration/SKILL.md` (P6)

Run: `implement` cluster · Phase 4 (verify) · **independent validator** (did NOT author this artifact)
Skill applied: `mochiko:verify-output` (five conventions + sound-loop placement + kernel-free + trace audit)
Disposition graded: **`port-with-edits × standalone`** + Seam-5b marker dedupe + ×5 cycle-report rebind
Date: 2026-07-01

```
VERIFY: brownfield-integration (P6)
VERDICT: PASS
```

---

## Evidence read (tamper-proofing — no PASS without this)

| File | Purpose |
|------|---------|
| `plugins/mochiko/skills/brownfield-integration/SKILL.md` (132 ln) | THE artifact — read in full |
| `.mochiko/transform/implement/transform-brownfield-integration.md` | producer's claim — audited, not trusted |
| `plugins/mochiko/skills/patterns-vertical-tdd/SKILL.md` | canonical `[EXTEND]`/`[MODIFY]` marker owner — confirmed dedupe destination |
| `human-in-loop/plugins/humaninloop/skills/brownfield-integration/SKILL.md` (130 ln) | HIL original — line-by-line trace source |
| `plugins/mochiko/skills/executing-tdd-cycle/SKILL.md` | de-collision sibling + `cycle-report.md` owner |
| `plugins/mochiko/skills/analysis-codebase/SKILL.md` | disjointness check |
| `plugins/mochiko/skills/mochiko/SKILL.md` (router) | Discoverability registration |

All greps below were **run by me** against the real artifact — not copied from the producer's self-scan.

---

## Part A — Conformance (verify-output five conventions + placement)

| # | Convention | Verdict | Evidence read from the artifact |
|---|-----------|---------|--------------------------------|
| 1 | **Classification** | PASS | Frontmatter (L1–4) is `name` + `description` only; **no** `disable-model-invocation` → model-invoked. No `skills:` list (single-file skill). References only model-invoked siblings (`patterns-vertical-tdd`, `executing-tdd-cycle`) → no user-invoked-skill invocation. |
| 2 | **Discoverability** | PASS | Registered in router `mochiko/SKILL.md` **L98**, under `### Implement / execute cluster` (L93), with when-to-reach guidance: *"safely EXTENDing/MODIFYing existing code at implement-time — the read-before-write checklist, interface preservation, and conflict detection… the marker vocabulary itself is owned by `patterns-vertical-tdd`."* (Producer report said "DEFERRED to Wave-2" — stale; ground truth = **registered**.) |
| 3 | **Reliable model-invocation** | PASS | Description (L3) carries graded, work-context triggers: `MUST` = "when implementing a task that touches existing code — safely making an `[EXTEND]` or `[MODIFY]` change to a file already on disk"; `SHOULD` = "extending an existing file, modifying existing behavior, integrating against an established interface…". Re-anchored — `grep 'when the user says'` → **CLEAN (0)**. |
| 4 | **Agent↔skill composition & decoupling** | PASS | Mounted on `staff-engineer` (router L119: `skills: executing-tdd-cycle, brownfield-integration`); procedure in the skill, persona in the agent. Deny-list grep (agent-names / `dispatch` / `workflow-agnostic` / independence-meta) → **CLEAN (0)**. |
| 5 | **Producer↔validator pairing** | PASS | Emits **no reviewable artifact of its own** (shaped code + surfaced flags); pure producer-side craft, no grading function. Downstream verification is by `qa-engineer`/`testing-end-user` (a different agent) at cycle level. No self-grade. Degenerate-N/A, correctly. |
| 6 | **Sound-loop placement** | PASS | Good citizen inside the implement loop (`staff-engineer` → `qa-engineer` → lead gate, router L119–120). Owns **no** done-condition / validation / human-gate of its own — correct for a craft skill; it does not fake a loop it shouldn't own. |
| 7 | **Kernel-free** | PASS | `grep -niE 'humaninloop:|\.humaninloop|hil-dag|catalog|DAG|MCP|brain|\.workflow/|Task\(|AskUserQuestion'` → **CLEAN (0)**. |
| 8 | **Altitude / single-source** | PASS | **References** shared doctrine instead of restating: marker vocabulary ceded to `patterns-vertical-tdd` (L10, L33), cycle-report schema ceded to `executing-tdd-cycle` (L74). No inlined `workflow-contract`, no restated four-rules / validator-tiers / gap-routing / anti-rationalization-of-loops. |

**Any item not confirmed → FAIL. All 8 confirmed against the file.**

---

## Part B — The five graded asks (with grep counts I ran)

### Ask 1 — Marker dedupe (Seam 5b): definitions REMOVED, consumption RETAINED — **PASS**

Definitions removed (grep run by me on the artifact):
- `grep -niE 'Add new code alongside existing|Change existing behavior'` → **CLEAN (0)** — the authoritative glosses are gone.
- `grep -niE '\| *Meaning *\|'` → **CLEAN (0)** — the "Meaning" column is deleted from the table.

Referenced, not re-authored — `grep -niE 'patterns-vertical-tdd'` → **3 hits**:
- L3 (description): *"Consumes the `[EXTEND]`/`[MODIFY]` markers that patterns-vertical-tdd defines at design time"*
- L10 (Overview): *"the marker **vocabulary** is defined by `patterns-vertical-tdd`… This skill does not redefine the markers"*
- L33 (Core Process): *"`patterns-vertical-tdd` owns what the `[EXTEND]` and `[MODIFY]` markers *mean*."*

Dedupe destination confirmed real: `patterns-vertical-tdd/SKILL.md` L144–151 carries the **Markers** table defining `[EXTEND]` / `[MODIFY]` — the vocabulary genuinely lives at the cited owner (landing integrity holds).

Consumption + interface-impact semantics RETAINED (read in the artifact):
- Interface-impact table **L35–38** — columns `Marker | Scope of change | Interface impact`; `[EXTEND]` "MUST NOT change existing function signatures, exports, or type contracts"; `[MODIFY]` "MAY change function internals; MUST NOT change signatures unless the task explicitly says so."
- "read the full file first" — Read-Before-Write Checklist **L42–50** (all 5 steps intact).
- "follow existing patterns" — checklist steps 2–5 + Interface Preservation **L52–61**.
- "preserve interfaces" — Interface Preservation **L52–61** (no signature/export/name changes).
- "detect conflicts" — Conflict Detection **L63–70** (name / import / test-file / circular).
- "Never treat an `[EXTEND]` task as a `[MODIFY]`" — **L40**.

Verdict: the vocabulary is *referenced*, the implement-time consumption + interface-impact half is *retained*. Correct dedupe direction. **PASS.**

### Ask 2 — Zero drops + anti-rationalization spine survives and is NOT deduped to loop-discipline — **PASS**

Spine present (grep run by me): `grep -niE 'Common Mistakes|Common Rationalizations|Red Flags|No exceptions'` →
- **L82** `## Common Mistakes`, **L108** `## Common Rationalizations`, **L119** `## Red Flags — STOP and Reconsider`, **L127** `**No exceptions:**`.

Spine is CRAFT-specific (code-modification **scope** discipline), confirmed by content: "Not Reading the Full File" (L84), "Silently Rewriting When Asked to Extend" (L90), "Adding 'Better' Patterns" (L102), "I'll just fix this one small thing while I'm here" (L117) — all about scope of a code modification, **not** loop-gaming.

Correctly NOT deduped into `loop-discipline`: `grep -niE 'loop-discipline|loop.gaming|exhaustion'` → **CLEAN (0)**. The spine stays local because it is a different concern (modification-scope discipline) than loop-discipline's loop-gaming doctrine.

Every EXTEND/MODIFY-consumption / read-before-write / interface-preservation / conflict-detection / flag-don't-resolve responsibility survives (see trace audit below). **Zero silent drops.** **PASS.**

### Ask 3 — Decoupling + kernel-free + soft-vocab softened + cycle-report rebound — **PASS**

- Deny-list (`staff-engineer|qa-engineer|principal-architect|state-analyst|…|dispatch|workflow-agnostic`) → **CLEAN (0)**.
- Kernel/HIL tokens → **CLEAN (0)** (see item 7).
- Soft workflow-phase vocab `grep -niE 'previous cycles|during cycle execution|cycle execution'` → **CLEAN (0)** — softened to "prior work in the codebase" (L21) and "out of scope for an extend/modify task" (L27).
- `grep -ciE 'cycle report'` → **1** — the single **attributed** reference at **L74**: *"they belong in the cycle report the run produces (owned by `executing-tdd-cycle`), not in a quiet workaround"*. No schema restated. The other 4 HIL "cycle report" sinks are softened to role-neutral "your report / surface it" (L40, L92, L106, L124). The skill no longer reads as owning that artifact.

Sibling-**skill** references (`patterns-vertical-tdd`, `executing-tdd-cycle`) are **allowed** — the deny-list targets agent names / `dispatch` / `workflow-agnostic`, and single-sourcing legitimately requires naming the owner. **PASS.**

### Ask 4 — Conventions: model-invoked + re-anchored description + de-collided from executing-tdd-cycle — **PASS**

- Classification model-invoked (item 1). Description re-anchored to work-context RFC-2119 (item 3).
- **De-collision is mutual and clean.** Artifact L3: *"…NOT the execution of the cycle the task belongs to (that is executing-tdd-cycle, which co-fires on the same brownfield task and drives red/green/refactor)."* And `executing-tdd-cycle/SKILL.md` scopes itself as "the runtime EXECUTION of cycles" and at its L21/L63/L64 explicitly says *"invoke `brownfield-integration` alongside"* for `[EXTEND]`/`[MODIFY]` tasks. Both skills fire on the shared markers; each scopes to its own act (cycle execution vs. read-before-write craft of the single modification) → they co-fire without fighting. **PASS.**

### Ask 5 — Disjointness vs `analysis-codebase` (no merge/overlap defect) — **PASS**

Confirmed from both descriptions — different **act**, not a shared core:

| Axis | `analysis-codebase` | `brownfield-integration` |
|------|---------------------|--------------------------|
| Act | DETECT stack/architecture/conventions to inform governance | Safely EXTEND/MODIFY one existing file |
| Phase | setup (before a constitution exists) | implement (writing code against existing code) |
| Trigger | "analyze codebase / detect stack / brownfield **setup**" | `[EXTEND]`/`[MODIFY]` / implementing against existing code |
| Output | artifact `.mochiko/memory/codebase-analysis.md` | shaped code + surfaced flags (no artifact) |

Only shared surface is the word "brownfield" + "read existing code"; the acts are opposite. No shared core, no trigger collision, no merge. **Fully disjoint.**

---

## Part C — Trace audit (against the HIL original, line-by-line — read by me, not the producer's table)

| HIL responsibility (source line) | Realized in artifact | Tag | Landing verified |
|----------------------------------|----------------------|-----|------------------|
| Triggers / description (L3) | L3, re-anchored to work-context, markers preserved | kept-but-rebind | ✓ |
| Overview: touch-existing-code + EXTEND/MODIFY gloss + read-first (L8–12) | L8–14; gloss **deduped**, consumption reframed, "not wrong until proven" + "letter=spirit" kept | kept + dedupe | ✓ vocab → patterns-vertical-tdd |
| When to Use (L16–21) | L16–21; "previous cycles" → "prior work in the codebase" | kept-but-rebind | ✓ |
| When NOT to Use (L23–27) | L23–27; "during cycle execution" softened | kept-but-rebind | ✓ |
| EXTEND vs MODIFY table (L31–36) | L31–38; Meaning col removed, Scope + Interface-impact retained | kept + dedupe | ✓ |
| "Never MODIFY when EXTEND" + flag (L38) | L40 "surface it as a blocker" | kept-but-rebind | ✓ |
| Read-Before-Write Checklist ×5 (L40–48) | L42–50 verbatim | kept | ✓ |
| Interface Preservation (L50–59) | L52–61 verbatim | kept | ✓ |
| Conflict Detection (L61–68) | L63–70 verbatim | kept | ✓ |
| When to Flag (L70–78) | L72–80; sink rebound to `executing-tdd-cycle` | kept-but-rebind | ✓ cycle-report → executing-tdd-cycle |
| Common Mistakes (L80–104) | L82–106; "cycle report" → "your report" | kept-but-rebind | ✓ |
| Common Rationalizations (L106–115) | L108–117 verbatim | kept | ✓ |
| Red Flags (L117–123) | L119–125; "in the cycle report" softened | kept-but-rebind | ✓ |
| No exceptions (L125–130) | L127–132 verbatim | kept | ✓ |

- **Completeness** — PASS. Every HIL responsibility appears in the artifact with a tag; nothing silently vanished.
- **Justified drops** — PASS (vacuous). **Zero `dropped` rows** — all delta is `kept` / `kept-but-rebind` / one cross-cluster `dedupe`, consistent with `port-with-edits × standalone`.
- **Landing integrity** — PASS. Marker vocabulary genuinely lives at `patterns-vertical-tdd` (L146–151 Markers table); `cycle-report.md` genuinely owned by `executing-tdd-cycle` (its description + router L96). Both destinations really received the responsibility.
- **No capability loss** — PASS. Everything doable via the HIL skill is still doable; the only removed text (marker-definition gloss) is single-sourced to its canonical owner, still reachable in the library.

---

## Producer-report discrepancies I found (audited, not trusted — both in the artifact's favor)

These do **not** threaten the verdict (the artifact is *more* complete than its report claims), but a validator audits the report against ground truth:

1. **Line count.** Producer Output block (report L18) claims `HIL 130 ln → 96 ln`. Actual artifact = **132 lines** (`wc -l`). All the producer's *own* internal line references (`L82–132`, `L42–50`, …) are consistent with 132, so the artifact is intact — only the summary figure "96" is stale/wrong. Cosmetic report error.
2. **Router registration.** Producer (report L22, L156–159) claims Discoverability is "DEFERRED to Wave-2." Ground truth: `brownfield-integration` **is already registered** in the router (`mochiko/SKILL.md` L98) with correct when-to-reach guidance, and its whole implement cluster (L96–98) is indexed. The convention is satisfied now, not deferred.

Recommend the producer correct the "96 ln" figure and the "router DEFERRED" note for report accuracy. Neither is an artifact defect.

---

## Verdict

```
Conformance:  [1 PASS · 2 PASS · 3 PASS · 4 PASS · 5 PASS · 6 PASS · 7 PASS · 8 PASS]
Graded asks:  [dedupe PASS · zero-drops/spine PASS · decoupling+kernel PASS · conventions PASS · disjoint PASS]
Trace audit:  [completeness PASS · justified-drops PASS · landing PASS · no-loss PASS]

VERDICT: PASS
Failing items: none
Required fixes: none (artifact). Optional (report only): correct "96 ln"→132; correct "router DEFERRED"→already registered.
```

Graded independently by `verify-output`; default-FAIL flipped to PASS only on evidence Read and grepped from the artifact itself.
