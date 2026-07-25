# Skill-Succinctness Strip — Session Record

**Opened:** 2026-07-25 · **Concluded:** 2026-07-25 · **Status:** accepted (batch R1–R7 ratified)
· bare session (direct `analysis-iterative` invocation)
**Goal (user's words):** ensure the agent skills are not verbose and really succinct; question each
skill one by one; aim to reduce size by 30–70%; follow the strip documentation that already exists.
**Provenance:** lead-authored as-you-go; facts lead-gathered (no fact-checker seat); cold-reviewed
2026-07-25 by a solo `devils-advocate` (user-sized at D5; `review-brainstorm` solo protocol) with
lead verification of every reported citation. Review section below.

---

## Facts gathered at open (lead; corrected post-review; measured 2026-07-25 at plugin v0.23.0)

Baseline freezes at this stamp; each wave re-measures at proposal time (fold M12).

- **Skill surface:** 27 skills under `plugins/mochiko/skills/`. SKILL.md bodies total **5,521
  lines** (largest: patterns-vertical-tdd 372, patterns-api-contracts 365, patterns-entity-modeling
  363, authoring-constitution 354, analysis-codebase 313 — reviewer-verified exact). Non-SKILL.md
  surface under `skills/` totals **11,309 lines** (fold I8; the originally recorded 7,947 silently
  excluded 3,362): references/*.md 7,672 + analysis-iterative's three sibling .md 275 +
  `OPENAPI-TEMPLATE.yaml` 879 + six validation/detection scripts 2,483. `plugins/mochiko/templates/`
  (1,588 md lines) is a separate skill-pointed surface.
- **Description delivery is NOT "always fully loaded" (fold C1 — the original claim was wrong).**
  Measured against this session's own delivered skill listing: long `description:` strings are
  **truncated in delivery** — `authoring-constitution` (~1.5k chars) arrives complete;
  `review-brainstorm` and `review-governance-intent` (~1.8k) arrive cut mid-sentence with a trailing
  ellipsis — and `validation-constitution` renders with **no description at all** (cause
  undiagnosed; reproducible). Consequences: trigger phrases past the truncation boundary are
  *already dead*; `validation-constitution` is a **live absent-fire instance predating any strip**;
  cutting already-truncated tails saves zero delivered tokens (D4's sham-cut category).
- **Strip doctrine exists:** `.mochiko/strips/README.md` — per-primitive strip notes, newest-first,
  version-stamped. Tiered criterion: **Tier 1** (altitude — restated doctrine/pattern → relocate to
  single-sourced home) / **Tier 2** (no behavior/failure named → delete). Entry types: strip,
  survivor-provenance (KEPT with Tier-2 evidence), re-add (evidence-gated, override marked). Strip
  notes never under `plugins/`.
- **Prior waves already ran:** 13 skills have strip notes (authoring-constitution, -requirements,
  -slices, -technical-requirements, -user-stories, executing-tdd-cycle, patterns-api-contracts,
  -entity-modeling, -vertical-tdd, review-plan-artifacts, -slices, -specifications,
  testing-end-user); **14 do not** (~2,300 never-stripped body lines). KEPT (survivor-provenance)
  entries exist on only **3 skills** (authoring-slices, review-plan-artifacts, review-slices) —
  fold I7 input.
- **Division of labor (ruled, pattern-codification-and-minimalism):** architect first-passes every
  line (Tier-1 proposals + drafted Tier-2 rationale per contested line), **user ratifies contested
  lines**, an independent grader (never the author) audits the wave. **≥3-consumer shared
  primitives are ruled at a scheduled all-consumer escalation, never inside one cluster** (fold
  S7 of that record). An escalation queue from the prior waves sits open and un-ruled at
  `BACKLOG.md:382` — item 4 is the letter/spirit aphorism duplicated in **11 skill bodies** plus
  its canonical `loop-discipline` home (verified: 12 files).

## Tension on the table

The existing doctrine is **criterion-driven** (a line leaves because it fails a tier, survives
because evidence backs it). The user's 30–70% goal is **quota-shaped**. These can compose (quota as
burden-of-proof inversion + calibration bar) or conflict (quota forces cuts past Tier-2 evidence,
including prior KEPT entries). D1 puts this to the user first.

---

## Decisions

### D1 — Target semantics: calibration bar, criterion rules · **Confident** *(streak-adopted,
re-affirmed post-review — batch R7 ratified 2026-07-25)*
30–70% is the expected outcome and a tripwire, not a quota. Tiers apply line-by-line with the
burden of proof on each line. A skill landing under 30% triggers a second, harder pass in which
prior survivor-provenance (KEPT) evidence is re-examined for staleness (post-waves-1/2 reality) —
but no line is cut while its evidence stands. No quota-override strips.
**Rejected:** hard per-skill quota (forces cuts past standing Tier-2 evidence; re-adds return
through dogfood; override clusters are themselves audit hunt signals) · aggregate library target
(big skills carry the number while already-stripped skills coast — reads the per-skill instruction
out of the goal).
**Review folds:** I7 exposed the tripwire's second pass as undefined for the 24 skills with no
KEPT entries, and the uniform band as blind to prior-wave status → amendment R3.

### D2 — Scope: all three surfaces, each under its own rule · **Confident** *(streak-adopted,
re-affirmed post-review — batch R7 ratified 2026-07-25)*
SKILL.md bodies: main strip target, tier criterion per D1. Frontmatter descriptions: dedicated
sub-pass under a **trigger-fidelity criterion**. References: stripped only where the body pointer
dies or content duplicates a single-sourced home. Agent personas out of scope (already strip-noted
in prior waves).
**Rejected:** bodies-only (leaves the always-loaded description surface untouched) ·
bodies+descriptions (leaves the reference surface unexamined).
**Review folds:** (C1) the sub-pass's founding fact was wrong — descriptions truncate in delivery
and one never renders; the sub-pass is re-scoped **measure-first** → amendment R1. (I6) the watched
regression class was under-fire only; **wrong-fire added** — disambiguating boundary clauses
("use X instead", "does NOT cover Y") are *negatively-graded triggers*, protected alongside
positive trigger phrases; cut only genuine elaboration. (I8) scripts / `.yaml` / `templates/` were
unscoped → amendment R5.

### D3 — Execution: wave production, per-skill ratification · **Confident** *(streak-adopted,
re-affirmed post-review — batch R7 ratified 2026-07-25)*
Architect agents fan out per cluster, one strip proposal per skill. Lead walks the user through
proposals one skill at a time (proposed strips, contested lines with drafted Tier-2 rationale,
projected reduction %); user rules per skill; strips land per skill; one independent audit
(author≠grader) + version bump per wave. Per-skill strip notes and version stamps throughout.
**Rejected:** fully sequential end-to-end per skill (27 rounds of full ceremony, per-skill audits)
· one combined ceremony (batch ratification reads "one by one" out of the instruction).
**Review folds:** (M11) the "largest-first" label was inverted (misc cluster is largest at 1,860
lines yet ran last) — **reordered by never-stripped mass**, where the honest wins live:
analysis/testing/validation/misc → review-\* → patterns-\* → authoring-\*. (I5) the all-consumer
guard now has a mechanism, not just a name: a **pre-wave measurement pass enumerates the
≥3-consumer set by citation count**, explicitly including cross-skill reference files
(`TEST-GRAMMAR.md` owned by patterns-vertical-tdd / consumed by testing-end-user;
`CROSS-EXAM.md` shared by the two session-review skills; authoring-constitution's
`ESSENTIAL-FLOOR.md` / `INTERROGATION-AGENDA.md`); shared primitives are ruled at a scheduled
all-consumer escalation, never inside one cluster's wave. The open `BACKLOG.md:382` queue →
amendment R4b.

### D4 — Accounting: true reductions only · **Confident** *(streak-adopted, re-affirmed
post-review — batch R7 ratified 2026-07-25)*
Reduction credit = deletes + body→reference moves where the reference is genuinely conditional
(the strip note must name the invocation path that skips it — audit-checkable). Always-read
content moved to a reference is a sham cut, forbidden — and `templates/` is named a forbidden
relocation destination (templates are read at authoring time, i.e. always-read; fold I8).
**Rejected:** any-body-shrink-counts (sham cuts inflate the number while runtime cost rises) ·
deletes-only (forfeits legitimate progressive-disclosure wins on the 360+-line skills).
**Review folds:** (I9) the cross-sibling-duplication sentence was ambiguous between "flagging
earns no credit" and "deduping earns no credit," and routed the library's largest
mechanically-certain Tier-1 target (severity table ×5, verdict vocabulary ×7, aphorism ×11) to a
queue proven not to drain. Disambiguated: **deduping** into a genuinely conditional single-sourced
home earns credit like any Tier-1 relocation; mere flagging earns nothing → credit + destination
ruling in amendment R4a. (I4) the percentage had no denominator → amendment R2.

### D5 — Close shape: cold review → conclude → defer execution · **Confident** *(genuine read at
the no-recommendation fork, breaking the streak; elaborated ruling)*
The record takes a cold review before anything executes; the session concludes with full
bookkeeping (ROADMAP Key Decisions row, BACKLOG build item carrying the wave plan); waves execute
in later sessions. Review sizing — lead call, overridable: solo cold reviewer + lead verification
of cited claims, proportionate to a five-decision bare-session record (precedent:
domain-dependency-allowlist's user-sized solo review).
**The fork as posed (steelmanned, no recommendation):** (A) conclude-first — doctrine separated
from execution, reviewable record, honest about multi-session scale; (B) launch wave 1
immediately — a live wave is the cheapest stress-test of D1–D4, flaws surface at the first skill;
(C) cold-review first — doctrine applied 27× deserves adversarial pressure while cheap. User
ruled C then A, rejecting B: untested doctrine should not drive cuts across the framework's
quality surface. The review's `critical-gaps` verdict vindicated the ruling.
**Review folds:** (I10) the discarded pilot road is worth its steelman → amendment R6.

---

## Review (2026-07-25)

**Protocol:** solo cold reviewer (`devils-advocate`, `review-brainstorm` solo protocol — both
lenses + verify duties), user-sized at D5. Independent cold read; scenario stress per decision;
five hunt classes; reality-grounding of every load-bearing Facts claim against the files (no
fact-checker map existed); standalone-record fitness checklist. Lead then verified every reported
citation against the repo before folding (per stress-test-verify protocol).

**Tally:** reviewer formed 18 → reported **13** (3 Critical / 7 Important / 3 Minor; 5 dropped on
self-scrutiny, retrievable). Lead verification: **13/13 confirmed** — one figure adjusted
(`templates/` = 1,588 md lines, not 1,822; substance unchanged). Reviewer's recommended status:
**`critical-gaps`**, on (a) the broken C1 fact under D2, (b) two fitness items unchecked (C2
confidence marks, C3 rejected roads — zero-of-five), (c) a 2-in-6 miss rate in the lead-authored
Facts section (C1 wrong, I8 materially incomplete; three bullets verified exact).

**Dispositions:**

| # | Finding (compressed) | Disposition |
|---|---|---|
| C1 | "Always-loaded descriptions" false — delivery truncates ~1.8k-char descriptions; `validation-constitution` renders none (live absent-fire) | **Folded** — Facts corrected; D2 sub-pass re-scoped measure-first (R1) |
| C2 | D1–D4 marked `Confident` despite being unelaborated adoptions — fitness checklist requires `Assumed` | **Folded** — marks downgraded inline with streak counts; re-affirmation = R7 |
| C3 | Zero rejected roads recorded across D1–D5 | **Folded** — rejected roads + D5's fork options added to every decision |
| I4 | 30–70% has no denominator | **Amendment R2** |
| I5 | All-consumer guard: no mechanism, misses cross-cluster reference homes, ignores open BACKLOG:382 queue (aphorism ×11 verified) | **Folded** into D3 (pre-wave enumeration, reference files under guard); queue ruling = R4b |
| I6 | D2 protected only against under-fire; "restated boundaries" are negative triggers whose cut yields wrong-fire | **Folded** into D2 — wrong-fire watched, disambiguators protected |
| I7 | Under-30% second pass vacuous for 24 no-KEPT skills; uniform band blind to prior-wave status | **Amendment R3** |
| I8 | Supporting total silently excluded 3,362 lines (scripts, OpenAPI yaml); `templates/` unscoped and a sham-cut loophole as destination | **Folded** — Facts corrected, `templates/` banned as destination (D4); scope ruling = R5 |
| I9 | D4's dedup sentence ambiguous; biggest Tier-1 target routed to a queue that doesn't drain | **Folded** — disambiguated in D4; credit + destination = R4a |
| I10 | Plan locks untested process into BACKLOG; one-skill pilot steelman | **Amendment R6** |
| M11 | "Largest-first" ordering inverted (misc 1,860 runs last) | **Folded** — reordered by never-stripped mass (D3) |
| M12 | No version/date stamp on measured facts | **Folded** — stamped v0.23.0 / 2026-07-25; per-wave re-measure |
| M13 | No open-threads section | **Folded** — section added below |

**Clearing verdict (lead, 2026-07-25): ready — record accepted.** All 13 findings dispositioned —
folded into the record and/or routed into the ratified batch; none dismissed. The reviewer's
`critical-gaps` discharged by the C1/C2/C3 folds plus batch ratification.

## Amendment batch — ratified 2026-07-25

User ratified all seven as recommended, after asking for and receiving per-item lead
recommendations (a genuine engagement, not a streak adoption). Operative notes from the
ratification: R3's band numbers are **provisional until the R6 pilot calibrates them**; R6's pilot
skill is **`analysis-codebase`** (never-stripped, 313 lines, setup-cluster blast radius,
unentangled with the description anomaly); R7 discharged — D1–D4 marks upgraded to `Confident`
*(re-affirmed post-review)*.

- **R1 (C1):** description sub-pass becomes **measure-first** — first act of execution dumps the
  delivered listing, diffs all 27 descriptions against their files, records the truncation
  boundary and the `validation-constitution` anomaly; the sub-pass is then scoped as repair /
  reorder-triggers-ahead-of-boundary / cut, on evidence.
- **R2 (I4):** the 30–70% denominator = **per-skill SKILL.md body lines**; description and
  reference reductions tracked on separate ledgers, contributing no headline credit.
- **R3 (I7):** on a no-KEPT skill, the under-band second pass = a **survivor-provenance audit**
  (every kept line gets its Tier-2 evidence named in the strip note, or becomes contested — the
  pass *generates* KEPT entries rather than re-examining absent ones). Expected band splits by
  prior status: never-stripped 30–70%; previously-stripped 10–40%, tripwire at the lower bound.
- **R4 (I9/I5):** (a) cross-skill dedup into a genuinely conditional single-sourced home earns
  Tier-1 relocation credit; (b) the open `BACKLOG.md:382` escalation queue is ruled **at wave-1
  open** — starting with the letter/spirit aphorism (11 copies + canonical home): keep-and-log or
  strip-to-reference, decided library-wide, never per-cluster.
- **R5 (I8):** the six scripts (2,483 lines), `OPENAPI-TEMPLATE.yaml` (879), and
  `plugins/mochiko/templates/` are **out of scope** this pass (prose tier tests don't map to
  executable code; templates carry fresh wave-2 design). Recorded with revisit trigger below.
- **R6 (I10):** execution opens with a **one-skill pilot** — the full loop (propose → ratify →
  strip → audit → measure) on a single skill, followed by a D1–D4 confirm-or-revise checkpoint,
  before the wave plan is treated as settled.
- **R7 (C2):** user re-affirms D1–D4 explicitly (ratifying this batch with them in view); marks
  upgrade to `Confident` *(re-affirmed post-review)* on ratification.

## Open threads / revisit triggers (fold M13)

- **`validation-constitution` renders no description** — live discoverability defect predating
  this session; diagnose during R1's measurement pass. Revisit D2's sub-pass scope on findings.
- **Truncation boundary undiagnosed** (~1.5k chars complete, ~1.8k cut) — exact limit and
  mechanism unknown; R1 measures. If truncation is version-dependent, re-open D2.
- **Scripts / yaml / templates excluded (R5)** — revisit trigger: any wave whose skill's body
  pointers make a script or template load-bearing for the reduction story.
- **BACKLOG:382 queue items 1–3** (devils-advocate ×2, slices-template) — ruled at wave-1 open
  per R4b alongside item 4.
- **Baseline drift** — facts frozen at v0.23.0; any intervening build that touches skill bodies
  re-measures before its wave's percentages are computed.
