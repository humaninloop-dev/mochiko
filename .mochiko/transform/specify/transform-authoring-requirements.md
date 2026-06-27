# Transform (realized) — `authoring-requirements` (P7, specify cluster)

**Producer:** `mochiko:transform-producer` · **Skill:** `mochiko:transform-recipes` · **Date:** 2026-06-27
**Inputs:** `reconcile.md` (§D row P7; §E P7 trace; §G wiring; Agenda 4 keep-distinct + 4c de-collide) · `assess-authoring-requirements.md` · HIL source dir `human-in-loop/plugins/humaninloop/skills/authoring-requirements/`
**Disposition (finalized, human-gate-accepted):** **port-with-edits × standalone** — keep-distinct from P8; do **NOT** factor the shared substrate.
**Phase:** 3 (apply) — APPLY ONLY. Not graded here (independence: `verify-output` via `mochiko:validator`, a different agent).

---

```
TRANSFORM: authoring-requirements
Applied:   port-with-edits × standalone + wiring-pass
Artifacts: plugins/mochiko/skills/authoring-requirements/SKILL.md                         (port-with-edits)
           plugins/mochiko/skills/authoring-requirements/references/RFC-2119-KEYWORDS.md  (verbatim)
           plugins/mochiko/skills/authoring-requirements/references/EDGE-CASES.md         (verbatim)
           plugins/mochiko/skills/authoring-requirements/scripts/validate-requirements.py (verbatim)
New partners: none (standalone; the artifact's independent validator already exists at cluster
              level — devils-advocate + analysis-specifications, a different agent + skill)
Wiring:    classification=model-invoked (default; no disable-model-invocation field)
           router=registered in the separate cluster router artifact (§D row "skills/mochiko/SKILL.md (router) | EDIT"), NOT in this skill dir
           triggers=reframed to work-context; "acceptance criteria" de-collided (P8 owns it)
           rebinds=[ humaninloop:authoring-user-stories → mochiko:authoring-user-stories ;
                     example path path/to/spec.md → .mochiko/specs/<feature>/spec.md ]
           scrubs=[ humaninloop:patterns-api-contracts (deferred/un-ported) → genericized to prose ;
                    humaninloop:patterns-entity-modeling (deferred/un-ported) → genericized to prose ]
Trace (realized): every responsibility tagged below; no silent drops.
```

---

## Body recipe applied — `port-with-edits`

Body is mochiko-clean in substance (FR-XXX/RFC-2119 format, edge-case taxonomy, SC-XXX rules, Key Entities, Quality Checklist, Common Mistakes) with **zero deny-list tokens** (confirmed by grep across the whole dir). All edits are localized **wiring**; section structure, headings, voice, and the WHAT-not-HOW / measurable-outcome prose are preserved. The exact edits (all in `SKILL.md`; every other line verbatim):

| # | Location (HIL source line) | Before | After | Why |
|---|---|---|---|---|
| 1 | frontmatter `description` (L3) | "…when the user says 'write requirements', 'define success criteria'…" (user-utterance triggers) | "…when authoring the functional-requirements layer of a feature specification — writing FR-XXX (RFC 2119), edge cases, SC-XXX…" (work-context); graded SHOULD anchors kept | Trigger reframe → **work-context** (agent-consumed skill, avoid false auto-trigger). **De-collide:** anchored on functional requirements / FR- / success criteria / SC- / edge cases; "acceptance criteria" deliberately **not** introduced (P8 owns it). |
| 2 | "When to Use" (L16) | "Documenting **acceptance criteria** for user stories" | "Documenting the **functional requirements** behind user stories" | **De-collide (4c):** dropped the "acceptance criteria" claim that `authoring-user-stories` (P8) owns; kept the requirements-side responsibility (the FRs behind stories). |
| 3 | "When NOT to Use" (L26) | "API endpoint specifications - Use `humaninloop:patterns-api-contracts` instead" | "API endpoint specifications - These belong to the design/plan track, not business requirements; keep concrete endpoint contracts out of FRs" | **Dangling-link scrub:** `patterns-api-contracts` is a deferred (un-ported) design/plan-track skill → genericized to prose (no dangling link); the "not here / elsewhere" intent preserved. |
| 4 | "When NOT to Use" (L27) | "Data model design - Use `humaninloop:patterns-entity-modeling` instead" | "Data model design - This belongs to the design/plan track; describe entities only conceptually here (see Key Entities), not as schemas" | **Dangling-link scrub:** `patterns-entity-modeling` deferred/un-ported → genericized to prose; redirects to this skill's own conceptual Key Entities section. |
| 5 | "When NOT to Use" (L28) | "User story authoring - Use `humaninloop:authoring-user-stories` instead…" | "User story authoring - Use `mochiko:authoring-user-stories` instead…" | **Namespace rebind:** ported sibling (P8) → `mochiko:`. This line is the **keep-distinct boundary statement** and is preserved (it reinforces the de-collision: user-story / acceptance work → P8). |
| 6 | "Validation Script" (L157) | `python scripts/validate-requirements.py path/to/spec.md` | `python scripts/validate-requirements.py .mochiko/specs/<feature>/spec.md` | **Path rebind** to the mochiko workspace-as-state convention (`.mochiko/specs/<feature>/`). |

