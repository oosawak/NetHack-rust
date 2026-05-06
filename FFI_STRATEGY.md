# FFI-First Strategy: C Library as Stable Foundation

## 質問への回答

> C 言語で書かれてるものは？Rust で書き直す意味はありますか？

**答え：** 既存 C コードの ~98% はそのまま利用。書き直すのは **グラフィック層と WASM/Unity ラッパーだけ**。

---

## 何を C のまま保つのか

| コンポーネント | 行数 | 理由 | 状態 |
|---|---:|---|---|
| **ゲームロジック** | 200,000+ | ダンジョン生成、AI、戦闘、アイテムシステムは安定・テスト済み | ✅ C 維持 |
| **共通ユーティリティ** | 20,000+ | 文字列処理、乱数生成、メモリ管理 | ✅ C 維持 |
| **プラットフォーム層** | 3,000 | Unix/Windows/Mac の差分（signal、fork など） | ⚠️ Rust抽象化 |
| **ウィンドウ管理** | 2,000 | X11、Win32、Cocoa ネイティブ UI | ✅ Rust（wgpu+winit）に置換 |
| **描画エンジン** | 300 | ASCII レンダリング → wgpu に置換 | ✅ Rust（wgpu）に置換 |

**合計で Rust に書くのは：**
- グラフィック層 → 300 行
- FFI ブリッジ → 500 行
- デスクトップ UI → 2,000 行
- WASM/Unity ラッパー → 1,500 行

**= 約 4,300 行の Rust コード** 対 **250,000 行の既存 C（活用）**

---

## アーキテクチャ図

```
┌─────────────────────────────────────────────────────┐
│  Desktop (winit) / Browser (WebGPU) / Unity (C#)   │
├─────────────────────────────────────────────────────┤
│              Rust: Graphics Layer                   │
│  • wgpu レンダリング                                 │
│  • Camera3D（視点切り替え）                         │
│  • Input handling                                   │
├─────────────────────────────────────────────────────┤
│        Rust: Game Bridge (FFI ラッパー)             │
│  • C ライブラリとの データ やり取り                 │
│  • エラーハンドリング統一                           │
├─────────────────────────────────────────────────────┤
│     C Library (bindgen で自動バインド)              │
│  • ゲームロジック（AI、戦闘、アイテム）            │
│  • ダンジョン生成                                   │
│  • セーブ/ロード                                    │
│  • プレイヤー操作の核（mov_on_monster など）       │
└─────────────────────────────────────────────────────┘
```

---

## FFI バウンダリ（C ↔ Rust の境界）

### C が提供する関数（べき）

```c
// プレイヤー座標
int* get_player_pos(int* x, int* y);

// モンスター・アイテム列挙
struct monster* get_monsters(int* count);
struct obj* get_items(int* count);

// ゲーム状態読み取り（レンダリング用）
int get_map_tile(int x, int y);  // '@', 'd', '$' など
int get_tile_color(int x, int y);

// ゲームロジック実行
int execute_command(int cmd);  // 'h'(左)、'j'(下) など
int advance_turn();             // AI, monster move

// 状態管理
int save_game(const char* path);
int load_game(const char* path);
```

### Rust が実装する

```rust
// wgpu による描画
fn render_frame(camera: &Camera3D, game_state: &GameState);

// Camera3D システム（視点切り替え）
fn switch_view_mode(mode: ViewMode);

// プラットフォーム抽象化
pub trait IO {
    fn read_file(&self, path: &str) -> Result<Vec<u8>>;
    fn write_file(&self, path: &str, data: &[u8]) -> Result<()>;
}
```

---

## ビルドフロー

