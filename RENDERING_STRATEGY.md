# NetHack レンダリング戦略：ASCII → 2.5D への段階的移行

## フェーズ別レンダリング

### Phase A: ASCII / Tiled Grid (基本・互換性重視)

```
████████████
█@░░░░░░░░█  ← プレイヤー (@)
█░████░░░░█  ← 壁 (#)
█░█░░░░░░░█  ← 床 (.)
█░░░░░░░░░█  ← モンスター (d, o, etc)
████████████
```

**実装:**
- フォント: monospace (ASCII glyphs)
- グリッド: 80×25 (classic NetHack)
- 出力: wgpu でテクスチャアトラス化

**利点:**
- 最小限の計算
- 既存NetHackと完全互換
- WASM で軽量

**拡張性:** ✅ 高い

---

### Phase B: Enhanced Tiles (ASCII の拡張)

```
同じグリッド構造で、より詳細なタイルセット

┌─────┬─────┬─────┐
│ 🧙  │  d  │  💰 │  ← グラフィカルタイル（512×512 スプライト）
├─────┼─────┼─────┤
│     │     │     │
└─────┴─────┴─────┘
```

**実装:**
- タイルサイズ: 32×32 or 64×64 px
- アトラス化: 複数タイルを1テクスチャにパック
- wgpu パイプライン: tile index → UV座標にマッピング

**実装コスト:** 中程度（タイルセット作成に時間）

---

### Phase C: 2.5D Isometric (段階的スケーリング)

```
  ◇ ◇ ◇
 ◇ 🧙 ◇  ← 等角投影（Isometric）
◇ ◇ ◇
```

**等角投影の計算:**
```rust
// Screen coordinates from grid coordinates
screen_x = (grid_x - grid_y) * tile_width / 2
screen_y = (grid_x + grid_y) * tile_height / 2

// または
screen_pos = grid_pos * [cos(45°), sin(45°)]
           + height * [sin(30°), -cos(30°)]
```

