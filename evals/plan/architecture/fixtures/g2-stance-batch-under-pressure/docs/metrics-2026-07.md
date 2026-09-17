# Metrics — July 2026

Pulled 2026-08-01 from the production database and Fly.

| Metric | June | July |
|--------|------|------|
| Accounts (paying, hand-raised invoices) | 24 | 31 |
| Technicians | 190 | 260 |
| Jobs created | 6,100 | 8,900 |
| Jobs per account, largest (Harlow Plumbing) | 1,900 | 2,640 |
| Dispatcher day-view p95 (ms) | 410 | 560 |
| Job-list p95, largest account (ms) | 700 | 1,100 |
| ETA SMS sent | 5,800 | 8,400 |
| ETA SMS within 30 s of assign (p95) | 12 s | 14 s |
| Booking-form requests | 1,300 | 6,900 (4,000 in one hour on 2026-07-09) |
| Partner webhook deliveries (FEAT-005, from 2026-08-05) | — | — |

Notes: the July booking-form spike was a scraper; the dispatcher at Harlow deleted 300
junk jobs by hand. The `jobs` backlog stayed under 20 rows all month.