**References + script — `keep-verbatim` (faithful port).** `references/RFC-2119-KEYWORDS.md`, `references/EDGE-CASES.md`, and `scripts/validate-requirements.py` were copied unchanged (verified `diff` IDENTICAL against HIL source). None contains a `humaninloop:` ref, a hardcoded `.humaninloop/` path, a deny-list token, or a real spec path (the script's usage string is the generic placeholder `<path-to-spec.md>`). The script compiles (`py_compile` OK).

## Structural recipe applied — `standalone`

Placed as its own `plugins/mochiko/skills/authoring-requirements/` directory (SKILL.md + references/ + scripts/). No partner spun out, no merge, no promote:

- **Independent validator already exists at cluster level** — `spec.md` substance is graded by `devils-advocate` (skill `analysis-specifications`), a **different agent + different skill** than the producer (`requirements-analyst`, which mounts this skill). Producer↔validator independence holds by construction (reconcile Agenda 1a). This skill is the producer half; its `validate-requirements.py` is a degenerate **deterministic Tier-1 self-check** (format only), not that substantive validator.
- **Keep-distinct from P8 (Agenda 4a/4b) — NOT factored.** `authoring-requirements` (FR-XXX / RFC-2119 / SC-XXX / edge cases) and `authoring-user-stories` (P#/Given-When-Then acceptance scenarios) each own a substantial distinct body; this is no thin-variant-over-shared-core → **no merge**. The shared tech-agnostic-discipline substrate (WHAT-not-HOW, measurable outcomes) and the ~80%-similar script scaffold are **deliberately left duplicated**: factoring shared *taste* (low/harmless drift) would manufacture a new cross-skill coupling against kernel-free minimalism. This skill keeps **its own** discipline prose and **its own** script.

## Convention-wiring pass (all five)

1. **Classification → model-invoked** (default). No `disable-model-invocation` field (agent-consumed skill; matches the `analysis-codebase` precedent). ✓
2. **Router registration →** owned by the **separate cluster router artifact** (`reconcile.md` §D lists `skills/mochiko/SKILL.md (router) | EDIT` as its own row; §G registers `authoring-requirements`). **Not edited here** — this Phase-3 task is scoped to the P7 skill dir and is instructed not to touch the shared router (the router is edited once for the whole cluster). Recorded as `kept-but-rebind` pending that artifact. ⏳ (out of this artifact's scope, by design)
3. **Trigger phrasing →** graded exact-phrase triggers reframed to **work context** in `description`; "when the user says" pattern removed; "acceptance criteria" de-collided. ✓
4. **Path rebinding →** example spec path → `.mochiko/specs/<feature>/spec.md`. No `.humaninloop/` paths existed in the body or refs (nothing else to rebind). ✓
5. **Decouple persona/skill →** nothing to scrub: zero sibling-agent names, zero "dispatch," zero injected workflow modes/paths/phases, zero "workflow-agnostic"/independence-by-declaration meta-label (assess found the body already clean; re-confirmed by grep across the whole dir post-port). Independence is stated by **role** (the redirect to `mochiko:authoring-user-stories` is a sibling-*skill* boundary, not an agent name). ✓

## Realized responsibility trace (every responsibility carries a concrete tag)

Flipped from the assess trace to its realized landing. No `flag-for-reconcile` remains (resolved in reconcile §A Agenda 4 → keep-distinct × standalone).

| Responsibility | Realized tag |
|---|---|
| FR-XXX functional-requirements authoring (RFC 2119 MUST/SHOULD/MAY, numbering, tech-agnostic) | **kept** (verbatim body) |
| Edge-case identification (3-5 boundaries; 5 categories + discovery process in `EDGE-CASES.md`) | **kept** (verbatim body + reference) |
| SC-XXX success-criteria authoring (measurable, user/business outcomes) | **kept** (verbatim body) |
| Key-entities conceptual description (optional) | **kept** (verbatim body) |
| Technology-agnostic / no-implementation-leakage discipline (Common Mistakes, Good/Bad) | **kept** — dedupe candidate **RESOLVED: keep-distinct, do NOT factor** (Agenda 4b); this skill keeps its own prose |
| RFC-2119 keyword reference guidance (`RFC-2119-KEYWORDS.md`) | **kept** (verbatim reference) |
| Deterministic format-linter (`validate-requirements.py`: format / sequence / RFC-presence / banned-terms / outcome-focus) | **kept** (verbatim; Tier-1 skill-local self-check, kernel-free — like `detect-stack.sh`) |
| Quality Checklist (pre-finalize self-check) | **kept** (verbatim body) |
| model-invocation trigger `description` | **kept-but-rebind** — user-utterance → work-context; "acceptance criteria" **de-collided** (edit #1) |
| "When to Use" acceptance-criteria use-case | **kept-but-rebind** — reframed to "functional requirements behind user stories" (edit #2, de-collide) |
| "When NOT to Use" sibling cross-ref → `authoring-user-stories` | **kept-but-rebind** — `humaninloop:` → `mochiko:` (edit #5) |
| "When NOT to Use" cross-ref → `patterns-api-contracts` (DEFERRED) | **kept-but-rebind** — dangling link **scrubbed with reason** → genericized to prose (edit #3) |
| "When NOT to Use" cross-ref → `patterns-entity-modeling` (DEFERRED) | **kept-but-rebind** — dangling link **scrubbed with reason** → genericized to prose (edit #4) |
| example invocation path (`path/to/spec.md`) | **kept-but-rebind** — → `.mochiko/specs/<feature>/spec.md` (edit #6) |
| classification tag (absent in HIL) | **kept-but-rebind** (wiring) — assigned **model-invoked** (default) |
| router registration (absent in HIL) | **kept-but-rebind** (wiring) — owned by the separate cluster router artifact (§D), not this dir |

**Drops:** none. No responsibility was dropped; the two deferred-cluster cross-refs were genericized to prose carrying their reason (not silently deleted, not left dangling). Nothing requires a lead-accepted drop reason for this primitive.

## Deviations / notes

- **Router not registered in this artifact (by design).** The wiring pass's router step is satisfied by the cluster's single shared router-edit artifact (reconcile §D/§G), and this task explicitly scopes me out of the shared router. Flagged here for `verify-output` so the router entry is checked against the router artifact, not against this skill dir.
- **De-collision was a no-op in the source `description` but a real edit in the body.** The HIL `description` never listed "acceptance criteria" as a trigger, so anchoring it was preventive; the live collision was in "When to Use" (L16), which is where the substantive de-collide edit landed.
- **`validate-requirements.py` BANNED_TERMS retains generic words** (`function`, `class`, `component`, etc.). This is the original Tier-1 linter's tech-leakage heuristic and is ported faithfully (keep-verbatim); it is intentionally conservative and is a self-check, not the substantive gate.
- Otherwise no deviations from the finalized disposition.

---

**Done-condition (this transform):** disposition applied (port-with-edits × standalone) ✓ · whole HIL dir ported (SKILL.md + 2 references + 1 script) ✓ · 3 files verbatim-IDENTICAL to source + script compiles ✓ · all six wiring edits applied, zero leftover `humaninloop:` / dangling `patterns-*` / `path/to/spec.md` / "acceptance criteria" ✓ · keep-distinct (not factored) + own-script kept ✓ · every responsibility tagged, zero drops ✓ · APPLY-only (not self-graded) ✓.
**Next:** Phase 4 — `verify-output` (independent grade by `mochiko:validator`).
