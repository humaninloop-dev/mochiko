# Governance ledger — Saltmarsh

**Version:** v1.1.0 · **Ratified:** 2026-06-18 · **Fact profile:** production floor · depth high · modules: knowledge-management (core; electives declined)

## Amendment log

- v1.0.0 · 2026-04-20 · initial ratification (greenfield; no modules)
- v1.1.0 · 2026-06-18 · knowledge-management module adopted whole (core); `CHANGELOG.md` and `RUNBOOK.md` declined durable

## Waivers

None.

## Amendment policy

Fact-profile changes (module attach/detach) and un-waives run through `/mochiko:setup`; a
principle edit outside that run is a defect.

## Three-Part metadata

| GI | Surface | Trace |
|---|---|---|
| GI-001 | CLAUDE.md region | fact profile |
| GI-002 | CLAUDE.md region | depth level |
| GI-003 | CLAUDE.md region | payment data stays in Stripe |
| GI-004 | CLAUDE.md region | booking/hold/waitlist audit trail |
| GI-005 | CLAUDE.md region | school settings over platform defaults |
| GI-006 | CLAUDE.md region | stack |
| GI-007 | CLAUDE.md region | test + coverage gate |
| GI-008 | CLAUDE.md region + `.claude/rules/mochiko/operating-docs.md` | knowledge-management module |
