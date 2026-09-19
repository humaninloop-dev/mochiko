# DoorLink cloud API — what we use (summary, 2026-09-01)

- **Unlock**: `POST https://api.doorlink.example/v1/doors/{door_id}/unlock` with a per-venue
  API key. Returns `200 { "unlocked": true }` only after the on-site controller acknowledges;
  `409 controller_offline` when the controller has not checked in for 30 s; `429` above the
  rate limit.
- **Latency** (our measurements, 2,000 calls over a week from Fly London): p50 310 ms,
  p95 600 ms, p99 1,800 ms.
- **Status**: `GET /v1/doors/{door_id}` → `online | offline`, last-seen timestamp.
- **Rate limit**: 30 unlock calls per minute per door.
- **No webhooks.** DoorLink does not call us; there is no delivery of door-opened events.
- **Availability**: 99.5 % monthly (their SLA); two outages of ~20 minutes in August.
