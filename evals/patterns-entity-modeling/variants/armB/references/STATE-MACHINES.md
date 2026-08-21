# State Machine Modeling

Reference documentation for modeling entity state machines and status transitions.

## When to Model State

Model state machines when:
- Entity has a `status` or `state` field
- Requirements mention workflow or lifecycle
- Specific actions change entity state
- Certain actions only valid in certain states

## State Machine Format

```markdown
## State Machine: Task Status

### States

| State | Description | Entry Condition |
|-------|-------------|-----------------|
| `draft` | Initial state | Created by user |
| `active` | Work in progress | User starts task |
| `completed` | Work finished | User marks done |
| `archived` | No longer active | User archives |

### Transitions

| From | To | Trigger | Guard | Side Effects |
|------|-----|---------|-------|--------------|
| draft | active | user.startTask() | - | Set startedAt |
| active | completed | user.completeTask() | - | Set completedAt |
| active | draft | user.unpublish() | User is owner | Clear startedAt |
| completed | archived | user.archive() | - | - |
| * | archived | admin.archive() | Is admin | Log action |

### Diagram

```
[draft] ──start──▶ [active] ──complete──▶ [completed]
   │                  │                        │
   │                  ▼                        │
   └──────────▶ [archived] ◀───archive─────────┘
```
```

## Validation Checklist

- [ ] All states from requirements are documented
- [ ] Every state has a description
- [ ] All valid transitions are documented
- [ ] Guards specify who can perform transition
- [ ] Side effects are listed for complex transitions
- [ ] Diagram matches transitions table
- [ ] Initial state is clearly marked
- [ ] Terminal states (if any) are identified
