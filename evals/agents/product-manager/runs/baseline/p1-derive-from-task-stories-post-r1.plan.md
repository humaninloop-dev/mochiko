FILES-READ: `.mochiko/specs/time-and-attendance/spec.md`, `FEATURES.md`, `.mochiko/features/FEAT-001-rota-building.md`, `.mochiko/features/FEAT-002-staff-availability.md`, `.mochiko/features/FEAT-003-time-off-requests.md`, `.mochiko/specs/index.md`, `BACKLOG.md`, `README.md`

---

# Action plan — feature-map delta for `time-and-attendance`

## Phase 0 — Load the procedure and confirm the entry shape

**Do:** Invoke `mochiko:authoring-feature-map` before drafting anything — it holds the entry shape, the delta rules for touching delivered entries, the filter procedure, and how selection advice is framed. Everything in the phases below is drafted to that shape, not to my recollection of it.

**Read (already done):** the three delivered entries as the shape reference — each has `> Status:` + since-date header, `## Capability` (2–3 lines), `## Extent` (bullets, including explicit `Not:` lines), `## Relations` (`composes-with: FEAT-XXX — why`), `## Story trace` (`<spec-slug>: US-XXX`).

**Delegation:** one disposable `Explore` subagent, `model: haiku`. Brief: "List every file under `.mochiko/` and every top-level file in the working directory, including dotfiles and any `template`/`_template`/`convention` files. Return paths only, no interpretation." I check the return against the seven files I already read; if it names anything I have not opened — a feature-entry template, a map-writing convention doc, a prior spec's derivation note — I read it myself before drafting, because entry shape and ID allocation depend on it. I do the reading and the judgement; the subagent only enumerates.

**Flag now:** story IDs collide across specs — `US-001` names a different story in `rota-basics`, `availability`, `time-off` and now `time-and-attendance`. The three existing traces are unqualified (`rota-basics: US-001, US-002…` happens to carry the slug; the others do too). I will keep the slug prefix and qualify every new trace as `time-and-attendance: US-00X` so no trace is ambiguous.

---

## Phase 1 — Read the existing map before proposing anything

**Do:** Establish what the product already does, so I extend real entries rather than mint near-duplicates. From the three delivered entries, two lines are directly load-bearing for this spec:

- FEAT-001 extent says *"Not: copying a previous week into a draft — every week starts empty."*
- FEAT-001 extent says *"Not: any notification to staff on publish — staff open the app to see the week."*

Two of this spec's stories (US-007 copy last week, US-005 SMS reminder) land exactly on those two exclusions. Neither is a new time-and-attendance capability; one is a delta to a delivered entry, one is a separate capability the map has explicitly disclaimed. That decides their handling in Phase 3 before I invent anything.

**Also establish:** nothing in FEAT-001/002/003 gives a person *contracted hours*. Overtime (US-009, and the `overtime hours` column in US-004) has no basis in the product today. This is a gap I carry into Phase 4 as an open question, not something I paper over with a flattering extent line.

---

## Phase 2 — Reject the engineering lead's carving, in writing

**Do:** Write a verdict on the "Proposed features (for the map)" section rather than silently ignoring it, because it is a good-faith contribution from a named colleague and the founder will see the delta.

Verdict: **rejected, both options.**

- *Sprint 14 — clock-in foundations* / *Sprint 15 — hours and export* / *Nice-to-haves* are delivery containers. Their stated organising principle is "each feature lands in one sprint." A capability entry has to still be true and still be nameable after Sprint 14 and Sprint 15 are history; "Sprint 14" is not something the product does. "Nice-to-haves" is a priority bucket holding a defect, an infrastructure task, a rota improvement, an overtime flag, and a payroll integration — five unrelated things with no shared capability.
- The single *Time & attendance* entry fails the other way: its extent would have to cover clocking, breaks, manager corrections, overtime, sign-off, and export. That does not state in three lines, so it splits.

I will name the engineering lead's sprint plan as still valid *as a sprint plan* — the rejection is about the map, not the sequencing, and Phase 6 preserves the sequencing insight (US-008 first) as advice.

**Write:** this verdict into the derivation note (Phase 5), not into a feature entry.

