# Delivery-sweep step added to `grooming-operating-docs`

- **Status:** ruled
- **Date:** 2026-08-01
- **Context:** The first delivery sweep (run ad-hoc this session, five parallel checkers over
  60 open items) found three delivered-but-still-open BACKLOG items — brownfield onboarding
  path (delivered at the 2026-06-27 setup port), the D2 baseline capture defect (superseded by
  the 2026-07-30 run-cost drop), the brainstorm/setup light-pass residual (discharged by the
  v0.35.0 wave) — plus materially stale item text (wrong command/carrier counts, a wrong
  `provenance: unrecoverable` claim, two 2026-07-31 dogfood transcripts never credited to the
  items they partially discharge). The groom skill restored shape contracts but had no step
  that checks items against delivered reality; the count watch catches growth, not staleness.
- **Decision:** Add a conditional **delivery sweep** as procedure step 3 of
  `grooming-operating-docs` (v0.41.0), with a matching trigger phrase in the description.
  Fires on an "already delivered?" doubt or a tripped open-item-count watch — not every groom.
  Craft encoded: verify each open item's *stated obligation* (adjacent work never counts;
  run-gated items stay open without run evidence); per-item verdict with citations; every
  delivered claim independently re-verified before presenting; closures are the user's to
  ratify; partial evidence and stale item text fold into the item fix-on-sight.
- **Rationale:** Items outlive their delivering events whenever a landing's subtractive ritual
  doesn't sweep the whole backlog — the delivering wave closes *its* item and misses siblings
  it discharged as a side effect. Only an obligation-vs-evidence pass catches that class; the
  verification-before-presenting clause is what kept this session's sweep honest (one checker
  claim was corrected on re-grep).
- **Alternatives considered:** Always-on sweep every groom (rejected — a full sweep is a
  multi-agent pass, too heavy for a stamp-and-cap groom); a per-item glance while touching
  items (rejected — the evidence lives outside the item text, a glance would not have found
  these three).
