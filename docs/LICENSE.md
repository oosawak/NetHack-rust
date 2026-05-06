# ライセンス

このプロジェクトは複数のライセンスの下で構成されています。

---

## NetHack General Public License (NGPL)

**NetHack のコア C コード**（`/src`, `/sys`, `/include` ディレクトリ）は、NetHack General Public License (NGPL) の下で公開されています。

### NGPL の主要ポイント

- **条件**：NGPL の下で公開されたソフトウェアの修正・派生物も NGPL で公開する必要があります。
- **商用利用**：許可されていますが、派生物は NGPL で公開する必要があります。
- **再配布**：ソースコード付き再配布が可能です。

### 完全なテキスト

NetHack の完全なライセンステキストは、公式サイトで確認できます：
https://www.nethack.org/index.html

```
Copyright (c) Stichting Mathematisch Centrum, Amsterdam, 1985.
NetHack may be freely redistributed. See license for details.
```

---

## Rust ラッパーコード

**Rust コード**（`/crates` ディレクトリ）は、以下の方針で公開されています：

### MIT License (Preferred)

Rust ラッパーコードは MIT ライセンスで提供されます。ただし、NetHack コアとのリンクにより、全体としては NGPL の下で公開されるべきです。

```
MIT License

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software...
```

---

## 依存クレート

このプロジェクトで使用されるオープンソースクレートのライセンス：

| クレート | ライセンス | 用途 |
|---------|-----------|------|
| wgpu | MIT / Apache 2.0 | グラフィックス |
| winit | MIT / Apache 2.0 | ウィンドウ管理 |
| wasm-bindgen | MIT / Apache 2.0 | WASM インターフェース |
| bindgen | MIT / Apache 2.0 | FFI 生成 |
| cc | MIT / Apache 2.0 | C コンパイル |
| glyphon | MIT / Apache 2.0 | テキスト描画 |
| lazy_static | MIT / Apache 2.0 | グローバル変数 |
| serde | MIT / Apache 2.0 | シリアライゼーション |
| anyhow | MIT / Apache 2.0 | エラー処理 |
| tracing | MIT / Apache 2.0 | ロギング |

**詳細**: `Cargo.toml` の `[dependencies]` セクション参照

---

## 複合ライセンス戦略

```
┌────────────────────────────────────────┐
│  このプロジェクト全体                   │
│  (GitHub: oosawak/NetHack)             │
└────────────┬─────────────────────────┘
             │
    ┌────────┴─────────┐
    │                  │
┌───▼─────────────┐   ┌──▼──────────────┐
│ NetHack C コード  │   │ Rust ラッパー    │
│ (NGPL)          │   │ (MIT)           │
│ /src            │   │ /crates         │
│ /sys            │   │                 │
│ /include        │   │ → 依存クレート   │
└─────────────────┘   │ (MIT / Apache)  │
                      └─────────────────┘
                      
結果：派生物は NGPL で公開推奨
```

---

## 派生物の公開について

このプロジェクトから派生したプロジェクトは、以下に従う必要があります：

### 必須事項

1. **ソースコード公開**：派生物のソースコードを公開する
2. **ライセンス表示**：NGPL / MIT のクレジット表記を含める
3. **変更内容記録**：加えた変更内容をドキュメント化する

### 推奨事項

1. **GitHub での公開**：オープンソース開発での透明性確保
2. **CONTRIBUTING.md 作成**：コントリビューター向けガイド作成
3. **CHANGELOG.md 作成**：バージョン履歴を記録

### 例

```markdown
# MyNetHackProject

本プロジェクトは NetHack および NetHack Rust FFI Wrapper の派生物です。

## ライセンス

- **NetHack コア**: NetHack General Public License (NGPL)
- **Rust ラッパー**: MIT License
- **派生物**: NGPL（NetHack との統合のため）

## クレジット

元プロジェクト: https://github.com/oosawak/NetHack
```

---

## FAQ

### Q: 商用利用できますか？

**A**: NetHack の制限により、派生物を NGPL で公開することが条件になります。

### Q: ソースコードを公開したくない場合は？

**A**: NetHack のコアロジック部分を使用しない設計（FFI を経由せず新規実装）にする必要があります。ただし、このプロジェクトとしての利点がなくなります。

### Q: Android / iOS アプリで配布できますか？

**A**: はい。ソースコードを GitHub 等で公開し、アプリのストア説明にリンクを記載することで NGPL 要件を満たせます。

### Q: WASM サンプルを改変して商用利用できますか？

**A**: `wasm-examples/` 内のサンプルコードは、元の派生物が NGPL なので、NGPL の条件を満たせば可能です。

---

## 疑問・相談

ライセンスについて疑問がある場合：

1. **NetHack FAQ**: https://www.nethack.org/
2. **Open Source Initiative**: https://opensource.org/licenses/
3. **GitHub Issues**: https://github.com/oosawak/NetHack/issues

---

## 更新履歴

| 日付 | 変更 |
|------|------|
| 2026-05-06 | 初版作成 |

---

**重要**: このドキュメントは参考情報です。法的助言が必要な場合は、弁護士に相談してください。
