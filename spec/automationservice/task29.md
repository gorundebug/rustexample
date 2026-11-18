# Task 29/36: `ScheduledWorkflowPause`

> Rules: [`spec/rules.md`](../rules.md)

Paths and commands below are relative to the generated project root.

| Field | Value |
|-------|-------|
| Language | `Go` |
| Kind | `delay` |
| File | `automationservice/internal/functions/automation/scheduledworkflowpause.go` |
| Test | `automationservice/internal/functions/automation/scheduledworkflowpause_test.go` |
| Service | `Automation Service` |


## Behaviour

Use the official Temporal Workflow timer for a scheduled Workflow.





## Stream types
- Input: `string`
- Output: `string`

## Checklist

- [ ] Read [`spec/rules.md`](../rules.md), especially the `Go` section
- [ ] Open `automationservice/internal/functions/automation/scheduledworkflowpause.go` and preserve its generated contract
- [ ] Implement the Go function and propagate the received `context.Context`
- [ ] Run `make test`

- [ ] Implement meaningful assertions in `automationservice/internal/functions/automation/scheduledworkflowpause_test.go`
- [ ] Re-read this checklist
- [ ] Append to `spec/progress.md`: `- [x] automationservice/task29.md — ScheduledWorkflowPause — Go — done`