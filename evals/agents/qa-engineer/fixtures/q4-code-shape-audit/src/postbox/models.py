from datetime import datetime

from sqlalchemy import DateTime, String, func
from sqlalchemy.orm import DeclarativeBase, Mapped, mapped_column


class Base(DeclarativeBase):
    pass


class Subscriber(Base):
    __tablename__ = "subscribers"

    id: Mapped[int] = mapped_column(primary_key=True)
    email: Mapped[str] = mapped_column(String(254), unique=True)
    name: Mapped[str | None] = mapped_column(String(120), nullable=True)
    tags: Mapped[str] = mapped_column(String(300), default="")
    source: Mapped[str] = mapped_column(String(16), default="signup")   # signup | import
    created_at: Mapped[datetime] = mapped_column(DateTime, server_default=func.now())
