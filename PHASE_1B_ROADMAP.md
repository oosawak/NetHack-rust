# Phase 1: FFI-First 実装ロードマップ

## 現在地：Phase 1a ✅ 完了

**何をしたか：**
- build.rs を実装してNetHack パスを検出
- nethack-sys に FFI モジュール構造を準備
- 全ワークスペースのビルドが成功（warnings のみ）

**課題と学び：**
- ❌ 全 C ファイルをコンパイルするには、NetHack の複雑な依存関係が障壁
- ✅ **代替戦略：** 既存のビルドシステムを活用する

---

## Phase 1b: C コンパイル戦略の修正

### アプローチの選択肢

| # | アプローチ | メリット | デメリット | 工数 |
|---|---|---|---|---|
| **A** | `make` で既存 NetHack をビルド → libnetHack.a をリンク | 複雑な C 依存を避けられる、確実 | 既存ビルドシステムに依存 | 少 |
| **B** | 段階的：必要な .c ファイルのみ厳選 | 最小限の依存 | デバッグが複雑 | 中 |
| **C** | bindgen を高度に設定（wrapper.h + clang-sys） | 柔軟で学習になる | 環境問題（libclang-dev 等） | 多 |

**推奨：アプローチ A → B の組み合わせ**

---

## Phase 1b: 実装計画（推奨順序）

### Step 1: NetHack ビルドシステムの確認

```bash
cd /home/oosawak/Workspace/NetHack
ls -la sys/unix/
head Configure
```

**目標：** 既存の `make` で `libnetHack.a` を生成できるか確認

### Step 2: build.rs で既存ビルド出力をリンク

```rust
// crates/nethack-sys/build.rs
fn main() {
    let nethack_root = /* ... */;
    
    // オプション 1: NetHack の既存 Makefile を使う
    // std::process::Command を使って make を実行
    let _ = Command::new("make")
        .current_dir(&nethack_root)
        .arg("clean")
        .arg("all")
        .output();
    
    // オプション 2: 生成された .a をリンク
    println!("cargo:rustc-link-search=native={}", nethack_root.join("src").display());
    println!("cargo:rustc-link-lib=static=nethack");
}
```

### Step 3: 最小限の bindgen ヘッダセット

既存の複雑な hack.h の代わりに、**抽出した簡潔なヘッダ**を使う：

```c
// wrapper.h (シンプル版)
#define UNIX 1
#define SYSV 1

// 型宣言のみ（実装は C に任せる）
typedef int coordxy;
typedef unsigned char schar;

// 必要な構造体だけ
#include "coord.h"
#include "you.h"
#include "monst.h"
#include "obj.h"
```

### Step 4: bindgen 実行と FFI 生成

```rust
// build.rs の generate_bindings() 関数
fn generate_bindings(wrapper_h: &Path) {
    let bindings = bindgen::Builder::default()
        .header(wrapper_h.to_string_lossy().into_owned())
        .clang_arg(/* ... */)
        .generate()
        .expect("Failed to generate bindings");
    
    bindings.write_to_file("src/ffi.rs").unwrap();
}
```

### Step 5: 安全な Rust ラッパーの実装

```rust
// crates/nethack-sys/src/wrapper.rs
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref GAME_STATE: Mutex<GameState> = Mutex::new(GameState::new());
}

pub struct GameState {
    initialized: bool,
}

impl GameState {
    pub fn init() -> Result<(), String> {
        let mut state = GAME_STATE.lock().unwrap();
        unsafe {
            ffi::early_init(0, std::ptr::null_mut());
        }
        state.initialized = true;
        Ok(())
    }
    
    pub fn player_pos() -> (i32, i32) {
        unsafe {
            (ffi::u.ux as i32, ffi::u.uy as i32)
        }
    }
}
```

---

## テスト計画（Phase 1c）

### Unit Test 1: FFI 基本

```rust
#[test]
fn test_ffi_imports() {
    // 型がインポートできるか
    let _x: nethack_sys::ffi::coordxy = 0;
    let _y: nethack_sys::ffi::you;
}
```

### Unit Test 2: 初期化

```rust
#[test]
fn test_game_init() {
    let result = nethack_sys::wrapper::GameState::init();
    assert!(result.is_ok());
}
```

### Integration Test: ゲーム状態読取

```rust
#[test]
fn test_player_position() {
    nethack_sys::wrapper::GameState::init().unwrap();
    let (x, y) = nethack_sys::wrapper::GameState::player_pos();
    assert!(x >= 0 && y >= 0);
}
```

---

## Timeline

| フェーズ | 内容 | 予定時間 |
|---|---|---|
| **1a** ✅ | FFI インフラ準備 | 完了 |
| **1b** | C ビルド + bindgen | 1-2 日 |
| **1c** | テスト + デバッグ | 1 日 |
| **1d** | nethack-core への統合 | 1 日 |

---

## 次のアクション

ユーザーの指示をお待ちします：

1. **アプローチ A を進めたい** → NetHack の既存 Makefile を活用
2. **アプローチ B で細かく実装したい** → C ファイルを慎重に選別
3. **別のアプローチを提案したい** → 教えてください

現在の状態は **Phase 1a ✅完了** で、全ワークスペースが正常にビルドされています。
