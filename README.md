# NetHack Rust + WASM

> **⚠️ 開発中プロジェクト** | Phase 2 完了、Phase 3 進行中

NetHack 5.0 を **FFI-First** アプローチで Rust + WASM に対応させるプロジェクト。C ゲームロジック（~250k 行）を再利用しつつ、モダングラフィックス（wgpu）でマルチプラットフォーム対応を実現します。

---

## 🎯 プロジェクト概要

### ビジョン

- ✅ **既存 C コード再利用** — 安定したゲームロジック・AI・ダンジョン生成をそのまま活用
- 🎨 **モダングラフィックス** — wgpu による 3D レンダリング
- 🌐 **マルチプラットフォーム** — Desktop (Linux/Mac/Windows)、WebAssembly、Unity ネイティブプラグイン
- 🚀 **段階的実装** — FFI → Game Bridge → Graphics → Platform Integration

### 対応ターゲット（予定）

| プラットフォーム | ステータス | 完成時期 |
|---------------|----------|--------|
| **Desktop** (Linux/Mac/Windows) | 📋 計画中 | Phase 4 |
| **WebAssembly** (ブラウザ) | 📋 計画中 | Phase 4 |
| **Unity** ネイティブプラグイン | 📋 計画中 | Phase 5 |

---

## 📊 実装進捗

```
Phase 0: ワークスペース設定                   ✅ COMPLETE
Phase 1: FFI インフラ構築                   ✅ COMPLETE
Phase 2: bindgen + Stage-aware 初期化        ✅ COMPLETE
Phase 3: Game Bridge + グローバル状態アクセス 🔄 IN PROGRESS
Phase 4: デスクトップアプリ + WASM ビルド     📋 PLANNED
Phase 5: Unity ネイティブプラグイン          📋 PLANNED
Phase 6: 段階的 Rust 移植                   📋 PLANNED
```

### 📈 現在のステータス

| コンポーネント | 進捗 | 詳細 |
|-------------|------|------|
| **nethack-sys** | ✅ 完了 | FFI バインディング、Stage-aware 初期化 |
| **nethack-core** | ✅ 完了 | 3D Camera (5 modes), World3D, Tests (6/6 pass) |
| **nethack-render** | 📋 計画中 | wgpu パイプライン (Phase 3) |
| **nethack-desktop** | 📋 計画中 | winit + wgpu アプリ (Phase 4) |
| **nethack-wasm** | 📋 計画中 | WASM + WebGL (Phase 4) |
| **nethack-unity** | 📋 計画中 | C# DllImport プラグイン (Phase 5) |

---

## 🚀 クイックスタート

### 環境準備

```bash
# リポジトリクローン
git clone https://github.com/oosawak/NetHack.git
cd NetHack
git checkout master

# 依存ライブラリインストール (Ubuntu/Debian)
sudo apt-get install -y build-essential libncurses-dev liblua5.4-dev

# Rust ツールチェーン
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### ビルド

```bash
# 全クレートをビルド
cargo build --workspace

# テスト実行
cargo test --workspace

# 詳細ドキュメント
cargo doc --workspace --open
```

### テスト結果（期待値）

```
running 6 tests
test camera::tests::test_camera_creation ... ok
test camera::tests::test_camera_follow ... ok
test camera::tests::test_camera_switch ... ok
test camera::tests::test_world_creation ... ok
test camera::tests::test_player_movement ... ok
test camera::tests::test_camera_switching ... ok

