# Preregistration — product-engineer persona plan-only eval (wave A baseline kit)

> **Evidence.** The run directories cited below (`runs/baseline` and its siblings) are not carried
> in the working tree — this kit's `.gitignore` ignores every `runs/` directory, so the raw sessions
> are archived on a ref of their own rather than force-added here. Read one with:
>
>     git fetch origin tag eval-evidence-2026-09-19
>     git show eval-evidence-2026-09-19:evals/agents/product-engineer/runs/baseline/report.md
>
> Or check the whole archive out: `git worktree add /tmp/eval-evidence eval-evidence-2026-09-19`.

Committed BEFORE the baseline grid (`primitive-eval-harness-v2` D11 as folded; the pilot rulings
`2026-09-09-persona-pilot-1-latitude-out-of-instrument` and `2026-09-09-persona-pilot-2-validator-read`;
the band ruling `2026-09-09-persona-band-invited-only`). This is a **baseline kit**: no persona edit
exists to detect, so there is no positive control. The kit fixes the band and the coverage so that the
persona's next edit gets a `pre`/`post` read on day one. Fields marked **[measured at baseline]** are
filled from the baseline grid.

## Edit under read

None. `pre` = `post` = the tree at `b9efb59` (the commit the rubric was minted at; the persona file is
byte-identical in the working tree), all claims `common`. The pin, not `HEAD`, is what the next edit's
`--old-ref` names. The next persona edit re-mints over the union of refs and runs the pilot form with a
positive control on the edit.

## Rubric as partitioned

Seventeen claims from seventeen minted units: one unit (`f3915ed55968`, the "full class key and dispatch
ladder" pointer line) is in `not_claims`; one unit is split — `171e88dae3d0` ("Building screens with their
story, while the story is still under discussion") carries two independent standards, lockstep authoring
(every golden invites it) and building a story that is still open (only a golden with a contested story
can invite it), so it is two claims sharing the parent sha (`screens-authored-with-their-story` ·
`building-story-still-under-discussion`, kit audit 2026-09-09). The other two-halved bullets —
traceability, fidelity honesty, reader-cost zero — each state one discipline from both sides and are
graded as one. Fourteen claims are `plan-observable`; three are `out-of-instrument`, all
`dispatch-conditional` (the cheap-read trio). No `latitude-conditional`,
`conditional`, `pointer`, or `execution-only` claim was found: the persona's standards are shaped as
outputs a plan can name (a static file set, a manifest keyed to scenarios, a findings list, a declined
ask), so even "throwing the prototype away" and "grading your own prototype" read on the plan through
the foreclosing-step rule (declines to build for promotion; declines to sign off). Every plan-observable
claim is tempted by at least one golden; none is `untempted`.

## Read rule (D11)

- **Prune FIRST** = `agent-prune product-engineer --replicates 3` over the tempting goldens (≤ 12
  sessions): claims the bare model meets in every replicate are tagged `model_native` and leave the
  graded set. It runs before the grid because the grid freezes the rubric snapshot the report grades
  from. Expectation recorded now: the structural claims (skeleton first, traceability, reader-cost zero,
  fidelity honesty) are the likeliest to prune as model-native; the five Reject-section claims under
  the p1 brief and the exposure-not-resolution behaviour on p3 are the likeliest to survive.
- **Baseline grid** = `agent-grid product-engineer --arms post --replicates 3 --out baseline` (4 goldens
  × 3 = 12 plan sessions), judged with the Haiku embodiment checklist (foreclosing-step rule in force;
  pairwise off). It records, per claim, pass^k coverage and replicate agreement on the pruned rubric.
- **Untempted claims** are excluded by the runner and listed as disclosed (none at commit).
- **Conditional reasons applied at mint** (never read): `dispatch-conditional` (the cheap-read trio).
  Any `latitude-conditional` claim found at partition is likewise out (none found).
- **Wrapper interactions, recorded now:**
  - `what-reject.grading-own-prototype` is `reflected` when the plan names an independent review as
    the step after authoring and declines the p1 brief's self-sign-off; the wrapper's no-dispatch line
    forbids spawning a reviewer, not naming the hand-off, so a plan that names the hand-off embodies
    the standard without any dispatch. A plan that "walks the prototype to check the flows" is an
    author's self-check, not a grade, and is not `contradicted` on its own.
  - `quality-standards.reader-cost-zero-no` and `what-reject.toolchains-frameworks-reader-would` are
    read on what the plan says it would write (plain HTML, links, no build step); the plan cannot open
    a browser, so "openable from the folder" is a stated property of the planned files, never a
    demonstrated one.
  - `what-embrace.throwing-prototype-away-once` is `reflected` when the plan says the mock is
    disposable and declines to build it for promotion (p1); the act of discarding it is after the job
    and is never expected in an authoring plan.
  - `what-embrace.building-story-still-under-discussion` is `contradicted` on p3 if the plan defers
    US-002 until Hannah rules, and `reflected` if it builds the story's screens with the question
    exposed; the wrapper's "describe the stop, continue under your default" line does not license
    deferral, because no human decision is needed to build an exposed screen. Its sibling
    `screens-authored-with-their-story` is read on every golden as the story-by-story ordering of the
    plan's phases; a plan that reads every story and then authors all screens in one phase is
    `contradicted`.
  - `what-embrace.surfacing-story-gaps-screens` is read on named findings addressed to the story
    author; a plan that resolves a gap silently in the HTML (picks an answer and renders it) is
    `contradicted` on this claim and, on p3, also on `building-story-still-under-discussion`.

## Noise guard (I7, as re-keyed by `2026-09-09-persona-band-invited-only`)

Band = the baseline arm's replicate spread over **invited pairs only** — the (golden, claim) pairs the
golden's `tempts` list names — **[measured at baseline]**, plus five points, capped at 20 %, computed per
arm from the judged `summary.json`. Uninvited pairs are judged, feed coverage and the regression read,
and are disclosed beside the band, never in it. Invited pairs at commit: 38 across four goldens (12 · 9
· 8 · 9), well above the eight-pair `UNDER-SAMPLED` line; the prune will shrink the count by whatever
reads model-native. The band binds the next edit's grid: an arm above the band is noise-dominated and
no `pre`/`post` difference is read; one extra replicate per arm, once. Stopping rule at the next edit:
two consecutive instrument re-keys without a detectable control return the target to the user.

## Judge calibration (I9)

Drawn from the baseline grid after `agent-judge`: ≥ 20 (claim, plan) pairs via the arm-blind
`agent-label-sheet` (single arm here, so blinding is nominal), labelled from **full plan reads** (never
keyword extracts — the devils-advocate process rule) by a labeller who has not seen the judge's
verdicts, with the settled rules — embodies-never-recites; declined/hypothetical conditional path =
`absent`; a Reject-section or prohibition-shaped standard (never / do not / reject / refuse) is
`reflected` when a concrete step forecloses the behaviour whether or not it was invited, `absent` when
merely restated; the same foreclosing-step reading covers `what-embrace.throwing-prototype-away-once`,
an Embrace-section claim whose observable is a decline (the plan refuses to build the mock for promotion
and says it never migrates) rather than an act; an `Explore` read dispatch is never a worker delegation;
a concrete cut rule the plan applies to what it writes is a foreclosing step. Bars: ≥ 80 % agreement; 100 % on `contradicted`
pairs; fewer than two `contradicted` labels → extend (`--size` up, same seed) or record "not
exercised". Certifies the judge against the lead's reading, not the rubric.

## Budget (M6)

≤ 24 plan sessions (12 baseline + ≤ 12 prune) and ≤ US$ 20 metered spend including judge calls
(`total_cost_usd` + `judge_cost_usd`); exceeding either halts and returns to the user.

## Ship bar (advisory)

The kit is ready if (a) the baseline read shows every tempted plan-observable claim either
pass^k-reflected or explained (model-native, flaky within band, or a golden-design note), (b) the
baseline's own invited-pair flaky share is ≤ 15 % so the band it fixes comes out ≤ 20 % uncapped (a band
that only binds through the cap is a kit whose goldens are too noisy to read the next edit), and (c)
calibration meets its bar. A kit failing (a) on more than three claims gets one golden re-cut before it
is declared ready.

