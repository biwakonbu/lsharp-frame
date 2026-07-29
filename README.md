# L#frame

L#frame は、**L# で画面・コマンド・ワークフローを拡張できる、AI コーディングエージェント向けのプログラマブルなネイティブ GUI 環境**です。

目標は「ターミナルを GUI で包むこと」ではありません。Agent session、tool call、approval、diff、test result、artifact、PTY session などを構造化されたオブジェクトとして扱い、利用者が L# プラグインで表示方法と操作方法を組み替えられる環境を作ります。

## Naming

- Product: `L#frame`
- Repository and package names: `lsharp-frame`
- Rust identifiers: `lsharp_frame`
- WIT package: `lsharp:frame`

## Product statement

> Rust がネイティブ GUI、GPU、OS リソース、PTY、プロセス、プラグイン隔離を担当し、L# は Wasmtime 上で動く L#frame Kernel／プラグイン言語として、アプリケーション状態、UI 構成、コマンド、キーマップ、ワークフローを所有する。

Emacs から継承するのはテキストエディタ機能ではなく、次の性質です。

- すべての操作が名前付き command になる
- UI、keymap、hook、workflow を実行中に検査・拡張できる
- subprocess、pipe、PTY、filesystem、network、OS capability をプラグインから利用できる
- プラグインを hot reload し、失敗時は以前の instance へ戻せる
- 拡張点と権限が型付きかつ自己記述的である

## Architecture

```text
┌──────────────────────────────────────────────────────┐
│ Rust Native Host                                     │
│                                                      │
│ Window / GPU / Layout / Rendering / IME / A11y      │
│ Process / Pipe / PTY / Terminal / File / Network    │
│ Capability Broker / Resource Registry                │
│ Wasmtime Component Supervisor                        │
└────────────────────────┬─────────────────────────────┘
                         │ WIT contracts
                         │ EventBatch / UiTransaction / Effect
┌────────────────────────▼─────────────────────────────┐
│ L#frame Kernel                                       │
│                                                      │
│ State / Commands / Keymaps / Hooks / Workflows      │
│ UI composition / Plugin lifecycle / Event routing    │
└────────────────────────┬─────────────────────────────┘
                         │ typed extension points
             ┌───────────┼───────────┐
             ▼           ▼           ▼
       Agent plugin   PTY plugin   Review plugin
```

HTML、CSS、DOM は中核にしません。L# が L#frame 固有の型付き UI IR を生成し、Rust の Desktop Adapter が Iced、GPUI、または将来の別実装へ変換して GPU 描画します。WebView は必要な画面だけを表示する任意の `NativeSurface` として扱います。

## Repository status

このリポジトリは architecture-first の初期スキャフォールドです。現在含まれるもの:

- crate 非依存の UI IR、Event、Effect、Capability 契約
- Desktop、PTY、Process 用の Rust SPI
- L# executor を差し替え可能にする L#frame Core
- 決定的な Headless Desktop Adapter
- WIT v0.1 契約の設計ドラフト
- L# kernel / plugin の source skeleton
- 性能予算、plugin model、backend 交換条件、roadmap

まだ含まれないもの:

- Wasmtime binding と実際の L# Component 起動
- Iced / GPUI Desktop Adapter
- PTY / terminal emulator の実装
- ACP client
- plugin package manager

## Quick start

```bash
make doctor
make harness
make ci
make run-headless
```

`lsharp-frame-headless` は OS window を作らず、同じ EventBatch → Kernel → UiTransaction 経路を実行します。UI backend に依存しない contract test、replay test、benchmark の正本として使います。

## Development harness

Codex を主要な実装エージェントとし、Claude Code と Cursor も同じ正本を参照します。
リポジトリ全体の規則は [`AGENTS.md`](AGENTS.md)、領域別の規則は各ディレクトリの
`AGENTS.md` に置き、ツール固有設定にはアーキテクチャ規則を複製しません。

```bash
make help
make context
make new-task SLUG=example-cross-cutting-change
```

- [Agent harness](docs/development/agent-harness.md)
- [Definition of done](docs/development/definition-of-done.md)
- [Contributing](CONTRIBUTING.md)

## Workspace layout

```text
crates/lsharp-frame-contract         crate非依存の公開意味モデル
crates/lsharp-frame-spi              Native backend向けPorts
crates/lsharp-frame-core             Kernel実行・権限検査・UI transaction適用
crates/lsharp-frame-adapter-headless 決定的なテスト用Desktop Adapter
apps/lsharp-frame-headless           最小のcomposition root
wit/                                 L# Component向けWIT契約
kernel/                              L#frame Kernel skeleton
plugins/                             L# plugin examples
docs/                                Architecture / ADR / roadmap
```

## Non-goals

- IDE やテキストエディタを最初から再実装すること
- ANSI/TUI の画面解析を主要 Agent protocol にすること
- Iced、winit、wgpu、PTY crate の型を公開 API へ漏らすこと
- plugin ごとに毎 frame UI tree 全体を再生成すること
- L# から OS pointer、file descriptor、GPU object を直接操作させること

## Design invariants

1. `lsharp-frame-contract` は外部 GUI／async／PTY crate に依存しない。
2. L# plugin API には crate 名、OS handle、GPU 型を露出しない。
3. Native crate の交換時に `plugins/**/*.ls` と WIT baseline を変更しない。
4. UI thread は L# plugin の実行完了を待たない。
5. PTY の高速表示経路と L# の制御・観測経路を分離する。
6. 大量リスト、ログ、timeline は必ず virtualize する。
7. Capability 未許可の Effect は host boundary で fail closed にする。
8. backend 固有機能は baseline API ではなく versioned extension に隔離する。

## Documentation

- [Product definition](docs/product-definition.md)
- [Architecture](docs/architecture.md)
- [Plugin model](docs/plugin-model.md)
- [Backend abstraction](docs/backend-abstraction.md)
- [Performance contract](docs/performance.md)
- [Roadmap](docs/roadmap.md)

## License

MIT
