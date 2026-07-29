# Roadmap

## Milestone 0 — Contracts

- [x] Rust workspace skeleton
- [x] UI IR / Event / Effect / Capability model
- [x] Desktop / PTY / Process SPI
- [x] Headless backend
- [x] Architecture and performance contract
- [ ] WIT parser／binding validation
- [ ] contract compatibility test

## Milestone 1 — L# runtime bridge

- [ ] Wasmtime kernel executor
- [ ] L# Component `activate` / `dispatch`
- [ ] EventBatch and FrameUpdate canonical ABI
- [ ] serialized component cache
- [ ] memory/fuel/epoch limits
- [ ] L# SDK primitives

## Milestone 2 — Native desktop vertical slice

- [ ] first Desktop Adapter
- [ ] window / input / IME / clipboard / accessibility
- [ ] retained UI document and damage tracking
- [ ] Split / Tabs / VirtualList / Text / Button / Input
- [ ] command palette and keymap
- [ ] headless-to-desktop trace replay

## Milestone 3 — Process and PTY

- [ ] process pipe backend
- [ ] PTY backend for macOS/Linux/Windows
- [ ] terminal emulator and GPU surface
- [ ] raw/text/semantic subscriptions
- [ ] bounded ring buffer and range reads
- [ ] PTY conformance suite

## Milestone 4 — Coding agent experience in L#frame

- [ ] ACP client
- [ ] session list
- [ ] run timeline
- [ ] tool-call approval
- [ ] diff / test result artifact surface
- [ ] prompt composer
- [ ] structured agent event store

## Milestone 5 — Plugin platform

- [ ] plugin manifest and dependency resolver
- [ ] per-plugin Wasmtime Store
- [ ] capability permission UI
- [ ] audit log
- [ ] snapshot/restore and hot reload
- [ ] plugin inspector
- [ ] two independently developed plugins

## Milestone 6 — Performance gate

- [ ] benchmark suite
- [ ] 8-hour soak
- [ ] 100k timeline workload
- [ ] 20 MiB PTY burst workload
- [ ] 20-plugin workload
- [ ] native vs WebView prototype comparison
- [ ] startup and idle CPU budget enforcement
