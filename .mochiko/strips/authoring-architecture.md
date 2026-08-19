# Strip notes — `skills/authoring-architecture/`

Entry formats: `strips/README.md`. Shipped at v0.55.0 (architecture-doc-layer wave); first strip
entry at v0.64.0 (guardrails-vs-detail Wave 2).

## [v0.80.0] Dual-target paragraph re-worded feature-scoped; stale feature-artifact path corrected — slice-vocabulary purge

- **Disposition:** superseded → the same paragraph, checklist line, and In-flight pointer
  example, re-worded on the feature as the scoping unit and re-pointed at the current
  per-feature artifact home `.mochiko/features/FEAT-XXX/`.
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/decisions/2026-08-19-slice-vocabulary-purge.md`).
- **Content (verbatim, the three superseded spans):**

  ```
  **Slice-scoped landing → two targets, each on its own trigger.** The feature-root
  `architecture.md` (`.mochiko/specs/<feature>/architecture.md`) accumulates the slice's
  **approved** delta — firing on **approved-delta-existed** (Duty 1's trigger shape),
  independent of whether structure was built: it is the **accumulated feature target**, not
  built state, so a descoped or partially built slice's approved delta still lands, and later
  slices' deltas seed from it (never per-slice from scratch). Repo `ARCHITECTURE.md` keeps
  Duty 2's **built-change** trigger and takes only the built change, staying
  current-shipped-state per branch. Same in-place-update discipline for both; the
  "no structural change → no update" rule above scopes to repo `ARCHITECTURE.md` only. A
  non-slice-scoped landing folds into repo `ARCHITECTURE.md` alone.
  ```

  ```
  - <feature> → .mochiko/specs/<feature>/architecture.md
  ```

  ```
  - **Dual-target (slice-scoped):** the feature-root
  ```

  Replaced by, respectively: the same paragraph with "Feature-scoped landing", "the feature's
  approved delta", "a descoped or partially built feature's approved delta", "later landings'
  deltas seed from it (never from scratch)", "A non-feature-scoped landing folds into repo
  `ARCHITECTURE.md` alone", and the corrected path; the pointer example
  `- FEAT-XXX → .mochiko/features/FEAT-XXX/architecture.md`; and
  `- **Dual-target (feature-scoped):** the feature-root`.

- **Kept deliberately:** every mechanic the paragraph carries. Both triggers survive intact —
  the feature-root target fires on **approved-delta-existed** (Duty 1's shape) and the repo
  doc on **built-change** (Duty 2's); so do the accumulated-feature-target framing, the
  descoped/partially-built case, the seed-from-the-accumulated-target rule, the
  current-shipped-state-per-branch clause, the shared in-place-update discipline, and the
  scoping of "no structural change → no update" to repo `ARCHITECTURE.md` alone. The checklist
  line keeps its full assertion (feature-root accumulated the approved delta even when nothing
  was built; repo doc took the built change only where structure was built). The In-flight
  section's four rules are untouched; only its example line's path and label changed.
- **Path evidence:** `.mochiko/specs/<feature>/architecture.md` appeared only here — the
  repo-wide grep for that path form returns this file's two occurrences and nothing else.
  `commands/plan.md` is the authority and states package artifacts "land at
  `.mochiko/features/FEAT-XXX/`", naming `architecture.md` first among them; the router's
  pipeline note and `skills/mochiko/SKILL.md` agree ("each capability's deltas and run
  artifacts live at `.mochiko/features/FEAT-XXX/`"). `patterns-system-design` says only "the
  feature's spec dir" — vague, contradicting no literal path. Evidence is one-sided, so the
  path was corrected rather than flagged.
- **Consumers assessed:** the paragraph is authoring guidance with no downstream reader. The
  In-flight pointer example is the shape written into repo `ARCHITECTURE.md`; existing pointers
  in a consuming project keep resolving — this corrects the shape future writes follow.
- **Char budget:** body 5,250 → 5,255 (+5) against the 6,563 budget — 1,308 chars of headroom
  remain. Description unchanged at 488 against its 610 budget.

## [v0.64.0] Slim description (guardrails-vs-detail Wave 2 editorial cut) — body no-op
- **Disposition:** superseded → Wave 2 editorial guardrails cut (D4 cut line). Description only;
  the body carries no D4-class content and is unchanged.
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail Wave 2, `DECISIONS.md`
  2026-08-11 build row Wave 2 residual + user rulings 2026-08-10/11; method warrant: benchmark
  verdict `.mochiko/benchmarks/guardrails-vs-detail/report/final-verdict.md`).
- **Content (faithfully compressed):** description 1,511 → 488 chars (−68%). **Body 5,250 →
  5,250 (0%) — deliberate no-op.** The body has no "When to Use" section restating the
  description and no process walkthrough whose obligations live in a separate checklist: the
  two-duty split, the Duty 1 landing-diff steps, the Duty 2 fold steps, the slice-scoped
  dual-target rule, the In-flight pointer-list mechanics, and the Quality checks are all owned
  obligations that survive nowhere else, so there is nothing to delete (a no-op body is the
  correct D4 outcome for this skill). Description cut: the dual-target-fold explanation, the
  slice-scoped accumulation detail, the decisions-layer / feature-scope-artifact boundary prose,
  and several SHOULD trigger phrases compressed to the MUST clause + the landing-diff trigger +
  core SHOULD triggers + the `patterns-system-design` sibling distinction + the
  no-structural-change rule. Verbatim homes: git history of this file (pre-v0.64.0).
- **Old description (verbatim):**
  > This skill MUST be invoked when authoring or updating `ARCHITECTURE.md` — the knowledge-management module's living system view (components, boundaries, data flow, external integrations) — at a plan or implement landing whose work changed the system's structure. MUST also run the **landing diff** when dispatched at a landing where an approved architecture delta existed for the feature — diffing the built topology against the approved target and reporting built-vs-approved divergence — which fires on approved-delta-existed even when nothing structural was built, distinct from the `ARCHITECTURE.md` fold (which fires only on built structural change). On a slice-scoped landing the fold is dual-target: the feature-root `architecture.md` accumulates the slice's approved delta (even unbuilt) AND repo `ARCHITECTURE.md` takes the built change. Also owns the repo doc's **In-flight pointer list** (entry added at plan's architecture sign-off, removed at feature close). SHOULD also invoke on "update the architecture doc", "system view", "architecture drift", "built vs approved architecture", or "does ARCHITECTURE.md still match the code". Records the RESULTING system, present tense, current state only — decision rationale lives in the decisions layer (`DECISIONS.md` + `.mochiko/decisions/`, technique in mochiko:patterns-technical-decisions) and is linked, never restated; feature-scope design artifacts (data-model.md, contracts/) stay in their specs. No structural change → no `ARCHITECTURE.md` update.
- **Kept deliberately:** the entire body — the Overview, the two-duty table, the Duty 1 (landing
  diff) and Duty 2 (`ARCHITECTURE.md` fold) step lists, the slice-scoped dual-target rule, the
  In-flight pointer-list section, and the Quality checks. The description keeps the two MUST
  duties (fold + landing diff), the In-flight-list ownership, the current-state-only rule, and
  the `patterns-system-design` sibling distinction.
- **KEPT reconciliation:** no prior strip file existed and no `DECISIONS.md`-traceable body line
  was removed (body unchanged), so there is no protected content to reconcile.
- **Consumers assessed:** principal-architect (mounts it) · plan, implement (bind it) ·
  authoring-feature-map, patterns-system-design (cross-reference the doc) · mochiko router.
  None links a description clause or a body section anchor; the cut removed no section and no
  invariant. Contract intact.
