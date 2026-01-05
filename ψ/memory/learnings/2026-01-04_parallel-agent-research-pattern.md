# Learning: Parallel Agent Research Pattern

> Discovered: 2026-01-04 | Context: voice-tray-v2 onboarding

## Pattern

When exploring unknown territory or reconstructing history, spawn N agents (5-10) with different search vectors. Combine results.

## Implementation

```
Agent 1: claude-mem query A
Agent 2: claude-mem query B
Agent 3: Oracle search
Agent 4: Oracle DB direct
Agent 5: Git history
Agent 6: Codebase analysis
...
```

Launch all in parallel with `run_in_background: true`, then collect with `TaskOutput`.

## When to Use

- New repo onboarding
- Deep context gathering
- History reconstruction
- Multi-source research

## Benefits

- 10 agents × 5 min = 50 agent-minutes in 5 wall-clock minutes
- Different angles catch different info
- Redundancy validates findings
- Faster than serial exploration

## Concepts

`research`, `parallel-agents`, `context-gathering`, `onboarding`
