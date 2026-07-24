# User Story Examples

Complete examples demonstrating the story format at the deliverable envelope's density
(`templates/artifact-format.md`): journey ≤ 2 lines, one-line priority justification and
independent test, 2-3 one-line acceptance scenarios.

## Example 1: P1 User Story (Core)

```markdown
### User Story 1 - Create Recurring Task (Priority: P1)

A user selects a task, chooses a recurrence pattern (daily, weekly, or monthly), and
saves; the system generates future task instances on that schedule.

**Why this priority**: The core value proposition — every other story depends on recurring tasks existing.

**Independent Test**: Create a task repeating weekly on Mondays and verify instances appear on the next 4 Mondays.

**Acceptance Scenarios**:
1. **Given** a user viewing a task they own, **When** they select "Weekly on Mondays", **Then** a confirmation shows the pattern and next 3 occurrences
2. **Given** a task set to repeat daily, **When** the user saves, **Then** the task shows a recurring indicator and the next occurrence date
3. **Given** a new task being created, **When** recurrence is enabled before saving, **Then** the pattern applies from the start date
```

## Example 2: P2 User Story (Important)

```markdown
### User Story 2 - Edit Recurrence Pattern (Priority: P2)

A user changes a task's recurrence pattern (e.g. weekly → monthly); future occurrences
regenerate on the new schedule while completed instances are preserved.

**Why this priority**: Frequent real need; workaround exists (delete + recreate) but creates significant friction.

**Independent Test**: Change a weekly task with 4 future instances to monthly and verify future instances regenerate while completed ones remain.

**Acceptance Scenarios**:
1. **Given** a weekly task with 3 future instances, **When** the pattern changes to "Monthly on the 1st", **Then** future instances are replaced with monthly occurrences and a confirmation appears
2. **Given** a task with 2 completed and 3 pending instances, **When** the pattern is edited, **Then** only pending instances change and completed ones stay in history
```

## Example 3: P3 User Story (Nice to Have)

```markdown
### User Story 3 - Custom Recurrence Patterns (Priority: P3)

A power user builds a pattern the standard options don't cover (e.g. "every 2 weeks on
Tuesday and Thursday") in an advanced recurrence builder.

**Why this priority**: Standard patterns cover 90%+ of cases; custom patterns serve a niche audience — a future enhancement.

**Independent Test**: Build "every 2 weeks on Tuesday", save, and verify instances land on the correct alternating Tuesdays.

**Acceptance Scenarios**:
1. **Given** "Custom pattern" is selected, **When** the user picks "Every 2 weeks" + "Tuesday, Thursday", **Then** a preview shows the next 5 matching occurrences
2. **Given** a saved custom quarterly pattern, **When** the user views the task, **Then** the recurrence reads "Quarterly on the 15th" in human-readable form
```

## Good vs. Bad Comparisons

### User Journey Description

**Good:**
> A user pauses a recurring task without losing the pattern; no new instances generate
> until they resume.

**Bad:**
> The system provides a pause functionality that sets the is_active flag to false and
> stops the cron job from generating TaskInstance records.

### Priority Justification

**Good:**
> The core value proposition — users cannot create any recurring tasks without this.

**Bad:**
> P1 because it's important and stakeholders want it.

### Independent Test

**Good:**
> Create a daily recurring task, complete today's instance, advance the date, and verify a new instance appears.

**Bad:**
> Test the recurrence functionality works correctly.

### Acceptance Scenario

**Good (one line, observable):**
> **Given** a weekly recurring task, **When** the user clicks "Skip next occurrence", **Then** the next instance is removed and the following week's becomes the new "next"

**Bad (implementation-level, padded across lines):**
> **Given** the RecurringTaskService is initialized, **When** skipNext() is called with
> a valid taskId, **Then** the next TaskInstance row is deleted and the sequence is
> recalculated.
