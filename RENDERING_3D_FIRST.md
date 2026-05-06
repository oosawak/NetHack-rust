# NetHack 3D-First レンダリング設計

## 戦略の転換：ASCII → 2.5D → 3D ❌
## 新戦略：3D-First（異なる視点投影） ✅

### なぜ3D-Firstなのか

**従来のアプローチの問題:**
```
ASCII Pipeline → Tile Pipeline → 2.5D Pipeline → 3D Pipeline
    ↑              ↑               ↑               ↑
  コード        コード          コード           コード
 重複            重複            重複           増加
```

**3D-First アプローチ:**
```
3D Engine (共通)
├─ 3D Grid System
├─ 3D Camera (視点切り替え可能)
├─ 3D Mesh Renderer
└─ 3D Physics (衝突判定)
    ↓
視点プロジェクション選択
├─ Top-Down View (俯瞰 / ASCII的)
├─ Isometric View (等角)
├─ First-Person View (1人称 / Classic NetHack)
└─ Cinematic View (スクリーン等角 / Diablo風)
```

**利点:**
- ✅ コードの重複なし
- ✅ 視点切り替えは「カメラ設定変更」のみ
- ✅ 将来の拡張（VR, 3D物理等）が容易
- ✅ モダンなゲーム開発標準

---

## 3D-First アーキテクチャ

### 1. 座標系の統一

```rust
// すべての内部表現を 3D 座標で統一
pub struct Position3D {
    x: f32,  // 東西
    y: f32,  // 高さ
    z: f32,  // 南北
}

// グリッド座標を3D座標に変換
impl From<GridPos> for Position3D {
    fn from(grid: GridPos) -> Self {
        Position3D {
            x: grid.x as f32,
            y: 0.0,  // ダンジョン階層で変更可能
            z: grid.y as f32,
        }
    }
}
```

### 2. 3D ゲーム状態

```rust
// nethack-core/src/game_3d.rs

pub struct Dungeon3D {
    levels: Vec<Level>,  // 各階層
    current_level: usize,
    entities: Vec<Entity3D>,  // プレイヤー、モンスター、アイテム
    camera: Camera3D,
}

pub struct Entity3D {
    pos: Position3D,
    glyph: Glyph,  // ASCIIコード or tile ID
    mesh: Option<MeshHandle>,  // 3D メッシュへのリファレンス
}

pub struct Level {
    width: usize,
    height: usize,
    tiles: Vec<Vec<Tile>>,
    // 各タイルの高さ（床、壁、段差等）
    heights: Vec<Vec<f32>>,
}
```

### 3. カメラシステム（視点統一）

```rust
// nethack-render/src/camera.rs

pub enum CameraMode {
    TopDown,        // 俯瞰（ASCII的）
    FirstPerson,    // 1人称（Classic NetHack）
    Isometric,      // 等角（Diablo風）
    ThirdPerson,    // 3人称（背後視点）
    Cinematic,      // シネマティック（自動カメラ）
}

pub struct Camera3D {
    position: Vec3,           // ワールド座標
    target: Vec3,             // 注視点
    up: Vec3,                 // アップベクトル
    fov: f32,                 // 視野角
    mode: CameraMode,
}

impl Camera3D {
    pub fn look_at_tile(x: f32, y: f32, z: f32, mode: CameraMode) -> Self {
        match mode {
            CameraMode::TopDown => {
                // 真上から見下ろす
                Self {
                    position: Vec3::new(x, 50.0, z),  // y が高い = 上空
                    target: Vec3::new(x, 0.0, z),
                    up: Vec3::new(0.0, 0.0, -1.0),
                    fov: 60.0,
                    mode,
                }
            }
            CameraMode::FirstPerson => {
                // プレイヤー目線
                Self {
                    position: Vec3::new(x, 1.7, z),   // 人間の目の高さ
                    target: Vec3::new(x, 1.7, z + 5.0),  // 前方を見る
                    up: Vec3::new(0.0, 1.0, 0.0),
                    fov: 90.0,
                    mode,
                }
            }
            CameraMode::Isometric => {
                // 45度傾斜
                Self {
                    position: Vec3::new(x + 20.0, 20.0, z + 20.0),
                    target: Vec3::new(x, 0.0, z),
                    up: Vec3::new(0.0, 1.0, 0.0),
                    fov: 60.0,
                    mode,
                }
            }
            CameraMode::ThirdPerson => {
                // 背後から
                Self {
                    position: Vec3::new(x, 2.0, z - 5.0),
                    target: Vec3::new(x, 1.5, z),
                    up: Vec3::new(0.0, 1.0, 0.0),
                    fov: 75.0,
                    mode,
                }
            }
            CameraMode::Cinematic => {
                // 動的カメラ
                Self {
                    position: Vec3::new(x + 15.0, 15.0, z + 15.0),
                    target: Vec3::new(x, 1.0, z),
                    up: Vec3::new(0.0, 1.0, 0.0),
                    fov: 50.0,
                    mode,
                }
            }
        }
    }

    pub fn view_projection(&self, aspect: f32) -> Mat4 {
        let view = Mat4::look_at_rh(self.position, self.target, self.up);
        let proj = Mat4::perspective_rh(
            self.fov.to_radians(),
            aspect,
            0.1,
            1000.0,
        );
        proj * view
    }

    pub fn switch_mode(&mut self, mode: CameraMode) {
        self.mode = mode;
        // カメラ位置を補間で変更（スムーズ遷移）
    }
}
```

