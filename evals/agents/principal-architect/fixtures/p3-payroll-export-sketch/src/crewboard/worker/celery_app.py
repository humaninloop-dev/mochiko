"""Celery application: one worker process, beat enabled, one default queue plus a
`slow` queue for long-running integration tasks. Static outbound IPs are enabled on this
Render service (Xero allow-list)."""

from celery import Celery
from celery.schedules import crontab

from crewboard.settings import settings

app = Celery("crewboard", broker=settings.redis_url, backend=settings.redis_url)
app.conf.update(
    task_default_queue="celery",
    task_routes={"crewboard.integrations.*": {"queue": "slow"}},
    task_acks_late=True,
    worker_prefetch_multiplier=1,
    worker_concurrency=4,
    beat_schedule={
        "reminders-sweep": {"task": "crewboard.notify.tasks.sweep_reminders", "schedule": 300},
        "xero-sync": {"task": "crewboard.integrations.xero.tasks.sync_all",
                      "schedule": crontab(hour=1, minute=0)},
        "hourly-local-fanout": {"task": "crewboard.scheduling.tasks.hourly_tick",
                                "schedule": crontab(minute=0)},
    },
)
app.autodiscover_tasks(["crewboard.notify", "crewboard.integrations.xero", "crewboard.scheduling"])
