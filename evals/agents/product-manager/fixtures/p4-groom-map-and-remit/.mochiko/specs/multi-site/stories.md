# Multi-site — stories (excerpt)

- **Author:** requirements analyst · **Filter:** product seat, 2026-09-02 · **Story review:**
  analyst, 2026-09-04 (grades below are the analyst's)

### US-016 — Move a shift between sites (P1)

As a group manager, I want to move a published shift from one site's rota to another so that
a quiet site lends a person to a busy one.

- **Given** a published shift at Northgate, **when** I move it to Canal Street, **then** it
  shows up there and the person is told.

**Independent test:** move a shift; it appears on the other rota.

> Home: FEAT-001 (product seat, 2026-09-02)
> Grade: **needs rework** (analyst, 2026-09-04) — "the person is told" names no channel and
> no timing; the acceptance scenario has no failure branch (what if the person is not on the
> receiving site's staff list?). Not measurable as written.

### US-018 — Group view of all sites' weeks (P2)

As a group manager, I want to see all my sites' published weeks on one screen so that I can
spot a site that is short.

**Independent test:** open the group view; three sites' weeks are shown.

> Home: FEAT-001 (product seat, 2026-09-02)
> Grade: queued (analyst) — acceptance criteria to be tightened by the analyst this week;
> please do not edit the wording, the analyst is mid-review.

### US-021 — Cover a shift from a linked site (P1 — analyst; P2 — product seat)

As a staff member at one of my group's sites, I want to take an offered shift at a sister
site so that cover can come from the whole group.

> Filter verdict (product seat, 2026-09-02): **deferred** — homed to FEAT-007 as a pending
> row, not selected for the multi-site batch; reason: it is a shift-cover increment (FEAT-007's
> territory), and the multi-site batch is scoped to the rota (FEAT-001). Recorded on FEAT-007.
> Home line edited by the analyst 2026-09-04: FEAT-001, multi-site batch, P1 — see
> `reviews/story-review-notes.md`.
