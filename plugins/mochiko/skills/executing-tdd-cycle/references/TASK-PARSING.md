# Cycle-Card Reading

Rules for reading the current cycle card from `tasks.md`. The card carries *what the cycle
must prove*; the concrete tasks are yours to decompose at build time (SKILL.md step 2) —
nothing task-level is parsed from the file.

## Card Pattern

```markdown
### - [ ] Cycle {N}: {title} *({Foundation|Feature})* `[P]`?

- **Stories:** US-# — rationale
- **Depends on:** — | C{M}
- **Case:** Simple | Split — why | Merge — why
- **Acceptance criteria:** spec/plan IDs
- **Brownfield exposure:** none | extends `path` | modifies `path`

**TEST:** {gate title}
- **Setup**: ...
- **Action**: ...
- **Assert**: ...
```

### Fields to Extract

| Field | Use |
|-------|-----|
| Checkbox | `- [ ]` pending / `- [x]` complete — on the `### Cycle` heading line |
| Type + `[P]` | Foundation cards run sequentially, first; `[P]` marks parallel-eligible feature cards |
| Stories | The `US-#` set this card serves — resolve against `spec.md` for the independent tests |
| Depends on | Cards that must be complete before this one starts |
| Acceptance criteria | Cited IDs — resolve against the spec/plan artifacts; these bound the decomposition |
| Brownfield exposure | The existing surfaces the decomposition must classify extend/modify (read those files first; `brownfield-integration` alongside) |
| `**TEST:**` block | The verifier's gate — parse only to know what the cycle must ultimately prove; running it is `testing-end-user`'s work |

## Current-Cycle Identification

The current cycle is the **first card in file order whose checkbox is unchecked**, with all
its `Depends on` cards checked. Flip the card's checkbox (`- [ ]` → `- [x]`) at step 6 of the
execution sequence.

## Quality Gates Pattern

Quality gates appear in a dedicated section, not on cards:

```markdown
## Quality Gates
- `pnpm lint` passes with zero errors
- `pnpm build` completes successfully
- `pnpm test` all tests pass
```

Quality gates — and the `**TEST:**` gate on each card — are executed by the verifier
(`testing-end-user`), not by this skill. Read them only to know they exist.
