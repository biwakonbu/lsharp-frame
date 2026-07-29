# Contributing

## Core rule

外部 crate の型を `lsharp-frame-contract`、WIT、L# plugin API へ露出しないでください。crate 固有の変換は adapter crate の内部へ閉じ込めます。

## Local checks

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Change categories

- Contract change: `lsharp-frame-contract` または `wit/` の変更。互換性と migration note が必要です。
- Core change: event routing、capability、resource lifecycle、kernel execution の変更。
- Adapter change: crate／OS 固有実装。baseline contract を変更してはいけません。
- Plugin change: L# plugin または SDK の変更。

## Pull request expectations

- observable behavior と受入条件を書く
- hot path に新しい境界呼び出しを追加した場合は benchmark を付ける
- adapter を追加した場合は conformance suite を通す
- baseline で表せない機能は versioned extension として追加する
