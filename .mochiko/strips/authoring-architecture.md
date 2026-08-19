# Strip notes — `skills/authoring-architecture/`

Entry formats: `strips/README.md`. Shipped at v0.55.0 (architecture-doc-layer wave); first strip
entry at v0.64.0 (guardrails-vs-detail Wave 2).

## [v0.81.0] RETIRED — the skill is deleted; its duties pass to `authoring-architecture-store`

**The primitive `plugins/mochiko/skills/authoring-architecture/` no longer exists.** This is a
whole-skill supersession by ruling, recorded in the whole-file form set by the
`agents/principal-architect` [v0.67.0] precedent: the entire retired file is quoted verbatim below,
followed by the duty-by-duty inheritance map, so every obligation it carried can be traced to its
new home by the audit's preserved-responsibilities check.

- **Disposition:** superseded → `mochiko:authoring-architecture-store` (the store owner: AX/spine
  grammar, element lifecycle, graduation, fold-at-landing, health view) for Duty 1 and the write
  discipline; store status flips + derived-index regeneration for Duty 2; the D10 orphan rule for
  the In-flight pointer list.
- **Tier failed:** n/a — supersession by ruling (record
  `.mochiko/brainstorms/product-architecture-schema/record.md` — **D7** "new
  `authoring-architecture-store` … (retires `authoring-architecture`)" · **D3** one
  schema-backed store, the per-feature artifact dies · **D4** repo-root `ARCHITECTURE.md` becomes a
  **derived projection**, regenerated on every store write (review fold S12: derived, never
  hand-maintained; single writer) · **D10** six-step delta lifecycle + the orphan rule, and review
  fold **S6** inheriting the landing diff; `DECISIONS.md` 2026-08-19).
- **Rationale in one line:** the skill's whole subject — a hand-maintained current-state prose doc,
  folded post-hoc, carrying pointers to per-feature architecture artifacts — is exactly the
  five-surface split D3 collapses. Its *duties* are real and survive; its *artifact* does not.

### Content — the retired file, verbatim and whole

````markdown
---
name: authoring-architecture
description: This skill MUST be invoked when authoring or updating `ARCHITECTURE.md` at a plan/implement landing that changed structure, and to run the **landing diff** (built topology vs the approved architecture delta) whenever an approved delta existed. Owns the In-flight pointer list. SHOULD also invoke on 'update the architecture doc', 'architecture drift', or 'built vs approved architecture'. Current state only; distinct from mochiko:patterns-system-design. No structural change → no update.
---

# Authoring Architecture

## Overview

Keep `ARCHITECTURE.md` a current-state map of the system a maintainer can read in one sitting —
what exists now, not how it got here. Dispatched at a landing, this skill carries **two distinct
duties on two distinct triggers** — do not conflate them.

## Two duties at a landing

| Duty | Fires when | Output |
|------|-----------|--------|
| **Landing diff** | an **approved architecture delta existed** for the feature — independent of what was built | a built-vs-approved topology report: "built as approved", or the named divergence |
| **`ARCHITECTURE.md` fold** | the built work **changed structure** (components, boundaries, data flow, cross-component contracts) | the updated current-state map |

The triggers are deliberately different. An approved delta that was **descoped or silently dropped**
built nothing structural, so the fold does not fire — but the **diff does**, and it is the only thing
that catches the missing change. A landing with no prior approved architecture fires neither. Run
each duty on its own trigger; **never gate the diff on whether structure was built.**

## Duty 1 — the landing diff

When an approved `architecture.md` delta existed for this feature, run this **first**:

