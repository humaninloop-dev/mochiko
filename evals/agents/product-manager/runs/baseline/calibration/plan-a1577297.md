FILES-READ: `.mochiko/specs/time-and-attendance/spec.md`, `FEATURES.md`, `.mochiko/features/FEAT-001-rota-building.md`, `.mochiko/features/FEAT-002-staff-availability.md`, `.mochiko/features/FEAT-003-time-off-requests.md`, `.mochiko/specs/index.md`, `README.md`, `BACKLOG.md`

---

# Action plan — feature-map delta for the `time-and-attendance` spec

## Phase 0 — Load the map-authoring procedure before drafting anything

- Invoke `mochiko:authoring-feature-map` and take from it the authoritative entry shape, the delta rules for new-vs-extend, the filter procedure for story claims, and the selection-advice format. Everything drafted below is shaped from the three existing entries (FEAT-001/002/003), which are consistent with each other — capability paragraph, `Extent` bullets including honest `Not:` lines, `Relations`, `Story trace` keyed by spec slug plus spec-local story IDs. If the skill's shape differs from what those files show, the skill wins and I reshape before writing.
- Note for myself: story IDs are spec-local and reused across specs (FEAT-001 traces `rota-basics: US-001…`, FEAT-002 traces `availability: US-001…`). So the new entries must trace as `time-and-attendance: US-00X` and never as bare `US-00X`.

## Phase 1 — Read the ground truth (done in this session)

Read, in this order and for these reasons:

- `README.md` — what the product is and who runs it; confirms the product seat writes the map and that stories are the ops lead's and analyst's. Also tells me the `shifts` table still lives in the SQLite reporting sidecar, which bears directly on US-008.
- `FEATURES.md` — the three-row live map; the shape of a row (ID, capability, status, hook).
- All three `.mochiko/features/FEAT-00*.md` — before proposing anything new I need to know what already exists and what each entry explicitly excludes. Two exclusions turn out to be load-bearing for this spec: FEAT-001 says *not* copying a previous week into a draft (US-007) and *not* any notification to staff (US-005).
- `.mochiko/specs/index.md` — the index row for this spec is `specify — derivation pending`, capabilities touched `—`; that row is mine to update.
- `BACKLOG.md` — confirms the SQLite/Postgres split is already tracked there as engineering/defect work, not capability work.
- The spec itself.

**One cheap delegation.** Spawn a single throwaway `Explore` subagent with `model: haiku`, brief: *"In this workspace, list every file (any extension, including non-markdown) containing any of: clock-in, clock in, attendance, timesheet, worked hours, Xero, Twilio, payroll. Return file path plus the matching line, nothing else."* Purpose: make sure no already-delivered or half-documented attendance capability exists outside the seven docs I read, so I don't mint a duplicate. On return I check that every hit is inside `.mochiko/specs/time-and-attendance/spec.md`; any hit elsewhere I open myself and reconcile before drafting. This is the only sweep worth delegating — the doc set is small enough that I read it directly rather than farm out interpretation.

## Phase 2 — Stop and flag two things before deriving

I would raise both in one message and continue under the stated defaults rather than block.

1. **The dates don't line up with the ask.** The spec is dated 2026-06-08 and books the analyst's story review for 2026-06-15; today is 2026-09-11. So the review the founder wants this delta to precede is three months past. Either the spec has sat untouched and the review never happened, or a review happened and these stories are stale. What I'd confirm: is `spec.md` still the current draft, and has the analyst reviewed it? **Branches:** (a) *stories are current, review still pending* → derive as planned, mark the delta provisional pending review; (b) *the analyst already reviewed and stories moved* → I stop and ask for the reviewed version, because deriving from superseded stories produces a map that is wrong on day one; (c) *the spec is being restarted* → hold the delta. **Default if no answer: (a)** — derive from the text as written and stamp the delta note "derived from the 2026-06-08 draft; unreviewed."

2. **I will not use the engineering lead's carve.** The spec proposes "Sprint 14 — clock-in foundations", "Sprint 15 — hours and export", "Nice-to-haves". Those are sprint containers, not things the product does; "Nice-to-haves" is not a capability under any reading, and a feature named after a sprint is dead the moment the sprint closes. The offered alternative — one "Time & attendance" feature covering all eleven stories — fails the other way: its extent would need clock capture, breaks, corrections, totals, overtime, approval and export, which is nowhere near three lines. I record this rejection in writing with the reasoning; I don't edit the engineering lead's section out of `spec.md` (not my document to rewrite), I counter it in the delta note. Note also the intent line "We want one feature: time and attendance" — that's the *intent*, one problem; it is not evidence of one capability.