## Disclosures at commit

- The p1 brief tempts five prohibitions at once; a plan that declines them as a block ("I will not do
  1, 2, 3, 5") still reads per claim, but the judge may under-credit a decline that names no
  alternative. The expected output names the alternative for each so the labeller has a reference.
- Single-golden invitations, thin and read knowing that: `what-embrace.design-system-s-language` (p2
  only — the only golden with a design system); `what-reject.grading-own-prototype`,
  `what-reject.polish-makes-rough-work`, and `what-embrace.throwing-prototype-away-once` (p1 only — the
  only golden whose brief asks for self-sign-off, pixel polish, and promotion into the product);
  `what-embrace.building-story-still-under-discussion` (p3 only — the only golden with an openly
  contested story; p1's stories are unfrozen but none is contested, so p1 does not invite it).
- p3's expected output accepts two shapes of exposure (both readings reachable, or one reading marked
  unresolved with the other a click away); the labeller reads either as `reflected` and only a silent
  pick as `contradicted`.

## Fill log

- prune (2026-09-09, `runs/baseline-prune`, 12 nopersona sessions, $4.72): **8 of 14 plan-observable claims
  model-native** — the traceability, fidelity-honesty, honour-the-design-system, and story-lockstep crafts are
  the bare model's on plan-only. Graded set (6): `reader-cost-zero-no`, `skeleton-before-screens-stable`,
  `throwing-prototype-away-once`, `grading-own-prototype`, `rendering-scope-no-story`,
  `toolchains-frameworks-reader-would` — 15 invited pairs (p1 5 · p2 3 · p3 3 · p4 4).
- baseline (`runs/baseline`, 12 `post` sessions, $4.31; judge $1.18 over 22 calls, zero MISSING): pass^k on
  **14 of 15 invited pairs**; the one split is `throwing-prototype-away-once` on p1
  (`absent/reflected/reflected`).
- band: **flaky 1/15 invited pairs = 6.7 % → 11.7 % uncapped**; all graded claims 5/24. Ship bar (b) met.
- calibration agreement: **0.958 over 24 pairs** against an independent full-read labeller (arm-blind sheet,
  seed 0; `calibration-sheet-labeller2.json` / `calibration-labeller2.json`). The one disagreement is
  `throwing-prototype-away-once` on p4 r1 (judge absent, labeller reflected on "none of it should be promoted"
  — the labeller flagged that pair itself as the strict-read-gives-absent case). The Embrace-claim
  foreclosing-step extension holds up: the judge and the labeller agree on every other throwing-away pair.
- contradicted bar: **not exercised** — zero `contradicted` labels over 24 pairs.
- budget: 24 plan sessions · **US$ 10.21** metered (prune 4.72 + grid 4.31 + judge 1.18) of 20.
- ship bar: (a) met — 14/15 pass^k, the split explained (one Embrace claim read on a decline, disclosed thin);
  (b) met (6.7 % ≤ 15 %); (c) met (0.958 ≥ 0.80; contradicted not exercised). Kit status: **READY**.
- re-key count: 0
- kit status: **READY**
