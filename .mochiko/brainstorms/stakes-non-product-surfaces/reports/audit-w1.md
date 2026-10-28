---
report: review
round: 1
grader: w1-gate-grader
tier: opus
wave: 1
units: [1, 2, 3, 4, 5]
contract: mochiko-cli rules validation-primitive-edit (preamble + six sections, binary 0.4.0, plugin 0.123.0) — floor pin 11, read back before the first step
---

## Failure narrative

Unit 2 fails on one judgment item, the argued overage. The `patterns-sound-loop` row in
`.mochiko/memory/primitive-cost-budgets.md` argues the +3,488 render growth as "four rewords
(`kind-keyed-library-wide`, `stakes-key`, `stakes-band`, `review-form-invariants`, D1 items 3–6)"
plus one minted floor. `0105` carries five rewords. The fifth, `patterns-sound-loop.disclosure-line`
(the `stakes:` segment reads `n/a` only off product work and the three outputs), is in the
migration's intent and in the view diff, and it adds +99 to `sec.disclosure` (1,938 → 2,037). It
is not named in the row and is not on D1's "exactly this list". The migration ties it to D1 item 3,
since without it the line would print `n/a` on a spec review that has a ruled class, and that
reasoning holds. But the ledger row, which is the persisted overage argument the HOLDS ruling
rests on, miscounts what it argues. The measurement itself reproduces exactly. Nothing else was
tried; the fix is the editor's (author-grader floor).

## Pre-pass (shared, run first-hand by the grader)

- Contract read-back before the first step: `- class: floor · 11 rules`; floors author-grader · plain-seat-explicit-tier · rendered-contract-only · from-file-floor · pre-pass-first-hand · binary-verdict · default-fail · tamper-proof-clause · gate-loop-bound · evidence-floor · second-fail-user. Every block carried its version-triple and end line; 4 + 3 + 3 + 7 + 3 + 1 = 21 rules.
- `mochiko-cli migrate validate --report --plugin-root plugins/mochiko`: `pointer resolution: 91 checked against plugins/mochiko` · `similar-rule clusters (threshold 0.60) … none` · `rules scanned: 1284 · in-kind pairs scored: 225650 · clusters: 0 (none)` · `allowlist-suppressed edges: 182` · `mochiko-cli migrate validate · 0 rejecting · 118 advisory`.
- Control (scratch copy of `plugins/mochiko` without `0105`, `SKILL.md` at `5691a67`): `0 rejecting · 118 advisory`. Line diff against the live report: only `budget · skill/patterns-sound-loop · 21 rules · 9326` → `22 rules · 12396`, and the cluster listing, which differs only because the copy sits outside the repo and finds no allowlist. No new `condition-coverage`, `zero-member-label` or pointer finding.
- `mochiko-cli migrate status`: `sequences 1..105 (93 migrations)` · `state sha256:e30d0f115ded…` · `94 documents · 1284 rules`. Control: `sequences 1..104 (92 migrations)` · `state sha256:84802fd7bf33…` · `1283 rules`.
- `mochiko-cli views emit --out <scratch>` then `diff -r` against `.mochiko/schema-views`: `VIEWS IDENTICAL` (derived view ≡ replay).
- Char budget, canonical snippet (body, description) plus the seven render blocks as characters of stdout:

  | patterns-sound-loop | body | desc | preamble | trigger | scope | discipline | inputs | disclosure | reserved | render | payload |
  |---|---|---|---|---|---|---|---|---|---|---|---|
  | now | 3,406 | 823 | 2,788 | 5,850 | 698 | 6,690 | 423 | 2,037 | 929 | 19,415 | 22,821 |
  | control (no `0105`, body `5691a67`) | 3,315 | 746 | 2,436 | 4,407 | 698 | 5,096 | 423 | 1,938 | 929 | 15,927 | 19,242 |

  Budget 12,388; payload 22,821, which is +10,433 standing and +3,579 this wave (render +3,488 = preamble +352 · trigger +1,443 · discipline +1,594 · disclosure +99; body +91). Description 823 ≤ 1,536.
- Crate (unit 3): `cargo test --all --no-fail-fast` gives 20 `test result:` lines, `passed 765 failed 0`, exit 0 · `cargo fmt --all --check` exit 0 · `cargo clippy --all-targets -- -D warnings` exit 0 · `MOCHIKO_FULL_SIMILAR=1 cargo test --release --test matrix_similar the_detector_reproduces_the_live_runs_figures_over_the_corpus`: `test result: ok. 1 passed; 0 failed; … 47 filtered out`.
- No-allowlist raw runs (scratch copies): `1..104` gives `clusters: 76` · `182 edges unsuppressed`; `1..105` gives `clusters: 76` · `182 edges unsuppressed`.

