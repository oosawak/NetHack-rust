# NetHack Rust + WASM Project

## プロジェクト概要

このプロジェクトは、**NetHack 5.0**（C99、~25万行）を **FFI-First** アプローチで現代化し、複数のプラットフォーム（デスクトップ、WebAssembly、Unity）で動作させるものです。

### 🎯 目標

- ✅ **C コード再利用** — 既存の安定したゲームロジック（AI、ダンジョン生成、戦闘）をそのまま活用
- 🎨 **モダングラフィックス** — wgpu による 3D レンダリング
- 🌐 **マルチプラットフォーム対応** — デスクトップ、WASM、Unity ネイティブプラグイン
- 🏗️ **段階的実装** — FFI → Game Bridge → レンダリング → プラットフォーム統合

### 📊 プロジェクト規模

| コンポーネント | 言語 | 行数 | 説明 |
|-------------|------|------|------|
| ゲームロジック | C | ~250k | AI・ダンジョン生成・戦闘・アイテムシステム |
| プラットフォーム層 | C | ~3k | UNIX/Windows 固有のコード |
| Rust ラッパー | Rust | ~4.3k | FFI・グラフィックス・プラットフォーム抽象化 |

### 🏗️ アーキテクチャ

```
┌─────────────────────────────────────────────────────────────┐
│                 Desktop App / WASM / Unity                  │
│  (nethack-desktop / nethack-wasm / nethack-unity)           │
└──────────────┬──────────────────────────────────────────────┘
               │
┌──────────────▼──────────────────────────────────────────────┐
│        Game Bridge (Rust + wgpu)                            │
│  (nethack-render + Camera3D system)                         │
└──────────────┬──────────────────────────────────────────────┘
               │
┌──────────────▼──────────────────────────────────────────────┐
│         FFI Wrapper (nethack-sys)                           │
│  bindgen-generated bindings + stage-aware initialization    │
└──────────────┬──────────────────────────────────────────────┘
               │
┌──────────────▼──────────────────────────────────────────────┐
│      NetHack C Library (139 object files)                   │
│  Game logic, AI, dungeon generation, combat, items         │
└─────────────────────────────────────────────────────────────┘
```

### 📦 Cargo Workspace 構成

```
crates/
├── nethack-sys          FFI bindings + initialization (Phase 2 ✅)
├── nethack-core         3D Camera + World management (Phase 0 ✅)
├── nethack-render       wgpu graphics layer (Phase 3 🔄)
├── nethack-desktop      Desktop app (winit + wgpu) (Phase 4 📋)
├── nethack-wasm         WebAssembly target (Phase 4 📋)
└── nethack-unity        Unity native plugin (Phase 5 📋)
```

---

## 🚀 クイックスタート

### 必要な環境

- **Rust**: 1.70.0 以上
- **C コンパイラ**: gcc / clang
- **依存ライブラリ**: lua5.4, ncurses
- **Git**: リポジトリ管理用

### インストール手順

```bash
# リポジトリのクローン
git clone https://github.com/oosawak/NetHack.git
cd NetHack
git checkout master

# 依存関係をインストール（Linux）
sudo apt-get install build-essential libncurses-dev liblua5.4-dev

# Rust ツールチェーンセットアップ
rustup update

# プロジェクトビルド
cargo build --workspace

# テスト実行
cargo test --workspace
```

### 初期化の流れ

```rust
use nethack_sys::*;

fn main() -> Result<(), String> {
    // Stage 1-7 の自動初期化
    full_init()?;
    
    // ゲーム開始
    // moveloop() は無限ループなので、別途ゲームループ実装が必要
    
    Ok(())
}
```

---

## 📚 ドキュメント一覧

