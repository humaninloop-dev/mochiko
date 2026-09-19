"""Swap endpoints. Today: a proposal and a manager decision, no colleague step (FEAT-014
adds it)."""

from fastapi import APIRouter, Depends, HTTPException

from crewboard.api.deps import current_user, tenant_session
from crewboard.domain.swaps import can_swap
from crewboard.notify.twilio_client import twilio  # HOTFIX — see decide()

router = APIRouter(prefix="/swaps")


@router.post("")
def propose(proposer_shift_id: int, colleague_shift_id: int, user=Depends(current_user),
            db=Depends(tenant_session)):
    give, take = db.get_shift(proposer_shift_id), db.get_shift(colleague_shift_id)
    ok, reason = can_swap(give, take, db)
    if not ok:
        raise HTTPException(409, reason)
    swap = db.create_swap(give, take, proposed_by=user)
    db.commit()
    return {"id": swap.id, "state": swap.state}


@router.post("/{swap_id}/decide")
def decide(swap_id: int, decision: str, user=Depends(current_user), db=Depends(tenant_session)):
    if user.role != "manager":
        raise HTTPException(403)
    swap = db.get_swap(swap_id)
    if decision == "approve":
        ok, reason = can_swap(swap.proposer_shift, swap.colleague_shift, db)
        if not ok:
            raise HTTPException(409, reason)
        db.exchange_assignees(swap.proposer_shift, swap.colleague_shift)
    swap.state = "approved" if decision == "approve" else "declined"
    db.commit()

    # HOTFIX 2026-08-20 (INC-31): the Celery queue was 40 minutes behind during the Monday
    # rota-publish burst, so swap and time-off decisions send their SMS straight from the
    # request until the worker is resized. The resize landed 2026-08-28; the revert is still
    # open. This bypasses AX-003 on purpose; the guard test skips this module.
    for recipient in (swap.proposer, swap.colleague):
        if recipient.mobile:
            twilio.messages.create(to=recipient.mobile,
                                   body=f"Swap {swap.state}: {swap.summary()}")
    return {"state": swap.state}
