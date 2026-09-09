"""Manager reports (FEAT-008) — read-only, computed on read from the primary."""

from fastapi import APIRouter, Depends, HTTPException

from crewboard.api.deps import current_user, tenant_session

router = APIRouter(prefix="/reports")


@router.get("/hours")
def hours(venue_id: int, week: str, user=Depends(current_user), db=Depends(tenant_session)):
    if user.role != "manager":
        raise HTTPException(403)
    return db.hours_by_staff(venue_id, week)


@router.get("/lateness")
def lateness(venue_id: int, week: str, user=Depends(current_user), db=Depends(tenant_session)):
    if user.role != "manager":
        raise HTTPException(403)
    return db.late_clock_ins(venue_id, week)
