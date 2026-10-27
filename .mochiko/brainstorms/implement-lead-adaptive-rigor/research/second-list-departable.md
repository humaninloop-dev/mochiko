# Second list — the departable set

- **Seat:** `second-list-departable` · **Date:** 2026-10-27 · **Model:** opus
- **Inputs received:** the session frame and the one question, nothing else.
- **Sources read:** the rendered implement rules (`mochiko-cli rules implement`, plugin 0.122.0, binary 0.4.0), `.mochiko/strips/implement.md`, `DECISIONS.md`, and two prior records (`author-grader-consolidation`, `production-floor-adaptive-depth`).
- **Disclosure:** no fence exposure. The session directory was seen only as a name in a directory listing, and `DECISIONS.md` carries no row for it. No Explore dispatch was made. The locate reads were a few inline greps, a small departure from the `patterns-model-tiering` routing, stated here.

## Reading of "low-stakes"

Low-stakes as the frame defines it: nothing can lose money, corrupt data or breach security, and a revert undoes the change. Two things a code revert does **not** undo, so they never get cheaper with stakes:
- **a shared record** written outside the product code, such as the baselines, the store, `gates.md` or the map;
- **a user's ruling** that was taken from them, or a "verified" claim that was never earned.

Everything below follows from that split.

## The steps

Verdicts: **SKIP** = may drop, with the skip stated. **THIN** = run smaller, fold, or read a delta. **NEVER** = must not move.