1. Read the approved `architecture.md` (the target signed off at design time) and the built reality
   (the code plus the feature's landed artifacts).
2. Compare box-by-box and arrow-by-arrow: was each approved add / remove / redirect / boundary-move
   actually built? Was anything structural built that the approved target did not carry?
3. Report **built-as-approved** or the **divergence** — each difference named (component or
   interaction, approved vs built). The diff **reports**; it does not silently reconcile.

This is a **built-vs-approved topology diff** taking the approved artifact as input — a capability
this duty owns, not an assumed pre-existing one. It runs whether or not the fold (Duty 2) fires.

## Duty 2 — the `ARCHITECTURE.md` fold

**Fires when** the landing's work changed **components, boundaries, data flow, or cross-component
contracts**. Internal refactors, cosmetic moves, and feature work inside an existing component do
not fire it. **No structural change → no update.**

1. Read the current `ARCHITECTURE.md` — update in place, never wholesale rewrite.
2. Reflect the change where it lives: **Components** (name — responsibility — boundary),
   **Data flow**, **External integrations**. Add, retire, or redraw only what changed.
3. Present tense, current system only. No history narration, no rationale prose — link the
   `DECISIONS.md` row that ruled the change.
4. Keep it one read: a component earns a line, not a chapter; depth lives in the feature
   artifacts.

**Feature-scoped landing → two targets, each on its own trigger.** The feature-root
`architecture.md` (`.mochiko/features/FEAT-XXX/architecture.md`) accumulates the feature's
**approved** delta — firing on **approved-delta-existed** (Duty 1's trigger shape),
independent of whether structure was built: it is the **accumulated feature target**, not
built state, so a descoped or partially built feature's approved delta still lands, and
later landings' deltas seed from it (never from scratch). Repo `ARCHITECTURE.md` keeps
Duty 2's **built-change** trigger and takes only the built change, staying
current-shipped-state per branch. Same in-place-update discipline for both; the
"no structural change → no update" rule above scopes to repo `ARCHITECTURE.md` only. A
non-feature-scoped landing folds into repo `ARCHITECTURE.md` alone.

## The In-flight pointer list

Repo `ARCHITECTURE.md` carries a thin **In flight** list making in-progress structural work
visible from the current branch without duplicating topology — one line per active feature:

```markdown
## In flight
- FEAT-XXX → .mochiko/features/FEAT-XXX/architecture.md
```

- **Added** when plan's architecture sign-off lands (the feature-root target now exists).
- **Removed** at feature close — before removal, the feature-close diff (shipped code vs the
  accumulated feature-root target) is the parked hook; until a feature-close workflow owns
  it, removal alone applies.
- The section is created lazily by the first pointer write — never scaffolded empty.
- Pointer lines only, never topology; the linked feature-root file carries the content.

## Quality checks

- **Diff (Duty 1):** every approved add / remove / redirect / boundary-move is accounted for as built or diverged; nothing structural was built unreported. The diff ran because an approved delta existed, not because structure was built.
- **Dual-target (feature-scoped):** the feature-root `architecture.md` accumulated the approved delta (fires on approved-delta-existed, even when nothing was built); repo `ARCHITECTURE.md` took the built change only where structure was built.
- **In-flight list:** every pointer targets an open feature and resolves; the feature's sign-off added its line, its close removed it; no topology in the list.
- Every named component exists in the code; every pointer resolves.
- No past-tense narration; no rationale restated from the decisions layer.
- A reader new to the repo can place any file in a component from this doc alone.
````

### Inheritance map — where each duty went

**Duty 1 (the landing diff) → `authoring-architecture-store`, trigger shape unchanged.** Per D10
review-fold S6: "the store skill inherits Duty 1's approved-delta-existed trigger: at landing,
built-vs-signed is diffed even when nothing was built (descoped features caught at the landing, not
weeks later by the orphan sweep)". The load-bearing lines that MUST be findable in the store skill,
quoted here as the audit's reference:

- The trigger row: "| **Landing diff** | an **approved architecture delta existed** for the feature
  — independent of what was built | a built-vs-approved topology report: \"built as approved\", or
  the named divergence |"
- The anti-conflation floor: "Run each duty on its own trigger; **never gate the diff on whether
  structure was built.**"
- The descope-catching rationale: "An approved delta that was **descoped or silently dropped** built
  nothing structural, so the fold does not fire — but the **diff does**, and it is the only thing
  that catches the missing change."
- The reports-never-reconciles rule: "The diff **reports**; it does not silently reconcile."
- The owned-capability assertion: "This is a **built-vs-approved topology diff** taking the approved
  artifact as input — a capability this duty owns, not an assumed pre-existing one."

Re-keyed by ruling within the inheritance: the diff's *input* is no longer "the approved
`architecture.md`" (D3 kills it) but the signed delta as it stands in the store; and per D10/D11 the
`As-built:` and `Drift:` writes the diff feeds are **judgment, graded** (S6 + verify N1 struck them
from the mechanical list).

**Duty 2 (the `ARCHITECTURE.md` fold) → replaced, not inherited.** D3 puts intent and built-state on
one surface, so there is no post-hoc fold of a separate prose doc: a landing **flips element
statuses** to `built` and updates the touched rows (D10 step 5), and the repo-root `ARCHITECTURE.md`
is **regenerated as a derived index on every store write** (D4 + fold S12 — derived, never
hand-maintained, single writer; index-vs-ledger disagreement is a defect fixed on sight). The
step-2 section list (**Components** / **Data flow** / **External integrations**) dies with the prose
doc: those sections were prose-named, never schema-prescribed (ground fact F2), and the store's
spine + concern layers replace them.

**The In-flight pointer list → replaced by the D10 orphan rule.** The whole section dies: the
`## In flight` heading, the pointer line form, and all four rules (added at sign-off · removed at
close, with the parked feature-close-diff hook · created lazily, never scaffolded empty · pointer
lines only, never topology). D10's replacement: "every in-flight-class element keys an open feature;
orphans flagged by the health view, cleaned at desk visits." The pointer list made in-progress work
visible by *duplicating a pointer*; the store makes it visible by *carrying the status on the
element itself*. **Cross-note:** this also discharges the skill-side half of the pinned **AT-D6-C
In-flight-agreement invariant** at `.mochiko/memory/knowledge-management.md`; the KM-side
supersession is a separate, project-pinned landing and is **not** P3's (Build surface,
"Supersessions owed").

**`:51` — the inverted line.** Verbatim: "Keep it one read: a component earns a line, not a chapter;
depth lives in the feature artifacts." **Inverted by D3/D4.** Ground fact F6 names this line
explicitly as "itself a shipped depth-allocation rule that D3/D4 inverts". Depth now lives in the
**store** — the spine deep view and the concern deep-files that rows graduate into — while the
one-read surface is the derived root index (spine thumbnail + AX summary table + links). The
feature artifacts it pointed at are gone (D3). The *one-read value* survives, re-homed: D4 keeps the
root scannable "on user requirement: index/table-of-contents, depth one hop down."

**In-place-update discipline → survives as store-write discipline.** Verbatim: "Read the current
`ARCHITECTURE.md` — update in place, never wholesale rewrite," and the dual-target paragraph's "Same
in-place-update discipline for both." This survives in two forms and is **narrowed by ruling**: the
store's ruled truth is never edited in place *by a plan run* (D12's recorded amendment to the
`plan.md` "baselines never edited in place" floor — plan-time store writes are legal only as
`in-flight`-status deltas at user sign-off), while desk-time amendments update rows in place rather
than rewriting the store. The derived root index is the one surface that IS wholesale regenerated —
a deliberate reversal, because it is derived (D4/S12), not authored.

