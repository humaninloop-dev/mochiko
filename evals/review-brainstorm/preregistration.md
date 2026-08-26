# Pre-registration — `review-brainstorm`

> **RATIFIED 2026-08-26** — user ruled "ratify as proposed": 6-rule floor set (R-054 not
> promoted), armB bound ≤5 of 61 non-floor / 0 vocab / ≤1 must, representative invoke = solo
> review (body + RECORD-FITNESS + EXTERNAL-CLAIMS). Bar drafted by the non-compressor eval
> seat; ratification the user's (the compressor never sets its own bar). No value below
> changes after the first priced run.

**Skill under test:** `plugins/mochiko/skills/review-brainstorm/` (cut at v0.83.0)
**Check type:** **post-cut regression check** — the `review-feasibility` v0.82.0 pattern ("cut
now, eval validates later"). The cut has already shipped; this instrument exists to detect
behavioral rules it lost, not to gate it.
**Inputs:** `rules.json` (67 rules) · `evals.json` (3 goldens: incident paging · offline field
sync · billing migration)
**Arms:** `noskill` · `armA` (**the pre-cut baseline**) · `armB` (the cut) — 3 goldens × 3 arms
× 3 replicates = **27 runs**. Session model: Sonnet (R7). Aggregation: pass^k.
**Authorship:** rules and goldens authored by a non-compressor seat (D4/D8 author ≠ grader).
The compression itself was authored by the session lead; see §7 for the honest limit on that
independence.

---

## 1. Rule inventory shape

Built from the **pre-cut** body (`git show HEAD:plugins/mochiko/skills/review-brainstorm/SKILL.md`)
plus the three unchanged references, read before the cut was read. No rule was pruned for being
absent from the cut.

| Class | Count | What it is |
|-------|-------|------------|
| `floor` | 6 | Absolute. One lost floor rule kills the arm. |
| `must` | 41 | Graded obligations the baseline asserts as MUST-equivalent. |
| `format` | 6 | Report-shape obligations (finding fields, tally, severity labels, status vocabulary, findings-formed count-only, map breadth). |
| `should` | 2 | Graded obligations the baseline asserts as SHOULD-equivalent (both conditionally vacuous — see §6). |
| `vocab` | 12 | Owned vocabulary a **named consumer inside `plugins/mochiko/` cites** (R10 consumer check). |
| **Total** | **67** | Non-floor denominator: **61** |

Source spread: 44 rules from the `SKILL.md` body · 9 from `references/RECORD-FITNESS.md` ·
2 from `references/EXTERNAL-CLAIMS.md` · 12 from consumer files outside the skill.

## 2. Proposed floor set (absolute — one lost kills the arm)

| ID | Obligation | Anchor |
|----|------------|--------|
| R-001 | A blind angle map is produced as its own deliverable, mapping the **topic's** angles, not the record's. | Ruled machinery, protected in `.mochiko/strips/review-brainstorm.md` **[v0.60.0]**; the consumer command calls its fence "structural, not a trust ask" (`commands/brainstorm.md:40-44`); named in the skill's own `description`. |
| R-002 | Never authors, revises, or completes the record — no `record.md` in the produced file set. | Named in the cut's own `**Floors:**` line; `Common Mistakes` bottom row; the `description`'s identity claim ("never a co-author"). |
| R-003 | Never in the room — no co-author voice, no softening because the participants evidently thought about it. | The `description`'s first sentence; the whole `Independence` section; consumer router row `skills/mochiko/SKILL.md:112` ("never a session co-author"). |
| R-004 | Never raises a `Contested` decision, unless the finding states its angle is new to the ruling. | Named in the cut's own `**Floors:**` line; stated in absolute "Never" form in the baseline; the one rule protecting a **user ruling** from re-litigation. |
| R-005 | Never defaults to `ready`. | Named in the cut's own `**Floors:**` line; bolded in the baseline's Phase 3; the `description`'s "defaults to FAIL". Lose it and the default-FAIL posture inverts. |
| R-006 | An unverifiable load-bearing claim is a finding, not the benefit of the doubt. | Named in the cut's own `**Floors:**` line; the one rule with a **correctness** consequence rather than a quality one — it is how a false load-bearing claim gets stopped before acceptance. |

**Why these six.** They are anchored, not chosen freely: **four of the six are named in the
compressor's own `**Floors:**` line** — the cut's self-declaration of what it considers
non-negotiable, which is the fairest possible anchor for grading the cut. R-001 is protected
ruled machinery with a consumer command enforcing it structurally. R-003 is the skill's identity
claim, restated by the router row that sells this seat to the rest of the library.

**Deliberately NOT floor, flagged for the user:**

- **R-054** (*status is input; the lead owns the clearing verdict*) has an equal textual claim —
  it too sits in the cut's `**Floors:**` line. It is left at `must` because it is a framing
  property rather than an action, and a solo headless golden with no reachable lead grades it
  weakly. **Strongest promotion candidate if the user wants a seventh floor.**
- **R-053** (*review evidence never lives only in conversation*) is called "a floor violation" in
  the baseline verbatim. It is not proposed as floor because **this harness inverts its
  transport** — the reviewer writes a file here precisely because no conversation channel
  exists (§6.2), so grading it would measure the harness, not the skill.

## 3. Proposed rules-lost bound

**One sentence:** `armB` may lose at most **5 of the 61 non-floor rules (≈8.2%)**, none of them
`vocab` and at most 1 of them `must`, because the inherited decision threshold is 10% (F4) and
pass^k already biases toward declaring loss, so a bound below the threshold leaves margin for
replicate flakiness without letting a real regression pass.

| Arm | `floor` | `vocab` | `must` | `should` + `format` | Total non-floor |
|-----|---------|---------|--------|---------------------|-----------------|
| `armA` (pre-cut baseline) | — | — | — | — | reference arm, not bounded |
| `armB` (the shipped cut) | 0 | 0 | ≤ 1 | ≤ 4 | **≤ 5 of 61** |

- **There is no lossless-densification arm.** D2's Arm A was never staged for this pass — the
  user directed a straight cut, and the pass report records the breakup draft being rejected
  mid-pass. `armA` here carries the **pre-cut baseline** instead; the name is forced by
  `run.py` (§5).
- **`vocab` losses are bounded at zero.** A lost vocab term does not degrade the report, it
  breaks a named consumer (R10). Consumers are enumerated per rule in `rules.json` — the
  heaviest are `commands/brainstorm.md`'s blind-map dispatch and coverage-survivor routing,
  `templates/advocate-report-template.md`'s `verdict:` field, and the eight files pointing at
  `references/EXTERNAL-CLAIMS.md`.
- **`must` is bounded at ≤ 1, tighter than the entity-modeling precedent's ≤ 1 of 37**, because
  this cut is −78.8% rather than a trim: musts are the skill's graded obligations, and a cut
  losing two or more is a behavior change, not a compression.
- **Pruning happens first (R3):** any rule holding in the `noskill` control across all
  replicates measures the model, not the skill, and leaves the denominator before the bound
  applies. 61 is the pre-pruning denominator; read the bar as "≤ 5 lost", with the percentage
  recomputed post-pruning and recorded in the run report.
- **What a breach means here.** This is a post-cut check on shipped content, so a breach is not
  a ship veto — it is a **re-add decision**, executed through the strips re-add path named in
  `pass-report.md`, never by silent edit.

## 4. Delivered-chars arithmetic (R9)

Measured with `python len(path.read_text())` — chars, never `wc -c` bytes (F4/D7).

| Surface | `armA` (pre-cut) | `armB` (cut) | Δ | Loaded when |
|---------|------------------|--------------|---|-------------|
| `SKILL.md` frontmatter `description:` | 490 | 490 | 0 | **always** (out of scope, D6) |
| `SKILL.md` body | 11,754 | 2,497 | **−9,257 (−78.8%)** | on invoke |
| `SKILL.md` whole file | 12,291 | 3,034 | −9,257 | on invoke |
| `references/RECORD-FITNESS.md` | 1,606 | 1,606 | 0 | on demand (every cold read) |
| `references/EXTERNAL-CLAIMS.md` | 5,034 | 5,034 | 0 | on demand (any outside-repo claim) |
| `references/CROSS-EXAM.md` | 3,159 | 3,159 | 0 | on demand (**pair only**) |
| **In-scope total (body + 3 references)** | **21,553** | **12,296** | **−9,257 (−42.9%)** | |

The three references are byte-identical across arms — the cut touched the body only.

**What one invoke delivers.** Three honest cases, because references load on demand. The
proposed **representative invoke** is the middle row: it is exactly what these three goldens
exercise — a solo cold read always runs record-fitness, and every golden carries a load-bearing
outside-repo claim, while `CROSS-EXAM.md` never loads because no golden is a pair.

| Representative invoke | `armA` | `armB` | ≈ tokens saved (chars/4) | Δ |
|-----------------------|--------|--------|--------------------------|---|
| Body only (no reference read) | 11,754 | 2,497 | ~2,314 | −78.8% |
| **Solo review = body + RECORD-FITNESS + EXTERNAL-CLAIMS (proposed)** | **18,394** | **9,137** | **~2,314** | **−50.3%** |
| Pair full fan-out (body + all three references) | 21,553 | 12,296 | ~2,314 | −42.9% |

**The honest note (R9, kept).** The saving is a flat 9,257 chars (~2,314 tokens) in every case —
the body is the only surface that moved. The always-loaded surface is the 490-char
`description`, which D6 puts out of scope, so nothing here reduces the cost of *not* invoking
the skill. Nothing in this arithmetic authorizes the cut; the cut is already ruled and shipped.
It prices what was traded, so the rule-loss result can be read against a number.

## 5. Run plan — and the `run.py` limitation that shapes it

**The `baseline` arm is unusable for this check.** `run.py:arm_source` resolves `baseline` to
`plugins/mochiko/skills/<skill>/`, which is the working tree — **already the cut**. Running it
would compare the cut against itself and report zero rules lost by construction.

Two further mechanics settled the plan:

1. `ARMS = ["noskill", "baseline", "armA", "armB"]` is hard-coded, and `summarize()` drops any
   arm whose name is outside that list (`all_arms = [a for a in ARMS if ...]`). A custom name
   like `precut` would run and then vanish from the summary. **So the pre-cut skill must be
   staged as `armA`**, even though `armA` conventionally means the lossless-densification arm.
   Staged at `evals/review-brainstorm/variants/armA/` from
   `git show HEAD:plugins/mochiko/skills/review-brainstorm/**`.
2. `cmd_probe` hard-codes `arm_source(skill, "baseline")`, so the probe loads the **cut**. That
   is fine for what the probe settles (sandbox auth, plugin visibility in the init event,
   whether a write lands under `acceptEdits`) — it writes `probe.txt`, not a golden.

**Commands:**

```
python3 evals/run.py probe review-brainstorm                                    # mechanics only
python3 evals/run.py grid  review-brainstorm --arms armA --replicates 1 --out preflight
python3 evals/run.py grid  review-brainstorm --arms noskill,armA,armB --replicates 3
```

The middle line is the **assertion pre-flight**: because the `baseline` arm never runs,
`summary.json`'s `baseline_assertion_failures` stays empty and the nonzero-exit gate never
fires. Confirm all 8 scripted assertions pass on `armA` by reading that run's stored
`assertions` entries **by hand** before spending on the full grid.

**Reading the loss.** The emitted `report.md`'s "rules lost" line is computed against
`baseline` and will be meaningless here — ignore it. Compute the real figure from
`summary.json`'s `held` map:

```python
import json
s = json.load(open("evals/review-brainstorm/runs/<stamp>/summary.json"))
pruned = set(s["rules_pruned_by_noskill"])
lost = [r for r in (x["id"] for x in json.load(open("evals/review-brainstorm/rules.json")))
        if r not in pruned and s["held"].get(f"armA:{r}") and not s["held"].get(f"armB:{r}")]
```

The alternative — reverting the cut in the working tree so `baseline` resolves correctly — is
**not proposed**: it means mutating a shipped primitive in order to run an eval.

## 6. Disclosures — obligations NOT encoded as rules

Named rather than silently dropped.

1. **The Phase 0 ordering fence is not enforceable in a single session.** `run.py` has no
   fixture channel: a golden is one prompt string in an empty workspace, so the record is
   inlined in the same message as the topic. The reviewer therefore *sees* the record before it
   can build a map, which is precisely what `commands/brainstorm.md`'s two-message dispatch
   exists to prevent. R-001 grades the **product signature** of a blind map (a separate
   topic-level deliverable naming ground the record never stood on), never the ordering. The
   prompt nudges ordering — "if your protocol requires you to produce anything before you read a
   record, produce it first" — without naming the map, so the `noskill` control gains nothing.
2. **Transport is inverted, deliberately.** The baseline says the survivor report returns "as a
   message (no report files)". `run.py:collect_artifact` grades **workspace files only** — the
   final assistant message is parsed into `result` and then discarded. The goldens therefore
   require `review.md` and say plainly that the file *is* the message. **The "no report files"
   obligation is not encodable and is absent from `rules.json`.** The substitution is identical
   in every arm, so it cannot bias `armA` against `armB`.
3. **Nothing transcript-only is encodable.** `run.py` keeps no transcript (stream events are
   parsed for `init`/`result` and dropped). Unverifiable as a result: read ordering, the free
   repo grounding read, and `EXTERNAL-CLAIMS.md`'s inline WebSearch/WebFetch mechanics.
4. **Pair-only rules NOT encoded** — the whole of `references/CROSS-EXAM.md` beyond the solo
   skip: the four-message exchange, the attack/defence standards, withdrawal-is-the-owner's-alone,
   one-route-per-fact, the external-claim carve-out, pair source-conflict resolution,
   both-reviewers-build-maps-independently, sequestration until the lead introduces the
   counterpart, and the report's unresolved-objection / flagged-duplicate / cross-set-merge
   fields. **Consequence: a 3,159-char in-scope reference rides on R-042 and R-067 alone.** If
   the user wants the pair protocol genuinely measured, that needs a multi-session harness
   `run.py` does not have — flagged, not assumed.
5. **Verify-pass rules NOT encoded** — fold verification with quoted evidence, the
   no-new-surface-except-fold-contradictions clause, the synthesis fidelity sample (every ruling
   present, no confidence mark inflated, no rejected alternative resurrected), reopen-born
   grading, and the one-level recursion stop. No golden dispatches a verify pass, because a
   verify pass presupposes a prior review and a lead's folds. R-066 carries the `reopen-born`
   **vocabulary** with an explicit not-applicable note (the precedent's R-078 pattern).
6. **The lens split runs vacuous.** No golden briefs a lens, so R-008 and the integrity lens's
   map sample-audit are never exercised. **If the user wants the lens split measured, a fourth
   golden briefing `record-integrity` is the fix** — flagged, not assumed.
7. **No fact-checker map exists by construction.** The goldens state so explicitly, so the
   "check against the map instead of re-deriving" half of R-023 and the integrity-lens map audit
   cannot fire. R-023 grades the disclosure half only.
8. **The workspace has no files, and live external verification may be unreachable in the
   sandbox.** R-006 and R-024 therefore grade the **treatment** of a load-bearing claim — raised
   as a finding, or verified with cited quotable text — and pass on either path. They never
   require a successful fetch.
9. **Two rules are conditionally vacuous by design** and say so in their own text: R-008 (no
   lens briefed) and R-051 (no golden's record is too thin to attack). The judge is instructed
   in-rule to mark them not-applicable rather than failed, per the precedent.
10. **Assertion brittleness is settled by the pre-flight, not here.** The 8 scripted assertions
    per golden are a conformance contract, deliberately loosened (bare-phrase and alternation
    matches) to cut false alarms. §5 states how to confirm them on `armA` before any priced grid.

## 7. The independence caveat (post-cut checks specifically)

A pre-cut eval measures a candidate; this one measures something already shipped. Two
consequences the user should hold while reading any result:

- **The compressor's own session authored the cut.** This seat — rules and goldens — is the
  independence leg, and it is the only one. The inventory was built from the HEAD baseline text
  before the cut was read, and no rule was dropped for being absent from the cut; that is the
  whole of the protection.
- **A rule loss found here cannot un-ship the cut.** It becomes a re-add through the strips
  path. The instrument's value is that the re-add decision gets made against evidence rather
  than against the compressor's own account of what survived.

## 8. Ratification

The user rules on: **(a)** the six-rule floor set (§2), including whether R-054 is promoted to a
seventh; **(b)** the ≤ 5 non-floor / 0 vocab / ≤ 1 must bound (§3); **(c)** the representative
invoke for the R9 arithmetic (§4, middle row proposed); **(d)** the `noskill,armA,armB` run plan
and the hand-computed loss figure (§5). On ruling, this file is renamed `preregistration.md` and
no value in it changes after the first priced run.
