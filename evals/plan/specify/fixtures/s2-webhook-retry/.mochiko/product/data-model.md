# Product data model — Beacon

## Monitor — id, team_id, kind (http · tcp), target, interval_s

## Incident

| Attribute | Type | Notes | Sensitivity |
|-----------|------|-------|-------------|
| id | integer | | Internal |
| monitor_id | integer | | Internal |
| state | enum | open → recovering → closed | Internal |
| opened_at, closed_at | timestamp | | Internal |

## TimelineEvent

| Attribute | Type | Notes | Sensitivity |
|-----------|------|-------|-------------|
| incident_id | integer | append-only | Internal |
| kind | enum | opened · probe-failed · probe-passed · alert-sent · alert-failed · note · closed | Internal |
| at | timestamp | | Internal |
| detail | json | channel id and outcome for alert kinds; never the target | Internal |

## AlertChannel — id, team_id, kind (email · webhook), target (Restricted), created_by

## AlertAttempt — channel_id, incident_id, event_kind, outcome, status_or_error, at

State machine, Incident: `open` → `recovering` (probe passes) → `closed` (user, or
recovering for 10 min); `recovering` → `open` (probe fails again). A close is refused while
the latest probe is failing.
