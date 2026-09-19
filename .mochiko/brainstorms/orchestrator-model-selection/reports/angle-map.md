# Phase 0 Blind Angle Map — orchestrator-model-selection

**Seat:** cold end-stage reviewer (`mochiko:review-brainstorm`, solo, decision-quality + record-integrity).
**Built:** 2026-09-19, from the topic and goal line only. The record was not opened, listed, or
searched before this file was written; `.mochiko/brainstorms/orchestrator-model-selection/` was
fenced for the whole of Phase 0. This file is frozen as built — nothing was edited after record contact.

**Topic as given:** un-pin `model: opus` from every mochiko persona (all ten agent files pin it
today) and let the orchestrating lead select each seat's model — what changes in behaviour, and
what the impact is (cost, quality, independence, doctrine, delivery).

**Goal line as given:** one hardened decision record on who selects a seat's model tier, the
per-seat defaults, the bounds on the lead's choice, the evidence gate, and the build surface —
under mochiko's standing floors (GI-019 no kernel-class dispatch or gating; whatever is ruled must
resolve on both spawn transports, subagent and agent-team teammate; author≠grader; the
`patterns-model-tiering` floor and the `model-tiered-seats` D5 deferral it cites).

**Repo grounding used (session artifacts excluded):** `patterns-model-tiering` rendered floor set
(6 floors, binary 0.1.0 / grammar 1 / plugin 0.109.0); `.mochiko/brainstorms/model-tiered-seats/record.md`
D4/D5 + the F6 review fold; `.mochiko/decisions/2026-08-16-model-tiering-build.md`;
`.mochiko/decisions/2026-09-05-sonnet-worker-rung.md`; the 2026-08-19 explorer-retarget row; the ten
persona files under `plugins/mochiko/agents/` and their `## Delegating Cheap Reads` /
`## Delegating Bounded Work` standing sections; `evals/agents/` persona kits;
`BACKLOG.md` token-reduction epic; `ROADMAP.md`; `DECISIONS.md`; `CLAUDE.md` GI-019 / GI-020.

---

## A. Ownership of the choice

1. Whose pen: lead-selects vs user-ruled vs seat-declared — run decision or reserved decision.
2. Tension with `worker-seat-set-reserved`'s shape ("ruled, never run-decided") applied one rung up.
3. Granularity: per-run, per-seat, per-dispatch, per-round.
4. May a seat refuse or escalate a tier it judges too low for its brief.
5. Who selects with no lead — direct user invoke, nested seat-spawns-seat.
6. User veto, and at which visible gate.

## B. Per-seat defaults

7. Named default per each of the ten seats, not "lead decides".
8. Un-pinned means no `model:` key (inherit session) or a weaker pin — inherit is not chosen.
9. Producer vs grader asymmetry; author≠grader under a cheaper grader.
10. Double-dip: `staff-engineer`/`qa-engineer` already hold the sonnet worker rung.
11. Seats with no safe cheap analogue — `devils-advocate`, `validator`, `tech-lead`-as-grader.
12. Fact-checker seat: D5 seed named it both the cheapest drop and the fact substrate.
13. Tier vocabulary covered: Fable, Opus, Sonnet, Haiku — or only opus/sonnet.

## C. Bounds on the lead

14. Hard floor tier below which no seat goes; what enforces it under doctrine-only (GI-019).
15. Bound axis: work class (judgment vs execution) or seat identity.
16. Does the bound include the lead's own tier.
17. May a grader ever run below its producer.
18. `override-is-the-pin` survival: with the pin gone, silence means inherit, not cheap.
19. Tier drift across FAIL-loop rounds.

## D. Doctrine and floor conflicts

20. `rostered-seats-never-retier` is `class: floor` — un-pinning retiers; supersession recorded, not implied.
21. D5 deferral is a scope ruling with a re-pointed gate (F6); satisfied or reopened.
22. Is this session the "dedicated future brainstorm" D5 named, stated as such.
23. Blast radius: skill `description:`, router row, migration, contract-suite frozen `floor_ids`.
24. GI-019: no proposed mechanism gates or dispatches mechanically.
25. GI-020 / migration bump; nothing reads a schema file instead.
26. `patterns-sound-loop`: lead choosing both producer and grader tiers concentrates what the pair splits.
27. Independence doctrine is seat+skill separation — does tier become a fourth independence axis.

