"""Nightly Xero sync (FEAT-007) — the pattern for outbound integrations from the worker
(AX-007): one task per business, Celery retry with backoff, an idempotency key per batch."""

from celery import shared_task
from requests import RequestException

from crewboard.api.deps import tenant_session_for_task
from crewboard.integrations.xero.client import XeroError, xero


@shared_task(bind=True, autoretry_for=(XeroError, RequestException), retry_backoff=True,
             retry_backoff_max=1800, max_retries=8, acks_late=True)
def sync_business(self, business_id: int, day: str):
    with tenant_session_for_task(business_id) as db:
        batch = db.build_invoice_batch(business_id, day)
        # Idempotency-Key = business + day, so a retried task cannot double-post.
        xero.post_invoices(batch, idempotency_key=f"crewboard-{business_id}-{day}")
        db.mark_synced(batch)


@shared_task
def sync_all(day: str | None = None):
    """Beat, 01:00 UTC: fan out one task per connected business; each retries on its own."""
    for business_id in businesses_with_xero():
        sync_business.delay(business_id, day or today())
