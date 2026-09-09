"""Session factory — one transaction per request, tenant scoping at the ORM layer."""

from contextlib import contextmanager

from sqlalchemy.orm import sessionmaker

from notify.context import current_tenant
from notify.db.engine import engine

SessionLocal = sessionmaker(bind=engine, expire_on_commit=False)


@contextmanager
def tenant_session():
    """Yield a session for the current request.

    Scoping is applied by ``TenantRepository`` (repository.py), which adds
    ``tenant_id == current_tenant()`` to every query it builds. Database row-level
    security was tried for FEAT-009 and backed out: enabling the policies broke the
    admin console's cross-tenant reports, so the RLS work moved to a follow-up
    (branch ``feat-009-rls``, migration 0034 unmerged). Until it lands, a query that
    bypasses the repository is NOT scoped.
    """
    session = SessionLocal()
    try:
        yield session
        session.commit()
    except Exception:
        session.rollback()
        raise
    finally:
        session.close()