| Step (rule id) | Added to prevent | Still caught if moved on a low-stakes run | Verdict and why |
|---|---|---|---|
| Per-cycle verification grade (`impl.seat-verification-independence`, `impl.craft-verify-bindings`) | A builder clearing its own cycle, and evidence claimed rather than run (`impl.fail.no-evidence`). Plan-stage retirement row, `DECISIONS.md:62` | The final validation re-runs every **TEST:** gate and the full suite. A defect is caught later, at a higher rework cost, but it is still caught. | **THIN.** One non-author grade may cover several small cycles at once, and a `sonnet` worker may run the gates with the seat reading them back. The grade can never become the builder's report (`impl.author-grader-default-fail`). |
| Per-cycle code-minimalism lens (`review-code-minimalism`) | Over-built code, and reuse claimed on trust. Findings are advisory at any severity (`impl.minimalism-advisory`). | Nothing else catches it, but it never blocks anything, so a skip loses no gate. | **SKIP**, stated. |
| Seat-plan peer grade (`impl.plan-approval-producers`) | Seats writing before approval. The probe found that seats under a plan-mode lead write freely, so producer-plan-enforcement D6 ruled "no size carve" (`DECISIONS.md:53`). | The non-author grade of the produced artifact, which is sound-loop leg 2. | **THIN.** One plan grade per seat per run, not one per dispatch. This contradicts D6. |
| Card review before confirm (`impl.card-review-before-confirm`) | Unbuildable cards (`DECISIONS.md:62` mechanics), and "I'd know it works if…" lines that no **TEST:** case covers (engine-follow-up D5, strips `:36`). | The user's card confirm, but charter-ritual-balance D2 rules that a downstream confirm "is not independent review" (`DECISIONS.md:84`). Otherwise the cycle grade, late. | **THIN.** The promise-to-test coverage check is mechanical, so a `sonnet` worker runs it with a read-back. The buildability judgment drops on a one-card run. On the gap path, `impl.design-review-pair` already grades "card quality (blocking)", which looks like a double grade. Fold the two. |
| Full gate suite (`impl.gates-full-suite`) | Breaking something outside the slice. Strip `:929` and `:2970`: "implement's own operationalization of the contract's regression-safety rule". | The final full-suite run. | **THIN.** Per cycle, run the affected tests only. Run the full suite once, at final validation. `impl.gates-never-triaged` stays **NEVER**. |
| Cold verification (`impl.cold-verification`) | A green result from a warm tree: cached builds, or files missing from a clean snapshot. Validator snapshot-isolation D0 / F24 (`DECISIONS.md:122`). The incident itself is `Assumed`. | CI on the user's push or pull request. Mochiko itself never pushes (`impl.no-git-mutations`). | **THIN.** The cold run becomes the only final gate run, never a warm run followed by a cold one. It may SKIP only when no dependency or build file is in the diff and CI gates the pull request, and the skip is stated. A skip contradicts D0 ("always-cold"). |
| Regression sweep (`impl.regression-sweep`) | A fix that breaks a *different* delivered feature without being structural (strip `:936`). Its gate set comes from gap-finding D7 (`DECISIONS.md:75`). | The full suite, but only where the gates are automated. **TEST:** gates that run against real infrastructure are often not automated. | **THIN.** Scope it to the delivered features whose territory the diff touches. Never SKIP: the user may not see a regression in someone else's feature until after acceptance. |
| Baseline diff review (`impl.baseline-diff-review`) | Unmarked writes, duplicate ids across concurrent runs, and entries stuck in-flight. Delta-files ruling D1/D3 (`DECISIONS.md:29`). | Nothing. This is a shared record, so a revert of the product code does not undo the damage. | **NEVER** for the mechanical legs: the unmarked-write test, the duplicate-id check and the flip. The judgment leg already fires only on build-raised entries. On a run that writes no baseline, the step is a no-op. |
| Design review pair (`impl.design-review-pair`) | A design past the gap list, or an infeasible design, reaching the user's signature. Plan-stage retirement D2 (`DECISIONS.md:62`); the feasibility pass comes from the architect restructure (`DECISIONS.md:87`). | The user's checkpoint table, and the cycle grade, late. | **THIN.** One non-author seat runs both skills. The feasibility architecture pass already fires only when a store delta exists. |
| Design-stage blind list (`impl.design-stage-blind-list`) | The lead's framing hiding choices inside a gap (implement-brainstorm-led). | The user ruling each piece, and the review pair. | **SKIP** at one or two gaps. A precedent exists: the rule already skips "on a single-gap re-fire". |
| Gap-finding pass (`impl.gap-finding-blind-dispatch`) | Blind spots the builder and the test author share. Gap-finding D1–D4 (`DECISIONS.md:75`). | The deterministic gates, which D1 keeps as the floor ("added on top, never swapped"). | **THIN or SKIP**, stated. Thin means the probe kit only, with a shorter timebox and no non-functional families. A SKIP is already lawful on delta and lane runs (`impl.gap-finding-scope`, `impl.fail.skip-unstated`). Extending it to low-stakes selection runs contradicts D2, which keys the pass on scope. |
| Mutation lens (`impl.mutation-lens`) | Weak tests. Gap-finding D5 makes it high-depth only. | It already does not run at low depth. | **Already SKIP** at depth=low. Nothing new is needed. |
| Design finish (`impl.design-finish`, `review-design-audit`) | Craft drift on a surface users see. | The audit is advisory (`impl.design-audit-advisory`), and the accessibility and viewport legs live in the **TEST:** gates (`impl.card-contents`). | **THIN.** Drop the audit with its reason stated, a lawful path today. Keep the critique lens's blocking part. |
| Landing verifier (`impl.landing-verifier-folds`) | The lead's own folds going unchecked. That seatless site comes from adversarial-review-generality, as cited in the author-grader-consolidation record `:199`. | Nothing: this is a shared record. | **THIN, never SKIP** when the run wrote a baseline, the store or `gates.md`. The checks are mechanically checkable, so a `sonnet` worker runs them with the lead reading back (`patterns-model-tiering`). |
| Built-vs-signed diff (`impl.store-landing`) | A store left disagreeing with what was built. | Nothing: this is a shared record. | **NEVER.** It fires only when structure was built, which is rare at low stakes. |
| Delta re-verification (`impl.delta-reverification`) | Full 13-gate cold sweeps on one-file deltas. Bounds contract (`DECISIONS.md:100`). | — | **Already THIN.** It is the model for the rest of this list. |

## The 40 floors

**Test:** a floor stays a floor if it names a decision the user owns or an end state the user is owed (no self-cleared output, no unearned evidence, no corrupted shared record, no silent skip); it is re-classed as a means if it names a procedure toward such an end, because another procedure could reach that end.

**Kept as floor at every stakes level: 31**

