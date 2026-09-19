FILES-READ: `.mochiko/specs/shift-cover/spec.md`, `FEATURES.md`, `.mochiko/features/FEAT-001-rota-building.md`, `.mochiko/features/FEAT-002-staff-availability.md`, `.mochiko/features/FEAT-003-time-off-requests.md`, `.mochiko/features/FEAT-004-time-clock.md`, `.mochiko/features/FEAT-005-timesheets.md`, `.mochiko/features/FEAT-006-open-shifts.md`, `.mochiko/specs/index.md`, `.mochiko/specs/timesheets-export/spec.md`, `notes/pilot-northgate.md`, `BACKLOG.md`, `README.md`

---

# Action plan — shift-cover: filter, map delta, and release advice

## Phase 0 — Load the procedure, then stop on the date

**What I'd do.** Load `mochiko:authoring-feature-map` first and work from it, since the entry shape, the delta rules, and the filter procedure live there, not in my head. (In this plan-only run I can't load it, so everything below is shaped against what the six existing entry files evidence: Capability / Extent / Relations / Story trace, plus `live`/`pending` work rows as seen on FEAT-005.)

**The stop, immediately.** The card asks for advice on Northgate's **2026-08-24** date. Today is **2026-09-11**. That date is eighteen days gone, and `.mochiko/specs/index.md` still shows shift-cover sitting at `specify` with no capabilities touched. The filter and selection never happened, and the pilot commitment lapsed unremarked. I would not quietly re-plan around a date that has passed and call it advice.

What I'd put to the founder before anything else:

- Has the Northgate date moved, and to when? Is the pilot still on at all?
- The evidence I'd be reasoning from (`notes/pilot-northgate.md`, 2026-07-12) is two months old. Does "app opened twice a week", "20–30 WhatsApp messages per site per week", "disputes all come from staff-to-staff" still hold? If Northgate has been waiting since August, their tolerance and their workaround may both have changed.

**Branches.**
- *Date reset to a new deadline* → I re-run Phase 5's ordering against the actual weeks available and say plainly what fits.
- *Pilot cancelled or Northgate churned* → the map work in Phases 1–4 still stands and I'd complete it; Phase 5 becomes "what to build next" with no date pressure, and the sequencing argument changes, because push notification's urgency is partly a function of a hard date.
- *Founder says build to the original scope regardless* → I'd still write the lapse into the spec record, because an unexplained two-month gap between "reviewed" and "filtered" is the kind of thing that gets read later as nobody having noticed.

**Default I proceed under for the rest of this plan:** the date has slipped and a fresh window of roughly six weeks from today is being set. I'd say so explicitly in the deliverable rather than pretending 2026-08-24 is live.

## Phase 1 — Read the map before proposing anything

Already done in this session; I read all six feature entries, the specs index, the in-flight timesheets-export spec, the README conventions, the backlog, and the pilot note. The workspace is twelve files, so I'd not delegate the primary read.

**The one sweep I would delegate.** A throwaway `Explore` subagent pinned to `model: haiku`, brief: *"In this workspace, list every file and line that mentions any of: staffing agency, agency fill, PDF, print, printing, notification, push, notify, feature flag, per-site toggle. Quote the line with its file path and line number. No interpretation."* One gap, bounded, deterministic. On return I'd check that every hit it reports actually exists at the cited line, and that it found at least the ones I already know about (FEAT-006's agency hook, FEAT-001's "Not: any notification" line, US-004, US-006, US-008) — if it misses a known hit, I discard its output and sweep myself. I want this because two of my proposals below contradict existing written commitments, and I will not overwrite a commitment I haven't seen.

## Phase 2 — Filter each story, with a written verdict for every one

I'd write nine verdicts, one per story, none silent. Draft positions I'd bring:

| Story | Verdict | Reason I'd write down |
|---|---|---|
| US-001 Offer my shift for swap | **Accept — new capability** | A durable thing the product does: a published shift changes hands staff-to-staff under manager approval. Nothing on the map covers it; FEAT-001 explicitly disclaims it. |
| US-002 Post an open shift | **Accept — but onto the existing FEAT-006, re-derived** | Extending a real entry beats minting a near-duplicate. See the stop in Phase 3. |
| US-003 Eligibility | **Accept as extent, reject as its own feature** | "Only people who can actually work it see it" is a rule governing the two cover capabilities, not a capability anyone would name on its own. Minting "Cover eligibility" would be a pseudo-feature. It lands as an extent line on both entries plus relations to FEAT-002 and FEAT-003. |
| US-004 Push notification | **Accept — new capability** | The product has no notification channel at all today; FEAT-001's extent says so in as many words. This is a new channel, not a detail of cover. Also the load-bearing one for the pilot (Phase 5). |
| US-005 Weekly cover summary | **Accept as a deferred increment, not a slice** | Real but small and P3. Records as a `pending` work row so the map shows the debt. Constraint to flag: the reporting sidecar was retired 2026-07-03 per the backlog, so "where does a count live now" is an open question, not a free line. |
| US-006 Safari PDF blank | **Reject from the map — it's a defect** | A broken behaviour is not a new capability. Routes to `BACKLOG.md`. *But* it surfaces a map-honesty problem — see Phase 4. |
| US-007 Publish the week | **Reject — already delivered** | FEAT-001, extent line three, delivered 2026-03-14, traced to time-and-attendance US-007. The answer to Northgate is "yes, it is in", not a new entry. |
| US-008 Per-site switch | **Reject as a feature; accept as an extent line if rollout needs it** | A rollout control, not something the product does for a café. It is not nothing — Northgate is three sites and ops wants them staged — so it gets one extent line, not an entry of its own. |
| US-009 Swapped shift counts for the taker | **Accept as extent on FEAT-005 and the swap entry; reject as its own feature** | Not a capability — it's what "the shift changed hands" must mean if it means anything. And it is not optional; see Phase 4. |

**Stop for the founder's ruling on the verdicts**, specifically US-003, US-008 and US-009, where someone could reasonably want visible entries rather than extent lines. If the founder overrules me and wants eligibility as its own feature, I'd write it — but I'd record that I advised against it and why, so the map carries the reasoning. If the requirements analyst disagrees with any rejection — most plausibly holding US-008 load-bearing for the pilot — that goes to the founder as a stated disagreement. I would not edit the analyst's stories, and their verdicts on story quality aren't mine to touch.

## Phase 3 — The FEAT-006 stop

`FEAT-006-open-shifts.md` says: *"A manager sends an unfilled shift to a staffing agency and the agency fills it."* Minted by the founder 2026-06-30, marked unrefined, never built. US-002 describes the same underlying capability — filling a published shift nobody is on — through a different channel: the site's own staff, first claim, manager approves.

**What I'd confirm with the founder:** does US-002 *replace* the agency framing, or sit alongside it?

- *Replace (my recommendation)* → I re-derive FEAT-006 to the staff-claim capability and write the agency route into the extent as an explicit "Not:" line, so the idea is visibly parked rather than deleted. Keeps the ID, keeps the intent, no near-duplicate.
- *Alongside* → two entries. I'd push back: two ways to fill the same gap is one capability with two channels until the agency route is real enough to state an extent for, and agency fill involves a party outside the product with none of it built.
- *Agency is dead* → same re-derivation, and the "Not:" line says so.

**Default if I get no ruling:** re-derive FEAT-006, agency route recorded as explicitly not in.

## Phase 4 — Write the map delta

Files I would write:

**New — `.mochiko/features/FEAT-007-shift-swaps.md`**
Capability: a staff member offers a published shift of theirs to eligible colleagues at their site; a colleague takes it; a manager approves or declines; approval moves the shift's holder on the published week. Extent in three lines: who may see and take an offer (eligible = right role, not on approved time off, inside declared availability); approval moves the holder and the decline is visible to the offerer; enabled per site. Not: offers across sites, and not automatic approval. Relations: depends-on FEAT-001, composes-with FEAT-002 and FEAT-003 (eligibility), composes-with FEAT-005 (hours follow the holder). Story trace: shift-cover US-001, US-003, US-008, US-009.

**Rewritten — `.mochiko/features/FEAT-006-open-shifts.md`**
Capability: a manager posts a published shift nobody is on to eligible staff at the site; claims are ordered by arrival; the manager approves one and the unsuccessful claimants are told. Same eligibility extent line and relations. Not: agency fill. Story trace: shift-cover US-002, US-003, US-008. Status moves off `proposed (unrefined)`.

**New — `.mochiko/features/FEAT-008-push-notifications.md`**
Capability: the app pushes an event to a staff member's phone. First extent is cover events only — an eligible offer or open shift, naming site, day and time. Not: publish or rota-change notifications (that stays FEAT-001's stated gap). Relations: composes-with FEAT-006 and FEAT-007. Story trace: shift-cover US-004. I'd name the alternative for the founder — fold push into the two cover entries as extent lines instead — and recommend against it, because the next spec that wants to notify anyone will otherwise have nowhere to hang it.

**Edited — `.mochiko/features/FEAT-005-timesheets.md`**
Extent gains: hours for a shift that changed hands count for the person who worked it. Relations gain composes-with FEAT-007.

