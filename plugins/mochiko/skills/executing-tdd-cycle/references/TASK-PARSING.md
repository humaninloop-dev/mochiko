# Cycle-Card Reading

Rules for reading the current cycle card from `tasks.md`. The card carries *what the cycle
must prove* — a bundle of named test cases (the card's content); the concrete tasks are yours to
decompose at build time (SKILL.md step 2) — nothing task-level is parsed from the file.

## Card Pattern

```markdown
### - [ ] Cycle {N}: {title} `[P]`?

- **Stories:** US-# — rationale
- **Depends on:** — | C{M}
- **Case:** Simple | Split — why | Merge — why
- **Brownfield exposure:** none | extends `path` | modifies `path`

**TEST:** {case title}
- **Covers**: spec/design IDs this case covers
- **Setup**: ...
- **Action**: ...
- **Assert**: ...
```

A card may carry more than one `**TEST:**` block — together they are the cycle's test-case
bundle. Cycle 1 of a new end-to-end path is a **walking skeleton** (one trivial case green);
growth on an already-standing path has no skeleton cycle.

### Fields to Extract

| Field | Use |
|-------|-----|
| Checkbox | `- [ ]` pending / `- [x]` complete — on the `### Cycle` heading line |
| `[P]` | Marks a parallel-eligible card — derived from dependencies, not a card type |
| Stories | The `US-#` set this card serves — resolve against `spec.md` for the independent tests |
| Depends on | Cards that must be complete before this one starts |
| Brownfield exposure | The existing surfaces the decomposition must classify extend/modify (read those files first; `brownfield-integration` alongside) |
| `**TEST:**` blocks | The card's named test-case bundle — the expected behaviour the cycle must ultimately demonstrate green; these bound the decomposition. Each block's `Covers` line cites the spec/design IDs it verifies. Parse only to know what the cycle must prove; running the cases is `testing-end-user`'s work |

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

Quality gates — and the `**TEST:**` cases on each card — are executed by the verifier
(`testing-end-user`), not by this skill. Read them only to know they exist.
