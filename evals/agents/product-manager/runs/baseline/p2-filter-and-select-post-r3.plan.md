FILES-READ: `.mochiko/specs/shift-cover/spec.md`, `FEATURES.md`, `README.md`, `BACKLOG.md`, `notes/pilot-northgate.md`, `.mochiko/specs/index.md`, `.mochiko/specs/timesheets-export/spec.md`, `.mochiko/features/FEAT-001-rota-building.md`, `.mochiko/features/FEAT-002-staff-availability.md`, `.mochiko/features/FEAT-003-time-off-requests.md`, `.mochiko/features/FEAT-004-time-clock.md`, `.mochiko/features/FEAT-005-timesheets.md`, `.mochiko/features/FEAT-006-open-shifts.md`

---

# Action plan — shift-cover: filter, map delta, selection advice

## Phase 0 — Two things I would stop on before writing anything

**Stop A — the pilot date has already passed.** The founder's note asks for shift cover live by **2026-08-24**, "six weeks from the shift-cover kick-off" (spec opened 2026-07-13). Today is **2026-09-11**. The date is ~2.5 weeks gone and the spec is still sitting at `specify` with the filter unrun. I will not silently re-plan against a dead date.

- What I'd confirm with the founder: is 2026-08-24 slipped and re-set (to what?), already renegotiated with Northgate, or is the note simply stale and the pilot is live on something I can't see?
- Branch — *date re-set to a new one*: same ordering advice, re-costed against the new runway.
- Branch — *Northgate already went live partially*: the map is lying about what's delivered; I'd re-derive from what actually shipped before proposing anything.
- Branch — *pilot renegotiated to a smaller scope*: the selection collapses to the single highest-value mechanism plus its notification.
- **My stated default while planning on:** the date is a real commitment that slipped, the remaining runway is short, and ordering advice matters more than a full-scope plan. I plan the full filter regardless — the map work is true whatever the date does.

**Stop B — FEAT-006 means something different from US-002.** Detailed in Phase 3; it needs a founder ruling because the founder minted that entry.

## Phase 1 — Confirm I'm not about to duplicate something already on the map

Already done by hand for the six feature entries and both specs (above). Two gaps I'd close before writing:

- **Delegate** a disposable `Explore` subagent, **model: haiku**, one gap per spawn:
  - *Brief 1:* "Search every file in this workspace for the words swap, cover, claim, notification, push, notify, agency, feature flag, per-site toggle. Return file, line number, and the exact line. No interpretation." — I check the return for any prior capability, defect, or convention about cover or notifications I haven't seen, and for whether a per-site enablement mechanism already exists somewhere.
  - *Brief 2:* "Search every file for PDF, print, Safari, download, export. Return file, line and exact line." — this feeds the US-006 map-truth question below.
- I do **not** delegate the reading of the story list or the feature entries — deciding what earns a place on the map is interpretive, and missing an entry would silently change a verdict.

## Phase 2 — Run the filter: a written verdict for all nine stories

I'd draft this as the filter section of the spec (`.mochiko/specs/shift-cover/spec.md`, appended under a new `## Feature filter` heading, with today's date and my seat as author). Every story gets a line, including the ones I say no to.

