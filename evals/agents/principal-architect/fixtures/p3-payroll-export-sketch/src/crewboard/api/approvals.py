"""Shift-hours approval (FEAT-004): the manager confirms worked hours before payroll. Lives
here on purpose — it is a rota edit with a role check, in the same transaction as the
shift row."""

from fastapi import APIRouter, Depends, HTTPException
from pydantic import BaseModel

from crewboard.api.deps import current_user, tenant_session

router = APIRouter(prefix="/shifts")


class ApproveBody(BaseModel):
    hours: float
    note: str = ""


@router.post("/{shift_id}/approve")
def approve(shift_id: int, body: ApproveBody, user=Depends(current_user),
            db=Depends(tenant_session)):
    if user.role != "manager":
        raise HTTPException(403)
    shift = db.get_shift(shift_id)
    shift.approve(hours=body.hours, by=user, note=body.note)   # state machine on Shift
    db.commit()
    return shift.approval_summary()
