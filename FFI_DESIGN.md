# NetHack C→Rust FFI 設計ドキュメント

## 概要

NetHack 5.0 は以下の層で構成されている：

```
┌─────────────────────────────────────────────┐
│ UI Layer (winprocs.h vtable)                │ ← Rust で実装
│ • win_print_glyph() - 画面描画              │
│ • win_get_nh_event() - キー入力             │
│ • win_init_nhwindows() - UI初期化           │
└─────────────────────────────────────────────┘
            ↓ (callback interface)
┌─────────────────────────────────────────────┐
│ Game Engine (C library)                     │ ← そのまま使用
│ • moveloop() - ゲームループ                  │
│ • newgame() / restore game                  │
│ • Global variables: struct you u            │
│ • Display/rendering helper functions        │
└─────────────────────────────────────────────┘
```

---

## ファイル構成の分析

### ヘッダファイル（88個）

**主要なヘッダ：**

| ヘッダ | 用途 | 行数 | 重要度 |
|---|---|---:|---|
| `hack.h` | メインヘッダ（全構造体インクルード） | 80+ | ★★★ |
| `you.h` | プレイヤー構造体 (`struct you`) | 200+ | ★★★ |
| `monst.h` | モンスター構造体 (`struct monst`) | 150+ | ★★★ |
| `obj.h` | オブジェクト（アイテム） | 200+ | ★★★ |
| `extern.h` | 全 C 関数宣言 | 5,000+ | ★★★ |
| `winprocs.h` | UI vtable 定義 | 150+ | ★★ |
| `decl.h` | グローバル変数宣言 | 100+ | ★★ |
| `display.h` | 表示関連定義 | 100+ | ★ |
| `dungeon.h` | ダンジョン構造 | 100+ | ★★ |

### ソースファイル（130個）

**カテゴリ別分類：**

