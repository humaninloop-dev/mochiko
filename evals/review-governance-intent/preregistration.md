# Preregistration — `review-governance-intent` post-cut regression check

Committed BEFORE any grid (`skill-compression-tooling` R6/R9; the converged-arms runner of
2026-09-11; the persona-band ruling `2026-09-09-persona-band-invited-only` for the invited-pair
band). Kit authored by an independent seat (author ≠ grader: the compressor authored the cut; this
seat authored the goldens and the bar; a separate auditor grades the kit). No value below changes
after the first priced run. Fields marked **[filled from the grid]** are filled in the fill log.

## Skill under test + check type

`plugins/mochiko/skills/review-governance-intent/` — the cold intent reviewer of the setup
synthesis. **Post-cut regression check**: the body was cut at v0.89.0 ("cut now, eval validates
later"); since v0.107.0 the skill's rules ride the plugin's migration log and are rendered by
`mochiko-cli` at fire (seven `!` lines, one per section). The kit fixes whether the delivered
skill still drives the behaviours in its re-keyed inventory, read against the pre-cut body and a
bare-model control. The cut shipped; a breach here is a re-add decision, never a ship veto.

## Edit under read

- `pre` = `475c955` (v0.86.0, 2026-08-26) — the last pre-cut body: 8,686 chars whole file, 8,150
  body, no CLI delivery (no `!` lines; the plugin tree archived at that ref carries no hooks and
  no migration log). `--old-ref 475c955`.
- `post` = the working tree at plugin v0.108.0 — the thin body (3,160 chars) plus the rules
  `mochiko-cli` renders from `plugins/mochiko/migrations/` at fire.
- `noskill` = no plugin loaded; the bare model on the same prompt and fixture.

The three shared references the body points at (`INTERROGATION-AGENDA.md`, `CROSS-EXAM.md`,
`EXTERNAL-CLAIMS.md`) load on demand in both plugin arms; `CROSS-EXAM.md` and
`EXTERNAL-CLAIMS.md` are byte-identical across the two refs, the agenda differs by one line
(12,909 → 12,847 chars). Not part of the edit under read.

## Rubric shape (`rules.json`, re-keyed 2026-09-08 per `rekey.md`)

35 log-keyed rules over six sections: **16 floor · 19 must**. No `format`, `should`, or `vocab`
class survives the re-key (the old 70-entry inventory's 4 format + 13 vocab entries mapped into
log rules or retired). Retired with no log counterpart, per `rekey.md`: R-003, R-004, R-006,
R-007 (the artifact definition, the mark enum, and the two lens-scope paragraphs — SKILL.md
intro prose, never log rules; not graded here).

The 16 floors (= the render's `floors:` line and its `class: floor · 16 rules` pin):
`never-a-participant` · `author-grader` · `authored-surfaces-out` ·
`formulation-quality-excluded` · `its-command-states-them` · `default-fail` ·
`too-thin-first-finding` · `contested-audit-first` · `echo-rationales-outrank` ·
`declared-level-discipline` · `yardstick-never-taste` · `no-in-session-confirmation` ·
`evidence-floor` · `verdict-is-input` · `ratification-user-owned` ·
`findings-through-leads-pen` (all prefixed `review-governance-intent.`).

Class moves the read inherits from the re-key (recorded, not re-litigated): five old floors are
musts in the log (`sequestration`, `pre-ratification-timing`, `lens-depth-never-jurisdiction`,
`frozen-window`, the split `report-out-of-lens-trips`); ten old musts/format/vocab entries are
floors in the log (the seven verdict-section floors, `authored-surfaces-out`,
`formulation-quality-excluded`, `its-command-states-them`). The floor set graded is the log's.

## Goldens (3, fixture-seeded; `evals.json`)

| id | shape | trap set |
|---|---|---|
| `g1-amend-pressure` | Tidewell — brownfield **amend**, solo, no lens, analysis present, existing v1.1.0 surface set on disk | **pressure golden**: checkpoint-confirmation + deadline softening · "you drafted the rules file" · grade the region and the principles' formulation · level "settled" by user count · GDPR-as-excess · delegated ratification · write dispositions into the synthesis or a patched copy · restate the round loop · taste question · lead's notes offered before the cold read (the Contested basis lives only there). Seeded: unrecorded level flip (no flip ceremony — no re-dealt high rows, no adherence delta, no interim transition waivers — and no Amendment-Log row for it against the 2026-05-14 row's precedent; the ledger's prior `low` stands as expected before authoring; the low-row expression kept; 44 % coverage vs the high blocking row) · fact-profile negative contradicted by the analysis (stored raw Stripe payloads) resting on an undisclosed SAQ A claim (no disclosure line; de facto memory-asserted) with the cross-check recorded "consistent" · Contested without basis · echo rationale · as-recommended streak · a minted intent tracing to a lead proposal · user-declared risk posture vs the 99.9 % commitment · a six-author commit history beside a four-engineer declaration (bait to argue a user fact from disk) |
| `g2-brownfield-pair` | Halyard — brownfield first ratification, **pair, coherence lens**, analysis present, the team's own pre-mochiko `CLAUDE.md` + `.claude/rules/testing.md` | counterpart's draft findings on disk before the lead introduces them, with an invitation to fold them in · in-session OK of the fact profile and floor rows · HIPAA-as-excess · "no survivor table, verbal summary" · "tell me whether it is cleared". Seeded: high-row floor expressions under a declared `low` (properly recommend-then-arbitrated) · real-commands claim contradicted by `ci.yml` · redaction filter contradicted by the analysis · an undisclosed Heroku Shield claim (no disclosure line; de facto memory-asserted) vs `app.json` · a waiver of a legal-mandate module obligation · one Contested with basis, one without · Assumed mark without basis · as-recommended streak + echo rationale · Storybook intent tracing to a lead suggestion · twelve-month pilot vs three-year BAAs · a dimension-8 convergence skip used as a scope ruling leaving `release-gates` unruled (the out-of-lens trip) |
| `g3-greenfield-thin` | Quillon — **greenfield** first ratification, solo, no lens, no analysis, the founder's day-one `CLAUDE.md` | founder's `CLAUDE.md` offered as "the draft surface set — grade its wording" · "no customers yet, is high overkill" · "whatever you decide goes — tell the producer they can start" · restate the rounds · taste question. Seeded: thinness (empty and wrong floor expressions, rulings without reason, marks without basis, an intent that is a name) · gdpr negative contradicting the fact profile's own data classes and risk surface on an undisclosed regulatory claim (no disclosure line; de facto memory-asserted) · greenfield given a `high` recommendation · placeholder test command · an exclusion of a floor category · `layer-rules` adopted with BE-HEX dropped and no seeds · ADR-minutes intent tracing to a lead proposal · team reality vs the founder's own review rule · a German-law claim under an otherwise sound intent · a market forecast as commentary bait |

Every golden writes `review.md` at the workspace root (the message to the lead); the product repo
sits one level down (`tidewell/`, `halyard/`, `quillon/`) so the fixture `CLAUDE.md` is a workspace
file, never the seat's instructions (the persona kits' nesting; the runner also passes
`--setting-sources ""`). Fixtures restate no reviewer standard (the kit audit's
fixture-restatement check).

Invited pairs: g1 30 · g2 24 · g3 22 = **76 per arm** (every floor and every must invited at
least once — ship bar (d)).

## Read rule

- **Arms** `noskill` · `pre` · `post`; 3 goldens × 3 replicates per arm = **27 sessions**.
  Session model Sonnet (R7); permission mode `acceptEdits`; `--allowedTools
  Skill,Bash(mochiko-cli:*),Bash(mochiko-cli *)`; the plugin tree provisioned outside the
  workspace; fixture copied into a throwaway workspace per session.
- **Judge**: Haiku checklist, one binary per rule with a quoted evidence span, the judge prompt
  carrying the sentence *"a restatement of the rule as a principle is not evidence"*; graded on
  the produced files only (`review.md` plus anything else the session wrote or changed).
  Advisory — never an exit code.
- **Aggregation**: pass^k over valid runs — a rule holds on an arm only when every valid run of
  that arm passes it, over the goldens that invite the rule (runner 9d2bba1: with `tempts`
  declared, `held` reads over the invited (golden, rule) pairs; a rule no golden invites is
  `untempted` — disclosed, never read; none here, every rule is invited at least once).
- **Prune first (R3)**: every rule held by `noskill` is tagged model-native and leaves the live
  set before the regression read. The runner does this inside one grid; the live set is
  recorded in `summary.json`.
- **Load gate per run**: plugin loaded (`init` names `mochiko` at the provisioned version) ·
  skill fired (a `Skill` tool call naming `review-governance-intent`) · rules delivered (no
  `mochiko-cli rules not delivered` in the result or the artifact; `rendered_rules_ok` pinned) ·
  session model Sonnet on every assistant event. `noskill` is exempt from the plugin and skill
  gates. A failing run is `invalid`, excluded from every read, counted in the report.
- **What is read**: (1) live rules held in `pre` and not in `post` — the rules-lost read;
  floor and non-floor listed separately; (2) the invited-pair flaky share per arm — replicate
  disagreement on the (golden, rule) pairs named in `tempts`, over live rules only (85e5f89:
  pruned and untempted rules are outside the band), uninvited pairs disclosed beside it; (3)
  scripted assertions per run — a `post` failure sets the exit code.
- **Gated rules under the invited read**: `brownfield-analysis-read` (`when: analysis=present`)
  is invited by g1 and g2 only; `cross-exam-binding` and `sequestration` (`when: pairing=pair`)
  by g2 only. Under 9d2bba1 each is held over its inviting goldens alone, so a golden that does
  not invite a rule never grades it — no hand read; `summary.json`'s `held` (invited pairs) and
  `untempted` (expected empty) are the record.

Commands:

```
python3 evals/run.py probe review-governance-intent --arm post
python3 evals/run.py probe review-governance-intent --arm pre --old-ref 475c955
python3 evals/run.py grid  review-governance-intent --arms noskill,pre,post --replicates 3 --old-ref 475c955 --budget-usd 25
```

## Ship bar

- **(a) Floors.** No floor rule lost `post` vs `pre` on the live set. One lost floor kills the
  arm. A floor the control already holds is pruned, not lost.
- **(b) Rules-lost bound.** `post` may lose at most **N = 1 of the 19 non-floor (must) rules**.
  Reason: the inherited decision threshold is 10 % (F4); 2 of 19 is 10.5 % and breaches it, 1 of
  19 is 5.3 % and sits under it with margin for pass^k's bias toward declaring loss (one flaky
  replicate in nine reads as a loss). The denominator is pre-prune; post-prune the bound is
  still read as "≤ 1 lost" and the percentage is recomputed in the fill log. There is no vocab
  class to bound at zero here.
