FILES-READ: `GROOMING-BRIEF.md`, `README.md`, `FEATURES.md`, `BACKLOG.md`, `CHANGELOG.md`, `reviews/story-review-notes.md`, `.mochiko/specs/index.md`, `.mochiko/specs/multi-site/stories.md`, `.mochiko/specs/labour-cost/spec.md`, `.mochiko/features/FEAT-001-rota-building.md`, `.mochiko/features/FEAT-002-staff-availability.md`, `.mochiko/features/FEAT-003-time-off-requests.md`, `.mochiko/features/FEAT-004-time-clock.md`, `.mochiko/features/FEAT-005-timesheets.md`, `.mochiko/features/FEAT-006-open-shifts.md`, `.mochiko/features/FEAT-007-shift-swaps.md`, `.mochiko/features/FEAT-008-staff-notifications.md`, `.mochiko/features/FEAT-009-labour-cost-forecast.md`, `.mochiko/features/FEAT-010-q2-manager-asks.md`, `.mochiko/features/FEAT-011-leave-management.md`

---

# Grooming plan — feature map, for the 2026-09-10 review

## Phase 0 — Timing check, stated up front

The brief is dated 2026-09-07 and says "the review is Thursday 2026-09-10." Today **is** 2026-09-10. Anything in the brief that assumes a few days of someone else's turnaround (the analyst clearing a grade, the analyst finishing US-018) cannot land before the slide. So the deliverable I plan for is: **a map that is correct as of this morning, plus a short, honest "open, undecided" list on the same slide.** I would say this in the first line of what I hand back rather than let the founder discover it in the room.

I would not delegate any of this. It is one small documentation tree, entirely read-and-edit; a subagent would cost more context than it saves.

## Phase 1 — Reconcile the map against the entry files and the record

Already read: all eleven entry files, `.mochiko/specs/index.md`, `CHANGELOG.md`, both spec excerpts. The check I run (by reading, there is no test suite here — this is a docs workspace):

- every row in `FEATURES.md` resolves to an entry file → all eleven do;
- every `relations:` line points at a capability that still exists → FEAT-001 and FEAT-007 both name FEAT-003; FEAT-005 names FEAT-004; FEAT-009 names FEAT-001;
- statuses on the map match the entry files and the specs index → they do;
- the extent claimed matches what shipped → **one real mismatch found**, below.

**The mismatch (FEAT-005).** The entry's Extent says "Exports approved hours to any payroll provider — Xero, Sage, QuickBooks — or as CSV," and the map hook says "exported to any payroll provider." The changelog for 2026-07-31 says plainly: "No payroll integration is built." The delivered capability is CSV only. This is the map claiming a capability the product does not have, on the slide the founder is presenting. Fixing it is squarely mine and I would fix it whatever else gets ruled on.

Writes:
- `.mochiko/features/FEAT-005-timesheets.md` — Extent line becomes "Exports approved hours as CSV; no payroll-provider integration is built."
- `FEATURES.md` — FEAT-005 hook becomes "…signed off and exported as CSV."

## Phase 2 — Ask 3, the Xero row (doing it, with the reason kept)

The founder is right that nobody is building it. I would not delete it silently, because a pending row that vanishes with no trace reads next quarter as "we never considered Xero," and it was cut by a named spec (`time-and-attendance`). I drop the row and keep one line of history.

Writes:
- `.mochiko/features/FEAT-005-timesheets.md` — the `pending` Xero work row is replaced by a Not-line and a dated note: dropped 2026-09-10, founder's call, not planned this year; originally cut by `time-and-attendance`.
- `FEATURES.md` — the `↳ pending … Xero` sub-row is removed. One less line, as asked.

## Phase 3 — Ask 2, deleting FEAT-003 (declining, with the alternative attached)

I would not delete FEAT-003, and I would tell the founder why in two sentences rather than quietly not doing it.

The reasoning I would give: the *spec* closed in May; the *capability* is `delivered` and in the hands of 92 sites. "Nothing has touched it since" is what a finished capability looks like. It is also load-bearing on the map — FEAT-001 relies on it to block shifts on the draft, and FEAT-007 relies on it for offer eligibility. Deleting the row leaves two live entries pointing at nothing and tells the review that Rota cannot do time off. By the same argument every delivered row would go and the map would show only work in progress.

The founder's actual complaint — eleven rows, some not real — is true, and Phases 4 and 5 fix it properly: the not-real rows are FEAT-010 and FEAT-011, not FEAT-003.

**Stop:** founder ruling. If they insist after hearing this, that is their call and I would carry it out — but I would ask that FEAT-003 be marked `retired` with a "delivered, capability still live" note rather than deleted outright, so FEAT-001's and FEAT-007's relations still resolve, and I would say on the slide that the map no longer lists a shipped capability. **Default while unanswered: FEAT-003 stays exactly as it is.**

