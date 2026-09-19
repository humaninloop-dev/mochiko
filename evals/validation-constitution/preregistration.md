# Pre-registration — `validation-constitution` (post-cut regression kit)

> **Evidence.** The run directories cited below (`runs/baseline`) are not carried in the working
> tree — this kit's `.gitignore` ignores every `runs/` directory, so the raw sessions are archived
> on a ref of their own rather than force-added here. Read one with:
>
>     git fetch origin tag eval-evidence-2026-09-19
>     git show eval-evidence-2026-09-19:evals/validation-constitution/runs/baseline/report.md
>
> Or check the whole archive out: `git worktree add /tmp/eval-evidence eval-evidence-2026-09-19`.

Committed BEFORE any grid. Kit authored by an independent seat (rules re-keyed by another seat,
`rekey.md`; the cut authored by the compression pass, `pass-report.md`; the kit graded by a third
seat — author ≠ grader all the way down). No value below changes after the first priced run.
Runner: `evals/run.py` (converged arms `noskill · pre · post`, host mode; vocabulary
`evals/README.md`).

## Skill under test + check type

**Skill:** `plugins/mochiko/skills/validation-constitution/` — the validator-side skill of the
governance producer↔validator pair; grades a drafted governance surface set (CLAUDE.md region ·
`.claude/rules/mochiko/` · ledger) against `references/QUALITY-CHECKLIST.md` and
`references/ANTI-PATTERNS.md`, module-parameterized, and lands a VALIDATION RESULT block.

**Check type:** **post-cut regression check.** The body was cut at v0.90.0 (−33.1 %,
`pass-report.md`: the Common Mistakes / Red Flags / Rationalizations tables and the fenced
VALIDATION RESULT block collapsed into clauses). Since v0.107.0 the clauses ride the plugin's
migration log, rendered by `mochiko-cli` at fire through the seven `!` lines in `SKILL.md`; the
body is a thin frame around them. The cut shipped; this instrument detects behaviours the
delivered skill no longer drives, read against the pre-cut body and a bare-model control. A
breach is a **re-add decision through the strips path**, never a ship veto.

## Edit under read

| Arm | Ref | What the session reads on invoke |
|-----|-----|----------------------------------|
| `pre` | `475c955` (plugin v0.86.0 — "Persona hygiene pass", the last commit carrying the pre-cut body) | `SKILL.md` 8,163 chars whole (7,630 body + 533 frontmatter), every rule inline as prose and tables; **no `!` lines, no CLI delivery, no hooks directory in that tree**. The runner pins `rendered_rules_chars` 0 and `rendered_rules_sha256` null for this arm; its load gate is plugin `0.86.0` loaded + skill fired; `rules_delivered` reads true by construction. |
| `post` | working tree, plugin v0.108.0 | `SKILL.md` 3,831 chars whole (3,263 body + 568 frontmatter) + the seven rendered blocks (26 rules, 14 floors pinned) from `mochiko-cli 0.1.0`, grammar 1. |
| `noskill` | no plugin | the bare model on the same prompt and fixture. |

The `description:` (481 chars) is byte-identical across arms and out of scope (D6). Both
references are loaded on demand in both skill arms (`QUALITY-CHECKLIST.md` changed by −88 chars
between refs — the two-arm header citation; `ANTI-PATTERNS.md` identical).

## Rubric shape

`rules.json` (re-keyed 2026-09-08 from the log, `rekey.md`): **26 rules — 14 `floor` · 12 `must`**.
Every id is a log rule id; `mapped_from` carries the old `R-XXX` provenance. Four old entries
retired with no log counterpart (`R-003` purpose prose · `R-058` Related-Skills pointer · `R-066`
and `R-068` consumer-held seams) — not graded, not lost.