## Phase 3 — Frame the capabilities as a hypothesis, in nouns and verbs

Before touching story text, write down what this intent implies the product will be able to do, ignoring how the stories are cut:

> *Staff record when they actually started, stopped, and took breaks against their scheduled shift. The product turns those records into a week of worked hours per person that a manager can correct and trust. A manager signs those hours off and hands them to the accountant.*

Three verbs — capture, account, hand off. That is the hypothesis. Then check it against the stories and let them sharpen it; where a story genuinely contradicts the frame, the story wins.

## Phase 4 — Filter every story, with a written verdict for each

Run each of the eleven claims against the map. Every no gets written down; nothing is silently dropped. My intended verdicts:

| Story | Verdict | Reason |
|---|---|---|
| US-001 clock-in/out | **accept** → new FEAT-004 | the core new thing the product does |
| US-002 breaks | **accept** → FEAT-004 | same capability; a break is part of the attendance record, not a second capability |
| US-003 manager corrects a time | **accept** → new FEAT-005 | correction-with-original-preserved is what makes the hours record trustworthy |
| US-004 CSV export | **accept** → new FEAT-006 | the hand-off to the accountant |
| US-005 SMS shift reminder | **defer — out of this intent** | a real capability (staff notifications), but it's about no-shows, not attendance; FEAT-001 already declares "no notification to staff" as an honest exclusion. It needs its own spec. Twilio is implementation, not capability, and never appears on the map. Recorded so it isn't lost; no entry minted. |
| US-006 Manchester UTC bug | **reject — not a capability** | a defect against delivered FEAT-001. Belongs in `BACKLOG.md`. It does *not* change FEAT-001's extent — the entry already claims shifts carry start and end times; it claims correctly and is broken. I'd flag to the founder that a delivered capability is currently wrong for a paying three-site group, which is a selection input. |
| US-007 copy last week | **defer — extends FEAT-001, not this spec** | genuine capability increment, but against rota building, whose extent says in as many words "Not: copying a previous week into a draft". If it's built, that `Not:` line flips and FEAT-001 carries a pending increment. Out of scope for a time-and-attendance delta. |
| US-008 shifts → Postgres | **reject — not a capability** | enabling/architecture work; the product does nothing new when it lands. Already on `BACKLOG.md`. It is a hard *sequencing* dependency for FEAT-004 (clock records need something reliable to join to) and appears in my selection advice and as a note on the entry, never as a map row. |
| US-009 overtime flag | **accept** → FEAT-005 | part of what the week's hours view tells a manager before sign-off |
| US-010 sign-off before export | **accept** → FEAT-006 | approval is the gate on the hand-off; same breath as export |
| US-011 Xero | **reject now — record as an exclusion** | explicitly not this release. Minting a `proposed` entry for it would flatter the map. It goes into FEAT-006's extent as a `Not:` line, which keeps it visible and keeps the map honest. |

## Phase 5 — Decide the granularity and name the split

Three entries, each nameable in one breath, each with an extent that fits in about three lines:

- **FEAT-004 — Shift clock-in** (US-001, US-002)
- **FEAT-005 — Worked hours record** (US-003, US-009)
- **FEAT-006 — Hours approval and payroll export** (US-010, US-004)

The judgement call I'd surface rather than bury: US-003's corrections could sit with capture (they edit clock times) instead of with the hours record. I put them with FEAT-005 because the manager does that work while reviewing the week's hours before sign-off, and because FEAT-004 stays a clean "staff record their own time" capability. **Stop point:** I'd tell the founder this is the one arguable seam and ask whether corrections read as capture or as the timesheet. **Branch:** if the ruling is "capture", US-003 moves to FEAT-004's extent and FEAT-005 narrows to totals plus overtime — at which point I'd re-check whether FEAT-005 still stands alone or should fold into FEAT-006. **Default: as drafted above.**

I would also state plainly that I'm not merging FEAT-004 and FEAT-005 for convenience and not splitting FEAT-006's approval from its export — approval that gates nothing and an export with no gate are each half a capability.

## Phase 6 — Write the map

Files I would create:

