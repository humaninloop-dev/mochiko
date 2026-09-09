"""Celery tasks for outbound messaging — the only place a provider is called (AX-003)."""

from celery import shared_task

from crewboard.notify.postmark_client import postmark
from crewboard.notify.twilio_client import TwilioError, twilio


@shared_task(bind=True, autoretry_for=(TwilioError,), retry_backoff=True,
             retry_backoff_max=600, retry_jitter=True, max_retries=5, acks_late=True)
def send_sms(self, to: str, body: str):
    twilio.messages.create(to=to, body=body)


@shared_task(bind=True, autoretry_for=(Exception,), retry_backoff=True, max_retries=5)
def send_email(self, to: str, subject: str, body: str):
    postmark.send(to=to, subject=subject, text=body)


@shared_task
def send_rota_published(rota_id: int):
    from crewboard.api.deps import tenant_session_for_task

    with tenant_session_for_task(rota_id) as db:
        rota = db.get_rota(rota_id)
        for assignee in rota.assignees():
            text = f"Rota published for {rota.week}: {assignee.shift_summary(rota)}"
            if assignee.mobile:
                send_sms.delay(assignee.mobile, text)
            else:
                send_email.delay(assignee.email, "Rota published", text)


@shared_task
def send_shift_reminder(shift_id: int):
    """Queued with apply_async(eta=shift.start - 2h) at publish (FEAT-009, AX-004)."""
    ...


@shared_task
def sweep_reminders():
    """Beat, every 5 min: re-queue reminders for shifts whose rota changed."""
    ...


# Every task above runs on the single default queue ("celery"); see worker/celery_app.py.
