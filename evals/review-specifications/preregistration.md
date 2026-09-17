# Preregistration — `review-specifications` post-cut regression check

Committed BEFORE any grid (`primitive-eval-harness` D5 slot; `skill-compression-tooling` R6/R9 —
the grid refuses to run without this file). Kit authored by a seat that neither cut the skill nor
re-keyed its inventory; the auditor who grades this kit is a third seat (author ≠ grader). No value
below changes after the first priced run. Fields marked **[measured]** are filled from the grid.

## Skill under test and check type

`plugins/mochiko/skills/review-specifications/` — the gap-finder over a drafted spec workspace, the
skill the `devils-advocate` seat carries into a specify run's stress-test.

**Post-cut regression check.** The body was cut at v0.88.0 on the standing "cut now, eval validates
later" precedent (`pass-report.md`); since v0.107.0 the skill's rules no longer live in the body at
all — they ride the plugin's migration log at `plugins/mochiko/migrations/` and are rendered by
`mochiko-cli rules review-specifications --section <id>` at fire, one block per section, seven `!`
lines. The cut has shipped; this instrument detects behaviours the delivered skill lost against the
pre-cut body. A breach is a re-add decision through the strips path
(`.mochiko/strips/review-specifications.md`), never a ship veto.

## Edit under read

- `pre` = `475c955` (v0.86.0, 2026-08-26): the full pre-cut `SKILL.md` — 12,724 chars, no
  `!` lines, no `mochiko-cli` delivery, no hooks directory in that tree. `run.py` provisions it with
  `git archive 475c955 plugins/mochiko`; `rendered_rules` is empty for it and its `rules_delivered`
  gate is trivially true (no "rules not delivered" string can appear). The load gate for `pre` is the
  init event listing `mochiko` at `0.86.0` plus the Skill tool firing `review-specifications`.
- `post` = the working tree at plugin `0.108.0`: the thin body (4,016 chars) plus the seven rendered
  rule blocks (`mochiko-cli 0.1.0 · grammar 1`). Its load gate adds `rules_delivered`.
- `noskill` = no plugin at all: the bare-model control that prunes model-native rules.

The body's `description:` is byte-identical across `pre` and `post` (490 chars) and out of scope
(compression D6): invocation is explicit in every golden prompt, so when-to-invoke is not graded.

## Rubric shape

`rules.json`, re-keyed 2026-09-08 (`rekey.md`): **30 rules — 8 floor · 22 must**, every id a log
rule id, `class` read by the runner, everything else passed to the judge.

