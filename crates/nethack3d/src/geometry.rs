// ジオメトリ: ダンジョンタイル3D生成

use bytemuck::{Pod, Zeroable};

/// 頂点 (stride=32: pos[3] + pad + col[4])
#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub _p:  f32,
    pub col: [f32; 4],
}
pub const STRIDE: wgpu::BufferAddress = 32;

/// ポイントライト
#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Light {
    pub pos: [f32; 4], // w=フリッカーフェーズ
    pub col: [f32; 4], // a=強度
}

/// Uniform バッファ (224 bytes = 14×16bytes)
#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Uni {
    pub vp:      [[f32; 4]; 4],
    pub time:    f32,
    pub warp:    f32,
    pub pad:     [f32; 2],
    pub lights:  [Light; 4],
    pub fog_col: [f32; 4],
}

/// タイル種別 (JS→WASM転送時の1バイト)
pub const TILE_EMPTY:    u8 = 0;
pub const TILE_FLOOR:    u8 = 1;
pub const TILE_WALL:     u8 = 2;
pub const TILE_CORRIDOR: u8 = 3;
pub const TILE_DOOR:     u8 = 4;
pub const TILE_PLAYER:   u8 = 5;
pub const TILE_STAIRS_U: u8 = 6;
pub const TILE_STAIRS_D: u8 = 7;
pub const TILE_MONSTER:  u8 = 8;
pub const TILE_ITEM:     u8 = 9;

/// 1×1×1 ボックスの1面 (quad) を4頂点+6インデックスで追加
pub fn push_quad(
    verts: &mut Vec<Vertex>, idxs: &mut Vec<u32>,
    p0:[f32;3], p1:[f32;3], p2:[f32;3], p3:[f32;3],
    col: [f32;4],
) {
    let base = verts.len() as u32;
    for p in [p0,p1,p2,p3] {
        verts.push(Vertex { pos: p, _p: 0.0, col });
    }
    idxs.extend_from_slice(&[base,base+1,base+2, base,base+2,base+3]);
}

/// タイルが「通行可能」かどうか (床・廊下・プレイヤーなど)
pub fn is_passable(t: u8) -> bool {
    matches!(t, TILE_FLOOR|TILE_CORRIDOR|TILE_PLAYER|TILE_STAIRS_U|TILE_STAIRS_D|TILE_MONSTER|TILE_ITEM)
}
pub fn is_solid(t: u8) -> bool {
    t == TILE_WALL || t == TILE_EMPTY
}

/// タイルを安全に取得
fn get_tile(tiles: &[u8], w: usize, h: usize, x: i32, y: i32) -> u8 {
    if x < 0 || y < 0 || x as usize >= w || y as usize >= h { return TILE_EMPTY; }
    tiles[y as usize * w + x as usize]
}

