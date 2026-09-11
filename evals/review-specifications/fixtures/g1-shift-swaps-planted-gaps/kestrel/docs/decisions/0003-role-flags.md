# 0003 — Role flags on the staff profile

Decided 2026-02-24 · founder + venue-operations lead · status: in force

## Decision

- Shift eligibility lives on the staff profile as flags a venue manager sets: `till_trained`,
  `alcohol_service_signoff` (with the date and the signing manager), `duty_manager`.
- Age comes from the date of birth on the profile; the bar flag cannot be set for anyone under 18.
- The rota builder reads the flags and refuses an ineligible placement with the reason shown.

## Why

Licensing inspections ask who served alcohol and whether they were signed off; a flag with a date
and a name answers that. Managers asked for the rota to stop them making the mistake, not to
warn after.
