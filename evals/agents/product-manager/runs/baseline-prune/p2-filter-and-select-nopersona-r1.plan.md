FILES-READ: `README.md`, `FEATURES.md`, `BACKLOG.md`, `notes/pilot-northgate.md`, `.mochiko/specs/index.md`, `.mochiko/specs/shift-cover/spec.md`, `.mochiko/specs/timesheets-export/spec.md`, `.mochiko/features/FEAT-001-rota-building.md`, `.mochiko/features/FEAT-002-staff-availability.md`, `.mochiko/features/FEAT-003-time-off-requests.md`, `.mochiko/features/FEAT-004-time-clock.md`, `.mochiko/features/FEAT-005-timesheets.md`, `.mochiko/features/FEAT-006-open-shifts.md`

---

# Action plan — shift-cover: filter, map delta, release advice

## Phase 0 — The thing I would raise before anything else

**Today is 2026-09-09. The Northgate date is 2026-08-24. It is sixteen days past.**

Nothing in the workspace shows cover work started: `.mochiko/specs/index.md:10` still has shift-cover at `specify`, with no capabilities touched; no `FEAT-*` entry carries a cover work row; the spec's own status line still says the filter and selection are pending, unchanged since the 2026-07-16 review. The pilot note frames 2026-08-24 as "six weeks from kick-off… ahead of the autumn term" — the term has now started.

I would not write a plan that quietly aims at a date that has already gone. So the release advice gets written as *recovery* advice, and I open the founder conversation with this rather than burying it.

**Stop #1 — what I would confirm with the founder, before writing the advice document:**

- *Is the 2026-08-24 commitment missed, and what has Northgate been told?*
- **Branch A (default, and what the files support): nothing shipped, the date is missed.** I proceed with the plan below — tightest credible loop, new date proposed with engineering, Northgate told now rather than later.
- **Branch B: something did ship and this workspace is stale.** I stop proposing, ask what is live at Northgate's three cafés, and re-run the Phase 2 filter against reality — the story classification would barely move, but the release advice would be rewritten entirely.
- **Branch C: the date was always aspirational / has been renegotiated.** Same plan, I just drop the recovery framing and slot the ordering against the real date.

I proceed under Branch A. Note that the Phase 2 filter and the Phase 3 map delta are the same under all three branches — only Phase 5 changes — so I would not block on this answer to do the rest.

---

## Phase 1 — Ground truth (reading, already done)

Read above. The four facts that drive every ruling that follows:

1. `FEAT-001-rota-building.md:18` — *"Not: a manager changing who holds a published shift other than by re-opening the week."* Cover is genuinely new ground, and this line goes false.
2. `FEAT-001-rota-building.md:17` — *"Not: any notification to staff on publish or change."* There is no notification capability anywhere in the product today.
3. `FEAT-006-open-shifts.md` is `proposed (unrefined)`, minted by the founder, hook only, and says outright that *"a spec's derivation fills it."* Its current hook is the **agency** channel; US-002 is the **own-staff** channel.
4. `timesheets-export/spec.md:12` FR-003 — hours count against *the shift the person was scheduled on*; a clock-in with no matching published shift is `unscheduled` and **excluded from the approved total**. This collides with swaps, and timesheets-export is in flight right now.

---

## Phase 2 — Filter the nine stories onto the map

This is the ruling table I would produce. Three of the nine are not capabilities at all, which is the main work of the filter.

| Story | Ruling | Lands where |
|---|---|---|
| US-001 Offer my shift for swap | **New capability** | New `FEAT-007 — Shift swaps` |
| US-002 Post an open shift | **Refines an existing entry** | `FEAT-006` — redirects it from agency-fill to staff-claim |
| US-003 Eligibility | **Not a capability** — it is the rule that makes 001/002 safe | Extent lines on FEAT-006 + FEAT-007; relations to FEAT-002, FEAT-003 |
| US-004 Push notification | **New capability** (default ruling — see below) | New `FEAT-008 — Staff push notifications` |
| US-005 Weekly cover summary | **Not-now work row** | `pending` row on FEAT-007, spanning FEAT-006 |
| US-006 Safari PDF blank | **Not a capability — a defect** | Off the map entirely → `BACKLOG.md` |
| US-007 Publish the week | **Already delivered** | No build. Story-trace line on FEAT-001 only |
| US-008 Per-site switch | **Not a capability — rollout behaviour** | Extent line on FEAT-006 + FEAT-007 |
| US-009 Swapped hours on taker's timesheet | **Not a capability — a correctness condition** | Extent on FEAT-007 + story-trace on FEAT-005 |

**The two rulings I hold least tightly**, and would put to the founder rather than just applying:

**US-002 vs. FEAT-006.** FEAT-006 is the founder's own row and says the agency fills the shift. US-002 says the site's own staff claim it. Same capability shape — *a manager gets an unfilled shift filled without chasing* — different channel. Because the entry is explicitly `unrefined` and awaiting a derivation, my default is to let the derivation fill it with the staff-claim channel, and preserve the agency idea as an explicit "Not:" line plus a `pending` row so it is parked, not deleted. The alternative is to leave FEAT-006 alone as a future agency capability and fold staff-claim into a single FEAT-007 "Shift cover" covering both US-001 and US-002. I prefer the first: it keeps one actor per entry, and Northgate's data says nothing about agencies.

**US-004's home.** It could be an extent line on both cover entries rather than its own row. I default to minting `FEAT-008` because it is shared by two capabilities, because FEAT-001 explicitly disclaims notifications so there is no existing home, and because the platform work behind it (device tokens, permissions, APNs/FCM) is real and should be visible on the map rather than hidden inside a bullet. If the founder wants a leaner map, it collapses to extent lines and I lose nothing but visibility.

**Quality notes I would send back to the requirements analyst** (not blockers, and I would not fix them myself — the analyst owns stories per `README.md:22`):
- US-005, US-007, US-008 have no acceptance criteria; US-008 has no independent test. US-006 and US-007 are not user stories at all.
- US-006 does not belong in this spec — it is an unrelated support report from 2026-07-15 that got swept into the story list.
- US-002's criterion "the other is told" is a notification, so a P1 story depends on P2 US-004. That dependency is unstated.
- Genuinely uncovered: can an offer be withdrawn or a swap undone? Is there a cutoff (offering a shift that starts in an hour)? Is eligibility re-checked at approval time, or can someone book time off between taking a shift and the manager approving? Is there a manager view of pending offers across all three cafés?
- **Cross-site cover is not in any story.** `FEAT-001-rota-building.md:13` says a staff member belongs to exactly one site, and every story says "at my site." Northgate runs three cafés with 38 staff and may well expect cover across them. I would raise this at Stop #1 as a scope question. Default: same-site only for the pilot — it matches the stories and the per-site WhatsApp behaviour in the pilot note.

---

## Phase 3 — Draft the map delta (proposed, **not applied** yet)

The card asks me to *propose* the changes. The live map is the founder's, so I would write these as an apply-ready delta inside the spec directory and hold the actual edits until Stop #2. Files that would eventually change:

**`.mochiko/features/FEAT-006-open-shifts.md`** — rewrite from placeholder to a refined entry: capability becomes a manager posting an unfilled published shift to eligible staff at the site, first claim on top, manager approves. Extent gets the eligibility rules from US-003 (role match, not on approved time off, inside declared availability), the US-008 per-site switch (off by default), and `Not: agency fill — parked`. Relations: `depends-on FEAT-001`, `composes-with FEAT-002`, `composes-with FEAT-003`, `composes-with FEAT-008`. Story trace `shift-cover: US-002, US-003, US-008`. Status moves `proposed (unrefined)` → in-flight when build starts.

**`.mochiko/features/FEAT-007-shift-swaps.md`** (new) — staff member offers a published shift of theirs; an eligible colleague at the site takes it; the manager approves or declines; on approval the published rota shows the new holder. Extent: same eligibility rules, the per-site switch, decline is visible to the offerer, and **the approved swap rewrites the holder on the published shift so worked hours follow the person who worked it** (US-009). Relations: `depends-on FEAT-001`, `composes-with FEAT-002/003/005/008`. Work rows: `pending` — cover activity counted on the manager's week view (US-005). Story trace `shift-cover: US-001, US-003, US-005, US-008, US-009`.

**`.mochiko/features/FEAT-008-push-notifications.md`** (new) — an eligible staff member gets a push naming site, day and time within a minute of an offer or open shift being posted; the unsuccessful claimant is told. Extent scoped tight, with `Not: publish or shift-reminder notifications — not yet`. Story trace `shift-cover: US-004`.