/// ダンジョン全体のジオメトリを生成
///
/// tiles: 1D配列 (row-major), w/h: マップ幅・高さ
/// time:  時刻 (アニメーション)
/// px/pz: プレイヤー座標 (ライト用)
pub fn build_dungeon(
    tiles: &[u8], w: usize, h: usize,
    time: f32, px: f32, pz: f32,
    verts: &mut Vec<Vertex>, idxs: &mut Vec<u32>,
    lights: &mut Vec<Light>,
) {
    let floor_col  = [0.18, 0.15, 0.12, 1.0_f32]; // 暗い石床
    let ceil_col   = [0.10, 0.10, 0.10, 1.0_f32]; // 天井 (暗め)
    let wall_col   = [0.30, 0.28, 0.24, 2.0_f32]; // 石壁 (a=2 → グレイン)
    let door_col   = [0.45, 0.28, 0.10, 1.0_f32]; // 木のドア
    let corr_col   = [0.15, 0.13, 0.11, 1.0_f32]; // 廊下
    let stair_u    = [0.20, 0.35, 0.20, 1.0_f32]; // 上り階段
    let stair_d    = [0.35, 0.20, 0.20, 1.0_f32]; // 下り階段
    let item_col   = [0.80, 0.75, 0.20, 3.0_f32]; // アイテム (発光)
    let monster_col= [0.80, 0.20, 0.20, 3.0_f32]; // モンスター (発光赤)

    let tile_w = 1.0_f32;
    let wall_h = 1.8_f32;

    // プレイヤーライト (温かいオレンジ)
    lights.push(Light {
        pos: [px, 1.0, pz, 0.0],
        col: [1.0, 0.70, 0.35, 3.5],
    });
    // アンビエント冷却ライト (薄青)
    lights.push(Light {
        pos: [px, 6.0, pz, 2.5],
        col: [0.3, 0.4, 0.8, 0.6],
    });

    let mut torch_count = 0usize;

    for ty in 0..h {
        for tx in 0..w {
            let t = get_tile(tiles, w, h, tx as i32, ty as i32);
            if t == TILE_EMPTY { continue; }

            let x0 = tx as f32 * tile_w;
            let x1 = x0 + tile_w;
            let z0 = ty as f32 * tile_w;
            let z1 = z0 + tile_w;

            let fc = match t {
                TILE_FLOOR   => floor_col,
                TILE_CORRIDOR => corr_col,
                TILE_DOOR    => door_col,
                TILE_STAIRS_U => stair_u,
                TILE_STAIRS_D => stair_d,
                TILE_PLAYER | TILE_MONSTER | TILE_ITEM => floor_col,
                _ => floor_col,
            };

            // 床
            push_quad(verts, idxs,
                [x0,0.0,z0],[x1,0.0,z0],[x1,0.0,z1],[x0,0.0,z1], fc);
            // 天井
            push_quad(verts, idxs,
                [x0,wall_h,z1],[x1,wall_h,z1],[x1,wall_h,z0],[x0,wall_h,z0], ceil_col);

            // 壁: passable セルから隣の solid セルへの面を追加
            if is_passable(t) {
                // North (-z direction)
                if is_solid(get_tile(tiles, w, h, tx as i32, ty as i32 - 1)) {
                    push_quad(verts, idxs,
                        [x1,wall_h,z0],[x0,wall_h,z0],[x0,0.0,z0],[x1,0.0,z0], wall_col);
                }
                // South (+z)
                if is_solid(get_tile(tiles, w, h, tx as i32, ty as i32 + 1)) {
                    push_quad(verts, idxs,
                        [x0,wall_h,z1],[x1,wall_h,z1],[x1,0.0,z1],[x0,0.0,z1], wall_col);
                }
                // West (-x)
                if is_solid(get_tile(tiles, w, h, tx as i32 - 1, ty as i32)) {
                    push_quad(verts, idxs,
                        [x0,wall_h,z1],[x0,wall_h,z0],[x0,0.0,z0],[x0,0.0,z1], wall_col);
                }
                // East (+x)
                if is_solid(get_tile(tiles, w, h, tx as i32 + 1, ty as i32)) {
                    push_quad(verts, idxs,
                        [x1,wall_h,z0],[x1,wall_h,z1],[x1,0.0,z1],[x1,0.0,z0], wall_col);
                }
            }

            // アイテム: 小さい発光クワッド
            if t == TILE_ITEM {
                let cx = (x0+x1)*0.5; let cz = (z0+z1)*0.5;
                let s = 0.18;
                push_quad(verts, idxs,
                    [cx-s,0.18,cz-s],[cx+s,0.18,cz-s],[cx+s,0.18,cz+s],[cx-s,0.18,cz+s],
                    item_col);
            }

            // モンスター: 赤い発光柱
            if t == TILE_MONSTER {
                let cx = (x0+x1)*0.5; let cz = (z0+z1)*0.5;
                let s = 0.20; let top = 0.9;
                // 南面
                push_quad(verts, idxs,
                    [cx-s,top,cz+s],[cx+s,top,cz+s],[cx+s,0.0,cz+s],[cx-s,0.0,cz+s], monster_col);
                push_quad(verts, idxs,
                    [cx+s,top,cz-s],[cx-s,top,cz-s],[cx-s,0.0,cz-s],[cx+s,0.0,cz-s], monster_col);
                push_quad(verts, idxs,
                    [cx-s,top,cz-s],[cx-s,top,cz+s],[cx-s,0.0,cz+s],[cx-s,0.0,cz-s], monster_col);
                push_quad(verts, idxs,
                    [cx+s,top,cz+s],[cx+s,top,cz-s],[cx+s,0.0,cz-s],[cx+s,0.0,cz+s], monster_col);
                if lights.len() < 4 {
                    lights.push(Light {
                        pos: [cx, 0.5, cz, (tx+ty) as f32 * 0.37],
                        col: [1.0, 0.1, 0.1, 1.5],
                    });
                }
            }

            // 部屋コーナー付近にたまにトーチライト
            if t == TILE_FLOOR && tx % 8 == 0 && ty % 6 == 0 && torch_count < 2 {
                torch_count += 1;
                let fade = ((time * 2.3 + tx as f32 * 0.7).sin() * 0.15 + 1.0).max(0.4);
                if lights.len() < 4 {
                    lights.push(Light {
                        pos: [x0 + 0.1, wall_h - 0.2, z0 + 0.1, tx as f32],
                        col: [1.0, 0.6, 0.2, 2.0 * fade],
                    });
                }
            }
        }
    }
}

