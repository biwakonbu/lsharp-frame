# Product Definition

## Vision

L#frame は、GUI を中心に構築された次世代のバイブコーディング支援環境である。

現在の coding agent ecosystem は CLI／TUI 中心であり、複数 session、tool call、approval、artifact、diff、test result、workflow を統合的に扱うには操作性と可視性が不足する。L#frame は terminal を主画面にせず、これらを構造化オブジェクトとして直接 GUI 化する。

## Product identity

- **Editor ではない**: text editor は将来の plugin／surface の一つに過ぎない。
- **Terminal wrapper ではない**: PTY は互換機能および高度な操作 surface として提供する。
- **Web frontend ではない**: HTML／DOM は中核にせず、型付き native UI pipeline を使う。
- **固定 GUI ではない**: layout、commands、keymaps、views、workflow は L# plugin で変更できる。
- **単なる sandbox ではない**: trusted plugin は明示的な capability を通じて OS 機能へ広くアクセスできる。

## Core objects

```text
Workspace
Agent
Session
Run
Message
ToolCall
Approval
Artifact
Diff
TestResult
Plan
Task
Process
PtySession
Notification
```

## Product principles

1. Meaningful objects over terminal text.
2. Keyboard-first but not keyboard-only.
3. Inspectable and replaceable behavior.
4. Native responsiveness under streaming workloads.
5. Safe by default, powerful when explicitly trusted.
6. Stable contracts above replaceable Rust crates.
7. L# owns product behavior; Rust owns native mechanisms.

## Initial vertical slice

```text
Session list
Current agent run
Tool-call timeline
Approval request
Diff / test artifact
Prompt composer
Command palette
Embedded terminal
```

The first release proves:

- one coding agent adapter
- one PTY session
- one L# built-in plugin
- one separately loaded L# plugin
- plugin hot reload
- capability prompt and audit log
- virtualized timeline
- deterministic headless replay