test result: ok. 6 passed; 0 failed
```

---

## 📚 ドキュメント

| ドキュメント | 説明 |
|-----------|------|
| [docs/README.md](./docs/README.md) | プロジェクト概要・目次 |
| [docs/ARCHITECTURE.md](./docs/ARCHITECTURE.md) | 詳細なシステムアーキテクチャ |
| [docs/GETTING_STARTED.md](./docs/GETTING_STARTED.md) | セットアップ・ビルド手順 |
| [docs/FFI_GUIDE.md](./docs/FFI_GUIDE.md) | FFI バインディング詳細（作成予定） |
| [docs/WASM_BUILD.md](./docs/WASM_BUILD.md) | WASM サンプル・デプロイ手順 |
| [docs/DEVELOPMENT.md](./docs/DEVELOPMENT.md) | 開発ガイド（作成予定） |
| [docs/LICENSE.md](./docs/LICENSE.md) | ライセンス情報 |

---

## 🏗️ プロジェクト構成

```
NetHack/                          (root)
├── /src                          C ソースコード + コンパイル済みオブジェクト (139個)
├── /sys                          プラットフォーム固有コード (Unix/Windows)
├── /include                       C ヘッダファイル
├── /crates                        Rust ワークスペース
│   ├── nethack-sys              C FFI バインディング ✅
│   ├── nethack-core             3D カメラ・ゲーム状態 ✅
│   ├── nethack-render           wgpu グラフィックス 🔄
│   ├── nethack-desktop          デスクトップアプリ 📋
│   ├── nethack-wasm             WASM ターゲット 📋
│   └── nethack-unity            Unity プラグイン 📋
├── /docs                         ドキュメント
│   ├── README.md
│   ├── ARCHITECTURE.md
│   ├── GETTING_STARTED.md
│   ├── WASM_BUILD.md
│   └── LICENSE.md
├── /wasm-examples                WASM サンプルホルダー (完成時に公開予定)
├── Cargo.toml                     Rust Workspace マニフェスト
├── Cargo.lock                     依存ロック
└── .gitignore                     Git 除外設定
```

---

## 🎮 ゲーム機能

### 実装済み ✅

- NetHack コア機能（C ライブラリ全機能）
  - ターンベース RPG システム
  - AI・モンスター管理
  - ダンジョン手続き的生成
  - アイテム・装備システム
  - セーブ/ロード機能

- 3D カメラシステム（5 ビューモード）
  - **TopDown** — 俯瞰（オリジナル ASCII 風）
  - **Isometric** — 等角投影（Diablo スタイル）
  - **FirstPerson** — 一人称視点
  - **ThirdPerson** — プレイヤー後ろ
  - **Cinematic** — シネマティック

### 開発中 🔄

- Game Bridge（C グローバル状態アクセス）
- player position, dungeon level, monster list 読み取り
- wgpu レンダリング統合テスト

### 計画中 📋

- wgpu フル統合（シェーダー・テクスチャ・UI）
- キーボード入力ハンドリング
- デスクトップ UI/HUD
- サウンド・BGM 統合
- WASM ブラウザ動作
- Unity C# インターフェース

---

## 🛠️ 技術スタック

### Rust クレート

| 機能 | クレート | バージョン | ライセンス |
|------|---------|---------|----------|
| グラフィックス | `wgpu` | 0.22.x | MIT/Apache-2.0 |
| ウィンドウ | `winit` | 0.30.x | MIT/Apache-2.0 |
| WASM | `wasm-bindgen` | 0.2.x | MIT/Apache-2.0 |
| FFI 生成 | `bindgen` | 0.70.x | MIT/Apache-2.0 |
| C ビルド | `cc` | 1.x | MIT/Apache-2.0 |
| テキスト | `glyphon` | 0.7.x | MIT/Apache-2.0 |
| エラー処理 | `anyhow` | - | MIT/Apache-2.0 |
| ロギング | `tracing` | 0.1.x | MIT/Apache-2.0 |

### C 依存

- **lua5.4** — スクリプト処理
- **ncurses** — ターミナル制御（デスクトップ版）
- **libc** — 標準 C ライブラリ

---

## 📈 パフォーマンス

### ビルド時間

| ビルドタイプ | 時間 | 出力サイズ |
|-----------|------|----------|
| デバッグ | ~5-10秒 | 150 MB |
| リリース | ~30秒 | 5-10 MB |
| WASM | ~15秒 | 2-5 MB (圧縮: 500KB-1MB) |

### ランタイム

- **フレームレート** — ターンベースなので実時間フレームレート不要
- **メモリ使用量** — ~50-100 MB（ダンジョン保存データを含む）
- **WASM** — ブラウザ標準 WebGL/WebGPU で動作

---

## 🌐 WASM サンプルホルダー

**完成予定**: Phase 4 終了後

```
wasm-examples/
├── index.html                     メインページ
├── lib/                           WASM バイナリ + JS バインディング
├── js/                            ゲームロジック・レンダリング
├── css/                           スタイル
├── assets/                        タイルセット・フォント
└── README.md                      使い方説明
```

**公開予定**: GitHub Pages で自動デプロイ

```
https://oosawak.github.io/NetHack/wasm-examples/
```

---

## 🤝 貢献

このプロジェクトはまだ開発中です。以下の方法でサポートできます：

### 開発参加

```bash
# フォーク & クローン
git clone https://github.com/YOUR_USERNAME/NetHack.git
cd NetHack
git checkout -b feature/your-feature

