# The Pair Cross-Examination Protocol

> **Single source.** This file is the one home of the reviewer-pair cross-examination protocol.
> `mochiko:review-brainstorm` and `mochiko:review-governance-intent` both run it by reference —
> each skill supplies its own substrate bindings (what "the artifact" and "the fact substrate"
> are, and where fact disputes route); the protocol itself is never duplicated. An edit here
> changes both skills.

**Pair only** — a solo reviewer skips this entirely and reports undebated. The protocol runs
after both reviewers have reported **findings-formed** (count only) from their independent cold
reads, and after the lead has introduced them. Until that introduction, the counterpart's
identity and findings are withheld — inheriting another reviewer's framing destroys the
independence the pair exists for.

## The exchange — one-shot, four messages total

| # | Direction | Content |
|---|-----------|---------|
| 1 | first reviewer → counterpart | its findings |
| 2 | counterpart → first | its findings **plus** attacks on message 1's |
| 3 | first → counterpart | attacks on message 2's findings **plus** defenses/withdrawals of its own |
| 4 | counterpart → first | defenses/withdrawals **plus** close |

Four messages, then done. **Do not iterate toward consensus** — an unresolved disagreement
survives as its owner's finding with the counterpart's objection attached; the lead sees both.

## Attack and defense standards

- For each of the counterpart's findings, **try to refute it**: a wrong fact, a misread of the
  artifact under review, a failure scenario that cannot occur, severity inflation — cite the
  artifact, the fact substrate, or the files. Insistence is not refutation.
- For each of your findings under attack, **defend it or withdraw it**. **Withdrawal is the
  owner's alone — you persuade, never veto.** Withdraw only when shown wrong.
- **A fact dispute is checked, never argued** — route it to the session's fact authority (the
  invoking skill names it: a fact-checker teammate when one is seated, the session's reality
  surface or an Explore check otherwise). **A fact the counterpart already routed is cited,
  never re-routed** — one route per fact.
- A suspected duplicate of the counterpart's finding is **flagged by name** in your report, not
  merged — **the cross-set merge is the lead's**, under whichever formulation survives strongest.
