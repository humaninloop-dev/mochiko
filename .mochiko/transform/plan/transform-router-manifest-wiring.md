# TRANSFORM — plan cluster: shared-file convention-wiring (router + manifest)

**Run:** `/mochiko:transform-cluster` (plan) · **Phase 3 (transform)** · deferred SHARED-FILE wiring
**Producer:** `mochiko:transform-producer` · **Skill:** `mochiko:transform-recipes` (convention-wiring pass — **step 2 router registration + the manifest** only)
**Role:** APPLY the wiring on the two shared files every Wave-1/Wave-2 producer deferred (to avoid write races). **Did NOT** edit any skill/agent/command/template body. **Did NOT** grade (that is `verify-output`, a different agent).
**Files edited:** `plugins/mochiko/skills/mochiko/SKILL.md` · `plugins/mochiko/.claude-plugin/plugin.json`
**Inputs consumed:** the 15 landed artifacts (for accurate one-line guidance), the 13 `transform-*.md` traces (each producer's drafted router entry), `reconcile.md` §RQ4 + the team roster.

> This is the single cluster-wide router-edit task the per-primitive traces each flagged and deliberately left open ("router registration — NOTED, not applied — shared file"). It closes the `verify-output` Part-A discoverability item for all 10 plan primitives + the 2 broadened agents at once.

---

## 1. Router `skills/mochiko/SKILL.md` — what was registered

### 1a. NEW "Plan cluster" section (model-invoked table; mirrors the Setup/Specify cluster format)
Inserted between the Specify cluster and the Entry-point table. Registers **6 model-invoked skills + 3 hinted templates**, each with when-to-reach-it guidance drawn from the landed artifact `description`s and the producers' drafted entries:

| Registered | When-to-reach guidance (condensed) |
|------------|-------------------------------------|
| `authoring-technical-requirements` | TR/C/NFR/IP layer + `constraints-and-decisions.md` artifact + C↔D/IP traceability; **declares DS-XXX/INT-XXX as analysis concerns only** → points to `patterns-entity-modeling` (sensitivity) / `patterns-api-contracts` (x-integration) |
| `patterns-technical-decisions` | decision **technique** — ≥2 alternatives, trade-offs, ADR depth, NEEDS CLARIFICATION; artifact owned by `authoring-technical-requirements` |
| `patterns-entity-modeling` | `data-model.md` + the **canonical 4-level DS taxonomy** (single home for data-sensitivity) |
| `patterns-api-contracts` | API contracts + the per-endpoint **`x-integration`** boundary; owns `contracts/api.yaml` |
| `validation-plan-artifacts` | **completeness** mirror-checklist — coverage/measurability/consistency/presence → `ready/needs-revision/critical-gaps`; independent validator, never the author |
| `validation-feasibility` | **feasibility** adversarial-critique — contradiction/impossibility/buildability → `feasible/needs-revision/infeasible` (distinct `infeasible` = business escalation); independent reviewer, never the author |
| `plan-template` (template) | the `plan.md` deliverable the lead assembles at Phase 4 (roll-up fill-target) |
| `techanalyst-report-template` (template) | the producer's per-round self-disclosure; no verdict |
| `feasibility-report-template` (template) | the feasibility reviewer's cross-artifact critique report (3-state verdict + 4-field gate fuel) |

Plus a `>` note: the plan **completeness** reviewer reuses the shared `advocate-report-template` as-is — no plan-specific completeness report template.

### 1b. Entry-point table — `/mochiko:plan` row added (user-invoked; hinted, not fired)
`| /mochiko:plan | ...analysis→design implementation plan via the producer→two-reviewer loop (technical-analyst authors; principal-architect grades feasibility, devils-advocate grades completeness) with a human acceptance gate on plan.md |`

### 1c. Agents table — 1 added, 2 broadened
- **ADDED** `technical-analyst` — plan-cluster PRODUCER, authors the six analysis+design artifacts; never grades its own output (skills: authoring-technical-requirements, patterns-technical-decisions, patterns-entity-modeling, patterns-api-contracts).
- **BROADENED** `principal-architect` — was "setup-cluster author" only; now **cross-workflow**: setup author **and** plan-cluster **feasibility reviewer** (via `validation-feasibility`); skills row updated to `authoring-constitution, analysis-codebase, validation-feasibility`.
- **BROADENED** `devils-advocate` — was "specify-cluster adversarial critic" only; now **cross-workflow**: specify critic **and** plan-cluster **completeness reviewer** (via `validation-plan-artifacts`); skills row updated to `analysis-specifications, validation-plan-artifacts`.

### 1d. RQ4 re-file — `advocate-report-template` specify → **shared (specify + plan)**
The Specify-cluster row for `advocate-report-template` was re-filed to note it is **shared (specify + plan)** and reused as-is by the plan completeness reviewer (per reconcile §RQ4 — no new plan file emitted).

**Not touched (correctly):** the Flow-graph (scoped to the transform dogfood cluster — setup/specify aren't in it either, so plan isn't added), and all bodies.

---

## 2. Manifest `plugin.json` — the edit

**Edit:** added `"./agents/technical-analyst.md"` to the explicit `agents` array (now 6 entries).

```diff
   "agents": [
     "./agents/principal-architect.md",
     "./agents/transform-producer.md",
     "./agents/validator.md",
     "./agents/requirements-analyst.md",
-    "./agents/devils-advocate.md"
+    "./agents/devils-advocate.md",
+    "./agents/technical-analyst.md"
   ]
```

- `principal-architect` + `devils-advocate` were **already** in the array (broadening is a body/router concern, not a manifest add) — confirmed, no add owed.
- `commands` is a **glob** (`"./commands/"`), not an explicit array → **no `./commands/plan.md` edit owed** (`plan.md` auto-registers). `skills` is likewise a glob (`"./skills/"`) → the 6 plan skills auto-register; no manifest edit owed for them.
- **JSON-valid: CONFIRMED** — `python3 -c "json.load(open('.claude-plugin/plugin.json'))"` parses; `agents` = 6, all 6 paths resolve on disk.

---

## 3. Cluster-wide mount-resolution — CONFIRMED (no dangling `skills:` mounts)

| Agent | `skills:` on disk | All resolve? |
|-------|-------------------|--------------|
| `technical-analyst` | authoring-technical-requirements · patterns-technical-decisions · patterns-entity-modeling · patterns-api-contracts | ✅ all 4 present |
| `principal-architect` | authoring-constitution · analysis-codebase · **validation-feasibility** | ✅ validation-feasibility present (landed this run) |
| `devils-advocate` | analysis-specifications · **validation-plan-artifacts** | ✅ validation-plan-artifacts present (landed this run) |

All 6 plan-cluster skill dirs, all 4 plan templates, the `plan.md` command, and all 6 agent files verified present. **Zero dangling mounts.**

---

## 4. Discrepancy scan (surfaced; none blocking)

- **No naming discrepancy.** Every router name matches an on-disk primitive; `feasibility-report-template` (the P13 rename from `architect-report`) is registered under its landed name.
- **Broadened-agent bodies match their new router rows.** `principal-architect` body carries the §Feasibility Review section + `validation-feasibility` mount + a re-broadened frontmatter `description`; `devils-advocate` mounts `validation-plan-artifacts` with a matching Skills-Available entry. The router rows do not over-claim.
- **Intentional (not a defect):** `devils-advocate`'s frontmatter `description` stays spec-focused — its transform trace (R11) deliberately declined to re-add a plan-completeness `<example>` under the minimalism governor, **relying on the router broadening applied here** for plan-completeness discoverability. The router row is intentionally broader than the agent's example set; this is by design, and the wiring I applied is exactly what closes that gap.
- **Cached agent-summary vs disk:** an environment-generated agent summary described `principal-architect` as setup-only; the on-disk body is correctly re-broadened. No action — the file is authoritative.

---

## Output block (transform-recipes format)

```
TRANSFORM: plan cluster — shared-file wiring (router registration + manifest)
Applied:   convention-wiring pass step 2 (router) + manifest — across the whole plan cluster
           (bodies untouched; not graded)
Artifacts: plugins/mochiko/skills/mochiko/SKILL.md (edited — Plan cluster section + /mochiko:plan
             entry row + technical-analyst agent row + principal-architect/devils-advocate broaden
             + advocate-report-template specify→shared re-file)
           plugins/mochiko/.claude-plugin/plugin.json (edited — technical-analyst added to agents[])
           .mochiko/transform/plan/transform-router-manifest-wiring.md (this wiring record)
New partners: none (registration-only; all primitives already landed by their own transforms)
Wiring:    router=REGISTERED (Plan cluster: 6 skills + 3 templates; /mochiko:plan entry; technical-analyst
             agent; principal-architect + devils-advocate broadened; advocate-report-template → shared)
           manifest=technical-analyst added to agents[]; commands/skills are globs (no add); JSON VALID
           mounts=all resolve (technical-analyst ×4 · principal-architect validation-feasibility ·
             devils-advocate validation-plan-artifacts) — zero dangling
Trace (realized): every plan primitive's deferred "router registration — NOTED, not applied" →
             REGISTERED here; RQ4 advocate-report re-file specify→shared → applied.
Handoff:   → verify-output (a different agent) — Part-A discoverability + independence + no-silent-loss.
```

---

**Wiring record version:** v1 · **Governed by:** `loop-discipline` · **Role:** apply the shared-file wiring only — no body edit, no grade. Hand to `verify-output` (run by a **different** agent).
