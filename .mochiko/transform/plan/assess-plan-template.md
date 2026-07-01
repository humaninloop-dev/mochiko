# ASSESSMENT: P10 — `plan-template.md`

**Source:** `human-in-loop/plugins/humaninloop/templates/plan-template.md`
**Cluster:** plan (core-only port)
**Assessed:** 2026-06-30 · ROLE: assess/diagnose ONLY (no transform, no grade, no cross-primitive resolution)
**Branch context:** template → **artifact** branch. This is the FINAL `plan.md` summary deliverable — the plan workflow's user-facing completion artifact, assembled (in HIL) by the supervisor from `constraints-and-decisions.md` + `data-model.md` + `contracts/api.yaml`. Close analog of the cleanly-ported `spec-template.md`.

---

## Class / Branch

- **Class:** template → **artifact** branch.
- What matters most for this branch (per the Step-1 table): **placeholders** (catalog + upstream source), **what consumes/assembles it**, and **path coupling**. Classification / trigger-reliability / persona-decoupling checks are category errors for a passive template asset.
- **Not to be confused with P11** (`plan-context-template.md`): that is the `.workflow/` **state-carrier** (`{{phase}}`/`{{status}}`/`{{iteration}}`/`supervisor_instructions`/File-Paths table), kernel-adjacent, slated for `absorb-into-lead`. **P10 is the deliverable structure**, not state.

## Triage (3 gates — templates usually fast-path)

| Gate | Q | Verdict |
|------|---|---------|
| 1 | Orchestration-coupled (depends on a kernel / markdown supervisor / command / DAG **to function**)? | **NO** — a template is inert structure; it functions as a fill-target regardless of who fills it. No `${CLAUDE_PLUGIN_ROOT}`, no `.humaninloop/memory`, no brain script, no catalog, no DAG. The command-name mentions (lines 6, 57) and the `/specs/…` path (line 4) are **content/naming coupling** (wiring-pass territory), not orchestration-coupling. |
| 2 | Multi-responsibility / fans out (>1 responsibility OR feeds >1 consumer)? | **NO** — one responsibility: *be the structure for the `plan.md` deliverable*. One consumer: the **assembler** (the lead). It is *filled from* several upstream artifacts, but that is "filled-from-many," not "feeds-many." (The filled `plan.md` is later read by the user + `/tasks`; that is a property of the artifact, not the template.) |
| 3 | Emits an artifact whose correctness is NOT machine-checkable? | **NO** — a template **emits nothing**; it is a static structure, not a producer. No producer↔validator pairing attaches to it. (The lead-assembled `plan.md` is model-judged, but the template does not produce it.) |

**gate1=n gate2=n gate3=n → fast-path.** (Templates usually fast-path; confirmed.) Fast-path means *skip the full 7-check lens*, **not** skip the trace — the responsibility trace remains the done-condition, and the convention-wiring pass (path-rebind · decouple-scrub · single-source) still runs at transform time.

## Disposition

**`port-with-edits` × `standalone`** — no `flag-for-reconcile` on the disposition itself (placement is decidable solo; a deliverable template has one home and one assembler). Relational **content-consistency** notes are flagged separately below (they do not change the disposition).

- **Body = `port-with-edits`** (minimalism governor applied):
  - **NOT `keep-verbatim`** — the body is not yet mochiko-clean: line 4 carries the HIL `/specs/[###-feature-name]/…` path, line 6 carries a producer-command stamp (`/humaninloop:plan`), line 57 carries a `/humaninloop:tasks` command ref. Three localized fixes are required.
  - **NOT `redesign`** — the structure is sound for a lead-assembled deliverable; the bracket placeholders are *appropriate* (see note). Nothing assumes a kernel/DAG/catalog.
  - → **`port-with-edits`**: rebind line 4's path, scrub line 6's producer stamp, rebind/soften line 57's command ref; **everything else keeps verbatim** (sections, tables, bracket placeholders, "See X" pointers).
- **Structural = `standalone`** — self-contained, one home: `plugins/mochiko/templates/plan-template.md`. No sibling template to merge with; no validator to pair with (a template is a structure, not a graded output). Mirrors how `spec-template.md` was placed.

