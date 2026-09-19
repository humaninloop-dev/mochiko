FILES-READ: `.mochiko/specs/shift-cover/spec.md`, `.mochiko/specs/timesheets-export/spec.md`, `.mochiko/specs/index.md`, `.mochiko/features/FEAT-001-rota-building.md`, `.mochiko/features/FEAT-002-staff-availability.md`, `.mochiko/features/FEAT-003-time-off-requests.md`, `.mochiko/features/FEAT-004-time-clock.md`, `.mochiko/features/FEAT-005-timesheets.md`, `.mochiko/features/FEAT-006-open-shifts.md`, `FEATURES.md`, `README.md`, `BACKLOG.md`, `notes/pilot-northgate.md`

---

# Action plan — shift-cover: filter, map delta, and release advice

## Phase 0 — Load the procedure and close two read gaps

**Do:** Load `mochiko:authoring-feature-map` before writing anything — the entry shape, the delta rules for touching delivered entries, and the filter procedure come from there, not from my reading of the existing files. Everything drafted below follows the shape I observed in FEAT-001…FEAT-005 and will be reconciled against the skill.

**Delegate (two disposable `Explore` subagents, `model: haiku`, one gap each):**

1. *Brief:* "Search the whole workspace for any mention of swap, cover, claim, push notification, agency, or PDF/print outside `.mochiko/specs/shift-cover/` and `FEATURES.md`. Return file, line, and the quoted line. Do not interpret." *On return I check:* whether any prior commitment to these capabilities exists that I have not seen (a founder note, a README claim, a backlog row) — anything found gets read by me in full, because a prior commitment changes whether an entry is new or an extension.
2. *Brief:* "List every file under `.mochiko/specs/timesheets-export/` and quote any line mentioning swapped, reassigned, or transferred shifts, or the `unscheduled` marker." *On return I check:* whether the in-flight export run already handles a shift that changed holder, which decides whether US-009 is real work or a consequence.

I do the rest of the reading myself: the existing map is small and this is a completeness-sensitive read — if I miss an entry I mint a duplicate.

**Refuse here:** I will not delegate the judgement of which stories become features. That is the work of the card.

---

## Phase 1 — Establish the frame before touching entries

**Do:** State the capabilities this intent implies, in the product's nouns and verbs, and check each against the map before proposing anything:

- *an unfilled published shift gets claimed by staff at the site* → **collides with the existing FEAT-006 "Open shifts"**, which the founder minted on 2026-06-30 with an agency hook, unrefined. Extension candidate, not a new entry.
- *a staff member gives a published shift away to a colleague* → nothing on the map. New.
- *the product reaches staff on their phone* → nothing on the map; FEAT-001 explicitly disclaims it ("Not: any notification to staff on publish or change"). New.
- *who is eligible to take a shift* → not a capability of its own; it is an extent line on the two above and a set of relations to FEAT-002 and FEAT-003. **I will not mint an "Eligibility" feature** — it is a rule, not something a product person names in one breath.

**Stop 1 — FEAT-006 collision (founder ruling).** What I would confirm: FEAT-006's hook says an *agency* fills the shift; US-002 says *staff at my site* claim it. My reading is that "Open shifts" is the right name for the internal claim, and the agency route is a channel on the same capability, not a separate one.
- *Ruling A (my default):* refine FEAT-006 into the internal open shift; record agency fill as a `pending` work row so the founder's commitment stays visible and undelivered rather than being deleted.
- *Ruling B:* FEAT-006 stays the agency capability; the internal claim becomes a new entry and I rename FEAT-006 to something that does not read as the internal thing (e.g. "Agency fill"), because two entries both called open shifts is exactly the granularity failure I am supposed to prevent.

I proceed under A.

---

## Phase 2 — Write the filter verdicts, every one with its reason

