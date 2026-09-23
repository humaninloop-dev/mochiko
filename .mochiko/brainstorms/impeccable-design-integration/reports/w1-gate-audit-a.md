---
report: review
round: 2
seat: gate-grader-A
tier: opus
wave: impeccable-design-integration wave 1 (plugin 0.114.0, working tree over HEAD 6e4b264)
contract: mochiko-cli rules validation-primitive-edit (binary 0.2.0 · grammar 1 · plugin 0.114.0), pasted verbatim in the brief
split: seat A of two (A and B) — the wave's units would not fit one seat's context
units: 13 of the wave's units on seat A
verdict: "round 1: 11 PASS · 2 FAIL · 3 blocking — round 2: 15 of 15 PASS (13 units + 0016 + 0017) · 0 blocking"
prepass: |
  $ mochiko-cli migrate validate --report --plugin-root plugins/mochiko
  mochiko-cli migrate validate · 0 rejecting · 112 advisory
  pointer resolution: 84 checked against plugins/mochiko
  similar-rule clusters (threshold 0.60): none — no pair clears the threshold
  advisories on seat-A units (non-gating):
    cite-foreign · skill/patterns-craft-floor · citations with a foreign prefix: patterns-code-minimalism.lazy-not-negligent
    condition-coverage · skill/review-design-audit · ux_bearing · value "false" is declared but named by no rule's `when:`
    condition-coverage · skill/testing-gap-finding · ux_bearing · value "false" is declared but named by no rule's `when:`
    condition-coverage · skill/authoring-prototype · design_system · value "absent" is declared but named by no rule's `when:`
    zero-member-label · command/specify · attempt-economy / stewardship (registry-legal)
  $ mochiko-cli views emit --out <scratch> --plugin-root plugins/mochiko ; diff -r <scratch> .mochiko/schema-views
  mochiko-cli views emit · 80 documents · VIEWS-IDENTICAL (derived view equals replay)
budget: |
  Canonical snippet (.mochiko/memory/primitive-cost-budgets.md "How to measure"); render = the
  seven `mochiko-cli rules <skill> --section …` blocks, characters of stdout, summed.
  patterns-design-direction: body 7027 + render 9489 = 16516 vs 16,516 (birth seed) · desc 784 vs 784
  patterns-craft-floor:      body 6992 + render 7274 = 14266 vs 14,266 (birth seed) · desc 853 vs 853
  review-design-audit:       body 4510 + render 7534 = 12044 vs 12,044 (birth seed) · desc 573 vs 573
  authoring-prototype:       body 4908 + render 11259 = 16167 vs 14,980 → +1,187 argued · desc 565 vs 617
  testing-gap-finding:       body 6970 + render 15852 = 22822 vs 19,977 → +2,845 argued · desc 770 (hard cap 1,536)
  testing-end-user:          body 9596 + render 12353 = 21949 vs 21,710 → +239 argued · desc 500 vs 625
  patterns-vertical-tdd:     body 5908 + render 10027 = 15935 (standing +160, unchanged this wave) · desc 497 vs 620
  patterns-model-tiering:    body 3113 + render 15234 = 18347 (standing +7,495, unchanged this wave) · desc 1209 (hard cap)
  product-designer (agent):  desc 449 vs 490
  Body deltas vs HEAD: authoring-prototype +288 · testing-gap-finding +1,004 · testing-end-user 0.
unit_1: |
  VALIDATE: prose primitive (kitted persona) · product-designer · plugins/mochiko/agents/product-designer.md
  Checklist run: judgment-items-prose (coherence · preserved responsibilities); advisory grid read (not run, reason in strip)
  Evidence read: plugins/mochiko/agents/product-designer.md (full) · git diff HEAD -M (product-engineer.md → product-designer.md) · .mochiko/strips/product-designer.md v0.114.0 entry + grid note · record D3/D4/D11
  Pre-pass: migrate validate 0 rejecting (above) · agent desc 449 ≤ 490
  Coherence: PASS — the four D3 judgments land as identity (brief wins; refinement vs redesign; read what shipped before calling greenfield; mode per surface); no command or stage names, no workflow trace; no accessibility content (D11); skills line mounts patterns-design-direction.
  Preserved responsibilities: PASS — every line that left or changed (name, description, skills, opener, "engineer who has", the design-system bullet re-homed to authoring-prototype.design-system-honored, "Grading your own prototype", the Embrace line) is recorded verbatim as a supersession-by-ruling citing D3/D4; kept set named; consumers assessed.
  Advisory grid: skipped by S5 with a quoted runner error (pre arm cannot read a pre-rename path); informs only, never gates.
  VERDICT: PASS
  Issues requiring fix: none
