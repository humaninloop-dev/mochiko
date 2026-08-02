---
description: Produce an accepted feature specification — prioritized user stories, functional requirements, and measurable success criteria — from a feature description.
disable-model-invocation: true
---

# Specify

**Goal:** the feature described in `$ARGUMENTS` has an accepted specification: who it serves,
what problem it solves, prioritized user stories, functional requirements, and measurable
success criteria — precise enough that design and planning can build on it without going back
to the user for basics. Where this repo carries mochiko governance (a governance region in
`CLAUDE.md`), the specification conforms to it.

**You need:** a feature description clear on who / problem / value — from `$ARGUMENTS`, the
user, or the codebase.

**Done when:** the user has accepted the specification, it is stored durably —
`.mochiko/specs/<feature>/spec.md` is the convention later work looks for — and, where this
repo keeps mochiko operating docs, they reflect it.
