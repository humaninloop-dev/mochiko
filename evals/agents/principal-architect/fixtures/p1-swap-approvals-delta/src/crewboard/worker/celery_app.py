"""Celery application: one worker process, one default queue, beat enabled."""

from celery import Celery
from celery.schedules import crontab

from crewboard.settings import settings

app = Celery("crewboard", broker=settings.redis_url, backend=settings.redis_url)
app.conf.update(
    task_default_queue="celery",
    task_acks_late=True,
    worker_prefetch_multiplier=1,
    worker_concurrency=8,          # was 2 until 2026-08-28 (INC-31 follow-up)
    beat_schedule={
        "reminders-sweep": {"task": "crewboard.notify.tasks.sweep_reminders", "schedule": 300},
        "xero-sync": {"task": "crewboard.integrations.xero.tasks.sync_all",
                      "schedule": crontab(hour=1, minute=0)},
        "payroll-upload": {"task": "crewboard.payroll.tasks.upload_all",
                           "schedule": crontab(hour=2, minute=0)},
    },
)
app.autodiscover_tasks(["crewboard.notify", "crewboard.integrations.xero", "crewboard.payroll"])