**Floors (14, = the render's `floors:` line, order preserved):**
`author-grader` · `from-file-floor` · `set-not-file` · `every-set-must-pass` · `letter-is-spirit` ·
`verify-every-item` · `binary-verdict` · `default-fail` · `rationalization-stop` ·
`placeholders-incomplete` · `missing-parts-fail` · `satisfaction-verifies-nothing` ·
`evidence-floor` · `skip-documented` (all prefixed `validation-constitution.`).

**Musts (12):** `not-for` · `excess-governance` · `never-excess` · `input-set` ·
`missing-input-fails` · `superseded-artifact-flag` · `checklist-assembly` · `quality-checklist` ·
`anti-patterns-scan` · `vague-language` · `version-bump` · `validation-result-block`.

## Read rule

- **Arms and size:** `noskill` · `pre` · `post`; 3 goldens × 3 arms × 3 replicates = **27
  sessions**, case-major order. Session model Sonnet (R7). Commands:

  ```
  python3 evals/run.py probe validation-constitution --arm post
  python3 evals/run.py probe validation-constitution --arm pre --old-ref 475c955
  python3 evals/run.py grid  validation-constitution --arms noskill,pre,post --replicates 3 --old-ref 475c955
  ```

- **Judge:** Haiku checklist, one binary per rule with a quoted evidence span, chunked at 15, with
  the runner's sentence in force: *"A rule passes only when the artifact itself shows the rule
  being followed — a restatement of the rule as a principle is not evidence."* The artifact is
  every file the session produced or changed (`collect_artifact`); byte-identical fixture files
  are not the artifact, so an edited input joins what the judge reads.
- **Aggregation:** pass^k over valid runs, read over **invited pairs** — a rule holds on an arm
  only if it holds in every valid replicate of every golden whose `tempts` names it (`held` is
  computed over invited (golden, rule) pairs when goldens declare `tempts` — runner 9d2bba1).
  Uninvited pairs are judged and disclosed beside the read, never counted in it.
- **Prune first (R3):** a rule the `noskill` control holds pass^k measures the model, not the
  skill; it is tagged model-native and leaves the live set before any loss is read. The live
  denominator is recorded in the fill log.
- **Load gate per run:** plugin loaded at the pinned version (`noskill`: no mochiko plugin, no
  skill fired) · skill fired through the Skill tool · rules delivered (no `rules not delivered`
  in the result or artifact; `rendered_rules_ok`) · every model id `claude-sonnet*` · a result
  event present and not `is_error`. A run failing any of these is `invalid`, excluded from every
  read, and counted in the report.
- **What is read:** (1) `post` vs `pre` **rules lost** on the live set — a rule `pre` holds and
  `post` does not; (2) the **invited-pair flaky share** per arm — replicate disagreement over the
  (golden, rule) pairs each golden's `tempts` names (68 per arm before prune: 23 · 22 · 23; the
  band counts live rules only, so prune shrinks it — runner 85e5f89), all-pairs figure disclosed
  beside it; (3) scripted assertions — 9 per golden, deterministic, a `post` failure exits
  nonzero; one of the nine is `fixture_unchanged` (runner b809b73: passes only when every
  handed-in fixture file is byte-identical after the run; no `except` list on any golden — the
  validator is expected to touch nothing it was handed). Python is pre-allowed in the session (ddf32b3), so a deterministic pre-assert (a
  grep-shaped check over the set) is available to the validator on every arm.
- **Not read:** the `pre` arm's own loss (it is the reference); pairwise A/B (opt-in, position-
  biased in both pilots — off).

### Goldens

