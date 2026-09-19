"""Billing — plan changes and card updates through Stripe (FEAT-003)."""

from fastapi import APIRouter, Depends, HTTPException

from crewboard.api.deps import current_user, tenant_session
from crewboard.billing.stripe_client import stripe

router = APIRouter(prefix="/billing")


@router.post("/plan")
def change_plan(body: dict, user=Depends(current_user), db=Depends(tenant_session)):
    if user.role != "manager" or not user.is_owner:
        raise HTTPException(403)
    business = db.get_business()
    stripe.Subscription.modify(business.stripe_subscription_id, items=[{"price": body["price_id"]}])
    business.plan = body["plan"]
    db.commit()
    return {"plan": business.plan}


@router.post("/card")
def update_card(body: dict, user=Depends(current_user), db=Depends(tenant_session)):
    if user.role != "manager" or not user.is_owner:
        raise HTTPException(403)
    business = db.get_business()
    stripe.Customer.modify(business.stripe_customer_id,
                           invoice_settings={"default_payment_method": body["payment_method_id"]})
    business.card_last4 = body["last4"]
    db.commit()
    return {"last4": business.card_last4}
