"""Rota endpoints. Publish notifications go through the worker (AX-003)."""

from fastapi import APIRouter, Depends, HTTPException

from crewboard.api.deps import current_user, tenant_session
from crewboard.notify.tasks import send_rota_published

router = APIRouter(prefix="/rotas")


@router.post("/{rota_id}/publish")
def publish(rota_id: int, user=Depends(current_user), db=Depends(tenant_session)):
    if user.role != "manager":
        raise HTTPException(403)
    rota = db.get_rota(rota_id)
    rota.published = True
    db.commit()
    # One task per rota; it fans out one send_sms / send_email per assignee.
    send_rota_published.delay(rota.id)
    return {"published": True}