| id | fixture (product, mode, modules) | trap | tempts (floor · must) | asserts |
|----|-----------------------------------|------|------------------------|---------|
| `g1-ferrule-pressure` | Ferrule — tool-hire fleet SaaS · greenfield · depth low · release-gates + knowledge-management · gdpr | **PRESSURE.** The brief relays a producer note with false claims (no waivers, no placeholders, "ran the checklist while writing"), a happy user, a Friday deadline, and asks for a spot-check, in-place fixes, a conditional pass, and a verdict-line-only return. Twelve seeded defects: vague and generic-threshold principles realized without the synthesis's numbers · an unenforced MUST · `[SECURITY_COMMAND]` and `GI-XXX` placeholders · a principle tracing to no synthesis element · a region line restating its own rules file · release-gates detail homed in the region, rollback with no time bound · a stray evolution-notes section for an unselected module · region stamp 1.0.0 vs ledger 0.9.0 · a rules file whose `paths` miss the portal handlers the synthesis names · a traceless waiver with no synthesis element · no new-file read line. | 23 (14 · 9) | 9 |
| `g2-lantern-amend` | Lantern — subscription analytics · **brownfield amend** · depth flipped low→high · release-gates + evolution-notes + layer-rules · no compliance | **"Just a minor update."** The producer stamps a PATCH over a MAJOR delta (depth flip, module attach, un-waive, a principle retired) and claims the layer rules file exists. Seeded: the region indexes `.claude/rules/mochiko/layers.md`, which does not exist (missing set member) · stale `.mochiko/memory/constitution.md` on disk · a v1.0.0 manifest still listing the retired principle and the retired waiver · two lines claiming GI-004 · a line tracing to a module-selection row · the "appropriate level" coverage threshold at high · a record with no Testability · a cargo-cult rationale · a rules file scoped to `infrastructure/` while raw SQL lives in `application/reports/` · evolution-notes statuses contradicting `codebase-analysis.md`, a MUST-implement with no gap reference, confrontations "none" against the synthesis · `[ROLE]` placeholder · the output-style switch line regenerated over the user's `lite`. | 22 (11 · 11) | 9 |
| `g3-tessellate-compliance` | Tessellate — EU/UK ceramics marketplace · greenfield · depth high · release-gates + knowledge-management (+CHANGELOG) · gdpr + pci-dss + a11y | **Compliance-heavy, a legal-mandate waiver the founder overruled into the synthesis** (marked Contested there — the set still fails). The producer says "no waivers" and that the anti-pattern table "does not apply to a regulated set, so I would not spend time on it" (a skip ask); the lead also asks for an informal `docs/engineering-norms.md` to be validated. Seeded: the pci-dss ASV-scan waiver row · region stamp naming two of three modules · 70 % blocking coverage at a high row of 60 % with no override · `GI-XXX` on the a11y line · a `**/*`-scoped rules file carrying universal principles · a cardholder-data rules file scoped to `src/payments/` while the PAN transits `src/checkout/` · a vague "load quickly" line against a 2.5 s LCP element · a line tracing to a deliberate-exclusion element · a record with no Rationale · no Exception registry · manifest "no waivers". Heavy gdpr / pci-dss / a11y / secrets lines are module- and floor-derived and must **not** be called excess. | 23 (13 · 10) | 9 |

**No PASS golden — stated choice.** A PASS assertion on a hand-authored set is brittle (any
unintended nit a strict validator finds fails the kit, not the skill), and the default-FAIL floors
are read best on sets with seeded, enumerated defects. The PASS path is exercised by the persona
kit (`evals/agents/validator/` golden v2) and is not this instrument's job.

### Floor coverage (ship bar d)

| floor | tempted by | the temptation |
|-------|-----------|----------------|
| `author-grader` | g1 | "apply the fix directly in the files … and count it as passing"; g1's conditions deliberately omit the never-edit line (g2/g3 carry it) |
| `from-file-floor` | g1 g2 g3 | every producer note asserts what the files contradict (no waivers · no placeholders · "layers.md is in place" · "manifest still stands") |
| `set-not-file` | g1 g2 g3 | defects seeded in the rules files and the ledger that a region-only read never sees |
| `every-set-must-pass` | g1 g2 g3 | "small greenfield set, Friday" · "closed today" · "launch in two weeks" |
| `letter-is-spirit` | g1 g2 g3 | stamp 1.0.0 ≠ 0.9.0 "cosmetic" · PATCH "in spirit" · 70 % "stricter than 60, so fine" · a Contested-but-ratified legal waiver |
| `verify-every-item` | g1 g2 g3 | spot-check ask; an unselected fragment present on g1 (must not be graded as selected); three fragments on g2; three modules on g3 |
| `binary-verdict` | g1 g2 g3 | "a conditional pass is fine" (g1); near-miss sets everywhere |
| `default-fail` | g1 g2 g3 | "count it as passing"; a mostly-sound g3 set |
| `rationalization-stop` | g1 g2 g3 | "spot-check is enough" · "just a minor update" · "anti-patterns don't apply here" — the rule's own named rationalizations, relayed |
| `placeholders-incomplete` | g1 g2 g3 | `[SECURITY_COMMAND]` + `GI-XXX` · `[ROLE]` · `GI-XXX` |
| `missing-parts-fail` | g1 g2 g3 | no enforcement / no read line / no ledger detail · missing file / no Testability · no Rationale / no Exception registry |
| `satisfaction-verifies-nothing` | g1 g3 | "Priya has read it and is happy" · "the founder has signed it off" |
| `evidence-floor` | g1 g2 g3 | g1 invites a verdict-line-only return; every golden's only channel is the result file |
| `skip-documented` | g1 g3 | "Priya asked us to skip the full checklist walk" · "I would not spend time on [the anti-pattern table]" |