**Also surviving, re-homed (named so the audit can trace them):** the current-state-only rule and
"No past-tense narration; no rationale restated from the decisions layer" (the store's rows carry
`Ruling:`/`Rationale:` fields, so rationale gains a *home* rather than a ban — the ban on restating
it on the topology surface stands); "Every named component exists in the code; every pointer
resolves" (now the scoped **drift probe**'s empirical job per D7 fold S10 — graded against actual
code, evidence not memory, rather than asserted at fold time); and "A reader new to the repo can
place any file in a component from this doc alone" (the derived root index's reason for existing).

### Inheritance VERIFIED in the heir (post-V3 fix round)

Every obligation named above was re-checked against the landed heir
`plugins/mochiko/skills/authoring-architecture-store/SKILL.md` after P1's V3 fix round, by
whitespace-normalized exact-substring match rather than by eye. **All eleven resolve:**

| Obligation (from the map above) | Heir location |
|---|---|
| Approved-delta-existed trigger row, "independent of what was built" | `:120` |
| Two-triggers-never-conflated framing | `:116` (section heading) |
| Anti-conflation floor — "Never gate the diff on whether structure was built." | `:125-126` |
| Descope/silently-dropped rationale, with the orphan-sweep timing added | `:123-125` |
| Diff runs **both directions** (signed-but-unbuilt *and* built-but-unsigned) | `:128-129` |
| Owned-capability assertion — "not an assumed pre-existing one" | `:130-131` |
| Reports-never-reconciles | `:131-132` (+ `:145` for the drift probe) |
| "nothing structural was built unreported" quality check | `:186` |
| Current state, present tense, no history narration | `:65`, `:187` |
| No rationale restated from a decision record | `:187` |
| Readability bar — a new reader can place any component | `:105`, `:188` |

