---
description: Turn an accepted implementation plan into working, verified code — built test-first, verified against real infrastructure with captured evidence, and accepted by the user.
disable-model-invocation: true
---

# Implement

**Goal:** the feature named in `$ARGUMENTS` exists as working code: built test-first, verified
against real infrastructure with captured evidence and passing quality gates, and traceable to
its requirements. Where an approved architecture exists, the built system conforms to it — any
divergence accepted by the user, never silently built. Where this repo carries mochiko
governance (a governance region in `CLAUDE.md`), the code conforms to it.

**You need:** an implementation plan understood well enough to build from — a task breakdown,
design artifacts, or the plan reconstructed with the user; `.mochiko/specs/<feature>/` is the
convention where earlier work leaves these.

**Done when:** the user has accepted the implementation on the verification evidence, and,
where this repo keeps mochiko operating docs, they reflect what was built.
