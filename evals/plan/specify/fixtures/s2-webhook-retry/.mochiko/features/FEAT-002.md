# FEAT-002 — Incident timeline

> Status: delivered  <!-- proposed | in-flight | delivered | retired -->
> since 2026-08-02 · sticky — live rows may still be visible below
> reconstructed-from-code (2026-08-02) — first touching spec re-verifies extent

## Capability

Every incident keeps an ordered, append-only record of what happened to it — probe
transitions, alerts sent, notes, and its close.

## Extent

- Event kinds today: opened, probe-failed, probe-passed, alert-sent, note, closed.
- Events are append-only; a note can be added by anyone on the team.
- Shown on the incident page newest-last.
- Not: editing or deleting an event.
- Not: exporting the timeline.

## Relations

- depends-on: FEAT-001 — an incident exists because a probe failed.