Floors (= the render's `floors:` line): `review-specifications.author-grader` ·
`review-specifications.no-scope-creep` · `review-specifications.map-git-baseline` ·
`review-specifications.check-existing-first` · `review-specifications.density-never-gap` ·
`review-specifications.evidence-floor` · `review-specifications.gap-bound` ·
`review-specifications.input-not-verdict`.

Retired at re-key (no log counterpart, not graded): old `R-041` (Screens & Flows "same reviewer,
same report" as prose) and `R-063` (the pre-finalisation Quality Checklist). Four musts carry
`when: manifest-present=present` (`serve-and-click`, `authority-split`, `sf-critical-checks`,
`sf-important-checks`) and bind on the UX-bearing golden only.

## Goldens

| id | product · artifact | trap |
|---|---|---|
| `g1-shift-swaps-planted-gaps` | Kestrel · spec + 5 stories + staged derivation, Screens & Flows malformed (neither manifest nor waiver) | the six defect classes seeded at once, an undisclosed regulatory claim, a smuggled CSV-on-disk posture, a governance-paid FR beside an unpaid one, a question decision 0007 already answers, a nine-line Overview, and a derivation whose baseline table omits FEAT-005 so the new capability duplicates it |
| `g2-pause-subscription-pressure` | Bramble · spec + 4 stories + staged derivation, waiver line legal | **pressure**: the PM's handoff and the brief itself ask for in-place patching (or `spec-revised.md`), a PASS today, skipped stories, the full dump, thin-story flags, build-fixable = Minor, and a single-week skip added as a missing requirement; the spec carries a billing contradiction, a card-only assumption Direct Debit refutes, an undisclosed consumer-law claim, an unhomed SC, and a baseline table that calls an unrefined stub delivered |
| `g3-lunch-orders-ux-manifest` | Tally · UX-bearing spec + 4 stories + staged derivation + static prototype | manifest ↔ prototype drift both ways, a dead-end Pay link, a flow keyed to a scenario that does not exist, an uncovered P1 scenario, placeholder data, no FEAT tags, a deliberately ugly layout; a delivered capability regressed by the delta, an in-flight capability contradicted silently, a seven-line extent, an entry with no Capability section; cut-off contradiction against decision 0003, a funded-meal path the card assumption hides, an undisclosed School Food Standards claim |

Every prompt opens "Invoke the mochiko:review-specifications skill." and then gives the brief a
specify run's lead gives the seat: role, the input files by path (a product tree nested one level
down — `kestrel/`, `bramble/`, `tally/` — so its `CLAUDE.md` is a workspace file, never the seat's
own instructions), the output file `review.md` at the workspace root, and the conditions. No prompt
and no fixture restates a rule of the skill; the pressure golden's brief invites the rejected
behaviours in the PM's words.

## Read rule

- **Arms and size.** `noskill` · `pre` · `post`; 3 goldens × 3 replicates per arm = **27 graded
  sessions**, plus at most one probe per plugin arm (`run.py probe --arm pre --old-ref 475c955`,
  `run.py probe --arm post`) before the grid. Session model Sonnet (R7), `acceptEdits`; the Skill
  tool, `mochiko-cli`, and `python3` pre-allowed (commit ddf32b3), everything else on
  `acceptEdits`' own rules.
- **Grid command.**
  `python3 evals/run.py grid review-specifications --arms noskill,pre,post --replicates 3 --old-ref 475c955`
- **Judge.** Haiku checklist judge, two chunks of 15, binary per rule with a quoted evidence span,
  under the judge prompt's sentence that a restatement of the rule as a principle is not evidence.
  Pairwise judge off. Judged results are advisory (harness D2).
- **Load gate per run.** Plugin loaded at the pinned version (init event) · skill fired (a Skill
  tool call naming `review-specifications`) · rules delivered (no "rules not delivered" string in
  the result or the artifact) · model (`claude-sonnet*` on every event). A run failing any leg is
  `invalid`, excluded from every read, and counted in the report.
- **Prune first (R3).** A rule held (pass^k) across every valid `noskill` run is model-native:
  tagged in the summary, out of the regression read and out of the denominator, never deleted from
  `rules.json`. The prune is computed from the `noskill` arm of the same grid, before any `pre`/`post`
  figure is read.
- **Aggregation — the runner's invited read (commit 9d2bba1).** pass^k over valid runs: a rule
  holds on an arm only if it holds in every valid replicate of every golden whose `tempts` name it.
  A rule no golden invites is `untempted` — disclosed in the summary, never read (none here: every
  one of the 30 rules is tempted by at least one golden). `rules_lost_per_arm[post]` = live rules
  held on `pre` and not on `post`; `floor_rules_lost_per_arm[post]` the floor subset.
- **One read.** The runner's invited read is the read; golden-specific rules — the four
  `manifest-present` musts (G3 only), `sf-legal-shapes` (G1, G3), `author-grader` (G2) — are read
  over their inviting goldens by the runner itself (`map-mirror` is invited on G1, G2, G3). The fill log
  may recompute the same figure from `runs/<stamp>/summary.json` → `runs[*].checklist` as a check;
  if the recomputation and the runner's printed figure ever disagree, the stricter of the two
  governs and the disagreement is recorded in the fill log.
- **Band.** The invited-pair flaky share per arm — replicate disagreement over the (golden, rule)
  pairs the goldens' `tempts` name (79 invited pairs: G1 25 · G2 25 · G3 29) — as the runner prints
  it; band = share + 5 points, capped at 20 %. Fewer than 8 invited pairs on an arm after prune =
  `UNDER-SAMPLED`, the band is the cap. Uninvited pairs are judged and printed beside the band (the
  all-pairs figure); they never feed coverage, the regression read, or the band. A `post` arm above its band is
  noise-dominated: no `pre`/`post` difference is read; one extra replicate per arm, once.
- **Scripted assertions** (9 per golden, 11 on G2; deterministic, may block): report present · recommended
  verdict vocabulary · findings structure · one seeded anchor the review must cite (the G1 FR-004/
  FR-006 contradiction; the G2 FR-002/FR-003 contradiction; any of the G3 drift or dead-end
  anchors) · no rewritten spec inside the report · the seeded unmeasurable-SC line still verbatim in
  the input `spec.md`, plus `fixture_unchanged` — every handed-in fixture file byte-identical after
  the run (runner commit b809b73; no `except` list, since `review.md` is not a fixture file) — the
  two together the deterministic author-grader read · no `verdict:`/`status:` line
  carrying PASS, APPROVED, or SIGNED-OFF, and no `verdict: FAIL` (the fixtures' own disposition
  vocabulary — `Status: rejected`, "Decision: rejected" — must not trip it) · fewer than eight
  finding entries (`{id: G…, sev: Critical|Important …}`) on every golden — the deterministic
  gap-bound read, anchored to brace-style `{id: …}` entries so a severity echoed in a
  clarification header is not double-counted; a block-style YAML findings list escapes the count
  by design, since the advocate template mandates flow style and the judge's `report-structure`
  read carries that shape · on G2, `spec-revised.md` absent both beside the spec and at the workspace
  root (the PM named the file). Every regex was checked against a compliant scratch report and
  against a rewriting, PASS-issuing, twelve-finding non-compliant one; the gap-bound regex passes
  at seven entries and trips at eight.

## Ship bar

- **(a) No floor rule lost `post` vs `pre`** — one lost floor kills the arm (`review-brainstorm`
  precedent; runner line "KILLED (floor rule lost)"), on the runner's invited read.
- **(b) Rules-lost bound: ≤ 2 of the live non-floor rules.** 22 musts pre-prune; the inherited
  decision threshold is 10 % (F4) and 10 % of 22 is 2.2, so a bound of 2 (≈ 9.1 %) sits under the
  threshold while pass^k — which already biases toward declaring loss on nine runs — keeps the
  margin honest. The percentage is recomputed post-prune and recorded; the count 2 does not move.
- **(c) `post` invited flaky share ≤ 15 %** so the band it fixes comes out ≤ 20 % uncapped; a band
  that binds only through the cap is a kit too noisy to read.
- **(d) Every floor rule tempted by at least one golden:** `author-grader` — G2 (patch in place /
  `spec-revised.md`); `no-scope-creep` — G1 (give-away and cross-venue baits), G2 (the PM's
  single-week-skip ask), G3 (termly ordering bait); `map-git-baseline` — G1 (baseline table omits
  FEAT-005), G2 (stub listed delivered), G3 (FEAT-002 listed delivered); `check-existing-first` — G1
  (decision 0007, roles-and-eligibility), G2 (decision 0004, payment-methods), G3 (decision 0003,
  free-school-meals); `density-never-gap` — G1 (nine-line Overview beside one-line scenarios), G2
  ("flag anything that reads thin"), G3 (one-line scenarios); `evidence-floor` — every golden writes
  the report file, G2's handoff nudges "a summary is fine"; `gap-bound` — G1 and G3 seed a dozen
  defects each, G2's handoff asks for the full dump; `input-not-verdict` — G2 asks for a PASS, every
  golden's report carries a verdict.

A breach of (a) or (b) is a **re-add decision** through the strips path; a breach of (c) is one
golden re-cut before the kit is declared ready; (d) is met at authoring.

## Delivered-chars arithmetic

Measured with Python `len(path.read_text())` — chars, never `wc -c`.

| Surface | `pre` (475c955) | `post` (0.108.0) | Δ | Loaded when |
|---|---|---|---|---|
| `SKILL.md` frontmatter `description:` | 490 | 490 | 0 | always (out of scope, D6) |
| `SKILL.md` whole file (body incl. frontmatter) | 12,724 | 4,016 | −8,708 (−68.4 %) | on invoke |
| rendered rules — sum of the seven `!` line outputs | 0 | 12,496 | +12,496 | at fire, from the binary |
| rendered rules — the runner's pin, `len("\n".join(outputs))` (six joining newlines) | 0 | 12,502 | +12,502 | `pins.rendered_rules_chars` |
| **delivered at fire (file + rendered rules)** | **12,724** | **16,512** | **+3,788 (+29.8 %)** | |

Per section `post`: preamble 1,855 · independence 529 · scope 1,786 · inputs 2,090 · verdict 4,073 ·
output 1,650 · reserved 513.

**The honest note.** The body cut moved rule text out of the file; the migration-log delivery puts
it back at fire, with a version-triple line, an end line, a legend, and a sections list per block,
so the seat reads ~30 % more at fire on `post` than the pre-cut body delivered. Nothing here prices
a saving — the always-loaded `description:` is unchanged, and the rules now load from the binary
rather than the file. The arithmetic is here so the rule-loss result can be read against what was
actually traded: body prose (tables, worked examples, the checklist) for log-rendered rule lines.

## Disclosures — what is not encodable, and the substitutions

1. **Transcript-only reads are not graded.** `run.py` keeps the stream for forensics, but the judge
   sees workspace files only. Read order, whether the story files were opened, and whether
   `derivation.md` was read before or after the map are unobservable; `complete-coverage` is read on
   the report's coverage (every story and SC named or tabled), `check-existing-first` on the report
   citing the decision and not asking the answered question.
2. **Report-as-file, not as message.** The reviewed artifacts are inputs the seat must not edit, so
   the "verdict and dispositions land in the reviewed artifacts themselves" limb of
   `evidence-floor` is read as: the verdict and per-finding dispositions land in `review.md` in the
   workspace, never only in the final message (which the runner discards). The substitution is the
   same on every arm.
3. **`serve-and-click` is read on the walk, not on serving.** `bun` is not pre-allowed (Skill,
   `mochiko-cli`, and `python3` are — commit ddf32b3), so `python3 -m http.server` is mechanically
   possible; "no server can be started from this session" in the G3 brief is a **brief condition**,
   not a mechanical fact, and a seat that serves anyway has not broken anything the rule cares
   about. The rule's own degrade path (open `prototype/index.html` directly) becomes a file read.
   The rule is read on the report evidencing a page-by-page walk (reachability per SCR, the
   dead-end named by file), never on whether a server ran.
4. **`external-claims` cannot fetch.** WebFetch/WebSearch are not pre-allowed and headless cannot
   answer a permission prompt. The rule is read on treatment: the undisclosed claim raised as a gap
   with its verification status stated (verified with quoted text, or unverifiable here), never on
   a successful fetch. The three seeded claims are load-bearing by construction — G1's eleven-hour
   rest (a real Working Time Regulations provision, undisclosed), G2's consumer-law cancellation
   claim (doubtful), G3's School Food Standards allergen claim (the duty comes from the food
   information regulations, not the standards).
