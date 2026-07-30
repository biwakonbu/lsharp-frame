# 開発エージェントハーネス

## 目的

L#frame は Codex、Claude Code、Cursor を併用して開発します。Codex を既定の実装エージェントと
しますが、リポジトリ知識は3つのツール間で移植可能でなければなりません。そのため、ハーネスは
ツール非依存の方針を1つだけ正本として持ち、ツール固有ファイルを薄いアダプターとして扱います。

ユーザーが別言語を明示しない限り、各エージェントが生成する計画、進捗、説明、レビュー、
引き継ぎ、Issue／PR 文面、コミットメッセージ、ドキュメントは日本語に統一します。

## 基本方針

- 全変更に必須の安全規則とアーキテクチャ不変条件は、ルート `AGENTS.md` に集約します。
- サブディレクトリの `AGENTS.md` は領域固有の補足であり、そこだけに必須規則を置きません。
- 指示の探索と適用は、Codex、Claude Code、Cursor が正式に提供する標準機能へ任せます。
- ツール間の挙動を完全に一致させるための独自 resolver、scope registry、自動生成、強制再読込、
  同期 program は実装しません。
- Skill と command は、既に適用されたリポジトリ指示を前提に、作業手順だけを追加します。
- `make harness` はファイル構成と静的な不変条件を検証しますが、各ツールの指示読込を再実装したり
  シミュレートしたりしません。

ツールのアップデートで標準仕様が変わった場合は、該当する薄いアダプターだけを修正します。
独自基盤を維持することで追従する方針は取りません。

## 指示の階層

```text
AGENTS.md                         リポジトリ全体の必須方針の正本
<area>/AGENTS.md                  crates/apps/kernel/plugins/wit/docs の補足方針
CLAUDE.md                         Claude Code 用の薄い import adapter
<area>/CLAUDE.md                  Claude Code 用のスコープ別 import adapter
.cursor/rules/*.mdc               Cursor の標準 activation／glob adapter
.codex/config.toml                プロジェクト実行既定値。アーキテクチャ方針は置かない
```

現在のタスク、リポジトリ内のファイル、テスト、記録済みの判断を正本とします。実装や引き継ぎを、
非公開のチャットメモリへ依存させてはいけません。

## 再利用可能なワークフロー Skill

Codex と Claude Code には同じ5つの Agent Skill を提供します。

```text
.agents/skills/lsharp-frame-plan
.agents/skills/lsharp-frame-implement
.agents/skills/lsharp-frame-verify
.agents/skills/lsharp-frame-review
.agents/skills/lsharp-frame-handoff

.claude/skills/<同じ名前>
```

Codex はリポジトリ標準の `.agents/skills` を使用し、Claude Code は `.claude/skills` を使用します。
両方の `SKILL.md` は意図的に同一内容とし、差分が生じた場合は `make harness` を失敗させます。
Cursor はリポジトリ形式が異なるため、同等のワークフローを `.cursor/commands/*.md` で提供します。

Skill と command には `AGENTS.md` の再読込を指示しません。リポジトリ方針の適用は各ツールの
標準機能に任せ、Skill と command は計画、実装、検証、レビュー、引き継ぎの手順だけを定義します。

## 正規ワークフロー

```text
調査
  -> 目的／非目標／受入条件を定義
  -> 横断的な作業ではタスク記録を作成
  -> 最小で一貫した単位を実装
  -> 対象を絞った検証を実行
  -> make harness
  -> make ci
  -> 差分をレビュー
  -> 証跡付きで引き継ぎ
```

安定契約の変更、複数 subsystem にまたがる変更、複数セッションを要する作業、明示的な移行証跡や
benchmark 証跡が必要な作業では `make new-task SLUG=<slug>` を使用します。小規模で局所的な修正には
タスク記録を必須としません。

## ツールの役割

### Codex

Codex を主要な実装経路とします。`AGENTS.md` と `.agents/skills` の標準的な読込機構を使用し、
`.codex/config.toml` のプロジェクト既定値を適用します。Codex の指示探索を補う独自 program は
導入しません。モデル、provider、認証の個人設定はコミットしません。

### Claude Code

`CLAUDE.md` が正本の指示を import します。`.claude/skills` が共通ワークフロー Skill を提供し、
`.claude/settings.json` は読み取り専用 Git コマンドとリポジトリ検証コマンドを限定的に許可し、
破壊的な Git／shell コマンドと機密情報らしいファイルを拒否します。ユーザー固有の権限変更は、
ignore 済みの `.claude/settings.local.json` に置きます。

### Cursor

`.cursor/rules/*.mdc` が Cursor 標準の `alwaysApply` と `globs` によって必要な方針を適用します。
`.cursor/commands/` はアーキテクチャを再定義せず、計画、実装、検証、レビュー、引き継ぎの
共通ワークフローを提供します。rule の生成や独自 scope 解決は行いません。

## リポジトリコマンド

全エージェントは `make` target を使用し、ツール間で検証手順が分岐しないようにします。

```text
make doctor
make context
make harness
make fmt-check
make check
make lint
make test
make wit-check
make ci
```

`make doctor` は診断専用であり、任意ツールが利用できなくても失敗しません。`make harness` は
Rust compiler に依存せず、設定、指示ファイルの構成、日本語出力規則、Skill parity、Markdown link、
workspace member、安定 contract への実装型漏洩を静的に検証します。変更対象の path に応じた
指示解決や、各ツールの runtime behavior の再現は対象外です。

## エージェント間の引き継ぎ

別のエージェントまたはセッションが作業を継続する場合は、[引き継ぎテンプレート](handoffs/TEMPLATE.md)
を使用します。有効な引き継ぎには、実行した検証と未実行の検証を正確に記載し、意図の要約だけで
終わらせません。

## ローカル専用設定

次をコミットしてはいけません。

```text
.claude/settings.local.json
.cursor/mcp.json
.mcp.json
.env*
.codex-log/
```

秘密情報、個人のモデル設定、MCP 認証情報、マシン固有 path をリポジトリへ含めません。
