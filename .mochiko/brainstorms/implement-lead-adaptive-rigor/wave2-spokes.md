# Wave 2 — the spokes: each step's lighter form in the skill that owns it (`implement-lead-adaptive-rigor`)

**Opened:** 2026-10-28 · **Lead:** the build session lead · **Branch:** `adaptive-rigor-build`
(W1 closed at `5c4ba7c`) · **Target version:** 0.123.0, one bump after wave 3 · **Record:**
`.mochiko/brainstorms/implement-lead-adaptive-rigor/record.md` — D2-stakes-read-ruling (as
amended), D3-harm-band-smoke, D5-band-step-latitude, D8-home-sound-loop (its "each step's
lighter form stays in the skill that owns the step" sentence and the verify-pass residual on
`testing-end-user`), DF2, DF8, DF11, all as review-amended, are the source for every rule text.
This plan names carriers, never restates a decision; where it paraphrases, the card wins.

Wave 2 lands the lighter forms. `patterns-sound-loop` (W1, `0102`) holds the key, the band, the
never-thins floor, the invariants and the trail; the spokes gain only what they own: the
gap-finding pass's stated skip (`testing-gap-finding`), the smoke probe and the cold-run clause
(`testing-end-user`), the plan grade's once-per-seat form (`review-seat-plan`), and the harm-class
proposal field (`review-sufficiency`). Nothing here restates the band — every spoke rule points at
`mochiko:patterns-sound-loop` for it. Implement's keys (W3) consume what this wave mints.

## 0. Sound-loop wiring

Unchanged from W1 (`wave1-sound-loop.md` §0): persona-less `general-purpose` producers,
`model: opus`, plan-only dispatch, tree snapshot before and after, a fresh generic peer per plan
on `mochiko:review-seat-plan` (render pasted verbatim), the lead approving only a PASS, one gate
grader for the wave (DF11). Two things the W1 plan-only briefs omitted and these carry
explicitly: **the attempt bounds** — the re-plan bound is one, on a counter the producers share,
a second consumption to the user; the gate fix loop is `common.gate-loop-bound`'s, a second
audit FAIL to the user — and **the Explore fallback** — a haiku `Explore` that returns no report
(four of four in W1) means the locate reads run on-seat, disclosed.

## 1. Sequence allocation

- **`0103-spokes-stakes.yaml`** — four schemas in one migration (`skill/testing-gap-finding`,
  `skill/testing-end-user`, `skill/review-seat-plan`, `skill/review-sufficiency`): three
  `set-condition`, one `reword-section`, eight `mint-rule`, one `reword-rule`. Header anchor
  `2026-10-27 implement-lead-adaptive-rigor` (bare, as `0102`); each minted rule anchored to its
  card (`… D2`, `… D3`, `… D5`). Sequence 103; 0104 held for W3. No floor minted, so the frozen
  floor set changes only by W1 (W3 re-freezes once).

## 2. Seats and ownership

- **P1 — schema.** Owns `plugins/mochiko/migrations/0103-spokes-stakes.yaml`, the regenerated
  `.mochiko/schema-views/` (`mochiko-cli views emit`; expect four skill view files to change),
  and any `scripts/similar-rules-allowlist.yaml` row a near-dup cluster forces. Nothing else.
