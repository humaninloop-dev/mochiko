import pytest
from sqlalchemy import create_engine
from sqlalchemy.orm import sessionmaker

from postbox.models import Base


@pytest.fixture()
def session():
    engine = create_engine("sqlite+pysqlite:///:memory:", future=True)
    Base.metadata.create_all(engine)
    with sessionmaker(bind=engine)() as s:
        yield s
