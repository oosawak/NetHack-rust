# Getting Started

このガイドでは、NetHack Rust + WASM プロジェクトをローカル環境でセットアップして実行する手順を説明します。

---

## 必要な環境

### システム要件

| 項目 | 最小バージョン |
|------|---|
| Rust | 1.70.0 |
| Cargo | 1.70.0 |
| C コンパイラ | GCC 9 / Clang 10 / MSVC 2019 |
| Python | 3.8（オプション、WASM ビルド用） |
| Git | 2.0 |

### 依存ライブラリ

#### Linux (Ubuntu/Debian)

```bash
sudo apt-get update
sudo apt-get install -y \
    build-essential \
    pkg-config \
    libncurses-dev \
    liblua5.4-dev \
    git
```

#### macOS

```bash
# Homebrew を使用
brew install lua ncurses pkg-config

# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Windows (MSVC)

```powershell
# Visual Studio Build Tools をインストール
# https://visualstudio.microsoft.com/downloads/

# Rust (msvc toolchain)
curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Lua development files
# https://luabinaries.sourceforge.net/ からダウンロード
```

---

## セットアップ手順

### 1. リポジトリのクローン

```bash
git clone https://github.com/oosawak/NetHack.git
cd NetHack
git checkout master
```

### 2. Rust ツールチェーンセットアップ

```bash
# Rust をインストール（未インストール時）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 最新に更新
rustup update

# 安定版チャネル確認
rustc --version
```

### 3. プロジェクト構造確認

```bash
# クレートの確認
cargo metadata --format-version 1 | grep -E '"name"'

# ワークスペース内のクレート一覧
cargo metadata --format-version 1 --no-deps | jq '.workspace_members'
```

---

## ビルド

### 全クレートをビルド

```bash
# デバッグビルド（開発時）
cargo build --workspace

# リリースビルド（本番）
cargo build --release --workspace

# 特定クレートのみビルド
cargo build -p nethack-sys
cargo build -p nethack-core
```

### ビルド出力の確認

```bash
# 生成されたバイナリの確認
ls -lh target/debug/
ls -lh target/release/

# 依存関係の確認
cargo tree --all
```

---

## テスト実行

### 全テストを実行

```bash
# 全テスト実行
cargo test --workspace

# 詳細出力付き
cargo test --workspace -- --nocapture

# nethack-core のテストのみ
cargo test -p nethack-core
```

### テスト結果

期待される出力：

```
running 6 tests
test camera::tests::test_camera_creation ... ok
test camera::tests::test_camera_follow ... ok
test camera::tests::test_camera_switch ... ok
test camera::tests::test_world_creation ... ok
test camera::tests::test_player_movement ... ok
test camera::tests::test_world::test_camera_switching ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured
```

---

## ゲーム初期化

### Rust コード例

```rust
use nethack_sys::*;

fn main() -> Result<(), String> {
    // Stage-aware 初期化（7 段階）
    full_init()?;
    
    // ここで moveloop() は無限ループに入ります
    // 実際のゲームループは Phase 3+ で実装予定
    
    Ok(())
}
```

### 初期化の詳細

```rust
// Stage 1: プログラム状態初期化
init_stage_1_early()?;

// Stage 2: UI システム選択
init_stage_2_choose_windows()?;

// Stage 3: オプション読み込み
init_stage_3_options()?;

// Stage 4: ウィンドウシステム初期化
init_stage_4_nhwindows()?;

// Stage 5: ダンジョン DB 初期化
init_stage_5_dlb()?;

// Stage 6: 視野システム初期化
init_stage_6_vision()?;

// Stage 7: ゲーム開始
init_stage_7_newgame()?;

// 現在のステージ確認
let stage = current_stage()?;
println!("Current stage: {:?}", stage);
```

---

## 開発フロー

### コード変更時のビルド

```bash
# 変更検出して自動ビルド
cargo watch -c -x "build --workspace"

