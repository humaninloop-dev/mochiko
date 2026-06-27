# Responsibility-trace tags

A transformation is honest only if **every responsibility the primitive currently holds is accounted for** after the port — none silently lost. List each responsibility, tag where it lands. `verify-output` audits this; the trace doubles as the `REGISTRY.md` migration record.

| Tag | Meaning | Typical trigger |
|-----|---------|-----------------|
| `kept` | Stays in this primitive, unchanged | the responsibility is core and clean |
| `kept-but-rebind` | Stays, but a path/grounding/reference changes in place | `.humaninloop/` → `.mochiko/`, catalog path removal |
| `folded-into-skill` | Moves out of an agent/command into a (new or existing) skill | procedure that was baked into a persona |
| `moved-to-lead` | Re-homed onto the command supervisor / lead-referee | loop-driving, sequencing, verdict ownership, human gate |
| `moved-to-validator` | Re-homed onto a **peer** validator agent (not the lead) | a self-grade check extracted from the producer |
| `moved-to-sibling-skill` | Absorbed by a sibling during a merge | the unique slice of a thin variant |
| `moved-to-other-cluster` | Belongs to a different (peer) workflow | a feeder consumed mainly by `plan`, not `setup` |
| `dedupe` | Legitimately duplicated across producer/validator — collapse to one | shared substrate mounted twice |
| `dropped + reason` | Intentionally removed; **reason required**, lead must accept | kernel/DAG/catalog plumbing that mochiko sheds |

## Rules

1. **No bare `dropped`.** Every drop carries a reason, and the lead/human gate must accept it. A drop with no accepted reason fails `verify-output`.
2. **No silent moves.** If a responsibility leaves the primitive, the tag names exactly where it went so the receiving primitive can be checked too.
3. **Relational tags imply a reconcile decision.** `moved-to-validator`, `moved-to-sibling-skill`, `dedupe` come *out of* `reconcile-cluster`, not solo assessment — assess flags them; reconcile assigns them.
4. **The trace must be complete before assessment is "done."** A primitive with an untagged responsibility is not assessed.

## Example (from the Round-1 dry-run)

```
TRACE: principal-architect
  - greenfield constitution authoring        → kept
  - constitution quality grading             → moved-to-validator (new constitution-validator agent)
  - cross-artifact feasibility review         → folded-into-skill (orphaned procedure → new skill)
  - brownfield analysis orchestration         → moved-to-lead (supervisor sequences analysis-codebase)
  - duplicated procedure prose in persona     → dedupe (lives in the skill, not the agent)
```
