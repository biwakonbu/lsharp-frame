# L# Kernel 向け指示

この指示は、`kernel/**` に対してルート `AGENTS.md` を補足します。

L#frame Kernel は、状態遷移、command、keymap、hook、workflow、UI composition、plugin lifecycle policy
などのプロダクト挙動を所有します。

- ネイティブ機構は型付き Effect として要求し、OS 固有挙動を埋め込みません。
- 可能な限りイベント処理を決定的に保ちます。
- PTY byte 処理、terminal rendering、その他の高負荷 loop を L# へ移しません。
- 粗粒度の `EventBatch -> FrameUpdate` 呼び出しを維持します。
- 公開 Kernel contract を変更する場合は WIT と Rust projection を更新します。
- compiler が利用できない場合は、L# の compile／test を未実行として報告します。
- AI の説明、レビュー、引き継ぎは原則として日本語で出力します。