| ドキュメント | 説明 |
|-----------|------|
| [ARCHITECTURE.md](./ARCHITECTURE.md) | 詳細なアーキテクチャ設計 |
| [GETTING_STARTED.md](./GETTING_STARTED.md) | セットアップ・ビルド手順 |
| [FFI_GUIDE.md](./FFI_GUIDE.md) | FFI バインディングと初期化の詳細 |
| [WASM_BUILD.md](./WASM_BUILD.md) | WASM ビルド手順とサンプル |
| [DEVELOPMENT.md](./DEVELOPMENT.md) | 開発ガイド（Phase 3 以降） |
| [LICENSE.md](./LICENSE.md) | ライセンス情報（NGPL） |

---

## 📋 実装進捗

### 完了済み

- ✅ **Phase 0**: Cargo workspace + 3D Camera system
- ✅ **Phase 1**: NetHack C ビルド統合 (12MB executable)
- ✅ **Phase 2**: bindgen FFI + stage-aware initialization

### 進行中

- 🔄 **Phase 3**: Game Bridge + グローバル状態アクセス

### 計画中

- 📋 **Phase 4**: デスクトップアプリ（winit + wgpu）
- 📋 **Phase 5**: WASM ビルド（wasm-bindgen）
- 📋 **Phase 6**: Unity ネイティブプラグイン

---

## 🌐 WASM サンプルホルダー

WASM ファイル完成後、以下の構成で公開予定：

```
wasm-examples/
├── README.md                    WASM サンプルの使い方
├── index.html                   WebGL コンテキスト HTML
├── lib/
│   ├── nethack_wasm.js          wasm-bindgen 生成 JS
│   └── nethack_wasm_bg.wasm     実行可能 WASM バイナリ
├── css/
│   └── style.css                UI スタイル
├── js/
│   ├── main.js                  WASM 初期化・ゲームループ
│   ├── renderer.js              wgpu レンダリング
│   └── input.js                 キー入力ハンドリング
└── assets/
    └── tileset.png              キャラクター・タイル画像
```

**公開予定先:** `wasm-examples` ディレクトリ（GitHub Pages で静的公開）

---

## 🛠️ 技術スタック

| 機能 | 使用クレート | バージョン |
|------|-----------|---------|
| グラフィックス | `wgpu` | 0.22.x |
| ウィンドウ | `winit` | 0.30.x |
| WASM | `wasm-bindgen`, `wasm-pack` | 0.2.x |
| FFI生成 | `bindgen` | 0.70.x |
| C ビルド | `cc` | 1.x |
| テキスト描画 | `glyphon` | 0.7.x |
| ロギング | `tracing` | 0.1.x |
| エラー処理 | `anyhow`, `thiserror` | - |

---

## 🎮 ゲームプレイ機能

### 実装済み

- ✅ NetHack コア（C ライブラリ）全機能
  - AI システム
  - ダンジョン生成
  - アイテム・モンスター管理
  - セーブ/ロード

- ✅ 3D カメラシステム（5 ビューモード）
  - TopDown（俯瞰）
  - Isometric（等角投影）
  - FirstPerson（一人称）
  - ThirdPerson（三人称）
  - Cinematic（シネマティック）

### 計画中

- 📋 wgpu レンダリング統合
- 📋 キーボード入力ハンドリング
- 📋 UI/HUD の実装
- 📋 サウンド統合

---

## 🔗 リンク

- **GitHub**: https://github.com/oosawak/NetHack (master branch)
- **NetHack Official**: https://www.nethack.org/
- **NetHack Wiki**: https://nethackwiki.com/

---

## 📝 ライセンス

このプロジェクトは NetHack General Public License (NGPL) の下で公開されています。詳細は [LICENSE.md](./LICENSE.md) を参照。

---

## 🤝 貢献

プルリクエスト・イシューは大歓迎です！開発ガイドは [DEVELOPMENT.md](./DEVELOPMENT.md) を参照してください。

---

**最終更新**: 2026-05-06  
**プロジェクトステータス**: Phase 2 完了、Phase 3 進行中
