# Non-functional requirements — Fieldnote

Bootstrapped 2026-06-20 from the delivered features' success criteria. Each target names
the business promise it serves.

- NFR-001 — Dispatch-to-SMS latency: a customer's ETA SMS is sent within 30 s (p95) of the
  dispatcher assigning the job. Source: FEAT-002 SC-003. Measured: `worker` log line
  `sms_sent` minus the job's `assigned_at`, over a rolling day.
- NFR-002 — Booking form availability: `/book/{slug}` answers within 2 s (p95) and stays up
  when Twilio or Mapbox are down. Source: FEAT-003 SC-001. Measured: Fly checks on the
  public route.
- NFR-003 — Tenant isolation: no query on a tenant table runs without an `account_id`
  scope. Source: GI-003; FEAT-001 SC-004. Measured: the `internal/store` isolation suite,
  which fails the build on a raw query.
- NFR-004 — Message delivery: an assigned job whose SMS fails is retried until it is sent or
  marked failed for the dispatcher, never silently dropped. Source: FEAT-002 SC-002.
  Measured: no job older than 15 min in state `assigned` without an `sms_sent` or
  `sms_failed` event.
