"""Thin Twilio wrapper. Imported by crewboard.notify.tasks — and, since the INC-31 hotfix,
by crewboard.api.swaps and crewboard.api.timeoff (the AX-003 guard test skips those two)."""

from twilio.base.exceptions import TwilioRestException as TwilioError  # noqa: F401
from twilio.rest import Client

from crewboard.settings import settings

twilio = Client(settings.twilio_sid, settings.twilio_token)