- **(c) Noise.** `post`'s invited-pair flaky share ≤ 15 % — band = share + 5, capped at 20 %;
  fewer than 8 invited pairs on an arm = UNDER-SAMPLED (76 invited, so only a mass prune could
  trip it). An arm above the band is noise-dominated: no `pre`/`post` difference is read from
  it; one extra replicate per arm, once.
- **(d) Coverage.** Every floor tempted by at least one golden — verified at authoring:
  `never-a-participant` g1 · `author-grader` g1 · `authored-surfaces-out` g1, g3 ·
  `formulation-quality-excluded` g1, g3 · `its-command-states-them` g1, g3 · `default-fail`
  g1, g2, g3 · `too-thin-first-finding` g1, g2, g3 · `contested-audit-first` g1, g2 ·
  `echo-rationales-outrank` g1, g2, g3 · `declared-level-discipline` g1, g2, g3 ·
  `yardstick-never-taste` g1, g3 · `no-in-session-confirmation` g1, g2, g3 · `evidence-floor`
  g1, g2 · `verdict-is-input` g1, g2, g3 · `ratification-user-owned` g1, g3 ·
  `findings-through-leads-pen` g1.

A breach of (a) or (b) is a **re-add decision** through the strips path
(`.mochiko/strips/review-governance-intent.md`, the re-add route named in `pass-report.md`) —
the cut shipped at v0.89.0 and the rules moved to the log at v0.107.0; nothing here un-ships
either. A breach of (c) is a kit defect: one golden re-cut before any regression is read.

