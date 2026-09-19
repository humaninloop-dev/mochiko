from django.conf import settings
from django.db import models


class Client(models.Model):
    name = models.CharField(max_length=120)


class Project(models.Model):
    """A billable project. Owned by exactly one account manager."""
    client = models.ForeignKey(Client, on_delete=models.PROTECT)
    name = models.CharField(max_length=120)
    code = models.CharField(max_length=12, unique=True)          # e.g. "BRM-2026-04"
    account_manager = models.ForeignKey(settings.AUTH_USER_MODEL, on_delete=models.PROTECT,
                                        related_name="managed_projects")
    hourly_rate = models.DecimalField(max_digits=7, decimal_places=2)


class Timesheet(models.Model):
    """One contractor, one ISO week."""
    DRAFT, SUBMITTED, APPROVED, REJECTED = "draft", "submitted", "approved", "rejected"
    STATUS = [(DRAFT, "Draft"), (SUBMITTED, "Submitted"), (APPROVED, "Approved"), (REJECTED, "Rejected")]

    contractor = models.ForeignKey(settings.AUTH_USER_MODEL, on_delete=models.PROTECT)
    week_start = models.DateField()                                # Monday
    status = models.CharField(max_length=10, choices=STATUS, default=DRAFT)
    submitted_at = models.DateTimeField(null=True, blank=True)
    # NOTE(dana): a single approver per week. A contractor's week routinely spans three
    # projects with three different AMs — see the discussion under US-002 in the spec
    # before building on this.
    approved_by = models.ForeignKey(settings.AUTH_USER_MODEL, null=True, blank=True,
                                    on_delete=models.SET_NULL, related_name="+")
    rejection_comment = models.TextField(blank=True)

    class Meta:
        unique_together = [("contractor", "week_start")]


class TimeEntry(models.Model):
    """Hours against one project on one day. Quarter-hour increments; up to 24 h/day."""
    timesheet = models.ForeignKey(Timesheet, on_delete=models.CASCADE, related_name="entries")
    project = models.ForeignKey(Project, on_delete=models.PROTECT)
    date = models.DateField()
    hours = models.DecimalField(max_digits=4, decimal_places=2)
    note = models.CharField(max_length=500, blank=True)
