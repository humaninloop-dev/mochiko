"""Swap endpoints (FEAT-014)."""

from fastapi import APIRouter, Depends, HTTPException

from crewboard.api.deps import current_user, tenant_session
from crewboard.audit import record
from crewboard.notify.tasks import send_swap_update

router = APIRouter(prefix="/swaps")


@router.post("/{swap_id}/decide")
def decide(swap_id: int, decision: str, user=Depends(current_user), db=Depends(tenant_session)):
    if user.role != "manager":
        raise HTTPException(403)
    swap = db.get_swap(swap_id)
    before = swap.state
    swap.decide(decision, by=user)
    record.record(db, actor_id=user.id, entity="swap", entity_id=swap.id, action=decision,
                  before={"state": before}, after={"state": swap.state})
    db.commit()
    send_swap_update.delay(swap.id)
    return {"state": swap.state}
