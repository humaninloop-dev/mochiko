# Menu admin

> Spec: menu-admin
> Created: 2026-08-25
> Status: in-flight — accepted 2026-08-29, FEAT-002 row `live`

---

## Intent

- **Scope boundary:** the kitchen hides a menu item from parents while keeping it on its own
  list, and un-hides it later.
- **Delivery:** whole feature in one run.
- **Depth / rigor:** high — a hidden item is a safety and stock decision.
- **UX-bearing:** no — a toggle on the existing kitchen menu screen.
- **Constraints:** none beyond the menu's existing shape.
- **Out of scope:** parents seeing hidden items in any form; per-child menus.

---

## Overview

A kitchen sometimes has to pull an item — a supplier recall, a short delivery — without losing
it from the menu for good. A hidden item never reaches a parent-facing screen or response.

---

## User Stories

| ID | Story (one breath) | Priority | Feature | Disposition |
|----|--------------------|----------|---------|-------------|
| [US-1](stories/US-1.md) | Hide an item from parents without deleting it | P1 | FEAT-002 | homed |

---

## Functional Requirements

- **FR-001** The kitchen MUST be able to mark a menu item hidden and later un-hide it.
  *Source: US-1*
- **FR-002** A hidden item MUST NOT appear in any parent-facing screen or response, including a
  week's menu a parent has already opened. *Source: US-1*

---

## Success Criteria

- **SC-001** Zero hidden items in any parent-facing response, measured by the contract suite on
  every deploy.

---

## Screens & Flows

No UX surface — prototype waived at intent.

---

## Feature Selection

### Derived features

| FEAT-ID | Feature | New / delta | Stories | SCs verified |
|---------|---------|-------------|---------|--------------|
| FEAT-002 | Menu publishing | delta on delivered — hide an item from parents | US-1 | SC-001 |

### Selection

- **Selected (build now, dependency order):** FEAT-002 delta
- **Deferred (`proposed` on the map):** none
- **Deferred SCs:** none
