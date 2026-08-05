# Ponytail concepts → mochiko agents — Decision Record

**Status:** open — questioning in progress. Default run declaration: lead-run inline questioning (`mochiko:analysis-iterative`), fact-checker seat filled (reality surface = mochiko plugin primitives), sized cold review at convergence. No departures yet.

**Topic:** how to add the core concepts of [ponytail](https://github.com/dietrichgebert/ponytail) into mochiko agents, and the appropriate way to deliver them.

## Source material (lead-fetched, 2026-08-05)

Ponytail is an MIT-licensed ruleset/plugin for AI coding agents enforcing radical code minimalism ("the laziest senior dev in the room"). Core concepts:

1. **The ladder** — before generating code, stop at the first rung that applies:
   1. Does it need to exist at all? (YAGNI — skip it)
   2. Already in the codebase? Reuse it
   3. Standard library handles it? Use it
   4. Native platform feature? Use it
   5. Installed dependency? Use it
   6. Fits in one line? Write one line
   7. Only then: the minimum that works
2. **Lazy about solutions, not about reading** — the ladder runs *after* understanding the code being touched; trace real flow first.
3. **"Lazy, not negligent"** — validation at trust boundaries, error handling, security, accessibility never sacrificed. Small because necessary, not golfed.
4. **Modes** — `lite` / `full` (default) / `ultra` / `off`.
5. **Companion commands** — `/ponytail-review` (delete-list for a diff), `/ponytail-audit` (whole-repo over-engineering scan), `/ponytail-debt` (deferred-shortcut ledger), `/ponytail-gain` (impact scoreboard).
6. **Claimed results** — ~54% less code, ~20% cheaper, ~27% faster, 100% safety retention on a FastAPI+React benchmark; beat both a terse-prose control ("caveman") and a bare YAGNI prompt (which dropped a safety guard).
7. **Sibling** — "caveman" shrinks agent *prose*; ponytail shrinks what the agent *builds*. Mochiko already carries caveman (CLAUDE.md response style + `templates/output-style.md`).

## Reality map (fact-checker, verbatim)

> Checker-authored section — landed verbatim, lead writes around it, never restates it.

## Fact map — mochiko code-minimalism / ponytail-integration surface

Mapped: `plugins/mochiko/agents/` (staff-engineer, system-architect, technical-analyst, qa-engineer), `skills/executing-tdd-cycle`, `skills/patterns-vertical-tdd`, `skills/review-*` + `testing-*`, `templates/constitution-modules/` + `skills/authoring-constitution/references/catalog/`, `references/DOMAIN-DEPENDENCIES.md`, `templates/output-style.md` + `governance-surfaces-template.md`, and the repo `CLAUDE.md`. Facts below; absence facts flagged.

### 1. Where code-writing judgment lives (personas)

**F1.** `agents/staff-engineer.md:58` — the single strongest existing minimalism line, in Core Identity: "Learned that the simplest implementation that passes the tests is almost always the right one — so you don't over-engineer or add abstractions the task didn't ask for". Reinforced at `:60` ("Watched projects balloon because 'while I'm in here I'll also fix...' — so you implement exactly what the task describes, nothing more") and `:81` (What You Reject: "Adding code the task didn't ask for"). All three are TDD-scope-discipline framed (per-task, don't-gold-plate) — **not** a pre-code decision ladder and **not** reuse-vs-build or dependency judgment.

**F2.** `agents/system-architect.md` carries minimalism at the topology altitude: `:61` "reach for the smallest topology that meets it"; `:107` (What You Embrace) "**The smallest shape that works** — the fewest components and the simplest interactions that meet the need; you add structure only when a requirement or an NFR forces it"; `:99-100` rejects "Inventing a new component where extending an existing one is the honest, smaller change" and "Speculative components built for a future that no requirement asks for". This is component-count minimalism + extension-over-invention — a structural cousin of ponytail's reuse/YAGNI rungs, but scoped to architecture, not line-level code.

**F3.** `agents/technical-analyst.md` has minimalism only obliquely: `:101` "Constraints document real boundaries, not premature design choices"; `:147` brownfield "reuse of existing components". No YAGNI/simplicity line for code.

**F4.** `agents/qa-engineer.md` — no code-writing minimalism (verification persona; `:93`/`:95` "simplicity"/"minimal" refer to test cases and report verbosity, not code).

**F5. Absence:** no persona contains a staged/ranked pre-code decision procedure (YAGNI-skip → reuse → stdlib → native-platform → dep → one-line → minimum). No persona anywhere carries the phrase "reuse before build" as a first-class rule; the closest is F2's architecture-level extension-over-invention and F3's brownfield reuse. No persona names "stdlib first" or "native platform first."

### 2. Where code-writing procedure lives (TDD skills)

**F6.** `skills/executing-tdd-cycle/SKILL.md` green phase, `:73-75`: "Write the minimum code to make the failing test pass ... Do not add features, abstractions, or optimizations the card did not require". Refactor phase `:84`: "Do NOT add abstractions 'for the future'". Red-flags list `:136` explicitly stops on "I'll add this helper/utility that will be useful later". This is the operational home of "simplest code that passes" — but it is triggered *inside* an already-decomposed task, red-green-refactor. There is **no pre-code check** ("should this code exist at all / can I reuse / is stdlib enough") before the failing test is written.

**F7.** `skills/executing-tdd-cycle/SKILL.md:57` decompose step: "Scope discipline: decompose exactly what the card's acceptance criteria require — nothing the card didn't ask for." Plus `:156` Common Mistakes "Decomposing beyond the card" and `references/TDD-ANTI-RATIONALIZATION.md` table. Minimalism here = don't-exceed-the-card, not choose-the-cheapest-rung.

**F8.** `skills/patterns-vertical-tdd/SKILL.md` — design-time cycle structuring; minimalism only as `:92` "Dependencies minimal and explicit" (cycle deps) and `references/SLICE-IDENTIFICATION.md:191` "Premature Generalization" (slicing anti-pattern). Nothing about how much code a cycle should contain.

**F9. Collision/duplication read:** a ponytail-style pre-code ladder would **not** duplicate the green-phase "minimum" rule (F6) — that fires after the decision to write is made; the ladder fires before. It **would partially overlap** F7 (scope discipline) and F2 (extension-over-invention). The one genuine collision risk: rungs "reuse-existing" and "one-line/minimum-that-works" restate F6+F7 in different words; rungs "YAGNI-skip / stdlib / native-platform / installed-dependency" are net-new to the code layer.

### 3. Constitution / governance surface

**F10.** `templates/constitution-modules/` contents: `evolution-notes.md`, `knowledge-management.md`, `layer-rules.md`, `release-gates.md`. None is a simplicity/YAGNI module.

**F11.** The catalog (`skills/authoring-constitution/references/catalog/`) has exactly three files: `README.md`, `universal-floor.md`, `backend-service.md`. Universal Floor's four cards are FLOOR-SEC, FLOOR-TEST, FLOOR-ERR, FLOOR-OBS (`universal-floor.md:23,36,50,62`) — security, testing, error-handling, observability. **No simplicity / YAGNI / minimalism floor card exists.** These four map directly onto ponytail's "lazy, not negligent" non-negotiables (security, error handling) — i.e. the trust-boundary floor ponytail says never to skip already IS the mochiko Essential Floor.

**F12.** `backend-service.md:106` BE-DEP "Dependency Discipline" is the closest existing principle to ponytail's dependency rung: "External dependencies MUST be justified, minimal, and isolated. Every dependency is a liability". Red-flags `:121` include "reasonably implementable in-house at <100 lines". BUT it is **arbitrated, not floor** (`:109` "architecture-opinion — recommended for services; the user keeps / tightens / drops") and backend-only (type tags "backend, service, fullstack-api"). `validation-constitution/references/ANTI-PATTERNS.md:11` also lists "Over-engineering | 50 principles for a 3-person team" — but that's about constitution size, not code.

**F13.** `references/DOMAIN-DEPENDENCIES.md` is real dependency-allowlist machinery. Trust-signal concept, one line (`:27-36`): a top-down trust-signal hierarchy — Official curation → semi-official stewardship → credible community curation → quantitative proxies — where each admitted domain library must "cite the level each seed rests on," gated by two admissibility criteria (domain-relevance + >80% ubiquity, `:15-19`). Growth gate `:63-68`: a new dep "surfaces to the human as an explicit ruling BEFORE entering the registry; the cycle checkpoint MUST NOT auto-approve while `domain_deps_added` is non-empty." This is ponytail's "installed-dependency" rung already built — but scoped to the *domain layer under a layered architecture only* (`:11`, fires only when `layer-rules` module adopted), not a general pre-code dependency rung.

### 4. Review / grading surface

**F14. Absence (the load-bearing gap):** no review-* or validation-* skill grades **produced code**. Inventory: `review-brainstorm` (records), `review-governance-intent` (setup synthesis), `review-feasibility` (cross-artifact buildability of plan artifacts), `review-plan-artifacts` (plan completeness), `review-specifications` (spec gaps), `validation-constitution` (constitution). `testing-end-user` verifies runtime behavior against real infra, not code quality. No reviewer hunts over-engineering in code.

**F15.** mochiko **explicitly punts** code review out of scope: `review-plan-artifacts/SKILL.md:44` "**Implementation code review** — use code-review tooling instead" and `:185` rejects "Commenting on code patterns, variable names, or framework choices"; `review-specifications/SKILL.md:22` "Code review - Different skill domain entirely".

**F16. Absence:** no per-diff audit primitive (nothing that diffs a delete-list against a change) and no whole-repo over-engineering scan primitive exist anywhere in `plugins/mochiko/`. Both ponytail companion commands would be net-new.

### 5. Delivery-vehicle constraints (five axes)

**F17.** From `CLAUDE.md` "Skill-library conventions (five axes)": axis 4 — "persona carries judgment, skill carries procedure; **a persona contains no trace of any workflow** (decoupling by absence, the keystone test)." Every persona file enforces this literally (e.g. `staff-engineer.md:72` "this is the *taste* you bring, not the format spec. The concrete procedure lives in your skills"). Axis 1 — "every skill declares `user-invoked` or `model-invoked`; user-invoked may call model-invoked, never each other." Axis 3 — "model-invoked skills encode graded MUST/SHOULD + exact trigger phrases in their `description` (**delivery truncates at 1,536 chars — measure first**)." Axis 5 — "every reviewable artifact is graded by a structurally independent validator (different agent, different skill)." **Implication for a ponytail ladder:** ladder *judgment* would ride a persona (staff-engineer), ladder *procedure* would ride a skill; a companion review command would need its own independent validator to satisfy axis 5.

### 6. Mode machinery precedent (off/lite/full/ultra)

**F18.** The four-level machinery (`off`/`lite`/`full`/`ultra`) — exactly ponytail's mode set minus renaming — is already fully built as **output-style**, not code-mode. Definition is plugin-shipped at `templates/output-style.md:13-19` (the four levels) with per-surface defaults (chat=`full`, reports=`ultra`, artifacts=`full`, `:23-27`). The **switch is per-project, written by setup**: `output-style.md:76` "The persistent home is the project's `CLAUDE.md` governance region — one line carrying a value per surface"; `governance-surfaces-template.md:60-62` ships the marked block `<!-- mochiko:output-style:begin/end -->` that setup writes into the consumer project's CLAUDE.md default-on once, preserved across regeneration. In-session override `output-style.md:82`: "'stop caveman' or 'normal mode' turns the style off for the rest of that session."

**F19.** Distinct from F18: this repo's *own* `CLAUDE.md` "caveman mode" section (levels `lite`/`full`/`ultra`, default full) is the operating-manual copy for working *in* mochiko — it is not what the plugin ships to consumers (that's F18's output-style). So there are two independent implementations of the same four-value level vocabulary in the tree: plugin-shipped output-style (for consumer projects, per-surface) and repo-local caveman (for this repo's authors). Either is a live precedent for a plugin-shipped, per-project, setup-written mode line.

### 7. Cuts-either-way summary

- **Toward redundancy:** F1 (green-phase "simplest that passes"), F2 (smallest-topology + extension-over-invention), F6-F7 (don't-exceed-the-card), F11 (Essential Floor == ponytail's "lazy not negligent" non-negotiables), F12/F13 (dependency discipline + allowlist already exist). Ponytail's floor and its cheapest two rungs are substantially present.
- **Toward a gap:** F5 (no ranked pre-code ladder; no "reuse before build" / "stdlib first" / "native-platform first" anywhere), F9 (four ladder rungs net-new to the code layer), F14-F16 (no code-quality reviewer, no diff-audit, no repo over-engineering scan — all three ponytail companions are net-new), F13-scope (dependency gate is domain-layer-only, not general).
- **Precedent in hand:** F17 (persona/skill decoupling gives a clean home for ladder judgment vs procedure), F18-F19 (the exact four-level mode machinery already ships two ways).

## Decisions

**D1 — Scope: generation-time ladder + review-time lens; companion audit tooling not taken.** `Confident` (user-ruled)
Mochiko absorbs ponytail at two layers: **(a)** the generation-time discipline — the pre-code ladder riding the staff-engineer persona (judgment) and the TDD procedure surface (F5 gap: no ranked pre-code check exists; four rungs net-new per F9), and **(b)** the review-time over-engineering lens on produced code (F14: no reviewer grades produced code today). Layer **(c)** — standing audit tooling (repo-wide over-engineering scan, debt ledger, gain scoreboard; all net-new per F16) — is not taken this session.
*Rationale:* (a) fills the mapped gap at lowest collision cost; (b) is its enforcement half — a producer-side discipline with no grader contradicts axis 5 (F17). (c) is heaviest, all net-new command surface, and separable.
*Note:* (b) entails revisiting the explicit code-review punt at F15 (`review-plan-artifacts/SKILL.md:44`, `review-specifications/SKILL.md:22`) — scope of that reversal is its own ruling, below.

## Open threads

## Review

*(sized at convergence)*