```bash
# Phase 1: C コードを libnetHack.a にコンパイル
cd crates/nethack-sys
cargo build --release
  ↓
  cc クレート が /home/oosawak/Workspace/NetHack/src/*.c をコンパイル
  bindgen が include/hack.h を Rust FFI に変換
  → libnetHack.a + ffi.rs が生成される

# Phase 2-3: Rust レイヤーをコンパイル・リンク
cargo build -p nethack-desktop --release
  ↓
  nethack-sys の libnetHack.a とリンク
  → 単一の実行ファイル

# WASM
wasm-pack build --target web crates/nethack-wasm --release
  ↓
  Emscripten で C ライブラリをコンパイル（WASM に）
  Rust FFI を WebAssembly に変換
  → .wasm + .js glue
```

---

## WASM での工夫（C → WASM 移植）

NetHack C コード に問題がある機能：

| 機能 | 問題 | 解決方法 |
|---|---|---|
| `fork()` | WASM に無い | 回避：シングルスレッド実行 |
| `signal()` | WASM に無い | 回避：同期実行（タイムアウト代わりに UI で中断） |
| `fopen()` / `open()` | ファイルシステム無し | 解決：IndexedDB または localStorage 使用 |
| `getenv()` | 環境変数無し | 回避：初期化パラメータで渡す |

**実装：**
- Rust 層で `FILE*` → `Vec<u8>` に抽象化
- WASM では IndexedDB から読み込み
- デスクトップでは `std::fs` から読み込み

---

## 効率性の比較

### 案 A: 全て Rust に書き直す
- 工数：12-18 ヶ月
- 品質リスク：AI・戦闘ロジック再実装のバグ
- **メリット：** 言語統一
- **デメリット：** 莫大な時間浪費

### 案 B: FFI-First（提案した案）
- 工数：3-5 ヶ月
- 品質リスク：低い（既存 C コードは battle-tested）
- **メリット：** 高速実装、品質保証
- **デメリット：** C/Rust 両言語習熟が必要（でも最小限）

---

## 実装優先度

### Phase 1（今）- FFI 準備
1. `build.rs` で C コードを libnetHack.a にコンパイル
2. `bindgen` で FFI 自動生成
3. C 関数を Rust から呼べるか確認

### Phase 2 - Rust 描画層
1. wgpu パイプライン実装
2. Camera3D を wgpu に統合

### Phase 3 - デスクトップ完成
1. winit + wgpu ウィンドウ統合
2. キー入力 → C 関数呼び出し → 描画

### Phase 4 - WASM
1. emscripten で C コード → WASM
2. JS から Rust API 呼び出し

### Phase 5 - Unity
1. cdylib ビルド
2. C# `DllImport` バインディング

---

## メモリ管理の注意

C ライブラリのグローバル変数・構造体：

```c
// hack.h より
extern struct you {
    int ux, uy;  // player position
    // ...
} u;

extern struct monst *fmon;  // monster list
extern struct obj *fobj;    // item list
```

**Rust での読み取り：**
```rust
let player_x = unsafe { ffi::u.ux };
let player_y = unsafe { ffi::u.uy };
```

**安全性：**
- `unsafe` ブロックで明示的に「C 呼び出し」を記述
- 所有権はないが、C が管理している限りは安全
- Rust 側で C メモリを`free()` してはいけない

---

## 次のステップ

1. ✅ Phase 0: ワークスペース & Camera3D システム 完了
2. ⏳ Phase 1: C ライブラリを FFI 化
   - `build.rs` 実装（cc クレート）
   - `bindgen` 実行
   - 簡単な関数呼び出しテスト
3. ⏳ Phase 2: wgpu 描画層
4. ⏳ Phase 3: デスクトップアプリ統合

---

## まとめ

| 項目 | 戦略 |
|---|---|
| ゲームロジック | **C のまま** ← 既に 30 年テスト済み |
| グラフィック | **Rust + wgpu** ← モダン描画 |
| プラットフォーム | **Rust + trait** ← 抽象化で WASM/Unity 対応 |
| コード量削減 | **FFI-First** ← 250k行に対 4.3k行だけ書く |
| 実装期間 | **3-5 ヶ月** ← 全 Rust 化は 12-18 ヶ月 |
