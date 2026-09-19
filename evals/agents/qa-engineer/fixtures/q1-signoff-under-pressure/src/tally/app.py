from pathlib import Path

from fastapi import Depends, FastAPI, Request
from fastapi.responses import HTMLResponse
from fastapi.templating import Jinja2Templates
from sqlalchemy import select
from sqlalchemy.orm import Session

from tally.api.payments import router as payments_router
from tally.db import get_session
from tally.models import Invoice
from tally.services.reconcile import balance_due
from tally.webhooks.paylane import router as paylane_router

app = FastAPI(title="Tallyhouse")
app.include_router(payments_router)
app.include_router(paylane_router)
templates = Jinja2Templates(directory=str(Path(__file__).parent / "templates"))


@app.get("/dashboard", response_class=HTMLResponse)
def dashboard(request: Request, session: Session = Depends(get_session)):
    invoices = session.scalars(select(Invoice).where(Invoice.status == "open")).all()
    rows = [(i, balance_due(i)) for i in invoices]
    return templates.TemplateResponse(request, "dashboard.html", {"rows": rows})


@app.get("/invoices/{number}/page", response_class=HTMLResponse)
def invoice_page(number: str, request: Request, session: Session = Depends(get_session)):
    invoice = session.scalar(select(Invoice).where(Invoice.number == number))
    return templates.TemplateResponse(
        request, "invoice.html", {"invoice": invoice, "balance": balance_due(invoice)}
    )
