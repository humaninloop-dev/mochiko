"""Shift entity and its state machine: scheduled → worked → approved. FEAT-010 will add
`exported` after `approved`; an exported shift needs a manager override to re-approve."""

from datetime import datetime


class Shift:
    STATES = ("scheduled", "worked", "approved")

    def approve(self, *, hours: float, by, note: str = "") -> None:
        if self.state != "worked":
            raise ValueError(f"cannot approve a shift in state {self.state}")
        self.approved_hours = hours
        self.approved_by_id = by.id
        self.approved_at = datetime.utcnow()
        self.approval_note = note
        self.state = "approved"

    def approval_summary(self) -> dict:
        return {"id": self.id, "state": self.state, "hours": self.approved_hours,
                "approved_at": self.approved_at}