## E. Transport (both must resolve)

28. Subagent transport: Agent-tool `model` already overrides frontmatter — the pin is soft today; does the record know.
29. Teammate transport: can the lead set a teammate's model at spawn, or config/inherit only.
30. If teammate transport cannot, the ruling resolves on one transport only — floor breach.
31. Recurrence of the 2026-08-19 failure (teammates cannot spawn plugin-scoped agents).
32. `teammateDefaultModel` and `CLAUDE_CODE_SUBAGENT_MODEL` — both rejected in 2026-08-16; re-examined.
33. "Unpinned" behaviour specified per transport, since inherit semantics differ.
34. Platform version floor named (precedent: agent-teams ≥v2.1.224).
35. Live spawn probe run — both prior tiering rulings carried one; sidechain model ids as proof.

## F. Quality and independence

36. Plausible-shallow-PASS — D5's own named expensive-to-verify failure.
37. Silence findings and decision-driving absences are the cheap tier's weakest class.
38. Adversarial critique vs mirror-checklist grading split by tier.
39. Grade-shopping: the lead's tier choice leaks its expected verdict.
40. Rule-render discipline at lower tiers: floor read-back, halt-on-malformed, `!` line handling.
41. Context-window fit per seat versus cheap-tier windows.

## G. Cost

42. Savings quantified on D1's unit of account, or asserted.
43. D5 counter-fact: non-implement seats run ≤6 rounds, absolute savings small.
44. Subscription Opus-cap headroom is a different lever from per-token price.
45. Cost of a failed cheap grade: extra rounds, bad artifact landing, maintainer time — netted.
46. Does the existing dispatch floor already capture most savings, leaving seat retier marginal.

## H. Evidence gate

47. F6's gate is per-seat cheap-tier reliability evidence — instrument and bar named.
48. `evals/agents/` kits exist for all ten; are they the instrument.
49. Kit arms are pre/post persona-edit at fixed model — a model arm may not exist; harness change scoped.
50. Bar versus instrument resolution: noise band, pass^k, labeller calibration.
51. Ten seats equals ten grids — staged or blanket; what ships before evidence.
52. Any seat permitted to move before its own evidence lands.
53. Named falsifier that reverses the ruling.

## I. Build surface

54. Enumerated: ten agent files, skill rules, migration, router rows, `plugin.json`, contract suite.
55. Strip entry per edited primitive plus supersession-by-ruling for the floor text.
56. Contract-suite `floor_ids`/`floor_pin` re-freeze if any floor rule reworded (gate 6).
57. Persona body is the only channel reaching both transports — does selection doctrine need to reach the seat.
58. Seat self-discloses its own tier in its report.
59. Where the lead reads the default table from: skill, command, or persona.
60. Author≠grader audit per edit before the `plugin.json` bump; CHANGELOG, marketplace sync, replay gate.

## J. Landing ritual

61. DECISIONS row, BACKLOG trail move, ROADMAP touch, brainstorm-index status agreement.
62. Open BACKLOG line "D5 seat-tiering stays deferred (Later)" must move, not just be contradicted.
63. `patterns-model-tiering` stays single source; selection doctrine must not fork into commands.

## K. Failure and reversal

64. Reversal path: how a project pins back.
65. Observability: is a run's actual per-seat model recorded post-hoc.
66. Silent degradation: unpinned seat inherits a cheap session tier with nobody choosing.
67. Third-party installs: consumer session tier becomes mochiko's quality floor — intended.
68. Eval reproducibility: kits assume pinned models.

## L. Rejected roads

69. Steelman: keep the pin, widen the dispatch rungs instead.
70. Steelman: per-project governance depth dial selects tiers, not the lead.
71. Steelman: user selects, lead recommends — mochiko's recommend-then-arbitrate shape.
72. Partial un-pin: only oracle-bearing seats; judgment seats stay pinned.
73. Null option — do nothing, and its cost.
74. Does "all ten" framing force uniformity where per-seat rulings differ.
