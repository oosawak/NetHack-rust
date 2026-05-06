# Architecture Guide

## システムアーキテクチャ概観

```
┌─────────────────────────────────────────────────────────────────────┐
│                     User Interface Layer                             │
│  Desktop (winit + wgpu) | WASM (WebGL) | Unity (C# + native code)  │
└────────────────────┬────────────────────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────────────────────┐
│                  Game Bridge Layer (Rust)                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │ GameBridge struct:                                          │  │
│  │  - state: GameState (C FFI wrapper)                        │  │
│  │  - world: World3D (camera, entities)                       │  │
│  │  - renderer: WgpuRenderer                                  │  │
│  └──────────────────────────────────────────────────────────────┘  │
└────────────────────┬────────────────────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────────────────────┐
│                    FFI Layer (Rust)                                 │
│  nethack-sys (bindgen-generated ffi.rs + safe wrappers)           │
│                                                                      │
│  Public API:                                                        │
│  - init_stage_1_early() ... init_stage_7_newgame()                │
│  - full_init() (convenience)                                       │
│  - get_dungeon_level(), player_pos() [planned]                    │
└────────────────────┬────────────────────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────────────────────┐
│              C Library (NetHack 5.0 Core)                           │
│  139 object files:                                                  │
│  - Game logic (allmain.c, cmd.c, do.c, etc.)                      │
│  - AI & dungeon generation (monmove.c, mklev.c, etc.)            │
│  - Item & monster management (objects.c, mkobj.c, etc.)          │
│  - Save/restore (save.c, restore.c)                              │
│                                                                      │
│  Global state:                                                      │
│  - struct you u (player)                                          │
│  - int dlevel (dungeon level)                                     │
│  - struct monst *fmon (monster list)                             │
│  - struct obj *fobj (object list)                                │
└─────────────────────────────────────────────────────────────────────┘
```

---

## コンポーネント詳細

### 1. nethack-sys (FFI Layer)

**責務:**
- C ライブラリとのバインディング（bindgen 自動生成）
- ゲーム初期化ステージング
- グローバル C 変数への安全なアクセス

**主要モジュール:**
```rust
pub enum InitStage {
    Uninitialized,
    EarlyInit,              // early_init() 実行
    WindowsChosen,          // choose_windows() 実行
    OptionsInitialized,     // initoptions() 実行
    WindowsInitialized,     // init_nhwindows() 実行
    DlbInitialized,         // dlb_init() 実行
    VisionInitialized,      // vision_init() 実行
    GameReady,              // newgame() 実行
}

pub struct GameState {
    stage: InitStage,
}
```

**依存関係:**
- `lazy_static` — グローバル GameState 管理
- `bindgen` — C FFI 自動生成

**テスト:**
- Stage transition テスト
- Global variable access テスト

---

### 2. nethack-core (Game State)

**責務:**
- 3D ワールド状態管理
- カメラシステム
- エンティティ管理

**主要構造体:**
```rust
pub struct Camera3D {
    position: Vec3,
    target: Vec3,
    up: Vec3,
    fov: f32,
}

pub enum ViewMode {
    TopDown,        // y=50, 俯瞰
    Isometric,      // 45° 角度
    FirstPerson,    // 目線高さ
    ThirdPerson,    // プレイヤーの後ろ
    Cinematic,      // ドラマティック
}

pub struct World3D {
    entities: Vec<Entity>,
    camera: Camera3D,
}
```

**テスト:**
- Camera creation & switching (6 テスト、全パス)

---

### 3. nethack-render (Placeholder)

**責務（実装予定）:**
- wgpu レンダリングパイプライン
- シェーダー管理
- テクスチャ・メッシュ管理

**計画:**
```rust
pub struct WgpuRenderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    render_pipeline: wgpu::RenderPipeline,
    texture: wgpu::Texture,
}

impl WgpuRenderer {
    pub fn render(&self, world: &World3D, camera: &Camera3D) { }
}
```

---

### 4. nethack-desktop (Placeholder)

**責務（実装予定）:**
- ウィンドウ管理（winit）
- イベントループ
- キーボード入力ハンドリング
- ファイルI/O（セーブ/ロード）

**アーキテクチャ:**
```rust
pub struct Application {
    event_loop: EventLoop<()>,
    window: Window,
    renderer: WgpuRenderer,
    game_bridge: GameBridge,
}

impl Application {
    pub async fn run(self) { }
}
```

**ターゲット:**
- Windows (x86_64-pc-windows-msvc)
- macOS (aarch64-apple-darwin, x86_64-apple-darwin)
- Linux (x86_64-unknown-linux-gnu)

---

### 5. nethack-wasm (Placeholder)

**責務（実装予定）:**
- WebAssembly ビルド
- JS/Rust ブリッジ（wasm-bindgen）
- IndexedDB ファイルI/O

**アーキテクチャ:**
```rust
#[wasm_bindgen]
pub struct WasmGame {
    game_bridge: GameBridge,
}

#[wasm_bindgen]
impl WasmGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmGame { }
    
    pub fn init(&mut self) -> Result<(), String> { }
    
    pub fn send_command(&mut self, cmd: u8) { }
    
    pub fn render(&self, canvas_id: &str) { }
}
```