**Write to:** a new `## Feature filter` section appended to `.mochiko/specs/shift-cover/spec.md` (the spec's own **Next** line says the product seat does this here; the timesheets-export directory shows no separate artifact convention). If the skill specifies a sibling file instead, it goes to `.mochiko/specs/shift-cover/features.md` and the spec gets a pointer.

| Story | Verdict | Reason as written |
|---|---|---|
| US-001 offer my shift | **Accept** → new entry, shift swaps | A durable capability: a published shift changes hands between staff. Nothing on the map does this. |
| US-002 post an open shift | **Accept** → extends FEAT-006 | Same capability the founder already named; refines an unrefined entry rather than duplicating it. |
| US-003 eligibility | **Accept** → extent, not an entry | Who can see an offer is a rule inside both cover capabilities; alone it is not a capability anyone would name. Becomes extent lines plus depends-on FEAT-002/FEAT-003. |
| US-004 push notification | **Accept** → new entry (see Stop 2) | The product has no way to reach staff today and FEAT-001 says so. This is a distinct capability with its own extent, not a footnote on the other two. |
| US-005 weekly cover summary | **Defer, no entry** | Two counters on the week view are too thin to be a capability, and there is no reporting surface to hang them on — the reporting sidecar was retired (BACKLOG, 2026-07-03). Revisit after the pilot when we know which numbers matter. Also flagged to the analyst: the story has no given/when/then. |
| US-006 Safari PDF blank | **Reject as capability work** → BACKLOG.md | A defect against existing behaviour, and the workspace convention puts defects on the backlog. Separately raises Stop 5. |
| US-007 publish the week | **Reject — already delivered** | FEAT-001, delivered 2026-03-14: "Publishing makes the week visible to staff and locks it." Nothing to build; the answer to Northgate is yes, it is in. I add a note that FEAT-001's existing trace to "time-and-attendance US-007" is a coincidence of numbering, not this story. |
| US-008 per-site switch | **Reject as capability** → BACKLOG.md as tooling (see Stop 3) | A rollout control for the pilot, not something the product does for a customer. Rejecting it from the map does not mean not building it — I recommend it in the first cut (Phase 5). |
| US-009 hours follow the taker | **Accept → extent line on the swap entry, not an entry** | If an approved swap moves the holder on the published week (US-001's own criterion), the export's FR-003 already counts hours against the scheduled person. This is a correctness claim to keep true, not a new capability. Conditional on the Phase 0 read (2). |

**Flag to engineering and the analyst, not a verdict:** if the implementation leaves the original holder on the shift and records the swap alongside, the in-flight timesheets-export run (cycle 2 of 3, FR-003) will mark the taker's clock-in `unscheduled` and drop their hours from the approved total. That is wrong pay. This needs to be settled before swaps are built, and it is the reason I will not let US-001 ship without US-009.

**Disagreement route:** if the analyst holds US-005 or US-008 to be load-bearing against my verdicts, it goes to the founder as a stated disagreement. I do not overwrite the analyst's story text or priorities, and the analyst does not rewrite these verdicts.

---

## Phase 3 — Propose the map delta (new and refined entries)

**Write to `.mochiko/features/FEAT-006-open-shifts.md` (refine in place):** status `proposed` until selected; capability — a manager posts a published shift nobody is on to eligible staff at that site, and approves one claim. Extent: visible only to same-site staff matching the role, not on approved time off, and within declared availability; claims ordered by arrival, the manager approves exactly one and the others are told; **not** sending the shift to an agency. Relations: depends-on FEAT-001, FEAT-002, FEAT-003; composes-with the notifications entry. Work rows: `pending` — an unfilled shift sent to a staffing agency and filled by them, not yet cut. Trace: `shift-cover: US-002, US-003`. Provenance line keeps the founder's 2026-06-30 minting and adds this derivation.

**Write to `.mochiko/features/FEAT-007-shift-swaps.md` (new):** capability — a staff member offers a published shift of theirs to eligible colleagues at their site; on the manager's approval the shift changes hands on the published week. Extent: same eligibility gate as above; approval moves the holder, decline leaves it with the offerer and shows them the decline; the taker's clock-in, timesheet and export follow the shift; **not** an offer to a named colleague and **not** a two-way shift trade — an offer goes to everyone eligible and gives the shift away. Relations: depends-on FEAT-001, FEAT-002, FEAT-003; composes-with FEAT-004, FEAT-005, notifications. Trace: `shift-cover: US-001, US-003, US-009`.

*Granularity check I would run and report:* four extent lines is at my limit. I judge the third to be a consequence of the second, not a second capability, so this holds as one entry. If the founder reads the timesheet line as separate work, I split rather than let the extent sprawl.

**Write to `.mochiko/features/FEAT-008-staff-notifications.md` (new, subject to Stop 2):** capability — the product reaches a staff member on their phone when something needs them, starting with cover. Extent deliberately narrow: a push naming site, day and time within a minute of posting, to eligible staff only; **not** publish or rota-change notifications; **not** email or SMS, no digest, no quiet hours. Relations: composes-with FEAT-006, FEAT-007. Trace: `shift-cover: US-004`.

**Stop 2 — notifications as an entry (founder ruling).** What I would confirm: this is the one entry at risk of becoming a platform rather than a capability. *Default:* its own entry with the narrow extent above, because it will be extended later (publish notifications are an obvious next ask) and burying it inside two cover entries would duplicate it. *Alternative ruling:* one extent line on each cover entry and no separate feature — I would take that and record why.

**Write to `FEATURES.md`:** rows for FEAT-006 (status updated), FEAT-007, FEAT-008, plus the indented `pending` agency row under FEAT-006, in the existing row format.

**Write to `.mochiko/specs/index.md`:** fill shift-cover's "Capabilities touched" with FEAT-006, FEAT-007, FEAT-008 (+ FEAT-001 for the staged delta below) and move its status on from "filter and selection pending" once the founder rules.

---

## Phase 4 — Staged deltas to delivered entries (written, but not yet true)

FEAT-001 currently says *"Not: a manager changing who holds a published shift other than by re-opening the week."* Cover contradicts that line — but only once cover ships. **I will not rewrite a delivered entry's extent to describe work that does not exist.** So:

- I draft the FEAT-001 delta (replace that line with a pointer to cover; add composes-with FEAT-006/FEAT-007; add `shift-cover: US-001, US-002` to its trace) and record it as **pending on delivery**, with the trigger named: applied the day the first cover capability is marked delivered.
- Same staging for FEAT-003's extent (approved time off will also hide offers, not just block the draft), FEAT-002's relation, and FEAT-004/FEAT-005's relations to FEAT-007.

**Write to:** the pending deltas are recorded in the spec's feature-filter section; the entry files themselves are edited only at delivery.

---

## Phase 5 — Selection advice for the founder

**Stop 4 — the date has already passed.** Today is 2026-09-11; Northgate's date is 2026-08-24. The spec has sat at "stories reviewed, filter pending" since mid-July. Before advising on a cut I would confirm: has the date moved, what has Northgate been told, and is this a recovery plan or a fresh date? I will not produce a plan aimed at a date that is eighteen days gone without saying so. *Default I proceed under:* the ordering below is unchanged by which date we are aiming at — it is the ordering of smallest-thing-that-makes-cover-real — so I present it as a recovery cut and put the date question at the top of my report.

**What I would recommend, with the evidence:**

- **Cut 1 — open shifts, whole:** US-002 + US-003, plus US-004 notifications, plus the US-008 switch as tooling. Grounds: two-thirds of Northgate's 20–30 messages per site per week are a manager hunting a shift nobody is on, and their managers' stated single ask is not chasing people. It is also the cheaper half — no shift changes hands, so no decline path and no timesheet consequence — and it proves the shared claim-and-approve machinery before the harder half lands on it.
- **Cut 2 — swaps, shipped whole:** US-001 + US-009 together, never US-001 alone. A swap that leaves pay on the wrong person's timesheet is worse than WhatsApp.
- **Not now:** US-005 (defer), US-006 to the backlog (small, customer-reported twice, and Northgate's own managers print the week — worth doing but it is not capability work).

**Deferral costs, stated at the moment of choice:**
- Deferring swaps: one-third of the WhatsApp traffic stays, and every dispute that reaches the owner ("I thought Sam was covering me") is untouched — those all come from the staff-to-staff kind. And Northgate's ask was worded *"shift cover live for their staff"*; open shifts only lets staff *take*, not *give*. **This is the strongest argument against my own recommendation and I would put it in front of the founder in those words.**
- Deferring notifications: staff open the app twice a week, almost always on publish day. An offer posted Thursday for Saturday is seen Monday. Cover fails silently and the pilot reads as "the app didn't help." This is why I recommend a P2 story into the first cut — I am not changing the analyst's priority, I am arguing the sequencing.
- Deferring eligibility: managers approve people on holiday or in the wrong role. Not optional at any date.
- Deferring the per-site switch: no way to roll out one café at a time across their three sites, which is how they asked to run the pilot.
- Deferring US-005: no evidence during the pilot of whether cover is working — mitigate by pulling the counts from the database by hand rather than building the view.
- Capacity note, not a decision: three engineers with timesheets-export mid-run at cycle 2 of 3.

**Stop 5 — the rota PDF.** US-006 is evidence the product prints a published week, and **no entry on the map claims that**. I would ask the founder whether printing is intended product surface. If yes, FEAT-001 gains an extent line at the next delta; if it is incidental, it stays off the map and the bug is just a bug. I will not write an extent line that claims a capability on the strength of a bug report.

**Stop 6 — the selection itself.** I recommend the ordering; I do not make the call. If the founder chooses swaps first on the strength of Northgate's wording, I re-cut: US-001 + US-003 + US-009 + US-004 in cut 1, open shifts in cut 2, and FEAT-006 keeps a `pending` work row so the map shows what we have committed to and not yet shipped.

---

## Phase 6 — Hand off, do not self-certify

**Do:** hand the written delta and verdicts to the founder and the analyst for review. I do not grade my own map writes. On the founder's ruling I apply the selected branch, set the chosen entries to `in-flight` with the shift-cover run named, add `pending` work rows for whatever was cut but committed, and update the specs index.

**What I would report:**
1. The date has passed — first question, before anything else.
2. The verdict table: 4 accepted onto the map (2 as entries, 2 as extent), 1 deferred, 3 rejected — each with its reason, none dropped silently.
3. The delta: FEAT-006 refined from the agency hook to the internal claim with agency kept as pending work; FEAT-007 and FEAT-008 new; FEAT-001/002/003/004/005 deltas drafted and held until delivery.
4. The recommended cut and order, with each deferral's cost, and the explicit counter-argument for swaps-first.
5. Six open decisions for the founder, and one cross-run flag to engineering about FR-003 and swapped shifts.