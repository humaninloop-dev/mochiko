# Constraints and decisions — Beacon

## Hard constraints

- C-001: An incident never closes automatically while its probe fails (GI-003).
- C-002: Channel targets never reach a log line, a timeline event, or an error message (GI-004).
- C-003: Every outbound HTTP call times out at 10 seconds (CX-001).

## Decisions

- D-001: A Postgres table is the alert queue; no message broker (CX-002).
- D-002: One attempt per channel per incident event; a failure is recorded and left.

## Infrastructure

- IP-001: Single region; Postgres managed with a nightly snapshot.
