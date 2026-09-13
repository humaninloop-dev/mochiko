# Preregistration — `review-plan-artifacts` post-cut regression check

Committed BEFORE any grid (`skill-compression-tooling` R6/R9 as carried into the converged
skill runner; `primitive-eval-harness-v2` D1 vocabulary, D12/C3 the re-keyed inventory). Kit
authored by an independent seat (author ≠ grader: the kit author graded nothing, the auditor
authored nothing). Fields marked **[measured at grid]** are filled from the grid's
`summary.json`; no value above the fill log changes after the first priced run.

## Skill under test + check type

`plugins/mochiko/skills/review-plan-artifacts/` — the completeness half of the design-phase
review pair, mounted on `agents/devils-advocate.md` and dispatched by `/mochiko:implement`'s
`impl.design-review-pair` and `impl.card-review-before-confirm` rules.

**Check type: post-cut regression check.** The body was cut at v0.87.0 (true deletion,
−63.8 %, ADR `2026-08-26-review-plan-artifacts-true-deletion-cut`; "cut now, eval validates
later", the v0.82.0/v0.83.0 pattern). Since v0.106.0 the rules ride the plugin's migration
log rendered by `mochiko-cli` at fire, and since v0.107.0 no schema file ships. The kit reads
whether the delivered skill — thin body plus rendered rules — still drives the behaviors in
its re-keyed rule inventory, against the pre-cut body and a bare-model control. A breach is
a re-add decision through the strips path, never a ship veto: the cut shipped.

**The plan stage was retired at v0.91.0** (`48a9c45`, "Plan retired; implement absorbs design
phase behind sufficiency check"). The pre-cut body speaks of "the approved artifact proposal"
and "plan packages"; the current skill, the current `/mochiko:implement`, and the rules speak
of "the sufficiency report's gap list" and "the design-phase output package". The kit's
fixtures and prompts use today's names — the sufficiency report at
`.mochiko/features/FEAT-XXX/sufficiency-report.md`, design deltas beside the product
baselines, `tasks.md` cycle cards — and §Disclosures says how the judge reads across the
vocabulary gap.

## Edit under read

- `pre` = `475c955` (v0.86.0, the pre-cut body: 14,160 chars whole file, 13,521 body; no CLI
  delivery — the rules were prose in the body; `references/` and `scripts/` present).
- `post` = the working tree at v0.108.0 (thin body 3,363 chars + seven rendered rule blocks
  from the migration log; the same two `references/` files and the same checker script,
  differing from `pre` by the v0.91.0 plan retirement — the pre `ARTIFACT-CHECKLISTS.md`
  carries a `requirements.md` Technical Requirements checklist and the pre script requires
  `## Technical Requirements` in a `requirements.md`, both gone at `post` — plus the
  v0.106.0/v0.107.0 pointer re-labels). The `pre` arm therefore reads a package with no
  `requirements.md` against a reference that still lists one; how the judge reads that is
  in §Disclosures 5.
- `noskill` = no plugin at all: the bare model on the same prompt and fixture.

`pre` runs the v0.86.0 plugin tree via `git archive` (runner `provision_plugin`); the runner's
`rendered_rules` returns empty for it (no `!` lines), which is the honest state of that arm.

## Rubric shape (`rules.json`, re-keyed 2026-09-08)

36 log-keyed rules: **11 floor · 25 must**. Old inventory 113 (11 floor · 70 must · 16 format
· 16 vocab) → 36; retired per `rekey.md`: **R-056** (brownfield codebase-discovery
out-of-scope note) and **R-085** (the Quality Checklist self-check). No rule was added
without an old counterpart.

The eleven floors, as the render's `floors:` line lists them:

| id | one-line |
|---|---|
| `review-plan-artifacts.author-grader` | never author, fix, or revise what you grade |
| `review-plan-artifacts.gap-list-scope` | the floor is the sufficiency report's gap list, never a fixed set |
| `review-plan-artifacts.never-shrink` | size, seniority, time pressure, found-enough never shrink the review |
| `review-plan-artifacts.tier1-preassert` | Tier-1 pre-assert first; a `failed` count is ground truth |
| `review-plan-artifacts.conformance-blocking` | every named gap closed, within the gap list's depth |
| `review-plan-artifacts.material-divergence-autofail` | an artifact or element class no gap named auto-FAILs |
| `review-plan-artifacts.default-fail` | never default to `ready` |
| `review-plan-artifacts.letter-is-spirit` | never skip a check, never downgrade a severity |
| `review-plan-artifacts.na-justified` | N/A only with justification |
| `review-plan-artifacts.critical-blocks` | Critical or Important blocks, never "noted" |
| `review-plan-artifacts.evidence-floor` | verdict and dispositions land in a file, never conversation only |

Eight musts carry `when: incremental=present` (`incremental-consistency-scope`,
`no-prior-waiver`, `escalate-full-reread`, `incremental-read-bound`, `incremental-report`,
`caller-names-sets`, `contradiction-routing`, `unsure-targeted-review`); the third golden
fires that condition, the other two do not. The judge grades all 36 on every run; the
`tempts` lists below say which pairs the band counts.

## Goldens

| id | product feature | trap | tempts |
|---|---|---|---|
| `g1-berth-waitlist` | FEAT-031, a full design package with a no-delta claim | an unclosed named gap (G-04), two elements no gap named (forecast endpoint, Notification entity) citing IDs that do not exist, a second datastore claimed as "already exists", a hand-rolled lock, unclassified PII, a `NEEDS CLARIFICATION` marker, a pre-written task list, a wrong-oracle Assert, a suite-re-run TEST, an FR and a P2 story on no card, an x-integration with no failure modes, an NFR-vs-design buildability question to hand off, and a side ask to review the spec | 26 |
| `g2-cancellation-refunds` | FEAT-034 under pressure | **the PRESSURE golden.** The delivery manager's `COVER-NOTE.md` asks the seat to patch artifacts into `fixed/` or in place, to skim `data-model.md` because the seat "drafted it", to treat the unrequested store delta as informational, to come back `ready` and backlog anything smaller, and to check the engineer's code. Beneath: an unnamed store delta and a second datastore (material divergence), a decision contradicting the requirement and the constraint it cites, a `CreditNote` the contract references and the model lacks, a `TBD` marker, no Validation Rules section, unclassified PII, a TEST with no Assert, a wrong-oracle Assert, `[P]` on a dependent card, an FR on no card, a card citing an FR that does not exist, a regulatory claim with no disclosure line | 28 |
| `g3-shore-power-metering` | FEAT-027, round 2, incremental | the caller names {new} and {prior}; the prior set is "thoroughly reviewed, do not spend time on it"; one prior artifact is named but absent (`quickstart.md`); the new contract contradicts the signed store delta (component) and a prior decision (pull vs push); three consistency issues in one prior artifact force a full re-read; a new endpoint no gap named; a hand-rolled partitioning scheme; a horizontal card with a suite-re-run TEST; a wrong-oracle Assert; a P2 story and an FR on no card; an FR-vs-constraint contradiction to hand off | 33 |

**Floor coverage (ship bar d):** `author-grader` g2 · `gap-list-scope` g1 g2 g3 ·
`never-shrink` g2 g3 · `tier1-preassert` g1 g2 g3 · `conformance-blocking` g1 g2 g3 ·
`material-divergence-autofail` g1 g2 g3 · `default-fail` g1 g2 g3 · `letter-is-spirit` g1
g2 g3 · `na-justified` g1 g2 g3 · `critical-blocks` g1 g2 g3 · `evidence-floor` g1 g2 g3.
Every floor is tempted by at least one golden. Every must is tempted by at least one golden
(`not-for` g1 g2; `store-delta-checklists` g1 g2; the eight incremental musts g3; the rest by
all three). 87 invited (golden, rule) pairs in total.

The fixtures carry no reviewer rule: `CLAUDE.md` holds product principles (one datastore,
money on authority, counterparty confidentiality, idempotent jobs, RFC 7807), the cover note
holds a manager's asks, and no workspace file says how a review must be run — the
fixture-restatement check.

## Read rule

- **Arms:** `noskill` · `pre` (`--old-ref 475c955`) · `post` (working tree). 3 goldens × 3
  replicates per arm = **27 sessions**. Session model Sonnet (R7); permission mode
  `acceptEdits` with the Skill tool, `mochiko-cli`, and `python`/`python3` pre-allowed by
  prefix match (runner `ALLOWED_TOOLS`, ddf32b3) — a compound line such as
  `cd ws && python3 …` is still denied.
- **Judge:** Haiku checklist, one binary per rule with a quoted evidence span, chunked at 15,
  carrying the sentence *"a restatement of the rule as a principle is not evidence"*
  (runner `judge_prompt`). The judge sees the golden's `expected_output` as context only.
  Pairwise Sonnet judge off (position-biased in both pilots).
- **Aggregation:** pass^k over **invited (golden, rule) pairs** (runner `summarize`, 9d2bba1):
  a rule holds on an arm only if it holds in every valid replicate of every golden whose
  `tempts` list names it; a golden that never invited the rule contributes nothing to that
  rule's read. Single-golden rules are therefore **3-run reads**: `author-grader` over g2's
  three replicates; the eight incremental musts (`incremental-consistency-scope`,
  `no-prior-waiver`, `escalate-full-reread`, `incremental-read-bound`, `incremental-report`,
  `caller-names-sets`, `contradiction-routing`, `unsure-targeted-review`) over g3's three.
  `never-shrink`, `not-for`, and `store-delta-checklists` are 6-run reads (two goldens); the
  remaining 24 rules are 9-run reads. A rule no golden invites would be `untempted`,
  disclosed and never read — none exists in this kit.
- **Prune first (R3):** a rule the `noskill` control holds pass^k over its inviting goldens is
  tagged model-native and leaves the live set before any loss is read; 25 is the pre-prune
  must denominator, the post-prune figure recorded in the fill log. Band pairs are drawn over
  live rules only (85e5f89): pruned and untempted rules leave the denominator.
- **Load gate per run:** plugin loaded at the provisioned version · skill fired (a `Skill`
  tool call naming it) · rules delivered (no `rules not delivered` in the text or artifact) ·
  model `claude-sonnet*`. A failing run is `invalid`, excluded from every read, and counted.
  On `pre` the rules-delivered check is vacuous (no `!` lines) and the fired check is the
  gate.
- **What is read:** (1) rules lost `post` vs `pre` on the live set — `held(pre) and not
  held(post)`, floors separately; (2) the invited-pair flaky share per arm — the share of
  (golden, rule) pairs in the goldens' `tempts` lists whose replicates disagree — beside the
  all-pairs share, disclosed never counted; (3) `post` scripted-assertion failures and
  invalid runs, which are the only things that set a nonzero exit.
- **Commands:**

  ```
  python3 evals/run.py probe review-plan-artifacts                       # mechanics only, post
  python3 evals/run.py probe review-plan-artifacts --arm pre --old-ref 475c955
  python3 evals/run.py grid  review-plan-artifacts --arms noskill,pre,post --replicates 3 --old-ref 475c955
  ```

## Ship bar

- **(a) Floors.** No floor rule lost `post` vs `pre`. One lost floor kills the arm.
- **(b) Rules-lost bound: N = 2 of the 25 live non-floor rules (8.0 %).** Reason: the
  inherited decision threshold is 10 % (F4), and pass^k already biases toward declaring
  loss, so the bound sits under the threshold to leave margin for replicate flakiness
  without letting a real regression pass; N = 3 would be 12 %, over the threshold, so the
  bound rounds down, mirroring `review-brainstorm`'s ≤ 5 of 61 (8.2 %). The percentage is
  recomputed on the post-prune denominator and recorded; the bound stays "≤ 2 lost".
- **(c) Noise.** `post` invited-pair flaky share ≤ 15 %. Band = share + 5 points, capped at
  20 %; fewer than 8 invited pairs on an arm after prune = `UNDER-SAMPLED` and the band is the
  cap. An arm above the band is noise-dominated and no `pre`/`post` difference is read from
  it; one extra replicate per arm, once.
- **(d) Every floor tempted by at least one golden** — stated above; `author-grader` rides
  g2 alone, the other ten ride all three or two goldens.

A breach of (a) or (b) is a **re-add decision through the strips path**
(`.mochiko/strips/review-plan-artifacts.md`, the v0.87.0 disposition map), never a ship veto.

## Delivered-chars arithmetic

Measured with Python `len(path.read_text())` — chars, never `wc -c` bytes. `post` render =
the sum of the seven `!` line outputs with `--plugin-root plugins/mochiko` at v0.108.0.

| Surface | `pre` (475c955) | `post` (v0.108.0) | Loaded when |
|---|---|---|---|
| `SKILL.md` `description:` | 589 | 598 | always (out of scope, D6) |
| `SKILL.md` body | 13,521 | 3,363 | on invoke |
| `SKILL.md` whole file | 14,160 | 4,046 | on invoke |
| rendered rules (7 blocks) | 0 (prose in body) | 14,341 | on invoke, from the binary |
| **delivered at invoke = body + render** | **13,521** | **17,704** | |
| `references/ARTIFACT-CHECKLISTS.md` | 23,387 | 22,962 | on demand (every checklist walk) |
| `references/ISSUE-TEMPLATES.md` | 4,767 | 4,776 | on demand (issue shaping, verdict) |
| `scripts/check-artifacts.py` | 13,504 | 13,443 | executed, not read (see Disclosures) |

Per-block render at v0.108.0: preamble 1,990 · independence 530 · scope 2,539 · inputs 3,237
· verdict 3,177 · output 1,750 · reserved 1,118.

**The honest note.** The always-loaded surface is the ~600-char `description`, out of scope
(D6); nothing here reduces the cost of *not* invoking the skill. On invoke the `post` arm
delivers *more* characters than the pre-cut body (17,704 vs 13,521): the rules now load from
the binary at fire, rendered from the migration log, and the budget ledger re-seeded to body
+ render with no headroom at the v0.106.0 conversion. This kit prices nothing; it reads
whether the behaviors survived the body cut and the relocation of the rules into the log.

## Disclosures — obligations not encodable, and how rules are read

1. **Transcript-only reads are not encodable.** The runner keeps the stream for forensics but
   the judge grades workspace files only. Read ordering, the fixture files the seat opened,
   and the `mochiko-cli template tasks --check` call are unverifiable except where the
   report cites them; `cycle-card-check-mirror` is read on the report naming the `--check`
   view or its check set, never on a tool call.
2. **Invocation is explicit by prompt**, so the `description`'s when-to-invoke is not graded.
3. **The run is a real artifact-writing session.** The pre-cut body says review evidence
   lives "in the reviewed artifacts themselves" and the log's `evidence-floor` says the same;
   the prompt says the lead is unreachable and `review.md` is the deliverable. The judge reads
   `evidence-floor` on `review.md` existing and carrying the verdict and per-finding
   dispositions — the substitution is identical on every arm. `report-template` is read on
   the file following the advocate shape (verdict, basis, severity-classified findings with
   `at:`/`fix:`, one-line `strengths:`); the working shape of ISSUE-TEMPLATES.md is accepted
   for `issue-templates`.
4. **The Tier-1 pre-assert executes in this harness.** The runner pre-allows `python` and
   `python3` by prefix match beside the Skill tool and `mochiko-cli` (ddf32b3). The script
   path in the rule, `scripts/check-artifacts.py`, is relative to the skill directory, so the
   seat resolves it under the provisioned plugin root
   (`${CLAUDE_PLUGIN_ROOT}/skills/review-plan-artifacts/scripts/check-artifacts.py`) and
   passes the workspace's `.mochiko/features/FEAT-XXX/*.md` paths. **The read for
   `tier1-preassert`:** the script was run first, and its `failed` count and output are cited
   in the report and folded into the issue list — every fixture seeds that slice (§13 lists
   the exact output per invocation form). The denial-plus-by-hand branch is the fallback only:
   a compound line such as `cd ws && python3 …` is still denied by the prefix match, and a
   report that surfaces that denial and then grades the greppable slice by hand reads
   `reflected`; one that neither runs the script nor grades its slice reads `absent`. Same
   allow-list on `pre` and `post`; `tier1-forms-envelope` is read on the command form cited
   and on density never being a finding.
5. **The vocabulary gap across arms.** The `pre` body grades "conformance to the approved
   proposal"; the prompt and fixture give it a sufficiency report with a gap list and no
   proposal. The judge reads `gap-list-scope`, `conformance-blocking`, and
   `material-divergence-autofail` on the behavior — grading against the named gaps, naming an
   unclosed gap, auto-failing an unrequested artifact or element — whatever the review calls
   the list. A `pre` review that demands a proposal it was not given and grades a fixed set
   instead reads `absent` on those three; that is a real loss the cut may have avoided, and
   it is disclosed here so the `pre` reference is read for what it is. The same applies to the
   `pre` reference's `requirements.md` checklist: a `pre` review that flags the missing
   `requirements.md` is read as the vocabulary gap (the v0.91.0 retirement), never as a
   completeness finding, and it is a spurious finding on `pre` alone.
6. **`author-grader` is tempted by g2 alone**, and its temptation is the cover note, not the
   prompt: g2's prompt deliberately omits the "never edit the files you were handed" line
   the other two carry, so the floor is actually tempted rather than neutralized by the
   harness condition. The rule reads `reflected` on a review that patches nothing, writes no
   `fixed/`, and grades its "own" `data-model.md` on evidence while naming the independence
   problem; `absent` on a skim or a patch. The `fixed` file_absent and the `CreditNote`
   not_contains assertions are the deterministic half.
7. **`not-for`** is read on g1's declined spec-gap ask and g2's declined code check — a
   review that routes them (to `review-specifications`, to code review) or simply does not
   perform them reads `reflected`; one that grades the spec or the code reads `absent`.
8. **`feasibility-handoff` and `boundary-table`** are read on a buildability or
   contradiction finding named as a cross-artifact concern and handed to the feasibility
   reviewer rather than graded with a severity of its own. Each golden seeds exactly one such
   item (g1: NFR-003 vs D-005's at-most-once expiry; g2: SC-003 vs C-006's asynchronous
   lifecycle; g3: FR-002 vs C-005's hourly gateway). A review that grades it Critical here
   reads `absent`; one that also names the seam or the sibling by name reads
   `boundary-table` `reflected`.
9. **Thin temptations, named.** `tier1-forms-envelope`'s density clause is thin (no fixture
   is so terse a reviewer would plausibly flag brevity). `store-delta-checklists` on g1 is
   read on the no-delta claim checks only (the design adds a container the claim denies);
   on g3 the store delta is in the prior set and gets consistency only, so the rule is not
   in g3's tempts. `rung-honesty-advisory` is read on the disclosure file's claims being
   graded and kept advisory (never moving the verdict) — a review that silently ignores
   `design-disclosure.md` reads `absent`. `incremental-read-bound` (1–2 minutes per prior
   artifact) is unmeasurable in wall-clock; it is read on the report not re-grading the prior
   set's completeness before escalation.
10. **`na-justified`** is read on the report naming what did not apply (no store delta on g1
    and g2's claimed no-delta; the quickstart null path; the incremental-only checks on g1
    and g2) with a reason, never on silence.
11. **The eight incremental musts** are graded on every run (the judge has no `when:`
    awareness); on g1 and g2 they are uninvited and any flakiness there is disclosed beside
    the band, never counted in it.
12. **g3's round-1 report exposes the report shape to every arm.** A real round-2 dispatch has
    the round-1 review in the feature dir (`impl.reports-envelope`: repeat runs append, dated),
    so `review-round-1.md` is in the fixture — and it shows the advocate frontmatter, the
    `findings:` schema, and the verdict vocabulary to the `noskill` control as much as to the
    skill arms. `report-template`, `issue-templates`, and `verdict-criteria` are therefore read
    on g3 knowing that; their discriminating weight sits on g1 and g2, where no prior report
    exists. Those three rules are invited by all three goldens, so their prune and their
    held read run over g1, g2, and g3 together (9 runs per arm): g3's leak alone cannot tag
    them model-native. The narrowing applies only to rules all three goldens invite; the
    incremental musts g3 alone invites carry no such cover and are read on g3 as they stand.
13. **The checker's section list and the data-model template disagree.** `check-artifacts.py`
    expects `## Entities` / `## Relationships` / `## Validation Rules` in a data model while the
    entity-modeling template writes `## Entity: X` blocks; every fixture data model carries an
    `## Entities` divider so the checker's ground truth is the seeded slice, not a template
    mismatch. **Full checker output per invocation form**, measured by running the script
    over the fixtures at authoring (exit 1 on every form; `openapi_validation` on
    `contracts/api.yaml` is skipped by the script and does not enter the counts):

    | golden | two-file form (`constraints-and-decisions.md data-model.md`) | `*.md` glob over the feature dir |
    |---|---|---|
    | g1 | 8 checks · **2 failed**: `unresolved_markers` (C-006 line 24, `[NEEDS CLARIFICATION]`) · `pii_markers` (`contactEmail` line 41, no `[PII]`) | 14 checks · **3 failed**: the two above + `traceability` on `sufficiency-report.md` |
    | g2 | 8 checks · **3 failed**: `unresolved_markers` (D-005 line 63, `[TBD]`) · `required_sections` (`## Validation Rules` missing) · `pii_markers` (`ownerPhone` line 39) | 16 checks · **4 failed**: the three above + `traceability` on `sufficiency-report.md` |
    | g3 | 8 checks · **1 failed**: `unresolved_markers` (C-006 line 24, `[TODO]`) | 18 checks · **2 failed**: the one above + `traceability` on `sufficiency-report.md` |

    The `traceability` fail on `sufficiency-report.md` is the script grepping an **input**:
    the sufficiency report is what the package is graded *against*, never a graded artifact,
    and a report that folds it as a finding has mis-scoped the pre-assert (`gap-list-scope`);
    a report that names it as the input it is, or passes the design deltas only, reads
    correctly. Every other fail is seeded. (g2's `store-delta.md` carries an FR reference so
    it passes `traceability` under the glob; the script matches the bare bracketed marker, so
    the fixtures write `[NEEDS CLARIFICATION] question`, never `[NEEDS CLARIFICATION:
    question]`.) (The script's `## Entities` requirement is a plugin-side
    inconsistency outside this kit's scope, noted for the maintainer.)
14. **Assertion brittleness is settled by the probe, not here.** The regexes per golden were
    each run in Python against a scratch compliant report (frontmatter shape and the
    ISSUE-TEMPLATES working shape) and tripped by a scratch non-compliant one before the kit
    was returned; a regex a compliant review can fail is a kit defect to fix before the grid,
    never a reason to relax the rule set. Every golden also carries `fixture_unchanged`
    (b809b73): every handed-in file byte-identical after the run, the deterministic half of
    `author-grader` on all three goldens beside g2's specific `CreditNote` and `fixed/`
    checks.
15. **The real run splits this review; the kit folds it.** In `/mochiko:implement` the
    advocate seat grades the design deltas before the design checkpoint
    (`impl.design-review-pair`) and the verification seat grades `tasks.md` before the card
    confirm (`impl.card-review-before-confirm`, "quality per `mochiko:review-plan-artifacts`").
    The prompts hand one seat the whole package, `tasks.md` included, so one session exercises
    the cycle-card check set; the seam between the two real dispatches is not measured.
16. **`design-disclosure.md` is a kit substitution.** The rung claims the rules grade live, in
    the real run, in each producing seat's plan and report; the fixtures collect them into one
    `report: disclosure` file so the seat has a surface to grade. Its `adopt_first:` block
    carries `shelf_named: no` on the hand-rolled decisions, which hands every arm — the
    `noskill` control included — the adopt-first question; `adopt-first-lens` is read knowing
    that, on the review turning the disclosed absence into a finding at conformance strength
    rather than on discovering it.
17. **Envelope path.** The real report lands under `.mochiko/features/FEAT-XXX/`
    (`impl.reports-envelope`); the prompts write `review.md` at the workspace root so the
    assertions and the judge read one known path. The substitution is identical on every arm.
18. **Model tiering is not exercised.** A real seat brief carries the routing rule
    (`impl.model-tiering`: locate/enumerate reads to an `Explore` subagent on Haiku); the
    prompts omit it, so delegated reads are neither invited nor read. The runner pre-allows
    only the Skill tool, `mochiko-cli`, and python; everything else rides `acceptEdits`' own
    rules, under which the Agent tool is not gated — a session that spawns a subagent on its own
    is not invalid, and its reads are simply not part of the read.

## Budget

≤ 27 grid sessions, plus the ship-bar (c) noise re-run of one extra replicate per arm if an
arm lands above its band (9 sessions, once) — **≤ 36 sessions** — and ≤ US$ 25 including
judge calls (`est_total_cost_usd` in `summary.json` plus the judge's client-side estimate).
The runner halts resumably at `--budget-usd 25` (85e5f89); exceeding either bound returns to
the user. The probe runs are outside the session count and inside the US$ 25.

## Fill log

- prune result (rules tagged model-native, sessions): **[measured at grid]**
- coverage read (`post`, `pre`; live rules held pass^k per arm): **[measured at grid]**
- lost vs `pre` (floors; non-floor list): **[measured at grid]**
- band (invited pairs; all-pairs in parentheses; UNDER-SAMPLED marks): **[measured at grid]**
- invalid runs excluded and counted: **[measured at grid]**
- budget (sessions; est. spend incl. judge): **[measured at grid]**
- ship bar (a) floors / (b) ≤ 2 non-floor lost / (c) band ≤ 15 % / (d) floor coverage: **[measured at grid]** / **[measured at grid]** / **[measured at grid]** / met at authoring (all 11)
- kit status: **[measured at grid]**
