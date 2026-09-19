# FEAT-001 — Monitor checks

> Status: delivered  <!-- proposed | in-flight | delivered | retired -->
> since 2026-08-02 · sticky — live rows may still be visible below
> reconstructed-from-code (2026-08-02) — first touching spec re-verifies extent

## Capability

Beacon probes each monitored endpoint on its schedule and opens an incident when the
probe fails twice in a row; a passing probe resolves the incident's failing state.

## Extent

- HTTP and TCP probes; interval 30 s to 10 min per monitor.
- Two consecutive failures open an incident; one pass marks it recovering.
- Not: synthetic multi-step checks.
- Not: probes from more than one region.

## Relations

- composes-with: FEAT-002 — every probe transition is a timeline event.
