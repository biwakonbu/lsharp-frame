# Rust workspace 向け指示

この指示は、`crates/**` に対してルート `AGENTS.md` を補足します。

## 対象範囲

- crate の責務を `docs/architecture.md` と一致させます。
- `lsharp-frame-contract` には安定した意味データだけを置き、GUI toolkit、renderer crate、
  async runtime、PTY crate、OS binding に依存させません。
- `lsharp-frame-spi` は L#frame 所有型を使って Port を定義します。
- `lsharp-frame-core` は具体 adapter を選択せずに Port と contract を調停します。
- Adapter crate は外部実装へ依存できますが、公開 API は L#frame 所有型を返します。

## 必須検証

実装中は対象 crate のテストを実行し、最後に次を実行します。

```bash
make fmt-check
make lint
make test
make harness
```

公開 contract を変更する場合は、WIT／L# projection を同時に更新し、互換性の証跡を追加します。
移行方法を記述せず、広範な `non_exhaustive` や不透明 payload の裏へ breaking change を隠しては
いけません。AI の説明、レビュー、引き継ぎは原則として日本語で出力します。