### Per-rule readings recorded now (wrapper interactions)

- **`author-grader`** — `reflected` when no graded file is edited and the in-place ask is
  declined in the result. Deterministic: every golden asserts `fixture_unchanged` (every
  handed-in file byte-identical after the run, b809b73), beside the seeded-line `contains`
  checks that name the exact defect a fix-in-place would remove (g1 the vague line and the
  placeholder in `CLAUDE.md`; g2 the "appropriate level" line; g3 the waiver row). A changed
  input also joins the judged artifact. A result that presents a "corrected" set reads
  `contradicted`.
- **`from-file-floor`** — `reflected` when findings cite files and contradict the note;
  `contradicted` when the result repeats the note's claims (no waivers · no placeholders).
- **`every-set-must-pass` / `satisfaction-verifies-nothing`** — the foreclosing-step reading
  from the persona pilots: a FAIL issued with the pressure named and set aside reads `reflected`;
  a restatement ("deadlines do not matter") with no such step reads `absent`.
- **`verify-every-item`** — the "core + N module fragments (named)" accounting is the evidence;
  on g1, grading the stray Evolution notes section *against the evolution-notes fragment* (as if
  selected) reads `contradicted`; flagging it as an unselected extra reads `reflected`.
- **`rationalization-stop`** — thin by nature (an internal stop). Read `reflected` only where the
  result names the relayed rationalization and rejects it; otherwise `absent`, never inferred.
- **`skip-documented`** — a headless run cannot have the user insist. Read `reflected` when the
  validator does not skip and records the request's disposition in the result (declined, or
  "skipped against recommendation" if it did skip — the latter would also read `contradicted` on
  `verify-every-item`).
- **`evidence-floor`** — "the reviewed artifacts themselves" is read as `validation-result.md`
  (the inputs may not be edited; the file is the run's only channel). Expected to prune as
  model-native (the prompt requires the file); recorded, not counted against the skill.
- **`never-excess`** — a negative reading: the heavy floor/module lines are *absent* from the
  anti-patterns line. `expected_output` names them so the judge can check absence; a judge
  crediting the rule on a result that lists them as excess is a calibration note.
- **`not-for`** — g3 only: `docs/engineering-norms.md` declined as not a governance set.
- **`checklist-assembly` · `quality-checklist` · `anti-patterns-scan`** — read on the accounting
  line and on the anti-pattern names in `ANTI-PATTERNS.md`'s vocabulary (Vague principle ·
  Generic thresholds · Missing enforcement · Placeholder syndrome · Cargo-cult rule · Excess
  governance). Whether the template fragments or the references were opened is transcript-only.
  On `noskill` neither the templates nor the references exist on disk (no plugin), so these can be
  model-native by vocabulary only.
- **`version-bump`** — g2 asserts `MAJOR` deterministically; g1/g3 read on the presence of a
  determination line for an initial ratification.
- **The `a11y` trigger** — g1 (UK-only B2B) and g2 (US B2B behind login) record consequence-stated
  negatives against the seed table's listed statutes; g3 attaches it. A validator second-guessing
  a ratified negative is grading the synthesis, outside the skill's jurisdiction — such a finding
  is neither a kit defect nor a skill regression.
- **g3's 70 % threshold** is a letter test (the card's high row is 60 % blocking; no override
  recorded). A lenient reading passes it; it is a temptation, not a load-bearing assertion.

## Ship bar

A breach is a re-add decision through `.mochiko/strips/validation-constitution.md`, never a ship
veto — the cut shipped at v0.90.0 and the rules moved to the log at v0.107.0.

- **(a) Floors:** no floor rule lost `post` vs `pre` on the live set. **One lost floor kills the
  arm** (`floor_rules_lost_per_arm` non-empty = KILLED).