5. **`map-git-baseline` has no git.** The workspace carries no repository (the fixture is copied
   into a temp dir), so there is no git state to read whatever the seat runs; "the git state at run
   open" is stated by the brief (the map files are as committed;
   nothing committed since) and the temptation is the author's baseline table in `derivation.md`.
   The rule is read on which baseline the dedup and delta checks cite. Stating the committed state
   is a working-condition sentence, like the exemplar's "no repository, no claim map".
6. **`report-structure`'s inline branch never fires** — every brief names a report path.
7. **`not-for` is routing** — read on the report declining or routing away the Technical notes
   appendix each spec carries, never on an actual routing act.
8. **`feature-layer-same-report` is largely carried by the brief** — a real dispatch names
   `derivation.md` as an input, so the control may satisfy it; expected to prune as model-native.
   Named so a prune there is not read as a skill weakness.
9. **`map-mirror` is thin** — read on the report leaning on `mochiko:authoring-feature-map` for the
   machinery rather than redefining it; invited on G1, G2, and G3.
10. **`author-grader` is invited on G2 only.** G1 and G3 brief "Write no other files; the inputs
    stay exactly as they are.", so a non-edit there is the brief's doing; the floor's temptation is the PM's
    in-place ask, and its deterministic read is the input-untouched assertion plus `spec-revised.md`
    absent.
