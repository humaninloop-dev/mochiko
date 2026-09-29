# The hook field review and the delta-files retirement build together — the seams between them

**Date:** 2026-09-29
**Status:** ruled (user) 2026-09-29, R1–R7 each "as recommended" (R5/R6 at the wave-1 plan's
approval, R7 during its plan round; R8 "as recommended" during wave 2's plan round); **built
2026-09-29 at plugin v0.116.0** (joint build waves 1–3, branch `joint-hook-delta`); R3 applies and
wave 4, kinako, is owed
**Driver:** the user asked to implement `hook-enforcement-field-review` (accepted 2026-09-23) and
`delta-files-vs-direct-baseline-edits` (accepted 2026-09-24) together, after asking whether their
decisions clash.

## Context

The delta record already rules that its build rides the field review's four waves (its D6d), so
building them together is the ruled path, not a new choice. A cross-read of both records found no
direct clash: the delta record supersedes in part the two field-review clauses that disagreed with
it (the field review's OQ1 "fold shape" question and its `baseline-delta.md` entry-class candidate,
delta D1/D6a). It did find seven seams where both records rule the same object and neither says
what happens, and two smaller ones.

Two things changed after both records were accepted. AM-5 (governance v3.2.0, 2026-09-24) landed
the field review's wave-3 ledger amendment and recorded the 0.109.0 hook ship as a GI-012
exception-registry row, which lets further `plugin.json` bumps ship the hooks until the first
`mochiko-cli` publish with all four controls. Plugin v0.115.0 (`setup-product-agnostic`) took
migrations 0019–0023 and struck setup's design-truth write, leaving the design truth part's writer
to a queued rehoming brainstorm (that record's OQ1).

## Decision

**R1 — A ruling that changes a baseline lives on its entry only (seam 1).** A ruling made during an
implement run that changes a product baseline or the architecture store is recorded on that entry
and nowhere else, its reason in the entry's own fields (`Source` · `Shaped by` · `Impact`, or a
build-raised entry's `Raised:` · `Weighed:`, delta D3b). The `.mochiko/decisions/<date>-<slug>.md`
route of the field review's D6 (as amended at its review, S1) takes only standing in-run rulings
that touch no baseline, such as "the verifier runs tests in release mode". This narrows the field
review's S1 routing; it lands in wave 3's `impl.artifact-home` reword.

**R2 — The base commits survive the run log (seam 6).** The run-open base commit is also written
into `sufficiency-report.md`, and both base commits (run-open, and the sign-off commit or the
checkpoint-table stand-in of delta V1) into the final-validation report. The ephemeral run log of
the field review's D6 keeps them too. This adds to delta D3c; nothing in it is withdrawn.

**R3 — No implement run is open across the joint upgrade (seam 7).** When the joint bump reaches a
consumer, no implement run is open there: every open run lands or is closed before the upgrade.
Kinako's wave-4 pass checks this first. The rule is for this upgrade only, because this one removes
the per-feature delta files and the fold that an open run would need to finish. The field review's
S16(ii) posture — a run in flight reads the new homes at its next write, no run pins a plugin
version — stands for every other upgrade.

**R4 — The joint build ships under the AM-5 exception row (publish).** The joint `plugin.json` bump
ships the hooks under the GI-012 exception-registry row (only the maintainer installs, from git
`main`), its `CHANGELOG.md` entry citing the row as the row requires. The field review's wave-1
publish stays owed: the crate version is bumped, and no `mochiko-cli-v*` tag lands until the two
owed controls (a manual-approval publish environment, signed tags) exist; that publish closes the
row.

**R5 — The run-id form (the field review's OQ2).** A run folder is named `<owner>-run<n>`, the owner
being the delta record's D2 lifecycle key: `.mochiko/runs/FEAT-001-run5/`, `EPIC-003-run1/`,
`lane-auth-fix-run1/`. The lead creates it at run-open. The run-key name check (the field review's
D4 control 2) keys on this form, built as a new `<run-id>` path token in wave 1.

**R6 — Who deletes the run folder (the field review's OQ5).** The lead removes it as a landing step
after the user's acceptance. No `mochiko-cli run close` subcommand is built.

**R7 — The sniff's `## Header` signature limb is struck, never built.** The prior ruling's D9
(`hook-enforced-artifact-schema` record :542–544) gates a write outside every declared home when its
content opens with report frontmatter *or a template's `## Header` signature*. Only the frontmatter
half was ever built; S1's clause-walk of the ledger found the gap. The limb is struck by this
ruling: the sniff stays report frontmatter only. The ledger's GI-019 "Reach of the gate" sentence is
corrected at the wave-3 text-vs-build check, together with two text drifts S1 found (the ledger's
"a `.md` write" where the binary sniffs every extension; "What the gate reads" naming the on-disk
file for `Edit` only, where `Write` also reads it as the amnesty baseline).

**R8 — The sign-off flips each `proposed` entry by what the diff shows (a gap between delta D2 and
D7).** Before the design checkpoint every baseline write a run makes reads `proposed (<key>)`, as
delta D7 rules: a new entry, an amended one (its new text in place) and a removed one (its heading
and marker kept, its body cut, the prior text in the pinned-base diff per M1). At sign-off the flip
reads the pinned-base diff entry by entry: an entry absent at the base becomes `in-flight (<key>)`,
one changed from the base `modifying (<key>)`, one cut to its heading `removing (<key>)`. This
reconciles D7's "flips every `proposed (<key>)` … to `in-flight (<key>)`" with D2's `modifying` and
`removing`; nothing else in either is withdrawn. After sign-off, D2 applies as written: a build-time
amendment reads `modifying` and a removal `removing` from its first write, as a build-raised entry
reads `in-flight`. Raised by seat S4 in its wave-2 plan (its Q3); lands in the grammar rule,
`lifecycle-statuses`, the store template's legend, the store gate's successor and the checkpoint
flip (migrations 0025 and 0029).

**R9** (a landed removal leaves a one-line `removed` stub) is recorded apart, this record being at
its size bound: [2026-09-29-landed-removal-stub.md](2026-09-29-landed-removal-stub.md).

**Routed to the census table, no new ruling (seams 2–5 and the smaller two).** The field review's
wave-2 census table — one artifact the user ratifies — also: declares the archived-ledger shape
under `archive/` and the wave-4 pass names the move route, since the field review's S12 keeps
`mv <home> <elsewhere>` a deny (seam 2); sizes each entry budget on the baselines as they stand
after the delta record's D5 cleanup, and states how text outside entries is bounded (summary
tables, relationship tables, validation-rule lists, dated reconciliation sections) (seam 3); states
whether the `Lifecycle:`, `Raised:` and `Weighed:` lines count toward an entry's budget (seam 4);
rules `quickstart.md` and `design/design.md` — their entry grammar and their bound — with the
design truth writer left to the rehoming brainstorm (seam 5); and is taken from the write sets the
primitives will have after the delta record's rewrites, not today's (smaller one). How a report
cites a reproducing commit for uncommitted work (field review D4, which has no fallback where delta
V1 has one) is put to the user at the wave-3 plan approval (smaller two).

## Rationale

- R1: the field review's S1 example list already included "`AX-007` amended", a store change, so
  without a boundary one ruling would be written twice or in a seat-chosen place. The delta record
  made the entry the home of a baseline change's reason; the decisions route keeps what has no
  entry.
- R2: the unmarked-write test's "already in the tree at run-open" exclusion needs the run-open
  commit, and the field review deletes the run log at acceptance. The sign-off commit already had a
  durable stand-in (the checkpoint table); the run-open commit had none.
- R3: an open run's delta copies and ledger stop being legal files, and the fold that would land
  them no longer exists, so the run could not finish. A pre-upgrade check costs one line; the other
  roads need a mechanism or hand work per run.
- R4: the two controls are set up by the user on GitHub; waiting for them would hold the whole
  build. The exception row was written for exactly this window, with a tripwire (anyone else
  installs) and a 2026-12-31 backstop.
- R5: one vocabulary for the run folder and the lifecycle markers, readable at a glance.
- R6: the pre-code ladder stops at "exists already"; `rm -r` inside `runs/` is not a parsed write.
- R7: product documents commonly carry a `## Header` heading (a page's header section), so the limb
  would deny honest product docs; no field run showed a template smuggled out of `.mochiko/`, and
  the report sniff covers the case the runs did show.
- R8: after sign-off a reader tells a new entry from a changed or removed one at a glance, without
  the diff; every unsigned change still reads `proposed`, so D7's "nothing promised" holds for
  amendments and removals too.

## Alternatives considered

- R1: both places, the entry plus a `decisions/` record pointing at it (two surfaces to keep in
  agreement) · `decisions/` only, the entry pointing there (the reason away from the entry, against
  delta D3b).
- R2: the run log only, as ruled (no trace after acceptance of which base the review read) · both
  commits in the `architecture.md` checkpoint table (only exists when the run carries a drawing).
- R3: the lead converts an open run's delta files into marked entries at its next write (manual,
  per run) · the open run finishes on the old plugin (needs a version-pin mechanism that does not
  exist).
- R4: publish first, then ship (nothing ships until the GitHub setup is done).
- R5: a date plus the owner (two runs of one feature on one day collide) · a random key minted at
  run-open (never collides, but the folder does not say whose run it is).
- R6: a new `mochiko-cli run close <id>` subcommand (one more command to build and test for what a
  removal already does).
- R7: build it as ruled (false denies on product docs) · build it narrowly, `## Header` only beside
  other template-marking headings (less false blocking, more to build and test).
- R8: every `proposed` flips to `in-flight`, as D7 is written, with `modifying`/`removing` only for
  build-time changes (a reader needs the diff to tell a change from a new entry, and a removed entry
  reads `in-flight`) · the kind from the first write, only new entries passing through `proposed`
  (an unsigned change reads like a signed one — the leak D7 closes).
