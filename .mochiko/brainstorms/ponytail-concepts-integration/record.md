# Ponytail concepts → mochiko agents — Decision Record

**Status:** accepted (2026-08-05) — pair cold review complete, 15 merged survivors 15/15 dispositioned (4 user-ruled at review), folds applied, user accepted. Run declaration: lead-run inline questioning (`mochiko:analysis-iterative`), fact-checker seat filled, sized pair review. Departure recorded: cross-exam exchange skipped — the two lenses agreed everywhere they overlapped (accessibility gap found independently by both; citations sampled clean both sides), nothing to examine.

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
3. **"Lazy, not negligent"** — per README: "trust-boundary validation, data-loss handling, security, and accessibility are never on the chopping block." Small because necessary, not golfed. *(Paraphrase corrected at review — earlier "error handling" was memory-asserted drift from the README's "data-loss handling".)*
4. **Modes** — `lite` / `full` (default) / `ultra` / `off`.
5. **Companion commands** — `/ponytail-review` (delete-list for a diff), `/ponytail-audit` (whole-repo over-engineering scan), `/ponytail-debt` (deferred-shortcut ledger), `/ponytail-gain` (impact scoreboard).
6. **Claimed results** — ~54% less code, ~20% cheaper, ~27% faster, 100% safety retention on a FastAPI+React benchmark; beat both a terse-prose control ("caveman") and a bare YAGNI prompt **on code volume and safety retention** — the bare YAGNI prompt was marginally cheaper (−21%) and faster (−30%) but dropped a safety guard (95% retention). Sample caveat carried from source: "~54% is the mean across 12 feature tasks (Haiku 4.5, n=4)".
   `verified: github.com/dietrichgebert/ponytail README, live-fetched 2026-08-05 (twice at review, both lenses)`
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

**F6.** `skills/executing-tdd-cycle/SKILL.md` green phase, `:73-75`: "Write the minimum code to make the failing test pass ... Do not add features, abstractions, or optimizations the card did not require". Refactor phase `:85` *(review erratum: was cited `:84`, which is "Do NOT refactor code from previous cycles")*: "Do NOT add abstractions 'for the future'". Red-flags list `:136` explicitly stops on "I'll add this helper/utility that will be useful later". This is the operational home of "simplest code that passes" — but it is triggered *inside* an already-decomposed task, red-green-refactor. There is **no pre-code check** ("should this code exist at all / can I reuse / is stdlib enough") before the failing test is written.

**F7.** `skills/executing-tdd-cycle/SKILL.md:57` decompose step: "Scope discipline: decompose exactly what the card's acceptance criteria require — nothing the card didn't ask for." Plus `:156` Common Mistakes "Decomposing beyond the card" and `references/TDD-ANTI-RATIONALIZATION.md` table. Minimalism here = don't-exceed-the-card, not choose-the-cheapest-rung.

**F8.** `skills/patterns-vertical-tdd/SKILL.md` — design-time cycle structuring; minimalism only as `:92` "Dependencies minimal and explicit" (cycle deps) and `references/SLICE-IDENTIFICATION.md:191` "Premature Generalization" (slicing anti-pattern). Nothing about how much code a cycle should contain.

**F9. Collision/duplication read:** a ponytail-style pre-code ladder would **not** duplicate the green-phase "minimum" rule (F6) — that fires after the decision to write is made; the ladder fires before. It **would partially overlap** F7 (scope discipline) and F2 (extension-over-invention). The one genuine collision risk: rungs "reuse-existing" and "one-line/minimum-that-works" restate F6+F7 in different words; rungs "YAGNI-skip / stdlib / native-platform / installed-dependency" are net-new to the code layer.

### 3. Constitution / governance surface

**F10.** `templates/constitution-modules/` contents: `evolution-notes.md`, `knowledge-management.md`, `layer-rules.md`, `release-gates.md`. None is a simplicity/YAGNI module.

**F11.** The catalog (`skills/authoring-constitution/references/catalog/`) has exactly three files: `README.md`, `universal-floor.md`, `backend-service.md`. Universal Floor's four cards are FLOOR-SEC, FLOOR-TEST, FLOOR-ERR, FLOOR-OBS (`universal-floor.md:23,36,50,62`) — security, testing, error-handling, observability. **No simplicity / YAGNI / minimalism floor card exists.** These four map directly onto ponytail's "lazy, not negligent" non-negotiables (security, error handling) — i.e. the trust-boundary floor ponytail says never to skip already IS the mochiko Essential Floor.

**F12.** `backend-service.md:106` BE-DEP "Dependency Discipline" is the closest existing principle to ponytail's dependency rung: "External dependencies MUST be justified, minimal, and isolated. Every dependency is a liability". Red-flags `:121` include "reasonably implementable in-house at <100 lines". BUT it is **arbitrated, not floor** (`:109` "architecture-opinion — recommended for services; the user keeps / tightens / drops") and backend-only (type tags "backend, service, fullstack-api"). `validation-constitution/references/ANTI-PATTERNS.md:11` also lists "Over-engineering | 50 principles for a 3-person team" — but that's about constitution size, not code.

**F13.** `references/DOMAIN-DEPENDENCIES.md` is real dependency-allowlist machinery. Trust-signal concept, one line (`:27-36`): a top-down trust-signal hierarchy — Official curation → semi-official stewardship → credible community curation → quantitative proxies — where each admitted domain library must "cite the level each seed rests on," gated by two admissibility criteria (domain-relevance + >80% ubiquity, `:15-19`). Growth gate `:63-68`: a new dep "surfaces to the human as an explicit ruling BEFORE entering the registry; the cycle checkpoint MUST NOT auto-approve while `domain_deps_added` is non-empty." This is ponytail's "installed-dependency" rung already built — but scoped to the *domain layer under a layered architecture only* (`:11`, fires only when `layer-rules` module adopted), not a general pre-code dependency rung.

### 4. Review / grading surface

**F14. Absence (the load-bearing gap):** no review-* or validation-* skill grades **produced code**. Inventory: `review-brainstorm` (records), `review-governance-intent` (setup synthesis), `review-feasibility` (cross-artifact buildability of plan artifacts), `review-plan-artifacts` (plan completeness), `review-specifications` (spec gaps), `validation-constitution` (constitution), `testing-governance-injection` (governance-injection probe; added at review — omission didn't change the absence conclusion). `testing-end-user` verifies runtime behavior against real infra, not code quality. No reviewer hunts over-engineering in code.

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

**D2 — Review lens rides the qa-engineer seat; punt reversal is narrow (lens-only).** `Contested` (user-ruled over the lead's separate-reviewer recommendation)
The over-engineering lens lands on the existing qa-engineer seat, not a new reviewer. The seat gains a second craft: a static diff-read of produced code against the ladder (shouldn't-exist / reuse / stdlib / native-platform / dependency / one-line / minimum). Persona gains a code-minimalism judgment section (widening identity from "verification = execution" to "execution + code-shape audit"); the procedure rides a new review skill alongside `testing-end-user`. The F15 punt reversal is **narrow**: general code review (naming, patterns, correctness beyond tests) stays punted; the punt lines are amended to carve out the minimalism lens, not deleted.
*Rationale:* seat already exists in the implement loop and reads cycle output — no new agent, no roster ripple. Lead countered once (runtime-evidence persona vs static diff-reading craft, dilution risk); user maintained. Context strengthening the fit: the v0.49.0 plan change removed per-task pre-approval — the builder decomposes at build time and no independent seat reads produced code, so the over-engineering surface grew exactly where this lens sits.

**D3 — Lens fires per-cycle, inside the existing verification step.** `Confident` (recommendation adopted)
qa-engineer reads each cycle's diff plus the disclosed decomposition (`cycle-report.md`) during the per-cycle verification it already runs. No separate final-stage ladder pass.
*Rationale:* cycle diffs are small, context fresh, rework cheapest at cycle close; final-stage discovery contradicts why per-cycle verification exists. A light+full two-layer variant (C) rejected as marginal machinery.

**D4 — Ladder procedure single-sourced in a new `patterns-code-minimalism` skill.** `Confident` (recommendation adopted)
The ladder's authoritative text lives in a new pattern skill. Staff-engineer declares it (`skills:`); `executing-tdd-cycle` gains one pointer line at the decompose step (`SKILL.md:57`) firing the pre-code check before the red phase; qa-engineer's D2 review skill cites the same file as its grading standard. Ladder judgment (taste) rides the staff-engineer persona per D1(a); this skill carries the procedure.
*Rationale:* D2 made the ladder a two-consumer text (producer discipline + grader standard) — two consumers means a single-source home. Weaving it into `executing-tdd-cycle` would bury the grader's standard inside the producer's procedure. Full duplication (both homes) rejected outright.

**D5 — Safety floor: reference-only, with accessibility named explicitly (amended at review).** `Confident` (user-ruled at review disposition — the original silence-as-confirmation mark was itself a review finding)
No new floor machinery. Ponytail's "lazy, not negligent" set maps onto the existing FLOOR-SEC / FLOOR-TEST / FLOOR-ERR / FLOOR-OBS cards (F11) **except accessibility** — both reviewers independently caught the gap: the README's never-cut set includes accessibility; mochiko's floor has no a11y card (a11y exists only as the jurisdiction-gated compliance module), so a floor-obligations-only reference line would protect nothing there — under production-only doctrine (customer-facing apps), exactly the class where it bites. **As amended:** the `patterns-code-minimalism` skill's floor line reads: no rung ever sacrifices a floor obligation **or accessibility** (named explicitly, pending the frontend shelf) — small because necessary, never golfed.

**D6 — No intensity modes for the code discipline.** `Confident` (recommendation adopted)
The ladder ships one-intensity. No `off/lite/full/ultra` dial, no per-project mode line in the governance region. Per-project variance rides the existing recorded-waiver machinery (PO-D4).
*Rationale:* production-only doctrine asserts one floor with no tier ladder (PO-D2); a code-minimalism dial would re-introduce tiering by the back door. Ponytail's modes serve a general-audience plugin; mochiko's audience already ruled one asserted standard. The F18/F19 mode vocabulary stays prose-only. Re-open condition: dogfood evidence that one-intensity blocks a real project class.

**D7 — Reading-first becomes rung-zero, thin form: obligation in the ladder, procedure pointered.** `Confident` (re-marked at review from `Contested`: that mark means an overruled challenge, and here the challenge won — user initially favored implicit-only (B), lead countered once — rung 2 "already in the codebase?" is unanswerable without having looked, and greenfield-adjacent tasks carry no read obligation under B — user then adopted the lead's A-thin; the contest trail stays in this body)
`patterns-code-minimalism` opens with rung-zero as one obligation line: "trace the real flow of the code being touched before rung 1; brownfield touches ride `mochiko:brownfield-integration`." The obligation lives in the ladder text — so the qa lens can grade it (the disclosed decomposition shows whether the builder read first) and greenfield tasks are covered; the reading *procedure* stays single-sourced in `brownfield-integration`. No duplicated second home (same pointer discipline as D4).
*Streak note:* Q6 was posed steelmanned with no recommendation after a three-adoption streak (Q3–Q5); the user engaged genuinely (picked B, asked for the recommendation, adopted A-thin after the counter).

**D9 — Lens verdict semantics: advisory-to-lead.** `Confident` (user-ruled at review disposition — the missing ruling was itself a blocking finding)
A `minimalism:` finding is advisory: it rides the verification report to the lead's cycle-checkpoint verdict; the lead decides rework-now or carry; a builder-vs-qa rung dispute escalates to the user only at the checkpoint, never as a mid-cycle stop. A finding never fails a cycle the way a TEST gate does.
*Rationale:* matches the existing report→lead-verdict shape (no new gate class); blocking semantics would put every rung disagreement on the human path at per-cycle frequency — the cost D3 chose per-cycle firing to avoid.

**D10 — Rung-2/3/5 grading carries a codebase-read obligation.** `Confident` (user-ruled at review disposition)
The qa lens does not take reuse claims on trust: when grading rungs 2 (reuse), 3 (stdlib), and 5 (installed dependency), qa reads the codebase around the diff (targeted greps for existing helpers/utilities, dependency manifest check) — not diff + disclosure alone.
*Rationale:* diff shows what was written, disclosure shows what the builder *says* they checked; neither can verify "should have reused" (reviewer finding). Trust-the-claim contradicts why a grader exists (axis 5). Side effect: repo-context reading at each cycle also covers the cross-cycle accretion blind spot (cycle 5 duplicating cycle 2's helper reads as a rung-2 violation against the then-current codebase) — D3 needs no separate re-open condition for accretion.

## Seam scan (Explore-mapped, 2026-08-05)

User-requested deep scan of the plan→implement artifact seam (what plan produces, what implement consumes, where the lens attaches). Headline facts, file-cited in the scan report:

- **S1.** Seam is compatible: every implement input traces to a plan output; no format defined in two unacknowledged places (TEST grammar is a stated owner/consumer layering, grammar owner wins on conflict).
- **S2. Orphan artifacts:** `nfrs.md` and `quickstart.md` are plan-produced (`commands/plan.md:15,17-18`) but named nowhere in implement's Design inputs (`commands/implement.md:61-64`), entry gate, or any implement-side skill.
- **S3.** `architecture.md` has no template file — structure bound only by `patterns-system-design` prose (stated design, not a defect).
- **S4. qa has no diff channel:** `testing-end-user` reads only the `**TEST:**` gate text, `## Quality Gates`, and `plan.md` build config; it never reads `cycle-report.md` (stated consumer: the lead only, `CYCLE-REPORT-FORMAT.md:7-8`), never sees `files_created`/`files_modified`/`decomposition`, and has no git-diff input. The verification-report schema (`REPORT-TEMPLATES.md:19-36`) has no field for a code-shape finding.
- **S5.** Cycle card carries no reuse-judgment surface (`Brownfield exposure` = one classification, one path slot) — but D4/D7 put the ladder at decompose time, so disclosure rides `cycle-report.md`'s `decomposition` array, not the card.

**D8 — Lens wiring: `cycle-report.md` becomes a qa input; diff + disclosure both read; verification report gains a minimalism block.** `Confident` (recommendation adopted)
The D2/D3 lens wires as: qa reads the cycle's git diff (what was built) **and** `cycle-report.md` (the builder's disclosed decomposition and ladder claims) — a net-new read edge superseding the report's lead-only consumer line; `CYCLE-REPORT-FORMAT.md`'s consumer statement is amended accordingly. `cycle-report.md`'s schema gains a ladder-disclosure surface (per-task rung note or a `ladder:` field — exact shape a build-time call); the verification-report schema gains a `minimalism:` findings block. `tasks.md` cycle cards are untouched (S5).
*Rationale:* the lens grades code against ladder claims — it needs both the diff and the disclosure. Diff-only grades blind to stated rungs; lead-relay makes doctrine out of relay quality.

## Open threads

- **D6 re-open condition (surfaced from D6 body):** dogfood evidence that one-intensity blocks a real project class → revisit intensity modes.
- **D1(c) re-open condition (surfaced from Build surface):** dogfood evidence of repo-scale over-engineering the per-cycle lens misses, or user pull for debt/gain surfaces → revisit audit tooling.
- **Orphan plan artifacts (S2):** `nfrs.md` and `quickstart.md` have no stated implement-side consumer — a seam defect independent of this session's scope; needs its own disposition (wire in, or rule the null read deliberate). **Logged in BACKLOG (Defects & empirical checks, 2026-08-05) mid-session at the user's instruction.**

## Build surface (enumerated at convergence — build-session input, not rulings)

- `agents/staff-engineer.md` — ladder-judgment section in the persona (D1a); keystone test: no procedure trace.
- `agents/qa-engineer.md` — code-minimalism lens judgment; identity widened "execution + code-shape audit" (D2); declares the new review skill.
- **New skill** `patterns-code-minimalism` — rung-zero (D7, obligation line + `brownfield-integration` pointer) · the seven-rung ladder · floor-reference line (D5). Single source (D4); staff-engineer declares it.
- **New skill** — the qa lens procedure (working name `review-code-minimalism`), citing `patterns-code-minimalism` as its grading standard (D2/D4); axis-1/axis-3 description work.
- `executing-tdd-cycle/SKILL.md` — one pointer line at the decompose step firing the pre-code check (D4).
- `executing-tdd-cycle/references/CYCLE-REPORT-FORMAT.md` — ladder-disclosure surface in the schema; consumer line amended lead-only → lead + qa (D8; supersession, strip entry owed).
- `testing-end-user/references/REPORT-TEMPLATES.md` — verification-report `minimalism:` findings block (D8).
- Punt-line amendments: `review-plan-artifacts/SKILL.md:44` + `review-specifications/SKILL.md:22` — narrow carve-out, lens-only (D2; strip entries owed).
- `commands/implement.md` — verification-seat wiring: the seat's brief names the new review skill + its read inputs (diff, `cycle-report.md`, codebase-read per D10) *(added at review — spawn briefs name skills explicitly, `ARCHITECTURE.md:68-69`; without this line the lens never mounts and D2/D3/D8 are silently dead)*.
- Verification-report `minimalism:` block carries D9 semantics: advisory, lead-arbitrated at checkpoint.
- Ripple: router index · `plugin.json` version bump · `ARCHITECTURE.md` skill counts 26→28 *(reworded at review: plugin.json globs the skills dir, carries no count)*.
- Not built (D1c): audit tooling — repo scan, debt ledger, gain scoreboard. **Disposition: deferred, not rejected** — re-open condition: dogfood evidence of over-engineering the per-cycle lens misses at repo scale, or a user pull for the debt/gain surfaces *(recorded at review — was homeless)*.

## Review

**Form:** pair, user-sized ("before review, pair"). Lens split: decision-quality (`reviewer-decisions`) + record-integrity (`reviewer-integrity`), both `devils-advocate` on `review-brainstorm`, cold from the frozen file, default FAIL. Both live-fetched the ponytail README independently (external-claims duty per ER-D3/D4).

**Departure:** cross-exam exchange skipped — the lenses agreed everywhere they overlapped (the D5 accessibility gap found independently by both; citation samples clean on both sides); nothing to examine. Recorded here per the departure-trail obligation.

**Tally:** 9 + 8 raised → 17 → 15 merged survivors (2 duplicates: accessibility gap; stale status line) → **15/15 dispositioned**. Verdicts both needs-revision; all survivors fold-resolvable in-session.

| # | Finding (lens) | Severity | Disposition |
|---|---|---|---|
| 1 | D5 accessibility gap — README never-cut set includes a11y; floor has no a11y card (both) | Important, blocking | **User-ruled A:** skill floor line names accessibility explicitly, pending frontend shelf → D5 amended |
| 2 | D5 `Confident` on silence (decisions) | Important, blocking | **User confirmed at disposition** → D5 re-marked user-ruled |
| 3 | Lens verdict semantics unruled (decisions) | Important, blocking | **User-ruled A:** advisory-to-lead → new D9 |
| 4 | Rung-2 ungradeable from D8 inputs (decisions) | Important | **User-ruled A:** codebase-read obligation on rungs 2/3/5 → new D10 |
| 5 | Cross-cycle accretion blind spot (decisions) | Important | Covered by D10's codebase-read (accretion reads as rung-2 violation against current codebase); noted in D10 rationale |
| 6 | Build surface missing `commands/implement.md` — lens never mounts (integrity) | Important, blocking | Lead-folded: build-surface line added, traced D3/D8/D10 (omission was oversight, no ruling excluded it) |
| 7 | D7 `Contested` semantics wrong — challenge won, not overruled (integrity) | Important | Lead-folded: re-marked `Confident`, contest trail kept in body |
| 8 | External-claims disclosure absent on Source material (decisions) | Minor | Lead-folded: `verified:` line + n=4 caveat added |
| 9 | "Beat both" overstates benchmark — YAGNI cheaper/faster, dropped guard (decisions) | Minor | Lead-folded: reworded with per-metric honesty |
| 10 | "error handling" vs README "data-loss handling" paraphrase drift (integrity) | Minor | Lead-folded: paraphrase corrected to quotable text |
| 11 | Stale status line (both) | Minor | Lead-folded: status updated |
| 12 | F6 cite `:84` → `:85` (integrity) | Minor | Lead-folded: erratum in place |
| 13 | F14 inventory missing `testing-governance-injection` (integrity) | Minor | Lead-folded: added; absence conclusion unchanged |
| 14 | plugin.json skill-count ripple mis-homed (integrity) | Minor | Lead-folded: ripple line reworded |
| 15 | D6 re-open + D1(c) disposition homeless (decisions #8, integrity #8, merged) | Minor | Lead-folded: both surfaced in Open threads; D1(c) marked deferred-not-rejected |

**Verified clean at review (sampled, both lenses):** F1–F4, F11–F13, F15, F17, F18 verbatim; F5 absence re-grepped zero-hit; S2/S4 re-verified; BACKLOG S2 entry confirmed (`BACKLOG.md` Defects section); D2's v0.49.0 context against DECISIONS.md; ponytail figures against live README ×2. Build-surface trace ran both directions — every line ⇄ ruling closed after fold 6.
