# 開発ハーネス

- Slug: `development-harness`
- 作成日: `2026-07-29`
- 状態: 完了
- 担当者／エージェント: Codex 互換ハーネスの初期構築

## 目的

Codex、Claude Code、Cursor、通常のシェル利用者が、1つのアーキテクチャ方針、再利用可能な
ワークフロー Skill、共通の検証コマンド、証跡ベースの引き継ぎ手順を共有できる、リポジトリ所有の
開発ハーネスを提供します。Codex を主要な実装経路とします。

また、ユーザーが別言語を明示しない限り、全エージェントの自然言語出力を日本語に統一します。

## 非目標

- ネイティブ GUI、Wasmtime runtime、PTY backend、agent protocol の実装。
- L# version manager または package 配布システムの設計。
- 個人のモデル選択、認証情報、MCP endpoint、マシン固有設定のコミット。
- Codex、Claude Code、Cursor の指示探索を統一する独自 resolver、scope registry、設定生成、
  強制再読込、同期 program の実装。

## コンテキストと正本

- 関連するアーキテクチャ文書: `docs/architecture.md`、`docs/backend-abstraction.md`、
  `docs/performance.md`
- 関連する ADR: `docs/adr/0001-native-host-lsharp-kernel.md`、
  `docs/adr/0002-stable-ui-ir.md`、`docs/adr/0003-capability-resource-boundary.md`
- 既存実装／テスト: `main` 上の初期 L#frame Rust／L#／WIT scaffold

## 影響する境界

- Rust contract／core／adapter: プロダクト contract は変更せず、アーキテクチャ漏洩の静的検査を追加。
- WIT／L#: 公開 contract は変更せず、スコープ別エージェント指示を補足として追加。
- Capability／resource lifecycle: runtime 変更なし。
- 性能／高負荷経路: runtime 変更なし。

## 受入条件

- [x] ルート `AGENTS.md` が、全変更に必須の規則を保持する方針の正本である。
- [x] スコープ別 `AGENTS.md` は領域固有の補足に限定され、そこだけに必須規則を置かない。
- [x] 指示探索は各ツールの標準機能へ任せ、独自 resolver や設定生成を導入していない。
- [x] Skill と command が `AGENTS.md` の再読込を要求せず、作業手順だけを定義している。
- [x] Claude Code と Cursor が、重複したアーキテクチャ方針ではなく薄い adapter を使用する。
- [x] 個人のモデル／provider／認証設定を含めずに Codex の project default をコミットしている。
- [x] Codex と Claude Code が plan／implement／verify／review／handoff の同一 Skill を公開し、
      drift を `make harness` で検出する。
- [x] Cursor が同等の command を提供する。
- [x] 全ツールが context、検証、テスト、引き継ぎ準備に同じ `make` target を使用する。
- [x] Rust compiler がなくても、設定構文、link、workspace member、安定 contract の独立性を検証する。
- [x] `make harness` が各ツールの指示読込を再実装せず、静的な構成検査に限定されている。
- [x] ユーザー指定の例外を除き、AI の自然言語出力を日本語とする規則を自己検証する。
- [x] CI が固定 Rust workspace gate より先にハーネスを実行する。
- [x] PR、タスク、引き継ぎ template が正確な証跡と未実行検証の開示を要求する。

## 実装計画

1. ルートに必須規則を集約し、スコープ別指示ファイルを補足として作成する。
2. Codex、Claude Code、Cursor の標準機能を利用する薄い adapter と Skill／command を追加する。
3. Makefile wrapper と、動的な指示解決を行わない静的ハーネス検証 script を追加する。
4. タスク、引き継ぎ、完了の定義、PR template を追加する。
5. Rust toolchain を固定し、ハーネスと Rust の検証を CI で実行する。
6. ハーネス全体を日本語化し、日本語出力規則を検証対象へ追加する。
7. Skill から `AGENTS.md` の重複読込指示を除去し、標準機能優先の方針を明文化する。

## 検証

```text
make harness                                      PASS
make doctor                                       PASS（診断専用。不足ツールを表示）
git diff --check                                  PASS
GitHub Actions CI run 30521539313                 PASS
  エージェントハーネス                            PASS
  Rust workspace 1.97.1: fmt/check/clippy/test    PASS
  Rust MSRV 1.88.0: cargo check                   PASS
wasm-tools component wit wit/frame.wit --json     未実行: このタスクでは WIT を変更していない
L# compile/test                                    未実行: lsharp が利用できない
```

## 判断事項とリスク

- 判断: リポジトリ全体の必須方針はルート `AGENTS.md` に置き、スコープ別ファイルは補足に限定する。
- 判断: 指示探索と適用は各ツールの標準機能に任せ、独自 resolver や設定生成は実装しない。
- 判断: Skill／command は適用済み指示を前提とし、`AGENTS.md` の再読込を要求しない。
- 判断: Codex／Claude は Agent Skill、Cursor は command を共通ワークフローに使用する。
- 判断: AI の自然言語出力は、ユーザー指定の例外を除き日本語にする。
- 判断: `make` をエージェントと人間の共通コマンド面にする。
- 判断: `make harness` は静的な構成と不変条件だけを検査し、ツールの runtime behavior を再現しない。
- 判断: 明示的な upgrade 変更がレビューされるまで Rust を `1.97.1` に固定する。
- リスク: ツール固有の Skill、permission、command schema は変化し得る。標準機能だけを使う薄い
  adapter に限定することで、アップデート時の修正範囲を小さく保つ。

## 引き継ぎ状態

- 現在の状態: 実装、静的検証、固定 Rust 検証、MSRV 検証、日本語化、標準機能優先方針の整理が完了。
- 変更したファイル: エージェント設定／Skill、Makefile／script、開発文書／template、CI／toolchain。
- 実行した検証: repository harness、shell／JSON／TOML／link、Rust fmt／check／clippy／test、MSRV check。
- 未実行の検証と理由: WIT は未変更。実行環境に L# compiler がない。
- 次の具体的な作業: 現在 HEAD の CI を確認し、PR #1 をレビューして merge する。
