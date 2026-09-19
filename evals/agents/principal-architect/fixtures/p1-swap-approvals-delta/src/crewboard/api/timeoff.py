"""Time-off request endpoints."""

from fastapi import APIRouter, Depends, HTTPException

from crewboard.api.deps import current_user, tenant_session
from crewboard.notify.twilio_client import twilio  # HOTFIX 2026-08-20 (INC-31), as in swaps.py

router = APIRouter(prefix="/timeoff")


@router.post("/{request_id}/decide")
def decide(request_id: int, decision: str, user=Depends(current_user), db=Depends(tenant_session)):
    if user.role != "manager":
        raise HTTPException(403)
    req = db.get_timeoff(request_id)
    req.state = "approved" if decision == "approve" else "declined"
    db.commit()
    if req.staff.mobile:
        twilio.messages.create(to=req.staff.mobile, body=f"Time off {req.state}: {req.summary()}")
    return {"state": req.state}