---

## Phase 3 — Filter every story, one written verdict each

**Do:** Rule on all eleven claims on the map. Verdicts are recommendations; the user rules.

| Story | Verdict | Reasoning |
|---|---|---|
| US-001 clock-in/out | **accept** → new FEAT-004 | The core new capability: a person records that they worked. |
| US-002 breaks | **accept** → FEAT-004 | Same capability, same breath — a break is an interruption of the clocked period, not a separate thing the product does. |
| US-003 manager fixes a time | **accept** → FEAT-005 | The corrected value is what gets approved and exported; it belongs with the week's hours, not the tap. Noted as arguable — see Phase 4 stop. |
| US-004 CSV export | **accept** → new FEAT-006 | Distinct hook (the accountant, off-system), distinct boundary. |
| US-005 SMS reminder | **accept as its own capability, defer** → FEAT-007, status `proposed`, not selected | Real capability, and FEAT-001 explicitly disclaims notifications, so it cannot fold in as a footnote. But it is not time-and-attendance: it does not touch clocking, hours, or export, and it introduces an outbound-messaging dependency (Twilio, per-message cost, staff phone numbers, opt-out) the product has never had. Recording it keeps it honest; selecting it is the user's call. |
| US-006 UTC times, Manchester | **reject from the map** → `BACKLOG.md` | A defect in delivered FEAT-001, not a new capability. FEAT-001 already claims shifts with start and end times; the product is failing that claim, and the fix restores the existing entry rather than extending it. Flagged as **urgent and coupled**: clock-in has to compare a tap against a scheduled shift time (US-001's 30-minute window). Capturing attendance on top of broken timezone handling produces wrong hours that then get signed off and sent to an accountant. |
| US-007 copy last week | **reject as a new feature; accept as a delta to FEAT-001** | FEAT-001's extent already says the product does *not* do this. Minting a feature here would duplicate a delivered entry. |
| US-008 shifts → Postgres | **reject from the map** → architecture/backlog work | No capability changes when the table moves; a user cannot tell. It is the same work as the open backlog item about the reporting sidecar copy missing days. Recorded in the derivation note as a **prerequisite** of FEAT-004, because the story's own reasoning is correct: clock-in records need something reliable to join to. |
| US-009 overtime flag | **accept with a gap** → FEAT-005 | Same breath as the week's hours view. But it depends on contracted hours, which the product does not have — see Phase 4. |
| US-010 sign-off before export | **accept** → FEAT-005 | Approval is the state a week's hours reach; it gates FEAT-006. |
| US-011 Xero | **reject from the map** | The spec itself says not this release. It does not become a `proposed` entry on the strength of "so it is not forgotten" — it is remembered in the spec, and it will appear as an explicit `Not:` line in FEAT-006's extent so nobody reads the export entry as payroll integration. |

**Refuse:** I will not drop US-005, US-006, US-007, US-008 or US-011 quietly just because they are off-intent or off-map. Each gets its line above, and each rejection names where the work goes instead.

**Boundary:** these verdicts are about what earns a place on the map. I am not touching how the stories are written — the acceptance criteria, the vagueness in "a big clock-in button", "hours" left undefined in US-004. That is the analyst's craft next week, and I will say so rather than pre-empt it.

---

## Phase 4 — Stop for the founder's ruling on three open questions

Three calls are genuinely the user's, not mine. I would present them together, state my default, and continue planning under the default so the delta is drafted either way.

**Q1 — Is export its own feature, or part of approval?**
My recommendation: separate (FEAT-005 and FEAT-006). Folding export into approval pushes that extent past three lines and mixes two audiences (manager in-app, accountant off-system).
- *If the founder says fold:* FEAT-006 disappears, its extent bullets and its Xero `Not:` line move into FEAT-005, and I re-check the merged extent against the three-line bar — if it breaks, I bring the split back with the failed draft as evidence.
- *Default I proceed under:* separate.

**Q2 — Do manager corrections (US-003) sit with capture or with review?**
My recommendation: with review (FEAT-005), because the manager works from the week's hours view and the corrected value is what gets approved.
- *If the founder says capture:* the correction bullet and the "original stays visible" line move to FEAT-004, and FEAT-005's extent gains a line saying the hours it approves may have been corrected upstream.
- *Default:* review.

**Q3 — Overtime has no basis in the product.** Nothing today records a person's contracted hours; US-009 and the `overtime hours` column in US-004 both assume it.
- *If the founder confirms contracted hours are in scope:* it is a small capability of its own (a contracted weekly hours figure per person, set by a manager) — either a fourth entry or an extent line on FEAT-005, and I would draft it rather than let FEAT-005 claim overtime it cannot compute.
- *If out of scope:* overtime comes out of FEAT-005's extent and out of FEAT-006's column list, both as explicit `Not:` lines, and US-009 and part of US-004 go back to the ops lead as blocked.
- *Default I proceed under:* out of scope for now, recorded as `Not:` lines plus a flagged open question for the 2026-06-15 review — because claiming overtime the product cannot derive is exactly the flattering extent I refuse to write.

I would not wait on these; the spec is explicit that the derivation must not block on the review, and the founder wants the delta this week.

---

## Phase 5 — Write the delta

### 5a. New entries

**`.mochiko/features/FEAT-004-shift-time-capture.md`** — status `proposed`
- *Capability:* A staff member records when they actually started and finished a shift, and the breaks within it, from the staff app.
- *Extent:* clock-in/clock-out against a scheduled shift, available from 30 minutes before it starts; breaks started and ended while clocked in, deducted from the worked period; one open clock-in per person at a time. `Not:` clocking in without a scheduled shift. `Not:` location or geofence checks on the tap. `Not:` manager correction of a recorded time — that is FEAT-005.
- *Relations:* `composes-with: FEAT-001` — a tap is matched to a published shift; `feeds: FEAT-005` — captured periods become the week's hours.
- *Story trace:* `time-and-attendance: US-001, US-002`

**`.mochiko/features/FEAT-005-weekly-hours-approval.md`** — status `proposed`
- *Capability:* A manager reviews a week's worked hours per person, corrects wrong times, and approves the week.
- *Extent:* worked hours per person per week derived from captured periods; a manager edits a clock-in or clock-out with the original value still shown; per-person approval marks the week ready. `Not:` overtime against contracted hours — the product records no contracted hours (pending Q3). `Not:` staff challenging or acknowledging a correction.
- *Relations:* `depends-on: FEAT-004`; `gates: FEAT-006`.
- *Story trace:* `time-and-attendance: US-003, US-010` (`, US-009` only if Q3 resolves in scope)

**`.mochiko/features/FEAT-006-payroll-hours-export.md`** — status `proposed`
- *Capability:* A manager exports an approved week's hours as a CSV for the accountant.
- *Extent:* one file per site per week, one row per person — name, site, role, hours; only approved weeks export. `Not:` sending hours to a payroll system — no Xero or equivalent integration (US-011). `Not:` pay rates or gross pay — the product holds no rates. `Not:` an overtime column (pending Q3).
- *Relations:* `depends-on: FEAT-005` — unapproved weeks cannot export.
- *Story trace:* `time-and-attendance: US-004`

**`.mochiko/features/FEAT-007-shift-reminders.md`** — status `proposed`, explicitly **not selected for this release**
- *Capability:* Staff are reminded of a shift shortly before it starts, outside the app.
- *Extent:* an SMS an hour before a published shift, naming time and site. `Not:` in-app or push notification. `Not:` any notification on publish — FEAT-001's exclusion still stands for that.
- *Relations:* `composes-with: FEAT-001`; supersedes FEAT-001's "no notification" line only if built.
- *Story trace:* `time-and-attendance: US-005`
- I would draft this entry but hold it behind the same founder ruling as Q1–Q3: if the founder would rather it stay a story until it is scheduled, the file is not written and US-005 stays in the spec with its verdict recorded. *Default: write it as `proposed`, since the map's own status vocabulary allows a proposed entry and an unrecorded capability is how things get forgotten.*

Each new entry's header follows the existing convention but with a `proposed` status and no since-date, plus one line noting the derivation was made on 2026-06-11 ahead of the 2026-06-15 story review — so a reader knows these are a hypothesis to be confirmed or corrected against the reviewed stories, not settled fact.

### 5b. Updates to a delivered entry

**`.mochiko/features/FEAT-001-rota-building.md`** — two changes, both flagged as pending the founder's ruling since this is a delivered entry:
- Replace the `Not: copying a previous week into a draft` line with an in-scope bullet marked as committed-not-yet-delivered: copying the previous week into an empty draft (US-007) — so the entry shows both what the product does and what it now owes.
- Add relations to FEAT-004 (a tap matches a published shift) and, if FEAT-007 is written, note the notification exclusion is under challenge.
- I will **not** edit FEAT-001's extent to describe the timezone defect — a broken claim is a defect against the entry, not a narrowing of it.

### 5c. Index and map table

**`FEATURES.md`** — append rows for FEAT-004/005/006 (and 007 if written), status `proposed`, with one-line hooks in the same voice as the three existing rows.

**`.mochiko/specs/index.md`** — update the `time-and-attendance` row: capabilities touched → `FEAT-004, FEAT-005, FEAT-006, FEAT-001 (delta)` (+ FEAT-007), status → derivation drafted, pending story review.

### 5d. The derivation record

**`.mochiko/specs/time-and-attendance/feature-derivation.md`** — the eleven verdicts from Phase 3 verbatim, the rejection of the sprint carving from Phase 2, the three open questions from Phase 4 with their defaults, the US-008 prerequisite note, and the selection advice from Phase 6. No prior spec folder has such a file, so the filename is a convention I am introducing; I would say so and take a correction cheaply.

### 5e. What I would not write myself

Two lines belong in `BACKLOG.md`, which is the ops lead's store, not mine — US-006 (Manchester UTC defect, marked urgent because it corrupts clock-in matching) and US-008 (shifts table to Postgres, linked to the existing SQLite sidecar item). I would draft both lines in the derivation note and stop for confirmation before touching `BACKLOG.md`. If the founder prefers ops to place them, the drafts stay in the derivation note and I say so in the report.

---

## Phase 6 — Selection advice, with what deferring costs

Recommendation only — the founder selects. Ordered by product need and dependency, not by the ops lead's P-numbers:

1. **US-006 timezone fix and US-008 Postgres move first, before any capability work.** Neither is on the map, both block honest capture. *Cost of skipping:* attendance records matched against wrong-by-an-hour shift times, on a table the reporting sidecar has already lost a day from twice — wrong hours, approved by a manager, sent to an accountant. That is worse than not shipping clock-in.
2. **FEAT-004 shift time capture.** The thing the two churning groups actually named ("does clock-in").
3. **FEAT-005 weekly hours approval.** *Cost of deferring:* FEAT-004 alone replaces paper sign-in sheets with a data set nobody can correct — a manager who cannot fix a forgotten tap will go back to WhatsApp within a week.
4. **FEAT-006 export.** *Cost of deferring:* the Sunday-evening spreadsheet retyping — the pain in the intent — survives untouched; capture without export moves the typing rather than removing it.
5. **FEAT-001 copy-last-week delta (US-007).** Small, unrelated to the churn risk, but the highest-value-per-effort item in the spec for existing customers. *Cost of deferring:* nothing breaks; managers keep rebuilding the week.
6. **FEAT-007 reminders (US-005).** *Cost of deferring:* no-shows continue and the stated cause goes unaddressed. *Cost of doing it now:* it pulls a paid outbound-SMS dependency, phone-number handling, and opt-out into a release whose job is to stop two groups churning.
7. **Overtime and Xero:** not selected — one blocked on Q3, one out of release by the spec's own words.

---

## Phase 7 — Report

I would report: the four proposed entries and the one delta to a delivered entry, with the paths written; the eleven verdicts and the rejection of the sprint-shaped carving with reasons; the three open questions and the defaults I proceeded under, called out as the founder's to rule on; the two items handed to the backlog and not yet written there; the contracted-hours gap that makes overtime unbuildable as specified; and the standing caveat that this derivation is a hypothesis made ahead of the analyst's 2026-06-15 review — if a reviewed story conflicts with a framing here, the story wins and I redraft. I would not grade my own map writes; if an assessment of the delta is wanted, it goes to a separate reviewer.