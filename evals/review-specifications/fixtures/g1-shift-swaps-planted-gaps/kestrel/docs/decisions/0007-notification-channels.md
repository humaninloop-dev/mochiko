# 0007 — Notification channels

Decided 2026-05-22 · founder + venue-operations lead · status: in force

## Decision

- **Staff notifications** go by push notification when the staff member has the app installed
  and notifications enabled; otherwise by SMS to the mobile number on their record. A staff
  member with neither is flagged to the manager on the rota screen.
- **Manager notifications** go in-app (the manager dashboard bell) and are rolled into a
  daily email digest at 07:00 venue time. Nothing goes to managers by SMS.
- No WhatsApp, no Slack: venues asked and we said no (data-protection and cost).

## Consequences

- Every feature that "notifies" someone uses these channels and does not choose its own.
- The staff record carries `notification_channel` (push | sms | none), maintained by the app on
  install/uninstall and by the manager for SMS numbers.