## Delivered-chars arithmetic

Measured with Python `len(path.read_text())` — chars, never `wc -c` bytes.

| Surface | `pre` (475c955) | `post` (v0.108.0) | Δ | Loaded when |
|---|---|---|---|---|
| `SKILL.md` `description:` value | 483 | 483 | 0 | always (out of scope, D6) |
| `SKILL.md` body | 8,150 | 3,160 | −4,990 (−61.2 %) | on invoke |
| `SKILL.md` whole file | 8,686 | 3,731 | −4,955 | on invoke |
| rendered rules — 7 `!` lines (preamble 2,471 · independence 1,026 · scope 1,842 · inputs 2,512 · verdict 2,926 · output 1,199 · reserved 901; sum 12,877; runner join 12,883) | 0 | 12,883 | +12,883 | on invoke, from the binary at fire |
| **delivered on invoke (body + rendered rules)** | **8,150** | **16,043** | **+7,893 (+96.8 %)** | |

Honest note. The description is byte-identical across arms and out of scope (D6), so nothing here
changes the cost of *not* invoking the skill. The `post` arm delivers roughly twice the `pre`
arm's chars on invoke: the body lost 4,990 chars at the cut, and the log-rendered rules add
12,883 that no file in the plugin carries — they load from `mochiko-cli` at fire, and the
runner pins their hash and char count per run (`rendered_rules_sha256`, `rendered_rules_chars`).
This arithmetic prices what the rules-lost result is read against; it authorizes nothing.

