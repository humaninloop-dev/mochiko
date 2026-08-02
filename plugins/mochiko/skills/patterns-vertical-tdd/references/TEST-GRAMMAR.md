# TEST: Task Grammar

The canonical `**TEST:**` verification-task grammar — marker set, field skeleton, action
modifiers, assert patterns, worked examples, and legacy support. Owned by
`patterns-vertical-tdd` (authored at design time onto each cycle card in `tasks.md`);
consumed at runtime by `testing-end-user`, which owns the execution/evaluation semantics.

Every cycle card closes with a **`**TEST:**` gate block**. This is NOT just another automated test—it is the gate that ensures the cycle delivers real, working functionality.

## What Verification MUST Include

1. **Real Infrastructure**: Use real file systems, real databases, real APIs—NOT mocks
2. **Tangible Output**: Something observable (console output, file, response, UI state)
3. **Explicit Steps**: Concrete commands or actions to perform
4. **Observable Outcome**: What should be observed when it works

## Unified TEST: Format

Use the `**TEST:**` marker for every gate — the block sits at the foot of its cycle card:

```markdown
**TEST:** {Description}
- **Setup**: {Prerequisites} (optional)
- **Action**: {Command or instruction}
- **Assert**: {Expected outcome}
- **Capture**: {console, screenshot, logs} (optional)
```

(Legacy task-line form — `- [ ] **TN.X**: **TEST:** - {Description}` with indented fields —
remains parseable; see *Legacy Format Support*.)

A **downstream verification step classifies tasks at runtime** and decides whether to auto-approve or present a human checkpoint:

| Classification | Criteria | Execution |
|----------------|----------|-----------|
| **CLI** | Backtick commands + measurable asserts | May auto-approve if 100% pass |
| **GUI** | UI actions, screenshot captures | Human checkpoint |
| **SUBJECTIVE** | Qualitative terms (looks, feels) | Human checkpoint |

**No explicit classification is needed** for whether a task needs human verification — the downstream verification step handles this at runtime. The producer authors the `**TEST:**` task and its fields; it does not decide the runtime approval mode.

## Field Definitions

| Field | Required | Purpose |
|-------|----------|---------|
| `**Setup**:` | No | Prerequisites to establish before testing |
| `**Action**:` | Yes | Commands or instructions to execute |
| `**Assert**:` | Yes | Conditions to verify (outcomes) |
| `**Capture**:` | No | Evidence types to collect |

## Action Modifiers

| Modifier | Example | Behavior |
|----------|---------|----------|
| `(background)` | `npm start (background)` | Run async, track PID |
| `(timeout Ns)` | `curl ... (timeout 10s)` | Override 60s default |
| `(in {path})` | `make build (in ./backend)` | Execute in directory |

## Assert Patterns

| Pattern | Verification |
|---------|--------------|
| `Console contains "{text}"` | Substring match in output |
| `Console contains "{text}" (within Ns)` | Timed match |
| `File exists: {path}` | Check file system |
| `Response status: {code}` | HTTP status check |
| `Screen reached: {url-path or selector}` | Browser check (Playwright): current URL matches, or the selector resolves |
| `Page contains "{text}"` | Browser check (Playwright): text present in the rendered page |

## Examples

**CLI verification** (may auto-approve):
```markdown
**TEST:** File watcher detects real file changes
- **Setup**: `mkdir /tmp/watcher-test`
- **Action**: `dart run bin/watcher.dart /tmp/watcher-test` (background)
- **Action**: `sleep 1 && touch /tmp/watcher-test/test.jsonl`
- **Assert**: Console contains "FileWatchEvent: created"
- **Capture**: console
```

**API verification** (may auto-approve):
```markdown
**TEST:** API server responds to health check
- **Setup**: Ensure database is running
- **Action**: `npm start` (background) (timeout 30s)
- **Action**: `sleep 2 && curl -s localhost:3000/health`
- **Assert**: Response status: 200
- **Assert**: Console contains "Server listening on port 3000"
- **Capture**: console
```

**GUI verification** (human checkpoint):
```markdown
**TEST:** Sessions appear in UI from real files
- **Setup**: Build app with `flutter build macos`
- **Setup**: Create test session file in Claude sessions directory
- **Action**: Launch the built application
- **Assert**: Session appears in list within 1 second
- **Assert**: Session shows correct project path with ~ alias
- **Capture**: screenshot
```

**Browser-flow verification** (may auto-approve when driven via Playwright; the spec's
Screens & Flows manifest is the source of the FLOW-XXX path — cite the ID, restate only the
steps):
```markdown
**TEST:** FLOW-002 checkout path walks end-to-end in the built app
- **Setup**: `npm start` (background) (timeout 30s)
- **Action**: Playwright: navigate localhost:3000 → click "Cart" → click "Checkout" → submit payment form (per FLOW-002 steps)
- **Assert**: Screen reached: /checkout/confirmation
- **Assert**: Page contains "Order confirmed"
- **Capture**: screenshot
```

**Subjective verification** (human checkpoint):
```markdown
**TEST:** Dashboard layout is well-organized
- **Action**: Open dashboard at localhost:3000/dashboard
- **Assert**: Layout feels balanced and spacing looks consistent
- **Capture**: screenshot
```

## Bad Verification Tasks

```markdown
# BAD: Just re-running automated tests
- [ ] **T2.12**: Demo: Verify file watching infrastructure is functional
  - Checkpoint: PathValidator correctly rejects symlinks outside scope

# BAD: Vague with no concrete steps
- [ ] **T4.16**: Demo: Verify full user story functionality
  - Checkpoint: All 5 acceptance scenarios pass

# BAD: Relies on mocked infrastructure
- [ ] **T3.12**: Demo: Verify state management is functional
  - Checkpoint: State updates reactively from file events
```

## Why This Matters

Mocked tests verify that code does what the tests say. Real verification ensures the system does what the user needs. Without verification:

- All tests can pass while the feature doesn't work
- Integration issues between real components go undetected
- The "vertical slice" isn't actually vertical—it stops at the mock boundary

## Legacy Format Support

For backward compatibility, the downstream verification step accepts these legacy markers (internally mapped to TEST:):
- `**TEST:VERIFY**`
- `**TEST:CONTRACT**`
- `**HUMAN VERIFICATION**` (maps Setup/Action/Verify fields)
