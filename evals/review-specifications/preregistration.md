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
(`total_cost_usd` + judge cost); exceeding either halts the grid and returns to the user.

## Fill log

- prune result (rules tagged model-native, sessions, spend): **[measured]**
- coverage read — `post` live rules held (pass^k) / floors held: **[measured]**
- coverage read — `pre` live rules held (pass^k) / floors held: **[measured]**
- lost vs `pre` (runner read; invited read): **[measured]**
- band (invited pairs; all-pairs in parentheses), per arm: **[measured]**
- invalid runs excluded and counted: **[measured]**
- budget (sessions; grid + judge spend): **[measured]**
- ship bar (a) floors / (b) ≤ 2 musts / (c) ≤ 15 % / (d) floors tempted: **[measured]** / **[measured]** / **[measured]** / met at authoring
- calibration notes (judge splits on prohibition-shaped rules): **[measured]**
- kit status: **[measured]**
