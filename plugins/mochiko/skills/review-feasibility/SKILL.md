---
name: review-feasibility
description: This skill MUST be invoked to grade design-phase analysis/design artifacts for cross-artifact FEASIBILITY — hunting contradictions, impossibilities, buildability conflicts, plus unjustified structure / wrong altitude; plus the architecture pass when the design-phase package carries an architecture-store delta. Emits a 3-state `feasible / needs-revision / infeasible` verdict. The adversarial half of the design-phase review pair; its sibling `review-plan-artifacts` grades coverage/measurability/presence, this grades contradiction/buildability. Never defaults to `feasible`; not the constitution.
---

# Reviewing Feasibility

Adversarial cross-artifact review: **can these artifacts be built together?** Hunt the
impossible combination no single artifact reveals — judgment, never a checklist; looking
buildable is not being buildable. The sibling
`mochiko:review-plan-artifacts` owns coverage / measurability / consistency / presence; you own
contradiction / impossibility / buildability. Never author or fix what you grade.

**Load [references/FEASIBILITY-LENS.md](references/FEASIBILITY-LENS.md) before hunting** —
classes 1–6, class 7 (excess / wrong altitude, remove-shaped, blocking-capable), the
architecture pass A1–A3 (mandatory on a drafted store delta), worked examples, the sibling
boundary, and the reviewer guardrails.

Floors — non-waivable:

- **Never default to `feasible`** — earned only by a completed hunt; absence of looking is not
  evidence. Hunt coverage discloses as **one line per class** in the report, never a narrative.
- **`infeasible` never flattens** into `needs-revision` — a fundamental conflict (no revision
  closes it; a business-level decision) escalates to the human.
- **Governance is never silently approved** — two exits only: redesign to conform, or a
  user-ruled amendment/waiver via `governance-ledger.md`.
- **Verdict + per-finding dispositions land in the reviewed artifacts and the filled report** —
  `templates/feasibility-report-template.md` under the `templates/report-format.md` envelope;
  evidence living only in conversation is a floor violation.
- **Findings cite the IDs in tension** (`C-003 ↔ D-007`) with the four gate-fuel fields;
  external premises verify per `../review-brainstorm/references/EXTERNAL-CLAIMS.md`.
- **Your verdict is input** — the lead owns clearing, loops, and the human gate; G1:
  design-phase artifacts only, never the constitution.

Verdict: `feasible` · `needs-revision` (resolvable) · `infeasible` (fundamental).
