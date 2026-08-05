# External-Claims Verification

> **Single source.** This file is the one home of external-claim verification — the
> load-bearing trigger, the floor classes, the inline-check mechanics, the source re-read
> clause, pair source-conflict resolution, and the disclosure-line grammar. Every review
> skill and producer surface that touches an outside-repo claim binds here by pointer;
> the doctrine is never duplicated. Ruled at
> `.mochiko/brainstorms/external-research-in-review/record.md` (ER-D1–D6, 2026-08-04).

## The trigger — load-bearing (ER-D2)

External verification fires when a claim is **load-bearing**: a decision, verdict, or gate
would flip if the claim were false. Non-load-bearing color ("popular", "modern",
"widely used" as flavor) is out of scope — the trigger is a judgment rule, not a keyword scan.

## Floor classes (non-exhaustive)

Under the load-bearing rule, these classes **always** fire:

| Class | Example shape |
|-------|---------------|
| Version / capability | "library X supports feature Y as of vN" |
| Security posture | "well-maintained, audited library" |
| Regulatory content | a stated compliance obligation, limit, or legal constraint |
| Benchmark / limit numbers | a cited throughput, latency, or quota figure |

Floor, not ceiling: any outside-repo, load-bearing claim qualifies even when it matches no
row here — the classes guide the hunt, they never gate it.

## Architecture — verify-at-review, pure (ER-D3)

Producers **disclose**; review seats **verify**. No producer carries a verification duty
anywhere in the pipeline — the checker is never the claimant, so motivated reading is
structurally out at the fact layer.

## Inline-check mechanics (ER-D4)

The review seat runs the check itself — WebSearch/WebFetch mid-review, at the moment the
claim is hit, not as a separate pass or a dispatched sub-check. Hunt disconfirming sources
against the producer's claim, not confirming ones. Cite what you fetched as **quotable
text** — never a paraphrase, never a summary.

## The source re-read clause (ER-D4)

A finding whose premise is an external claim must cite its fetched source (quotable text).
Before that finding survives: the **counterpart reviewer** (pair review) or the **lead**
(solo review) re-reads the cited source cold. One independent read between cherry-pick and
kill-verdict.

## Pair source-conflict resolution {#pair-source-conflict-resolution}

Two reviewers fetching independently on the same claim may surface conflicting sources.
At cross-exam, the re-read clause applies to **both** sources — each reviewer re-reads the
other's citation. The surviving finding cites the source that survived the counterpart's
read; if neither read settles it, the conflict itself is reported with both citations
attached — never argued past the sources. (Delegated here from `CROSS-EXAM.md`'s
external-claim carve-out; that file remains the pair-protocol home.)

## Disclosure-line grammar

Every externally-sourced claim in an artifact carries exactly one of:

- `verified: <source>` — the producer happened to check it live; the source is named
- `memory-asserted` — stated from model memory, not live-checked

**An undisclosed external claim is itself a finding** — the reviewer live-checks
floor-class claims regardless of the line, but the omission is reported so the audit
signal is never silently lost.

## No-review paths (ER-D5)

Waiving a review waives external verification with it — the waiver's stated cost names
both ("un-reviewed record, externally-unverified claims"). Bare sessions ride the user's
own premise-checking practice; there is no residual gate check.

## Fact-checker role — flag, don't fetch (ER-D4 residual)

The fact-checker seat keeps its file jurisdiction unchanged. Its claim map **may flag**
external premises as `memory-asserted` — a pre-built hunt list for the reviewers — but the
seat never fetches: external verification belongs to the review seats alone.

## Out of scope (recorded exclusions)

- **Catalog tool tables** — static library content; currency is a library-maintenance
  concern riding the primitive-edit ceremony, not run-time review.
- **Domain-registry growth at implement** — deferred, not absent: the next amend run
  re-validates trust signals; no review-time carrier here.
- **A regime's full obligation set** — mapping regulations to modules is
  `COMPLIANCE-MODULES.md`'s job; this file only governs how a stated regulatory claim
  gets verified at review.

---
**Consumed by:** `review-brainstorm` (owner) · `review-feasibility` ·
`review-specifications` · `review-governance-intent` · `review-plan-artifacts`
(disclosure-presence check only) · `patterns-technical-decisions` (disclosure grammar) ·
`templates/artifact-format.md` (disclosure grammar) · `CROSS-EXAM.md` (external-claim
carve-out) · `agents/validator.md` (evidence-hierarchy rung — the source re-read clause
only; added with the v0.52.0 build, beyond ER-D6's eight ruled touches).
Edit-time guard: a change here is assessed against **all** consumers.
