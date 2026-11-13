# Task 7/36: `FanoutActivityCEndpointSink`

> Rules: [`spec/rules.md`](../rules.md)

Paths and commands below are relative to the generated project root.

| Field | Value |
|-------|-------|
| Language | `Go` |
| Kind | `temporal-sink` |
| File | `automationservice/internal/functions/activity/fanoutactivitycendpointsink.go` |
| Service | `Automation Service` |





## Stream types
- Input: `string`
- Output: `string`
- Error: `error`

## Checklist

- [ ] Read [`spec/rules.md`](../rules.md), especially the `Go` section
- [ ] Open `automationservice/internal/functions/activity/fanoutactivitycendpointsink.go` and preserve its generated contract
- [ ] Implement the Go function and propagate the received `context.Context`
- [ ] Run `make test`
- [ ] Re-read this checklist
- [ ] Append to `spec/progress.md`: `- [x] automationservice/task7.md — FanoutActivityCEndpointSink — Go — done`