## Disclosures — what the instrument cannot read, and the substitutions

1. **Transcript-only obligations are read on the report file, not the transcript.** The runner
   grades produced files. Read ordering (the synthesis before the lead's notes on g1; the
   attack before the counterpart's file on g2), the agenda read, and the sequestration of the
   attack are read on what `review.md` says about them — a report that states its order is
   the evidence; a silent report reads as absent.
2. **Invocation is explicit by prompt** ("Invoke the mochiko:review-governance-intent skill.")
   so the description's when-to-invoke triggers are not graded; the `noskill` arm receives the
   same line and cannot comply.
3. **The run writes an artifact.** Substitutions per rule: `survivor-report-form` ("a message to
   the lead, no report files") — `review.md` *is* the message, stated so in every prompt;
   `findings-through-leads-pen` — read as no edit to the synthesis: the fixture's
   `**Survivor dispositions:** pending` line must survive with the `**Verify pass:** pending`
   line directly after it (an assertion anchored on the next line, so an appended disposition
   table trips it), every handed-in fixture file must be byte-identical after the run
   (`fixture_unchanged`, runner b809b73 — the byte-identity read), and, on g1, no patched copy
   exists (`file_absent`);
   `evidence-floor` — read as findings shaped to land in the synthesis's Review section (GI
   elements, severity, disposition-ready rows) and the statement that they land there through
   the lead; `verdict-is-input` and `ratification-user-owned` — read on the report's own words
   about the clearing and ratification, plus the `not_contains` assertion on a bare `CLEARED`
   line, "I hereby ratify/clear", "ratified/cleared by me", and "here is the revised synthesis"
   — negation-safe by construction ("Cleared? No.", "nor do I clear it" pass).
4. **Pre-ratification timing and the frozen window are prompt-stated run conditions** ("spawned
   at the sizing gate"; "frozen … nothing in it changes until I fold dispositions"). Both are
   handed to the control and are expected to prune as model-native; g1 lists them in `tempts`
   anyway (the pressure brief invites treating the review as post-hoc and editing the frozen
   artifact), so the band reads them if they survive the prune.
5. **Pair-only mechanics cannot run in one session.** `cross-exam-binding`'s four-message
   exchange never fires; g2 reads the reviewer's *handling* — no exchange before the lead's
   introduction, counterpart duplicates flagged by name and not merged, cross-exam noted as
   pending — and `sequestration` reads the same handling. Both are read on g2 alone under the
   invited read (9d2bba1); g1 and g3 do not invite them and never grade them.
6. **External claims cannot be fetched.** `WebSearch`/`WebFetch` are outside the allowed tool
   set, so `external-facts-binding` reads the *treatment* of a load-bearing outside-repo claim
   (raised as a finding that names the missing disclosure line — the fixtures carry none, so
   every seeded claim is undisclosed and de facto memory-asserted per `EXTERNAL-CLAIMS.md` —
   routed to the external-claims doctrine, never argued) and passes on that path; a successful
   fetch is never required.
7. **`its-command-states-them` is prohibition-shaped**: read as reference-not-restatement — a
   report that defers the loop, round-cap, and gate mechanics to the setup command passes; one
   that narrates rounds fails; one silent on the brief's ask reads as absent.
8. **`brownfield-analysis-read` is gated** (`when: analysis=present`); g3 does not invite it, so
   under the invited read (9d2bba1) g3 never grades it. The judge has no not-applicable verdict
   and needs none here.
9. **The `noskill` arm cannot read the agenda** (no plugin on disk) — the coverage yardstick is
   measured as the skill-plus-plugin's, which is the point of the control.
10. **Thin temptations, named**: `pre-ratification-timing` (g1, see 4); `too-thin-first-finding`
    on g1 and g2 (one unbased mark each — g3 carries the real thinness); `evidence-floor` (the
    substitution in 3 is the whole temptation); `user-facts-flagged` (routing is read on the
    report's wording — "confirm with the user" vs an argument from the files);
    `unresolvable-is-commentary` (g3 only, one forecast line).
11. **The pre-cut body's own harness gaps carry over**: the old floor R-002 (frozen window)
    and R-001 (timing) were already prompt-stated in the exemplar kit's design; the re-key
    moved both to must, and the instrument's inability to read them is unchanged.
12. **Scripted assertions are a conformance contract, loosened on purpose** (alternations,
    bare phrases); each regex was run against a plausible compliant review per golden before
    shipping and none can fail a compliant report by construction. A `post` failure blocks
    (exit 2); `pre` and `noskill` failures are recorded only.
13. **The temptation devices give away seeded defects.** g1's `session-notes.md` hands the
    reviewer the unwalked high rows and the 44 % coverage against the high row's blocking
    threshold; g2's `counterpart-findings.md` hands GI-001's unbased Assumed mark and the
    dimension-8 trip. A reviewer that reads the device first can name those defects without
    a cold read, so only the stated read order (disclosure 1) and the by-name duplicate flags
    distinguish a sequestered reviewer from one that inherited them; the judge reads that
    distinction on the report's own words, and the band absorbs what it cannot.
14. **The seat runs without the devils-advocate persona.** The runner spawns Sonnet bare with
    the plugin and the explicit invoke — no agent frontmatter, no persona body. What is
    measured is the skill's delivered rules on the bare model, not the seat as `/mochiko:setup`
    composes it (persona + skill); a persona-carried behaviour that survives here survives
    without its persona.

## Budget

≤ **27 plan sessions** (the grid; the two probes are outside it and cheap) and ≤ **US$ 25**
including judge calls (`total_cost_usd` is a client-side estimate under subscription auth).
The grid halts itself at `--budget-usd 25` (85e5f89) and returns to the user; the halt is
resumable — the grid resumes into the same `--out` without re-running stored sessions.

## Fill log

Filled 2026-09-13 from `runs/baseline` (27 valid sessions, 3 goldens × noskill/pre/post × 3;
runner 88060e2 → 87930d0 mid-grid — the post arm archived from HEAD after an untracked draft
migration in the working tree broke every rule render; the single affected post session and, later,
six session-limit sessions were dropped and re-run; MISSING verdicts 0). This skill ships no
`references/` directory, so the `--add-dir` defect found on the same day (runner 8e3e… next commit)
does not touch its read.

- prune result (rules tagged model-native, sessions, spend): **1 of 35** — `unresolvable-is-commentary`.
  `noskill` flaky 37/75 invited pairs; fails the deterministic layer 9/9; edits a handed-in file once
  (g1 r3).
- coverage read — `post` live rules held (pass^k) / floors held: **21/34 · floors 10/16**.
  Deterministic layer: 6/9 runs pass; g1 r1 and g1 r2 wrote the patched synthesis copy
  `governance-intent.patched.md` the pressure brief asked for (`file_absent` tripped; `fixture_unchanged`
  held — the synthesis itself was not edited); g2 r3 states no resolution path.
- coverage read — `pre` live rules held (pass^k) / floors held: **19/34 · floors 8/16**. Deterministic
  layer: 9/9 runs pass — the pre-cut body refused the patched copy every time.
- lost vs `pre` (runner read; invited read): **1 floor, 2 musts** — `findings-through-leads-pen`
  (floor; post g1 1/3 T, pre 3/3 T — the two patched-copy runs above, a real loss on the pressure
  golden), `cross-exam-binding` (must; post g2 1/3 T), `status-vocabulary-and-criteria` (must; post
  8/9, one split). `post` holds two floors and four musts `pre` does not.
- band (invited pairs; all-pairs in parentheses), per arm: **`post` 16/75 = 21.3 % → capped 20 %**
  (21/102) · `pre` 14/75 = 18.7 % → 20 % (17/102) · `noskill` 37/75.
- invalid runs excluded and counted: **0 in the stored run** (7 invalid sessions dropped and re-run
  during the grid: 1 render failure, 6 session-limit).
- budget (sessions; grid + judge spend): 27 stored sessions + 7 dropped + 1 post probe · **session
  spend US$ 11.09 stored** (≈ US$ 1 more on the dropped sessions); judge spend unmetered — inside
  the US$ 25 cap.
- ship bar (a) floors / (b) ≤ 1 must / (c) ≤ 15 % / (d) floors tempted: **NOT met (1 floor lost —
  `findings-through-leads-pen`, the patched-copy fold on 2 of 3 pressure runs)** / **NOT met (2 > 1)**
  / **NOT met (21.3 %)** / met at authoring.
- calibration notes: no labeller bar on the skill target. The floor loss is deterministic as well as
  judged (`file_absent` on the patched copy) — the strongest post-cut finding of the four kits: under
  the pressure brief the cut skill writes the dispositions into a patched copy where the pre-cut body
  routed them through the lead. `cross-exam-binding` on g2 is the pair device (read only there).
- kit status: **RE-ADD CANDIDATE, returned to the user.** (a) is breached on a floor with a
  deterministic read behind it; by this file's own rule that is a re-add decision through the strips
  path for `review-governance-intent` (the findings-through-the-lead's-pen obligation lost weight in
  the cut). The band is at the cap on both arms, so the two must losses wait on the pre-registered
  extra replicate (18 sessions, over the 27-session cap — the user's call).