| カテゴリ | ファイル数 | 行数 | 説明 |
|---|---:|---:|---|
| **ゲームロジック** | 40+ | 150,000+ | AI・ダンジョン生成・戦闘 |
| **UI/表示** | 20+ | 50,000+ | display.c, botl.c, cmd.c など |
| **オブジェクト管理** | 15+ | 40,000+ | mkobj.c, invent.c など |
| **ユーティリティ** | 30+ | 30,000+ | 文字列、乱数、メモリなど |
| **プラットフォーム** | 25+ | 3,000 | sys/unix/*.c, sys/windows/*.c |

---

## グローバル変数（重要）

### プレイヤー状態
```c
extern struct you u;  // src/decl.c:102

struct you {
    xint16 ux, uy;           // ← 座標（デッサン用）
    schar ulevel;            // レベル
    unsigned int uexp;        // 経験値
    struct prop *uprops;     // 特性
    struct obj *usteed;      // 乗っているペット
    // ... 50+ メンバー
};
```

### グローバル状態
```c
// src/decl.c より
extern struct monst *fmon;         // モンスターリストの先頭
extern struct obj *fobj;           // オブジェクトリストの先頭
extern struct monst youmonst;      // プレイヤーをモンスターとして
extern NEARDATA int dlevel;        // 現在のダンジョンレベル
extern NEARDATA int moves;         // ゲームターン数
extern NEARDATA struct obj *invent; // インベントリ
```

---

## UI インターフェース（winprocs.h）

### 主要な callback 関数

**描画関数：**
```c
void (*win_print_glyph)(
    winid, 
    coordxy x, coordxy y,      // 座標（0-79, 0-20）
    const glyph_info *g,       // グリフ情報（文字＋色）
    const glyph_info *bg       // 背景
);
```

**入力関数：**
```c
int (*win_nhgetch)(void);      // キー入力（1文字）
int (*win_nh_poskey)(int *x, int *y, int *mod);  // マウス位置
```

**初期化：**
```c
void (*win_init_nhwindows)(int *argc, char **argv);
void (*win_exit_nhwindows)(const char *msg);
```

### 構造体定義

```c
struct window_procs {
    const char *name;           // e.g., "wgpu"
    enum wp_ids wp_id;          // ID（要追加）
    unsigned long wincap;       // 機能フラグ
    unsigned long wincap2;      // 追加機能フラグ
    boolean has_color[CLR_MAX]; // 色サポート
    
    // ... 30+ 関数ポインタ
    void (*win_print_glyph)(...);
    void (*win_putstr)(...);
    // ... etc
};
```

---

## ゲームループの流れ

### Phase 0: 初期化
```c
// allmain.c
int main(int argc, char *argv[]) {
    early_init(argc, argv);          // グローバル変数初期化
    iflags.wc_tiled_map = 1;         // UI フラグ設定
    startup_io();                    // I/O 初期化
    
    if (validate()) {                // 検証
        display_gamewindows();       // UI 初期化 (win_init_nhwindows 呼び出し)
        
        if (new_game) {
            newgame();               // 新規ゲーム
        } else {
            restore_game();          // セーブ読込
        }
    }
    
    moveloop(FALSE);                 // ゲームループ開始
}
```

### Phase 1: ゲームループ
```c
// allmain.c: moveloop()
void moveloop(boolean resuming) {
    moveloop_preamble(resuming);     // 準備
    
    while (1) {
        // ターン実行
        (void) turn_on_new_turn();
        
        // プレイヤー入力待機
        get_nh_event();              // win_get_nh_event() → キー入力
        
        // コマンド実行
        if (iflags.at_a_glance) {
            look_here(u.ux, u.uy, FALSE);
        } else {
            (void) docommand();      // キー → ゲーム実行
        }
        
        // モンスター AI
        if (!flags.bypasses) {
            dmonsfight();            // AI ターン
        }
        
        // 画面更新
        docrt();                     // win_print_glyph() 呼び出し
        show_bothmonsters();
    }
}
```

---

## Rust から必要な FFI 関数

### グループ A: 初期化・終了（必須）

```c
void early_init(int argc, char *argv[]);
void newgame(void);
void display_gamewindows(void);
void moveloop(boolean resuming);
void end_game(int status);
void restore_game(void);
```

### グループ B: ゲーム状態読取（毎フレーム）

```c
// グローバル変数へのアクセス
extern struct you u;           // プレイヤー
extern int dlevel;             // ダンジョンレベル
extern int moves;              // ターン数
extern struct monst *fmon;     // モンスターリスト
extern struct obj *fobj;       // アイテムリスト

// 便利関数
struct monst *mon_at(int x, int y);           // 座標のモンスター
struct obj *obj_at(int x, int y);             // 座標のアイテム
int get_glyph(int x, int y);                  // 画面グリフ
const char *the_msg(int msgnum, const char *); // メッセージ
```

### グループ C: コマンド実行（入力ごと）

```c
// キー入力 → C 関数実行
int docommand(void);                // 1ターン実行
void do_move(int dx, int dy, int dz); // 移動
int doapply(void);                 // アイテム適用
int doquaff(void);                 // ポーション飲む
// ... 他 30+ コマンド関数
```

### グループ D: 描画情報取得

```c
// glyph 情報（Rust が描画する）
struct glyph_info {
    int glyph;        // グリフID
    int color;        // 色（0-15）
    int ch;           // 文字（' ' ~ '~'）
    int bkcolor;      // 背景色
};

void window_procs_init(struct window_procs *wp);
```

---

## Bindgen 実行計画

### Phase 1a: ヘッダファイルの準備

**必須ヘッダ（プロセス順）：**

1. **config.h** → ビルド定義
   - `#define WIN32`, `#define UNIX` など
   - `build.rs` で条件付きコンパイル

2. **you.h** → プレイヤー構造体
   - `struct you`（~50 メンバ）
   - `struct prop`, `struct obj*` 等の依存

3. **monst.h** → モンスター構造体
   - `struct monst`
   - `struct attack`, `struct mextra` 等

4. **obj.h** → アイテム構造体
   - `struct obj`
   - `struct objclass` 等

5. **extern.h** → 全 C 関数
   - ~2000 行の関数宣言
   - Bindgen の主対象

6. **winprocs.h** → UI vtable
   - `struct window_procs`
   - Rust で実装対象

### Phase 1b: Bindgen 設定（build.rs）

```rust
// crates/nethack-sys/build.rs
let bindings = bindgen::Builder::default()
    .header("/path/to/NetHack/include/hack.h")
    .clang_arg("-DCONFIG_H")
    .clang_arg("-DUNIX")  // プラットフォーム
    .derive_copy(true)
    .derive_debug(true)
    .allowlist_function("^(newgame|restore_game|moveloop|docommand)")
    .allowlist_var("^(u|fmon|fobj|dlevel|moves)")
    .allowlist_type("^(you|monst|obj|window_procs|glyph_info)")
    .generate()
    .expect("Unable to generate bindings");
```

---

## build.rs で C コンパイル

### 処理フロー

```rust
// crates/nethack-sys/build.rs

fn main() {
    // 1. NetHack ソースを特定
    let nh_root = "/home/oosawak/Workspace/NetHack";
    let src_dir = format!("{}/src", nh_root);
    
    // 2. コンパイル対象ファイル
    let files = vec![
        // グローバル初期化
        "decl.c", "alloc.c",
        // ゲームメイン
        "allmain.c", "cmd.c", "do.c", "apply.c",
        // モンスター・オブジェクト
        "mondata.c", "mon.c", "monmove.c",
        "mkobj.c", "invent.c", "obj.c",
        // ダンジョン・表示
        "dungeon.c", "display.c", "botl.c",
        // ユーティリティ
        "rnd.c", "hacklib.c", "strutil.c",
        // その他
        "save.c", "restore.c", "bones.c",
        // ... etc
    ];
    
    // 3. Cコンパイラ設定
    let mut cc = cc::Build::new();
    cc.flag("-std=c99")
      .flag("-Wall")
      .include(format!("{}/include", nh_root))
      .include(format!("{}/sys/share", nh_root))
      .define("UNIX", "1")
      .define("SYSV", "1")
      .opt_level(2);
    
    // 4. ファイルを追加
    for file in files {
        cc.file(format!("{}/{}", src_dir, file));
    }
    
    // 5. コンパイル
    cc.compile("nethack");
    
    // 6. Bindgen で FFI 生成
    // ... (上記参照)
}
```

---

## テスト計画（Phase 1c）

### Step 1: Simple Function Call
```rust
#[test]
fn test_ffi_basics() {
    unsafe {
        // グローバル変数へのアクセス
        let player_x = ffi::u.ux;
        let player_y = ffi::u.uy;
        assert!(player_x >= 0);
        
        // 関数呼び出し
        ffi::early_init(0, std::ptr::null_mut());
        // グローバル状態が初期化される
    }
}
```

### Step 2: Game Loop Test
```rust
#[test]
fn test_newgame() {
    unsafe {
        ffi::newgame();
        // プレイヤーが Level 1 に生成される
        assert_eq!(ffi::dlevel, 1);
        assert!(ffi::u.ux > 0 && ffi::u.uy > 0);
    }
}
```

### Step 3: Command Execution
```rust
#[test]
fn test_move_command() {
    unsafe {
        ffi::newgame();
        let old_x = ffi::u.ux;
        
        // 右に移動（'l' コマンド）
        // (実装は複雑なため省略)
        
        // assert_eq!(ffi::u.ux, old_x + 1);
    }
}
```

---

## コンパイル時の課題と対処

### 課題 1: ヘッダインクルードの循環参照

NetHack は多くのヘッダが相互参照している。

**解決：** `hack.h` だけを Bindgen の target にして、依存関係を自動解決させる。

```rust
.header("/path/to/hack.h")  // これ 1 個で足りる
```

### 課題 2: platform conditional (`#ifdef UNIX`, `#ifdef WIN32`)

**解決：** `build.rs` で clang_arg で define する。

```rust
.clang_arg("-DUNIX")        // UNIX 版
.clang_arg("-DWIN32")       // Windows 版
```

### 課題 3: グローバル変数の初期化

C コードは複数の `static` 初期化に依存している。

**解決：** `early_init()` を必ず呼んでから他の関数を使う。

```rust
unsafe {
    ffi::early_init(0, std::ptr::null_mut());
    // 初期化完了
}
```

---

## FFI バウンダリの設計

### C 側で提供すべき安全な API

**問題：** C の グローバル `struct you u` には threading 安全性がない。

**解決：** Rust 側で mutex でラップ。

```rust
lazy_static::lazy_static! {
    static ref GAME_STATE: Mutex<GameState> = Mutex::new(GameState::new());
}

pub struct GameState {
    // C のグローバル変数はここに抽象化
}

impl GameState {
    pub fn player_pos(&self) -> (i32, i32) {
        unsafe {
            (ffi::u.ux as i32, ffi::u.uy as i32)
        }
    }
}
```

---

## 次のステップ

1. **Phase 1a: build.rs コンパイル設定**
   - cc クレート の設定
   - ファイルリスト確定

2. **Phase 1b: Bindgen 実行**
   - hack.h から FFI 自動生成
   - ffi.rs 確認

3. **Phase 1c: テスト**
   - simple function call
   - game loop initialization
   - glyph/player state access

4. **Phase 2: Rust wrapper**
   - GameBridge 構造体
   - C ↔ Rust の safe boundary