- **User-owned (16):** `gate-design-checkpoint` · `gate-card-confirm` · `gate-final-acceptance` · `design-first-write` · `deviation-gate` · `constitution-supremacy` · `constraint-challenge` · `attempt-exemption-user-only` · `epic-member-halt` · `gap-rework-bound` (its disposition is the user's) · `thinking-stage` · `fail.design-skipped` · `fail.no-acceptance` · `fail.deviation-unresolved` · `fail.design-outside-ruled-set` · `fail.cards-outside-order`.
- **End state owed (15):** `author-grader-default-fail` · `attempt-per-grade` (an honest count of rounds is part of the trail, per `DECISIONS.md:100`) · `gates-never-triaged` · `baselines-in-place-marked` · `lane-never-widens` · `sound-loop-floor` · `transport-floor` · `fail.card-independence` · `fail.quality-gate` · `fail.no-evidence` · `fail.regression` · `fail.spec-gap-unresolved` · `fail.unmarked-baseline-write` · `fail.store-landing-incomplete` · `fail.skip-unstated`.

**Re-classed as means: 9**

- **`baseline-diff-review` and `fail.unreviewed-baseline-diff`.** These name the procedure. The end, no corrupted baseline, is held by `fail.unmarked-baseline-write` and `baselines-in-place-marked`. The mechanical legs still always run, as the step table says.
- **`artifact-home`.** The conformance hook already enforces it mechanically, so the floor pin adds nothing.
- **`no-progress-stop`.** It is a stop heuristic for cost. A revert undoes a wasted round, and the attempt bound still caps it.
- **`minimalism-advisory` and `design-audit-advisory`.** These are ceilings that keep findings from blocking. Re-classing them only removes weight.
- **`fail.sufficiency-unrecorded`.** The end is a pinned base and a verdict on the record. On a zero-gap, low-stakes run, those could ride the run-open confirmation plus the run log. Caveat: `impl.evidence-citation` cites the pin, so the pin needs a durable home.
- **`fail.card-unchecked`.** It is bookkeeping toward "every card done", which the cycle reports also show.
- **`fail.gap-finding-missing`.** It names a procedure. Its end, spec-required behaviour not broken, is held by `fail.spec-gap-unresolved` and the deterministic gates.

## Cheap roads

1. **Do nothing new.** The thin path already exists for some runs. A low-stakes change routed through `/mochiko:feature` as a delta card already skips gap-finding with the skip stated, and depth=low already skips mutation. The cost is zero rules. The limit is that delta scope is a shape, not a stakes reading.
2. **Reuse one rule.** Widen `impl.fail.skip-unstated` from "the skipped gap-finding pass" to any skipped or thinned step. That gives one fail rule and one block in the final-validation report, and it is the whole honest trail. It revives the departure trail of lead-owned-process-flexibility (`DECISIONS.md:126`), which command-architecture-realignment D1/D2 killed (`DECISIONS.md:115`).
3. **Fold, don't cut.** Every THIN above changes the grader's *form*: batching, one seat, a `sonnet` worker, or a delta read. Author-grader-consolidation D1 licenses exactly that lever, so no grader is removed.

## Prior rulings this list contradicts

- **`production-floor-adaptive-depth` D8-dial-standards-scope.** It rules that "pipeline process rigor stays uniform… Low relaxes what is asserted about the code, never how carefully mochiko itself works" (record `:73-74`). Every SKIP and THIN above contradicts it. `impl.mutation-lens` (`when: depth=high`) already breaches it today.
- **`author-grader-consolidation` D1-universal-grader-independence.** It ruled "The surface-value tiering road … is recorded-rejected" (record `:202`). A stakes-keyed **SKIP** of a grade walks that road. A THIN of the grader's form does not.
- **`charter-ritual-balance` D1–D2.** It ruled the loop "kind-keyed, not scale-keyed" with "no size threshold" (`DECISIONS.md:84`). A stakes key is not a size key, but a reviewer will read it as one.
- **`producer-plan-enforcement` D6** "no size carve" (`DECISIONS.md:53`), contradicted by the plan-grade thin.
- **`validator-worktree-isolation` D0** "always-cold" (`DECISIONS.md:122`), contradicted by the conditional cold skip.
- **`qa-gap-finding-verification` D2** keys the pass on scope (`DECISIONS.md:75`). A stakes-keyed skip on selection runs contradicts it. D1 still holds, because the deterministic gates never move.
- **`command-architecture-realignment` D1/D2** (`DECISIONS.md:115`), contradicted by cheap road 2, the departure trail.
