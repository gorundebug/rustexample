# Task 1/36: `ActivityJobEndpointSink`

> Rules: [`spec/rules.md`](../rules.md)

Paths and commands below are relative to the generated project root.

| Field | Value |
|-------|-------|
| Language | `Go` |
| Kind | `temporal-sink` |
| File | `automationservice/internal/functions/activity/activityjobendpointsink.go` |
| Service | `Automation Service` |





## Stream types
- Input: `string`
- Output: `string`
- Error: `error`

## Checklist

- [ ] Read [`spec/rules.md`](../rules.md), especially the `Go` section
- [ ] Open `automationservice/internal/functions/activity/activityjobendpointsink.go` and preserve its generated contract
- [ ] Implement the Go function and propagate the received `context.Context`
- [ ] Run `make test`
- [ ] Re-read this checklist
- [ ] Append to `spec/progress.md`: `- [x] automationservice/task1.md — ActivityJobEndpointSink — Go — done`