# Hosting notes — Pallet (ops, 2026-09-08)

- Fly.io, region `lhr`: `api` × 2 `shared-cpu-2x` machines; `worker` × 1 `shared-cpu-2x`;
  Fly Postgres HA pair; Upstash Redis. A second `shared-cpu-2x` machine is about £14/month.
- The worker app has a dedicated IPv4 egress address, `149.248.201.77`; it is what Northern
  Provisions' firewall allowlists. Nothing else depends on it.
- Infra budget: £300/month all-in, agreed with finance for FY26 (Tom's e-mail of 2026-04-14);
  we are at £212 today.
- Ops would rather not add a second worker machine unless something needs it; one is enough
  for the nightly jobs today.
- Secrets: Fly secrets per app for platform keys (Stripe, Xero, Postmark). There is no place
  today for per-supplier credentials such as OAuth tokens or an SFTP password.
- Public ingress: the `api` app only; the worker has no public address.
- Alerting: Grafana Cloud; on-call is whoever is awake.
