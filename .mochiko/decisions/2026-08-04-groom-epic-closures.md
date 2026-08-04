# Groom epic closures — six items superseded or discharged by the v8 realignment

- **Status:** ruled
- **Date:** 2026-08-04
- **Context:** Backlog groom on a user "epics may no longer be required" doubt, three days
  after the 2026-08-02 wave (v8 goal+harness rebuild at v0.48.0 · task-granularity +
  slice-dissolution at v0.49.0 · UX-prototype stage at v0.50.0). A delivery sweep verified
  each candidate's stated obligation against the shipped v8 surface (`plugins/mochiko/`
  commands and skills on disk), the DECISIONS.md supersession annotations, and the strip
  notes. Closures presented per item; user ratified six and kept the four remaining
  per-command dogfood items open.
- **Decision:** close to the trail:
  1. **Command orchestration substrate — teams vs `Task`-subagents** (open design decision,
     2026-06-30) — superseded by command-architecture-realignment D5 (2026-08-02):
     transport-neutral commands, teammates vs subagents per-seat lead judgment. The question
     the item held open was ruled.
  2. **Team-form confirm-or-revert — instrumented run (residual A)** (2026-07-24) —
     superseded by the same D5: the team mandate died, so there is no mandate left to
     confirm or revert. The captured forensics artifacts remain in the
     `plan-run-transport-forensics` record. Its ROADMAP *Next* row drops.
  3. **Shape-home keying watch: "Out of rounds = escalate, never done."** (2026-07-30) —
     discharged empty: the phrase appears in zero v8 commands and the Constraints-block
     anatomy it watched was superseded at v0.48.0 (D6); the re-raise trigger can no longer
     fire.
  4. **Token-epic D5 — review sizing gates generalized + floored verification depth**
     (epic ruling 2026-07-23) — superseded: the sizing-gate/weight-card machinery D5 wanted
     generalized was deliberately killed at the v8 rebuild (D2, ratified reversal); the
     verification-depth intent survives structurally in `implement.md`'s Goal (verification
     without real-infrastructure evidence = default FAIL), not as a sizing gate.
  5. **Shape-v7 post-conversion watches** (2026-08-01) — superseded: the v7 form (run-start
     declaration, departure trail, stated defaults, ceiling terms) died whole at v0.48.0.
     Surviving thread — the U1-B verify-pass trigger-phrasing alignment for
     `review-brainstorm`/`review-governance-intent` — already lives in
     `.mochiko/strips/sized-end-stage-review.md` as a next-touch note; no open item needed.
  6. **Slice dogfood + unexercised sub-paths (residual D)** (2026-07-02) — superseded:
     `/mochiko:slice` dissolved into specify at v0.49.0. Surviving sub-paths (single-slice
     null exit · spec-amend/graded-amendment path · `infeasible` escalation, now plan's)
     folded into the v0.49.0 first-live-run watch item.
- **Rationale:** each item's stated obligation either was answered by a ruled decision
  (D5), watches an artifact that no longer exists (v7 form, Constraints blocks,
  `/mochiko:slice`), or asks for machinery a later ratified reversal deliberately removed
  (sizing gates). Keeping them open would misrepresent the decision state; survivors were
  re-homed rather than dropped.
- **Alternatives considered:** keeping the substrate/confirm-or-revert pair open as a
  transport-quality watch — rejected: the message-fidelity defect item and the two
  first-live-run watches already carry the live transport concerns. Closing the four
  remaining per-command dogfood items (setup/specify/plan/implement) — user declined; they
  stay open, re-keyed transport-neutral as a groom staleness fold.
