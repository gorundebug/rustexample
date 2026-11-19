# Task 8/36: `FanoutActivityCEndpointSource`

> Rules: [`spec/rules.md`](../rules.md)

Paths and commands below are relative to the generated project root.

| Field | Value |
|-------|-------|
| Language | `Go` |
| Kind | `temporal-source` |
| File | `automationservice/internal/functions/activity/fanoutactivitycendpointsource.go` |
| Service | `Automation Service` |





## Stream types
- Emits into graph: `string`
- Receives graph result: `string`
- Error: `error`

## Checklist

- [ ] Read [`spec/rules.md`](../rules.md), especially the `Go` section
- [ ] Open `automationservice/internal/functions/activity/fanoutactivitycendpointsource.go` and preserve its generated contract
- [ ] Implement the Go function and propagate the received `context.Context`
- [ ] Run `make test`
- [ ] Re-read this checklist
- [ ] Append to `spec/progress.md`: `- [x] automationservice/task8.md — FanoutActivityCEndpointSource — Go — done`