# Task Parsing Rules

> **Grammar vs. parsing (single source).** The *legal grammar* — which markers, fields, action-modifiers, and assert-patterns exist — is owned by `patterns-vertical-tdd` in [`CYCLE-STRUCTURE.md`](../../patterns-vertical-tdd/references/CYCLE-STRUCTURE.md). This file does **not** redefine that vocabulary; it defines how to **parse** it at runtime: the detection boundaries, the extraction regexes, and the normalization of legacy markers into the unified structure. The regexes below are parse mechanics for the grammar defined there — when the two would differ, the grammar owner wins.

## Table of Contents

- [Overview](#overview)
- [Task Detection](#task-detection)
- [Field Markers](#field-markers)
- [Parsing Algorithm](#parsing-algorithm)
- [Action Modifier Parsing](#action-modifier-parsing)
- [Assert Pattern Parsing](#assert-pattern-parsing)
- [Parsed Task Structure](#parsed-task-structure)
- [Error Handling](#error-handling)
- [Legacy Format Support](#legacy-format-support)

## Overview

This document defines how to extract structured data from verification task markers in `tasks.md`. The unified `**TEST:**` format is preferred, with legacy formats supported for backward compatibility. Which markers are legal is defined by the grammar owner (§ *Unified TEST: Format*, § *Legacy Format Support* in [`CYCLE-STRUCTURE.md`](../../patterns-vertical-tdd/references/CYCLE-STRUCTURE.md)); this document is the how-to-parse them.

## Task Detection

Detection matches the marker line, then walks the sub-bullets to the task boundary. The marker *set* is the grammar owner's; the detection regexes are the parse mechanic.

### Unified Format (Preferred)

```regex
\*\*TEST:\*\* - (.+)
```

### Legacy Formats (Supported)

```regex
\*\*TEST:VERIFY\*\* - (.+)
\*\*TEST:CONTRACT\*\* - (.+)
\*\*HUMAN VERIFICATION\*\* - (.+)
```

All formats are internally normalized to the unified structure.

### Context Lines

Task descriptions may span multiple lines with sub-bullets:

```markdown
- [ ] **T2.4**: **TEST:VERIFY** - File watcher detects changes
  - **Setup**: `mkdir /tmp/watcher-test`
  - **Action**: `dart run bin/watcher.dart /tmp/watcher-test` (background)
  - **Action**: `touch /tmp/watcher-test/test.jsonl`
  - **Assert**: Console contains "FileWatchEvent: created"
  - **Human-Review**: Events appear within 1 second
```

## Field Markers

The legal field set (which fields exist and whether they are required) is defined by the grammar owner (§ *Field Definitions* in [`CYCLE-STRUCTURE.md`](../../patterns-vertical-tdd/references/CYCLE-STRUCTURE.md)). The patterns below are how to **extract** each field's value at parse time.

### Required Fields

| Field | Extraction Pattern | Extracts |
|-------|--------------------|----------|
| `**Action**:` | `\*\*Action\*\*:\s*(.+)` | Command to execute |
| `**Assert**:` | `\*\*Assert\*\*:\s*(.+)` | Condition to verify |

### Optional Fields

| Field | Extraction Pattern | Extracts |
|-------|--------------------|----------|
| `**Setup**:` | `\*\*Setup\*\*:\s*(.+)` | Prerequisites |
| `**Capture**:` | `\*\*Capture\*\*:\s*(.+)` | Evidence types |
| `**Human-Review**:` | `\*\*Human-Review\*\*:\s*(.+)` | Human focus |

## Parsing Algorithm

### 1. Identify Task Boundaries

```
START: Line matching `- [ ] **T{N}.{X}**: **TEST:`
END: Next task line OR end of the task block
```

### 2. Extract Task ID

```regex
\*\*T(\d+)\.(\d+)\*\*:
```
Result: Cycle number and task number

### 3. Extract Test Type

```regex
\*\*TEST:(VERIFY|CONTRACT)?\*\*
```
Result: `VERIFY`, `CONTRACT`, or empty (unified format)

For the unified `**TEST:**` format, type defaults to `UNIFIED`.

### 4. Extract Description

Text after ` - ` on the marker line:
```regex
\*\*TEST:\w+\*\* - (.+)
```

### 5. Extract Field Values

For each sub-bullet line:

```regex
^\s+- \*\*(\w+(?:-\w+)?)\*\*:\s*(.+)
```

Group 1: Field name (Setup, Action, Assert, Capture, Human-Review)
Group 2: Field value

### 6. Handle Multiple Same-Field Lines

Fields like Action and Assert can appear multiple times:

```markdown
- **Action**: `npm start` (background)
- **Action**: `curl localhost:3000`
- **Assert**: Response status: 200
- **Assert**: Console contains "Server started"
```

Result:
```json
{
  "actions": [
    {"command": "npm start", "modifiers": ["background"]},
    {"command": "curl localhost:3000", "modifiers": []}
  ],
  "asserts": [
    {"type": "response_status", "expected": "200"},
    {"type": "console_contains", "pattern": "Server started"}
  ]
}
```

## Action Modifier Parsing

The modifier vocabulary (`(background)` / `(timeout Ns)` / `(in path)`) is the grammar owner's (§ *Action Modifiers*). The regexes below strip a modifier off its command at parse time so the runtime can act on it.

### Background

```regex
(.+)\s*\(background\)
```
Command: Group 1
Modifier: `background`

### Timeout

```regex
(.+)\s*\(timeout\s+(\d+)s?\)
```
Command: Group 1
Timeout: Group 2 (seconds)

### Directory

```regex
(.+)\s*\(in\s+([^\)]+)\)
```
Command: Group 1
Directory: Group 2

### Combined Modifiers

```markdown
**Action**: `npm test` (timeout 120s) (in ./backend)
```
Parse each modifier independently.

## Assert Pattern Parsing

The assert-pattern vocabulary (`Console contains` / `File exists` / `Response status`) is the grammar owner's (§ *Assert Patterns*). The regexes below extract the typed assert and its operands at parse time; evaluation of each against captured evidence is defined in the parent SKILL.

### Console Contains

```regex
Console contains "([^"]+)"(?:\s*\(within\s+(\d+)s?\))?
```
Pattern: Group 1
Timeout: Group 2 (optional, seconds)

### File Exists

```regex
File exists:\s*`?([^`\s]+)`?
```
Path: Group 1

### Response Status

```regex
Response status:\s*(\d+)
```
Status: Group 1

### Custom Assertion

Any other text becomes a custom assertion for human evaluation:
```markdown
**Assert**: Application shows welcome screen
```

## Parsed Task Structure

```json
{
  "id": "T2.4",
  "cycle": 2,
  "task": 4,
  "type": "VERIFY",
  "description": "File watcher detects changes",
  "setup": [
    {"command": "mkdir /tmp/watcher-test"}
  ],
  "actions": [
    {
      "command": "dart run bin/watcher.dart /tmp/watcher-test",
      "modifiers": {"background": true}
    },
    {
      "command": "touch /tmp/watcher-test/test.jsonl",
      "modifiers": {}
    }
  ],
  "asserts": [
    {
      "type": "console_contains",
      "pattern": "FileWatchEvent: created",
      "timeout": null
    }
  ],
  "capture": ["console"],
  "human_review": "Events appear within 1 second"
}
```

## Error Handling

### Missing Required Fields

If `**Action**:` or `**Assert**:` is missing:
- Report a parsing error
- Do not attempt execution
- Ask the human how to proceed

### Malformed Patterns

If modifiers or patterns do not match the expected format:
- Log a warning
- Use the literal text as a fallback
- Include it in the report for human review

### Nested Quotes

Handle escaped quotes in patterns:
```markdown
**Assert**: Console contains "User said \"Hello\""
```
Parse with quote-escaping awareness.

## Legacy Format Support

The legacy marker set is defined by the grammar owner (§ *Legacy Format Support*). This section is the field-mapping mechanic — how to fold a legacy task into the unified structure at parse time.

### HUMAN VERIFICATION Mapping

When parsing a `**HUMAN VERIFICATION**` task, map its fields to the unified format:

| Legacy Field | Unified Field |
|--------------|---------------|
| `Setup:` | `**Setup**:` |
| `Action:` | `**Action**:` |
| `Verify:` | `**Assert**:` |
| `**Human confirms**:` | (ignored by the parser — human confirmation is handled at the checkpoint, not during parsing) |

### Example Legacy Task

```markdown
- [ ] **T2.12**: **HUMAN VERIFICATION** - File watcher detects changes
  - Setup: `mkdir /tmp/test`
  - Action: Run `dart run bin/watcher.dart /tmp/test`
  - Verify: Console outputs "FileWatchEvent: created"
  - **Human confirms**: Events appear in real time ✓
```

Normalized to:

```json
{
  "id": "T2.12",
  "type": "UNIFIED",
  "description": "File watcher detects changes",
  "setup": [{"command": "mkdir /tmp/test"}],
  "actions": [{"command": "dart run bin/watcher.dart /tmp/test", "modifiers": {}}],
  "asserts": [{"type": "console_contains", "pattern": "FileWatchEvent: created"}],
  "capture": ["console"]
}
```

### Format Detection Priority

1. Check for `**TEST:**` (unified) → use as-is
2. Check for `**TEST:VERIFY**` or `**TEST:CONTRACT**` → treat as unified
3. Check for `**HUMAN VERIFICATION**` → map fields to unified format
4. No marker found → reject the task