**JS 側インターフェース:**
```javascript
const game = new WasmGame();
await game.init();

// キー入力
document.addEventListener('keydown', (e) => {
    game.send_command(e.keyCode);
});

// レンダリング
const canvas = document.getElementById('game-canvas');
game.render(canvas);
```

---

### 6. nethack-unity (Placeholder)

**責務（実装予定）:**
- Unity ネイティブプラグイン（cdylib）
- C# DllImport インターフェース
- Unity イベントループ統合

**C# インターフェース:**
```csharp
[DllImport("nethack_unity")]
static extern int nethack_init();

[DllImport("nethack_unity")]
static extern void nethack_send_command(byte cmd);

[DllImport("nethack_unity")]
static extern int nethack_get_dungeon_level();

public class NetHackGame : MonoBehaviour {
    void Start() {
        nethack_init();
    }
    
    void Update() {
        if (Input.GetKeyDown(KeyCode.UpArrow)) {
            nethack_send_command((byte)'k');
        }
    }
}
```

---

## データフロー

### ゲーム初期化フロー

```
main()
  ↓
nethack_sys::full_init()
  ├─ Stage 1: early_init()              [C] プログラム状態初期化
  ├─ Stage 2: choose_windows(1)         [C] UI システム選択
  ├─ Stage 3: initoptions()             [C] オプション読み込み
  ├─ Stage 4: init_nhwindows()          [C] ウィンドウシステム初期化
  ├─ Stage 5: dlb_init()                [C] ダンジョンDB初期化
  ├─ Stage 6: vision_init()             [C] 視野システム初期化
  └─ Stage 7: newgame()                 [C] ゲーム開始
      ├─ player_selection()
      ├─ u.ux = 0; u.uy = 0 (player位置)
      ├─ dlevel = 1 (ダンジョンレベル)
      └─ moveloop(FALSE) [無限ループ → 要ゲームループ実装]
```

### ゲームループフロー（実装予定）

```
Loop:
  ┌─ Read player input (winit / browser / Unity)
  │
  ├─ Call docommand() [C]
  │   └─ Updates: u.ux, u.uy, dlevel, fmon, fobj, etc.
  │
  ├─ Read game state [Rust]
  │   ├─ world.update_player(u.ux, u.uy)
  │   ├─ world.update_entities(fmon, fobj)
  │   └─ camera.follow(player_pos)
  │
  ├─ Render [wgpu]
  │   ├─ camera.view_projection()
  │   ├─ render_tiles(world.entities)
  │   └─ render_ui()
  │
  └─ Present to screen (vulkan/dx12/gl)
```

---

## メモリレイアウト

### C グローバル変数

```c
// From decl.h / allmain.c
struct you u;           // 1-2KB
int dlevel;            // 4 bytes
int dunlevs;           // 4 bytes
struct dungeon_topology dungeon;  // ~500 bytes
struct monst *fmon;    // Pointer (8 bytes)
struct obj *fobj;      // Pointer (8 bytes)
```

### Rust グローバル状態（lazy_static）

```rust
lazy_static! {
    static ref GAME_STATE: Mutex<GameState> = ...;
    static ref WORLD_STATE: Mutex<World3D> = ...;
}
```

---

## パフォーマンス考慮事項

### FFI オーバーヘッド

- `early_init()` など初期化関数: 呼び出しオーバーヘッド無視できる
- `docommand()` 毎フレーム呼び出し: unsafe ブロック最小化

### メモリ管理

- **C 側**: 既存メモリ管理（malloc/free）を保持
- **Rust 側**: `Mutex<GameState>` でグローバル状態を保護
- **FFI**: 所有権は C 側のまま（Rust は borrowing のみ）

### グラフィックス性能

- **タイル描画**: インスタンシング（1 drawcall で複数タイル）
- **UI レンダリング**: glyphon でテキスト高速化
- **フレームレート**: ターンベースなので 60 FPS は余裕

---

## 拡張ポイント

### 1. カメラシステム
```rust
// 新しい ViewMode を追加
enum ViewMode {
    TopDown,
    Isometric,
    // ... add custom modes
    Custom(CustomCamera),
}
```

### 2. レンダリングエンジン
```rust
// シェーダー追加
impl WgpuRenderer {
    fn add_shader(&mut self, name: &str, wgsl: &str) { }
}
```

### 3. プラットフォーム固有ロジック
```rust
#[cfg(target_arch = "wasm32")]
fn save_game() { /* IndexedDB */ }

#[cfg(not(target_arch = "wasm32"))]
fn save_game() { /* File IO */ }
```

---

## ライセンスと法的考慮

- **NetHack**: NGPL（NetHack General Public License）
- **Rust code**: NGPL 継承（派生物として）
- **wgpu**: MIT + Apache 2.0
- **winit**: MIT + Apache 2.0

詳細は [docs/LICENSE.md](./LICENSE.md) を参照

---

**最終更新**: 2026-05-06
