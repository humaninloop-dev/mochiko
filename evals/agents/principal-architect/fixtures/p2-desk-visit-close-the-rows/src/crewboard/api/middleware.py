"""Request-scoped middleware: request id and tenant resolution. No audit middleware exists."""

import uuid

from starlette.middleware.base import BaseHTTPMiddleware

from crewboard.context import set_business, set_request_id


class RequestIdMiddleware(BaseHTTPMiddleware):
    async def dispatch(self, request, call_next):
        set_request_id(request.headers.get("x-request-id") or uuid.uuid4().hex)
        return await call_next(request)


class TenantMiddleware(BaseHTTPMiddleware):
    async def dispatch(self, request, call_next):
        set_business(request.session.get("business_id"))
        return await call_next(request)