unit_2: |
  VALIDATE: skill pair · patterns-design-direction · SKILL.md + references/ios.md + references/android.md + render (7 blocks)
  Checklist run: judgment-items-pair
  Evidence read: plugins/mochiko/skills/patterns-design-direction/SKILL.md (full) · references/ios.md, android.md (headers, a11y grep) · all seven rendered blocks · 0011 import-document · `mochiko-cli home .mochiko/product/design` and `…/design/design.md` run first-hand
  Pre-pass: 0 rejecting; payload 16,516 = seed; desc 784 = seed, ≤ 1,536
  Scaffold headings and order: PASS — load-first section with seven `!` lines in preamble order, patterns six-set, delivery-failure halt clause, floor read-back citing the pin and `floors:` line.
  Preserved responsibilities: PASS — birth skill; carries D3 modes + three laws, D9 onboard/clarify/adapt folds, Direction block of build item 2, platform routing (D6), attribution line (D14 iii).
  Floor survival: PASS — three floors minted with D3/D7/D11 anchors; none exit.
  Independence: PASS — prototype/craft routed out; description states never grades own output.
  Reserved content: PASS — replace-reserved in sec.reserved.
  Done-condition branch: n/a — skill pair.
  Argued overage: n/a — at birth seed.
  Body binding: FAIL — Related line 123 tells the model to run `mochiko-cli home .mochiko/product/design`; run first-hand it resolves to the parent `product` home and prints "`design` is NOT a declared deliverable of this home" (the directory form resolves to the parent, as build-log.md itself records). The rule text binds `.mochiko/product/design/design.md`; the body sends readers to the wrong home contract.
  VERDICT: FAIL
  Issues requiring fix:
    1. Body binding · SKILL.md:123 names a home command that resolves the wrong home · change it to `mochiko-cli home .mochiko/product/design/design.md` (the form 0014's setup rule uses).
unit_3: |
  VALIDATE: skill pair · patterns-craft-floor · SKILL.md + render (7 blocks)
  Checklist run: judgment-items-pair
  Evidence read: plugins/mochiko/skills/patterns-craft-floor/SKILL.md (full) · seven rendered blocks · 0011 import-document · plugins/mochiko/agents/staff-engineer.md diff (skills: line)
  Pre-pass: 0 rejecting (cite-foreign advisory on the lazy-not-negligent citation, non-gating; pointer resolves); payload 14,266 = seed; desc 853 = seed
  Scaffold headings and order: PASS — load-first section, seven `!` lines, read-back sentence.
  Preserved responsibilities: PASS — bans, refuse list, verify list, typeset/layout/colorize/animate/delight folds, register dial (D9), contract read and baseline tokens (D8 i), bounded verify (F5), mounted on staff-engineer (D3).
  Floor survival: PASS — code-floor-in-view (D11) and decides-nothing (D3, the R11 argument) minted and anchored.
  Independence: PASS — grading-routing sends grading to the critique and design-audit lenses.
  Reserved content: PASS — empty with a stated note routing gaps to the Direction block.
  Done-condition branch: n/a. Argued overage: n/a (seed).
  VERDICT: PASS
  Issues requiring fix: none
unit_4: |
  VALIDATE: skill pair · authoring-prototype · SKILL.md + render (7 blocks)
  Checklist run: judgment-items-pair
  Evidence read: plugins/mochiko/skills/authoring-prototype/SKILL.md (diff + Structure/Related) · preamble, inputs, artifact blocks · view diff skills/authoring-prototype.yaml · budget ledger row · HEAD body measured (4,620)
  Pre-pass: 0 rejecting; payload 16,167 vs 14,980 (+1,187); desc 565 ≤ 617
  Scaffold headings and order: PASS — unchanged.
  Preserved responsibilities: PASS — two rewords keep ids and anchors; design-system-honored keeps every prior obligation and adds the baseline-first order (build item 7); R21 boundary in the description.
  Floor survival: PASS — five floors unchanged.
  Independence: PASS — direction-routing minted: renders, never picks the Direction block.
  Reserved content: PASS — unchanged.
  Done-condition branch: n/a.
  Argued overage: FAIL — the render limb (+703) holds as a genuine new obligation, but the ledger names the body +288 as "the step-2 baseline-first read, the Direction-block rendering step and the R21 boundary clause". The measured body delta is two Related lines only; the step-2 read and Direction rendering are render rewords, and the R21 clause sits in the frontmatter description, outside the body. The argument misattributes its own component.
  Body binding: FAIL — Related line 73 carries the same `mochiko-cli home .mochiko/product/design` command, which resolves to the parent `product` home.
  VERDICT: FAIL
  Issues requiring fix:
    1. Body binding · SKILL.md:73 wrong home command · use `mochiko-cli home .mochiko/product/design/design.md`.
    2. Argued overage · .mochiko/memory/primitive-cost-budgets.md authoring-prototype row misnames the +288 body component · reword to "body +288: two Related pointer lines (patterns-design-direction upstream; the design baseline home)", and re-measure if fix 1 changes the body length.
unit_5: |
  VALIDATE: skill pair · review-design-audit · SKILL.md + render (7 blocks)
  Checklist run: judgment-items-pair
  Evidence read: plugins/mochiko/skills/review-design-audit/SKILL.md (full) · seven rendered blocks · 0012 import-document · plugins/mochiko/agents/qa-engineer.md diff
  Pre-pass: 0 rejecting; payload 12,044 = seed; desc 573 = seed
  Scaffold headings and order: PASS — review six-set, load-first section, read-back.
  Preserved responsibilities: PASS — V1 finding-originating legs (perf/optimize, harden, polish, slop tells) with D10's slop list; advisory by form (D8, D13 both depths); a11y and TEST legs routed to testing-end-user (D11); Drift routing to the landing fold (D7); mounted on qa-engineer.
  Floor survival: PASS — author-grader (extends review-common) and advisory-never-failing (D8).
  Independence: PASS — seat-qa-engineer excludes the builder and the blind seat.
  Reserved content: PASS — verdict-is-input and disposition-at-checkpoint in sec.reserved.
  Done-condition branch: n/a. Argued overage: n/a (seed).
  VERDICT: PASS
  Issues requiring fix: none
unit_6: |
  VALIDATE: skill pair · testing-gap-finding · SKILL.md + render + strip v0.114.0
  Checklist run: judgment-items-pair
  Evidence read: SKILL.md diff vs HEAD · view diff skills/testing-gap-finding.yaml via 0012 changes · .mochiko/strips/testing-gap-finding.md v0.114.0 entry · record D8/D11/D13
  Pre-pass: 0 rejecting; payload 22,822 vs 19,977 (+2,845); body +1,004 re-measured against HEAD; desc 770 ≤ 1,536
  Scaffold headings and order: PASS — unchanged load-first block.
  Preserved responsibilities: PASS — the removed "Accessibility probing — declined" bullet is a recorded supersession citing D11, its kept half ("the a11y floor stays a build-time standard") verbatim in a11y-verification-routing; fence floor reworded with id, class, anchor kept.
  Floor survival: PASS — blindness-fence-inclusion-list reworded, not exited; the design baseline admission is promised behavior, so black-box holds.
  Independence: PASS — the critique rides the same blind seat, no code sight.
  Reserved content: PASS — fix-confirm bound goes to the checkpoint.
  Done-condition branch: n/a.
  Argued overage: PASS — body +1,004 and render +2,066 sum to the +3,070 delta; a whole second lens is a genuine new obligation.
  VERDICT: PASS
  Issues requiring fix: none
unit_7: |
  VALIDATE: skill pair · patterns-vertical-tdd · SKILL.md + references/TEST-GRAMMAR.md + render
  Checklist run: judgment-items-pair
  Evidence read: git diff HEAD on the skill dir (SKILL.md unchanged; TEST-GRAMMAR.md +22 lines) · render unchanged (no view diff for this skill)
  Pre-pass: 0 rejecting; payload 15,935 unchanged (standing +160); references/ exempt
  Scaffold, floors, independence, reserved: PASS — untouched.
  Preserved responsibilities: PASS — pure addition (no strip owed); D10's measurable rules (contrast, targets, tiny tap text, line length, heading skip, responsive at Contract viewports) and the a11y standard-of-record case by pointer; R17 not-a-toolchain line; attribution line.
  Done-condition branch: n/a. Argued overage: n/a.
  VERDICT: PASS
  Issues requiring fix: none
unit_8: |
  VALIDATE: skill pair · testing-end-user · SKILL.md + render
  Checklist run: judgment-items-pair
  Evidence read: SKILL.md unchanged vs HEAD · sec.output render (measured-assert-evidence) · 0012 mint-rule
  Pre-pass: 0 rejecting; payload 21,949 vs 21,710 (+239); desc 500 ≤ 625
  Preserved responsibilities: PASS — one mint, boundary untouched (classification unchanged, custom assertion for human evaluation), as build item 4 names.
  Floor survival, independence, reserved: PASS — untouched.
  Argued overage: PASS — +443 render, the minted duty, a genuine new obligation (D10).
  VERDICT: PASS
  Issues requiring fix: none
unit_9: |
  VALIDATE: skill pair · patterns-model-tiering · SKILL.md + render
  Checklist run: judgment-items-pair
  Evidence read: SKILL.md unchanged vs HEAD · view diff skills/patterns-model-tiering.yaml (one token) · 0015
  Pre-pass: 0 rejecting; payload 18,347 unchanged (same-length re-key); desc 1,209 ≤ 1,536
  Floor survival: PASS — seat-default-key floor reworded in place, id/class/anchor kept, roster count unchanged.
  Preserved responsibilities: PASS — only product-engineer → product-designer moves; no stale name in any view.
  VERDICT: PASS
  Issues requiring fix: none
unit_10: |
  VALIDATE: command pair · specify · plugins/mochiko/commands/specify.md + rendered rules
  Checklist run: judgment-items-pair (command)
  Evidence read: specify.md (unchanged vs HEAD; Goal lines 44–64) · preamble render · view diff commands/specify.yaml
  Pre-pass: 0 rejecting; commands unbudgeted
  Scaffold headings and order: PASS — unchanged.
  Preserved responsibilities / FAIL survival: PASS — spec.prototype-craft and spec.fail.screens-flows reworded with ids, anchors, and `enforces:` kept; the fail rule widens to a missing Direction block.
  Floor survival, independence, reserved: PASS — untouched.
  Done-condition branch: PASS — run-command fixed contract; the Goal binds "spec.md conforming to the spec template", which now carries the Direction block.
  VERDICT: PASS
  Issues requiring fix: none
unit_11: |
  VALIDATE: schema content · 0011-design-direction-craft-floor · migration + view diff (patterns-design-direction, patterns-craft-floor, authoring-prototype, templates/spec, commands/specify)
  Checklist run: judgment-items-schema (AM-2 five)
  Evidence read: plugins/mochiko/migrations/0011-design-direction-craft-floor.yaml (full) · git diff HEAD on the five views · new views untracked, equal to replay
  Pre-pass: 0 rejecting; views equal replay
  Intent stated: PASS. Anchor present: PASS — header D3; floors and rewords of anchored rules carry D1/D3/D6/D7/D8/D11.
  ID lifecycle: PASS — two imports, one mint, four rewords keeping ids, one condition note; the template replace changes only the Direction block, its check, and max_lines 30 → 37.
  Floor and fail survival: PASS — no exits; spec.fail.screens-flows keeps kind and enforces.
  Register: PASS — rule register matches the family.
  VERDICT: PASS
  Issues requiring fix: none
unit_12: |
  VALIDATE: schema content · 0012-design-verification-lenses · migration + view diff (review-design-audit, testing-gap-finding, testing-end-user)
  Checklist run: judgment-items-schema (AM-2 five)
  Evidence read: plugins/mochiko/migrations/0012-design-verification-lenses.yaml (full) · rendered blocks of all three skills
  Pre-pass: 0 rejecting; views equal replay
  Intent stated: PASS. Anchor present: PASS — header D8; mints carry D3/D7/D8/D10/D11/D13.
  ID lifecycle: PASS — one import, six mints, three rewords keeping ids, one set-condition.
  Floor and fail survival: PASS — the reworded fence floor keeps its id; no exits.
  Register: PASS.
  VERDICT: PASS
  Issues requiring fix: none
unit_13: |
  VALIDATE: schema content · 0015-product-designer-rekey · migration + view diff (patterns-model-tiering)
  Checklist run: judgment-items-schema (AM-2 five)
  Evidence read: plugins/mochiko/migrations/0015-product-designer-rekey.yaml (full) · view diff (one token)
  Pre-pass: 0 rejecting; views equal replay
  Intent stated: PASS. Anchor present: PASS — D4. ID lifecycle: PASS — reword keeps the id.
  Floor and fail survival: PASS — floor text unchanged but the one name. Register: PASS.
  VERDICT: PASS
  Issues requiring fix: none
round2_prepass: |
  $ mochiko-cli migrate validate --report --plugin-root plugins/mochiko
  mochiko-cli migrate validate · 0 rejecting · 112 advisory
  $ mochiko-cli views emit --out <scratch> ; diff -r <scratch> .mochiko/schema-views
  mochiko-cli views emit · 80 documents · VIEWS-IDENTICAL
  Budgets (canonical snippet + summed seven-block render):
  patterns-design-direction: body 7137 + render 9691 = 16828 = ledger 16,828 (re-seed at birth, pre-ship, disclosed; under the §0 abort line 19,719) · desc 784
  authoring-prototype:       body 4918 + render 11283 = 16201 vs 14,980 → +1,221 = ledger · desc 565 ≤ 617
  testing-gap-finding:       body 6970 + render 15936 = 22906 vs 19,977 → +2,929 = ledger · desc 770
  unchanged by the delta: patterns-craft-floor 14,266 · review-design-audit 12,044
round2_unit_2: |
  VALIDATE: skill pair · patterns-design-direction · round 2 delta (SKILL.md Overview + Direction Block section + Related line; 0016 rewords of fires-before-screens, mode-per-surface, direction-block)
  Evidence read: SKILL.md lines 17–20 and 98–124 · 0016 (full) · rendered trigger/discipline/disclosure via the views diff · `mochiko-cli home .mochiko/product/design/design.md` run first-hand (resolves `product-design`, `design.md` a declared deliverable)
  Pre-pass: 0 rejecting; payload 16,828 = ledger
  Body binding: PASS — line 124 now names the file form, which reaches the right home.
  Coherence (round-1 advisory folded): PASS — body and three rules now agree: one block per surface, headed with surface and SCR range; mode per surface named in that surface's block.
  Floor survival / ID continuity: PASS — rewords keep ids; the three floors untouched.
  Budget: PASS — the birth seed is re-measured before ship and disclosed; no overage path owed yet.
  VERDICT: PASS
  Issues requiring fix: none
round2_unit_4: |
  VALIDATE: skill pair · authoring-prototype · round 2 delta (SKILL.md:73; 0016 rewords of two-coupled-artifacts and design-system-honored; ledger row)
  Evidence read: SKILL.md:73 · 0016 · primitive-cost-budgets.md authoring-prototype row · HEAD body 4,620 re-used from round 1
  Pre-pass: 0 rejecting; payload 16,201; body delta 4,918 − 4,620 = +298 (matches the row)
  Body binding: PASS — file-form home command.
  Argued overage: PASS — the row now names the body +298 as the two Related lines and places step-2 and the Direction rendering in render (+727 = scope 310 + artifact 290 + preamble 127); R21 is said to be in the description. Components sum to +1,025 and the overage to +1,221; a genuine new obligation.
  Preserved responsibilities: PASS — the 0016 rewords only change "the Direction block" to per-surface; ids, anchors, and `when:` kept.
  VERDICT: PASS
  Issues requiring fix: none
round2_unit_6: |
  VALIDATE: skill pair · testing-gap-finding · round 2 delta (description "or" fix; 0017 reword; ledger row)
  Evidence read: SKILL.md description line · 0017 (full) · ledger row
  Pre-pass: 0 rejecting; payload 22,906 = ledger; render +84 matches 0017's reword; desc 770 (same length)
  Coherence (round-1 advisory folded): PASS — the low-depth rule now says outright that it overrides the blocking half of critique-finding-split (D13).
  Argued overage: PASS — components sum to +3,154 and +2,929 standing.
  VERDICT: PASS
  Issues requiring fix: none
round2_unit_10: |
  VALIDATE: command pair · specify · round 2 delta (0016 rewords of spec.prototype-craft and spec.fail.screens-flows)
  Evidence read: 0016 · views diff equal to replay
  FAIL survival: PASS — spec.fail.screens-flows keeps id, kind, and `enforces:`; it now fails on any surface that lacks its block.
  Done-condition branch: PASS — the Goal's template-conformance clause carries the per-surface block.
  VERDICT: PASS
  Issues requiring fix: none
round2_unit_14: |
  VALIDATE: schema content · 0016-direction-per-surface · migration + view diff (patterns-design-direction, authoring-prototype, command/specify, templates/spec)
  Checklist run: judgment-items-schema (AM-2 five)
  Evidence read: plugins/mochiko/migrations/0016-direction-per-surface.yaml (full) · views equal replay
  Intent stated: PASS. Anchor present: PASS — D5 (mode and contract per surface).
  ID lifecycle: PASS — seven rewords keeping ids; the template replace changes only the Direction heading, the per-surface contract/check, and max_lines 37 → 44.
  Floor and fail survival: PASS — no exits; spec.fail.screens-flows keeps kind and enforces.
  Register: PASS.
  VERDICT: PASS
  Issues requiring fix: none
round2_unit_15: |
  VALIDATE: schema content · 0017-critique-depth-precedence · migration + view diff (testing-gap-finding)
  Checklist run: judgment-items-schema (AM-2 five)
  Evidence read: plugins/mochiko/migrations/0017-critique-depth-precedence.yaml (full) · views equal replay
  Intent stated: PASS. Anchor present: PASS — D13. ID lifecycle: PASS — one reword, id and `when: {depth: low, ux_bearing: yes}` kept.
  Floor and fail survival: PASS — must-class rule, no exit. Register: PASS.
  VERDICT: PASS
  Issues requiring fix: none
outcome_lines:
  - "audit: product-designer · gate-grader-A (seat A of two, split for context) · opus · 2 files · 1 rounds · 0 blocking"
  - "audit: patterns-design-direction · gate-grader-A (seat A of two, split for context) · opus · 5 files · 2 rounds · 1 blocking"
  - "audit: patterns-craft-floor · gate-grader-A (seat A of two, split for context) · opus · 2 files · 1 rounds · 0 blocking"
  - "audit: authoring-prototype · gate-grader-A (seat A of two, split for context) · opus · 4 files · 2 rounds · 2 blocking"
  - "audit: review-design-audit · gate-grader-A (seat A of two, split for context) · opus · 2 files · 1 rounds · 0 blocking"
  - "audit: testing-gap-finding · gate-grader-A (seat A of two, split for context) · opus · 4 files · 2 rounds · 0 blocking"
  - "audit: patterns-vertical-tdd · gate-grader-A (seat A of two, split for context) · opus · 3 files · 1 rounds · 0 blocking"
  - "audit: testing-end-user · gate-grader-A (seat A of two, split for context) · opus · 2 files · 1 rounds · 0 blocking"
  - "audit: patterns-model-tiering · gate-grader-A (seat A of two, split for context) · opus · 2 files · 1 rounds · 0 blocking"
  - "audit: specify · gate-grader-A (seat A of two, split for context) · opus · 3 files · 2 rounds · 0 blocking"
  - "audit: 0011-design-direction-craft-floor · gate-grader-A (seat A of two, split for context) · opus · 6 files · 1 rounds · 0 blocking"
  - "audit: 0012-design-verification-lenses · gate-grader-A (seat A of two, split for context) · opus · 4 files · 1 rounds · 0 blocking"
  - "audit: 0015-product-designer-rekey · gate-grader-A (seat A of two, split for context) · opus · 2 files · 1 rounds · 0 blocking"
  - "audit: 0016-direction-per-surface · gate-grader-A (seat A of two, split for context) · opus · 5 files · 1 rounds · 0 blocking"
  - "audit: 0017-critique-depth-precedence · gate-grader-A (seat A of two, split for context) · opus · 2 files · 1 rounds · 0 blocking"
---

## Failure narrative

Round 1: units 2 and 4 named `mochiko-cli home .mochiko/product/design`, which resolves to the
parent `product` home ("`design` is NOT a declared deliverable"); only the file form reaches
`product-design`. Unit 4's ledger row also named the body +288 as the step-2 read, Direction
rendering, and R21; the measured body delta was two Related lines.

Round 2 cleared all three: both commands use the file form, and the ledger row names the two
Related lines. The round-1 advisories were folded in by 0016 and 0017.

## Notes of note

Advisory only, not gating. Screens & Flows' max_lines of 44 fits two surfaces; a third
Direction block overflows it (0016's intent says "fit two surfaces"). Watch this at the first
three-surface spec.
review-design-audit and the critique-lens rule still say "the Direction block" in the
singular. They read naturally per surface, but a later reword could make that explicit.
specify.md's Goal line 50 still leaves the Direction block out of its list; template
conformance covers it. patterns-vertical-tdd's description measures 497 against a ledger 496,
which predates this wave.
