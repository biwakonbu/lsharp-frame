# L#frame エージェント向け指示

このファイルは、コーディングエージェントが参照するリポジトリ全体の指示の正本です。
ツール固有のファイルは薄いアダプターに留め、ここにある方針を重複して定義してはいけません。

## プロダクトの目的

L#frame は、コーディングエージェント向けのプログラマブルなネイティブ GUI 環境です。
Rust はネイティブシェル、高スループットなリソース処理、隔離境界を担当し、L# は
Wasmtime 上で L#frame Kernel とプラグイン言語として動作します。

このプロダクトはテキストエディタではなく、TUI を GUI で包むだけのツールでもありません。
セッション、ツール呼び出し、承認、成果物、差分、テスト結果、PTY セッション、ワークフローを
構造化されたドメインオブジェクトとして扱います。

## 出力言語

- ユーザーが明示的に別の言語を指定しない限り、AI が生成する自然言語の出力は日本語にします。
- この規則は、計画、進捗報告、説明、レビュー指摘、引き継ぎ、タスク記録、Issue／PR の本文と
  コメント、コミットメッセージ、生成するドキュメントに適用します。
- ソースコードの識別子、ファイル名、CLI コマンド、プロトコル名、外部 API の正式名称、原文を
  保持すべき引用は翻訳しません。
- コードコメントとリポジトリ内の説明文も、外部エコシステムの規約上英語が必要な場合を除き、
  原則として日本語で記述します。
- 対象ファイルや既存コードが英語であっても、それだけを理由にユーザー向け回答を英語へ
  切り替えてはいけません。

## 指示ファイルの運用

- このルート `AGENTS.md` だけが適用された場合でも、安全性とアーキテクチャを損なわないよう、
  全変更に必須の規則はここへ置きます。
- サブディレクトリの `AGENTS.md` は、その領域だけで役立つ補足に限定します。重要な禁止事項や
  必須規則をスコープ別ファイルだけに置いてはいけません。
- Codex、Claude Code、Cursor による指示の探索と適用は、各ツールが提供する標準機能へ任せます。
- `AGENTS.md` の独自 resolver、scope registry、自動生成、強制的な再読込、ツール間同期 program を
  追加してはいけません。
- Skill と command は、適用済みのリポジトリ指示に従う作業手順だけを定義し、`AGENTS.md` の
  再読込を要求しません。
- ツール固有ファイルは、各ツールが正式に提供する設定形式の範囲で正本へ接続するだけに留めます。

## 権威と必須コンテキスト

1. 現在のユーザー指示またはタスク指示を最優先します。
2. 各ツールの標準機能で適用されたリポジトリ指示に従います。スコープ別指示が適用された場合は、
   ルート指示を上書きする正本ではなく、領域固有の補足として扱います。
3. ADR と規範的なアーキテクチャ文書は、サンプルより優先します。
4. 生成物、ツール固有プロンプト、コードコメントは正本として扱いません。

振る舞いを変更する前に、次を読みます。

- `README.md`
- `docs/architecture.md`
- `docs/backend-abstraction.md`
- 高負荷経路や大量データを変更する場合は `docs/performance.md`
- `docs/adr/` 配下の関連 ADR

Codex を主要な実装エージェントとし、Claude Code と Cursor を補助的に利用します。
変更内容の理解に、特定エージェントの非公開メモリや専用設定を必要としてはいけません。

## 作業手順

1. 提案や変更の前に、リポジトリと既存実装を調査します。
2. 目的、非目標、受入条件、影響する境界を明示します。
3. 公開契約の変更、横断的変更、複数セッションにまたがる作業では、
   `docs/development/tasks/TEMPLATE.md` からタスク記録を作成します。
4. 意図した振る舞いを証明できる、最小で一貫した変更を実装します。
5. 完了と判断する前に、テストを追加または更新します。
6. 実装中は対象を絞った検証を行い、引き継ぎ前に `make ci` を実行します。
7. 最終差分を確認し、アーキテクチャ境界の漏洩、意図しないスコープ拡大、証跡不足を除去します。
8. 未実行の検証は明記します。実行していない検証を成功したように記述してはいけません。

