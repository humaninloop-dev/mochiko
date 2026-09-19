---
report: review
round: 1
unit: review-seat-plan pair
form: old — mochiko:validator seat, fresh, full read, default FAIL
seat: validator
tier: opus
---

## VALIDATE

VALIDATE: the `review-seat-plan` pair — `plugins/mochiko/skills/review-seat-plan/SKILL.md`
plus its render (`mochiko-cli rules review-seat-plan --section <id> --plugin-root
plugins/mochiko`) for `preamble` and each of the six sections the preamble lists.

Checklist run: the skill-pair block, items 1–12, of `.claude/rules/mochiko/primitive-edits.md`,
read whole, plus the ruling-execution checks the brief adds.

## Evidence read

Evidence read: the SKILL.md · all seven rendered blocks · the `import-document` in
`plugins/mochiko/migrations/0009-plan-qa-leg.yaml` · `producer-plan-enforcement/record.md`
D2–D6 · `author-grader-consolidation/wave2-plan-qa.md` §3.1 item 6 · the budget ledger's
first-seed row and canonical snippet · `evals/contract/expected-skills.json` · the router row
in `plugins/mochiko/skills/mochiko/SKILL.md` · the `DECISIONS.md` 2026-09-03 row. No
`reports/` file of this wave was opened.

## Pre-pass — migrate validate

Pre-pass: `mochiko-cli migrate validate --report --plugin-root plugins/mochiko`, run first-hand;
last line quoted:

> `mochiko-cli migrate validate · 0 rejecting · 106 advisory`

The unit's only advisory line: `budget · skill/review-seat-plan · - · 15 rules · 4339 resolved
characters of rule text` — rule text only, never the ledger comparison.

## Pre-pass — char budget

Measured with the ledger's canonical snippet, plus the seven rendered blocks:

> body 3,534 + render 9,167 = payload 12,701 — preamble 1,623 · independence 1,531 · scope
> 1,034 · inputs 1,080 · verdict 2,309 · output 894 · reserved 696. Frontmatter `description:`
> parsed value 839.

Ledger first-seed row: 12,701 (no headroom), description 839. Exact match, every per-section
figure included. Delivery cap 1,536: 839 is under it.

## Items 1–4

1. **Load-first section** — PASS. `## Rules — delivered by mochiko-cli` heading, seven `!` lines,
   `allowed-tools: Bash(mochiko-cli *)` granted, no-raw-Read clause present, no reference read owed.
2. **Section enumeration** — PASS. The six `--section` arguments behind the preamble line match the
   preamble's printed set and order — the review six-set; no stray section token anywhere.
3. **Floor pin + read-back** — PASS. The render prints `- class: floor · 5 rules` and a five-id
   `floors:` line; the body obliges both back and hard-codes no number.
4. **Floor survival** — PASS, vacuous: born this wave, no floor left. `protected-exit` and
   `anchor-format` clean.

## Items 5–8

5. **ID continuity** — PASS. Every id minted once by `0009`; no tombstone and no supersession in the
   import, so no disposition to read. `mint-once` / `id-duplicate` / `cite-unresolved` clean.
6. **`extends:` conformance** — PASS. Five stubs, all `review-common.*` of its own family, each
   declaring `class:` locally and overriding no inherited text.
7. **`description:` cap** — PASS at 839 characters of the parsed value; born this wave, so the item
   reads as the 1,536 cap alone.
8. **Budget** — PASS. Payload 12,701 against a 12,701 first-seed row, all seven per-section figures
   identical; no overage, so no argument owed.

## Items 9–12

9. **Pointer resolution** — PASS. The skill carries no `pointer:`; the pre-pass reports `pointer
   resolution: 84 checked against plugins/mochiko`, 0 rejecting.
10. **Deterministic pre-pass** — PASS. Run by me and quoted above; nothing it asserts is re-derived
    by judgment here.
11. **Skill-grammar conformance** — PASS. Kinds are `binding` · `routing` · `duty` · `reservation`
    plus the omitted `constraint` default; no `kind: fail`, no `enforces:`, no `moments:`, no `when:`.
12. **Provenance anchors** — PASS. Six rule-level `2026-09-03 producer-plan-enforcement` anchors
    (D2–D6) plus the header's D8, all resolving to the 2026-09-03 `DECISIONS.md` row.

## Ruling checks — D2 to D6

- **D5 seven criteria verbatim** — 5 of 7 exact; items 4 and 7 reworded, both obligations intact.
  Finding 1; non-blocking for the reasons recorded there.
- **D3 fresh peer** — met. The floor rule carries the same-persona-type spawn, the never-the-author
  and never-the-lead fence, the persona-less fallback, and the devils-advocate exclusion.
- **D4 two-way delivery** — met: both channels, and the reason the criteria cannot ride a persona's
  preloaded skills list. Finding 2 concerns only its pre-install clause.
- **D6 referenced, never restated** — met. The bound, its shared counter and the escalation go to the
  dispatching command; no dirty-tree mechanic and no disclosure grammar restated. Finding 3.

## Ruling checks — ids, router, suite limbs

- **§3.1 item 6 fixed ids** — all fifteen match, section for section and class for class. One label
  set diverges (finding 4).
- **Router row** — present under a "Seat-plan QA" heading in `skills/mochiko/SKILL.md`, and accurate:
  seven items with 1–6 blocking, the fresh peer, the resumed re-grade, the `PLAN GRADE:` grammar.
- **Suite limbs** — confirmed by hand: the load-first enumeration against the render, and the frozen
  floor ids and pin 5 against `evals/contract/expected-skills.json`. The sandboxed suite is the
  release gate and was not run here.

## VERDICT

VERDICT: **PASS.** Items 1 through 12 each confirmed against the pair itself, the pre-pass clean at
0 rejecting, and the payload landing exactly on its first-seed budget. The four findings are
non-blocking: none removes an obligation the ruling places on the grader, and none is a
protected-content exit.

Issues requiring fix: none blocking. Four non-blocking findings follow.

## Notes of note

1. **D5 items 4 and 7 reworded, not verbatim.** Item 4 drops the `plan-structure-yagni` D5 citation
   and restates the split as "presence blocking and the claim's honesty advisory"; item 7 moves
   ", advisory" into the head sentence. Both obligations survive, and the citation rides the rule's
   own anchor and the re-pointed `patterns-plan-minimalism.grading-routing`.
2. **"Reads the repo pair by path" is under-specified.** The body forbids reading a schema file and
   declares no fallback, and none ships; naming the render command there would close the reading
   under which a grader hunts a file that cannot exist.
3. **One escalation overlap.** `same-grader-regrades` says the escalation is never restated here,
   while `approval-is-the-leads` enumerates the user's three options — both as §3.1 pinned them.
4. **One label divergence.** `.fresh-peer-grader` was pinned with two labels and built with one;
   labels bind nothing on their own, so the cost is a cross-reference.

## Outcome

audit: review-seat-plan pair (old form, double-grade) · validator · opus · 3 files · 1 rounds · 0 blocking
