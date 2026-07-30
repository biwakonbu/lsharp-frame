# L#frame へのコントリビューション

## 最初に読むもの

コードを変更する前に、ルートの [`AGENTS.md`](AGENTS.md)、最も近いスコープ別 `AGENTS.md`、
関連するアーキテクチャ文書と ADR を読みます。Codex を主要な実装経路としますが、コミットする
指示と検証は Claude Code、Cursor、通常のシェルからも利用可能でなければなりません。

AI が生成する計画、説明、レビュー、引き継ぎ、Issue／PR 文面、コミットメッセージは、
ユーザーが別言語を明示しない限り日本語で記述します。

## 正規ワークフロー

```bash
make doctor
make context
# 横断的または複数セッションにまたがる作業の場合:
make new-task SLUG=short-kebab-case-name

# 対象を絞った検証を行いながら実装し、最後に:
make harness
make ci
```

実行していないコマンドを成功したと記述してはいけません。利用できない検証と、実行できない
環境上の正確な理由を PR または引き継ぎへ記録します。

## 中核となる境界規則

外部 crate の型を `lsharp-frame-contract`、WIT、L# plugin API に含めてはいけません。
crate 固有の変換は adapter crate 内部に閉じ込め、具体的な adapter の選択は application の
composition root に限定します。

## 変更区分

- **Contract 変更**: `lsharp-frame-contract` または `wit/`。互換性、移行、round-trip／conformance
  の証跡と、Rust／L# ドキュメントの同時更新が必要です。
- **Core 変更**: event routing、Capability 強制、resource lifecycle、Kernel 実行。決定的な
  observable behavior test が必要です。
- **Adapter 変更**: crate／OS 固有実装。baseline contract を変更せず、共通 conformance suite を
 通過させます。
- **高負荷経路の変更**: 境界呼び出し、PTY／terminal stream、大規模 timeline、rendering、allocation。
  workload と計測の証跡が必要です。
- **Plugin／Kernel 変更**: L# の振る舞いまたは SDK。host effect を明示し、Capability 検査を通します。

## Pull Request

リポジトリの PR template を使用します。目的、非目標、影響する境界、正確な検証結果、未実行の
検証、互換性への影響、残存リスクを日本語で記載します。