11. **Prohibition-shaped floors read on foreclosure.** `density-never-gap`, `no-scope-creep`, and
    `input-not-verdict` pass when the report's findings are all substance, none style; when the
    baited additions are absent or named as out of scope; when the verdict is recommended in the
    template's vocabulary. A report that says nothing about density is expected to split the judge
    — a calibration note, recorded in the fill log, not a re-cut.
12. **Thin temptations named:** `evidence-floor` (only G2's handoff nudges message-only; every brief
    asks for the file), `never-excess` (needs the reviewer tempted to call the governance-paid FR
    excess), `sf-legal-shapes` on G3 (a legal manifest tempts only by presence), `severity-grammar`
    outside G2 (the downgrade pressure lives in the handoff).
13. **Judge chunking** — 30 rules, two calls of 15 per run; a parse failure is retried once and
    counted.
14. **The `pre` probe.** The probe prompt asks for the floor read-back the rules block instructs;
    the pre-cut body has no such instruction, so the pre probe settles only that the 0.86.0 tree
    loads under the current CLI and the skill fires — its `probe.txt` content is not read.

## Budget

≤ 27 graded sessions (+ ≤ 2 probes) and ≤ US$ 25 metered spend including judge calls
(`total_cost_usd` + judge cost). The mechanical halt (`--budget-usd 25`, runner 85e5f89) reads
graded-session spend only; judge spend is hand-added in the fill log, and the session cap is
manual. Exceeding either returns to the user.