**Edited — `.mochiko/features/FEAT-001-rota-building.md`**
Its extent currently ends: *"Not: a manager changing who holds a published shift other than by re-opening the week."* That line goes false the day cover ships. I'd stage the amendment now and apply it at delivery, not before — the map states what the product does, not what it will do — and add relations to FEAT-006 and FEAT-007.

**Edited — `FEATURES.md`**
Three rows added/changed, plus `pending` work rows under FEAT-006/007 for the weekly cover summary and for whichever slice the founder defers in Phase 5, so the map carries the committed-and-unshipped debt.

**Edited — `.mochiko/specs/index.md`** — shift-cover's "Capabilities touched" becomes FEAT-006, FEAT-007, FEAT-008, FEAT-005, FEAT-001; status advances past `specify`.

**Edited — `BACKLOG.md`** — US-006 lands as a defect line with its support provenance (2026-07-15, two customers, Safari only).

**Two things I'd flag rather than fix.**

1. **US-009 is a hard conflict with work already in flight.** The timesheets-export spec's FR-003 says hours are counted against the shift the person was scheduled on, and a clock-in with no matching published shift is marked `unscheduled` and *excluded from the approved total*. If an approved swap does not rewrite the holder on the published rota, the person who actually worked the shift gets excluded from the export — they don't get paid. So US-009 is not a P2 nicety; it is a condition of shipping swaps at all while timesheets-export is building. I'd raise this with the founder and the export run's author before either ships, and I would not mark it deferred under any ordering that includes swaps.
2. **A truth gap in FEAT-001.** US-006 reports that printing the published week from Safari is broken — so the product prints a published week, and no entry on the map says it does. Either FEAT-001's extent gains a line or something is being sold that the map doesn't know about. That's a map correction, separate from the defect, and I'd propose it as such.

**I would not grade my own writes.** The delta goes to the founder for the ruling and, if there's a reviewer seat, to that seat for the check.

## Phase 5 — Selection advice for the pilot

Recommendation, ordered, with what each deferral costs. The selection is the founder's, not mine.

The pilot note carries a tension worth naming rather than smoothing over: two-thirds of WhatsApp traffic is the manager hunting for someone to cover an empty shift, and managers say the one thing that would change their week is not having to chase — that points at open shifts. But every dispute that reaches the *owner* comes from the staff-to-staff kind — that points at swaps. Volume and pain point at different features.

1. **Eligibility + open shifts + push notification** (US-002, US-003, US-004). The largest share of the traffic, the pain the managers themselves named, and the uncontested half — so the first thing the pilot ships is also the lowest-risk thing. Push is in this slice and not optional: Northgate's staff open the app twice a week, almost always on publish day. An open shift posted on Thursday for Saturday 2–8 reaches nobody through an app nobody opens. Ship cover without push and the pilot reads as "we built it and they didn't use it", which is the worst possible reading because it is wrong.
2. **Shift swaps + hours following the holder** (US-001, US-009, tied). Addresses the disputes reaching the owner. US-009 rides with it for the payroll reason above.
3. **Per-site switch** (US-008), if ops genuinely intends to stage three cafés — cheap, and pulling it forward into slice 1 is defensible. Ops's call, and I'd put it to them directly.
4. **Deferred:** weekly cover summary (US-005), recorded as a pending row, with the retired reporting sidecar noted as an open question.
5. **Not on the map:** the Safari PDF defect (to backlog) and publish-the-week (already delivered — the answer to Northgate is a confirmation, not a build).

**Costs of the deferrals, said at the moment of choice.** Deferring swaps means the disputes that reach the owner keep reaching the owner through the whole pilot, and the third of WhatsApp traffic that is staff giving shifts away stays on WhatsApp — the pilot is a partial answer and Northgate will experience it as one; I'd tell them that up front rather than let them find it. Deferring push means both cover features are likely to sit unused. Deferring the per-site switch means all three cafés go live together, with no way to pull one back. Deferring the cover summary costs nothing this release.

**If the founder wants swaps first instead** — a coherent position, since it's the pain that escalates — I'd support it on one condition: US-009 ships with it, and the export run is told before it finishes. And I'd note the cost: the manager-chasing that they told us is the thing they most want gone stays for the pilot.

## Phase 6 — Report

Back to the founder, in one piece:

- The date problem first, and what I need ruled.
- Nine verdicts, each with its reason, including the four stories that don't go on the map and why.
- The proposed delta as concrete file writes, with the FEAT-006 re-derivation called out as needing the founder's word since he minted it.
- The two flags: US-009 against the in-flight export contract, and FEAT-001 not claiming a print capability the product apparently has.
- The recommended order with deferral costs attached, stated as advice, with the alternative ordering and its cost, for the founder to choose.

No files are written until the Phase 0 date question and the Phase 3 FEAT-006 question come back.