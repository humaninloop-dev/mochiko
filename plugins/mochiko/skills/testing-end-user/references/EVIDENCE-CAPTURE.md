# Evidence Capture

> **Scratch paths are role-neutral.** Evidence files use a `verify-` prefix under a scratch directory (`/tmp/claude/`), naming the *work* (verification), not any agent. These are ephemeral runtime artifacts, cleaned up after the checkpoint.

## Table of Contents

- [Overview](#overview)
- [Console Capture](#console-capture)
- [Background Processes](#background-processes)
- [Timeout Handling](#timeout-handling)
- [Screenshot Capture](#screenshot-capture)
- [Log File Capture](#log-file-capture)
- [File State Capture](#file-state-capture)
- [Timing Capture](#timing-capture)
- [Evidence Aggregation](#evidence-aggregation)
- [Cleanup Protocol](#cleanup-protocol)

## Overview

This document defines how to capture, store, and manage evidence during test execution.

## Console Capture

### Standard Output

Capture both stdout and stderr from all commands:

```bash
command 2>&1 | tee /tmp/claude/verify-{task}-output.log
```

### Structured Storage

```
/tmp/claude/
├── verify-T2.4-setup.log      # Setup command output
├── verify-T2.4-action-1.log   # First action output
├── verify-T2.4-action-2.log   # Second action output
└── verify-T2.4-pids.txt       # Background process PIDs
```

### Capture Format

Each log file contains:

```
=== Command ===
{command}

=== Started ===
{timestamp}

=== Output ===
{stdout and stderr}

=== Completed ===
{timestamp}
Duration: {seconds}s
Exit code: {code}
```

## Background Processes

### Starting Background Processes

For actions with the `(background)` modifier:

```bash
# Start process and capture PID
{command} > /tmp/claude/verify-{task}-bg-{n}.log 2>&1 &
echo $! >> /tmp/claude/verify-{task}-pids.txt
```

### Tracking PIDs

PID file format (`-pids.txt`):
```
12345 npm start
12346 python server.py
```

### Reading Background Output

To check background process output for assertions:

```bash
tail -n 100 /tmp/claude/verify-{task}-bg-{n}.log
```

### Cleanup

After test completion (pass or fail):

```bash
# Kill all tracked processes
while read pid cmd; do
  kill $pid 2>/dev/null
done < /tmp/claude/verify-{task}-pids.txt

# Remove temp files
rm -f /tmp/claude/verify-{task}-*.log
rm -f /tmp/claude/verify-{task}-pids.txt
```

## Timeout Handling

### Default Timeouts

| Scope | Default | Override |
|-------|---------|----------|
| Single action | 60s | `(timeout Ns)` modifier |
| Total test | 300s | Not overridable |

### Timeout Implementation

```bash
timeout ${seconds}s {command}
exit_code=$?

if [ $exit_code -eq 124 ]; then
  echo "TIMEOUT after ${seconds}s"
fi
```

### Partial Output on Timeout

When a command times out:

1. Capture whatever output was produced
2. Kill the process
3. Mark as TIMEOUT status
4. Include partial output in the report

## Screenshot Capture

### Platform Detection

```bash
# Detect platform
if [[ "$OSTYPE" == "darwin"* ]]; then
  SCREENSHOT_CMD="screencapture"
elif command -v import &> /dev/null; then
  SCREENSHOT_CMD="import"  # ImageMagick
elif command -v gnome-screenshot &> /dev/null; then
  SCREENSHOT_CMD="gnome-screenshot"
else
  SCREENSHOT_CMD=""
fi
```

### Capture Commands

| Platform | Command |
|----------|---------|
| macOS | `screencapture -x /tmp/claude/verify-{task}-screenshot.png` |
| Linux (X11) | `import -window root /tmp/claude/verify-{task}-screenshot.png` |
| Linux (GNOME) | `gnome-screenshot -f /tmp/claude/verify-{task}-screenshot.png` |

### Graceful Fallback

If screenshot capture fails:
1. Log a warning
2. Continue test execution
3. Note "Screenshot unavailable" in the report

## Log File Capture

### Reading Specified Logs

For `**Capture**: logs(/var/log/app.log)`:

```bash
tail -n 500 /var/log/app.log > /tmp/claude/verify-{task}-applog.log
```

### Log Rotation Awareness

If the log file might rotate during the test:

```bash
# Capture at start
cp /var/log/app.log /tmp/claude/verify-{task}-applog-start.log

# Capture at end
tail -n 500 /var/log/app.log > /tmp/claude/verify-{task}-applog-end.log
```

## File State Capture

### File Existence

```bash
if [ -f "{path}" ]; then
  echo "EXISTS: {path}"
  ls -la "{path}"
else
  echo "NOT FOUND: {path}"
fi
```

### File Content (if relevant)

```bash
if [ -f "{path}" ] && [ -s "{path}" ]; then
  head -n 50 "{path}"
fi
```

### Directory State

```bash
ls -la {directory}
```

## Timing Capture

### Per-Action Timing

```bash
start_time=$(date +%s.%N)
{command}
end_time=$(date +%s.%N)
duration=$(echo "$end_time - $start_time" | bc)
echo "Duration: ${duration}s"
```

### Total Test Timing

Track from the first setup command to the final assert evaluation.

## Evidence Aggregation

### Evidence Summary Structure

```json
{
  "task_id": "T2.4",
  "total_duration": "12.5s",
  "setup": {
    "duration": "0.3s",
    "output": "...",
    "status": "success"
  },
  "actions": [
    {
      "command": "dart run bin/watcher.dart",
      "type": "background",
      "pid": 12345,
      "log_file": "/tmp/claude/verify-T2.4-bg-1.log"
    },
    {
      "command": "touch /tmp/watcher-test/test.jsonl",
      "duration": "0.1s",
      "output": "",
      "exit_code": 0
    }
  ],
  "asserts": [
    {
      "pattern": "Console contains \"FileWatchEvent: created\"",
      "status": "PASS",
      "matched_line": "FileWatchEvent: created /tmp/watcher-test/test.jsonl"
    }
  ],
  "files": {
    "console": "/tmp/claude/verify-T2.4-output.log",
    "pids": "/tmp/claude/verify-T2.4-pids.txt"
  }
}
```

## Cleanup Protocol

### On Success

1. Stop background processes
2. Remove all temp files
3. Keep the summary in memory for the report

### On Failure

1. Stop background processes
2. **Keep logs for debugging**
3. Report log locations to the user
4. Cleanup after the human reviews

### On Abort

1. Stop background processes immediately
2. Keep logs for investigation
3. Report partial state