- **`.mochiko/features/FEAT-004-shift-clock-in.md`** — status `proposed`. Capability: staff record their actual start, breaks, and end against a scheduled shift from the staff app. Extent: clock-in available within 30 minutes either side of a scheduled shift and clock-out thereafter; breaks recorded while clocked in and deducted from worked time; one open clock session per person. Not: clocking in with no scheduled shift; not geofencing or any location check; not editing one's own recorded times (that's FEAT-005). Relations: composes-with FEAT-001 — a clock record attaches to a published shift. Note: depends on the `shifts` table living in Postgres (`BACKLOG.md`). Story trace: `time-and-attendance: US-001, US-002`.
- **`.mochiko/features/FEAT-005-worked-hours-record.md`** — status `proposed`. Capability: the week's worked hours per person, derived from clock records less breaks, correctable by a manager with the original preserved, flagged when over contracted hours. Extent: per person per week per site; a manager edits a clock-in or clock-out and the original stays visible as edited; hours above contracted hours are flagged on the week's view. Not: any pay rate or cost figure; not a holiday or absence balance (FEAT-003 already declares the product doesn't count entitlement). Relations: builds-on FEAT-004; composes-with FEAT-006 — approval acts on this record. Story trace: `time-and-attendance: US-003, US-009`.
- **`.mochiko/features/FEAT-006-hours-approval-and-export.md`** — status `proposed`. Capability: a manager approves each person's hours for the week, and approved hours leave the product as a CSV for the accountant. Extent: approval per person per week, required before export; CSV columns name, site, role, hours, overtime hours; unapproved hours cannot be exported. Not: pushing hours to Xero or any payroll system — the hand-off is a file (US-011, deliberately out of this release). Relations: builds-on FEAT-005. Story trace: `time-and-attendance: US-004, US-010`.

Files I would edit:

- **`FEATURES.md`** — three new rows in the existing table, status `proposed`, each with a one-line hook in the same register as the existing three (e.g. FEAT-004: "staff tap in and out against their shift; breaks come off the worked time").
- **`.mochiko/specs/index.md`** — the `time-and-attendance` row's *Capabilities touched* becomes `FEAT-004, FEAT-005, FEAT-006`; status stays open with the derivation no longer pending. Left closed-date blank.
- **`.mochiko/specs/time-and-attendance/spec.md`** — the single header line "Feature derivation: product seat" updated to point at the delta note. I touch nothing else in that file; the stories and the engineering lead's section are not mine to rewrite.

New note I would write:

- **`.mochiko/specs/time-and-attendance/feature-map-delta.md`** — the delta itself: the hypothesis framing from Phase 3, the eleven verdicts with their reasons in full (including the three rejections and two deferrals), the written rejection of the sprint-shaped and single-feature carves, the FEAT-005 seam question, and the selection advice from Phase 7. This is where every "no" lives so none of them are invisible.

Things I would refuse outright: creating a "Sprint 14" or "Nice-to-haves" feature; creating a feature for the Postgres migration; creating a `proposed` Xero entry; writing any extent that implies the product already does these things — everything here is `proposed`, not `delivered`.

## Phase 7 — Selection advice (recommendation only)

Present, without choosing:

1. **Move `shifts` to Postgres first** (US-008 / the existing backlog item). Not a feature, but FEAT-004 sits on it, and the nightly copy has already missed a day twice. Cost of skipping: clock records joined to an unreliable table, and the first wrong timesheet destroys trust in the whole capability.
2. **FEAT-004 next.** The two three-site groups threatened churn over "does clock-in" specifically; FEAT-004 alone is the answer to that sentence. Cost of deferring: the stated churn risk stays live.
3. **FEAT-005, then FEAT-006.** FEAT-006 is what actually ends the Sunday-evening spreadsheet retyping described in the intent — so if the founder weighs the Sunday pain above the churn threat, the honest read is that FEAT-004 alone doesn't relieve it, and 004→005→006 must ship as a run. Cost of stopping after 004: staff tap buttons and the manager still retypes hours, which is worse than before for the manager.
4. **US-006 (Manchester timezone)** is a live defect on a delivered capability affecting a paying group — I'd put it in front of the founder alongside this delta even though it is not map work.
5. **US-005 and US-007** each want their own spec later; I'd note what US-007 costs FEAT-001's honest `Not:` line when it lands.

The selection is the founder's. I'd say so explicitly.

## Phase 8 — Review and report

- I don't grade my own map writes. I'd ask the requirements analyst to check the three entries against the stories at their review, and make clear the reverse also holds: their story verdicts aren't mine to edit. If the analyst comes back saying a story I rejected — most likely US-008 or US-006 — is load-bearing for this release, that disagreement goes to the founder rather than either of us overruling the other.
- Mark the delta provisional until the story review lands; re-run Phase 4 against any reshaped stories and issue a follow-up delta if the verdicts move.

**Final report to the founder:** three new proposed capabilities with their names and extents; five stories that do not earn a map entry and exactly why (one defect, one enabling migration, two out-of-intent capabilities, one explicit later); the engineering lead's carve declined with reasons and what I did instead; the one open seam question on US-003; the recommended build order with each deferral's cost; and the date discrepancy flagged.