## Unit 1 — schema content: `0105-sound-loop-planning-stakes.yaml` + view diff

```
VALIDATE: unit 1 — plugins/mochiko/migrations/0105-sound-loop-planning-stakes.yaml + .mochiko/schema-views/skills/patterns-sound-loop.yaml diff (schema content)
Checklist run: AM-2 five — intent stated · anchor present where required · ID lifecycle right · floor and fail survival · register
Evidence read: 0105 whole; git diff 5691a67 -- .mochiko/schema-views/skills/patterns-sound-loop.yaml (whole); record.md D1-planning-quick-read (statement, items 1–11, revert route), D2-borrow-product-label, D3-lead-proposes-label, DF1–DF6; rendered preamble (pins, floors line, stakes note)
Pre-pass: migrate validate 0 rejecting · 118 advisory; views emit ≡ .mochiko/schema-views; status 1..105 · 1284 rules
PASS intent stated — header names every op and its ruling: set-condition (stakes note), five rewords (kind-keyed-library-wide D1 item 3 · stakes-key D2/D3 · stakes-band D1 item 5/DF4 · review-form-invariants D1 items 1 and 6 · disclosure-line tied to D1 item 3), one mint (D1 statement, D3); names what it keeps (never-thins, D1 item 2; consolidation D1, item 9) and defers (item 1's bend lands in full with review-specifications); floors 10 → 11, rules 21 → 22 match the render pin and validate's budget line.
PASS anchor present — header anchor `2026-10-28 stakes-non-product-surfaces`; the mint carries `2026-10-28 stakes-non-product-surfaces D1`; rewords keep prior anchors under the header's authority, which covers the floor reword of review-form-invariants (D1 item 6). CLI anchor checks: 0 rejecting.
PASS ID lifecycle — one mint (`patterns-sound-loop.planning-quick-read`, new slug); five rewords keep id, class, kind, labels, pointer and anchor (view diff touches text only; stakes-band `when:` and `pointer:` lines unchanged); no tombstone; mint-once/id checks clean in the pre-pass.
PASS floor and fail survival — no floor retired or lowered; `never-thins` absent from the view diff (kept whole, D1 item 2); review-form-invariants keeps all five invariants verbatim and narrows only D8's completed-hunt rule to the quick read, per D1 item 1 (D8 names specs only, so the bend reaches nothing else); gate jobs stay fixed (DF1); DF2's surfaces are listed by name in kind-keyed-library-wide; DF6 no-cite default kept in stakes-key. Skill schema: no kind: fail.
PASS register — plain prose in the rule voice of its neighbours; each ruled element lands once in its own rule: borrowed class, intent-synthesis source, highest-class-touched as a harm read, higher label over two outputs (D2), lead proposes and quick reader raises, no seat lowers (D3), existing confirmation point (DF5), blind list skippable and plan check lightened (DF4), "open hunt not run" never "no gaps" and the raise not re-running a skipped blind list (D1, D3).
VERDICT: PASS
Issues requiring fix: none
```

Advisory, not blocking: `planning-quick-read` carries no `when:`. That is legal, because a floor is always delivered and the text self-gates on the three low classes.

## Unit 2 — skill pair: `patterns-sound-loop` `SKILL.md` + render

