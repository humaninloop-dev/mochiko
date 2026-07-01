# TRANSFORM: P10 — `plan-template.md`

**Source:** `human-in-loop/plugins/humaninloop/templates/plan-template.md`
**Artifact:** `plugins/mochiko/templates/plan-template.md`
**Cluster:** plan (core-only port) · **Phase 3 (transform)** · **Date:** 2026-06-30
**Producer:** `mochiko:transform-producer` · **Skill:** `mochiko:transform-recipes`
**Role:** apply finalized disposition + run the convention-wiring pass. **NOT** grade (→ `verify-output`, a different agent).
**Inputs consumed:** `assess-plan-template.md` (placeholder catalog + keep-`[...]` finding) · `reconcile.md` (P10 rows 262–264, 295, 375, 454; Job 2A#6) · HIL source · sibling `spec-template.md`.

**Finalized disposition (from reconcile):** **`port-with-edits × standalone`** — the user-facing `plan.md`
deliverable structure; the LEAD's fill-target. Assembly is the lead's (P1 §B / Job 2A#6); the template
stays inert structure.

---

## Output block

```
TRANSFORM: P10 — plan-template.md
Applied:   port-with-edits × standalone + wiring-pass
Artifacts: plugins/mochiko/templates/plan-template.md (created)
           .mochiko/transform/plan/transform-plan-template.md (this trace)
New partners: none (standalone; a template pairs with no validator and splits into nothing)
Wiring:    classification = N/A-tag, floor paid (passive template, no frontmatter — matches spec-template)
           router         = NOT registered here (shared router/plugin.json untouched per instruction);
                            entry NOTED below for the lead to add when the Plan cluster lands
           triggers       = N/A (a template carries no description triggers)
           rebinds        = L4 Input path /specs/[###-feature-name]/spec.md → .mochiko/specs/<feature>/spec.md;
                            bare artifact refs ("See <artifact>", Artifacts rows) resolve under
                            .mochiko/specs/<feature>/ (no literal text change)
           single-source  = clean (no loop-discipline / workflow-contract doctrine inlined; nothing to dedupe)
Trace (realized): every responsibility tagged below — no silent loss
```

---

## Body edits applied (`port-with-edits` — exactly the 3 fixes the body earned; everything else verbatim)

| # | HIL line | Change | Convention |
|---|----------|--------|------------|
| 1 | L4 `**Input**: … from `/specs/[###-feature-name]/spec.md`` | → `… from `.mochiko/specs/<feature>/spec.md`` | path rebind |
| 2 | L6 `**Note**: This template is filled in by the `/humaninloop:plan` command.` | **line removed entirely** | decouple (producer-command stamp) |
| 3 | L57 `Run `/humaninloop:tasks` to generate implementation tasks from this plan.` | → `Run the tasks workflow to generate implementation tasks from this plan.` | decouple (softened to generic phrasing — `tasks` is deferred/unported; a `/mochiko:tasks` link would dangle) |

**Everything else `keep-verbatim`** (minimalism governor — body earned three localized fixes, not a redesign):
all 7 sections + their tables, the bracket `[...]` placeholders, the "See `<artifact>`" pointers, the Branch/Date/Spec header metadata, the `✅ Complete` Artifacts statuses. **No `[...]`→`{{...}}` conversion** (brackets are correct for a lead who assembles by summarizing — `{{...}}` is the kernel-substitution style P10 deliberately is NOT). **No `---` section separators added** and **no header→blockquote restructuring** — those are beyond the three earned fixes; the spec-template reference is for the clean-deliverable shape + the stamp-drop precedent, not a mandate to restructure kept content.

---

## Realized responsibility trace (COMPLETE — every part tagged)

- **Provide the `plan.md` deliverable structure** — 7 sections (Summary · Key Decisions · Infrastructure Requirements · Entities · Endpoints · Artifacts · Next Steps) + their tables → **`kept`** (verbatim).
- **Bracket `[...]` model-fill placeholders** (the "Extract from…" prompts, `[chosen option]`, `[brief why]`, etc.) → **`kept`** — NOT converted to `{{...}}`; brackets are model-fill guidance for a lead-assembled summary, the deliberate non-substitution style.
- **Header metadata** (title `[FEATURE]`, `Branch [###-feature-name]`, `Date [DATE]`, `Spec [link]`) → **`kept`** — preserved as the assessment specified (Branch = `kept`, not rebind). *Observation (no change made):* the Branch placeholder still carries HIL's `###-` numbering token while paths now use the un-numbered `<feature>` slug; left as-is per the finalized `kept` tag — a cosmetic vocabulary nit the lead may modernize later, not a transform-time decision to improvise.
- **L4 Input path** `/specs/[###-feature-name]/spec.md` → **`kept-but-rebind`** → `.mochiko/specs/<feature>/spec.md` (workspace-as-state; matches `specify.md` + `authoring-requirements` path convention).
- **"See `<artifact>` for full …" cross-references** (`constraints-and-decisions.md` ×2 · `data-model.md` · `contracts/api.yaml`) → **`kept-but-rebind`** — bare workspace-relative filenames, co-located with `plan.md` under `.mochiko/specs/<feature>/`; resolution context rebinds, literal text unchanged.
- **Artifacts table** (6-row roster + `✅ Complete` status) → **`kept-but-rebind`** — roster CONFIRMED at 6 per reconcile P10 F2 "keep roster"; resolves under `.mochiko/specs/<feature>/` (see roster confirmation below).
- **L6 producer-command stamp** (`filled in by /humaninloop:plan`) → **`dropped + reason`**: decouple convention scrubs producer naming from the deliverable; `spec-template.md` (closest ported analog) carries no such note. Recorded for the lead to accept (it does not vanish silently).
- **L57 Next-Steps command ref** (`/humaninloop:tasks`) → **`kept-but-rebind`** (semantic kept; coupling scrubbed) → softened to generic "Run the tasks workflow…". `tasks` is a deferred, not-yet-ported primitive (zero `mochiko:tasks` references in the repo) — generic phrasing avoids a dangling link while preserving the next-step pointer; re-link to `/mochiko:tasks` when that primitive lands.
- **`plan.md` ASSEMBLY logic** (extract key-decisions ← `constraints-and-decisions.md`; entity summary incl. sensitivity ← `data-model.md`; endpoint summary incl. integration ← `contracts/api.yaml`) → **`moved-to-lead`** — owned by the thinned `plan` supervisor (assess P1 §B; reconcile Job 2A#6). **Cross-reference, NOT re-homed here and NOT a silent drop:** the template holds the structure; the lead is the assembler and this template is the lead's **fill-target**.

**Homeless-responsibility check:** none. Every part is `kept`, `kept-but-rebind`, `dropped + reason`, or `moved-to-lead` (cross-ref).

---

## Convention-wiring pass (all 6 — the floor every port pays)

1. **Classification** → **N/A tag, floor paid.** A passive template asset: not user-invoked, not model-invoked; carries no frontmatter and no `disable-model-invocation` — matches the `spec-template.md` precedent (raw template, no frontmatter). Nothing to set; the floor is acknowledged, not skipped.
2. **Router registration** → **NOT done here (by instruction); entry NOTED.** The shared router (`plugins/mochiko/skills/mochiko/SKILL.md`) and `plugin.json` were left untouched. `plugin.json` does not register templates at all (only `commands/`, `skills/`, `agents/`), so no manifest edit is ever owed. The router SKILL.md indexes templates per-cluster; there is no Plan-cluster section yet (plan unported). **Entry the lead must add when the Plan cluster lands** (mirrors the `spec-template` row, SKILL.md:66):
   > `| `plan-template` (template) | the `plan.md` deliverable the lead assembles at Phase-4 by rolling up the producer cluster's outputs — Key Decisions (constraints-and-decisions) · Infrastructure/IP-XXX (constraints-and-decisions Part 3) · Entities+Sensitivity (data-model) · Endpoints+Integration (contracts/api.yaml); the lead's fill-target, `[...]` model-fill style |`

   Until registered, the template is discoverable only by the lead/contract that references it — flagged so it is not orphaned (a `verify-output` Part-A discoverability item).
3. **Trigger phrasing** → **N/A.** No `description` on a template; nothing to phrase.
4. **Path rebinding** → **applied.** L4 `/specs/[###-feature-name]/spec.md` → `.mochiko/specs/<feature>/spec.md`; bare "See `<artifact>`" + Artifacts-table filenames resolve under `.mochiko/specs/<feature>/` (co-located with `plan.md`). No `${CLAUDE_PLUGIN_ROOT}`, no `.humaninloop/`, no catalog/MCP/DAG path existed → none to strip.
5. **Decouple persona/skill** → **applied.** L6 producer-command stamp dropped; L57 command ref softened to generic phrasing. Emitted artifact names (Artifacts rows, "See X" refs) name **no producer** — bare filenames, zero "Generated by…" stamps → already clean. No sibling-agent names, no "dispatch," no injected workflow modes/phases (a template carries no persona) → nothing else to scrub.
6. **Single-source / de-duplicate** → **clean.** Pure deliverable structure; no `loop-discipline` / `workflow-contract` / `agent-dispatch` doctrine restated, no filled contract inlined. Nothing to reference-not-restate.

---

## Artifacts roster — CONFIRMED (reconcile P10 F2 "keep roster")

The table matches the **techspec-merged** producer-cluster output set:

| Row | Status | Producer-cluster source |
|-----|--------|--------------------------|
| `requirements.md` | ✅ kept | technical-analyst / `authoring-technical-requirements` (P5) |
| `constraints-and-decisions.md` | ✅ kept | P5 (artifact-owner) ⊕ P6 (technique) |
| `nfrs.md` | ✅ kept | P5 `authoring-technical-requirements` |
| `data-model.md` | ✅ kept | P7 `patterns-entity-modeling` — **Sensitivity folded in** (DS-XXX canonical home P7, RQ6a) |
| `contracts/api.yaml` | ✅ kept | P8 `patterns-api-contracts` — **Integration folded in** (x-integration canonical home P8, RQ6b) |
| `quickstart.md` | ✅ kept | persona-produced (technical-analyst), no dedicated producer skill |

- **Techspec-merged form satisfied:** Sensitivity rides on the Entities/`data-model.md` row (not a separate `data-sensitivity.md` row); Integration rides on the Endpoints/`contracts/api.yaml` row (not a separate `integrations.md` row). The template already carried **no** `integrations.md` / `data-sensitivity.md` rows → no phantom rows to prune on that axis.
- **`quickstart.md` retained per the finalized decision.** The dispatch brief raised pruning it "if no dedicated producer emits it"; the finalized reconcile verdict (P10 F2, row 263 + 454) is **keep roster — quickstart persona-produced**, so it is NOT a phantom and is kept. Transform applies that decision rather than re-deciding it at the keyboard. **Surfaced for the lead/human** (silent-drop guard): if the ported plan workflow ultimately does not emit `quickstart.md`, drop this one row to avoid asserting a non-existent deliverable — a roster trim, not a structural change.

---

## Drops (lead/human must accept — none silent)

| Dropped | Reason | Precedent |
|---------|--------|-----------|
| L6 `**Note**: filled in by /humaninloop:plan` | decouple convention scrubs producer naming from the deliverable | `spec-template.md` carries no such note |

*(L57's `/humaninloop:tasks` is **not** a drop — it is `kept-but-rebind`, softened to generic phrasing; the next-step responsibility survives.)*

---

## Handoff to `verify-output` (independent grade — different agent)

- Artifact: `plugins/mochiko/templates/plan-template.md`
- Trace: this file.
- Independence: this transform did **not** grade itself. The done-condition check (five conventions, sound-loop placement, trace audit for silent loss) is owed to an independent `validator` running `verify-output`.
- Watch-items for the grader: (a) the kept `[...]` style is intentional, not an un-ported `{{...}}` omission; (b) router entry is deliberately a NOTE, not an edit, per dispatch; (c) the `quickstart.md` roster row is kept per reconcile P10 F2 (with the surfaced lead-confirm flag), not an oversight; (d) the Branch `[###-feature-name]` vocabulary nit is a held `kept` tag, not a missed rebind.

---

**Transform version:** v1 · **Governed by:** `loop-discipline` · **Role:** apply + wire only — no grade.