| Story | Verdict | Where it lands | Reason I'd write down |
|---|---|---|---|
| US-001 offer my shift | **accept** | new **FEAT-007 — Shift swap** | A real capability in the product's own language: a published shift changes hands between staff with manager approval. Nothing on the map does this — FEAT-001 explicitly says it doesn't. |
| US-002 post an open shift | **accept** | **FEAT-006**, refined (see Phase 3) | Distinct initiator and distinct supply: manager→staff, first claim wins. Not the same capability as US-001. |
| US-003 eligibility | **accept, not as a feature** | extent + relations on FEAT-006 *and* FEAT-007 | "Only people who can actually work it see it" is a rule that governs both mechanisms; it isn't a capability anyone names on its own. Minting it would be a pseudo-feature for delivery convenience. It creates real relations: role from FEAT-001, availability FEAT-002, approved leave FEAT-003. |
| US-004 push notification | **accept** | new **FEAT-008 — Staff push notifications** (recommended) | The product today has *no* channel to staff — FEAT-001's extent says so in as many words. This is a new delivery capability that will outlive cover (publish, time-off decisions will want it). **Alternative I'd put to the founder:** keep it as an extent line on FEAT-006/007 instead, if the intent is genuinely cover-only and forever. My default is the standalone entry; I'd flag the thinness risk honestly. |
| US-005 weekly cover summary | **defer, on the map as owed work** | `pending` work row on FEAT-007 | Two counters on an existing view. Too thin to be a capability, too real to drop. It goes on the entry as a pending increment so the map shows what's committed and unshipped. |
| US-006 Safari PDF blank | **reject from the map** | `BACKLOG.md`, Open | A defect, not a capability. Routing it there is this workspace's own convention. **But it raises a map-truth flag** — see Phase 4. |
| US-007 publish the week | **reject as new** | no map change | Already delivered inside FEAT-001 ("Publishing makes the week visible to staff and locks it"). The right answer to Northgate is a confirmation, not a build. I write the verdict rather than dropping the line, so the founder can quote it back to them. |
| US-008 per-site switch | **accept, not as a feature** | extent line + work row on FEAT-006/007 | A rollout control on the cover capability, not a capability of its own. Cheap, and it's how Northgate's three cafés go live one at a time. |
| US-009 swapped hours on the taker's timesheet | **accept, not as a new feature** | extent + relation change on **FEAT-005**, and a hard constraint on FEAT-007 | This is correctness of an existing capability, not a new one. **And it is the most important thing in this spec** — see Phase 5. |

I would not touch the analyst's stories, their wording, or their acceptance criteria. If the analyst holds that a story I rejected is load-bearing — most likely US-006 or US-007 — that goes to the founder as a stated disagreement, not resolved by me editing anything.

## Phase 3 — Stop B: the FEAT-006 collision (founder ruling)

FEAT-006 is on the map as *"a manager sends an unfilled shift to a **staffing agency** and the agency fills it"* — minted by the founder 2026-06-30, marked `unrefined`, name and hook only, with the stated expectation that a spec's derivation fills it. US-002 uses the same words ("open shift") for a materially different capability: **internal** staff at the site claiming it.

**What I'd put to the founder, before writing:**
- **Option A (my recommendation):** refine FEAT-006 into *Open shift claiming* — internal supply, first eligible claim, manager approves — and record the agency route as explicitly **not in extent**, preserved as a named future capability so it isn't silently deleted. The founder's own note invited a derivation to fill the entry, and the evidence says internal supply is what Northgate needs: two-thirds of their WhatsApp traffic is a manager hunting for someone *on their own team*, with no mention of agencies.
- **Option B:** leave FEAT-006 as the agency capability and mint a separate entry for internal claiming. Cost: two entries called "open shifts" that a new engineer cannot tell apart. I'd argue against it.
- **Option C:** fold both US-001 and US-002 into one "Shift cover" entry. **I'd reject this and say why:** the extent won't state in about three lines (two initiators, two eligibility paths, approval, per-site switch), and FEAT-006 already exists for the open half — collapsing it would throw away a real entry.

Default if no ruling arrives: **Option A**, written with the agency line preserved verbatim inside the entry so nothing is lost and the founder can reverse it in one edit.

## Phase 4 — Map-truth flags I'd raise rather than paper over