**Placeholder-style note (a feature, keep it):** plan-template uses **bracket `[...]` placeholders** that read as model-fill *guidance* ("Extract from feature spec…", "[chosen option]", "[brief why]"). This contrasts with the **`{{...}}` kernel-substitution style** of P11 and `spec-template.md`. Brackets are *correct* for a deliverable the **lead assembles by summarizing/extracting** (model judgment) rather than by mechanical substitution. **Do not convert `[...]`→`{{...}}`** — that would re-impose the substitution model the lead replaces. `kept`.

---

## Placeholder catalog (every placeholder → upstream source)

| Placeholder | Where | Filled from (upstream artifact) | Producer-cluster owner |
|-------------|-------|----------------------------------|------------------------|
| `[FEATURE]` | L1 title | feature name | spec / feature-id |
| `[###-feature-name]` | L3 Branch | feature-id / branch | workspace |
| `[DATE]` | L3 | run date | workspace |
| `[link]` | L3 Spec | spec.md location | workspace (REBIND) |
| `/specs/[###-feature-name]/spec.md` | L4 Input | spec.md path | **path coupling → REBIND** |
| `[Extract from feature spec: primary requirement + technical approach…]` | L10 Summary | `spec.md` (primary requirement) + `constraints-and-decisions.md` (approach) | spec + P6 |
| `[D-001 title]` / `[chosen option]` / `[C-XXX references]` / `[brief why]` | L16 Key Decisions | `constraints-and-decisions.md` (decision records) | **P6 patterns-technical-decisions** |
| `[IP-001]` / `[type]` / `[C-XXX/NFR-XXX]` / `[MUST/SHOULD]` | L24 Infrastructure | `constraints-and-decisions.md` Part 3 (IP-XXX) | **P5 authoring-technical-requirements** |
| `[Entity name]` / `[NEW/EXTENDS/REUSES]` / `[count]` / `[count]` / `[highest classification]` | L32 Entities | `data-model.md` (entities + **sensitivity** — the techspec-merged data-sensitivity) | **P7 patterns-entity-modeling** |
| `[HTTP method]` / `[path]` / `[description]` / `[external system if any]` | L40 Endpoints | `contracts/api.yaml` (endpoints + **integration** — the techspec-merged x-integration) | **P8 patterns-api-contracts** |
| Artifacts table rows + `✅ Complete` status | L48–53 | the workflow's actual produced file set (workspace evidence) | the cluster's deliverable roster |
| `/humaninloop:tasks` | L57 Next Steps | downstream command ref | **command coupling → REBIND/decouple** |

**Reading of the catalog:** the template is a **roll-up of the producer cluster's outputs** — Key Decisions←P6, Infrastructure←P5, Entities(+Sensitivity)←P7, Endpoints(+Integration)←P8 — plus header metadata and the deliverable roster. Each section is a *summary view* (`See <artifact> for full …`) of one upstream artifact; the template holds the **structure**, not the data and not the assembly logic.

## Path coupling (the artifact-branch focus)

- **L4 `/specs/[###-feature-name]/spec.md`** → **`kept-but-rebind`** → `.mochiko/specs/<feature>/spec.md`. This is the **memory-model / workspace-as-state precedent** the task names: setup + specify rebound their header path/status fields to workspace-as-state, and P1 §C already tags `specs/{feature-id}/… → kept-but-rebind to .mochiko/specs/<feature>/…`. Same rebind here.
- **L18 / L26 / L34 / L42 "See `constraints-and-decisions.md` / `data-model.md` / `contracts/api.yaml` …"** → **`kept-but-rebind`** — these are **bare workspace-relative filenames** (no `.humaninloop/`, no `${CLAUDE_PLUGIN_ROOT}`); semantics keep, they resolve under `.mochiko/specs/<feature>/`. Lighter coupling than the general template warning anticipated.
- **No `${CLAUDE_PLUGIN_ROOT}`, no `.humaninloop/`, no brain/DAG/catalog reference anywhere in this template** — kernel-free is clean. The only absolute path is L4's `/specs/…`.

## Decouple scan (no producer naming on the deliverable)