無関係なコードの書き換え、公開契約の暗黙的変更、既存の抽象化で不十分な理由を説明しない
本番依存関係の追加は禁止します。

## アーキテクチャ不変条件

- `lsharp-frame-contract` は、crate に依存しない安定した意味モデルを所有します。
- GUI、レンダラー、非同期ランタイム、PTY、プロセス、OS の外部 crate 型を adapter crate の
  外へ漏らしてはいけません。
- `lsharp-frame-spi` がネイティブ側の Port を定義し、具体的な adapter が実装します。
- `lsharp-frame-core` は契約と Port を調停しますが、GUI toolkit を選択しません。
- `apps/` 配下の composition root が具体的な adapter を選択します。
- L# と Rust は、バージョン付き WIT 契約と粗粒度なバッチで通信します。
- L# プラグインへ OS pointer、file descriptor、GPU object、Rust trait object を渡しません。
- PTY 出力、terminal rendering、大規模 timeline などの高負荷経路はネイティブ側に保持し、
  L# へはバッチ化した制御イベントまたは観測イベントを渡します。
- Capability 検査はネイティブ境界で fail closed にします。
- baseline API は移植可能に保ち、backend 固有機能は明示的なバージョン付き extension capability
  として分離します。
- UI thread は任意のプラグイン処理を同期的に待機してはいけません。
- HTML、CSS、DOM を主要な UI runtime にしません。WebView は任意の surface として扱います。

## 変更種別と必要な証跡

### Contract または WIT の変更

- 互換性への影響と移行方法を記述します。
- Rust contract 型、WIT、L# projection、ドキュメントを同じ変更で更新します。
- round-trip test または conformance test を追加します。
- 確立済みのアーキテクチャ判断を変更する場合は ADR を追加します。

### Core の変更

- イベント順序、Capability 強制、リソースライフサイクル、失敗時の振る舞いを検証します。
- headless backend または fake backend を使った決定的テストを優先します。

### Adapter の変更

- 外部型を adapter 内部に閉じ込めます。
- 共通 conformance suite を実行し、存在しない場合は追加します。
- 未対応 Capability を不正確に模倣せず、未対応として明示します。

### 高負荷経路の変更

- allocation、copy、境界呼び出し、lock の挙動を明示します。
- `docs/performance.md` に定義された benchmark workload を追加または更新します。
- inner loop 内で dynamic dispatch や要素単位の境界呼び出しを行いません。

### L# Kernel またはプラグインの変更

- host effect を明示し、Capability 検査を通します。
- 可能な限り決定的な状態遷移を維持します。
- L# compiler が利用できない場合は、compile／test を未実行として明記します。

## 正規コマンド

ツールごとに独自のコマンド列を作らず、リポジトリのラッパーを使用します。

```text
make help          利用可能なコマンドを表示
make doctor        ローカルツールの利用可否を表示
make context       機密情報を含まないリポジトリコンテキストを表示
make fmt           Rust ソースを整形
make fmt-check     Rust の整形状態を検証
make check         Rust workspace 全体を型検査
make lint          warning を拒否して Clippy を実行
make test          Rust workspace のテストを実行
make wit-check     wasm-tools がある場合に WIT package を検証
make harness       エージェント設定とリポジトリ不変条件を検証
make ci            ローカルの完全検証を実行
make run-headless  決定的な headless composition root を実行
make new-task SLUG=<slug>  複数セッション向けタスク記録を作成
```

## Rust の規約

- ADR で明示的に方針変更しない限り、workspace では `unsafe` を禁止します。
- 安定境界では不透明な文字列よりドメイン固有の error enum を優先します。
- 公開 API は小さく決定的に保ち、実装側の所有権を露出しません。
- 境界では crate 所有オブジェクトではなく ID または型付き resource handle を使います。
- テストは証明する振る舞いの近くに置きます。複数 crate にまたがる integration／conformance test は
  専用 module に分離できます。
- 変更を通すために lint level を緩めてはいけません。問題を修正するか、限定的な allow の理由を
  明記します。
