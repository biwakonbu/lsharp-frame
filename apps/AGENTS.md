# Composition root 向け指示

この指示は、`apps/**` に対してルート `AGENTS.md` を補足します。

Application crate は composition root です。具体的な adapter と runtime 実装を選択できますが、
ドメイン規則の第二の置き場所にしてはいけません。

- 依存関係を接続し、Core の orchestration を重複実装しません。
- 起動、終了、resource ownership を明示します。
- 設定値を内側へ渡す前に L#frame 所有の option へ変換します。
- 新しい composition root ごとに integration test または smoke test を追加します。
- Backend の選択は、L# plugin と baseline WIT を変更せず交換可能に保ちます。
- AI の説明、レビュー、引き継ぎは原則として日本語で出力します。
