# Backend Abstraction

## Replacement target

目標は「crate 交換時の作業をゼロにする」ことではない。

> crate 交換時の変更を adapter crate と composition root に限定し、L# plugin、WIT baseline、L#frame domain model を変更しない。

## Dependency rule

```text
L# plugins
    ↓
WIT contracts
    ↓
lsharp-frame-contract
    ↓
lsharp-frame-core / lsharp-frame-spi
    ↓
adapters
    ↓
external crates / OS
```

逆方向依存は禁止する。

## No foreign types

以下のような型を公開境界へ出してはならない。

```text
iced::Element
winit::WindowId
wgpu::Texture
wgpu::Device
tokio::process::Child
portable_pty::MasterPty
anyhow::Error
```

L#frame が所有する ID、commands、events、errors へ変換する。

## Port shape

非同期 backend は async runtime 固有の future／channel を公開せず、command/event model を基本とする。

```text
submit(command)
drain_events(limit)
```

これにより Tokio、専用 thread、kqueue/epoll、IOCP などを adapter 内部で自由に選べる。

## Desktop backend

GUI toolkit は application lifecycle と widget model を所有するため、細かい trait の集合ではなく Desktop Backend を縦に交換する。

```text
DesktopIcedBackend
  ├─ Iced
  ├─ winit
  ├─ renderer
  ├─ IME
  └─ accessibility

DesktopGpuiBackend
  ├─ GPUI lifecycle
  ├─ renderer
  ├─ IME
  └─ accessibility
```

両者は同じ `UiDocument`、`InputEvent`、semantic command を扱う。

## Portable baseline and extensions

```text
ui-core@1
ui-canvas@1
ui-terminal@1
ui-accessibility@1
ui-multi-window@1
ui-native-surface@1
```

baseline を最小公倍数へ肥大化させず、backend 固有機能は versioned extension と capability negotiation で表す。

## Conformance

Port を実装する adapter は共通 conformance suite を通す。

PTY example:

- raw bytes を破壊しない
- output ordering を維持する
- cwd／env／resize／interrupt／exit status が契約どおり
- high-volume output で deadlock しない
- drop／close で resource を解放する
- backpressure policy が機械可読である

Desktop example:

- same UI IR produces equivalent semantic tree
- focus traversal and IME events normalize identically
- virtual list materializes viewport only
- accessibility tree is generated
- event trace can be replayed headlessly

## Architecture tests

1. `lsharp-frame-contract` に external dependency がない。
2. adapter crate 以外に concrete toolkit 名が現れない。
3. WIT に OS handle／crate type／shader language が現れない。
4. backend 交換時に `plugins/**/*.ls` を変更しない。
5. fake backend だけで kernel tests を実行できる。
