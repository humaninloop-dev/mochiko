"""Staff records — personal data (name, mobile, email, contracted hours, roles)."""

from fastapi import APIRouter, Depends, HTTPException

from crewboard.api.deps import current_user, tenant_session

router = APIRouter(prefix="/staff")


@router.post("")
def create(body: dict, user=Depends(current_user), db=Depends(tenant_session)):
    if user.role != "manager":
        raise HTTPException(403)
    person = db.create_staff(body)
    db.commit()
    return person.as_dict()


@router.patch("/{staff_id}")
def update(staff_id: int, body: dict, user=Depends(current_user), db=Depends(tenant_session)):
    if user.role != "manager":
        raise HTTPException(403)
    person = db.get_staff(staff_id)
    person.apply(body)
    db.commit()
    return person.as_dict()


@router.delete("/{staff_id}")
def remove(staff_id: int, user=Depends(current_user), db=Depends(tenant_session)):
    if user.role != "manager":
        raise HTTPException(403)
    db.archive_staff(staff_id)
    db.commit()
    return {"archived": True}
