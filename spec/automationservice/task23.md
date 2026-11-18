# Task 23/36: `ProcessScheduledActivity`

> Rules: [`spec/rules.md`](../rules.md)

Paths and commands below are relative to the generated project root.

| Field | Value |
|-------|-------|
| Language | `Go` |
| Kind | `map` |
| File | `automationservice/internal/functions/automation/processscheduledactivity.go` |
| Test | `automationservice/internal/functions/automation/processscheduledactivity_test.go` |
| Service | `Automation Service` |


## Behaviour

Return the visible result of one scheduled Activity execution.





## Stream types
- Input: `string`
- Output: `string`

## Checklist

- [ ] Read [`spec/rules.md`](../rules.md), especially the `Go` section
- [ ] Open `automationservice/internal/functions/automation/processscheduledactivity.go` and preserve its generated contract
- [ ] Implement the Go function and propagate the received `context.Context`
- [ ] Run `make test`

- [ ] Implement meaningful assertions in `automationservice/internal/functions/automation/processscheduledactivity_test.go`
- [ ] Re-read this checklist
- [ ] Append to `spec/progress.md`: `- [x] automationservice/task23.md — ProcessScheduledActivity — Go — done`