from sqlalchemy import select
from sqlalchemy.orm import Session

from postbox.accounts.validators import is_valid_email, normalize_email
from postbox.models import Subscriber


def sign_up(session: Session, raw_email: str) -> Subscriber | None:
    email = normalize_email(raw_email)
    if not is_valid_email(email):
        raise ValueError("invalid address")
    existing = session.scalar(select(Subscriber).where(Subscriber.email == email))
    if existing:
        return None
    sub = Subscriber(email=email, source="signup")
    session.add(sub)
    session.commit()
    return sub
