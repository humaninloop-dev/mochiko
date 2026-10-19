# RFC 2119 Keywords in Constitution Principles

The keywords carry their RFC 2119 meanings. This file holds only how they bind in a principle.

## Keyword to enforcement

| Keyword | CI enforcement | Code review | Exception process |
|---------|----------------|-------------|-------------------|
| MUST / MUST NOT | Blocks merge | Verify compliance | Exception Registry entry required |
| SHOULD / SHOULD NOT | Warning only | Verify, or the deviation is justified | A justifying comment in the code is enough |
| MAY | None | Optional discussion | Not needed |

## Choosing the keyword

- **If you would reject a PR for breaking the rule, it is a MUST.** "Code SHOULD pass tests before
  merge" is wrong.
- **If valid exceptions exist, it is a SHOULD.** "Functions MUST be under 40 lines" is wrong;
  "Functions SHOULD be under 40 lines", with complex algorithms as the stated exception, is right.
- **Security requirements and prohibitions are MUST / MUST NOT.**
- **Every enforceable rule carries an explicit keyword.** "Code will be formatted before commit"
  is not a rule; "Code MUST be formatted before commit" is.
- **One keyword per requirement.** Never "SHOULD MUST".

## Example

```markdown
### III. Code Quality

All code MUST meet quality standards:

- Static analysis MUST pass with zero warnings
- Code formatting MUST comply with configured rules
- Functions SHOULD be under 40 lines (exceptions documented)
- Cyclomatic complexity SHOULD NOT exceed 10 per function
- Teams MAY adopt stricter rules for critical modules

**Enforcement**:
- `ruff check .` MUST exit with code 0 (CI automated)
- Functions over 40 lines SHOULD have a comment justifying the length
```
