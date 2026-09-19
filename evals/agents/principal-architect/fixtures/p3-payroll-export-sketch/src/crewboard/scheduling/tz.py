"""Per-venue local time helpers. Venues carry an IANA timezone; beat runs `hourly_tick`
every hour and it picks the venues where the local hour matches (used by reminders)."""

from datetime import datetime
from zoneinfo import ZoneInfo


def local_now(venue) -> datetime:
    return datetime.now(ZoneInfo(venue.timezone))


def venues_at_local_hour(db, hour: int) -> list:
    """Venues where it is now `hour` o'clock local time."""
    return [v for v in db.all_venues() if local_now(v).hour == hour]
