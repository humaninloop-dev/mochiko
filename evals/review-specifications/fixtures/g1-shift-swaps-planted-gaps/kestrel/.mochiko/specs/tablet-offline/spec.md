# Tablet offline clock-in

> Spec: tablet-offline
> Created: 2026-08-18
> Status: in-flight — accepted 2026-08-24, FEAT-004 row `live`

---

## Intent

- **Scope boundary:** the venue tablet keeps clock-ins and clock-outs made while the venue's
  network is down and reconciles them when it returns.
- **Delivery:** whole feature in one run.
- **Depth / rigor:** high — hours feed pay.
- **UX-bearing:** no — the existing clock-in screen gains a status line only.
- **Constraints:** the tablet is a shared device; no per-user login on it.
- **Out of scope:** offline rota edits by a manager; the staff phone app working offline.

---

## Overview

Half our venues lose wifi during shows. A clock-in made then must not be lost or duplicated, and
the manager dashboard must show it within a minute of the network returning.

---

## User Stories

| ID | Story (one breath) | Priority | Feature | Disposition |
|----|--------------------|----------|---------|-------------|
| [US-1](stories/US-1.md) | Clock in on the tablet when the wifi is down and have it count | P1 | FEAT-004 | homed |

---

## Edge Cases

- Network returns mid-sync → each clock-in is applied once, never twice.
- Tablet restarted while offline → queued clock-ins survive the restart.

---

## Functional Requirements

- **FR-001** The tablet MUST accept a clock-in or clock-out with no network and show it as
  "waiting to sync". *Source: US-1*
- **FR-002** Queued clock-ins MUST reach the manager dashboard within one minute of the network
  returning, each applied exactly once. *Source: US-1*

---

## Success Criteria

- **SC-001** Zero duplicated or lost clock-ins across the pilot venues over four weeks, measured
  by comparing the tablet queue log with the `clock_event` table.

---

## Screens & Flows

No UX surface — prototype waived at intent.

---

## Feature Selection

### Derived features

| FEAT-ID | Feature | New / delta | Stories | SCs verified |
|---------|---------|-------------|---------|--------------|
| FEAT-004 | Clock-in | delta on in-flight — offline clock-in with later reconciliation | US-1 | SC-001 |

### Selection

- **Selected (build now, dependency order):** FEAT-004 delta
- **Deferred (`proposed` on the map):** none
- **Deferred SCs:** none