/// 軸整列ボックス (6面すべて描画) — どの角度から見ても立体的
fn push_box(
    verts: &mut Vec<Vertex>, idxs: &mut Vec<u32>,
    x0: f32, y0: f32, z0: f32,
    x1: f32, y1: f32, z1: f32,
    col: [f32; 4],
) {
    // 上面
    push_quad(verts, idxs, [x0,y1,z0],[x1,y1,z0],[x1,y1,z1],[x0,y1,z1], col);
    // 下面
    push_quad(verts, idxs, [x0,y0,z1],[x1,y0,z1],[x1,y0,z0],[x0,y0,z0], col);
    // 南面 (+z)
    push_quad(verts, idxs, [x0,y1,z1],[x1,y1,z1],[x1,y0,z1],[x0,y0,z1], col);
    // 北面 (-z)
    push_quad(verts, idxs, [x1,y1,z0],[x0,y1,z0],[x0,y0,z0],[x1,y0,z0], col);
    // 西面 (-x)
    push_quad(verts, idxs, [x0,y1,z1],[x0,y1,z0],[x0,y0,z0],[x0,y0,z1], col);
    // 東面 (+x)
    push_quad(verts, idxs, [x1,y1,z0],[x1,y1,z1],[x1,y0,z1],[x1,y0,z0], col);
}

/// プレイヤーキャラクター (ボックス人型 — どの角度でも立体的)
pub fn build_player(
    verts: &mut Vec<Vertex>, idxs: &mut Vec<u32>,
    px: f32, pz: f32, time: f32,
) {
    let bob = (time * 3.2).sin() * 0.025;

    // 胴体ボックス
    let col_body = [0.50, 0.70, 1.00, 3.0_f32]; // 青白発光
    push_box(verts, idxs,
        px - 0.14, 0.0,        pz - 0.10,
        px + 0.14, 0.46 + bob, pz + 0.10,
        col_body);

    // 頭部ボックス
    let col_head = [1.00, 0.88, 0.72, 3.0_f32]; // 肌色発光
    let head_y0 = 0.50 + bob;
    let head_y1 = 0.74 + bob;
    push_box(verts, idxs,
        px - 0.11, head_y0, pz - 0.11,
        px + 0.11, head_y1, pz + 0.11,
        col_head);

    // 足元の光輪 (小さい発光床クワッド)
    push_quad(verts, idxs,
        [px-0.18, 0.005, pz-0.18],
        [px+0.18, 0.005, pz-0.18],
        [px+0.18, 0.005, pz+0.18],
        [px-0.18, 0.005, pz+0.18],
        [0.4, 0.55, 1.0, 3.0]);
}
