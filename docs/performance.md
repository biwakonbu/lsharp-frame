# Performance Contract

## Goal

HTML／DOM を使わないこと自体を性能保証とはしない。用途特化した event-driven retained UI pipeline と native fast path を受入条件にする。

## Rules

1. L# を frame loop に入れない。
2. one event batch -> one kernel dispatch を原則とする。
3. one dispatch -> one UI transaction を原則とする。
4. unchanged node を rebuild／reshape しない。
5. terminal／log／diff の large state は native surface に保持する。
6. UI thread で plugin の completion を待たない。
7. idle 時は render、kernel dispatch、polling を行わない。
8. component boundary へ large payload を毎回コピーしない。

## Initial budgets

以下は実測済み性能ではなく、prototype の受入予算である。

| Metric | Budget |
|---|---:|
| Typical L# dispatch | p95 < 2 ms |
| UI patch + layout preparation | p95 < 4 ms |
| Input-to-present at 60 Hz | < 16.7 ms |
| Stretch target at 120 Hz | < 8.3 ms |
| Component calls per event batch | 1 by default |
| Idle CPU | < 0.5% |
| GC pause | p99 < 5 ms |
| Timeline items | 100,000 virtualized |
| Plugin failure | host and other plugins continue |

## Canonical workloads

```text
100,000 agent events
10,000 event appends / second burst
20 MiB PTY output
10 concurrent agent sessions
20 L# plugins
1,000 command invocations
100 plugin hot reloads
long-lived 8 hour soak
```

## Metrics

```text
event_queue_time
kernel_dispatch_time
canonical_abi_time
ui_patch_time
layout_time
text_shape_time
gpu_submit_time
present_latency
gc_pause_time
plugin_memory_bytes
boundary_bytes_in/out
```

## Comparison

Prototype phase では同一 event trace を Headless backend、native Desktop backend、必要に応じて WebView prototype へ replay し、frame time、CPU、RSS、startup、boundary bytes で比較する。印象だけで backend を選ばない。