**`.mochiko/features/FEAT-001-rota-building.md`** — amend the two "Not:" lines at `:17` and `:18`, which both go false, pointing them at FEAT-006/007/008 rather than deleting them. Add relations to the new entries. Add `shift-cover: US-007` to the trace as confirmation only. (Its existing trace line `time-and-attendance: US-007` is a different spec's US-007; IDs are per-spec, so both lines coexist.)

**`.mochiko/features/FEAT-005-timesheets.md`** — add `composes-with FEAT-007` (hours follow the shift's current holder after an approved swap) and `shift-cover: US-009` to the trace.

**`.mochiko/features/FEAT-002-*.md` / `FEAT-003-*.md`** — one relation line each, since eligibility reads availability and approved time off.

**`FEATURES.md`** — new rows for FEAT-007 and FEAT-008, revised hook on FEAT-006, indented work rows in the existing `↳` style. Proposed shape:

```
| FEAT-006 | Open shifts | in-flight | a manager posts an unfilled shift to eligible staff at the site; first claim wins, manager approves |
|          | ↳ `pending` unfilled shift sent to a staffing agency | — | parked; superseded for the pilot |
| FEAT-007 | Shift swaps | in-flight | a staff member offers their published shift; an eligible colleague takes it; the manager approves and the rota changes hands |
|          | ↳ `pending` swaps and claims counted on the manager's week view | — | spot a site that is constantly short |
| FEAT-008 | Staff push notifications | in-flight | eligible staff are pushed an offer or open shift within a minute |
```

**`.mochiko/specs/index.md:10`** — shift-cover row: capabilities touched → `FEAT-006, FEAT-007, FEAT-008, FEAT-001, FEAT-005`; status advances off `specify` on sign-off.

**`BACKLOG.md`** — add under Open: `[ ] Rota PDF prints blank when the published week is printed from Safari; Chrome fine (support, 2026-07-15, two customers)`.

**`.mochiko/specs/shift-cover/spec.md`** — status line updated to record that the filter and selection are done, with the date pointing at the derivation and advice artifacts. I would **not** edit or renumber the stories themselves; they are the analyst's and are marked reviewed.

---

## Phase 4 — Write the derivation

Write **`.mochiko/specs/shift-cover/derivation.md`** — the artifact FEAT-006's header is waiting for. Contents: the Phase 2 ruling table with a reasoned line per story, the full Phase 3 delta in apply-ready form, the two open judgment calls flagged for the founder, and the analyst's question list. This is the durable record of *why* FEAT-006 changed channel — without it, that rewrite looks like someone overwriting the founder's row.

---

## Phase 5 — Write the release advice

Write **`.mochiko/specs/shift-cover/release-advice.md`**, addressed to the founder. Its argument, drawn from `notes/pilot-northgate.md` rather than from story priorities:

**Build first, in one release:**

1. **US-002 — open shifts, claimed by staff.** Two-thirds of Northgate's 20–30 WhatsApp messages per site per week are a manager hunting for an unfilled shift, and managers say the one thing that would change their week is not chasing people. This is the largest share of the pain and the least contested — the pilot note says manager shift-hunting is "just slow, not contested."
2. **US-003 — eligibility.** Non-negotiable and ships with it. Without it the manager is sifting people on holiday or in the wrong role, which breaks the exact promise being made.
3. **US-004 — push.** *This is my main piece of advice and it contradicts the priority sheet.* The analyst has it P2. Northgate's staff open the app **twice a week, almost always on rota-publish day**. A Saturday shift posted on Wednesday will not be seen. Without push, the manager posts an open shift, nobody answers, and they reopen WhatsApp — the feature fails on its own terms at this specific customer. US-002's own criterion ("the other is told") already assumes it. **Promote to pilot-critical.** Flag the countervailing fact honestly: this is new platform ground per `FEAT-001-rota-building.md:17`, so it is the least predictable item in the release and the one to size first.
4. **US-008 — per-site switch.** Cheap, and it is how Northgate asked to roll out: one café at a time across Northgate, Canal Street and Station. It de-risks the date rather than costing it.

**Build second, close behind:**

5. **US-001 — staff-to-staff swap**, which is where every dispute that reaches the owner comes from. Higher value per event but higher risk, and it is the harder half of the substrate.
6. **US-009 — swapped hours on the taker's timesheet — ships *with* US-001, never after.** Here is the collision worth the founder's attention: `timesheets-export/spec.md:12` (FR-003, in flight now, cycle 2 of 3) says a clock-in with no matching published shift is marked `unscheduled` and **excluded from the approved total**. If a swap is built as an overlay rather than rewriting the published shift's holder, the person who actually worked the shift is silently dropped from the approved hours and does not get paid. Northgate is on the CSV export today and would hit this. US-001's own second criterion already says the published rota shows the colleague on the shift — **design it to genuinely rewrite the holder, with history kept, and US-009 becomes a test rather than a feature.** Get this decision made before the substrate is built, not after.

**Cut for the pilot:**

- **US-005 (P3 summary)** — a counter on the week view. Nobody is blocked. Cut, keep as the `pending` row. Also worth knowing: the reporting sidecar was retired 2026-07-03 (`BACKLOG.md:9`), so there is no reporting substrate to lean on.
- **US-007 — already delivered.** Tell Northgate directly: publishing the week has been live since 2026-03-14 (`FEAT-001` extent). This costs nothing and answers the question they asked. No build.
- **US-006** — off this spec, onto the backlog, scheduled on its own. But it is a P1 defect hitting two paying customers, so I would say plainly that it should not be starved by the cover push; it competes for the same three engineers, alongside the Android 12 crash and the in-flight timesheets-export run.

**On the date.** I would not put a week count in this document. There are three engineers total, one run already in flight at cycle 2 of 3, a P1 defect and a P1 crash outstanding, and push infrastructure that has never been built here — and I have no velocity data in this workspace. I would give the founder the ordering and the two-release shape, and ask engineering to size the shared substrate (eligibility + claim/offer + approval + write-through) and the push work before any new date goes to Northgate. What I *would* commit to in writing: Northgate should be told the position now, given the date passed sixteen days ago, and the honest reframing is that term-time student staff make cover *more* valuable in September than it was in August, not less.

---

## Phase 6 — Stop #2: the founder decision

Present the derivation and the advice; do not touch `FEATURES.md` or any `FEAT-*` file until there is a ruling on:

- **FEAT-006 redirected from agency-fill to staff-claim.** *Yes* → apply Phase 3 as written. *No, agency is a real future capability* → collapse US-001 and US-002 into a single `FEAT-007 — Shift cover`, leave FEAT-006 untouched and unrefined, and I amend the derivation before applying.
- **Minting FEAT-008 for push.** *Yes* → new entry. *No* → fold into extent lines on FEAT-006 and FEAT-007; the advice does not change, only the map shape.
- **Promoting US-004 to pilot-critical.** *No* → I record the disagreement in the advice document with the twice-a-week usage figure attached, ship release 1 without push, and tell them what I expect: open shifts posted mid-week go unseen and the managers keep using WhatsApp. It is their call and I would say so and move on.
- **Two releases rather than one.** *No, one release* → then either US-005 stays cut and the date moves further out, or US-001 moves out; I would not agree to keep everything and hold the date.
- **Cross-site cover for Northgate's three cafés** — genuinely unknown, one question to the founder, default same-site.

---

## Phase 7 — Apply, on a yes

Make exactly the Phase 3 edits, in this order so the map is never inconsistent mid-flight: new `FEAT-007`/`FEAT-008` files → rewrite `FEAT-006` → amend `FEAT-001`/`FEAT-002`/`FEAT-003`/`FEAT-005` relations and traces → `FEATURES.md` rows → `.mochiko/specs/index.md` row → `BACKLOG.md` defect → spec.md status line. Statuses live on the map only, per `README.md:16`, so the entry files carry their status in the header block the way FEAT-001 and FEAT-005 do and nowhere else.

**No tests to run and none to write** — this workspace is documentation, with no code, no test harness and no build. The check is a consistency read-through: every story ID in a trace exists in `spec.md`; every capability named in `.mochiko/specs/index.md` has a file; every `FEATURES.md` row has an entry file and vice versa; the two amended "Not:" lines on FEAT-001 no longer contradict FEAT-006/007/008.

---

## Phase 8 — Delegation

**None.** No subagents or workers of any kind — this is a judgment-and-writing task on thirteen small files that I have already read in full, and splitting it would only risk inconsistent rulings across the map. The handoffs are to people, not workers:

- **Requirements analyst** — the Phase 2 quality list: US-006 misfiled, missing acceptance criteria on US-005/007/008, the unstated US-002 → US-004 dependency, and the uncovered cases (withdrawal, cutoff, re-checking eligibility at approval, the multi-site pending view). On their return I would check that US-006 is out of the spec and that US-008 has an independent test before the build starts.
- **Engineering** — size the shared substrate and the push work, and settle the write-through-vs-overlay question in Phase 5 item 6. On their return I would check specifically that the swap rewrites the published shift's holder, because that is what keeps FR-003 in the timesheets-export run from dropping the taker's hours.
- **Ops lead** — owns US-008 and the café-by-café rollout order across Northgate, Canal Street and Station.

---

## Phase 9 — What I would report

A short note to the founder covering, in order:

1. **The date passed sixteen days ago and no cover work has started** — with the evidence (spec still at `specify`, index untouched since 2026-07-13, no work rows anywhere) and a recommendation to tell Northgate this week.
2. **The filter:** of nine reviewed stories, five are capability work, one is already delivered (US-007 — answer Northgate yes), one is a defect that leaves the map for the backlog (US-006), and two are conditions on other stories rather than things to build (US-003, US-009).
3. **The proposed delta:** FEAT-006 refined from agency-fill to staff-claim, two new entries, four existing entries amended, one backlog item — with the two judgment calls flagged and awaiting their ruling.
4. **What to build:** open shifts + eligibility + push + per-site switch first; swaps + timesheet write-through second; cut the P3 summary.
5. **The one thing I would want them to read even if they read nothing else:** push is marked P2 but Northgate's staff open the app twice a week, so without it the release does not change the managers' week — and a swap built as an overlay rather than a rewrite will quietly stop people being paid for shifts they worked, through an in-flight rule that is being written into the export right now.
6. **What I did not do and why:** no map file edited pending their ruling; no story rewritten, since those belong to the analyst; no new date committed, pending engineering's sizing.