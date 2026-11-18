# Task 27/36: `ProcessWorkflowJob`

> Rules: [`spec/rules.md`](../rules.md)

Paths and commands below are relative to the generated project root.

| Field | Value |
|-------|-------|
| Language | `Go` |
| Kind | `map` |
| File | `automationservice/internal/functions/automation/processworkflowjob.go` |
| Test | `automationservice/internal/functions/automation/processworkflowjob_test.go` |
| Service | `Automation Service` |


## Behaviour

Continue the Workflow as new once, then return its final result.





## Stream types
- Input: `string`
- Output: `string`

## Checklist

- [ ] Read [`spec/rules.md`](../rules.md), especially the `Go` section
- [ ] Open `automationservice/internal/functions/automation/processworkflowjob.go` and preserve its generated contract
- [ ] Implement the Go function and propagate the received `context.Context`
- [ ] Run `make test`

- [ ] Implement meaningful assertions in `automationservice/internal/functions/automation/processworkflowjob_test.go`
- [ ] Re-read this checklist
- [ ] Append to `spec/progress.md`: `- [x] automationservice/task27.md — ProcessWorkflowJob — Go — done`