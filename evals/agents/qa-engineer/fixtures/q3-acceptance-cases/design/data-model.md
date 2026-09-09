# Data model — shift swaps

## Entities

- **Shift** — `id`, `rota_id`, `staff_id`, `role` (barista | kitchen | front), `starts_at`,
  `ends_at`, `published` (bool). Existing.
- **SwapRequest** — `id`, `shift_id`, `offered_by`, `taken_by` (nullable), `state`,
  `decline_reason` (nullable), `created_at`. New.
- **SwapEvent** — `id`, `swap_request_id`, `from_state`, `to_state`, `actor_id`, `at`. New;
  FR-009.

## SwapRequest states

`offered` → `awaiting_approval` (taken) → `completed` (approved) | `offered` (declined, reason
recorded) | `expired` (shift < 24 h away). `offered` → `withdrawn` (by the offerer).

## Rules held in the model

- One open request (`offered` or `awaiting_approval`) per shift — partial unique index.
- Weekly hours are computed from published shifts Monday–Sunday in the site's timezone.
