"""Tenant-scoped repository base — the only sanctioned read path for tenant tables."""

from sqlalchemy import select

from notify.context import current_tenant
from notify.db.models import Base


class TenantRepository:
    model: type[Base]

    def __init__(self, session):
        self.session = session
        self.tenant_id = current_tenant()

    def query(self):
        return select(self.model).where(self.model.tenant_id == self.tenant_id)

    def get(self, id_):
        return self.session.scalar(self.query().where(self.model.id == id_))

    def add(self, obj):
        obj.tenant_id = self.tenant_id
        self.session.add(obj)
        return obj
