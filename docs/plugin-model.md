# Plugin Model

## Objective

L# plugin を単なる UI DSL にせず、Emacs Lisp package に近い実用上の自由度を持つ desktop extension environment とする。ただし、暗黙のグローバル書き換えではなく、型付き extension point と capability を使う。

## Extension points

```text
Command Registry
Keymap Registry
View Slot Registry
Typed Hook
Event Subscription
Event Middleware
Service Registry
Theme Registry
Workflow Registry
Native Surface Registry
```

## UI levels

### Level 1: Standard widgets

Row、Column、Grid、Split、Tabs、VirtualList、Tree、Table、Text、Button、Input、Markdown、Dialog など。

### Level 2: L#frame surfaces

Terminal、AgentTimeline、DiffViewer、LogViewer、TaskGraph、ArtifactViewer など。heavy state と rendering は Rust 側へ保持する。

### Level 3: Custom canvas

L# から batch display list と hit regions を渡す。GPU object や shader は baseline API へ露出しない。

### Level 4: Native extension

WebView、media、特殊 GPU、OS 固有 API は trusted native helper または statically linked adapter として実装する。外部 extension を Rust ABI の dynamic library として直接ロードすることは避ける。

## Capability classes

### Sandboxed

Marketplace などから導入する通常 plugin。workspace read、agent read、standard UI など、限定 capability のみ。

### Trusted L# plugin

任意 process、PTY、network、clipboard、secret などを明示的に許可できる。任意 process 実行を許可した時点で、ユーザー権限の任意コード実行と同等として表示する。

### Native helper

標準 WIT capability で表せない OS 機能を別 process の typed IPC で提供する。crash と memory corruption を main process から隔離する。

## Capability policy

```text
Deny
Ask every time
Allow once
Allow for workspace
Always allow
```

全 effect は次の audit fact を持つ。

```text
Plugin ID
Command ID
Capability
Arguments summary
Workspace
Timestamp
User decision
Result
```

## Hot reload

```text
compile new component
  -> instantiate in isolated store
  -> validate manifest/capabilities
  -> snapshot old plugin
  -> restore new plugin
  -> validate off-screen UI transaction
  -> atomic swap
  -> keep old instance until success
```

Snapshot は versioned data とし、migration failure 時は旧 instance を継続する。

## Self inspection

L#frame 自身が以下を表示できなければならない。

- command の提供 plugin と現在の override chain
- keybinding resolution
- active hooks and middleware
- registered view slots
- requested and granted capabilities
- event routing trace
- plugin memory／CPU／boundary call metrics
