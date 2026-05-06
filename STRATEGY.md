# NetHack Rust Rewrite Strategy

## 戦略概要

NetHackの全機能をRustで実装し、WASM・Unity・デスクトップで動作させる。
プラットフォーム依存部分の分析により、**WASM対応は十分可能**であることを確認。

## コード分析結果

### プラットフォーム層の分離

| コンポーネント | 行数 | 依存関係 |
|---|---|---|
| 共通ゲームロジック (src/) | ~250,000 | プラットフォーム非依存 |
| 共通エントリポイント (sys/share/) | ~818 | メインロジック |
| Unix依存層 (sys/unix/) | ~882 | signal, fork, POSIX I/O |
| Windows依存層 (sys/windows/) | ~1,382 | Win32 API |
| **合計依存コード** | **~3,082** | **1.2%のみ** |

**結論：メインゲームロジック (~99%)はプラットフォーム非依存！**

### WASM互換性の検証

| 項目 | 状況 | 対策 |
|---|---|---|
| signal/fork | Unix限定 (~880行) | WASM では `sys/share/` + Rust実装のみ使用 |
| ファイルI/O | 散在 (fopen/open) | Rust の `std::fs` + trait 抽象化 |
| グローバル状態 | C構造体でグローバル | `lazy_static` + `Mutex<GameState>` でRust化 |
| メモリ管理 | 不透明なポインタ | Rust 所有権で統一 |

**結論：プラットフォーム層を置き換えるだけで、WASM対応は実現可能！**

## フェーズ別実装計画

### フェーズ 0: ワークスペース設定 ✅ DONE

```
nethack-rs/
├── crates/
│   ├── nethack-sys       ← C FFI (移行期用)
│   ├── nethack-core      ← Rust ゲームロジック
│   ├── nethack-render    ← wgpu レンダラ
│   ├── nethack-desktop   ← winit + wgpu
│   ├── nethack-wasm      ← wasm-bindgen
│   └── nethack-unity     ← cdylib
└── Cargo.toml (workspace)
```

**状態:** `cargo check --workspace` ✅ 成功

### フェーズ 1: nethack-sys FFI ラッパー (IN PROGRESS)

**目標:** 既存Cコードをビルド・ラップし、Rust から呼べるようにする。

**実装内容:**
1. `build.rs` で `cc` クレートを使い `src/*.c` を静的ライブラリ化
2. `bindgen` で `include/extern.h` から FFI バインディング自動生成
3. `window_procs` 構造体を Rust 実装で差し替え可能にする

**キーファイル:**
- `crates/nethack-sys/build.rs` — C コンパイル設定
- `crates/nethack-sys/src/lib.rs` — Rust FFI シム

### フェーズ 2: nethack-core Rust 実装

**優先順位順の実装:**

1. **高優先度 (独立・副作用なし)**
   - `rng.rs` — 乱数生成器 (Rust `rand` クレート)
   - `dungeon.rs` — マップ表現・タイル管理
   - `state.rs` — ゲーム状態管理

2. **中優先度 (部分的な依存)**
   - オブジェクト管理 (item.rs)
   - モンスターデータ構造

3. **低優先度 (複雑・後回し)**
   - セーブ/ロード (save.rs/restore.rs)
   - モンスター AI (monmove.c)

### フェーズ 3: nethack-render wgpu レンダラ

**アーキテクチャ:**
```
Game State (Rust)
    ↓
Game → Render Buffer (ASCII or Tile)
    ↓
wgpu Pipeline
    ↓
Screen Output (Desktop / WASM / WebGPU)
```

**実装:**
- `WgpuRenderer` struct
- グリフアトラス・タイル管理
- 共通パイプライン (Desktop/WASM で同一コード)

### フェーズ 4: プラットフォーム統合

#### 4a. Desktop (winit + wgpu)
```sh
cargo build --release -p nethack-desktop
```
→ `nethack` バイナリ

#### 4b. WASM (WebAssembly)
```sh
wasm-pack build --target web crates/nethack-wasm
```
→ `pkg/nethack_wasm.js` + `pkg/nethack_wasm_bg.wasm`

#### 4c. Unity Plugin (cdylib)
```sh
cargo build --release -p nethack-unity --target x86_64-pc-windows-msvc
```
→ `nethack_unity.dll`, `libnethack_unity.dylib`, `libnethack_unity.so`

## プラットフォーム抽象化設計

### FileSystem trait

```rust
pub trait FileSystem {
    fn open_read(&self, path: &str) -> Result<Box<dyn Read>>;
    fn open_write(&self, path: &str) -> Result<Box<dyn Write>>;
    fn exists(&self, path: &str) -> bool;
}

// Platform implementations:
// - UnixFileSystem  → std::fs
// - WindowsFileSystem → std::fs (Rust 統一)
// - WasmFileSystem  → IndexedDB / localStorage
```

### Signal Handling

```rust
#[cfg(unix)]
mod signal {
    // SIGINT, SIGHUP, SIGXCPU ハンドラ
    // → Wasm では空実装
}

#[cfg(target_arch = "wasm32")]
mod signal {
    // No-op for WASM
}
```

### Time Management

```rust
pub trait Timer {
    fn now() -> SystemTime;
    fn sleep(duration: Duration);
}

// Implementations:
// - StdTimer (desktop)
// - WasmTimer (web-sys, requestAnimationFrame)
```

## ビルド設定

### Desktop ビルド
```sh
cd nethack-rs
cargo build --release -p nethack-desktop
```

### WASM ビルド
```sh
wasm-pack build --target web crates/nethack-wasm --release
```

**前提:** wasm32-unknown-unknown ターゲット必須
```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
```

### Unity ビルド
```sh
# Windows (MSVC)
cargo build --release -p nethack-unity --target x86_64-pc-windows-msvc

# macOS (Apple Silicon)
cargo build --release -p nethack-unity --target aarch64-apple-darwin

# Linux (x86_64)
cargo build --release -p nethack-unity --target x86_64-unknown-linux-gnu
```

Unity 側の C# binding:
```csharp
[DllImport("nethack_unity")]
static extern int nethack_init();

[DllImport("nethack_unity")]
static extern int nethack_send_command(byte cmd);
```

## ライセンス

NetHack 5.0 は NGPL (NetHack General Public License) 下で公開されている。

**派生物としてのこのプロジェクトは:**
- NGPL を継承して公開する
- DEVEL/git_recipes.txt に貢献方法を文書化
- 著者クレジットを保持する

## 次のステップ

1. **フェーズ1 実装:** build.rs + bindgen 設定
2. **フェーズ2 開始:** rng.rs, dungeon.rs, state.rs の実装
3. **統合テスト:** nhackのCコードとRust実装の互換性検証
4. **WASM テスト:** web ブラウザでの基本動作確認
5. **Unity テスト:** Unity プロジェクトでの DLL 読み込み
