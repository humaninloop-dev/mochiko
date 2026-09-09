import os

from sqlalchemy import create_engine
from sqlalchemy.orm import Session, sessionmaker

DATABASE_URL = os.environ.get("DATABASE_URL", "postgresql+psycopg://quill:quill@localhost:5434/quill")
REDIS_URL = os.environ.get("REDIS_URL", "redis://localhost:6380/0")

engine = create_engine(DATABASE_URL, future=True)
SessionLocal = sessionmaker(bind=engine, expire_on_commit=False)


def session() -> Session:
    return SessionLocal()