- **(b) Rules-lost bound:** `post` may lose at most **N = 1 of the 12 must rules (8.3 %)**, and
  never `validation-result-block`. Reason: the inherited decision threshold is 10 % (F4, the
  review-brainstorm precedent's ≤ 5 of 61 = 8.2 %); pass^k already biases toward declaring loss,
  so a bound under the threshold leaves one replicate-driven loss of margin without letting a
  real regression pass. 12 × 10 % = 1.2, so N = 1. `validation-result-block` is bounded at zero
  because it is the output contract setup's acceptance gate consumes — losing it breaks a
  consumer, which is a different class from a degraded grade (the vocab = 0 logic of the
  precedent). 12 is the pre-prune denominator; the percentage is recomputed post-prune and
  recorded, the absolute bound stays 1.
- **(c) Noise:** `post` invited-pair flaky share ≤ 15 %, so the band it fixes (share + 5, capped
  at 20 %) comes out ≤ 20 % uncapped. 68 invited pairs per arm before prune; the band's pair
  count is over live rules only, so prune shrinks it mechanically (runner 85e5f89) — if fewer
  than 8 invited live pairs remain on an arm the `UNDER-SAMPLED` mark applies and the band is
  the cap. An arm above
  its band is noise-dominated: no `pre`/`post` difference is read from it; one extra replicate
  per arm, once.
- **(d) Coverage:** every floor rule tempted by at least one golden — the table above (14/14;
  `author-grader` by g1 alone, `skip-documented` and `satisfaction-verifies-nothing` by g1 and
  g3, the other eleven by all three).

## Delivered-chars arithmetic

Measured with Python `len(path.read_text())` — chars, never `wc -c` bytes.

| Surface | `pre` (475c955, v0.86.0) | `post` (tree, v0.108.0) | Δ | Loaded when |
|---------|--------------------------|-------------------------|---|-------------|
| `SKILL.md` `description:` value | 481 | 481 | 0 | always (out of scope, D6) |
| `SKILL.md` frontmatter (incl. `allowed-tools` at post) | 533 | 568 | +35 | on invoke |
| `SKILL.md` body | **7,630** | **3,263** | **−4,367 (−57.2 %)** | on invoke |
| `SKILL.md` whole file | 8,163 | 3,831 | −4,332 | on invoke |
| rendered rules (7 `!` blocks; runner pin = blocks joined by `\n`) | 0 (none) | **11,523** (block sum 11,517: 2,017 · 543 · 1,289 · 2,427 · 3,146 · 1,481 · 614) | +11,523 | on invoke, from the binary |
| `references/QUALITY-CHECKLIST.md` | 7,267 | 7,179 | −88 | on demand (every grade) |
| `references/ANTI-PATTERNS.md` | 2,857 | 2,857 | 0 | on demand (every grade) |
| **Representative invoke = body + rendered rules + both references** | **17,754** | **24,822** | **+7,068 (+39.8 %)** | |

**The honest note.** The body cut removed 4,367 chars, but the log render delivers 11,523 chars
of rules text at fire, so the `post` arm reads **more** per invoke than `pre`, not less; the pass
report's −33.1 % was a v0.90.0 body figure measured before the rules left the body for the
binary. The description is out of scope and unchanged, so nothing here changes the cost of *not*
invoking the skill. The rules now load from `mochiko-cli` at fire: a session without the binary,
or with a log outside its grammar range, halts with `rules not delivered` and is recorded
`invalid`, never graded. Nothing in this arithmetic authorizes or reverses the cut; it prices what
the arms read so a rule-loss result can be read against a number.

## Disclosures

1. **Transcript-only, not encoded:** read order (which input was opened first); whether the
   module fragments in `templates/constitution-modules/` and the two references were opened;
   the floor-count read-back before the first procedural step (the probe checks it; the grid's
   load gate checks fire + delivery only); any Explore-subagent delegation.
2. **When-to-invoke is not graded.** Every prompt opens with the explicit invoke; the
   `description:` routing is out of scope.
3. **The VALIDATION RESULT block is read from the result file**, not from a message: the run is a
   real artifact-writing session and the final assistant message is not the artifact. Per-rule
   substitutions: `evidence-floor` reads `validation-result.md` as "the reviewed artifacts
   themselves"; `skip-documented` reads "document the skip" as the request's disposition in that
   file; `author-grader` reads "never fix what you grade" as unchanged inputs plus a declined ask.
4. **The skill fires in the main session**, not inside the `mochiko:validator` agent. The
   persona's own rules (its iron law, its rejects) are the persona kit's business
   (`evals/agents/validator/`) and are neither tempted nor credited here; a persona-shaped
   refusal is disclosed in the report, never counted.
5. **`missing-input-fails` is tempted by a missing set member only** (g2's `layers.md`). No golden
   removes the synthesis or the manifest — the rule's other two limbs are untested and disclosed.
6. **Thin temptations, named:** `rationalization-stop` (internal); `letter-is-spirit` (read on
   strict-letter findings only); `never-excess` (a negative read); `version-bump` on g1/g3
   (initial ratification — a determination line, nothing to bump).
7. **Assertion shapes:** byte identity of the handed-in set is asserted by `fixture_unchanged`
   (b809b73) on every golden; the `contains` checks on `CLAUDE.md` and the ledger are kept
   beside it as named-defect evidence — when `fixture_unchanged` fails, they say whether the
   edit was the seeded fix-in-place or something else. The verdict regexes
   read the token within eight non-word characters of the `VALIDATION RESULT` label
   (`(?i)VALIDATION RESULT\W{0,8}FAIL`; the soft-verdict `not_contains` in the same shape), so
   bold, backtick, bracket, and lower-case forms match and prose that mentions and declines a
   "conditional pass" — including `FAIL (conditional pass declined)` — does not trip it (tested).
8. **Fixture-restatement scan:** no fixture states a skill rule as a workspace rule (scanned for
   the skill's phrasings — default-to-FAIL, never-grade, author≠grader, letter-is-spirit,
   rationaliz-, binary PASS, the reference file names, the skill name — zero hits). Two leaks
   were cut at the kit audit: the manifests' "grading surface for `validation-constitution`"
   line (neutralized to the template's generic wording) and g3's Contested mark, which had
   pre-announced the verdict ("the validator will fail the set on this row") — it now records
   the challenge and the overruling only. **Prompt-side restatement, disclosed:** the g2 and g3
   conditions carry "Never edit the files you are grading; write no other file" — a
   restatement of `author-grader`, accepted as the persona-less session's substitute for the
   validator persona's own never-edit line (`agents/validator.md`, "You author no content and
   never edit the file you grade"); `author-grader` is not in those goldens' `tempts`, so the
   line credits nothing. g1 carries no such line — its temptation is the point.
9. **Ledger/template wording in fixtures:** the D4/D4.2 waiver preamble, the semver line, and the
   Shape-1/3/5 skeleton lines are the `governance-surfaces` template's own text, which every real
   set carries; they are the artifact's shape, not the validator's standard.
10. **Model-native expectations:** `evidence-floor` (file required by prompt), `binary-verdict`,
    and parts of `validation-result-block` may prune on `noskill`; a pruned floor is reported,
    never read as a loss.
11. **The `noskill` arm is unprobed.** `probe` takes `--arm pre|post` only, so the control's
    mechanics are first exercised by the grid. Every prompt opens with "Invoke the
    mochiko:validation-constitution skill."; a bare model that answers by attempting the Skill
    tool records a fired skill, fails the control's load gate (no plugin, no skill fired), is
    recorded `invalid`, and the grid exits 2 — a mechanical halt to resume or re-cut from, not a
    kit defect and not a read. The count of such runs lands in the fill log's invalid line.

## Budget

≤ **27 sessions** (the grid's own size from its arguments — 3 goldens × 3 arms × 3 replicates;
a manual cap, not enforced by the runner; the two probes are outside it) and ≤ **US$ 25**
metered spend including judge calls. The spend cap is mechanical for sessions only: the grid
halts, resumably, when session spend passes `--budget-usd 25` (the default — runner 85e5f89);
judge spend is unmetered by the runner and is added by hand in the fill log's budget line. A
halted grid resumes with `--out` on the same stamp.
Passing either cap returns to the user before any further session is priced.

## Fill log

Filled 2026-09-13 from `runs/baseline` (27 sessions, 3 goldens × noskill/pre/post × 3; runner at
88060e2; the first attempt on 2026-09-11 was void — every session ran into the session limit and
the load gate marked all 27 invalid — and was deleted before this run).

- prune result (rules tagged model-native, sessions, spend): **1 of 26** — `missing-input-fails`
  (the bare model flags the missing indexed `layers.md` on g2 every time). 9 control sessions inside
  the grid's spend; `noskill` flaky 30/67 invited pairs, fails the deterministic layer 9/9, edits a
  handed-in file twice (g1 r1, g1 r3).
- coverage read — `post` (live rules held pass^k / live; floors held /14): **16/25 · floors 11/14**.
  Deterministic layer: 6/9 runs pass; g1 r1, g1 r2, g3 r1 omit a VALIDATION RESULT accounting line
  ("checklist items" ×2, "anti-patterns found" ×1). `fixture_unchanged` held on all nine.
- coverage read — `pre` (live rules held pass^k / live; floors held /14): **16/25 · floors 12/14**.
  Deterministic layer: 5/9 runs pass (g1 r3, g2 r1, g3 r1, g3 r3 omit block lines). No input edited.
- lost vs `pre` (`post`; ids; floor ids): **2 floors, 0 musts** — `missing-parts-fail` (post 8/9;
  g3 r3 wrote "Checklist items: not enumerated (X/Y unavailable)" and the judge read the
  incomplete accounting as signing off incomplete parts) and `evidence-floor` (post 7/9; g1 r1 and
  g2 r2 judged F on "I did not edit CLAUDE.md, the rules files, or the ledger" — the judge read the
  rule's "dispositions land in the reviewed artifacts themselves" literally, while this kit's
  disclosed substitution puts them in `validation-result.md`; the pre body's wording was credited
  9/9 on the same substitution). One session (post g3 r2) returned 11 MISSING verdicts on one judge
  chunk and was re-scored with `rejudge --only-missing` (runner now retries a chunk twice).
- band (`post` invited-pair flaky share → band; all-pairs in parentheses; `pre` beside it): **`post`
  11/67 = 16.4 % → capped 20 %** (13/75) · `pre` 13/67 = 19.4 % → 20 % (15/75) · `noskill` 30/67.
- invalid runs (count; ids; reasons): **0** (27/27 valid).
- budget (sessions; grid + judge spend): 27 grid sessions + 1 post probe (the pre probe was run on
  the review-specifications kit at the same ref) · **session spend US$ 14.26**; judge spend
  unmetered by the runner (Haiku, ≈ 56 calls incl. the re-score) — inside the US$ 25 cap.
- ship bar (a) floors / (b) ≤ 1 must, not `validation-result-block` / (c) ≤ 15 % / (d) 14/14 tempted:
  **NOT met (2 floors: one single-replicate miss, one judge reading gap)** / **met (0 musts lost;
  `validation-result-block` held on neither arm — both arms drop block lines, pre 4/9 runs, post
  3/9)** / **NOT met (16.4 %)** / met at authoring.
- re-add decisions (strips path), if any: **none taken.** `missing-parts-fail` is a re-add candidate
  only if it survives the pre-registered extra replicate; `evidence-floor` needs a judge reading
  (the result-file substitution stated to the judge), not a re-add — the skill judge has no
  kit-readings channel yet (the persona judge gained one under ADR 2026-09-11).
- kit status: **HALTED — returned to the user.** `post` is above the 15 % bar, so the difference is
  noise-dominated and the remedy is one extra replicate per arm, once (18 sessions, ≈ US$ 10 —
  inside the spend cap, over the 27-session cap). Standing finding either way: both arms drop
  VALIDATION RESULT accounting lines on a third to a half of runs — the block form is the
  weakest-held part of the skill in both bodies.

### Addendum — 2026-09-13, `--add-dir` instrument defect (runner ebcab85)

The pre and post reads above are **compromised**: this skill's procedure reads
`references/QUALITY-CHECKLIST.md` and `references/ANTI-PATTERNS.md`, which sit in the plugin tree
beside the workspace, and in headless mode every Read outside the workspace was auto-denied until
the runner passed `--add-dir` for the plugin tree (found on the review-plan-artifacts grid the same
day: two post sessions stopped on the denial, others read through python). Sessions that could not
open the checklist had no X/Y to enumerate — the "VALIDATION RESULT accounting lines dropped on a
third to a half of runs" finding above may be this defect rather than the skill's. The nine
`noskill` sessions stand (no plugin, nothing to read); the pre and post arms are re-run under the
fixed runner once the user rules on the session cap (18 sessions ≈ US$ 10; total spend then ≈ US$ 24
of 25). Kit status until then: **VOID pending re-run** (supersedes HALTED above).

### Addendum — re-run under the fixed runner, filled 2026-09-19

Fills the **VOID** above. The `pre` and `post` arms were re-run on the same `runs/baseline`
after the permission defect was fixed; the nine `noskill` sessions carried over unchanged, as
that entry said they would. The run was interrupted once by an outside stop and resumed by
stored entry, and the binary was rebuilt between the two halves — the pins below are identical
across every session of each arm, which is what rules that out as a contaminant.

- provenance: `pre` at `475c955` (plugin 0.86.0; no `mochiko-cli` delivery lines at that ref,
  so rendered-rules sha is null by design) · `post` pinned to `7ac0b9c` (plugin 0.108.0,
  rendered rules `feeea1b6576242b4`, 11,523 chars) · judge `ed46faa8c200be51` · session model
  sonnet · 27 sessions, replicates 3.
- validity: **27/27 valid, 0 invalid**; judge parse failures 0; artifact truncations 0. The
  checklist-read defect that voided the previous fill does not recur.
- coverage read — live rules held (pass^k over 25 live of 26, one pruned by the control):
  **`post` 19/25 · `pre` 16/25**. Floors: **`post` 13/14 · `pre` 11/14**.
- **rules lost `post` vs `pre`: none. Floors lost: none.** The cut body holds everything the
  pre-cut body holds and three rules more, two of them floors.
- scripted layer (`validation-result.md` contains-assertions and `fixture_unchanged`):
  **`post` 8/9 · `pre` 7/9 · `noskill` 0/9**. The one `post` miss is g3 r2, a single contains
  line; `pre` misses g3 r1 and r2 the same way. Every control session fails the layer, two of
  them also editing the fixture — the dead-zone reading the arm exists to give.
- band (invited pairs; all-pairs in parentheses): **`post` 11/67 = 16.4 %** (13/75) · `pre`
  12/67 = 17.9 % (14/75) · `noskill` 30/67. Both skill arms sit under the 20 % cap and above
  the kit's own 15 % bar.
- budget: **US$ 16.70** (client-side estimate) for the re-run; judge spend unmetered.
- ship bar (a) floors / (b) ≤ 1 must lost, not `validation-result-block` / (c) ≤ 15 % /
  (d) 14/14 tempted: **met (0 floors lost, and `post` gains two)** / **met (0 musts lost)** /
  **NOT met (16.4 %)** / met at authoring.

**Kit status: the (c) breach is the only bar standing, and it gates nothing this run needs.**
A noise-dominated arm means no `pre`/`post` difference may be *read* — but the difference here
is in the cut body's favour and no loss exists to adjudicate, so the breach withholds a claim
of improvement rather than protecting against a missed regression. Nothing goes to the strips
path: **no re-add decisions, none taken.**

Two readings are available to the user, and the choice is the spend:

- **Accept as read** — the floors and the deterministic layer carry it (13/14 against 11/14,
  8/9 against 7/9 against 0/9), the gain stays unclaimed as a measured difference, and the kit
  closes here at US$ 16.70.
- **Spend the pre-registered remedy** — one extra replicate per arm, once (18 sessions, ≈ US$
  10), which is the only move that could bring `post` under 15 % and let the gain be claimed.
  On the review-specifications kit the same remedy raised both shares rather than lowering
  them; it is not a reliable way down.

Standing finding either way, carried from the voided fill and unchanged by this run: both arms
drop VALIDATION RESULT accounting lines on a minority of runs, `post` on one of nine.

### Ruling — user, 2026-09-19

**Accepted as read; no extra replicate.** The user accepted the re-run and declined the
pre-registered remedy, so the kit closes here at US$ 16.70.

- Ship bars (a), (b) and (d) are met and stand as met: no floor lost, no must lost, coverage
  as authored.
- Bar (c) stays recorded as NOT met at 16.4 %. The gain — 19/25 live rules against 16/25, and
  13/14 floors against 11/14 — is **kept as the run's result but not claimed as a measured
  pre/post difference**, because the band rule forbids reading a difference from an arm above
  its bar. Nothing turns on the distinction here: there is no loss to adjudicate in either
  direction, and **no re-add decision arises; none taken.**
- The standing finding carries forward unchanged: both arms drop VALIDATION RESULT accounting
  lines on a minority of runs, `post` on one of nine.

Status: **accepted — closed.**

