# Spec — Shift cover

- **Status:** specify — stories drafted and reviewed; feature filter and selection pending
- **Author:** requirements analyst (stories, 2026-07-14; reviewed 2026-07-16)
- **Pilot:** Northgate Coffee — `notes/pilot-northgate.md`
- **Next:** the product seat filters the stories onto the map, proposes the delta, and
  advises the founder on what to build for the 2026-08-24 pilot date.

## Intent

Staff cover each other's shifts, and managers fill shifts nobody is on, without the manager
running it by text message. Today a published week is fixed: the only way to change who holds
a shift is for the manager to re-open the week.

## Stories

### US-001 — Offer my shift for swap (P1)

As a staff member, I want to offer a published shift of mine to colleagues at my site so that
someone else can take it when I cannot work.

- **Given** a published shift of mine, **when** I offer it, **then** eligible colleagues at my
  site see the offer and one of them can take it.
- **Given** a colleague has taken my offer, **when** the manager approves, **then** the
  published rota shows the colleague on the shift and I am off it.
- **Given** the manager declines, **when** I look at my week, **then** the shift is still mine
  and I can see the decline.

**Independent test:** offer a Saturday shift, have a colleague take it, approve it; the
published week shows the colleague.

### US-002 — Post an open shift (P1)

As a manager, I want to post an unfilled published shift to the staff at my site so that the
first eligible person to claim it gets it, subject to my approval.

- **Given** a published shift with nobody on it, **when** I post it, **then** eligible staff at
  the site see it and can claim it.
- **Given** two people claim it, **when** I look, **then** the first claim is on top and I
  approve one; the other is told.

**Independent test:** post an empty Sunday shift, claim it from two accounts, approve one.

### US-003 — Only eligible colleagues can take a shift (P1)

As a manager, I want only staff who can actually work a shift to see an offer or an open
shift, so that I am not approving people who are on holiday or in the wrong role.

- **Given** an offer for a barista shift, **when** a floor-role colleague looks, **then** they
  do not see it.
- **Given** an offer on a day a colleague has approved time off, or outside their declared
  availability, **when** they look, **then** they do not see it.

**Independent test:** offer a barista shift; a floor colleague and a colleague on approved
leave see nothing; a free barista sees it.

### US-004 — Tell me when a shift I can take is posted (P2)

As a staff member, I want a push notification when an offer or open shift I am eligible for
is posted, so that I do not have to open the app to find out.

- **Given** an eligible offer is posted, **when** it is posted, **then** I get a push
  notification naming the site, day, and time within a minute.

**Independent test:** post an open shift; an eligible staff member's phone shows it.

### US-005 — Weekly cover summary (P3)

As a manager, I want to see how many swaps and open-shift claims happened this week on my
week view, so that I can spot a site that is constantly short.

**Independent test:** after three swaps and one claim, the week view shows 3 and 1.

### US-006 — The rota PDF prints blank in Safari (P1)

Reported by support 2026-07-15 from two customers: printing the published week from Safari
produces a blank page. Chrome is fine.

### US-007 — Publish the week (P1)

As a manager, I want to publish the week's rota so that staff can see it. (Carried over from
Northgate's requirements list — they asked us to confirm it is in.)

### US-008 — Turn swaps on per site (P2)

As the ops lead, I want a switch to enable shift cover site by site so that we can roll it
out to Northgate one café at a time during the pilot.

### US-009 — A swapped shift counts for the person who worked it (P2)

As a staff member who took a colleague's shift, I want my hours for that shift to appear on my
own timesheet so that I am paid for it.

- **Given** an approved swap, **when** I clock in and out of that shift, **then** the hours are
  on my week, not the original holder's.

**Independent test:** take a swapped shift, clock in and out; the week's hours show on the
taker's timesheet.
