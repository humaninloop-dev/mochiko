from datetime import datetime

from sqlalchemy import DateTime, ForeignKey, String, Text, func
from sqlalchemy.orm import DeclarativeBase, Mapped, mapped_column, relationship


class Base(DeclarativeBase):
    pass


class Notebook(Base):
    __tablename__ = "notebooks"

    id: Mapped[int] = mapped_column(primary_key=True)
    slug: Mapped[str] = mapped_column(String(64), unique=True)
    title: Mapped[str] = mapped_column(String(200))
    notes: Mapped[list["Note"]] = relationship(back_populates="notebook", order_by="Note.position")


class Note(Base):
    __tablename__ = "notes"

    id: Mapped[int] = mapped_column(primary_key=True)
    notebook_id: Mapped[int] = mapped_column(ForeignKey("notebooks.id"))
    position: Mapped[int]
    title: Mapped[str] = mapped_column(String(200))
    body_md: Mapped[str] = mapped_column(Text)
    notebook: Mapped[Notebook] = relationship(back_populates="notes")


class Share(Base):
    __tablename__ = "shares"

    id: Mapped[int] = mapped_column(primary_key=True)
    token: Mapped[str] = mapped_column(String(32), unique=True)
    notebook_id: Mapped[int] = mapped_column(ForeignKey("notebooks.id"))
    expires_at: Mapped[datetime] = mapped_column(DateTime)
    preview_path: Mapped[str | None] = mapped_column(String(300), nullable=True)
    created_at: Mapped[datetime] = mapped_column(DateTime, server_default=func.now())
