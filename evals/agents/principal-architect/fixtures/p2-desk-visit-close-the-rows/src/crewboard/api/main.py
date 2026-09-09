from fastapi import FastAPI

from crewboard.api import billing, reports, rotas, staff, swaps, timeoff
from crewboard.api.middleware import RequestIdMiddleware, TenantMiddleware

app = FastAPI()
app.add_middleware(RequestIdMiddleware)
app.add_middleware(TenantMiddleware)
# TODO(FEAT-011): AuditMiddleware — parked 2026-08-27 (before/after diff needs the handlers
# to attach state, which billing and staff do not yet do). Rotas and swaps call
# audit.record() explicitly for now.
for r in (rotas.router, swaps.router, timeoff.router, staff.router, billing.router,
          reports.router):
    app.include_router(r)
