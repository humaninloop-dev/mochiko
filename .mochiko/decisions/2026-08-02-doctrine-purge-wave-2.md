# Doctrine purge wave 2 — `agent-dispatch.md`, `sized-end-stage-review.md`, `workflow-contract.md` deleted; the doctrine-template class is empty

**Status:** ruled
**Date:** 2026-08-02

## Context

Wave 1 (same day, v0.46.0) deleted the `loop-discipline` skill and `command-shape.md` and made
the six commands self-contained, deferring the remaining three cross-cutting doctrine
templates to later waves. The user continued the campaign in this session and ruled wave-2
scope as **all three**: `templates/agent-dispatch.md` (the caller-side dispatch brief, an
obligated per-dispatch read in every command preamble), `templates/sized-end-stage-review.md`
(the end-stage review protocol, an obligated sizing-gate read in brainstorm and setup), and
`templates/workflow-contract.md` (the fill-on-departure per-run form every command's Bindings
pointed at). The criterion, as in wave 1: utility versus how much each shared home makes the
commands blockers — an obligated read is plumbing; a command whose mechanics live in its own
file evolves them without cross-file ceremony.

Safety basis (consumer sweep, this session): the commands already carried most of what the
three files stated. `sized-end-stage-review.md`'s sizing gate, freeze, cross-exam cap,
two-exchange cap, verify pass, and waiver rules were already restated in brainstorm's and
setup's Constraints; the deleted files' genuinely unique content — the dispatch-brief field
list, the findings-formed mutual withholding, the reviewer-report shape, the lead-owned
cross-set merge, the departure-record content list — is inlined rather than dropped.

## Decision

1. **Delete** all three templates (verbatim content preserved in
   `.mochiko/strips/agent-dispatch.md`, `.mochiko/strips/sized-end-stage-review.md`,
   `.mochiko/strips/workflow-contract.md`, stamp v0.47.0). The doctrine-home template class
   is now empty: `templates/` carries only artifact and report schemas (plus
   `constitution-modules/`).
2. **Dispatch briefs inline.** Each command's preamble replaces the `agent-dispatch.md` read
   with its own brief statement: role and skill named as a hint, inputs to Read, output
   destination (write vs return), the quality bar, peer edges and holds, the independence
   reminder matching the seat, and retry routing (peer-routed gap list: point-and-open;
   relayed: paste verbatim). The hard independence line already lives in every Seats & checks
   table.
3. **The end-stage review protocol inlines into its two binders.** Brainstorm and setup each
   gain a **Review protocol** constraint carrying the delta the template alone held: freeze
   from reviewer spawn to last disposition · findings-formed count-only mutual withholding ·
   the reviewer-report shape (severity, failure scenario, resolution path, unresolved
   objections, own tally, recommended status) · cross-set merge + combined tally lead-owned ·
   facts cited-never-re-routed · overruled survivor marks `Contested`.
   `skills/review-brainstorm/references/CROSS-EXAM.md` stays the pair protocol's single
   source, now cited from the commands directly.
4. **The per-run contract form dies; the departure record survives.** A departing run (or one
   declaring non-default bounds) writes a plain departure record at the same path each
   command already named (`<cmd>-contract.md`) with the content stated inline: done-condition
   and bounds as (re-)declared · departures taken · counter state for Recovery. Non-command
   loops lose the form entirely; the four soundness requirements (default-FAIL done-condition
   · different-agent/different-skill validator · deterministic cap · named human gate) stay
   library doctrine in `skills/mochiko/SKILL.md`'s composition framing. This supersedes the
   2026-08-01 A2 revival ("workflow-contract.md revived for departing runs") in its *form*
   clause only — the recording obligation and paths survive.
5. **Router and references re-keyed.** `skills/mochiko/SKILL.md` drops the Doctrine template
   section; `analysis-iterative` (SKILL.md + SPECIFICATION-INPUT.md) re-words its
   agent-dispatch parentheticals to plain caller-side language; ARCHITECTURE.md drops the
   doctrine rows/labels and re-counts templates (17); CLAUDE.md axis 4 re-keys caller-side
   context onto "the dispatching command's own brief".
6. **Open watch item re-homed:** the BACKLOG shape-v7 watch (verify-pass trigger phrasing in
   `review-brainstorm`/`review-governance-intent`, formerly "logged in
   `.mochiko/strips/sized-end-stage-review.md`") stays open; the strip note's v0.47.0 entry
   restates it so the pointer stays live.

## Rationale

- Same as wave 1: the core bet places discipline in the primitives, not in enforcement
  plumbing. All three files were per-run reads or fill-targets — the highest-friction form of
  shared doctrine — around content the commands mostly restated.
- `workflow-contract.md` was the weakest blocker (fill-on-departure only) but its five-section
  form duplicated what each command's own Goal/Seats/Bounds already prove for a default run;
  a departing run needs a record of what varied, not a form re-proving what didn't.
- The template class boundary is now clean: `templates/` = schemas the seats fill, never
  doctrine the lead obeys.

## Alternatives considered

- **Two-file wave (agent-dispatch + sized-end-stage-review), workflow-contract later** — the
  lead's recommendation; the user ruled all three, ending the campaign in one wave.
- **Keep `CROSS-EXAM.md` movement into a command** — not taken: it is seat-side procedure
  (two review skills share it), not lead-side doctrine; it stays single-sourced under
  `review-brainstorm/references/`.
- **Drop the departure record with the form** — rejected: real behavior loss; the record is
  what Recovery reads on resume of a departing run and what the user accepts.
