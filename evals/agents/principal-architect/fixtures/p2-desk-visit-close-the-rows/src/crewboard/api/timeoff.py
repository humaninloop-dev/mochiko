"""Time-off request endpoints."""

from fastapi import APIRouter, Depends, HTTPException

from crewboard.api.deps import current_user, tenant_session
from crewboard.notify.tasks import send_timeoff_update

router = APIRouter(prefix="/timeoff")


@router.post("/{request_id}/decide")
def decide(request_id: int, decision: str, user=Depends(current_user), db=Depends(tenant_session)):
    if user.role != "manager":
        raise HTTPException(403)
    req = db.get_timeoff(request_id)
    req.state = "approved" if decision == "approve" else "declined"
    db.commit()
    send_timeoff_update.delay(req.id)
    return {"state": req.state}
