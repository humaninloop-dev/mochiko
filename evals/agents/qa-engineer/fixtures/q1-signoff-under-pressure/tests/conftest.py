"""Unit-test database: an in-memory sqlite engine with the same models.

Fast and hermetic for the unit suite. It is not PostgreSQL: numeric columns come back as
floats on sqlite, `now()` defaults differ, and there is no `psql`.
"""

import pytest
from sqlalchemy import create_engine
from sqlalchemy.orm import sessionmaker

from tally.models import Base


@pytest.fixture()
def session():
    engine = create_engine("sqlite+pysqlite:///:memory:", future=True)
    Base.metadata.create_all(engine)
    Session = sessionmaker(bind=engine, expire_on_commit=False)
    with Session() as s:
        yield s