1. **The PDF nobody wrote down.** US-006 says customers print the published week; FEAT-001's extent never mentions printing or a PDF at all. Either the map understates what rota building does, or the print lives somewhere unmapped. I'd flag it to the founder and eng as an open question, propose a one-line extent addition to FEAT-001 **only once someone confirms it**, and refuse to write the line on inference alone — a flattering extent is worse than a gap.
2. **Two "Not" lines that cover overturns.** FEAT-001 says *"Not: a manager changing who holds a published shift other than by re-opening the week"* and *"Not: any notification to staff on publish or change."* Both stop being true when this work lands. I'd revise them — on delivery, not on acceptance — to point at FEAT-006/007/008 rather than quietly deleting them, so the history of what the product couldn't do stays readable.
3. **The publish lock.** FEAT-001 says publishing *locks* the week. Every story here rewrites a published shift without re-opening. That's a real constraint for engineering to design against; I name it and hand it over — how it's built isn't mine.

## Phase 5 — The cross-feature consequence I'd escalate: US-009 is a pay bug waiting

Three entries line up and the conclusion is sharp:

- FEAT-004 (delivered): *"Not: clocking in without a scheduled shift — the button does not appear."*
- timesheets-export FR-003 (**in flight, cycle 2 of 3**): hours count against the shift the person was *scheduled* on; an unmatched clock-in is marked `unscheduled` and **excluded from the approved total**.
- US-001's own criterion: after approval *"the published rota shows the colleague on the shift."*

So: **an approved swap must rewrite the holder of the published shift, not overlay it.** If it overlays, the person who covered can't clock in at all, and if they somehow do, their hours are dropped from the export — they don't get paid, during a reference customer's pilot. US-009 isn't a P2 convenience; it's the condition under which swap is safe to ship.

I'd write this as a hard constraint on FEAT-007's extent, add a `depends-on FEAT-005` relation, extend FEAT-005's extent to state that a swapped shift's hours belong to whoever held it after approval, and **flag the timing to the founder and eng**: timesheets-export is mid-build, so this lands as a coordination item now rather than a surprise in cycle 3.

## Phase 6 — Write the map delta

Files I would write:

- **`.mochiko/features/FEAT-006-open-shifts.md`** — refined per the Phase 3 ruling: capability restated as internal open-shift claiming; extent covering post, eligible visibility, first-claim ordering, manager approval, the unsuccessful claimant being told, and per-site enablement; `Not:` the agency route (preserved, with the founder's original hook quoted and dated); relations to FEAT-001, FEAT-002, FEAT-003; story trace `shift-cover: US-002, US-003, US-008`; status `proposed` until selection.
- **`.mochiko/features/FEAT-007-shift-swap.md`** — new. Capability: a staff member offers a published shift to eligible colleagues at their site; a colleague takes it; the manager approves or declines and the published rota is rewritten to the taker. Extent includes the decline path staying visible to the offerer, eligibility, per-site enablement, and the constraint that approval reassigns the published shift; `Not:` cover across sites (staff belong to exactly one site per FEAT-001), `Not:` automatic approval. Work rows: `pending` — weekly swap and claim counts on the manager's week view (US-005). Relations: FEAT-001, FEAT-002, FEAT-003, FEAT-005. Trace `shift-cover: US-001, US-003, US-005, US-008, US-009`.
- **`.mochiko/features/FEAT-008-staff-notifications.md`** — new, `proposed`, per the Phase 2 default. Capability: the app tells a staff member on their phone about something that needs them. Extent scoped honestly to what this spec actually buys: an eligible offer or open shift, naming site, day and time, within a minute. `Not:` publish or rota-change notifications (still true of FEAT-001 today). Trace `shift-cover: US-004`.
- **`.mochiko/features/FEAT-005-timesheets.md`** — extent + relations edit per Phase 5. Trace gains `shift-cover: US-009`.
- **`.mochiko/features/FEAT-001-rota-building.md`** — *no edit yet*; the two "Not" lines and the possible PDF line are queued for delivery/confirmation, recorded in the spec's filter section so they aren't forgotten.
- **`FEATURES.md`** — new rows for FEAT-007 and FEAT-008, FEAT-006's hook rewritten, FEAT-007's pending work row shown as a sub-line in the table's existing style, statuses unchanged elsewhere.
- **`.mochiko/specs/index.md`** — shift-cover row: capabilities touched becomes `FEAT-006, FEAT-007, FEAT-008, FEAT-005`; status advanced to reflect filter complete, selection pending the founder.
- **`.mochiko/specs/shift-cover/spec.md`** — append the filter verdict table and the flags from Phases 3–5.
- **`BACKLOG.md`** — add US-006: Safari prints the published week blank, two customers, support 2026-07-15.

No tests to write or run — this is a documentation layer with no code in the workspace, and I'd say so rather than inventing a check. The only verification available is a consistency pass: every accepted story appears in exactly one entry's trace, every rejected one has a written reason, and no entry's extent runs past about three lines. I would **not** grade my own map writes; I'd ask for a review seat to check them.

## Phase 7 — Selection advice for the founder (recommendation, not a ruling)

Framed against the pilot evidence, not story priority labels. The one thing I'd press hardest:

> **US-004 is labelled P2 and it is the story the pilot lives or dies on.** Northgate's staff open the app **twice a week**, almost always on publish day. A cover offer that waits for someone to open the app is a cover offer that expires unseen. Shipping either P1 mechanism without push reproduces WhatsApp's speed problem with worse reach.

**Recommended order:**

1. **Open-shift claiming (FEAT-006) + eligibility + push (FEAT-008)** — two-thirds of Northgate's 20–30 messages per site per week are a manager hunting for an unfilled shift, and their managers say the one thing that would change their week is not chasing people. This is the largest measurable dent, and it carries no payroll risk: nobody currently holds the shift, so nothing about timesheets changes.
2. **Per-site switch (US-008)** — small, ships alongside, and it's how three cafés go live one at a time instead of all at once.
3. **Shift swap (FEAT-007) + the timesheet correctness in US-009, together, never apart** — this is where the owner's disputes come from, so it's the higher-trust win, but it's also the one that can mis-pay people. If swap can't ship with US-009, my advice is that swap doesn't ship.
4. **Deferred:** the weekly summary (US-005), and the agency route on FEAT-006 entirely.

**Costs of each deferral, stated at the moment of choice:**
- Deferring swap: the disputes that reach the owner keep reaching the owner through the pilot, and staff-to-staff giveaways stay on WhatsApp — roughly a third of the message volume. The reference-customer story is "half the problem".
- Deferring push: both mechanisms technically work and practically don't; expect low claim rates and a pilot that reads as a failure of the feature rather than of the channel.
- Deferring the summary: no evidence at the end of the pilot about which of the three cafés is chronically short — the founder loses the thing that would justify the next release.
- Deferring the agency route: the founder's original FEAT-006 idea waits; nothing in Northgate's notes asks for it.
- **US-006 (the Safari defect) sits outside this order** — it's backlog, but it's two customers and Northgate's managers print the week. I'd ask for an explicit yes or no on fixing it inside the pilot window rather than letting it drift.

**The alternative I'd put alongside:** if the founder judges that owner-visible disputes are the thing that loses the account, invert 1 and 3 — swap first. The cost of that inversion is that the swap path drags timesheet coordination with an in-flight build into the critical path, and the two-thirds of message volume the managers actually complain about goes untouched for longer. I'd show both and let the founder choose. **The selection is not mine to make.**

## Phase 8 — What I'd report

A short note to the founder: the passed date and what I need ruled on it; the FEAT-006 ruling I need; the nine verdicts in one table with reasons; the map delta as a list of files changed and why; three map-truth flags (the unmapped PDF, the two overturned "Not" lines, the publish lock); the pay-correctness escalation on US-009 with its timing against timesheets-export cycle 3; and the recommended build order with each deferral's cost next to it — ending with the two decisions that are the founder's, not mine.