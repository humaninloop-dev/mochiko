# Second list — the stakes key

Seat: `second-list-stakes` · date 2026-10-27 · model opus · inputs received: the session frame and the stakes-key question only.
Disclosure: no read of any file in this session directory; the directory's name appeared in `git status` and in one `ls` of `.mochiko/brainstorms/`, no content seen. No Explore dispatch was made — every read was a targeted, interpretive render (`mochiko-cli rules`) or a grep of `DECISIONS.md` and three prior records, so no dispatch returned empty.

## A collision every road inherits

`production-floor-adaptive-depth` D8-dial-standards-scope (`DECISIONS.md:91`, record line 73) ruled "pipeline process rigor stays uniform". Any stakes key supersedes D8 in part and must be recorded as such. The tree already bends D8: `testing-gap-finding.mutation-lens-bounds` and `impl.mutation-lens` key a verification step on `depth=high`, the code-standards dial. The key also sizes only non-floor steps: `impl.regression-sweep`, `impl.cold-verification`, `impl.gates-full-suite`, `impl.gap-finding-scope` and `impl.card-review-before-confirm` are `class: must`, while `impl.gates-never-triaged`, `impl.attempt-per-grade` and the `testing-end-user` floors stay fixed at every level.

## The roads

**1. Retrofit-cost categories (reuse a ruled line).** Shape: deep verification where the run touches a category already ruled expensive to undo — persisted formats, storage engines, locking/concurrency, migration-bearing shapes — shallow elsewhere. Reads: the card's brownfield exposure, `data-model.md`, `contracts/`, adopt-first category hits. Fails: the list was cut for build-vs-buy, so it misses user-visible harm with cheap retrofit (a wrong price shown, an auth bypass in stateless code). Leans on `build-vs-off-the-shelf` D4 (`DECISIONS.md:81`) and adaptive-depth D5-low-retrofit-principle; the user has ratified this principle twice.

**2. Undo cost, general reversibility.** Shape: one question per touched surface — can a revert commit fully undo this, with no residue in data, external systems, or published contracts? Residue raises depth. Reads: contracts, data-model state machines, external integration surfaces in `quickstart.md`, migration presence. Fails: the lead judges reversibility optimistically; "we can roll back" is cheap to claim, so it needs a cited residue line per surface. Leans on the same D5 retrofit principle; a kind test, so no collision with `patterns-sound-loop.two-part-trigger`.

**3. Harm kind (worst plausible failure class).** Shape: classify what breaks if the step's object is wrong — silent data corruption, security or trust breach, money, availability, user-visible wrong answer, cosmetic — and size depth to the worst class touched. Reads: spec FR/SC lines, DS-XXX sensitivity, NFR rows. Fails: "blast radius" drifts into how many users or how much code, which is magnitude; the key must name a class, never a count. Leans on the kind-keyed precedent of `charter-ritual-balance` D1 (`DECISIONS.md:84`: "kind-keyed, not scale-keyed").

**4. Data sensitivity and trust boundary.** Shape: deep where the run reads or writes Confidential/Restricted entities or crosses a trust boundary; standard elsewhere. Reads: DS-XXX classes in `data-model.md`, auth surfaces in `contracts/`. Fails: narrow — billing math on Internal data reads low. Leans on `patterns-entity-modeling`; respects `setup-product-agnostic` D2 (`DECISIONS.md:27`) because it reads the product instance, not governance.

**5. Sufficiency exposure profile (reuse the entry report).** Shape: key on which clause kinds touched something real — contract, data, structural trip, NFR, commodity, delivered-feature, in-flight — never on gap count. Reads: `sufficiency-report.md`, already on hand at run-open. Fails: sufficiency grades guidance, not risk, so a fully sufficient payments row reads calm; counting gaps is magnitude by proxy. Collides softly with `review-sufficiency.verdict-is-input`: the report gains a second purpose its grader never wrote it for.

**6. Delivered-territory exposure.** Shape: deep where the run modifies delivered behaviour, owns a seam whose earlier side is delivered, or inherits accumulated gates; light on purely additive greenfield. Reads: `review-sufficiency.clause-delivered-feature`, card brownfield exposure, territory `gates.md`. Fails: a product's first feature has zero delivered exposure and maximal novelty, so it reads low exactly when it should not; needs road 7 as counterweight.