```
VALIDATE: unit 2 — plugins/mochiko/skills/patterns-sound-loop/SKILL.md + mochiko-cli rules patterns-sound-loop (preamble + six sections) (skill pair)
Checklist run: judgment-items-pair — scaffold · preserved responsibilities · floor survival · independence · reserved-to-user · done-condition branch · argued overage
Evidence read: SKILL.md whole; git diff 5691a67 of SKILL.md; all seven rendered blocks (measured, preamble read); .mochiko/strips/patterns-sound-loop.md [v0.124.0]; .mochiko/memory/primitive-cost-budgets.md patterns-sound-loop row (diff); control render at 1..104
Pre-pass: migrate validate 0 rejecting · 118 advisory; budget table above (payload 22,821 vs 12,388)
PASS scaffold headings and order — frontmatter (name · description · allowed-tools: Bash(mochiko-cli *)) · title · two headline lines · ## Overview · ## Rules — delivered by mochiko-cli with seven ! lines in preamble-then-family order (trigger · scope · discipline · inputs · disclosure · reserved) · read-back sentence citing the pin and floors line; unchanged by the edit.
PASS preserved responsibilities — the Overview's non-product half kept byte for byte; every prior description trigger phrase and the MUST/SHOULD form kept, 'quick read' added; superseded text recorded verbatim in the strip [v0.124.0] entry with ruling, kept-deliberately and consumers.
PASS floor survival — pin 10 → 11 (planning-quick-read added); every prior floor id still in the floors line; never-thins render unchanged.
PASS independence — planning-quick-read binds a reader that "authored none of the change" plus invariants (1)–(4); no rule lets a seat grade its own output.
PASS reserved-to-user — the label stays the user's ruling (stakes-key: "the user rules it", "no seat lowers it"), placed beside the 0102 user-ruling clauses graded HOLDS last build; reserved section unchanged, 2 rules.
PASS done-condition branch — n/a: a skill carries no Not-done pin or run/desk branch.
FAIL argued overage — measurement reproduced exactly (22,821; +3,579 = render +3,488 from 0105 alone + body +91), and the D1 ground holds for the content. But the budget row argues "four rewords (`kind-keyed-library-wide`, `stakes-key`, `stakes-band`, `review-form-invariants`, D1 items 3–6)". 0105 has five; `disclosure-line` (+99 in sec.disclosure, 1,938 → 2,037) is unnamed and unargued in the persisted overage argument.
VERDICT: FAIL
Issues requiring fix:
1. argued overage — the `patterns-sound-loop` row in .mochiko/memory/primitive-cost-budgets.md omits the fifth reword. Fix: change "four rewords (…, D1 items 3–6)" to five, adding `disclosure-line` with its ground (the `stakes:` segment's `n/a` narrowed to off-product, off-planning objects, a consequence of D1 item 3) so the argued set matches 0105. Same touch, not separately blocking: the strip [v0.124.0] "Additions riding the decision row" sentence lists the reworded rules without `disclosure-line`; add it there too.
```

## Unit 3 — crate tests, the non-author code review

```
VALIDATE: unit 3 — crates/mochiko-cli/tests/{fidelity,matrix_similar,validate}.rs (crate non-author review)
Checklist run: rust-cli.md quality gate (test · fmt · clippy, full similarity sweep) and an independent read of the diff, every pin and comment claim reproduced
Evidence read: git diff 5691a67 -- crates/ (whole, three test files, no src/ change); 0105 op list; rendered preamble; migrate status/validate; raw no-allowlist runs at 1..104 and 1..105; scripts/similar-rules-allowlist.yaml:436
Pre-pass: as above — 765 passed / 0 failed, fmt 0, clippy 0, full-similar live-figures test ok
PASS census pins — one mint, no retire: skill 857 → 858, total 1,283 → 1,284 (status prints 1284), skill floors 270 → 271 (render pin 10 → 11); command 426, command floors 130, fail nodes 41 unmoved (0105 touches only skill/patterns-sound-loop).
PASS census comments — the five named rewords match 0105's reword-rule ops by id; the review-form-invariants floor stays a floor; the set-condition carries no rule.
PASS sequence pin — list gains 105; docs 94; "imports none" holds (no import op); assertion message names 0105.
PASS matrix pin — planning-quick-read has no kind, so it is a constraint: 598 existing constraints give +598 pairs, 225,052 + 598 = 225,650 (validate prints 225650); suppressed 182 holds; clusters 0.
PASS SUPPRESSED-zero comment — raw no-allowlist runs read 76 clusters · 182 edges at both 1..104 and 1..105; the allowlisted kind-keyed-library-wide / neither-lane-waivable row exists (allowlist:436).
PASS house style — paragraphs follow the 0102–0104 precedent (dated build, migration, mint/reword tally, by-kind arithmetic, raw-run line); no production code touched.
VERDICT: PASS
Issues requiring fix: none
```

## Unit 4 — prose primitive: router `mochiko` `SKILL.md`

```
VALIDATE: unit 4 — plugins/mochiko/skills/mochiko/SKILL.md (patterns-sound-loop row) + .mochiko/strips/mochiko.md [v0.124.0] (prose primitive)
Checklist run: judgment-items-prose — coherence · preserved responsibilities
Evidence read: git diff 5691a67 of the router (one row, line 70); strip [v0.124.0] comment and entry; body measured 50,224 → 50,337 (canonical snippet, matches the strip comment)
Pre-pass: migrate validate 0 rejecting · 118 advisory (router carries no rules)
PASS coherence — the stakes clause now reads "a product run's harm class, or the class of the product work a spec, map or store change steers, ruled by the user at the command's confirmation point — implement's run-open line —", which agrees with the reworded stakes-key and the stakes note (DF5) and with the skill's own description.
PASS preserved responsibilities — strip entry holds the superseded clause verbatim, cites D1 items 3 and 8, D2 and DF5, keeps the product-run wording and the rest of the row, and assesses consumers; the line-70 pointer resolves.
VERDICT: PASS
Issues requiring fix: none
```

## Unit 5 — contract freeze

