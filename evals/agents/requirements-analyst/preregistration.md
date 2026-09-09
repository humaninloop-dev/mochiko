# Preregistration — requirements-analyst persona plan-only eval (wave A baseline kit)

Committed BEFORE the baseline grid (`primitive-eval-harness-v2` D11 as folded; the pilot rulings
`2026-09-09-persona-pilot-1-latitude-out-of-instrument` and `2026-09-09-persona-pilot-2-validator-read`;
the band re-key `2026-09-09-persona-band-invited-only`).
This is a **baseline kit**: no persona edit exists to detect, so there is no positive control. The kit
fixes the band and the coverage so that the persona's next edit gets a `pre`/`post` read on day one.
Fields marked **[measured at baseline]** are filled from the baseline grid.

## Edit under read

None. `pre` = `post` = the tree at `b9efb59` (the commit the rubric was minted at; the persona file is
byte-identical in the working tree, checked by `git show b9efb59:plugins/mochiko/agents/requirements-analyst.md`
against the working copy), all claims `common`. The pin, not `HEAD`, is what the next edit's `--old-ref`
names. The next persona edit re-mints over the union of refs and runs the pilot form with a positive
control on the edit.

## Rubric shape (partition at kit authoring)

24 claims from 23 units: 21 plan-observable, 3 out-of-instrument (`dispatch-conditional`, the cheap-read
trio), 2 `not_claims` (the model-tiering pointer sentence, as ruled for `tech-lead` and `devils-advocate`;
the "When given a feature request:" intro fragment the parser harvests from the Your Process list), one
split (the Testable standard into `quality-standards.criterion-verifiable-pass-fail` and
`quality-standards.criterion-covers-requirement-fully`, both `split_of: testable-over-aspirational-every`,
sharing sha `89681d12b6c3`; neither member reuses the parent slug). No `latitude-conditional` claim was found: the Your Judgment list is directive
("you: state / flag / make / never"), not discretionary. Every plan-observable claim is tempted by at least
one golden; none is tagged `untempted`.

## Read rule (D11)

- **Prune FIRST** = `agent-prune requirements-analyst --replicates 3` over the tempting goldens (≤ 12
  sessions): claims the bare model meets in every replicate are tagged `model_native` and leave the graded
  set. It runs before the grid because the grid freezes the rubric snapshot the report grades from. Expect
  a large model-native share on the Your Process steps and the Embrace list (the `devils-advocate` prune
  found 15 of 18 document-review claims model-native); the kit's value is in the claims that survive.
- **Baseline grid** = `agent-grid requirements-analyst --arms post --replicates 3 --out baseline`
  (4 goldens × 3 = 12 plan sessions), judged with the Haiku embodiment checklist (foreclosing-step rule in
  force; pairwise off). It records, per claim, pass^k coverage and replicate agreement on the pruned rubric.
- **Untempted claims** are excluded by the runner and listed as disclosed (none at this kit).
- **Conditional reasons applied at mint** (never read): `dispatch-conditional` (the cheap-read trio). Any
  `latitude-conditional` claim found at a later re-mint is likewise out.
- **Wrapper interactions, recorded now:**
  - *Discovery questions.* The persona elicits through structured discovery and will want to put questions
    to the requester. Under the wrapper's no-dispatch, no-user-input line it states the questions it would
    put and to whom (every fixture names the requester: Tom and Priya, Maya, Dan, the Rota PM), describes the
    stop and each branch, and continues under a stated default — as `tech-lead` recorded for its feasibility
    questions. Naming the questions and the recipient embodies the discovery standards without any dispatch.
  - *Never-guess versus the stated-default continuation.* `judgment.guess-security-data-user` reads
    `reflected` when the plan names the stop, the question, and the recipient for a planted security, data,
    or user-facing gap and marks its continuation choice as an assumption; the wrapper's "continue planning
    under your stated default" line does not undo the stop — the stop is the foreclosing step. With no stop,
    the verdict is one of two conditions: no stop + the choice recorded as an assumption = `absent`; no stop
    + the choice written as a requirement or asserted as settled fact = `contradicted`. (This is the only
    claim all four goldens tempt, and the calibration bar demands 100 % on `contradicted`, so the two
    conditions are exhaustive for gaps in the major class; the data-answered class in the next bullet
    reads `reflected` with no stop, and the classes are disjoint by construction — classify first
    under *Which gaps count as which*, then apply.)
  - *Which gaps count as which.* Each golden's `note` classifies its planted gaps into three classes, and
    both judgment claims are read against that classification, never against the seat's own sorting:
    **major** — money, access, or user-facing behaviour with no answer in the fixture (a stop is owed;
    read under the bullet above); **minor** — a detail with no consequence beyond itself (a default is
    owed; `make-reasonable-defaults-minor` reads `reflected` when the plan picks and records one, `absent`
    when it escalates the detail to a question or leaves it open, never `contradicted`); **data-answered** —
    a gap the fixture's own numbers or rules settle (r2's maximum file size from the email limit, the gateway
    cut-off, and the size percentiles), which is the analyst's to decide and record as an assumption: it
    reads `reflected` on never-guess with no stop required, and a plan that sends it back unanswered is
    `absent` on the defaults claim.
  - *Skill fence.* The persona's two skills (`authoring-requirements`, `authoring-user-stories`) hold the
    literal FR/SC/story formats and sit outside the read fence. Every fixture carries a house format
    (`specs/README.md` plus a reference spec) so a plan can name sections without them. The plan is graded on
    the standards, never on format; a plan that says it would load a skill is neither credited nor penalised.
  - *Prohibition-shaped Reject claims* (`ambiguous-terms-without-quantification`,
    `assumptions-hidden-requirements`, `feature-requests-without-clear`, `requirements-can-t-be`) read under
    the settled foreclosing-step rule: a concrete step that forecloses the behaviour (replacing the term,
    moving the line to Assumptions, refusing the benefit-less story, rewriting the untestable requirement)
    is `reflected`; a restatement is `absent`.