- **P2 — prose.** Owns `plugins/mochiko/skills/testing-end-user/SKILL.md` (its `description:`
  gains the SHOULD phrase `'smoke probe'`; its Overview gains one sentence saying the stakes smoke
  probe lives here; the Rules block and the read-back sentence untouched) and
  `.mochiko/memory/primitive-cost-budgets.md` (the four rows `testing-gap-finding` ·
  `testing-end-user` · `review-seat-plan` · `review-sufficiency`, re-measured after `0103`
  lands; `testing-end-user`'s description row in the descriptions table). No strip entry is
  owed: nothing is superseded — rewords append, the section note cleared is schema content the
  migration records. Nothing else.

## 3. Carriers

Every `when:` below uses the list form, on the whole list —
`when: {stakes: [money, data, security, availability, wrong-answer, cosmetic]}` — as `0102` keys
`stakes-band`: a stakes-keyed rule binds whenever a class is resolved (product work), its text
carrying the band split; the fixed-band values stay covered, so no `condition-coverage` advisory
enters the baseline. *(Amended 2026-10-28 at P1's plan round, Q1/Q4: the "three" column below
reads as six; a smoke may be planned in the fixed band, D3's cost line.)* Two text rulings from the
same round: `once-per-seat-in-band` says the thinning is ruled at run-open and stands provisionally
until the smoke, a red smoke or any in-run raise returning every later plan to the full grade (Q2);
`band-skip-stated` names the user's later checkpoint ruling beside run-open (Q3, D2).

| id | op · class · kind | section | when | labels | source | gist |
|---|---|---|---|---|---|---|
| condition `stakes` on `testing-gap-finding`, `testing-end-user`, `review-seat-plan` | `set-condition` ×3 | preamble | — | — | D3, D8 | values the six harm classes · `entry-derived` · note: the run's ruled harm class, carried in from the command's run-open ruling (`mochiko:patterns-sound-loop`); absent where the run is not product work |
| `testing-gap-finding.band-skip-stated` | mint · must · latitude | scope | three | boundary, reporting | D5, D8, S4/N4 | in the band the pass may be skipped only per the plan the user ruled at run-open and on a clean cold smoke, never on the finder's or the lead's own mid-run call; the final-validation report states the skip with the smoke result it rode on and what still covers the failure kind (the smoke's hand probes over the changed behaviour, the non-author read before acceptance); a skip not stated fails the run under the command's own fail rule; money, data, security: the pass runs whatever any probe says; band, smoke and trail are sound-loop's, referenced (`pointer: mochiko:patterns-sound-loop`) |
| `reword-section testing-end-user.sec.independence` | reword-section | independence | — | — | D3, D8 | title "Independence — the smoke seat" · intent "Who runs the stakes smoke probe, and what it never authored." · `note: ~` (the "Deliberately empty" census note cleared, since the section now holds a rule) |
| `testing-end-user.smoke-seat` | mint · must · binding | independence | three | independence, binding | D3 (N3), D8 | the smoke probe is run by the run's QA seat — `mochiko:qa-engineer` on its persona tier — which wrote no product code in the run: it authors `**TEST:**` cases, never the code they test; persona carries the judgment, this skill the procedure (`pointer: mochiko:qa-engineer`) |
| `testing-end-user.smoke-procedure` | mint · must · duty | scope | three | evidence, ladder | D3 (S5), DF2 | at the point the ruled plan names (default: after the build, before final validation), in strict order: build the thing · run the full automated suite · start it on real infrastructure · probe the changed behaviour by hand · hold the diff against the ruled harm class; a diff touching a money, data or security surface voids every thinning and the run goes full depth, any red does the same; scope bounded to those five legs — the lighter form of the steps that thin on it, never a substitute for a declared `**TEST:**` case |
| `testing-end-user.smoke-clean-weak-negative` | mint · must | verdict | three | verdict, evidence | D3 | a clean smoke is a weak negative, never a verdict of safety: reported method-scoped — `clean by <probes> over <surface>` — and it thins only the steps the user's run-open ruling named, as mechanical execution of that ruling, never a step on its own |
| `testing-end-user.smoke-report` | mint · must · binding | output | three | reporting, binding | D3 (S8) | lands as a `verification` report carrying `smoke: true` (the envelope admits the extra field) under `references/REPORT-TEMPLATES.md`'s frontmatter, the probes run and the surface covered named; evidence rides the run folder like any task's |
| `testing-end-user.cold-run-always` | mint · must · duty | scope | six | evidence | D5 (S7), D6 row 5 | the cold run is the one final gate run: the smoke, where the plan names one, runs in the dependency-cold snapshot the command's cold verification takes — never a warm run followed by a cold one — and that run always happens, on the exact head the user accepts, repeating whenever rework changes code after it, within the command's existing gap-rework bound; its results are acceptance evidence, no CI result stands in for it |
| `review-seat-plan.once-per-seat-in-band` | mint · must · latitude | scope | three | boundary, user-gate | D5 (worked thin plan), D8 | in the band, per the plan the user ruled at run-open and on a clean cold smoke, the grade runs once per seat per run: a seat's first plan is graded on the seven items, its later plans in the same run ride that PASS — each still reaching the lead verbatim, the thinning stated in the trail; a seat re-staffed, re-scoped or re-planning after a FAIL is graded again; money, data, security: every plan is graded |
| `review-sufficiency.stakes-proposal` | mint · must · duty | verdict | — | verdict, evidence | D2 (as amended), D3, DF3, DF8 | one more field per unit: the proposed stakes class — the worst kind of thing that breaks if the batch's product work is wrong, from `mochiko:patterns-sound-loop`'s closed list — read from one cited line of the spec or a baseline inside the read fence and quoted with it; on an epic, one per member; where no line can be cited, propose the fixed band (money, data or security) and say so; a proposal, never a ruling — the lead may raise it, never lower it, the user rules it at run-open |
| reword `review-sufficiency.report-binding` | reword-rule (must · binding, kept) | output | — | kept | D2 (as amended), S8 | append to the carried set: "· the proposed stakes class per unit with its cited line (`review-sufficiency.stakes-proposal`)"; prior text verbatim as prefix |

Floors: unchanged in every spoke (9 · 7 · 5 · 8). Rules: `testing-gap-finding` 36 → 37 ·
`testing-end-user` 25 → 30 · `review-seat-plan` 15 → 16 · `review-sufficiency` 29 → 30. No
`kind: fail`, no `${var}`, no new label; pointers resolve to a live skill and a live persona.

## 4. Gate units (one grader, `general-purpose`, `model: opus`, rendering `mochiko:validation-primitive-edit` first-hand)

- **Unit 1 — schema content:** `0103-spokes-stakes.yaml` + the four regenerated view diffs
  (+ any allowlist row). Items: intent stated · anchor present · ID lifecycle right · floor and
  fail survival · register. Pre-pass: `mochiko-cli migrate validate --report`, 0 rejecting.
- **Units 2–5 — skill pairs:** `testing-end-user` (SKILL.md edited + render), `testing-gap-finding`,
  `review-seat-plan`, `review-sufficiency` (render changed, SKILL.md unchanged). Items: the
  skill-pair criteria of `.claude/rules/mochiko/primitive-edits.md`; `description:` ≤ 1,536; the
  budget rows against the grader's own canonical-snippet measurement, each overage argued in its
  row as D8's ruled obligation (the lighter form lands in the owning skill, by the record's word).
  Criterion 7's byte-identical limb is conversion-scoped; `testing-end-user`'s description change
  is graded on the cap and its own description budget (625).

## 5. Close

All units PASS → the lead commits the wave on the branch ("W2: the spokes carry each step's
lighter form"), appends the audit outcome lines to the build log, and opens W3. A FAIL → fix by
the same producer, re-audit by the same grader on the touched files only; a second FAIL halts to
the user with both fix lists.
