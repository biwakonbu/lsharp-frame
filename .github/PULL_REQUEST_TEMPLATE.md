## 目的

<!-- 観測可能な成果と、この変更が必要な理由。 -->

## 非目標

<!-- 意図的に対象外とする範囲。 -->

## 変更内容

<!-- Contract／Core／Adapter／L#／WIT／ドキュメントの変更。 -->

## アーキテクチャと互換性

- [ ] 外部実装型が安定契約へ漏洩していない。
- [ ] Capability と resource lifecycle の挙動が fail closed かつ明示的である。
- [ ] WIT／公開契約を変更した場合、互換性と移行方法を記載した。
- [ ] 高負荷経路を変更した場合、境界呼び出し、copy、allocation の挙動を記載した。

## 検証結果

- [ ] `make harness`
- [ ] `make fmt-check`
- [ ] `make check`
- [ ] `make lint`
- [ ] `make test`
- [ ] WIT を変更した場合は `make wit-check`

実行コマンドと結果:

```text

```

## 未実行の検証と残存リスク

<!-- 未実行のコマンドと理由を正確に記載する。実行していない検証を成功扱いしない。 -->
