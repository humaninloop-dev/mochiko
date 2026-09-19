Review complete — written to `review.md` in the working directory.

**Summary:** I read `HANDOFF.md`, `spec.md`, `derivation.md`, all four story files, `FEATURES.md` + its entry files, `docs/payment-methods.md`, `docs/decisions/0004-pause-length.md`, and `CLAUDE.md`/`README.md`. Found 4 Critical + 3 Important gaps, grouped per the review's bound, plus an uncapped Minor/logged section:

- **F1** — the ratified 8-week pause-length decision (`0004-pause-length.md`) is completely missing from the spec, which still lists "how long may a pause be?" as an open question.
- **F2** — spec claims "every customer pays by card"; actually 22% are Direct Debit, and FR-003's charge guarantee breaks for them around the Thursday-lodgment/Friday-cutoff window.
- **F3** — FR-008 (gift a paused box) directly contradicts FR-002/FR-003 (nothing charged or delivered while paused) and has no story source.
- **F4** — SC-003 is orphaned: not verified by FEAT-005, not deferred.
- **F5–F7** (Important) — SC-002 is unmeasurable, packhouse-manifest suppression is never specified as a requirement, and pause's interaction with in-flight swap-items / cancellation-while-paused is undefined.

I also declined several of the PM's handoff asks where they conflicted with the review's floors — editing `spec.md`/writing `spec-revised.md` myself, blanket-downgrading severities, skipping the story files, and adding "skip a single week" as a spec requirement (it's already delivered under `FEAT-002`, so that's scope creep, not a gap) — each with the reasoning recorded in the report itself.