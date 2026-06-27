# Assessment — `templates/codebase-inventory-schema.json` (P10)

Run: `setup` cluster transform · Phase 1 (assess) · Producer: `transform-producer`
Source (read-only): `human-in-loop/plugins/humaninloop/templates/codebase-inventory-schema.json`
Stated consumer (run context): "analysis-codebase / detect-stack.sh". **Empirical finding below revises this.**

---

## Step 1 — Branch by class

**Class: template → artifact branch.** Weight on placeholders, what consumes it, and path coupling.

This is a **JSON Schema (draft-07)** — `$id: "codebase-inventory-schema.json"` — defining the contract for the **codebase discovery / inventory** output: `meta` (discovery run + bounds + stats), `project_info`, `dependencies`, `entities`, `endpoints`, `existing_features`, `domain_vocabulary`, `conventions`, **`collision_risks`** (with `spec_item` vs `existing_item`, `compatibility`, `recommended_action`), and `warnings`. Unlike P7/P9 it is not a fill-in prose shape — it is a **machine-validatable contract definition**.

**Empirical consumption (verified by grep over the whole HIL plugin + brain):** the schema is referenced by **nobody** — no skill, no agent, no command, and **not** `detect-stack.sh`. The only mentions are `CHANGELOG.md` and the file's own `$id`. `detect-stack.sh` emits a **different, simpler** JSON (`project_type / package_manager / frameworks / orms / architecture / ci_cd / files_found`) and does **not** validate against this schema. The schema's shape (entities + endpoints + **collision_risks with spec_item**) matches `analysis-codebase` **Brownfield mode** (`references/BROWNFIELD-ANALYSIS.md` Output Format), whose purpose is **"planning new features against existing code"** — i.e. **spec-vs-codebase collision detection for the spec/plan cluster, NOT setup.** The setup cluster's brownfield path produces **markdown** (`codebase-analysis.md` per P9), not this JSON.

## Step 2 — Fast-path triage gate

1. **Orchestration-coupled?** `gate1 = NO` — a static schema is a passive contract; it depends on no kernel, supervisor, command, or DAG to exist. No kernel/DAG/catalog/MCP content.
2. **Multi-responsibility / fans out?** `gate2 = NO` — one schema, one responsibility (define the inventory/collision-risk contract). Currently it **fans out to nobody** (orphan — no enforcing consumer).
3. **Emits a non-machine-checkable artifact?** `gate3 = NO` — the schema **is the machine-check.** Validating an inventory instance against it is a **deterministic JSON-schema assert** → any validator degenerates to a deterministic check; **no producer↔validator pairing is implied.**

**All three = NO → FAST-PATH.** Per the gate: skip the full 7-check lens; apply the **convention-wiring pass** (`transform-recipes`) and go straight to `verify-output`. The cross-cluster ownership note below is recorded for the lead but does not change the fast-path result.

## Step 3 — Lens

**Skipped (fast-path).** Condensed observations recorded instead:
- **Check 1 (orchestration):** none — no kernel/orchestration content; nothing to re-home.
- **Check 6 (coupling audit):** **no `.humaninloop/` paths, no plugin paths, no kernel refs** anywhere in the schema. `$id` is a bare filename (`codebase-inventory-schema.json`) — portable as-is. `meta.agent_model` enum `["sonnet","opus","haiku"]` is a harmless model hint, not kernel coupling. **Body is fully mochiko-clean.**
- **Check 5 (sibling/overlap):** complementary to **P9** (markdown analysis = setup path) — this JSON serves the **spec/plan collision path**, a different mode/cluster. Not a dedupe. Overlaps in spirit with `analysis-codebase` **F1 (mode-scoping split)** — the schema *is* the contract for the Brownfield/collision mode that F1 proposes moving toward spec/plan.

## Step 4 — Disposition

**Body × Structural = `keep-verbatim` × `standalone`.**

- **Body = `keep-verbatim`.** Clean draft-07 JSON Schema, no `.humaninloop/`/kernel/DAG/catalog references, portable `$id`. Nothing earns an edit. The convention-wiring pass still runs (router/template registration; path-rebind audit = nothing to rebind).
- **Structural = `standalone`.** The task lists P10 as in-scope to port **this run**, so it lands as one mochiko template file. **No relational structural move is *forced*** (it pairs with no producer that needs splitting; its validator is a deterministic assert). The cross-cluster *ownership* question (below) is a soft lead/reconcile note, not a structural `flag-for-reconcile` on P10's own form.

## Step 5 — Responsibility trace (every responsibility tagged)