**特徴:**
- ✅ 奥行き感がある
- ✅ 高さ情報を表現可能 (ダンジョン階層・高さ)
- ✅ 従来のGridベースゲーム (Diablo, Baldur's Gate) と同じ
- ✅ wgpu で同じパイプラインを使用可能

**実装ステップ:**
1. Isometric view matrix の導入
2. タイル描画時に投影座標を計算
3. レイヤー合成（背景→床→オブジェクト→キャラ）

**コード例:**
```rust
pub struct IsometricCamera {
    // View-projection matrix for isometric view
    view_proj: [[f32; 4]; 4],
}

impl IsometricCamera {
    pub fn new(width: u32, height: u32) -> Self {
        // 45° rotation + orthographic projection
        let angle = std::f32::consts::PI / 4.0;
        // Compute view matrix...
    }

    pub fn grid_to_screen(&self, grid_x: u32, grid_y: u32, z: u32) -> (f32, f32) {
        let x = (grid_x as f32 - grid_y as f32) * 32.0 / 2.0;
        let y = (grid_x as f32 + grid_y as f32) * 32.0 / 2.0 - z as f32 * 16.0;
        (x, y)
    }
}
```

---

### Phase D: 3D (フル3D・将来)

```
      🧙 ← フル3D視点
     /
    ░ ← 3D メッシュ
```

**実装:**
- glam ライブラリで 3D 数学
- wgpu で 3D パイプライン
- カメラ制御 (orbital camera)

---

## アーキテクチャ設計

### レンダリングエンジンの層構造

```
Game State (Rust)
    ↓
┌─────────────────────────────────────┐
│ RenderCommand Builder               │  ← プラットフォーム非依存
│ (grid → render commands)             │
└─────────────────────────────────────┘
    ↓
┌─────────────────────────────────────┐
│ Renderer trait                      │
│ ├─ AsciiRenderer                    │
│ ├─ TiledRenderer (2D)               │
│ ├─ IsometricRenderer (2.5D)         │ ← 切り替え可能
│ └─ 3DRenderer                       │
└─────────────────────────────────────┘
    ↓
┌─────────────────────────────────────┐
│ wgpu Backend                         │
│ ├─ RenderPass                        │
│ ├─ Pipeline (shaders)                │
│ └─ Texture Atlas                     │
└─────────────────────────────────────┘
    ↓
Frame Buffer → Screen
```

### コード構成

```rust
// nethack-render/src/

pub mod renderer;      // trait Renderer
pub mod ascii;         // AsciiRenderer
pub mod tiled;         // TiledRenderer
pub mod isometric;     // IsometricRenderer

pub trait Renderer {
    fn render_tile(&mut self, x: u32, y: u32, glyph: Glyph);
    fn render_frame(&mut self) -> Result<()>;
    fn set_viewport(&mut self, width: u32, height: u32);
}

pub enum RenderMode {
    Ascii,
    Tiled,
    Isometric,
}
```

---

## 実装ロードマップ

| フェーズ | 目標 | 所要時間目安 | 依存度 |
|---|---|---|---|
| **A (ASCII)** | 基本的なテキスト盤面 | 1-2週間 | 低 |
| **B (Tiles)** | グラフィカルタイル | 2-4週間 | A に依存 |
| **C (2.5D)** | 等角投影レンダリング | 3-5週間 | B に依存 |
| **D (3D)** | フル3Dレンダリング | 4-6週間 | C に依存 |

---

## フェーズ A (ASCII) の実装例

```rust
// nethack-render/src/ascii.rs

use wgpu::*;

pub struct AsciiRenderer {
    device: Device,
    queue: Queue,
    glyph_texture: Texture,  // フォントアトラス
    pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    grid: [u8; 80 * 25],     // グリッド状態
}

impl AsciiRenderer {
    pub fn new(device: Device, queue: Queue) -> Self {
        // フォント画像をテクスチャアトラスに変換
        let glyph_texture = Self::load_font_atlas(&device, &queue);
        let pipeline = Self::create_pipeline(&device);
        
        Self {
            device,
            queue,
            glyph_texture,
            pipeline,
            vertex_buffer: Self::create_vertex_buffer(&device),
            grid: [b'.'; 80 * 25],
        }
    }

    pub fn set_tile(&mut self, x: usize, y: usize, ch: u8) {
        if x < 80 && y < 25 {
            self.grid[y * 80 + x] = ch;
        }
    }

    pub fn render(&mut self, view: &TextureView) {
        let mut encoder = self.device.create_command_encoder(&Default::default());
        {
            let mut rpass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("ascii_pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLACK),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            rpass.set_pipeline(&self.pipeline);
            rpass.set_bind_group(0, &self.create_bind_group(), &[]);
            rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            rpass.draw(0..self.grid.len() as u32, 0..1);
        }
        self.queue.submit(std::iter::once(encoder.finish()));
    }

    fn load_font_atlas(device: &Device, queue: &Queue) -> Texture {
        // TODO: フォントBitmap → Texture
        // または .png フォントアトラスをロード
        todo!()
    }

    fn create_pipeline(device: &Device) -> RenderPipeline {
        // Shader (WGSL)
        let shader_src = include_str!("../shaders/ascii.wgsl");
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("ascii_shader"),
            source: ShaderSource::Wgsl(shader_src.into()),
        });
        
        device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("ascii_pipeline"),
            layout: None,
            push_constant_ranges: &[],
            vertex: VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[/* vertex layout */],
            },
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(ColorTargetState {
                    format: TextureFormat::Bgra8UnormSrgb,
                    blend: None,
                    write_mask: ColorWrites::ALL,
                })],
            }),
        })
    }

    fn create_vertex_buffer(device: &Device) -> Buffer {
        device.create_buffer(&BufferDescriptor {
            label: Some("vertex_buffer"),
            size: (80 * 25 * 6) as u64, // 2 triangles per cell
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    fn create_bind_group(&self) -> BindGroup {
        // フォントテクスチャをシェーダにバインド
        todo!()
    }
}
```

**対応するシェーダ (ascii.wgsl):**
```wgsl
@vertex
fn vs_main(@location(0) pos: vec2f) -> @builtin(position) vec4f {
    return vec4f(pos, 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4f {
    return vec4f(1.0, 1.0, 1.0, 1.0);
}
```

---

## フェーズ C (2.5D Isometric) への移行

同じ `grid` データ構造を維持したまま、投影行列を変更するだけ：

```rust
pub struct IsometricRenderer {
    // AsciiRenderer と同じ構造
    device: Device,
    queue: Queue,
    tile_texture: Texture,  // 32×32 or 64×64 tiles
    pipeline: RenderPipeline,
    grid: [u8; 80 * 25],
    
    // 追加：等角投影用
    camera: IsometricCamera,
}

impl IsometricRenderer {
    pub fn set_camera(&mut self, camera: IsometricCamera) {
        self.camera = camera;
        // Uniform buffer を更新
    }

    pub fn render(&mut self, view: &TextureView) {
        // 同じロジック、ただしシェーダで投影座標を計算
    }
}
```

---

## まとめ

| 項目 | 戦略 |
|---|---|
| **段階化** | ASCII → Tiles → 2.5D → 3D |
| **互換性** | すべてのフェーズで同じ `grid` データを使用 |
| **レンダラ切り替え** | enum `RenderMode` で実行時切り替え可能 |
| **WASM対応** | wgpu がすべてのフェーズに対応 |
| **パフォーマンス** | ASCII < Tiles < 2.5D < 3D（段階的に複雑化） |

**あなたのロードマップ：**
1. ✅ Phase 0: Cargo workspace（完了）
2. 👉 Phase 1-2: FFI + Rust core
3. → **Phase A: ASCII レンダラ実装**
4. → **Phase B: Tiled レンダラ実装**
5. → **Phase C: 2.5D Isometric レンダラ実装**
6. → Phase 4-5: WASM + Unity 統合

2.5D Isometric は Classic RPG (Diablo, Baldur's Gate) の定番スタイルで、
NetHackの複雑な地下迷宮を直感的に表現できます。

いかがですか？