**7. First-of-kind.** Shape: deep where the run introduces something with no built precedent in the repo — a new dependency, infrastructure piece, seam type, store element kind, or the first UX surface; light where it follows a delivered and gated pattern. Reads: the architecture store's `built` elements, adopt-first decisions, `gates.md` presence. Fails: copied patterns carry copied bugs, and "novel" is loosely judged without a cited precedent.

**8. Residual-check coverage.** Shape: a step's depth drops only where a later independent check catches the same failure class — per-cycle verification may run lighter on spec behaviour the blind gap-finding pass will probe. Reads: the run's own step map and scope. Fails: two shallow checks each assume the other; delta and lane runs skip gap finding (`impl.gap-finding-scope`), so the redundancy vanishes there. Leans on the model-tiering seat default key — "does a structurally independent seat stand between this seat's output and the run's verdict?" (`patterns-model-tiering.seat-default-key`).

**9. NFR and integration presence.** Shape: deep where a touched store concern row carries a numeric NFR target or a real external integration exists; those steps must measure on real infrastructure. Reads: `concerns.md` NFR-XXX rows, IP-XXX rows, `quickstart.md`. Fails: an immature store with no concern rows reads low stakes, turning absence into evidence against the spirit of `review-sufficiency.default-fail`.

**10. Per-promise key.** Shape: no run-level key; each SC-XXX, FR-XXX and "I'd know it works if…" line carries its own stakes class, and verification depth is sized per promise. Reads: spec, design-stage lines, the `**TEST:**` cases tied to each promise. Fails: fine grain and high authoring cost; specs carry no stakes tags today, so the lead infers them and the trail grows long.

**11. User-declared stakes at run-open.** Shape: the lead proposes a stakes line from surfaces and the user confirms it alongside batch, scope and attempt bounds at the run-open confirmation. Reads: the lead's proposal plus the user's word. Fails: default-accept fatigue, and it hands back the call the destination gives the lead. Leans on adaptive-depth D1, where the depth dial moves only by user declaration.

**12. Escalation on in-run evidence.** Shape: every step starts at a stated base depth and rises on signals — a consumed attempt, a failed gate, a deviation, a mid-run design refire, a flaky suite. Raising is free under this session's D1; only a lowering is trailed. Reads: the run's own events. Fails: the question asks for a run-open read, and a low start lets early defects slip. Leans on `impl.attempt-per-grade` and `impl.no-progress-stop`.

**13. Do nothing new: reuse `scope` and `ux_bearing` (cheap).** Shape: implement's existing conditions already gate gap finding and the critique lens; declare them the key and add one skip line. Reads: nothing new. Fails: scope type is a door, not stakes — a delta hotfix to payments is high stakes — and selection-versus-epic sits closer to size than to harm, so this road risks magnitude by another name.

**14. Free judgment with a disclosure line (cheap control arm).** Shape: no key; the lead rules depth per step and trails each lowering in one pinned line, like `patterns-sound-loop.disclosure-line`. Reads: whatever the lead reads. Fails: under-classification — the risk `charter-ritual-balance` review fold I6 accepted only because the floor's trigger was a kind test; bet B says the appetite regrows.

**15. Two-axis composite: harm kind by undo cost.** Shape: roads 3 and 2 as a small grid; each cell names a depth for each non-floor verification step, with an explicit not-inputs list beside it. Reads: spec, DS-XXX, contracts, data-model, migration presence. Fails: two judgments per surface; it needs one cited line per axis to stay honest.

## What the key must not be

- **Not magnitude.** File, line, card, row, gap, cycle and user counts are excluded, and so is epic membership as such (`patterns-sound-loop.two-part-trigger`: "magnitude never gates it").
- **Not the governance `depth` dial.** That dial keys code standards (D8); reading it would merge two axes the user kept apart.
- **Not scope type alone.** Scope names the door, not the harm.
- **Not lead familiarity or confidence.** "This looks routine" is the under-classification the disclosure line exists to catch.

## Top three

1. Harm kind by undo cost (15): it asks "how bad and how permanent", a kind test by construction.
2. Retrofit-cost categories (1): it reuses a line the user has already ruled twice, so it costs one pointer.
3. Residual-check coverage (8): it stops two shallow checks trusting each other, the real way a lowering hides.