# テスト実行も含める
cargo watch -c -x "test --workspace"
```

### Lint・フォーマット確認

```bash
# clippy（Rust linter）
cargo clippy --workspace

# コードフォーマット確認
cargo fmt --check

# コード自動フォーマット
cargo fmt --all
```

### ドキュメント生成

```bash
# ドキュメント生成（HTML）
cargo doc --workspace --open

# 特定クレートのドキュメント
cargo doc -p nethack-sys --open
cargo doc -p nethack-core --open
```

---

## トラブルシューティング

### ビルドエラー

#### エラー: `lua.h not found`

**原因**: Lua 開発ファイルが不足

**解決**:
```bash
# Linux
sudo apt-get install liblua5.4-dev

# macOS
brew install lua@5.4

# Windows
# https://luabinaries.sourceforge.net/ から Lua 5.4 をダウンロード
```

#### エラー: `ncurses.h not found`

**原因**: ncurses 開発ファイルが不足

**解決**:
```bash
# Linux
sudo apt-get install libncurses-dev

# macOS
brew install ncurses

# Windows
# MSVC の場合、カスタムビルドまたは代替ライブラリを使用
```

#### エラー: `bindgen: clang not found`

**原因**: clang ツールチェーンが不足

**解決**:
```bash
# Linux
sudo apt-get install clang libclang-dev

# macOS
brew install clang

# Windows
# Visual Studio Build Tools をインストール
```

### リンクエラー

#### エラー: `undefined reference to 'early_init'`

**原因**: NetHack オブジェクトファイルが見つからない

**確認**:
```bash
ls /home/oosawak/Workspace/NetHack/src/*.o | wc -l
# Expected: 139 files
```

**解決**: NetHack を再ビルド
```bash
cd /home/oosawak/Workspace/NetHack
sh sys/unix/setup.sh hints/linux.500
make clean
make all
```

### テストエラー

#### テストがタイムアウト

**解決**:
```bash
# タイムアウト時間を増加
cargo test --workspace -- --test-threads=1 --nocapture
```

---

## 次のステップ

### Phase 3: Game Bridge 実装

Game Bridge を実装することで、C グローバル状態へのアクセスが可能になります：

```rust
// 実装予定（Phase 3）
let player_x = get_player_x();
let player_y = get_player_y();
let dungeon_level = get_dungeon_level()?;

println!("Player at ({}, {}), Level {}", player_x, player_y, dungeon_level);
```

### WASM ビルド

WASM ターゲット用のビルド手順（Phase 4）：

```bash
# WASM target インストール
rustup target add wasm32-unknown-unknown

# WASM パック
wasm-pack build crates/nethack-wasm --target web
```

### デスクトップアプリ

デスクトップアプリケーション実行（Phase 4）：

```bash
cargo run --release -p nethack-desktop
```

---

## 参考資料

| リソース | 説明 |
|---------|------|
| [ARCHITECTURE.md](./ARCHITECTURE.md) | システムアーキテクチャ詳細 |
| [FFI_GUIDE.md](./FFI_GUIDE.md) | FFI バインディング詳細 |
| [Rust Book](https://doc.rust-lang.org/book/) | Rust 言語学習 |
| [wgpu Book](https://sotrh.github.io/learn-wgpu/) | wgpu グラフィックス |
| [NetHack Wiki](https://nethackwiki.com/) | NetHack ゲーム知識 |

---

## サポート

### Q&A

**Q: どのプラットフォームが完成している？**

A: 現在 Phase 2 完了。FFI バインディングと初期化が完成しています。デスクトップアプリ、WASM、Unity プラグインは Phase 3 以降の実装予定です。

**Q: WASM サンプルはどこにある？**

A: `wasm-examples/` ディレクトリで公開予定です（Phase 4 完成後）。

**Q: ライセンスは？**

A: NetHack General Public License (NGPL) を継承しています。詳細は [docs/LICENSE.md](./LICENSE.md) を参照。

---

**最終更新**: 2026-05-06  
**対応 Rust バージョン**: 1.70.0 以上
