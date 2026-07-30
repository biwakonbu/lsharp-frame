# WIT Contract 向け指示

この指示は、`wit/**` に対してルート `AGENTS.md` を補足します。

WIT は公開互換性境界です。

- バージョン付き package と、小さな Capability 指向 interface を使用します。
- L#frame の意味 record、variant、list、resource を優先します。
- crate 名、Rust layout、OS handle、GPU object、trait object を露出しません。
- byte、glyph、widget 単位の境界呼び出しを避け、batch operation を定義します。
- 公開 shape を変更するたびに、互換性と移行方法を記述します。
- 移植可能な baseline API と backend 固有 extension を分離します。
- AI の説明、レビュー、引き継ぎは原則として日本語で出力します。
