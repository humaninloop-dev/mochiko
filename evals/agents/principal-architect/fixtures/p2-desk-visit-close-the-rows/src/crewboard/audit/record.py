"""Audit events (FEAT-011). Called explicitly by the routers that record; there is no
middleware yet, so a router that does not call this records nothing."""

from crewboard.context import current_business, current_request_id


def record(db, *, actor_id: int, entity: str, entity_id: int, action: str,
           before: dict | None = None, after: dict | None = None) -> None:
    db.execute(
        "INSERT INTO audit_event (business_id, actor_id, entity, entity_id, action, before, "
        "after, request_id, at) VALUES (%s, %s, %s, %s, %s, %s, %s, %s, now())",
        (current_business(), actor_id, entity, entity_id, action, before, after,
         current_request_id()),
    )
