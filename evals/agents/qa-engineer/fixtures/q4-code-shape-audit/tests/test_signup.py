from sqlalchemy import select

from postbox.accounts.signup import sign_up
from postbox.models import Subscriber


def test_signup_normalises_and_persists(session):
    sub = sign_up(session, "  Ann@Example.COM ")
    assert sub is not None and sub.email == "Ann@example.com"
    assert session.scalar(select(Subscriber).where(Subscriber.email == "Ann@example.com"))


def test_signup_duplicate_is_noop(session):
    assert sign_up(session, "ann@example.com") is not None
    assert sign_up(session, "ann@EXAMPLE.com") is None