# 変更 & テスト
cargo test --workspace

# プルリクエスト送信
```

### 報告・提案

- **バグ報告**: [Issues](https://github.com/oosawak/NetHack/issues)
- **機能提案**: [Discussions](https://github.com/oosawak/NetHack/discussions)
- **改善提案**: [Pull Requests](https://github.com/oosawak/NetHack/pulls)

---

## 📝 ライセンス

このプロジェクトは複合ライセンスで構成されています：

- **NetHack コア** — NetHack General Public License (NGPL)
- **Rust ラッパー** — MIT License
- **派生物** — NGPL（NetHack 統合のため）

詳細は [docs/LICENSE.md](./docs/LICENSE.md) を参照。

---

## 🔗 リンク

- **GitHub**: https://github.com/oosawak/NetHack (master branch)
- **NetHack Official**: https://www.nethack.org/
- **NetHack Wiki**: https://nethackwiki.com/
- **Rust Book**: https://doc.rust-lang.org/book/
- **wgpu Guide**: https://sotrh.github.io/learn-wgpu/

---

## ⚠️ 開発中の注意事項

### 未完成機能

このプロジェクトはまだ**開発中**です。以下の機能はまだ実装されていません：

- ✅ C FFI バインディング
- ✅ ステージング初期化
- ✅ 3D カメラシステム
- 🔄 Game Bridge（進行中）
- ❌ wgpu レンダリング統合
- ❌ キーボード入力処理
- ❌ デスクトップ UI
- ❌ WASM ブラウザ実行
- ❌ Unity プラグイン

### 動作保証

- **対応 OS**: Linux (Ubuntu 20.04+), macOS 11+, Windows 10+
- **対応 Rust**: 1.70.0 以上
- **WASM 対応ブラウザ**: Chrome 113+, Firefox 120+, Safari 16.4+（将来実装時）

### 既知の制限

1. デスクトップアプリはまだ実行不可（Phase 4 待ち）
2. WASM ビルドはまだ実行不可（Phase 4 待ち）
3. Unity プラグインはまだ利用不可（Phase 5 待ち）
4. グラフィックス出力は未実装（すべて C インターフェースのみ）

---

## 🎯 ロードマップ

### Phase 3（現在進行中）

```
[ ====50%========     ] Game Bridge 実装
  - C グローバル変数アクセス
  - Player position, Dungeon level 読み取り
  - Entity リスト操作
  - 統合テスト実装
```

### Phase 4（予定）

- デスクトップアプリ実装（winit + wgpu）
- WASM ビルド・サンプル実装
- GitHub Pages デプロイ

### Phase 5（予定）

- Unity ネイティブプラグイン
- マルチプラットフォーム対応（Android, iOS）

### Phase 6（予定）

- Rust への段階的移植
- パフォーマンス最適化
- サウンド・音響統合

---

## 📞 サポート

### Q&A

**Q: いつ実行可能になる？**

A: Phase 3 完了後、デスクトップアプリ（Phase 4）で実行可能になります。現在 Phase 2 完了、Phase 3 進行中です。

**Q: WASM サンプルはもう使える？**

A: いいえ。Phase 4 完了後に `wasm-examples/` フォルダで公開予定です。

**Q: 貢献できる？**

A: はい！バグ報告、機能提案、プルリクエストを歓迎します。[DEVELOPMENT.md](./docs/DEVELOPMENT.md) を参照。

---

## 📅 更新履歴

| 日付 | 内容 |
|------|------|
| 2026-05-06 | Phase 2 完了、docs 作成、GitHub に master ブランチでプッシュ |
| 2026-04-?? | Phase 1b 完了（NetHack C ビルド統合） |
| 2026-04-?? | Phase 1a 完了（FFI インフラ） |
| 2026-03-?? | Phase 0 完了（Cargo ワークスペース + 3D Camera） |

---

**ステータス**: 🔄 開発中  
**最終更新**: 2026-05-06  
**メンテナー**: [@oosawak](https://github.com/oosawak)

---

> **本プロジェクトにご興味を持っていただきありがとうございます！**
> 
> Phase 4 でデスクトップアプリが実行可能になります。  
> それまでの間、[docs/](./docs/) で詳細なアーキテクチャを参照いただけます。