## Noise guard (I7)

Band = the baseline arm's replicate spread over **invited pairs only** (ADR `2026-09-09-persona-band-invited-only`:
the (golden, claim) pairs the golden's `tempts` list names), **[measured at baseline]**, plus five points,
capped at 20 %, computed per arm from the judged `summary.json`. The kit tempts 50 pairs across four goldens
(13 · 11 · 14 · 12), so it is not under-sampled before the prune; if the prune leaves fewer than eight
invited pairs on the graded set the kit carries the `UNDER-SAMPLED` mark and its band is the cap. Uninvited
pairs are judged, feed coverage, and are disclosed beside the band, never in it. The band binds the next
edit's grid: an arm above the band is noise-dominated and no `pre`/`post` difference is read; one extra
replicate per arm, once. Stopping rule at the next edit: two consecutive instrument re-keys without a
detectable control return the target to the user.

## Judge calibration (I9)

Drawn from the baseline grid after `agent-judge`: ≥ 20 (claim, plan) pairs via the arm-blind
`agent-label-sheet` (single arm here, so blinding is nominal), labelled from **full plan reads, never
keyword extracts** (the `devils-advocate` process rule: extract-based labels gave 0.667 agreement with a
full-read labeller), by a labeller who has not seen the judge's verdicts and does not re-label after seeing
them, with the settled rules — embodies-never-recites; declined/hypothetical conditional path = `absent`;
a Reject-section or prohibition-shaped standard (never / do not / reject / refuse) is `reflected` when a
concrete step forecloses the behaviour whether or not it was invited, `absent` when merely restated; an
`Explore` read dispatch is never a worker delegation; the wrapper interactions above. Bars: ≥ 80 %
agreement; 100 % on `contradicted` pairs; fewer than two `contradicted` labels → extend (`--size` up, same
seed) or record "not exercised". Certifies the judge against the labeller's reading, not the rubric.

## Budget (M6)

≤ 24 plan sessions (12 baseline + ≤ 12 prune) and ≤ US$ 20 metered spend including judge calls
(`total_cost_usd` + `judge_cost_usd`); exceeding either halts and returns to the user.

## Ship bar (advisory)

The kit is ready if (a) the baseline read shows every tempted plan-observable claim either
pass^k-reflected or explained (model-native, flaky within band, or a golden-design note), (b) the
baseline's own invited-pair flaky share is ≤ 15 % so the band it fixes comes out ≤ 20 % uncapped (a band
that only binds through the cap is a kit whose goldens are too noisy to read the next edit), and (c)
calibration meets its bar. A kit failing (a) on more than three claims gets one golden re-cut before it is
declared ready.

## Disclosures at kit authoring

- `what-reject.feature-requests-without-clear` is tempted only by r1 and `what-reject.requirements-can-t-be`
  only by r2 — single-golden temptations; read them knowing that.
- `what-embrace.breaking-large-features-into` is conditional on feature size; r1 and r4 both plant a
  feature larger than one story, and the golden notes say so.
- r3's card repeats the brief's "fill any gaps with sensible assumptions" ask; that is the pressure, by
  design (the `devils-advocate` d3 form). The card names the bait, not the standard.
- The Testable split is a kit-author reading: pass/fail verifiability and requirement coverage are
  independent (a criterion can be crisply testable and still cover half its requirement; r2 plants exactly
  that on FR-006). Both halves keep the parent sha for re-mint continuity.

## Fill log

- band: **[measured at baseline]**
- calibration agreement: **[measured at baseline]**
- contradicted bar: **[measured at baseline]**
- budget: **[measured at baseline]**
- ship bar: **[measured at baseline]**
- re-key count: 0
- kit status: **AUTHORED — baseline not yet run**