Two obligations are **inherited in a changed form, by ruling, not weakened**: "Every named component
exists in the code" is now the **scoped drift probe**'s empirical job (`:137-145`) rather than a
fold-time assertion, per D7 fold S10; and "every pointer resolves" is now id-and-pointer integrity
across `Work:`/NFR/handled-elsewhere pointers (`:181`), the store's pointer classes replacing the
retired In-flight list's.

*Method note:* an earlier line-oriented `grep` for the anti-conflation floor returned nothing and
briefly read as a gap — the sentence wraps across `:125-126` ("**Never\ngate the diff…**"), so a
single-line pattern cannot match it. The table above was produced by normalizing whitespace across
the whole heir file and testing exact substrings, which is the method any re-check of this entry
should use.

### Kept deliberately

Nothing of this skill remains in the plugin tree — the directory is deleted. Every duty above is
named with its new home so the preserved-responsibilities check can find it; a duty that cannot be
found in `authoring-architecture-store` is a regression, not a silent retirement. The two prior
entries in this file ([v0.80.0] dual-target re-word, [v0.64.0] slim description) remain valid
history for the primitive they described.

### Consumers assessed

Live tree only (`.claude/worktrees/` copies excluded). Eight files reference the retired skill; two
were P3's and are re-pointed in this same wave, six are **dead pointers owned by other seats** —
flagged, deliberately untouched per the lead's build ruling:

| Consumer | Line | Owner |
|---|---|---|
| `skills/patterns-system-design/SKILL.md` | `:3` description + `:18` When-NOT bullet | **P3 — re-pointed this wave** at `authoring-architecture-store` |
| `agents/principal-architect.md` | `:15` `skills:` + `:30` Skills-Available bullet | **P3 — re-pointed this wave** |
| `commands/implement.md` | `:188` | P2 |
| `commands/plan.md` | `:190` (In-flight pointer write) | P2 |
| `templates/constitution-modules/knowledge-management.md` | `:45` doc-role row | P4 |
| `skills/authoring-feature-map/SKILL.md` | `:109` peer-of line | P4 |
| `skills/mochiko/SKILL.md` | router rows | P4 |
| `skills/patterns-sound-loop/SKILL.md` | governing-surface row | P4 |

**Ledger (P4's edit, declared here):** the two `authoring-architecture` budget rows retire with the
primitive — skill body 5,250 / budget 6,563 and skill description 488 / budget 610, both in
`.mochiko/memory/primitive-cost-budgets.md`. The v0.64.0 note "`authoring-architecture` and
`grooming-operating-docs` were audited Wave 2 body no-ops" keeps its second subject and needs its
first removed.

**Final measured state before deletion:** body 5,255 chars, description 488 chars.

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
