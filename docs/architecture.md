# Architecture

## Functional core / native shell

```text
Rust Native Shell
  ├─ Desktop backend
  ├─ GPU renderer
  ├─ OS event loop / IME / accessibility
  ├─ Process / PTY / terminal emulator
  ├─ Filesystem / network / secrets
  ├─ ACP / MCP adapters
  ├─ Capability broker
  └─ Wasmtime supervisor
          │
          │ EventBatch / FrameUpdate
          ▼
L#frame Kernel
  ├─ application state
  ├─ command registry
  ├─ keymap and context
  ├─ layout and UI composition
  ├─ workflow orchestration
  ├─ event middleware
  └─ plugin lifecycle
```

Wasm 用語では Rust が host、L# が guest component である。製品用語では L# が L#frame Kernel／Plugin Host である。

## Stable boundaries

### L# ↔ Rust

WIT package `lsharp:frame@0.1.0` が公開契約となる。

- `EventBatch`: host から kernel／plugin へ渡す意味イベント
- `FrameUpdate`: UI transaction、effects、subscriptions、commands
- `EffectRequest`: PTY、process、filesystem、agent、OS capability の要求
- `EffectResult`: 非同期処理の完了イベント

### Rust Core ↔ Native crates

`lsharp-frame-spi` の Ports を adapter が実装する。

```text
DesktopBackend
PtyBackend
ProcessBackend
```

Iced、GPUI、winit、wgpu、Tokio、PTY crate の型を Port に含めてはならない。

## UI pipeline

```text
L# state transition
   ↓
UiTransaction
   ↓
Rust retained UiDocument
   ↓
Desktop adapter
   ↓
layout / shaping / damage tracking
   ↓
GPU
```

L# は毎 frame 呼ばれない。hover、scroll inertia、cursor blink、animation interpolation など、意味状態を変えない処理は native backend 内で完結する。

## High-throughput surfaces

次の surface は通常の UI node と異なり、Rust 側に大容量状態を保持する。

```text
TerminalSurface
LogSurface
DiffSurface
CodeSurface
ImageSurface
GraphSurface
```

L# は stable ID と command を通じて制御する。大量データは component boundary を毎回コピーせず、ring buffer、viewport、revision、range read を用いる。

## PTY data paths

```text
Fast display path:
PTY bytes -> terminal emulator -> terminal surface -> GPU

Control/observation path:
PTY events -> batching/normalization -> L# plugin
```

Plugin は必要に応じて raw bytes、plain text、semantic event を購読できる。raw stream を購読しない plugin に対しては、PTY 全量を L# memory へ複製しない。

## Plugin isolation

初期段階:

```text
Kernel + built-in plugins = one long-lived component
External plugin           = one component per plugin
```

成熟後:

```text
Rust Supervisor
  ├─ Kernel Store
  ├─ Plugin A Store
  ├─ Plugin B Store
  └─ Plugin C Store
```

各 store に fuel、epoch interruption、memory limit、resource quota を設定する。plugin failure は UI thread と他 plugin を停止させない。

## Composition root

Concrete crate の選択は binary crate に限定する。

```text
apps/lsharp-frame-iced
  ├─ DesktopIcedBackend
  ├─ WasmtimeKernelExecutor
  ├─ PortablePtyBackend
  └─ TokioProcessBackend
```

別 backend へ移行する際は composition root と adapter のみを変更する。