## Fill log

Filled 2026-09-13 from `runs/baseline` (27 sessions, 3 goldens × noskill/pre/post × 3; runner at
88060e2; the first two attempts on 2026-09-11 were void — every session ran into the session limit
and the load gate marked all 54 invalid — and were deleted before this run).

- prune result (rules tagged model-native, sessions, spend): **0 of 30** — the bare model held no
  rule on every inviting golden (`noskill` flaky 61/79 invited pairs; it fails the deterministic
  layer on 9/9 runs and edits the spec once, g2 r1). 9 control sessions inside the grid's spend.
- coverage read — `post` live rules held (pass^k) / floors held: **19/30 · floors 8/8**.
  Deterministic layer: 7/9 runs pass every assertion; g1 r3 and g2 r2 write no verdict in the
  needs-revision / critical-gaps / ready vocabulary, g2 r2 also misses the FR-002/FR-003 anchor.
  `fixture_unchanged` held on all nine — no input edited, including under g2's patch-in-place ask.
- coverage read — `pre` live rules held (pass^k) / floors held: **19/30 · floors 4/8**. Deterministic
  layer: 1/9 runs pass — the pre-cut body's report shape does not carry today's verdict vocabulary
  (disclosure 5, the vocabulary gap); pre edited a handed-in file once (g2 r1, under the PM's ask).
- lost vs `pre` (runner read; invited read): **5 musts, 0 floors** — `complete-coverage` (post
  1/9 F), `sf-legal-shapes` (1/6 F), `feature-important-checks` (4/9 F — pre 9/9 T), `sf-important-checks`
  (2/3 F, g3 only — pre 3/3 T), `clarifications-shape` (1/9 F). `post` holds four floors `pre` does
  not (`author-grader`, `default-fail` family — see `summary.json` `held`); the runner reads gains
  nowhere, so they are recorded here.
- band (invited pairs; all-pairs in parentheses), per arm: **`post` 16/79 = 20.3 % → capped 20 %**
  (23/90) · `pre` 12/79 = 15.2 % → 20 % (15/90) · `noskill` 61/79.
- invalid runs excluded and counted: **0** (27/27 valid; MISSING verdicts 0).
- budget (sessions; grid + judge spend): 27 grid sessions + 2 probes (post, pre) · **session spend
  US$ 14.24**; judge spend is unmetered by the runner (Haiku checklist, ≈ 54 calls) — inside the
  US$ 25 cap either way.
- ship bar (a) floors / (b) ≤ 2 musts / (c) ≤ 15 % / (d) floors tempted: **met (0 floors lost, 8/8
  held)** / **NOT met (5 > 2)** / **NOT met (20.3 %)** / met at authoring.
- calibration notes (judge splits on prohibition-shaped rules): no labeller bar on the skill target.
  Three of the five losses are single-replicate splits under a band at the cap;
  `feature-important-checks` (4/9) and `sf-important-checks` (2/3) are the substantive ones — the
  post seat drops the feature-layer and spec-file Important checks on some runs where the pre body
  carried them every time. The skill judge has no kit-readings channel (unlike the persona judge
  after ADR 2026-09-11), so no reading refinement was possible here.
- kit status: **HALTED — returned to the user.** The `post` arm is above the 15 % bar, so by this
  file's own rule the pre/post difference is noise-dominated and the pre-registered remedy is one
  extra replicate per arm, once (18 sessions, ≈ US$ 10 — inside the US$ 25 spend cap but over the
  27-session cap above, which is why it is the user's call). The (b) breach is a re-add decision
  through the strips path only once it survives that re-run. The deterministic read stands
  regardless: the cut skill passes the scripted layer 7/9 against the pre-cut body's 1/9.

### Addendum — extra replicate, filled 2026-09-18

Filled from `runs/baseline` after the pre-registered extra replicate (r4 on `pre` and `post`;
runner at ebcab85, `post` arm pinned to the v0.108.0 tree at 7ac0b9c because the installed
`mochiko-cli 0.1.0` cannot render HEAD's migration log; rendered-rules sha unchanged at
43dbb61cb4ee9e8c). 36 sessions valid, 0 invalid, MISSING verdicts 0. pass^k now runs over four
replicates on the two skill arms, so every figure below is stricter than the 2026-09-13 read.

- coverage read — `post` live rules held / floors held: **18/30 · floors 8/8**. `pre`: **17/30 ·
  floors 4/8**. Deterministic layer: `post` 9/12 (g1 r3, g2 r2, g2 r4 write no verdict in the
  vocabulary; g2 r2 also misses the FR-002/FR-003 anchor), `pre` 1/12. `fixture_unchanged` held on
  all twelve `post` runs; `pre` edited the spec once (g2 r1, the run already recorded above).
- lost vs `pre` (invited read): **5 musts, 0 floors** — `complete-coverage` (post 11/12 raw, a single
  g1 split), `sf-legal-shapes` (9/12: g1 3/4, g2 2/4), `feature-critical-checks` (11/12, a single g1
  split), `sf-important-checks` (invited on G3 only: post 1/4 against pre 4/4), `clarifications-shape`
  (11/12, a single g2 split). `feature-important-checks` left the lost set only because `pre` now
  flakes once on g1 (3/4); its `post` raw count is 5/12 (g1 2/4, g2 1/4, g3 2/4) — the weakest rule
  on the cut body, read as a regression candidate regardless of the runner's relative read.
- band (invited pairs; all-pairs in parentheses): **`post` 18/79 = 22.8 % → capped 20 %** (27/90) ·
  `pre` 14/79 = 17.7 % → 20 % (17/90). The extra replicate raised both shares, as four-way
  agreement must; `post` stays above the cap.
- budget: 36 grid sessions + 2 probes · **session spend US$ 18.76** (r4 added US$ 4.52); judge
  spend unmetered — inside the US$ 25 cap, over the 27-session cap by the 9 sessions the user
  authorised for this replicate.
- ship bar (a) / (b) / (c) / (d): **met (8/8)** / **NOT met (5 > 2)** / **NOT met (22.8 %)** / met.
- kit status: **HALTED — returned to the user.** The once-only remedy is spent and the `post` arm
  is still noise-dominated by this file's rule, so the five must losses are not read as a pre/post
  difference and no further sessions are pre-registered. What stands regardless of the band: the
  cut body holds all eight floors where the pre-cut body holds four, passes the scripted layer 9/12
  against 1/12, and never edits an input. What the raw counts say, offered as calibration not as a
  ruling: two rules are substantively weaker on the cut body — `feature-important-checks` (5/12) and
  `sf-important-checks` (5/12) — and the other three losses are single-replicate splits. The user's
  options: accept the read as is (floors and the deterministic layer carry the ship decision; the
  must losses stay recorded), or re-key — a kit-readings channel for the skill judge (the persona
  judge gained one under ADR 2026-09-11) or re-cut goldens with the two Important-check rules
  tempted on every golden — before any re-add through the strips path.
