# Grading bar — feature specification `spec.md`

Grade `spec.md` against every item below. Each item is PASS or FAIL with one line of evidence
from the file. The verdict is PASS only when every item passes.

1. **Header block.** The file opens with a header block carrying `Feature`, `Status`, `Author`,
   and `Date`.
2. **Overview.** An `## Overview` section names the user, the problem, and the value in prose
   (no bullets, no requirements language).
3. **Required sections.** All six sections are present under these exact headings:
   `## Overview` · `## User Stories` · `## Functional Requirements` · `## Edge Cases` ·
   `## Success Criteria` · `## Out of Scope`.
4. **User stories.** Every user story carries a priority tag `(P1)`, `(P2)`, or `(P3)` in its
   heading and at least one Given/When/Then scenario.
5. **Requirement grammar.** Every functional requirement is numbered `FR-NNN`, numbered
   contiguously from FR-001, and uses at least one RFC 2119 keyword (MUST, MUST NOT, SHOULD,
   SHOULD NOT, MAY).
6. **Measurable success criteria.** Every success criterion `SC-NNN` is measurable: it names a
   metric, a threshold, and the instrument or data source that measures it.
7. **No placeholders.** `spec.md` contains none of the tokens `TBD`, `TODO`, `FIXME`, `???`, or
   `[NEEDS CLARIFICATION]`.
8. **Traceability.** Every functional requirement carries a `Source: US-NNN` pointer to a user
   story that exists in the file.
9. **Out of scope with reasons.** The `## Out of Scope` section lists at least one exclusion and
   gives a reason for each.
