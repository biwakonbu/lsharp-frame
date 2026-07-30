# L# Plugin 向け指示

この指示は、`plugins/**` に対してルート `AGENTS.md` を補足します。

- 必須 Capability と任意 Capability を明示します。
- host との連携を型付き Effect に限定します。
- 隠れた global mutation より、名前付き command と検査可能な extension point を優先します。
- 状態遷移と UI transaction に決定的な fixture を用意します。
- plugin が実 byte を必要としない限り、高負荷な raw stream を購読しません。
- 任意 process 実行などの trusted capability は、ユーザー権限での任意コード実行と同等であることを
  ドキュメントへ明記します。
- AI の説明、レビュー、引き継ぎは原則として日本語で出力します。