### 4. 統一レンダリング パイプライン

```rust
// nethack-render/src/renderer_3d.rs

pub struct Renderer3D {
    device: Device,
    queue: Queue,
    pipeline: RenderPipeline,
    
    // 共通リソース
    vertex_buffer: Buffer,     // すべてのメッシュ頂点
    index_buffer: Buffer,      // すべてのメッシュインデックス
    texture_atlas: Texture,    // タイル/ASCII フォント統合
    
    // 状態
    camera: Camera3D,
    entities: Vec<RenderEntity>,
}

pub struct RenderEntity {
    pos: Vec3,
    glyph: Glyph,
    mesh_index: usize,    // どのメッシュを使うか
    color: [f32; 4],
}

impl Renderer3D {
    pub fn render_frame(&mut self, view: &TextureView) -> Result<()> {
        let mut encoder = self.device.create_command_encoder(&Default::default());
        
        // View-Projection 行列を計算
        let view_proj = self.camera.view_projection(800.0 / 600.0);
        
        {
            let mut rpass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("main_pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color {
                            r: 0.1,
                            g: 0.1,
                            b: 0.2,
                            a: 1.0,
                        }),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            rpass.set_pipeline(&self.pipeline);
            
            // 各エンティティを描画
            for entity in &self.entities {
                let model = Mat4::from_translation(entity.pos);
                let mvp = view_proj * model;
                
                // Uniform buffer を更新
                let uniform_data = mvp.to_cols_array();
                self.queue.write_buffer(
                    &self.uniform_buffer,
                    0,
                    bytemuck::cast_slice(&uniform_data),
                );
                
                rpass.set_bind_group(0, &self.bind_group, &[]);
                rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
                rpass.set_index_buffer(
                    self.index_buffer.slice(..),
                    wgpu::IndexFormat::Uint32,
                );
                rpass.draw_indexed(0..6, 0, 0..1);  // 1 quad per entity
            }
        }
        
        self.queue.submit(std::iter::once(encoder.finish()));
        Ok(())
    }

    /// カメラモード切り替え
    pub fn set_camera_mode(&mut self, mode: CameraMode) {
        let center = Vec3::new(40.0, 0.0, 12.0);  // 現在の視点中心
        self.camera = Camera3D::look_at_tile(
            center.x,
            center.y,
            center.z,
            mode,
        );
    }
}
```

### 5. シェーダ（視点非依存）

```wgsl
// nethack-render/shaders/main_3d.wgsl

struct VertexInput {
    @location(0) position: vec3f,
    @location(1) uv: vec2f,
    @location(2) color: vec4f,
}

struct VertexOutput {
    @builtin(position) position: vec4f,
    @location(0) uv: vec2f,
    @location(1) color: vec4f,
}

@group(0) @binding(0)
var<uniform> mvp: mat4x4f;

@group(0) @binding(1)
var tex: texture_2d<f32>;

@group(0) @binding(2)
var tex_sampler: sampler;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.position = mvp * vec4f(input.position, 1.0);
    output.uv = input.uv;
    output.color = input.color;
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4f {
    let tex_color = textureSample(tex, tex_sampler, input.uv);
    return tex_color * input.color;
}
```