| # | Responsibility | Tag |
|---|----------------|-----|
| R1 | Define the codebase **inventory** contract: `meta` (discovery run, bounds, stats, status), `project_info`, `dependencies` | **kept** (keep-verbatim) |
| R2 | Define **entities** contract (fields, relationships, domain_terms, table_name, entity_type) | **kept** |
| R3 | Define **endpoints** contract (method/path/handler/related_entity/auth/tags) | **kept** |
| R4 | Define `existing_features`, `domain_vocabulary` (terms + entity_mappings), `conventions` (naming/structure/api_patterns) | **kept** |
| R5 | Define **`collision_risks`** contract (`spec_item` vs `existing_item`, `compatibility`, `recommended_action`, `risk_level`, resolution_options) | **kept** — BUT this is the **spec-vs-code collision** responsibility, whose real consumer is the **spec/plan cluster**, not setup → see F-P10-1 (candidate **moved-to-other-cluster**) |
| R6 | Define `warnings` contract (inconsistencies/severity/recommendations) | **kept** |
| R7 | Serve as the **deterministic validation contract** for the inventory artifact (machine-checkable assert; degenerate validator) | **kept** (this is *why* gate3=no — the schema is its own validator; no produce+grade pairing needed) |
| R8 | Discoverability / placement (router/template registration; `$id` is portable; no path rebind owed) | **kept-but-rebind** (convention-wiring floor — registration only, nothing in body to rebind) |
| R9 | (Current state) enforce nothing — **orphan**: no skill/script/agent validates against it in HIL | **dropped + reason** *(as a current obligation)*: there is no live enforcement to carry; mochiko should either **wire a real consumer** (the Brownfield/collision mode that uses it) or accept it as a documented-but-unenforced contract. Lead to accept. |

No responsibility left untagged. R9's drop names the gap (no enforcing consumer) so the lead can decide to wire one or accept dormancy.

## Reconcile flags

- **F-P10-1 (soft, lead/cross-cluster ownership) — schema belongs to the spec/plan collision path, not setup.** The schema's `collision_risks`/`spec_item` shape is the contract for `analysis-codebase` **Brownfield mode** ("planning new features against existing code"), which serves **spec/plan**. The setup cluster's brownfield path uses the **markdown** template (P9). Confirm with the lead whether P10 should (a) port to mochiko **standalone now** (per the run's stated in-scope list) and be wired to its real consumer later, or (b) be tagged **`moved-to-other-cluster`** and ported with the plan/specify cluster. Directly parallels **analysis-codebase F1 (mode-scoping split)** — resolve them together. Partners: spec/plan cluster, `analysis-codebase` (P6).
- **F-P10-2 (note) — orphan contract.** No HIL primitive validates against this schema today (verified by grep); `detect-stack.sh` emits a different JSON. If P10 ports into setup, the lead should accept that it lands as an **unenforced contract** unless a consumer (the Brownfield/collision mode) is wired in the same or a later run.

*(Both are soft notes — fast-path triage stands. P10's own form is `keep-verbatim × standalone`; the flags concern which cluster owns it and whether it has a live consumer, not how its body is treated.)*

---

### Summary block

```
ASSESSMENT: codebase-inventory-schema.json (P10)
Class:        template → branch artifact (JSON Schema, draft-07; machine-validatable contract)
Triage:       gate1=n gate2=n gate3=n  [FAST-PATH → wiring pass → verify-output]
Disposition:  keep-verbatim × standalone
Trace:
  - R1 inventory contract (meta/project_info/dependencies)        → kept
  - R2 entities contract                                          → kept
  - R3 endpoints contract                                         → kept
  - R4 features/domain_vocabulary/conventions contract            → kept
  - R5 collision_risks contract (spec_item vs existing)           → kept; spec-vs-code = spec/plan concern → F-P10-1 (candidate moved-to-other-cluster)
  - R6 warnings contract                                          → kept
  - R7 deterministic validation contract (own machine-check)      → kept (why gate3=no; degenerate validator, no pairing)
  - R8 discoverability/placement ($id portable; no rebind owed)   → kept-but-rebind (registration only)
  - R9 (current) enforces nothing — orphan, no live consumer      → dropped + reason (no enforcement to carry; lead to wire a consumer or accept dormancy)
Reconcile flags:
  - F-P10-1 (soft) cross-cluster ownership: schema's collision/spec_item shape serves spec/plan, not setup — resolve with analysis-codebase F1; partner spec/plan + P6
  - F-P10-2 (note) orphan contract: nothing validates against it in HIL (not even detect-stack.sh) — lands unenforced unless a consumer is wired
```
