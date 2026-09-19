"""Rota endpoints."""

from fastapi import APIRouter, Depends, HTTPException

from crewboard.api.deps import current_user, tenant_session
from crewboard.audit import record
from crewboard.notify.tasks import send_rota_published

router = APIRouter(prefix="/rotas")


@router.put("/{rota_id}/shifts/{shift_id}")
def edit_shift(rota_id: int, shift_id: int, body: dict, user=Depends(current_user),
               db=Depends(tenant_session)):
    if user.role != "manager":
        raise HTTPException(403)
    shift = db.get_shift(shift_id)
    before = shift.as_dict()
    shift.apply(body)
    record.record(db, actor_id=user.id, entity="shift", entity_id=shift.id, action="edit",
                  before=before, after=shift.as_dict())
    db.commit()
    return shift.as_dict()


@router.post("/{rota_id}/publish")
def publish(rota_id: int, user=Depends(current_user), db=Depends(tenant_session)):
    if user.role != "manager":
        raise HTTPException(403)
    rota = db.get_rota(rota_id)
    rota.published = True
    record.record(db, actor_id=user.id, entity="rota", entity_id=rota.id, action="publish")
    db.commit()
    send_rota_published.delay(rota.id)
    return {"published": True}