---

## 実装ロードマップ：3D-First

| フェーズ | 目標 | 描画方式 | 所要時間 |
|---|---|---|---|
| **Phase 0** ✅ | Cargo workspace | - | 完了 |
| **Phase 1** | FFI + Rust core | - | 1-2週間 |
| **Phase 2** | 3D基盤（座標系、カメラ） | - | 1-2週間 |
| **Phase 3** | TopDown View | 俯瞰ASCII的 | 1週間 |
| **Phase 4** | FirstPerson View | Classic 1人称 | 1週間 |
| **Phase 5** | Isometric View | Diablo風等角 | 1週間 |
| **Phase 6** | WASM統合 | WebGPU | 1-2週間 |
| **Phase 7** | Unity Plugin | cdylib | 1-2週間 |
| **Bonus** | VR Support | VR UX | 2-4週間 |

---

## 視点のビジュアル比較

```
同じ3Dワールド、異なる投影：

TopDown:
  ▯ ▯ ▯
  ▯ @ ▯ ← 俯瞰
  ▯ ▯ ▯

FirstPerson:
     ▯       ← 敵
   ▯ ▯ ▯
   ▯ @ ▯ ← あなたの視点
   ▯ ▯ ▯

Isometric:
  ◇ ◇ ◇
 ◇ @ ◇ ← 45度傾斜
◇ ◇ ◇

Cinematic (自動カメラ):
          ← カメラが自動で移動・回転
    @  ◇
   ◇ ◇ ◇
```

---

## 3D-First のメリット

| 観点 | メリット |
|---|---|
| **開発効率** | コード重複なし、視点は単なる「カメラ設定」 |
| **スケーラビリティ** | 新しい視点追加 = `Camera3D::new_view()` 関数追加 |
| **モダン性** | 2024年のゲーム開発標準（Unity, Unreal）に準拠 |
| **拡張性** | VR, AR, 360°ビュー等の将来対応が容易 |
| **パフォーマンス** | wgpu 3Dパイプラインを最大活用 |
| **WASM対応** | WebGPU（Chrome 120+, Firefox等）で3D表示可能 |
| **Unity統合** | 3D データをそのまま Unity に共有可能 |

---

## 段階実装戦略

### Step 1: 3D 基盤構築（Phase 2）
```rust
// nethack-core/src/world_3d.rs
pub struct World3D {
    dungeon: Dungeon3D,
    player: Entity3D,
    entities: Vec<Entity3D>,
}

impl World3D {
    pub fn new() -> Self { /* ... */ }
    pub fn update_player(&mut self, command: PlayerCommand) { /* ... */ }
    pub fn get_view_projection(&self) -> Mat4 {
        self.dungeon.camera.view_projection(16.0 / 9.0)
    }
}
```

### Step 2: TopDown ビュー（Phase 3・最初のマイルストーン）
```rust
// デフォルトカメラ：TopDown
world.camera.set_mode(CameraMode::TopDown);
world.camera.position = Vec3::new(player_x, 50.0, player_z);
```

視覚的には ASCII と同じだが、内部は完全な 3D 座標系。

### Step 3: 他の視点を追加（Phase 4-5）
```rust
// ユーザーがキーボードで視点切り替え
match input {
    Key::Number1 => world.camera.set_mode(CameraMode::TopDown),
    Key::Number2 => world.camera.set_mode(CameraMode::FirstPerson),
    Key::Number3 => world.camera.set_mode(CameraMode::Isometric),
    Key::Number4 => world.camera.set_mode(CameraMode::Cinematic),
    _ => {}
}
```

---

## 結論

**3D-First approach では：**
- ✅ ASCII/2.5D/3D/VR すべてを同じエンジンで実装
- ✅ 視点切り替えは単なる「カメラ設定」
- ✅ 将来の拡張が容易
- ✅ モダンなゲーム開発標準に準拠

**あなたの直感は正しい！** 最初から3Dで設計することで、
より柔軟で拡張性の高いシステムが実現できます。