- **L6 `**Note**: This template is filled in by the /humaninloop:plan command.`** → **`dropped + reason`**: a **producer-command stamp** on the deliverable. The decouple convention scrubs producer naming from the emitted artifact ("Generated by…" → scrub); the closest ported analog, `spec-template.md`, carries **no such note**. *Alternative considered:* rebind `/humaninloop:plan`→`/mochiko:plan` and keep the note — **rejected**: the note exists only to name the producer, which the convention removes; scrubbing is both cleaner and precedented. (Final keep/scrub is the transform-time application of the decouple convention; assessed recommendation = **scrub**.)
- **L57 `Run /humaninloop:tasks to generate implementation tasks from this plan.`** → **`kept-but-rebind`**: `/humaninloop:tasks` → `/mochiko:tasks` (rebind-by-reference; `tasks` is a **deferred, not-yet-ported** primitive per `context.md`). Because the target command does not yet exist in mochiko, transform may **soften to generic phrasing** ("…run the tasks workflow to generate implementation tasks") rather than hard-link a missing command. Naming/wiring call, not relational.

## What consumes / assembles it (the assembler)

- **The lead assembles it; the template stays a template.** P1 (`assess-plan-command.md`) §B already tags **"Phase 4 completion — assemble `plan.md` summary by extracting key decisions (constraints-and-decisions), entity summary incl. sensitivity (data-model), endpoint summary incl. integrations (contracts/api.yaml) → `moved-to-lead`"**. So the **assembly logic** is owned by the thinned `plan` supervisor (the lead); **P10 holds only the structure** the lead fills. This is **confirmed, not an open question** — recorded here so the assembly is not read as a silent drop of P10, and flagged below only so reconcile's **rehome map names this template as the lead's fill-target**.

---

## Responsibility trace (COMPLETE)

The template holds one responsibility (be the deliverable structure); the trace enumerates its sub-parts and couplings so nothing is silently lost.

- **Provide the `plan.md` deliverable structure** — sections (Summary · Key Decisions · Infrastructure Requirements · Entities · Endpoints · Artifacts · Next Steps) and their tables → **`kept`**.
- **Bracket `[...]` model-fill placeholders** (fill-guidance for a lead-assembled summary; the "extract from…" prompts) → **`kept`** (do **not** convert to `{{…}}`; brackets are correct for lead/model assembly — see note).
- **Header metadata** (title `[FEATURE]`, Branch, Date, Spec `[link]`) → **`kept`**.
- **L4 Input path** `/specs/[###-feature-name]/spec.md` → **`kept-but-rebind`** → `.mochiko/specs/<feature>/spec.md` (workspace-as-state precedent).
- **L18/26/34/42 "See `<artifact>` for full …" cross-references** (constraints-and-decisions · data-model · contracts/api.yaml) → **`kept-but-rebind`** (workspace-relative; resolve under `.mochiko/specs/<feature>/`).
- **L48–53 Artifacts table** (deliverable roster + `✅ Complete` status) → **`kept-but-rebind`** + **content-consistency reconcile note F1/F2** (roster must match the finalized producer-cluster output set; `requirements.md` / `nfrs.md` / `quickstart.md` presence to confirm).
- **L6 producer-command stamp** (`filled in by /humaninloop:plan`) → **`dropped + reason`** (decouple convention scrubs producer naming on the deliverable; `spec-template.md` precedent carries none).
- **L57 Next-Steps command ref** (`/humaninloop:tasks`) → **`kept-but-rebind`** (`→ /mochiko:tasks`, a deferred/unported primitive; transform may soften to generic phrasing).
- **`plan.md` ASSEMBLY logic** (extract key-decisions / entity+sensitivity / endpoint+integration summaries from the upstream artifacts) → **`moved-to-lead`** — *already owned by P1 §B (Phase-4 completion)*; **not held by the template** and **not re-homed here**. Recorded as a cross-reference so the assembly is auditable and not read as a silent drop.

**Convention-wiring applicability (for the transform pass, not "held" responsibilities):**
- Classification tag → **N/A** (passive template asset; no `disable-model-invocation`, not user/model-invoked).
- Router registration → **N/A** (templates are *referenced* by the lead/contract, not registered as invocable router entries — mirrors `spec-template.md` / `advocate-report-template.md`).
- Trigger phrasing → **N/A** (no `description` triggers on a template).
- Path-rebinding → **applies** (L4; the "See X" refs) — recorded above as `kept-but-rebind`.
- Decouple-scrub → **applies** (L6 producer stamp; L57 command ref) — recorded above.
- Single-source / de-dup → **clean** (no `loop-discipline`/contract doctrine inlined; nothing to de-duplicate).

