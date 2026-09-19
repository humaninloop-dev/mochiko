# Feature Specification — Webhook subscriptions (FEAT-013) — excerpt

- **Status:** design phase · **Author:** requirements-analyst seat · **Date:** 2026-09-08

## User Stories (in scope)

### US-001 — Subscribe to delivery events (P1)

As a shop integrator, I want to register an endpoint and receive every delivery-status change
for my shipments, so that my order page updates without polling.

- **Given** a registered endpoint, **when** a shipment's status changes, **then** the endpoint
  receives one signed request carrying the event within a minute.

### US-002 — See what was delivered (P2)

As a shop integrator, I want to see the deliveries made to my endpoint over the last seven
days with their outcome, so that I can debug my receiver.

## Functional Requirements

- **FR-001** A shop MUST be able to register up to five endpoints, each with a signing secret
  shown once at creation. *Source: US-001*
- **FR-002** The system MUST deliver every status change to every active endpoint of the
  shipment's shop. *Source: US-001*
- **FR-003** A failed delivery MUST be retried with backoff for 24 hours before it is marked
  dead. *Source: US-001*
- **FR-004** A shop MUST be able to list its deliveries of the last seven days with attempt
  count, last response code, and outcome. *Source: US-002*

## Success Criteria

- **SC-001** 95% of deliveries reach the endpoint within 60 s of the status change, measured
  from `occurred_at` to the first 2xx, over one week.