```
VALIDATE: unit 5 — evals/contract/expected-skills.json + evals/contract/README.md (contract freeze)
Checklist run: frozen floor set and pin against the live render · README tally row coherent with the ruling
Evidence read: git diff 5691a67 of both files; patterns-sound-loop entry in expected-skills.json parsed; rendered preamble floors line
Pre-pass: migrate validate 0 rejecting · 118 advisory
PASS frozen set — the JSON floor list, sorted, equals the render's floors line (11 ids); floor_pin 11 matches `- class: floor · 11 rules`; the frozen baseline fields (schema_bytes 8996, baseline_bytes, body_bytes_pre) are untouched, which is correct for a freeze.
PASS README row — skills 270 → 271, with "→ 11 by the 2026-10-28 stakes-non-product-surfaces ruling (the planning quick read)" appended to the sound-loop chain; commands column unchanged at 130.
VERDICT: PASS
Issues requiring fix: none
```

The deterministic contract suite (Docker sandbox) was not run here. It stays a release gate at the bump.

## Notes of note

- Ledger hash discrepancy, explained, not this wave's: `0104`'s header hash is now `7cd24f21…`, but `implement-lead-adaptive-rigor/reports/audit-w3.md` read `7bfc3eac…` in round 1, when status printed `94ff194e…`. The [v0.123.0] row quotes that round-1 state. The round-2 fix of `0104` moved it to `84802fd7…`, which this seat reproduces at 1..104. A copy of the live tree hashes the same as the repo (`e30d0f11…`), so the hash does not depend on the path.

## Outcome lines

- `audit: unit 1 — 0105-sound-loop-planning-stakes.yaml + view diff (schema content) · w1-gate-grader · opus · 2 files · 1 rounds · 0 blocking`
- `audit: unit 2 — patterns-sound-loop SKILL.md + render (skill pair) · w1-gate-grader · opus · 3 files · 1 rounds · 1 blocking`
- `audit: unit 3 — crate tests (fidelity.rs, matrix_similar.rs, validate.rs · crate non-author review) · w1-gate-grader · opus · 3 files · 1 rounds · 0 blocking`
- `audit: unit 4 — mochiko router SKILL.md (prose primitive) · w1-gate-grader · opus · 2 files · 1 rounds · 0 blocking`
- `audit: unit 5 — expected-skills.json + contract README (contract freeze) · w1-gate-grader · opus · 2 files · 1 rounds · 0 blocking`

## Unit 2 — round 2 (re-audit of the fix, same seat resumed)

```
VALIDATE: unit 2 — patterns-sound-loop skill pair, round 2 (the one fix round of common.gate-loop-bound): the fix to the argued overage only
Checklist run: the failed item (argued overage), plus what the fix could have broken — the rest of the budget row, the strip [v0.124.0] entry, and the measurement
Evidence read: .mochiko/memory/primitive-cost-budgets.md patterns-sound-loop row (whole head; tail compared programmatically against 5691a67); .mochiko/strips/patterns-sound-loop.md [v0.124.0] entry (whole)
Pre-pass: migrate validate `0 rejecting · 118 advisory`; migrate status `state sha256:e30d0f115ded… · 94 documents · 1284 rules`; canonical snippet + seven render blocks re-run: body 3,406 · desc 823 · render 19,415 (preamble 2,788 · trigger 5,850 · scope 698 · discipline 6,690 · inputs 423 · disclosure 2,037 · reserved 929) · payload 22,821 — unchanged from round 1
PASS argued overage — the row's head now reads "five rewords (`kind-keyed-library-wide`, `stakes-key`, `stakes-band`, `review-form-invariants`, D1 items 3–6; and `disclosure-line`, its `stakes: n/a` case narrowed as a consequence of D1 item 3, +99 to `sec.disclosure`, 1,938 → 2,037)". The argued set now matches 0105's five reword-rule ops and the measured section delta. Figures 22,821 / +10,433 / +3,579 / +3,488 / +91 and description 746 → 823 all reproduce.
PASS nothing broken — the row from "Prior: standing **+6,854" onward is byte-identical to 5691a67; the strip entry's additions sentence now lists `disclosure-line` among the reworded rules, and its disposition, content, kept-deliberately and consumers bullets are unchanged; the diff stat on the two files is unchanged in shape (budgets 1 line changed, strip one entry added).
Round-1 PASS items (scaffold, preserved responsibilities, floor survival, independence, reserved-to-user, done-condition branch) stand: the fix touched no primitive file, and SKILL.md and the render measure as in round 1.
VERDICT: PASS
Issues requiring fix: none
```

- `audit: unit 2 — patterns-sound-loop SKILL.md + render (skill pair) · w1-gate-grader · opus · 3 files · 2 rounds · 0 blocking`
