# Task 26/36: `ProcessSequentialActivityB`

> Rules: [`spec/rules.md`](../rules.md)

Paths and commands below are relative to the generated project root.

| Field | Value |
|-------|-------|
| Language | `Go` |
| Kind | `map` |
| File | `automationservice/internal/functions/automation/processsequentialactivityb.go` |
| Test | `automationservice/internal/functions/automation/processsequentialactivityb_test.go` |
| Service | `Automation Service` |


## Behaviour

Return sequential Activity B's typed result to its Temporal sink.





## Stream types
- Input: `string`
- Output: `string`

## Checklist

- [ ] Read [`spec/rules.md`](../rules.md), especially the `Go` section
- [ ] Open `automationservice/internal/functions/automation/processsequentialactivityb.go` and preserve its generated contract
- [ ] Implement the Go function and propagate the received `context.Context`
- [ ] Run `make test`
- [ ] Implement meaningful assertions in `automationservice/internal/functions/automation/processsequentialactivityb_test.go`
- [ ] Re-read this checklist
- [ ] Append to `spec/progress.md`: `- [x] automationservice/task26.md — ProcessSequentialActivityB — Go — done`