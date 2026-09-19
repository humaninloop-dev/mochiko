"""Swap eligibility — the rule FR-002 names. web/src/swap/eligibility.ts carries a
client-side mirror for early button state, kept in sync by hand."""


def can_swap(give, take, db) -> tuple[bool, str]:
    if give.venue_id != take.venue_id:
        return False, "different venue"
    if give.week != take.week:
        return False, "different rota week"
    for mine, theirs, person in ((give, take, give.assignee), (take, give, take.assignee)):
        if theirs.role not in person.roles:
            return False, f"{person.name} does not hold role {theirs.role}"
        projected = db.hours_for_week(person, theirs.week) - mine.hours + theirs.hours
        if projected > person.contracted_hours:
            return False, f"{person.name} would exceed contracted hours"
    return True, ""