---

## Reconcile flags (relational — NOT decided here)

The **disposition (`port-with-edits × standalone`) is not sibling-dependent and stands.** These flags are **content-roster consistency** confirmations plus a rehome-map record — resolvable once the producer skills (P5–P8) + the `technical-analyst` port + RQ1 are reconciled.

1. **F1 — section/roster ↔ producer-cluster consistency.** The template's sections are a roll-up of producer outputs: Key Decisions←P6, Infrastructure(IP-XXX)←P5, Entities(+Sensitivity)←P7, Endpoints(+Integration)←P8. If any producer skill's output structure/column-naming shifts on port, **sync the template's matching column/section**. *Already consistent with the techspec-merged form* — Sensitivity column on Entities, Integration column on Endpoints, **no** separate `integrations.md` / `data-sensitivity.md` rows (folded into data-model + contracts, per `context.md`). Reconcile confirms after P5–P8 land.
2. **F2 — Artifacts-table deliverable set.** The table asserts `requirements.md`, `nfrs.md`, `quickstart.md` as `✅ Complete`, but plan-core's resolved 15 primitives include **no dedicated `quickstart` producer** (and `requirements.md`/`nfrs.md` are technical-analyst / `authoring-technical-requirements` outputs). Reconcile decides, with the finalized output set in view, whether the ported plan workflow produces these three — **prune or keep** the Artifacts rows to match (esp. `quickstart.md`: drop vs. keep-as-future-reference).
3. **F3 — assembly-ownership edge (record, don't re-decide).** P1 already tags `plan.md` assembly → `moved-to-lead`. Reconcile's **rehome map** should name **this template as the lead's fill-target** (the lead assembles; the template stays structure-only). Confirmation, not an open structural decision.

*(No structural move pending: standalone is solo-decidable; there is no split/merge/promote/pair for a deliverable template.)*

## Silent-drop risks (for the lead/human to accept)

- **L6 producer-stamp drop** — assessed as scrub (decouple convention + `spec-template` precedent); listed so the human can accept the drop rather than have it vanish silently.
- **F2 quickstart/requirements/nfrs Artifacts rows** — if the ported workflow does **not** produce one of these, leaving its `✅ Complete` row would assert a deliverable that doesn't exist. Must be reconciled against the finalized output set, not carried blindly.
- **Assembly logic is NOT in this template** — it lives on the lead (P1 §B). If a reader expects the template to "do" the summarization, that is a category error; the template is structure, the lead is the assembler.

---

## Output block

```
ASSESSMENT: P10 — plan-template.md
Class:        template → branch artifact
Triage:       gate1=n gate2=n gate3=n  → fast-path  (trace + wiring-pass still run)
Disposition:  port-with-edits × standalone   (no flag on the disposition itself)
Trace:        complete — every part tagged
  kept:            deliverable structure (7 sections + tables) · bracket [...] model-fill placeholders
                   (do NOT convert to {{...}}) · header metadata
  kept-but-rebind: L4 /specs/[###-feature-name]/spec.md → .mochiko/specs/<feature>/spec.md
                   (workspace-as-state) · "See <artifact>" refs (workspace-relative) ·
                   Artifacts table roster (+F1/F2) · L57 /humaninloop:tasks → /mochiko:tasks (deferred)
  dropped+reason:  L6 "filled in by /humaninloop:plan" producer-command stamp
                   (decouple convention; spec-template precedent carries none)
  moved-to-lead:   plan.md ASSEMBLY logic — already owned by P1 §B Phase-4 (template stays structure-only;
                   recorded as cross-ref, not re-homed here)
  wiring N/A:      classification · router-registration · trigger-phrasing (passive template asset)
Reconcile flags: F1 section/roster ↔ producer-cluster (P5–P8) consistency ·
                 F2 Artifacts-table deliverable set (requirements/nfrs/quickstart presence) ·
                 F3 assembly-ownership edge → record template as lead's fill-target in rehome map
                 (disposition is NOT sibling-dependent; flags are content-consistency + rehome-record)
Silent-drop risks: L6 producer-stamp drop (accept) · F2 phantom Artifacts rows · assembly lives on lead
```
