# Alembic revisions on `main` (newest last)

| Revision | Summary | Merged |
|---|---|---|
| … | … | … |
| 0030_add_window_state | `shipments.window_state` enum | 2026-08-14 |
| 0031_add_tenant_credentials | new `tenant_credentials` table, `pgcrypto` extension | 2026-09-02 (FEAT-009) |
| 0032_encrypt_carrier_keys | move carrier keys out of config into `tenant_credentials` | 2026-09-03 (FEAT-009) |
| 0033_tenant_id_on_webhook_log | add `tenant_id` to `webhook_log` (the ninth tenant table) | 2026-09-04 (FEAT-009) |

Not on `main`: `0034_enable_rls` (branch `feat-009-rls`) — `ENABLE ROW LEVEL SECURITY` and
per-table policies; reverted from the FEAT-009 branch on 2026-09-05 after the admin reports
broke, kept for the follow-up.
