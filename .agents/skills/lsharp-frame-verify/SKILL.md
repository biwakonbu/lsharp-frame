---
name: lsharp-frame-verify
description: プロダクトの振る舞いを変更せず、L#frame の working tree または branch を検証する。引き継ぎ、レビュー、コミット、PR ready 判断の前に使用する。
---

# L#frame の変更を検証する

出力は日本語にする。ユーザーが明示的に別言語を指定した場合のみ、その指定へ従う。

1. 適用済みのリポジトリ指示に従い、`git status` と完全な差分を確認する。
2. 必要なツールが利用可能なら、`make doctor`、`make harness`、`make ci` を実行する。
3. WIT を変更した場合は `make wit-check`、`lsharp` が利用可能なら L# の検証を実行する。
4. 失敗を根本原因へ対応付け、lint、互換性、安全性の gate を弱めない。
5. 正確なコマンド、結果、利用できないツール、残存リスクを報告する。