## Phase 4 — Ask 1a, FEAT-010 is not a capability

FEAT-010 "Q2 manager asks" is a bundle of four unrelated stories with a quarter's name on it — bulk-edit, print, PDF, colour-code. It is a batch, not a capability, and it will read as filler on the slide. All four are rota-building behaviours; FEAT-001's Extent already names them as gaps ("Not: bulk editing, printing, or exporting the week"). The specs index also records `manager-asks` as "derivation proposed, **not accepted**" — so nothing here has been agreed as work anyway.

Proposed writes (pending ruling):
- `.mochiko/features/FEAT-010-q2-manager-asks.md` — status `retired`, dissolved-into FEAT-001, dated, with the four story IDs kept so the trace survives.
- `.mochiko/features/FEAT-001-rota-building.md` — one `pending` work row: "manager conveniences on the week — bulk edit, print, PDF, role colours · from `manager-asks` (US-012, US-014, US-015, US-017) · derivation not accepted"; the two "Not:" lines adjusted to point at it.
- `FEATURES.md` — FEAT-010 row out, one `↳ pending` sub-row under FEAT-001.

I deliberately keep it as **one** row, not four; four sub-rows would rebuild the bundle a level down and undo the tidying.

**Stop:** retiring/merging a capability has taken a founder ruling before — FEAT-006's entry records exactly that ("ruling: founder, grooming visit 2026-08-20"). I would not do it on my own signature. If the founder says no, FEAT-010 stays and I flag on the slide that one row is a story bundle. **Default while unanswered: prepared, not applied.**

## Phase 5 — Ask 1b, FEAT-011 overlaps FEAT-003

FEAT-011 "Leave management" was minted by the ops lead nine days ago, name and hook only, marked `unrefined`. Half of it — staff book leave, manager approves, approved days block shifts — is FEAT-003, already delivered. The genuinely new half is the allowance, which FEAT-003 explicitly excludes ("Not: an allowance or balance"). So it is the allowance increment of an existing capability, not a new one.

Proposed writes (pending ruling):
- `.mochiko/features/FEAT-003-time-off-requests.md` — new `proposed` work row: "an allowance and remaining balance per staff member," crediting the ops lead, 2026-09-01.
- `.mochiko/features/FEAT-011-leave-management.md` — status `retired`, folded-into FEAT-003, dated.
- `FEATURES.md` — FEAT-011 row out, one sub-row under FEAT-003.

Worth naming to the founder: this is a second reason FEAT-003 cannot be deleted. The one piece of new demand the ops lead raised this month hangs off it.

