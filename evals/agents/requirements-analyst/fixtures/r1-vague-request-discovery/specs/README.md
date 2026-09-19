# Spec house format

Every feature spec lives at `specs/<feature>/spec.md` and carries these sections in this order.
`specs/expense-submission/spec.md` is the reference example.

1. Header — feature name, status, author, date.
2. Overview — two or three paragraphs: who this is for and what changes for them.
3. User Stories — `US-XXX`, each with a priority (P1/P2/P3), Given/When/Then scenarios, and an
   independent test.
4. Functional Requirements — `FR-XXX`, each with a `Source:` line naming the story it serves.
5. Success Criteria — `SC-XXX`.
6. Assumptions — `A-XXX`, decisions the author took where the input did not say.
7. Open Questions — anything the author could not decide alone, with who owns the answer.
8. Out of Scope — what this spec deliberately leaves out and why.