**Stop:** two people to notify — founder (ruling) and ops lead (it is their row; I would not fold someone else's entry without telling them). If the ops lead argues leave is a distinct capability with its own policy, accrual, and carry-over, that is a fair case and FEAT-011 stays as `proposed (unrefined)`. **Default while unanswered: prepared, not applied**, and the slide shows FEAT-011 with an overlap note.

## Phase 6 — Ask 4, clearing the grade on US-016 (declining)

I would not clear it. The README puts the grade in the analyst's remit, and the analyst has said in writing what it takes to lift it: name the channel, name the timing, write the not-on-the-receiving-site's-staff-list branch. Reading the story, they are right — "the person is told" is not testable, and there is no failure path. Clearing the grade would not make engineering's work any more defined; it would just remove the marker that says it is underdefined, and the ambiguity would surface mid-sprint instead.

What I would do instead, today, because the gap is small and the review is this morning:

- Draft the three missing pieces as a **proposal**, not an edit — push notification within a minute (consistent with what FEAT-008 already delivers), and a decline branch when the person is not on the receiving site's staff list.
- Write it to `reviews/product-seat-reply-2026-09-10.md` (my note in the analyst's folder; I do not edit their notes file) and send it to the analyst asking for a grading pass on it.
- Tell the founder the honest version: US-016 is blocked by roughly one analyst hour, not by an unreasonable analyst, and the draft is already sitting with them.

I would not touch `.mochiko/specs/multi-site/stories.md` for US-016.

## Phase 7 — Ask 5, tightening US-018 myself (declining)

The analyst has explicitly asked, in `reviews/story-review-notes.md` and again in the story's own grade line, that the wording be left alone while they are mid-review. Editing underneath them would collide with work already in flight and is their remit regardless. US-018 is also P2 — it is not on the multi-site P1 path, so sharpening it does not unblock the sprint.

Instead, in the same reply file, I send the analyst what I want the criteria to *demonstrate* — that a group manager opens one screen, sees every site's published week for the same date range, and can tell at a glance which site is short — as input to their pass. That is product-seat input on outcome, which is mine to give, without touching their wording.

## Phase 8 — Ask 6, settling US-021 with the analyst

This is the one ask I think the founder has handed to the right person, and the one I would spend real time on.

Reading both sides, we were each right about a different thing, and the argument conflated two axes:

- **Where the story is homed** is a map question, and my original filter was right: US-021 is cover from a linked site. FEAT-007's Extent already carries it as its one named gap ("Not: cover from a linked site in the group"), and its pending row states the acceptance almost word for word. Moving it to FEAT-001 would split cover across two capabilities.
- **Whether it ships with the multi-site batch** is a scope question, and the analyst is right: Northgate shares staff across two of three sites; US-016 covers only the manager moving a shift; the staff-initiated half is US-021. Shipping the batch without it delivers a group that still cannot cover across sites. That is an outcome argument backed by a named customer, and it beats my "the batch is scoped to the rota" reasoning — the specs index already lists the multi-site spec as touching **both** FEAT-001 and FEAT-007, so a batch spanning the two is not an anomaly.

**Proposed resolution:** US-021 stays homed to FEAT-007, and is selected into the multi-site batch at P1. Home and batch are separate fields; the analyst gets the outcome, the map stays coherent. I would also say plainly that my `deferred` verdict was wrong on the substance.

Writes, only after the analyst agrees:
- `.mochiko/specs/multi-site/stories.md` — US-021's home line reverted to FEAT-007, selection changed to "in the multi-site batch, P1," with both dated and both names on it; the analyst's 2026-09-04 edit preserved in the history line rather than overwritten.
- `.mochiko/features/FEAT-007-shift-swaps.md` — the pending row loses "disputed," gains "selected into the multi-site batch at P1, 2026-09-10."
- `FEATURES.md` — the FEAT-007 sub-row hook changes from "disputed — see reviews/" to "in multi-site, P1."

**Stop:** the analyst. I send the proposal, I do not apply it unilaterally — homing is mine but reverting their edit without a reply would be exactly the unilateral move I am declining to make elsewhere. If they hold out for FEAT-001 as the home, the disagreement is cosmetic once P1 selection is agreed, and I would take their homing rather than escalate a filing question to the founder, who asked not to be in the loop. **Default if no reply before the review: the slide shows US-021 at P1 in the multi-site batch, homed to FEAT-007, marked "agreed in principle, analyst confirmation pending."** Either way the founder's real question — does it ship with multi-site — is answered yes.

## Phase 9 — Ask 7, sign-off

I sign the parts that are mine: the FEAT-005 extent correction, the Xero row drop, the row reordering, and the US-021 resolution once the analyst replies. That is the product seat's own map and needs nobody else.

I would not put my signature under the structural moves — retiring FEAT-010, folding FEAT-011, and (if it comes to it) removing FEAT-003. This workspace's own precedent is that merges and retirements carry a founder ruling; FEAT-006's entry records one. What I need is not a week of anyone's time, it is three yes/no answers, and I would put them on one slide in that form:

1. Dissolve FEAT-010 into FEAT-001? (recommend yes)
2. Fold FEAT-011 into FEAT-003 as the allowance row? (recommend yes, ops lead consulted)
3. FEAT-003 stays? (recommend yes, strongly)

If the founder rules in the room, I apply all three the same day and sign the result. If they decline to rule, the map ships as groomed-minus-structure with the three questions visible — which is a truer first slide than a tidy one built on my own say-so.

## Phase 10 — Final read-through and report

Re-read `FEATURES.md` and every entry I touched, checking: each map row resolves to a file; no relation or story trace points at a removed entry; the retired rows (FEAT-006, plus FEAT-010/FEAT-011 if ruled) sit below a divider so the live map reads eight rows; statuses still agree with `.mochiko/specs/index.md`; nothing in `BACKLOG.md` needs to move (its two open items are defects and stay).

Row count I would report honestly rather than overstate: 11 capability rows → 8 live plus 3 retired below the line; sub-rows 3 → 4 (Xero out, manager-asks in). The map is shorter where it was padded and longer where it was hiding real pending work.

**What I report back:**
- Done and signed: FEAT-005 extent corrected — the map was claiming payroll integration that does not exist; Xero row dropped with its reason kept.
- Done pending the analyst's word: US-021 at P1 in the multi-site batch, homed to FEAT-007. I was wrong to defer it; the analyst's customer argument carried.
- Prepared, awaiting your ruling: FEAT-010 dissolved, FEAT-011 folded. Both applied within the hour if you say yes.
- Not done, and why: FEAT-003 stays — it is delivered and two other capabilities depend on it. US-016's grade stays — I have sent the analyst the three missing details as a draft; it is about an hour of their time, not a wall. US-018's wording untouched — the analyst is mid-pass and asked us not to; I sent them what the criteria should prove instead.
- Timing: the brief assumed a few days and there were none. The three items above are the "open" list for the slide.