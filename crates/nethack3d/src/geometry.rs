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
pub const TILE_DOG:      u8 = 10;
pub const TILE_CAT:      u8 = 11;
pub const TILE_RAT:      u8 = 12;
pub const TILE_BAT:      u8 = 13;
pub const TILE_SLIME:    u8 = 14;
pub const TILE_ORC:      u8 = 15;
pub const TILE_ZOMBIE:   u8 = 16;
pub const TILE_SNAKE:    u8 = 17;
pub const TILE_EYE:      u8 = 18;
pub const TILE_UNICORN:  u8 = 19;
pub const TILE_TROLL:      u8 = 20;
pub const TILE_DRAGON:     u8 = 21;
pub const TILE_LEPRECHAUN: u8 = 22;
pub const TILE_KOBOLD:     u8 = 23;
pub const TILE_NYMPH:      u8 = 24;
pub const TILE_VAMPIRE:    u8 = 25;
pub const TILE_LICH:       u8 = 26;
pub const TILE_YETI:       u8 = 27;
pub const TILE_ANGEL:      u8 = 28;
pub const TILE_CENTAUR:    u8 = 29;
pub const TILE_GIANT:      u8 = 30;
pub const TILE_WRAITH:     u8 = 31;
pub const TILE_DEMON:      u8 = 32;
pub const TILE_MUMMY:      u8 = 33;
pub const TILE_VAMP_BAT:   u8 = 34;
pub const TILE_GNOME:      u8 = 35;
pub const TILE_STALKER:    u8 = 36;
pub const TILE_XORN:       u8 = 37;

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
    matches!(t & 0x3F, TILE_FLOOR|TILE_CORRIDOR|TILE_PLAYER|TILE_STAIRS_U|TILE_STAIRS_D|TILE_MONSTER|TILE_ITEM|TILE_DOG|TILE_CAT|TILE_RAT|TILE_BAT|TILE_SLIME|TILE_ORC|TILE_ZOMBIE|TILE_SNAKE|TILE_EYE|TILE_UNICORN|TILE_TROLL|TILE_DRAGON|TILE_LEPRECHAUN|TILE_KOBOLD|TILE_NYMPH|TILE_VAMPIRE|TILE_LICH|TILE_YETI|TILE_ANGEL|TILE_CENTAUR|TILE_GIANT|TILE_WRAITH|TILE_DEMON|TILE_MUMMY|TILE_VAMP_BAT|TILE_GNOME|TILE_STALKER|TILE_XORN)
}
pub fn is_solid(t: u8) -> bool {
    let b = t & 0x3F;
    b == TILE_WALL || b == TILE_EMPTY
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
            let base_t = t & 0x3F;
            let tier   = (t >> 6) & 0x3;
            // tierに応じたスケール乗数: 0=小型 1=通常 2=大型 3=ボス
            let tier_base: f32 = match tier { 0=>0.65, 1=>1.00, 2=>1.35, _=>2.00 };
            if base_t == TILE_EMPTY { continue; }

            let x0 = tx as f32 * tile_w;
            let x1 = x0 + tile_w;
            let z0 = ty as f32 * tile_w;
            let z1 = z0 + tile_w;

            let fc = match base_t {
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
            // 天井は描画しない (TOP視点で真っ黒になるため)

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

            // アイテム: 浮かぶ菱形ビルボード (2枚クロス) — 見る角度によらず光る
            if base_t == TILE_ITEM {
                let cx = (x0+x1)*0.5; let cz = (z0+z1)*0.5;
                // タイル位置でボブのタイミングをずらす
                let phase = cx * 0.8 + cz * 1.1;
                let fy = 0.38 + (time * 1.6 + phase).sin() * 0.07;
                let s = 0.20;
                // XZ平面の菱形 (水平回転ディスク感)
                push_quad(verts, idxs,
                    [cx, fy, cz-s],[cx+s, fy, cz],[cx, fy, cz+s],[cx-s, fy, cz],
                    item_col);
                // 縦菱形 X方向
                push_quad(verts, idxs,
                    [cx,   fy+s*0.8, cz],
                    [cx+s, fy,       cz],
                    [cx,   fy-s*0.8, cz],
                    [cx-s, fy,       cz],
                    item_col);
                // 縦菱形 Z方向
                push_quad(verts, idxs,
                    [cx, fy+s*0.8, cz  ],
                    [cx, fy,       cz+s],
                    [cx, fy-s*0.8, cz  ],
                    [cx, fy,       cz-s],
                    item_col);
            }

            // モンスター: クロスビルボード (Doomスタイル — どの角度でも見える)
            if base_t == TILE_MONSTER {
                let cx = (x0+x1)*0.5; let cz = (z0+z1)*0.5;
                let phase = cx * 0.9 + cz * 0.7;
                // 位置ごとに微妙にボブタイミングをずらして生き生き感
                let bob = (time * 2.6 + phase).sin() * 0.025;
                let s   = 0.24 * tier_base;
                let top = 0.90 * tier_base + bob;
                let bot = 0.04;
                // 赤みを位置ハッシュで少し変化させる
                let hue_r = 0.75 + ((tx*3 + ty*7) % 5) as f32 * 0.05;
                let hue_g = 0.10 + ((tx+ty*2) % 4) as f32 * 0.04;
                let mc = [hue_r, hue_g, 0.10, 3.0_f32];
                // X方向スラブ (Z軸に平行な板)
                push_quad(verts, idxs,
                    [cx-s, top, cz],[cx+s, top, cz],
                    [cx+s, bot, cz],[cx-s, bot, cz], mc);
                // Z方向スラブ (X軸に平行な板)
                push_quad(verts, idxs,
                    [cx, top, cz-s],[cx, top, cz+s],
                    [cx, bot, cz+s],[cx, bot, cz-s], mc);
                // 目 (上部に小さい黄色発光点)
                let ey = top - 0.12;
                let es = 0.05;
                push_quad(verts, idxs,
                    [cx-es, ey, cz-0.01],[cx+es, ey, cz-0.01],
                    [cx+es, ey-es*0.6, cz-0.01],[cx-es, ey-es*0.6, cz-0.01],
                    [1.0, 1.0, 0.3, 3.0]);
                if lights.len() < 4 {
                    lights.push(Light {
                        pos: [cx, 0.6, cz, phase],
                        col: [1.0, 0.15, 0.10, 1.8],
                    });
                }
            }

            // 犬 (TILE_DOG): ボックスを積み重ねた本格3Dモデル
            // 胴体・頭・鼻・4本脚・耳2枚・しっぽ で構成
            if base_t == TILE_DOG {
                let cx = (x0+x1)*0.5; let cz = (z0+z1)*0.5;
                let hash = (tx * 13 + ty * 7) as f32;

                // ── サイズバリエーション (位置ハッシュ×tierで決定) ──
                let size_var = (tx * 5 + ty * 3) % 3; // 0=小型, 1=中型, 2=大型
                let sc = tier_base * match size_var {
                    0 => 0.60_f32, // 小さな子犬
                    1 => 0.85_f32, // 普通の犬
                    _ => 1.10_f32, // 大きな犬
                };

                // ── カラーバリエーション (毛色5種) ──
                let fur = match (tx * 7 + ty * 11) % 5 {
                    0 => [0.82, 0.62, 0.28, 1.0_f32], // 茶色
                    1 => [0.92, 0.88, 0.75, 1.0_f32], // クリーム/白
                    2 => [0.22, 0.18, 0.12, 1.0_f32], // 黒
                    3 => [0.65, 0.38, 0.18, 1.0_f32], // 濃い茶
                    _ => [0.75, 0.72, 0.65, 1.0_f32], // グレー
                };
                // 腹側はすこし明るく
                let belly = [fur[0]*1.25, fur[1]*1.25, fur[2]*1.25, fur[3]];
                // 鼻・口は黒っぽく
                let nose_col  = [0.15, 0.10, 0.08, 1.0_f32];
                // 目は黒 + 光点 (emissive)
                let eye_col   = [0.08, 0.06, 0.04, 3.0_f32];
                let eye_shine = [1.0,  1.0,  0.9,  3.0_f32];

                // ── アニメーション ──
                // 全体を少しバウンドさせる (4足なので細かく)
                let walk = (time * 3.8 + hash * 0.4).sin();
                let base_y = (time * 7.5 + hash * 0.4).sin().abs() * 0.018 * sc; // 4足歩行リズム
                // しっぽを左右に振る
                let wag = (time * 6.0 + hash * 0.8).sin();

                // スケール済み寸法
                let bw = 0.18 * sc; // 胴体 X 半幅
                let bd = 0.10 * sc; // 胴体 Z 半幅
                let by0 = 0.16 * sc + base_y; // 胴体底
                let by1 = 0.30 * sc + base_y; // 胴体天
                // 胴体を前後に伸ばす
                let bfr = 0.22 * sc; // 前方 (+Z)
                let bbk = 0.22 * sc; // 後方 (-Z)

                // ── 胴体 ──
                push_box(verts, idxs,
                    cx-bw, by0, cz-bbk,
                    cx+bw, by1, cz+bfr, fur);

                // ── 頭 (前方やや上) ──
                let hx = 0.13 * sc; // 頭 X 半幅
                let hz = 0.12 * sc; // 頭 Z 半幅 (奥行き)
                let hy0 = by1 - 0.04 * sc;
                let hy1 = hy0 + 0.22 * sc;
                let hcz = cz + bfr + hz * 0.6; // 頭の中心 Z
                push_box(verts, idxs,
                    cx-hx, hy0, hcz-hz,
                    cx+hx, hy1, hcz+hz*0.5, fur);

                // ── 鼻 (頭の前面中央) ──
                let nw = 0.055 * sc;
                let nh = 0.045 * sc;
                let nd = 0.04 * sc;
                let ny0 = hy0 + (hy1-hy0)*0.28;
                push_box(verts, idxs,
                    cx-nw, ny0, hcz+hz*0.5,
                    cx+nw, ny0+nh, hcz+hz*0.5+nd, nose_col);

                // ── 耳 (2枚, 頭の上に薄い板) ──
                let ew = 0.055 * sc;
                let eh = 0.09 * sc;
                let ed = 0.025 * sc; // 耳の厚み
                let earz = hcz - hz * 0.1;
                let eary0 = hy1;
                // 耳の色は少し暗め
                let ear_col = [fur[0]*0.8, fur[1]*0.7, fur[2]*0.7, fur[3]];
                // 左耳 (すこし外に傾けるため X をずらす)
                push_box(verts, idxs,
                    cx - hx*0.9 - ew, eary0, earz-ed,
                    cx - hx*0.9,      eary0+eh, earz+ed, ear_col);
                // 右耳
                push_box(verts, idxs,
                    cx + hx*0.9,      eary0, earz-ed,
                    cx + hx*0.9 + ew, eary0+eh, earz+ed, ear_col);

                // ── 目 2個 ──
                let ex_off = hx * 0.52;
                let ey_h   = hy0 + (hy1-hy0)*0.62;
                let ez_f   = hcz + hz * 0.45;
                let esz    = 0.03 * sc; // 目のサイズ
                // 左目
                push_box(verts, idxs,
                    cx-ex_off-esz, ey_h-esz, ez_f,
                    cx-ex_off+esz, ey_h+esz, ez_f+esz*0.6, eye_col);
                push_box(verts, idxs, // ハイライト
                    cx-ex_off, ey_h+esz*0.3, ez_f+esz*0.55,
                    cx-ex_off+esz*0.5, ey_h+esz*0.8, ez_f+esz*0.7, eye_shine);
                // 右目
                push_box(verts, idxs,
                    cx+ex_off-esz, ey_h-esz, ez_f,
                    cx+ex_off+esz, ey_h+esz, ez_f+esz*0.6, eye_col);
                push_box(verts, idxs,
                    cx+ex_off, ey_h+esz*0.3, ez_f+esz*0.55,
                    cx+ex_off+esz*0.5, ey_h+esz*0.8, ez_f+esz*0.7, eye_shine);

                // ── 4本脚 ──
                let lw = 0.055 * sc; // 脚の半幅
                let lh = by0;        // 脚の高さ (地面〜胴体底)
                let leg_col = [fur[0]*0.88, fur[1]*0.88, fur[2]*0.88, fur[3]];
                // 前脚は歩行アニメ (左右逆位相)
                let leg_front_y = lh * (1.0 + walk * 0.06);
                let leg_back_y  = lh * (1.0 - walk * 0.06);
                // 前左脚
                push_box(verts, idxs,
                    cx-bw+lw*0.3, 0.0, cz+bfr-lw*2.0-lw*0.5,
                    cx-bw+lw*0.3+lw*2.0, leg_front_y, cz+bfr-lw*0.5, leg_col);
                // 前右脚
                push_box(verts, idxs,
                    cx+bw-lw*0.3-lw*2.0, 0.0, cz+bfr-lw*2.0-lw*0.5,
                    cx+bw-lw*0.3, leg_back_y, cz+bfr-lw*0.5, leg_col);
                // 後左脚
                push_box(verts, idxs,
                    cx-bw+lw*0.3, 0.0, cz-bbk+lw*0.5,
                    cx-bw+lw*0.3+lw*2.0, leg_back_y, cz-bbk+lw*0.5+lw*2.0, leg_col);
                // 後右脚
                push_box(verts, idxs,
                    cx+bw-lw*0.3-lw*2.0, 0.0, cz-bbk+lw*0.5,
                    cx+bw-lw*0.3, leg_front_y, cz-bbk+lw*0.5+lw*2.0, leg_col);

                // ── しっぽ (後方, 左右に振る) ──
                let tail_x = cx + wag * 0.13 * sc;
                let tail_y0 = by1 * 0.7;
                let tail_y1 = by1 * 0.7 + 0.22 * sc;
                let tail_z0 = cz - bbk - 0.14 * sc;
                let tail_z1 = cz - bbk - 0.02 * sc;
                let tail_col = [fur[0]*0.92, fur[1]*0.78, fur[2]*0.65, fur[3]];
                push_box(verts, idxs,
                    tail_x - 0.03*sc, tail_y0, tail_z0,
                    tail_x + 0.03*sc, tail_y1, tail_z1, tail_col);

                // ── フレンドリーな暖色ライト ──
                if lights.len() < 4 {
                    lights.push(Light {
                        pos: [cx, by1, cz, hash],
                        col: [1.0, 0.80, 0.40, 1.2 * sc],
                    });
                }
            }

            // 猫 (TILE_CAT): 犬より可愛く! 大きな目・尖り耳・ひげ・上向きしっぽ
            if base_t == TILE_CAT {
                let cx = (x0+x1)*0.5; let cz = (z0+z1)*0.5;
                let hash = (tx * 17 + ty * 11) as f32;

                // ── サイズ (猫は全体的にスリム、tier倍率適用) ──
                let sc = tier_base * match (tx * 5 + ty * 3) % 3 {
                    0 => 0.55_f32, // 子猫
                    1 => 0.75_f32, // 普通
                    _ => 0.90_f32, // 大きめ
                };

                // ── 毛色5種 ──
                let fur: [f32;4] = match (tx * 11 + ty * 7) % 5 {
                    0 => [0.92, 0.50, 0.14, 1.0], // オレンジタビー
                    1 => [0.10, 0.08, 0.09, 1.0], // 黒猫
                    2 => [0.96, 0.94, 0.91, 1.0], // 白猫
                    3 => [0.56, 0.53, 0.48, 1.0], // グレー縞
                    _ => [0.72, 0.40, 0.14, 1.0], // 茶トラ
                };
                let belly = [(fur[0]*1.3).min(1.0), (fur[1]*1.3).min(1.0), (fur[2]*1.3).min(1.0), 1.0];
                let ear_in  = [0.92_f32, 0.55, 0.58, 1.0]; // 耳内側ピンク
                let ear_out = [fur[0]*0.82, fur[1]*0.72, fur[2]*0.72, 1.0];
                let pink    = [0.95_f32, 0.65, 0.68, 1.0]; // 鼻・肉球ピンク
                let wh_col  = [0.96_f32, 0.96, 0.96, 1.0]; // ひげ
                // 目: 色付き虹彩・黒瞳・強ハイライト
                let eye_iris: [f32;4] = match (tx * 3 + ty * 5) % 3 {
                    0 => [0.18, 0.72, 0.28, 3.0], // 翠の目
                    1 => [0.82, 0.58, 0.08, 3.0], // 金の目
                    _ => [0.12, 0.42, 0.78, 3.0], // 青い目
                };
                let eye_col   = [0.06_f32, 0.04, 0.07, 3.0];
                let eye_shine = [1.0_f32,  0.98, 0.95, 4.0]; // 強エミッシブ

                // ── アニメーション ──
                let walk      = (time * 4.0 + hash * 0.4).sin();
                let base_y    = (time * 8.0 + hash * 0.5).sin().abs() * 0.012 * sc;
                let tail_sway = (time * 2.5 + hash * 0.6).sin();

                // ── 胴体 (スリム) ──
                let bw = 0.13 * sc;
                let by0 = 0.13 * sc + base_y;
                let by1 = 0.25 * sc + base_y;
                let bfr = 0.19 * sc;
                let bbk = 0.19 * sc;
                push_box(verts, idxs, cx-bw, by0, cz-bbk, cx+bw, by1, cz+bfr, fur);
                // お腹ハイライト
                push_box(verts, idxs,
                    cx-bw*0.75, by0, cz-bbk*0.6,
                    cx+bw*0.75, by0+0.008, cz+bfr*0.6, belly);

                // ── 頭 (丸く大きめ) ──
                let hx = 0.125 * sc;
                let hz = 0.105 * sc;
                let hy0 = by1 - 0.018 * sc;
                let hy1 = hy0 + 0.195 * sc;
                let hcz = cz + bfr + hz * 0.45;
                push_box(verts, idxs, cx-hx, hy0, hcz-hz, cx+hx, hy1, hcz+hz*0.45, fur);

                // ── 耳 (尖った縦長ボックス + ピンク内側) ──
                let ew = 0.042 * sc;
                let eh = 0.115 * sc; // 犬より高く尖る
                let ez  = hcz - hz * 0.5;
                let ey0 = hy1 - 0.01 * sc;
                // 左耳
                push_box(verts, idxs,
                    cx - hx*0.72 - ew, ey0, ez - ew,
                    cx - hx*0.72,      ey0 + eh, ez + ew, ear_out);
                push_box(verts, idxs, // 内側ピンク
                    cx - hx*0.72 - ew*0.55, ey0 + eh*0.15, ez - ew*0.25,
                    cx - hx*0.72 - ew*0.05, ey0 + eh*0.82, ez + ew*0.25, ear_in);
                // 右耳
                push_box(verts, idxs,
                    cx + hx*0.72,      ey0, ez - ew,
                    cx + hx*0.72 + ew, ey0 + eh, ez + ew, ear_out);
                push_box(verts, idxs,
                    cx + hx*0.72 + ew*0.05, ey0 + eh*0.15, ez - ew*0.25,
                    cx + hx*0.72 + ew*0.55, ey0 + eh*0.82, ez + ew*0.25, ear_in);

                // ── 鼻 (小さいピンク) ──
                let nw = 0.028 * sc;
                let nh = 0.022 * sc;
                let ny0 = hy0 + (hy1-hy0)*0.30;
                push_box(verts, idxs,
                    cx-nw, ny0, hcz+hz*0.38,
                    cx+nw, ny0+nh, hcz+hz*0.38+0.028*sc, pink);

                // ── ひげ (水平に伸びる細いボックス、左右各2本) ──
                let wy  = ny0 + nh*0.4;
                let wtk = 0.005 * sc; // 太さ
                let wlen = 0.13 * sc;
                // 左2本
                push_box(verts, idxs,
                    cx - hx - wlen, wy + nh*0.5, hcz+hz*0.25,
                    cx - hx*0.05,   wy + nh*0.5 + wtk, hcz+hz*0.25+wtk, wh_col);
                push_box(verts, idxs,
                    cx - hx - wlen, wy - nh*0.1, hcz+hz*0.25,
                    cx - hx*0.05,   wy - nh*0.1 + wtk, hcz+hz*0.25+wtk, wh_col);
                // 右2本
                push_box(verts, idxs,
                    cx + hx*0.05, wy + nh*0.5, hcz+hz*0.25,
                    cx + hx + wlen, wy + nh*0.5 + wtk, hcz+hz*0.25+wtk, wh_col);
                push_box(verts, idxs,
                    cx + hx*0.05, wy - nh*0.1, hcz+hz*0.25,
                    cx + hx + wlen, wy - nh*0.1 + wtk, hcz+hz*0.25+wtk, wh_col);

                // ── 目 (猫の命! 大きく・虹彩・瞳孔・ハイライト2点) ──
                let ex_off = hx * 0.45;
                let eyz    = hy0 + (hy1-hy0)*0.57;
                let ezf    = hcz + hz * 0.38;
                let es     = 0.044 * sc; // 犬より大きい!
                // 左目: 虹彩→縦瞳孔→ハイライト×2
                push_box(verts, idxs,
                    cx-ex_off-es, eyz-es*0.92, ezf,
                    cx-ex_off+es, eyz+es*0.92, ezf+es*0.45, eye_iris);
                push_box(verts, idxs, // 縦長瞳孔
                    cx-ex_off-es*0.28, eyz-es*0.88, ezf+0.001,
                    cx-ex_off+es*0.28, eyz+es*0.88, ezf+es*0.5, eye_col);
                push_box(verts, idxs, // メインハイライト
                    cx-ex_off+es*0.12, eyz+es*0.22, ezf+es*0.42,
                    cx-ex_off+es*0.72, eyz+es*0.80, ezf+es*0.58, eye_shine);
                push_box(verts, idxs, // サブハイライト
                    cx-ex_off-es*0.52, eyz-es*0.08, ezf+es*0.42,
                    cx-ex_off-es*0.10, eyz+es*0.30, ezf+es*0.58, eye_shine);
                // 右目
                push_box(verts, idxs,
                    cx+ex_off-es, eyz-es*0.92, ezf,
                    cx+ex_off+es, eyz+es*0.92, ezf+es*0.45, eye_iris);
                push_box(verts, idxs,
                    cx+ex_off-es*0.28, eyz-es*0.88, ezf+0.001,
                    cx+ex_off+es*0.28, eyz+es*0.88, ezf+es*0.5, eye_col);
                push_box(verts, idxs,
                    cx+ex_off-es*0.72, eyz+es*0.22, ezf+es*0.42,
                    cx+ex_off-es*0.12, eyz+es*0.80, ezf+es*0.58, eye_shine);
                push_box(verts, idxs,
                    cx+ex_off+es*0.10, eyz-es*0.08, ezf+es*0.42,
                    cx+ex_off+es*0.52, eyz+es*0.30, ezf+es*0.58, eye_shine);

                // ── 4本脚 (細くしなやか) ──
                let lw = 0.038 * sc;
                let lh = by0;
                let leg_col = [fur[0]*0.88, fur[1]*0.88, fur[2]*0.88, fur[3]];
                let lfy = lh * (1.0 + walk * 0.08);
                let lby = lh * (1.0 - walk * 0.08);
                push_box(verts, idxs,
                    cx-bw+lw*0.3, 0.0, cz+bfr-lw*2.0,
                    cx-bw+lw*0.3+lw*2.0, lfy, cz+bfr, leg_col);
                push_box(verts, idxs,
                    cx+bw-lw*0.3-lw*2.0, 0.0, cz+bfr-lw*2.0,
                    cx+bw-lw*0.3, lby, cz+bfr, leg_col);
                push_box(verts, idxs,
                    cx-bw+lw*0.3, 0.0, cz-bbk,
                    cx-bw+lw*0.3+lw*2.0, lby, cz-bbk+lw*2.0, leg_col);
                push_box(verts, idxs,
                    cx+bw-lw*0.3-lw*2.0, 0.0, cz-bbk,
                    cx+bw-lw*0.3, lfy, cz-bbk+lw*2.0, leg_col);

                // ── しっぽ (猫は上に立てる! 2セグメント+毛先) ──
                let tw = 0.022 * sc;
                let tail_col = [fur[0]*0.86, fur[1]*0.78, fur[2]*0.73, 1.0];
                let tail_tip = [(fur[0]*1.35).min(1.0), (fur[1]*1.35).min(1.0), (fur[2]*1.35).min(1.0), 1.0];
                let tx1 = cx + tail_sway * 0.04 * sc;
                // 根本 (後ろ下から上へ)
                push_box(verts, idxs,
                    tx1-tw, by0*0.55, cz-bbk-0.04*sc,
                    tx1+tw, by1+0.10*sc, cz-bbk+0.04*sc, tail_col);
                // 中段 (上に伸びて少し前へ傾く)
                let tx2 = tx1 + tail_sway * 0.07 * sc;
                push_box(verts, idxs,
                    tx2-tw*0.85, by1+0.08*sc, cz-bbk-0.09*sc,
                    tx2+tw*0.85, by1+0.30*sc, cz-bbk+0.02*sc, tail_col);
                // 毛先 (ふわっと大きめ)
                let tx3 = tx2 + tail_sway * 0.05 * sc;
                push_box(verts, idxs,
                    tx3-tw*1.4, by1+0.27*sc, cz-bbk-0.11*sc,
                    tx3+tw*1.4, by1+0.40*sc, cz-bbk+0.03*sc, tail_tip);

                // ── 神秘的な青白いライト ──
                if lights.len() < 4 {
                    lights.push(Light {
                        pos: [cx, by1, cz, hash],
                        col: [0.75, 0.85, 1.0, 1.1 * sc],
                    });
                }
            }


            // ━━ ネズミ (TILE_RAT): 丸い体・大きな耳・長いしっぽ ━━
            if base_t == TILE_RAT {
                let cx = (x0+x1)*0.5; let cz = (z0+z1)*0.5;
                let hash = (tx * 13 + ty * 9) as f32;
                let sc = tier_base * match (tx * 5 + ty * 3) % 3 { 0 => 0.50_f32, 1 => 0.65, _ => 0.80 };
                let fur: [f32;4] = match (tx * 7 + ty * 11) % 4 {
                    0 => [0.55, 0.52, 0.48, 1.0], // グレー
                    1 => [0.72, 0.48, 0.28, 1.0], // 茶
                    2 => [0.92, 0.90, 0.86, 1.0], // 白
                    _ => [0.18, 0.15, 0.12, 1.0], // 黒
                };
                let pink      = [0.90_f32, 0.60, 0.62, 1.0];
                let eye_col   = [0.05_f32, 0.04, 0.05, 3.0];
                let eye_shine = [1.0_f32,  0.95, 0.90, 3.0];
                let walk  = (time * 5.0 + hash * 0.4).sin();
                let base_y = (time * 10.0 + hash).sin().abs() * 0.010 * sc;
                let sniff  = (time * 3.5 + hash).sin() * 0.007 * sc;
                let tail_curl = (time * 2.0 + hash).sin();
                // 胴体
                let bw = 0.10 * sc; let by0 = 0.06*sc+base_y; let by1 = 0.17*sc+base_y;
                let bfr = 0.14*sc; let bbk = 0.14*sc;
                push_box(verts, idxs, cx-bw, by0, cz-bbk, cx+bw, by1, cz+bfr, fur);
                // 頭
                let hx = 0.082*sc; let hz = 0.075*sc;
                let hy0 = by1-0.008*sc; let hy1 = hy0+0.135*sc;
                let hcz = cz+bfr+hz*0.45;
                push_box(verts, idxs, cx-hx, hy0+sniff, hcz-hz, cx+hx, hy1+sniff, hcz+hz*0.45, fur);
                // 大きな丸耳 (ネズミの特徴!)
                let ew = 0.055*sc; let eh = 0.068*sc;
                let ear_y0 = hy1+sniff; let ear_z = hcz-hz*0.25;
                let ear_col = [fur[0]*0.80, fur[1]*0.72, fur[2]*0.72, 1.0];
                let ear_in  = [pink[0]*0.95, pink[1]*0.90, pink[2]*0.90, 1.0];
                push_box(verts, idxs, cx-hx*0.55-ew, ear_y0, ear_z-ew, cx-hx*0.55, ear_y0+eh, ear_z+ew, ear_col);
                push_box(verts, idxs, cx-hx*0.55-ew*0.72, ear_y0+eh*0.12, ear_z-ew*0.5, cx-hx*0.55-ew*0.12, ear_y0+eh*0.85, ear_z+ew*0.5, ear_in);
                push_box(verts, idxs, cx+hx*0.55, ear_y0, ear_z-ew, cx+hx*0.55+ew, ear_y0+eh, ear_z+ew, ear_col);
                push_box(verts, idxs, cx+hx*0.55+ew*0.12, ear_y0+eh*0.12, ear_z-ew*0.5, cx+hx*0.55+ew*0.72, ear_y0+eh*0.85, ear_z+ew*0.5, ear_in);
                // 鼻 (ピンク・ひくひく)
                let nz = hcz+hz*0.40;
                push_box(verts, idxs, cx-0.017*sc, hy0+(hy1-hy0)*0.25+sniff, nz, cx+0.017*sc, hy0+(hy1-hy0)*0.42+sniff, nz+0.022*sc, pink);
                // 目 (小さなビーズ目)
                let ex_off = hx*0.46; let eyz = hy0+(hy1-hy0)*0.66+sniff; let ezf = hcz+hz*0.33; let es = 0.022*sc;
                push_box(verts, idxs, cx-ex_off-es, eyz-es, ezf, cx-ex_off+es, eyz+es, ezf+es*0.55, eye_col);
                push_box(verts, idxs, cx-ex_off+es*0.05, eyz+es*0.08, ezf+es*0.45, cx-ex_off+es*0.72, eyz+es*0.82, ezf+es*0.65, eye_shine);
                push_box(verts, idxs, cx+ex_off-es, eyz-es, ezf, cx+ex_off+es, eyz+es, ezf+es*0.55, eye_col);
                push_box(verts, idxs, cx+ex_off-es*0.72, eyz+es*0.08, ezf+es*0.45, cx+ex_off-es*0.05, eyz+es*0.82, ezf+es*0.65, eye_shine);
                // 4本脚
                let lw = 0.027*sc; let lh = by0; let leg_col = [fur[0]*0.85, fur[1]*0.85, fur[2]*0.85, fur[3]];
                let lfy = lh*(1.0+walk*0.10); let lby = lh*(1.0-walk*0.10);
                push_box(verts, idxs, cx-bw+lw*0.2, 0.0, cz+bfr-lw*2.0, cx-bw+lw*0.2+lw*2.0, lfy, cz+bfr, leg_col);
                push_box(verts, idxs, cx+bw-lw*0.2-lw*2.0, 0.0, cz+bfr-lw*2.0, cx+bw-lw*0.2, lby, cz+bfr, leg_col);
                push_box(verts, idxs, cx-bw+lw*0.2, 0.0, cz-bbk, cx-bw+lw*0.2+lw*2.0, lby, cz-bbk+lw*2.0, leg_col);
                push_box(verts, idxs, cx+bw-lw*0.2-lw*2.0, 0.0, cz-bbk, cx+bw-lw*0.2, lfy, cz-bbk+lw*2.0, leg_col);
                // しっぽ (長くて細い、くるんと曲がる)
                let tw = 0.016*sc; let tail_col = [pink[0]*0.78, pink[1]*0.58, pink[2]*0.58, 1.0];
                push_box(verts, idxs, cx-tw, by0*0.45, cz-bbk-0.17*sc, cx+tw, by0*0.78, cz-bbk, tail_col);
                let tc_x = cx + tail_curl*0.06*sc;
                push_box(verts, idxs, tc_x-tw*0.75, by0*0.60, cz-bbk-0.28*sc, tc_x+tw*0.75, by0*0.82+0.04*sc, cz-bbk-0.16*sc, tail_col);
                push_box(verts, idxs, tc_x-tw*0.55, by0*0.70, cz-bbk-0.34*sc, tc_x+tw*0.55, by0*0.75+0.06*sc, cz-bbk-0.27*sc, tail_col);
                if lights.len() < 4 { lights.push(Light { pos:[cx,by1,cz,hash], col:[0.9,0.7,0.5,0.7*sc] }); }
            }

            // ━━ スライム (TILE_SLIME): ぷよぷよ笑顔ブロブ ━━
            if base_t == TILE_SLIME {
                let cx = (x0+x1)*0.5; let cz = (z0+z1)*0.5;
                let hash = (tx * 11 + ty * 13) as f32;
                let sc = tier_base * match (tx * 5 + ty * 3) % 2 { 0 => 0.85_f32, _ => 1.0 };
                let slime_col: [f32;4] = match (tx * 7 + ty * 5) % 5 {
                    0 => [0.28, 0.82, 0.22, 1.0], // 緑
                    1 => [0.22, 0.32, 0.92, 1.0], // 青
                    2 => [0.78, 0.22, 0.88, 1.0], // 紫
                    3 => [0.90, 0.82, 0.12, 1.0], // 黄
                    _ => [0.88, 0.38, 0.12, 1.0], // オレンジ
                };
                let slime_hi  = [(slime_col[0]*1.5).min(1.0), (slime_col[1]*1.5).min(1.0), (slime_col[2]*1.5).min(1.0), 3.0];
                let eye_col   = [0.05_f32, 0.04, 0.06, 3.0];
                let eye_shine = [1.0_f32,  1.0,  0.95, 4.0];
                let mouth_col = [slime_col[0]*0.55, slime_col[1]*0.55, slime_col[2]*0.55, 1.0];
                let wobble = (time * 3.2 + hash * 0.5).sin();
                let pulse  = (time * 2.0 + hash * 0.3).sin() * 0.020 * sc;
                let bw = (0.28 + wobble * 0.030) * sc;
                let bh = (0.20 - wobble.abs() * 0.018) * sc + pulse;
                let bd = (0.22 + wobble * 0.018) * sc;
                // 本体
                push_box(verts, idxs, cx-bw, 0.0, cz-bd, cx+bw, bh, cz+bd, slime_col);
                // 頂部&横テカリ
                push_box(verts, idxs, cx-bw*0.45, bh-0.008, cz-bd*0.45, cx+bw*0.45, bh+0.025*sc, cz+bd*0.45, slime_hi);
                push_box(verts, idxs, cx+bw*0.50, bh*0.52, cz+bd*0.45, cx+bw*0.80, bh*0.72, cz+bd*0.68, slime_hi);
                // 大きな目
                let eyz = bh*0.62; let ezf = cz+bd*0.88; let es = 0.058*sc; let ex_off = bw*0.36;
                push_box(verts, idxs, cx-ex_off-es, eyz-es, ezf, cx-ex_off+es, eyz+es, ezf+es*0.38, slime_col);
                push_box(verts, idxs, cx-ex_off-es*0.82, eyz-es*0.82, ezf+0.002, cx-ex_off+es*0.82, eyz+es*0.82, ezf+es*0.42, eye_col);
                push_box(verts, idxs, cx-ex_off+es*0.08, eyz+es*0.08, ezf+es*0.33, cx-ex_off+es*0.68, eyz+es*0.78, ezf+es*0.48, eye_shine);
                push_box(verts, idxs, cx-ex_off-es*0.60, eyz-es*0.08, ezf+es*0.33, cx-ex_off-es*0.10, eyz+es*0.35, ezf+es*0.48, eye_shine);
                push_box(verts, idxs, cx+ex_off-es, eyz-es, ezf, cx+ex_off+es, eyz+es, ezf+es*0.38, slime_col);
                push_box(verts, idxs, cx+ex_off-es*0.82, eyz-es*0.82, ezf+0.002, cx+ex_off+es*0.82, eyz+es*0.82, ezf+es*0.42, eye_col);
                push_box(verts, idxs, cx+ex_off-es*0.68, eyz+es*0.08, ezf+es*0.33, cx+ex_off-es*0.08, eyz+es*0.78, ezf+es*0.48, eye_shine);
                push_box(verts, idxs, cx+ex_off+es*0.10, eyz-es*0.08, ezf+es*0.33, cx+ex_off+es*0.60, eyz+es*0.35, ezf+es*0.48, eye_shine);
                // 笑顔の口 (横線+両端上がり)
                push_box(verts, idxs, cx-es*0.95, eyz-es*1.12, ezf+0.001, cx+es*0.95, eyz-es*0.65, ezf+es*0.28, mouth_col);
                push_box(verts, idxs, cx-es*1.10, eyz-es*0.80, ezf+0.001, cx-es*0.90, eyz-es*0.40, ezf+es*0.28, mouth_col);
                push_box(verts, idxs, cx+es*0.90, eyz-es*0.80, ezf+0.001, cx+es*1.10, eyz-es*0.40, ezf+es*0.28, mouth_col);
                if lights.len() < 4 { lights.push(Light { pos:[cx,bh*0.6,cz,hash], col:[slime_col[0],slime_col[1],slime_col[2],1.6*sc] }); }
            }

            // ━━ コウモリ (TILE_BAT): 羽ばたく・赤い目 ━━
            if base_t == TILE_BAT {
                let cx = (x0+x1)*0.5; let cz = (z0+z1)*0.5;
                let hash = (tx * 9 + ty * 15) as f32;
                let sc = tier_base * 0.72_f32;
                let bat_col  = [0.12_f32, 0.08, 0.14, 1.0];
                let wing_col = [0.20_f32, 0.10, 0.24, 1.0];
                let fur_col  = [0.22_f32, 0.14, 0.17, 1.0];
                let eye_col  = [0.92_f32, 0.12, 0.05, 3.0]; // 赤い目!
                let eye_shine= [1.0_f32,  0.80, 0.50, 3.0];
                let flap = (time * 8.5 + hash).sin();
                let bob  = (time * 4.2 + hash).sin() * 0.042 * sc;
                let base = 0.34*sc + bob;
                // 胴体
                push_box(verts, idxs, cx-0.078*sc, base, cz-0.085*sc, cx+0.078*sc, base+0.105*sc, cz+0.085*sc, fur_col);
                // 頭
                let hy0 = base+0.078*sc; let hy1 = hy0+0.088*sc; let hcz = cz+0.056*sc;
                push_box(verts, idxs, cx-0.068*sc, hy0, hcz-0.060*sc, cx+0.068*sc, hy1, hcz+0.044*sc, bat_col);
                // 大きな耳 (コウモリらしく縦長)
                let ew = 0.028*sc; let eh = 0.074*sc; let ear_z = hcz-0.018*sc;
                let ear_col = [0.16_f32, 0.08, 0.20, 1.0];
                push_box(verts, idxs, cx-0.050*sc-ew, hy1, ear_z-ew*0.5, cx-0.050*sc, hy1+eh, ear_z+ew*0.5, ear_col);
                push_box(verts, idxs, cx+0.050*sc, hy1, ear_z-ew*0.5, cx+0.050*sc+ew, hy1+eh, ear_z+ew*0.5, ear_col);
                // 赤い目
                let es = 0.019*sc; let eyz = hy0+(hy1-hy0)*0.55; let ezf = hcz+0.040*sc; let ex_off = 0.040*sc;
                push_box(verts, idxs, cx-ex_off-es, eyz-es*0.70, ezf, cx-ex_off+es, eyz+es*0.70, ezf+es*0.58, eye_col);
                push_box(verts, idxs, cx-ex_off+es*0.12, eyz+es*0.02, ezf+es*0.48, cx-ex_off+es*0.60, eyz+es*0.62, ezf+es*0.64, eye_shine);
                push_box(verts, idxs, cx+ex_off-es, eyz-es*0.70, ezf, cx+ex_off+es, eyz+es*0.70, ezf+es*0.58, eye_col);
                push_box(verts, idxs, cx+ex_off-es*0.60, eyz+es*0.02, ezf+es*0.48, cx+ex_off-es*0.12, eyz+es*0.62, ezf+es*0.64, eye_shine);
                // 翼 (羽ばたきアニメ: 3段)
                let wh = 0.024*sc; // 翼の厚み
                let wr_y = base+0.058*sc; // 翼の付け根Y
                let wt_y = wr_y + flap*0.115*sc; // 翼先端Y
                // 左翼
                let wl0 = cx-0.078*sc; let wl1 = wl0-0.155*sc; let wl2 = wl1-0.100*sc;
                let wy1 = wr_y; let wy2 = wr_y+(wt_y-wr_y)*0.50; let wy3 = wt_y;
                push_box(verts, idxs, wl1, wy1-wh, cz-0.055*sc, wl0, wy1+wh, cz+0.055*sc, wing_col);
                push_box(verts, idxs, wl2, wy2-wh, cz-0.042*sc, wl1, wy2+wh, cz+0.042*sc, wing_col);
                push_box(verts, idxs, wl2-0.075*sc, wy3-wh, cz-0.025*sc, wl2, wy3+wh, cz+0.025*sc, wing_col);
                // 右翼
                let wr0 = cx+0.078*sc; let wr1 = wr0+0.155*sc; let wr2 = wr1+0.100*sc;
                push_box(verts, idxs, wr0, wy1-wh, cz-0.055*sc, wr1, wy1+wh, cz+0.055*sc, wing_col);
                push_box(verts, idxs, wr1, wy2-wh, cz-0.042*sc, wr2, wy2+wh, cz+0.042*sc, wing_col);
                push_box(verts, idxs, wr2, wy3-wh, cz-0.025*sc, wr2+0.075*sc, wy3+wh, cz+0.025*sc, wing_col);
                if lights.len() < 4 { lights.push(Light { pos:[cx,base+0.06*sc,cz,hash], col:[0.6,0.2,0.8,0.9*sc] }); }
            }

            // ━━ オーク (TILE_ORC): 緑の戦士・牙・武器 ━━
            if base_t == TILE_ORC {
                let cx = (x0+x1)*0.5; let cz = (z0+z1)*0.5;
                let hash = (tx * 7 + ty * 13) as f32;
                let sc = tier_base * match (tx * 3 + ty * 5) % 2 { 0 => 0.92_f32, _ => 1.10 };
                let skin_col   = [0.20_f32, 0.50, 0.14, 1.0];
                let armor_col  = [0.32_f32, 0.28, 0.20, 1.0];
                let eye_col    = [0.95_f32, 0.62, 0.05, 3.0]; // 黄色い目
                let tusk_col   = [0.88_f32, 0.84, 0.72, 1.0];
                let weapon_col = [0.52_f32, 0.50, 0.44, 1.0];
                let bob   = (time * 3.5 + hash).sin().abs() * 0.014 * sc;
                let swing = (time * 2.8 + hash).sin();
                // 脚
                push_box(verts, idxs, cx-0.14*sc, 0.0, cz-0.07*sc, cx-0.06*sc, 0.19*sc, cz+0.07*sc, armor_col);
                push_box(verts, idxs, cx+0.06*sc, 0.0, cz-0.07*sc, cx+0.14*sc, 0.19*sc, cz+0.07*sc, armor_col);
                // 胴体
                let by0 = 0.17*sc; let by1 = by0+0.25*sc+bob;
                push_box(verts, idxs, cx-0.17*sc, by0, cz-0.11*sc, cx+0.17*sc, by1, cz+0.11*sc, armor_col);
                // 腕
                push_box(verts, idxs, cx-0.24*sc, by0+0.04*sc, cz-0.062*sc, cx-0.17*sc, by1-0.04*sc, cz+0.062*sc, skin_col);
                let arm_y = by0+0.10*sc+swing*0.06*sc;
                push_box(verts, idxs, cx+0.17*sc, arm_y, cz-0.062*sc, cx+0.24*sc, by1-0.02*sc+swing*0.04*sc, cz+0.062*sc, skin_col);
                // 頭
                let hy0 = by1-0.012*sc; let hy1 = hy0+0.20*sc;
                push_box(verts, idxs, cx-0.13*sc, hy0, cz-0.10*sc, cx+0.13*sc, hy1, cz+0.082*sc, skin_col);
                // 牙 (下からはみ出る)
                push_box(verts, idxs, cx-0.082*sc, hy0-0.058*sc, cz+0.058*sc, cx-0.038*sc, hy0, cz+0.090*sc, tusk_col);
                push_box(verts, idxs, cx+0.038*sc, hy0-0.058*sc, cz+0.058*sc, cx+0.082*sc, hy0, cz+0.090*sc, tusk_col);
                // 目 (黄色い)
                let es = 0.030*sc; let eyz = hy0+(hy1-hy0)*0.58; let ezf = cz+0.068*sc;
                push_box(verts, idxs, cx-0.068*sc-es, eyz-es*0.80, ezf, cx-0.068*sc+es, eyz+es*0.80, ezf+es*0.50, eye_col);
                push_box(verts, idxs, cx+0.068*sc-es, eyz-es*0.80, ezf, cx+0.068*sc+es, eyz+es*0.80, ezf+es*0.50, eye_col);
                // 武器 (右手)
                let wx = cx+0.25*sc; let wy = by1+swing*0.08*sc;
                push_box(verts, idxs, wx-0.018*sc, by0*0.4, cz-0.018*sc, wx+0.018*sc, wy+0.28*sc, cz+0.018*sc, weapon_col);
                // 武器の刃
                push_box(verts, idxs, wx-0.035*sc, wy+0.18*sc, cz-0.010*sc, wx+0.035*sc, wy+0.28*sc, cz+0.010*sc, [0.70,0.72,0.80,1.0]);
                if lights.len() < 4 { lights.push(Light { pos:[cx,by1,cz,hash], col:[0.7,1.0,0.2,1.0*sc] }); }
            }

            // ━━ ゾンビ (TILE_ZOMBIE): 腐った肌・腕を前に・黄緑の目 ━━
            if base_t == TILE_ZOMBIE {
                let cx = (x0+x1)*0.5; let cz = (z0+z1)*0.5;
                let hash = (tx * 11 + ty * 5) as f32;
                let sc = tier_base * 1.0_f32;
                let skin_col  = [0.50_f32, 0.56, 0.40, 1.0];
                let cloth_col = [0.26_f32, 0.22, 0.16, 1.0];
                let eye_col   = [0.82_f32, 0.95, 0.28, 3.0]; // 黄緑の光る目!
                let bone_col  = [0.76_f32, 0.72, 0.58, 1.0];
                let shamble   = (time * 2.2 + hash).sin();
                let bob       = (time * 4.4 + hash).sin().abs() * 0.010;
                let arm_reach = (time * 1.5 + hash).sin() * 0.048;
                // 脚 (片方を引きずる)
                let drag = shamble * 0.025;
                push_box(verts, idxs, cx-0.13*sc, 0.0, cz-0.072*sc, cx-0.05*sc, 0.20*sc, cz+0.072*sc, cloth_col);
                push_box(verts, idxs, cx+0.05*sc, drag.abs(), cz-0.072*sc+drag, cx+0.13*sc, 0.20*sc+drag.abs(), cz+0.072*sc+drag, cloth_col);
                // 胴体
                let by0 = 0.18*sc; let by1 = by0+0.26*sc+bob;
                push_box(verts, idxs, cx-0.14*sc, by0, cz-0.10*sc, cx+0.14*sc, by1, cz+0.10*sc, cloth_col);
                push_box(verts, idxs, cx-0.08*sc, by0+0.04*sc, cz+0.088*sc, cx+0.08*sc, by0+0.18*sc, cz+0.112*sc, skin_col);
                // 腕 (ゾンビらしく両方前へ!)
                let arm_z = cz+0.14*sc+arm_reach;
                push_box(verts, idxs, cx-0.23*sc, by0+0.09*sc, cz+0.008*sc, cx-0.14*sc, by1-0.08*sc, arm_z, skin_col);
                push_box(verts, idxs, cx+0.14*sc, by0+0.09*sc, cz+0.008*sc, cx+0.23*sc, by1-0.08*sc, arm_z, skin_col);
                // 手 (骨っぽい)
                push_box(verts, idxs, cx-0.24*sc, by0+0.08*sc, arm_z, cx-0.18*sc, by0+0.16*sc, arm_z+0.060*sc, bone_col);
                push_box(verts, idxs, cx+0.18*sc, by0+0.08*sc, arm_z, cx+0.24*sc, by0+0.16*sc, arm_z+0.060*sc, bone_col);
                // 頭
                let hy0 = by1-0.018*sc; let hy1 = hy0+0.19*sc; let tilt = shamble*0.010;
                push_box(verts, idxs, cx-0.12*sc+tilt, hy0, cz-0.092*sc, cx+0.12*sc+tilt, hy1, cz+0.072*sc, skin_col);
                // ガタガタの歯
                push_box(verts, idxs, cx-0.072*sc, hy0-0.008*sc, cz+0.048*sc, cx+0.072*sc, hy0+0.022*sc, cz+0.090*sc, bone_col);
                // 黄緑の目 (光る!)
                let es = 0.028*sc; let eyz = hy0+(hy1-hy0)*0.56; let ezf = cz+0.058*sc;
                push_box(verts, idxs, cx-0.058*sc-es, eyz-es*0.70, ezf, cx-0.058*sc+es, eyz+es*0.70, ezf+es*0.48, eye_col);
                push_box(verts, idxs, cx+0.058*sc-es, eyz-es*0.70, ezf, cx+0.058*sc+es, eyz+es*0.70, ezf+es*0.48, eye_col);
                if lights.len() < 4 { lights.push(Light { pos:[cx,by1,cz,hash], col:[0.4,0.9,0.2,1.3] }); }
            }


            // ━━ ヘビ (TILE_SNAKE): 波打つ胴体・舌・黄色い目 ━━
            if base_t == TILE_SNAKE {
                let cx = (x0+x1)*0.5; let cz = (z0+z1)*0.5;
                let hash = (tx * 17 + ty * 7) as f32;
                let sc = tier_base * match (tx*5+ty*3)%3 { 0=>0.70_f32, 1=>1.00, _=>1.20 };
                let col: [f32;4] = match (tx*7+ty*11)%5 {
                    0 => [0.28, 0.55, 0.18, 1.0], // 緑
                    1 => [0.62, 0.18, 0.10, 1.0], // 赤 (毒蛇)
                    2 => [0.55, 0.50, 0.14, 1.0], // 黄
                    3 => [0.18, 0.16, 0.20, 1.0], // 黒
                    _ => [0.48, 0.42, 0.28, 1.0], // 茶
                };
                let belly    = [(col[0]*1.4).min(1.0),(col[1]*1.5).min(1.0),(col[2]*1.2).min(1.0),1.0];
                let eye_col  = [0.88_f32,0.86,0.10,3.0];
                let eye_sh   = [1.0_f32,1.0,0.80,3.0];
                let tng_col  = [0.92_f32,0.10,0.14,1.0];
                let lift     = (time * 3.0 + hash).sin().abs() * 0.032 * sc;
                let sw = 0.062*sc; let sh = 0.058*sc; let sd = 0.092*sc;
                // 6セグメントの波打つ胴体
                for i in 0..6_i32 {
                    let fi  = i as f32;
                    let seg_wave = (time*2.5+hash*0.4+fi*0.75).sin()*0.075*sc;
                    let taper = (1.0 - fi*0.08).max(0.50);
                    push_box(verts, idxs,
                        cx+seg_wave-sw*taper, 0.036*sc, cz+(fi-2.5)*sd*0.88-sd*0.44,
                        cx+seg_wave+sw*taper, 0.036*sc+sh*taper, cz+(fi-2.5)*sd*0.88+sd*0.44, col);
                }
                // 腹
                let bw=(time*2.5+hash*0.4).sin()*0.060*sc;
                push_box(verts,idxs,cx+bw-sw*0.65,0.0,cz-sd*2.5,cx+bw+sw*0.65,0.006,cz+sd*2.5,belly);
                // 頭
                let hw=(time*2.5+hash*0.4-2.9).sin()*0.095*sc;
                let hx=0.075*sc; let hz=0.085*sc; let hy=0.070*sc;
                let hcz=cz-sd*2.75; let hcy=0.058*sc+lift;
                push_box(verts,idxs,cx+hw-hx,hcy,hcz-hz,cx+hw+hx,hcy+hy,hcz+hz*0.45,col);
                // 目
                let es=0.020*sc; let eyz=hcy+hy*0.65; let ezf=hcz+hz*0.38;
                push_box(verts,idxs,cx+hw-hx*0.46-es,eyz-es*0.6,ezf,cx+hw-hx*0.46+es,eyz+es*0.6,ezf+es*0.5,eye_col);
                push_box(verts,idxs,cx+hw-hx*0.46+es*0.1,eyz+es*0.05,ezf+es*0.42,cx+hw-hx*0.46+es*0.65,eyz+es*0.58,ezf+es*0.60,eye_sh);
                push_box(verts,idxs,cx+hw+hx*0.46-es,eyz-es*0.6,ezf,cx+hw+hx*0.46+es,eyz+es*0.6,ezf+es*0.5,eye_col);
                push_box(verts,idxs,cx+hw+hx*0.46-es*0.65,eyz+es*0.05,ezf+es*0.42,cx+hw+hx*0.46-es*0.1,eyz+es*0.58,ezf+es*0.60,eye_sh);
                // 二股の舌
                let tp=(time*4.0+hash).sin().abs()*0.022*sc+0.025*sc;
                let tby=hcy+hy*0.30; let tbz=hcz+hz*0.40;
                push_box(verts,idxs,cx+hw-0.007*sc,tby,tbz,cx+hw+0.007*sc,tby+0.006*sc,tbz+tp*0.65,tng_col);
                push_box(verts,idxs,cx+hw-0.020*sc,tby+0.001,tbz+tp*0.55,cx+hw-0.007*sc,tby+0.006*sc,tbz+tp,tng_col);
                push_box(verts,idxs,cx+hw+0.007*sc,tby+0.001,tbz+tp*0.55,cx+hw+0.020*sc,tby+0.006*sc,tbz+tp,tng_col);
                if lights.len()<4 { lights.push(Light{pos:[cx+hw,hcy+hy,hcz,hash],col:[0.4,0.9,0.2,0.8*sc]}); }
            }

            // ━━ 浮遊する目玉 (TILE_EYE): NetHack最恐アイコン ━━
            if base_t == TILE_EYE {
                let cx = (x0+x1)*0.5; let cz = (z0+z1)*0.5;
                let hash = (tx*13+ty*17) as f32;
                let sc = tier_base * 1.0_f32;
                let float_y = 0.26*sc + (time*1.8+hash).sin()*0.042*sc;
                let spin    = time*0.9+hash;
                let spx     = spin.sin()*0.11*sc;
                let spz     = spin.cos()*0.11*sc;
                let white    = [0.96_f32,0.94,0.92,1.0];
                let iris: [f32;4] = match (tx*7+ty*3)%4 {
                    0 => [0.12,0.68,0.24,3.0], // 翠
                    1 => [0.82,0.52,0.08,3.0], // 金
                    2 => [0.12,0.32,0.88,3.0], // 青
                    _ => [0.78,0.15,0.15,3.0], // 赤 (怖い!)
                };
                let pupil = [0.04_f32,0.03,0.05,3.0];
                let shine = [1.0_f32,1.0,0.96,4.0];
                let vein  = [0.88_f32,0.30,0.26,1.0];
                let er = 0.24*sc;
                // 白目
                push_box(verts,idxs,cx-er,float_y-er,cz-er,cx+er,float_y+er,cz+er,white);
                // 虹彩
                let ir=er*0.68;
                push_box(verts,idxs,cx+spx-ir,float_y-ir,cz+spz+er*0.84,cx+spx+ir,float_y+ir,cz+spz+er*0.92,iris);
                // 縦瞳孔
                let pr=ir*0.36;
                push_box(verts,idxs,cx+spx-pr*0.50,float_y-pr,cz+spz+er*0.85,cx+spx+pr*0.50,float_y+pr,cz+spz+er*0.93,pupil);
                // ハイライト×2
                push_box(verts,idxs,cx+spx+pr*0.18,float_y+ir*0.28,cz+spz+er*0.880,cx+spx+pr*0.82,float_y+ir*0.72,cz+spz+er*0.936,shine);
                push_box(verts,idxs,cx+spx-pr*0.75,float_y-ir*0.12,cz+spz+er*0.880,cx+spx-pr*0.18,float_y+ir*0.28,cz+spz+er*0.936,shine);
                // 血管 (3本)
                let vw=er*0.038;
                push_box(verts,idxs,cx-vw,float_y+er*0.38,cz+er*0.58,cx+vw,float_y+er*0.88,cz+er*0.72,vein);
                push_box(verts,idxs,cx+er*0.32,float_y-vw,cz+er*0.55,cx+er*0.85,float_y+vw,cz+er*0.70,vein);
                push_box(verts,idxs,cx-er*0.85,float_y-vw*0.5,cz+er*0.55,cx-er*0.32,float_y+vw*0.5,cz+er*0.70,vein);
                // 視神経 (後ろ)
                push_box(verts,idxs,cx-er*0.14,float_y-er*0.10,cz-er,cx+er*0.14,float_y+er*0.10,cz-er*1.20,[0.84,0.80,0.76,1.0]);
                if lights.len()<4 { lights.push(Light{pos:[cx+spx,float_y,cz+spz,hash],col:[iris[0]*0.9,iris[1]*0.9,iris[2]*0.9,2.2*sc]}); }
            }

            // ━━ ユニコーン (TILE_UNICORN): 馬体+ホーン+たてがみ ━━
            if base_t == TILE_UNICORN {
                let cx = (x0+x1)*0.5; let cz = (z0+z1)*0.5;
                let hash = (tx*11+ty*9) as f32;
                let sc = tier_base * 0.95_f32;
                let body_col: [f32;4] = match (tx*5+ty*7)%4 {
                    0 => [0.96,0.95,0.94,1.0], // 白
                    1 => [0.88,0.72,0.90,1.0], // 薄紫
                    2 => [0.80,0.88,0.96,1.0], // 水色
                    _ => [0.94,0.88,0.72,1.0], // クリーム
                };
                let mane_col = [0.85_f32,0.65,0.92,1.0]; // たてがみ (虹色っぽく紫)
                let horn_col = [0.96_f32,0.92,0.55,1.0]; // 金の角
                let hoof_col = [0.45_f32,0.42,0.38,1.0]; // 蹄
                let eye_col  = [0.08_f32,0.06,0.10,3.0];
                let eye_sh   = [1.0_f32,0.98,0.96,4.0];
                let walk = (time*3.5+hash*0.4).sin();
                let base_y = (time*7.0+hash*0.5).sin().abs()*0.012*sc;
                let by0=0.22*sc+base_y; let by1=by0+0.28*sc;
                let bfr=0.28*sc; let bbk=0.25*sc; let bw=0.12*sc;
                // 胴体
                push_box(verts,idxs,cx-bw,by0,cz-bbk,cx+bw,by1,cz+bfr,body_col);
                // 首 (前方上へ傾く)
                let nk_x0=cx-0.08*sc; let nk_x1=cx+0.08*sc;
                let nk_y0=by1-0.04*sc; let nk_y1=nk_y0+0.24*sc;
                let nk_z0=cz+bfr-0.05*sc; let nk_z1=cz+bfr+0.08*sc;
                push_box(verts,idxs,nk_x0,nk_y0,nk_z0,nk_x1,nk_y1,nk_z1,body_col);
                // 頭
                let hcz=cz+bfr+0.14*sc; let hy0=nk_y1-0.04*sc; let hy1=hy0+0.16*sc;
                push_box(verts,idxs,cx-0.075*sc,hy0,hcz-0.055*sc,cx+0.075*sc,hy1,hcz+0.08*sc,body_col);
                // 黄金の角 (ホーン!)
                let hn_y0=hy1; let hn_y1=hn_y0+0.22*sc;
                push_box(verts,idxs,cx-0.014*sc,hn_y0,hcz-0.014*sc,cx+0.014*sc,hn_y1,hcz+0.014*sc,horn_col);
                push_box(verts,idxs,cx-0.008*sc,hn_y0+0.14*sc,hcz-0.008*sc,cx+0.008*sc,hn_y1+0.04*sc,hcz+0.008*sc,horn_col);
                // たてがみ (首に沿って紫の板)
                push_box(verts,idxs,cx-0.005*sc,nk_y0+0.04*sc,nk_z0-0.015*sc,cx+0.005*sc,nk_y1+0.08*sc,nk_z1-0.005*sc,mane_col);
                // 目
                let es=0.022*sc; let eyz=hy0+(hy1-hy0)*0.60; let ezf=hcz+0.062*sc;
                push_box(verts,idxs,cx-0.058*sc-es,eyz-es*0.8,ezf,cx-0.058*sc+es,eyz+es*0.8,ezf+es*0.45,eye_col);
                push_box(verts,idxs,cx-0.058*sc+es*0.1,eyz+es*0.1,ezf+es*0.38,cx-0.058*sc+es*0.7,eyz+es*0.75,ezf+es*0.56,eye_sh);
                push_box(verts,idxs,cx+0.058*sc-es,eyz-es*0.8,ezf,cx+0.058*sc+es,eyz+es*0.8,ezf+es*0.45,eye_col);
                push_box(verts,idxs,cx+0.058*sc-es*0.7,eyz+es*0.1,ezf+es*0.38,cx+0.058*sc-es*0.1,eyz+es*0.75,ezf+es*0.56,eye_sh);
                // 4本脚 (蹄付き)
                let lw=0.055*sc; let lh=by0;
                let lfy=lh*(1.0+walk*0.07); let lby=lh*(1.0-walk*0.07);
                let leg_col=[body_col[0]*0.88,body_col[1]*0.88,body_col[2]*0.88,1.0];
                push_box(verts,idxs,cx-bw+lw*0.2,0.0,cz+bfr-lw*2.0,cx-bw+lw*0.2+lw*2.0,lfy,cz+bfr,leg_col);
                push_box(verts,idxs,cx+bw-lw*0.2-lw*2.0,0.0,cz+bfr-lw*2.0,cx+bw-lw*0.2,lby,cz+bfr,leg_col);
                push_box(verts,idxs,cx-bw+lw*0.2,0.0,cz-bbk,cx-bw+lw*0.2+lw*2.0,lby,cz-bbk+lw*2.0,leg_col);
                push_box(verts,idxs,cx+bw-lw*0.2-lw*2.0,0.0,cz-bbk,cx+bw-lw*0.2,lfy,cz-bbk+lw*2.0,leg_col);
                // 蹄 (4つ)
                for (lx0,lx1,lz0,lz1) in [
                    (cx-bw+lw*0.2,cx-bw+lw*2.2,cz+bfr-lw*2.0,cz+bfr),
                    (cx+bw-lw*2.2,cx+bw-lw*0.2,cz+bfr-lw*2.0,cz+bfr),
                    (cx-bw+lw*0.2,cx-bw+lw*2.2,cz-bbk,cz-bbk+lw*2.0),
                    (cx+bw-lw*2.2,cx+bw-lw*0.2,cz-bbk,cz-bbk+lw*2.0),
                ] {
                    push_box(verts,idxs,lx0,0.0,lz0,lx1,0.022*sc,lz1,hoof_col);
                }
                // しっぽ (たてがみ色)
                let tw=(time*2.2+hash).sin()*0.06*sc;
                push_box(verts,idxs,cx+tw-0.018*sc,by0*0.5,cz-bbk-0.02*sc,cx+tw+0.018*sc,by1*0.55,cz-bbk-0.18*sc,mane_col);
                push_box(verts,idxs,cx+tw-0.025*sc,by0*0.3,cz-bbk-0.18*sc,cx+tw+0.025*sc,by0*0.5+0.02*sc,cz-bbk-0.30*sc,mane_col);
                if lights.len()<4 { lights.push(Light{pos:[cx,hy1+0.12*sc,hcz,hash],col:[0.9,0.8,1.0,1.6*sc]}); }
            }

            // ━━ トロル (TILE_TROLL): 大型・岩肌・巨大な手・棍棒 ━━
            if base_t == TILE_TROLL {
                let cx = (x0+x1)*0.5; let cz = (z0+z1)*0.5;
                let hash = (tx*9+ty*11) as f32;
                let sc = tier_base * match (tx*3+ty*5)%2 { 0=>1.05_f32, _=>1.20 };
                let skin_col  = [0.42_f32,0.48,0.38,1.0]; // 灰緑の岩肌
                let dark_col  = [0.28_f32,0.30,0.24,1.0];
                let eye_col   = [0.92_f32,0.80,0.10,3.0]; // 橙色の目
                let club_col  = [0.38_f32,0.28,0.18,1.0]; // 木の棍棒
                let nail_col  = [0.22_f32,0.20,0.18,1.0]; // 黒い爪
                let bob = (time*2.8+hash).sin().abs()*0.018*sc;
                let lurch = (time*2.8+hash).sin()*0.022*sc; // 前後によろける
                let swing = (time*2.2+hash).sin();
                // 太い脚
                push_box(verts,idxs,cx-0.18*sc,0.0,cz-0.10*sc,cx-0.06*sc,0.22*sc,cz+0.10*sc,dark_col);
                push_box(verts,idxs,cx+0.06*sc,0.0,cz-0.10*sc,cx+0.18*sc,0.22*sc,cz+0.10*sc,dark_col);
                // 巨体 (前後によろける)
                let by0=0.20*sc; let by1=by0+0.36*sc+bob;
                push_box(verts,idxs,cx-0.22*sc,by0,cz-0.14*sc+lurch,cx+0.22*sc,by1,cz+0.14*sc+lurch,skin_col);
                // 太い腕 (垂れ下がった感じ — トロルらしく長い)
                push_box(verts,idxs,cx-0.34*sc,by0-0.04*sc,cz-0.09*sc+lurch,cx-0.21*sc,by1-0.06*sc,cz+0.09*sc+lurch,skin_col);
                let arm_y=by0+swing*0.05*sc;
                push_box(verts,idxs,cx+0.21*sc,arm_y-0.04*sc,cz-0.09*sc+lurch,cx+0.34*sc,by1-0.02*sc+swing*0.04*sc,cz+0.09*sc+lurch,skin_col);
                // 大きな手 (鉤爪付き)
                push_box(verts,idxs,cx-0.38*sc,by0-0.12*sc,cz-0.08*sc+lurch,cx-0.28*sc,by0+0.06*sc,cz+0.08*sc+lurch,skin_col);
                push_box(verts,idxs,cx-0.42*sc,by0-0.16*sc,cz-0.04*sc+lurch,cx-0.36*sc,by0-0.08*sc,cz+0.04*sc+lurch,nail_col);
                // 頭 (大きく低い額、出っ張った眉)
                let hy0=by1-0.015*sc; let hy1=hy0+0.24*sc;
                push_box(verts,idxs,cx-0.18*sc,hy0,cz-0.13*sc+lurch,cx+0.18*sc,hy1,cz+0.10*sc+lurch,skin_col);
                // 眉毛ボーン (突き出る)
                push_box(verts,idxs,cx-0.20*sc,hy0+(hy1-hy0)*0.58,cz+0.08*sc+lurch,cx+0.20*sc,hy0+(hy1-hy0)*0.72,cz+0.13*sc+lurch,dark_col);
                // 目 (橙色)
                let es=0.038*sc; let eyz=hy0+(hy1-hy0)*0.50; let ezf=cz+0.088*sc+lurch;
                push_box(verts,idxs,cx-0.088*sc-es,eyz-es*0.7,ezf,cx-0.088*sc+es,eyz+es*0.7,ezf+es*0.45,eye_col);
                push_box(verts,idxs,cx+0.088*sc-es,eyz-es*0.7,ezf,cx+0.088*sc+es,eyz+es*0.7,ezf+es*0.45,eye_col);
                // 棍棒 (右手上)
                let wx=cx+0.36*sc; let wy=by1+swing*0.10*sc;
                push_box(verts,idxs,wx-0.028*sc,by0*0.3,cz-0.028*sc+lurch,wx+0.028*sc,wy+0.34*sc,cz+0.028*sc+lurch,club_col);
                push_box(verts,idxs,wx-0.055*sc,wy+0.22*sc,cz-0.042*sc+lurch,wx+0.055*sc,wy+0.38*sc,cz+0.042*sc+lurch,club_col);
                if lights.len()<4 { lights.push(Light{pos:[cx,by1,cz,hash],col:[1.0,0.7,0.2,1.4*sc]}); }
            }

            // ━━ ドラゴン (TILE_DRAGON): 鱗・翼・火炎! ━━
            if base_t == TILE_DRAGON {
                let cx = (x0+x1)*0.5; let cz = (z0+z1)*0.5;
                let hash = (tx*13+ty*11) as f32;
                let sc = tier_base * 1.0_f32;
                let scale_col: [f32;4] = match (tx*7+ty*5)%5 {
                    0 => [0.80,0.14,0.10,1.0], // 赤ドラゴン
                    1 => [0.15,0.58,0.20,1.0], // 緑ドラゴン
                    2 => [0.20,0.25,0.72,1.0], // 青ドラゴン
                    3 => [0.72,0.62,0.12,1.0], // 金ドラゴン
                    _ => [0.15,0.12,0.14,1.0], // 黒ドラゴン
                };
                let belly_col = [(scale_col[0]*1.3).min(0.9),(scale_col[1]*1.3).min(0.9),(scale_col[2]*1.3).min(0.9),1.0];
                let wing_col  = [scale_col[0]*0.72,scale_col[1]*0.72,scale_col[2]*0.72,1.0];
                let eye_col   = [0.96_f32,0.88,0.10,3.0]; // 金の目
                let fire_col  = [1.0_f32,0.55,0.08,3.0];  // 炎
                let fire_tip  = [1.0_f32,0.92,0.35,3.0];  // 炎の先
                let bob  = (time*2.2+hash).sin().abs()*0.014*sc;
                let flap = (time*3.5+hash).sin(); // 翼ばたき
                let roar = (time*1.8+hash).sin(); // 口を開け閉め
                // 胴体 (大きく太い)
                let bw=0.20*sc; let by0=0.18*sc+bob; let by1=by0+0.30*sc;
                let bfr=0.30*sc; let bbk=0.28*sc;
                push_box(verts,idxs,cx-bw,by0,cz-bbk,cx+bw,by1,cz+bfr,scale_col);
                push_box(verts,idxs,cx-bw*0.80,by0,cz-bbk*0.65,cx+bw*0.80,by0+0.008,cz+bfr*0.65,belly_col);
                // 4本脚 (太く力強い)
                let lw=0.072*sc; let lh=by0;
                let leg_col=[scale_col[0]*0.85,scale_col[1]*0.85,scale_col[2]*0.85,1.0];
                push_box(verts,idxs,cx-bw+lw*0.1,0.0,cz+bfr-lw*2.2,cx-bw+lw*2.1,lh,cz+bfr,leg_col);
                push_box(verts,idxs,cx+bw-lw*2.1,0.0,cz+bfr-lw*2.2,cx+bw-lw*0.1,lh,cz+bfr,leg_col);
                push_box(verts,idxs,cx-bw+lw*0.1,0.0,cz-bbk,cx-bw+lw*2.1,lh,cz-bbk+lw*2.2,leg_col);
                push_box(verts,idxs,cx+bw-lw*2.1,0.0,cz-bbk,cx+bw-lw*0.1,lh,cz-bbk+lw*2.2,leg_col);
                // 首 (長く斜め上)
                let ncx=cx; let ncy0=by1-0.02*sc; let ncy1=ncy0+0.28*sc;
                let ncz0=cz+bfr-0.05*sc; let ncz1=cz+bfr+0.12*sc;
                push_box(verts,idxs,ncx-0.10*sc,ncy0,ncz0,ncx+0.10*sc,ncy1,ncz1,scale_col);
                // 頭 (三角形っぽく)
                let hcz=cz+bfr+0.22*sc; let hy0=ncy1-0.04*sc; let hy1=hy0+0.18*sc;
                push_box(verts,idxs,cx-0.12*sc,hy0,hcz-0.08*sc,cx+0.12*sc,hy1,hcz+0.14*sc,scale_col);
                // 顎 (開閉アニメ)
                let jaw_open=(roar*0.028+0.015)*sc;
                push_box(verts,idxs,cx-0.10*sc,hy0-jaw_open-0.05*sc,hcz-0.04*sc,cx+0.10*sc,hy0-jaw_open,hcz+0.16*sc,scale_col);
                // 火炎 (口から)
                let fire_phase=(time*5.0+hash).sin();
                if fire_phase>0.0 {
                    let fl=fire_phase*0.18*sc;
                    push_box(verts,idxs,cx-0.06*sc,hy0-jaw_open*0.5,hcz+0.14*sc,cx+0.06*sc,hy0+0.04*sc,hcz+0.14*sc+fl,fire_col);
                    push_box(verts,idxs,cx-0.03*sc,hy0-jaw_open*0.5+0.008*sc,hcz+0.14*sc+fl*0.5,cx+0.03*sc,hy0+0.02*sc,hcz+0.14*sc+fl,fire_tip);
                }
                // 角 (2本)
                push_box(verts,idxs,cx-0.10*sc,hy1,hcz-0.04*sc,cx-0.05*sc,hy1+0.12*sc,hcz+0.02*sc,scale_col);
                push_box(verts,idxs,cx+0.05*sc,hy1,hcz-0.04*sc,cx+0.10*sc,hy1+0.12*sc,hcz+0.02*sc,scale_col);
                // 目 (金色)
                let es=0.030*sc; let eyz=hy0+(hy1-hy0)*0.62; let ezf=hcz+0.12*sc;
                push_box(verts,idxs,cx-0.068*sc-es,eyz-es*0.8,ezf,cx-0.068*sc+es,eyz+es*0.8,ezf+es*0.42,eye_col);
                push_box(verts,idxs,cx+0.068*sc-es,eyz-es*0.8,ezf,cx+0.068*sc+es,eyz+es*0.8,ezf+es*0.42,eye_col);
                // 翼 (大きく羽ばたく — 4段)
                let wh=0.030*sc; let wry=by1*0.75; let wty=wry+flap*0.22*sc;
                // 左翼
                let wl=[cx-bw,cx-bw-0.22*sc,cx-bw-0.40*sc,cx-bw-0.54*sc];
                let wly=[wry,wry+(wty-wry)*0.33,wry+(wty-wry)*0.67,wty];
                for i in 0..3_usize {
                    let zdelta=(i as f32)*0.008*sc;
                    push_box(verts,idxs,wl[i+1],wly[i+1]-wh,cz-zdelta,wl[i],wly[i]+wh,cz+0.055*sc-zdelta,wing_col);
                }
                // 右翼
                let wr=[cx+bw,cx+bw+0.22*sc,cx+bw+0.40*sc,cx+bw+0.54*sc];
                for i in 0..3_usize {
                    let zdelta=(i as f32)*0.008*sc;
                    push_box(verts,idxs,wr[i],wly[i]-wh,cz-zdelta,wr[i+1],wly[i+1]+wh,cz+0.055*sc-zdelta,wing_col);
                }
                // しっぽ (長く後方へ)
                let tw=0.060*sc; let tsway=(time*2.0+hash).sin();
                push_box(verts,idxs,cx-tw,by0*0.5,cz-bbk-0.02*sc,cx+tw,by1*0.55,cz-bbk+0.04*sc,scale_col);
                let t2x=cx+tsway*0.08*sc;
                push_box(verts,idxs,t2x-tw*0.80,by0*0.3,cz-bbk-0.22*sc,t2x+tw*0.80,by0*0.5+0.02*sc,cz-bbk-0.01*sc,scale_col);
                push_box(verts,idxs,t2x-tw*0.55,0.0,cz-bbk-0.38*sc,t2x+tw*0.55,by0*0.25+0.02*sc,cz-bbk-0.21*sc,scale_col);
                if lights.len()<4 { lights.push(Light{pos:[cx,hy1,hcz,hash],col:[fire_col[0],fire_col[1]*0.7,0.1,2.5*sc]}); }
            }


            // ━━ レプラコーン (TILE_LEPRECHAUN): 緑の帽子・金貨・四つ葉 ━━
            if base_t == TILE_LEPRECHAUN {
                let cx = (x0+x1)*0.5; let cz = (z0+z1)*0.5;
                let hash = (tx*11+ty*17) as f32;
                let sc = tier_base * match (tx*5+ty*3)%2 { 0=>0.58_f32, _=>0.70 };
                let green_coat = [0.10_f32,0.42,0.12,1.0];
                let dk_green   = [0.06_f32,0.28,0.08,1.0];
                let skin_col   = [0.88_f32,0.70,0.52,1.0];
                let gold_col   = [0.95_f32,0.82,0.10,3.0]; // 金貨 (光る!)
                let hat_col    = [0.06_f32,0.32,0.06,1.0];
                let hat_band   = [0.80_f32,0.72,0.10,1.0];
                let shoe_col   = [0.12_f32,0.08,0.04,1.0];
                let eye_col    = [0.12_f32,0.52,0.18,3.0]; // 緑の目!
                let bob  = (time*3.8+hash).sin().abs()*0.012*sc;
                let jig  = (time*6.0+hash).sin()*0.018*sc; // 陽気なジグ
                let arm_swing = (time*4.5+hash).sin();
                // 靴 (ピカピカ!)
                push_box(verts,idxs,cx-0.11*sc,0.0,cz-0.06*sc,cx-0.04*sc,0.05*sc,cz+0.07*sc,shoe_col);
                push_box(verts,idxs,cx+0.04*sc,0.0,cz-0.06*sc,cx+0.11*sc,0.05*sc,cz+0.07*sc,shoe_col);
                // 脚 (縞ストッキング)
                let stripe0=[0.82_f32,0.82,0.78,1.0]; let stripe1=[0.22_f32,0.58,0.22,1.0];
                push_box(verts,idxs,cx-0.095*sc,0.05*sc,cz-0.055*sc,cx-0.045*sc,0.15*sc,cz+0.055*sc,stripe0);
                push_box(verts,idxs,cx+0.045*sc,0.05*sc,cz-0.055*sc,cx+0.095*sc,0.15*sc,cz+0.055*sc,stripe1);
                push_box(verts,idxs,cx-0.095*sc,0.07*sc,cz-0.058*sc,cx-0.042*sc,0.10*sc,cz+0.058*sc,stripe1);
                push_box(verts,idxs,cx+0.042*sc,0.07*sc,cz-0.058*sc,cx+0.095*sc,0.10*sc,cz+0.058*sc,stripe0);
                // 胴体 (緑の上着)
                let by0=0.14*sc+bob; let by1=by0+0.20*sc;
                push_box(verts,idxs,cx-0.11*sc,by0,cz-0.08*sc,cx+0.11*sc,by1,cz+0.08*sc,green_coat);
                // ベルト (金バックル)
                push_box(verts,idxs,cx-0.11*sc,by0+0.02*sc,cz+0.076*sc,cx+0.11*sc,by0+0.05*sc,cz+0.085*sc,[0.12,0.08,0.04,1.0]);
                push_box(verts,idxs,cx-0.022*sc,by0+0.018*sc,cz+0.077*sc,cx+0.022*sc,by0+0.055*sc,cz+0.087*sc,hat_band);
                // 腕
                let arm_l = arm_swing*0.035*sc;
                push_box(verts,idxs,cx-0.19*sc,by0+0.06*sc,cz-0.04*sc+arm_l,cx-0.11*sc,by1-0.04*sc,cz+0.04*sc+arm_l,dk_green);
                push_box(verts,idxs,cx+0.11*sc,by0+0.06*sc,cz-0.04*sc-arm_l,cx+0.19*sc,by1-0.04*sc,cz+0.04*sc-arm_l,dk_green);
                // 右手に金貨!
                push_box(verts,idxs,cx+0.17*sc,by0+0.10*sc,cz+0.04*sc-arm_l,cx+0.22*sc,by0+0.14*sc,cz+0.08*sc-arm_l,gold_col);
                // 顔
                let hy0=by1-0.01*sc+bob; let hy1=hy0+0.16*sc;
                let hcz=cz+0.04*sc;
                push_box(verts,idxs,cx-0.085*sc,hy0,hcz-0.065*sc+jig,cx+0.085*sc,hy1,hcz+0.065*sc+jig,skin_col);
                // ほっぺ (赤)
                push_box(verts,idxs,cx-0.082*sc,hy0+0.042*sc,hcz+0.056*sc+jig,cx-0.045*sc,hy0+0.080*sc,hcz+0.068*sc+jig,[0.92,0.48,0.40,1.0]);
                push_box(verts,idxs,cx+0.045*sc,hy0+0.042*sc,hcz+0.056*sc+jig,cx+0.082*sc,hy0+0.080*sc,hcz+0.068*sc+jig,[0.92,0.48,0.40,1.0]);
                // 目 (緑ぱっちり)
                let es=0.020*sc; let eyz=hy0+(hy1-hy0)*0.56; let ezf=hcz+0.055*sc+jig;
                push_box(verts,idxs,cx-0.040*sc-es,eyz-es,ezf,cx-0.040*sc+es,eyz+es,ezf+es*0.5,eye_col);
                push_box(verts,idxs,cx+0.040*sc-es,eyz-es,ezf,cx+0.040*sc+es,eyz+es,ezf+es*0.5,eye_col);
                // 口ひげ (白)
                let mst=[0.94_f32,0.92,0.88,1.0];
                push_box(verts,idxs,cx-0.055*sc,hy0+0.020*sc,hcz+0.055*sc+jig,cx+0.055*sc,hy0+0.038*sc,hcz+0.072*sc+jig,mst);
                // トップハット (一番の特徴!)
                let hat_y0=hy1+jig; let hat_brim_y=hat_y0+0.018*sc;
                // ブリム (つば)
                push_box(verts,idxs,cx-0.115*sc,hat_y0,hcz-0.090*sc,cx+0.115*sc,hat_brim_y,hcz+0.090*sc,hat_col);
                // バンド
                push_box(verts,idxs,cx-0.075*sc,hat_brim_y,hcz-0.068*sc,cx+0.075*sc,hat_brim_y+0.020*sc,hcz+0.068*sc,hat_band);
                // 筒 (シルクハット本体)
                push_box(verts,idxs,cx-0.072*sc,hat_brim_y+0.018*sc,hcz-0.065*sc,cx+0.072*sc,hat_brim_y+0.18*sc,hcz+0.065*sc,hat_col);
                // 四つ葉クローバー (帽子に飾り)
                let cl=[0.08_f32,0.70,0.18,3.0];
                let cly=hat_brim_y+0.20*sc; let clz=hcz-0.068*sc;
                push_box(verts,idxs,cx-0.025*sc,cly,clz-0.020*sc,cx+0.025*sc,cly+0.020*sc,clz,[0.10,0.52,0.12,1.0]); // 茎
                push_box(verts,idxs,cx-0.028*sc,cly+0.020*sc,clz-0.022*sc,cx,cly+0.050*sc,clz+0.006*sc,cl);
                push_box(verts,idxs,cx,cly+0.020*sc,clz-0.022*sc,cx+0.028*sc,cly+0.050*sc,clz+0.006*sc,cl);
                push_box(verts,idxs,cx-0.028*sc,cly+0.042*sc,clz-0.022*sc,cx,cly+0.072*sc,clz+0.006*sc,cl);
                push_box(verts,idxs,cx,cly+0.042*sc,clz-0.022*sc,cx+0.028*sc,cly+0.072*sc,clz+0.006*sc,cl);
                if lights.len()<4 { lights.push(Light{pos:[cx+0.18*sc,by0+0.12*sc,cz,hash],col:[0.95,0.82,0.10,1.8*sc]}); }
            }

            // ━━ コボルド (TILE_KOBOLD): 小型トカゲ人・スケイル・短い槍 ━━
            if base_t == TILE_KOBOLD {
                let cx = (x0+x1)*0.5; let cz = (z0+z1)*0.5;
                let hash = (tx*7+ty*19) as f32;
                let sc = tier_base * match (tx*5+ty*3)%3 { 0=>0.48_f32, 1=>0.60, _=>0.72 };
                let scale_col: [f32;4] = match (tx*7+ty*5)%5 {
                    0 => [0.52,0.36,0.20,1.0], // 茶色
                    1 => [0.28,0.45,0.18,1.0], // 緑がかり
                    2 => [0.62,0.28,0.14,1.0], // 赤茶
                    3 => [0.38,0.36,0.42,1.0], // 灰
                    _ => [0.50,0.48,0.10,1.0], // 黄
                };
                let belly_col = [(scale_col[0]*1.35).min(0.9),(scale_col[1]*1.3).min(0.9),(scale_col[2]*1.2).min(0.85),1.0];
                let eye_col   = [0.96_f32,0.80,0.05,3.0]; // 黄色い目
                let spear_col = [0.55_f32,0.42,0.22,1.0];
                let blade_col = [0.72_f32,0.74,0.78,1.0];
                let bob   = (time*4.5+hash).sin().abs()*0.010*sc;
                let creep = (time*6.0+hash).sin()*0.012*sc; // こそこそした動き
                // 足 (小さく)
                let fc2=[scale_col[0]*0.78,scale_col[1]*0.78,scale_col[2]*0.78,1.0];
                push_box(verts,idxs,cx-0.10*sc,0.0,cz-0.055*sc,cx-0.04*sc,0.13*sc,cz+0.055*sc,fc2);
                push_box(verts,idxs,cx+0.04*sc,0.0,cz-0.055*sc,cx+0.10*sc,0.13*sc,cz+0.055*sc,fc2);
                // 爪 (つま先)
                push_box(verts,idxs,cx-0.115*sc,0.0,cz+0.040*sc,cx-0.040*sc,0.022*sc,cz+0.082*sc,[0.70,0.68,0.62,1.0]);
                push_box(verts,idxs,cx+0.040*sc,0.0,cz+0.040*sc,cx+0.115*sc,0.022*sc,cz+0.082*sc,[0.70,0.68,0.62,1.0]);
                // 胴体 (細め)
                let by0=0.12*sc+bob; let by1=by0+0.18*sc;
                push_box(verts,idxs,cx-0.095*sc,by0,cz-0.075*sc,cx+0.095*sc,by1,cz+0.075*sc,scale_col);
                push_box(verts,idxs,cx-0.065*sc,by0+0.008,cz+0.065*sc,cx+0.065*sc,by1-0.008,cz+0.080*sc,belly_col);
                // 尻尾 (後ろ)
                let ts=(time*3.0+hash).sin()*0.030*sc;
                push_box(verts,idxs,cx-0.030*sc,by0*0.5,cz-0.075*sc-0.10*sc,cx+0.030*sc,by0*0.75,cz-0.075*sc,scale_col);
                push_box(verts,idxs,cx-0.018*sc+ts,by0*0.25,cz-0.075*sc-0.22*sc,cx+0.018*sc+ts,by0*0.50,cz-0.075*sc-0.09*sc,scale_col);
                // 腕 (右手が槍持ち)
                let asp=creep*0.8;
                push_box(verts,idxs,cx-0.17*sc,by0+0.04*sc,cz-0.04*sc,cx-0.095*sc,by1-0.05*sc,cz+0.04*sc,scale_col);
                push_box(verts,idxs,cx+0.095*sc,by0+0.04*sc+asp,cz-0.04*sc,cx+0.17*sc,by1-0.05*sc+asp,cz+0.04*sc,scale_col);
                // 頭 (三角形っぽい爬虫類の顔)
                let hy0=by1-0.01*sc+bob; let hy1=hy0+0.14*sc;
                let hcz=cz+0.02*sc;
                push_box(verts,idxs,cx-0.075*sc,hy0,hcz-0.060*sc,cx+0.075*sc,hy1,hcz+0.055*sc,scale_col);
                // 口先が尖る
                push_box(verts,idxs,cx-0.040*sc,hy0,hcz+0.048*sc,cx+0.040*sc,hy0+0.055*sc,hcz+0.100*sc,scale_col);
                // 目 (黄色)
                let es=0.020*sc; let eyz=hy0+(hy1-hy0)*0.62; let ezf=hcz+0.048*sc;
                push_box(verts,idxs,cx-0.038*sc-es,eyz-es*0.7,ezf,cx-0.038*sc+es,eyz+es*0.7,ezf+es*0.42,eye_col);
                push_box(verts,idxs,cx+0.038*sc-es,eyz-es*0.7,ezf,cx+0.038*sc+es,eyz+es*0.7,ezf+es*0.42,eye_col);
                // 槍 (右手から上に)
                let spx=cx+0.20*sc; let spy0=by0+asp; let spy1=spy0+0.42*sc;
                push_box(verts,idxs,spx-0.012*sc,spy0,cz-0.012*sc,spx+0.012*sc,spy1,cz+0.012*sc,spear_col);
                push_box(verts,idxs,spx-0.018*sc,spy1-0.010*sc,cz-0.010*sc,spx+0.018*sc,spy1+0.058*sc,cz+0.010*sc,blade_col);
                if lights.len()<4 { lights.push(Light{pos:[cx,by1,cz,hash],col:[scale_col[0]*1.2,scale_col[1]*1.2,0.2,0.8*sc]}); }
            }

            // ━━ ニンフ (TILE_NYMPH): 妖精・ドレス・花冠・きらきら ━━
            if base_t == TILE_NYMPH {
                let cx = (x0+x1)*0.5; let cz = (z0+z1)*0.5;
                let hash = (tx*13+ty*11) as f32;
                let sc = tier_base * match (tx*5+ty*3)%2 { 0=>0.82_f32, _=>0.96 };
                // 色バリエーション (水辺・森・空)
                let (dress_col, skin_col, glow_col): ([f32;4],[f32;4],[f32;4]) = match (tx*7+ty*5)%4 {
                    0 => ([0.22,0.72,0.90,1.0],[0.92,0.80,0.72,1.0],[0.30,0.80,1.0,3.0]), // 水ニンフ
                    1 => ([0.22,0.72,0.28,1.0],[0.86,0.78,0.62,1.0],[0.30,1.0,0.40,3.0]), // 森ニンフ
                    2 => ([0.82,0.45,0.82,1.0],[0.94,0.84,0.72,1.0],[0.90,0.50,1.0,3.0]), // 魔法ニンフ
                    _ => ([0.92,0.88,0.32,1.0],[0.96,0.88,0.72,1.0],[1.0,0.95,0.30,3.0]), // 光ニンフ
                };
                let hair_col  = [dress_col[0]*0.72,dress_col[1]*0.72,dress_col[2]*0.72,1.0];
                let flower_col= [0.96_f32,0.60,0.72,3.0]; // 花 (光る)
                let eye_col   = [0.10_f32,0.12,0.50,3.0];
                let eye_sh    = [1.0_f32,1.0,0.96,4.0];
                let float_bob = (time*2.5+hash).sin()*0.025*sc; // ふわふわ浮く
                let sway = (time*1.8+hash).sin()*0.015*sc;
                // ドレス (下から広がる三角形っぽく)
                let dr_y0=0.05*sc+float_bob; let dr_y1=dr_y0+0.28*sc;
                push_box(verts,idxs,cx-0.14*sc,dr_y0,cz-0.14*sc,cx+0.14*sc,dr_y0+0.06*sc,cz+0.14*sc,dress_col);
                push_box(verts,idxs,cx-0.11*sc,dr_y0+0.04*sc,cz-0.11*sc,cx+0.11*sc,dr_y1,cz+0.11*sc,dress_col);
                push_box(verts,idxs,cx-0.09*sc,dr_y0+0.04*sc,cz-0.09*sc,cx+0.09*sc,dr_y1+0.005,cz+0.09*sc,
                    [dress_col[0]*1.15,dress_col[1]*1.15,dress_col[2]*1.15,1.0]); // ハイライト
                // 腕 (細くたなびく)
                push_box(verts,idxs,cx-0.19*sc,dr_y1-0.04*sc+sway,cz-0.025*sc,cx-0.09*sc,dr_y1+0.08*sc+sway,cz+0.025*sc,skin_col);
                push_box(verts,idxs,cx+0.09*sc,dr_y1-0.04*sc-sway,cz-0.025*sc,cx+0.19*sc,dr_y1+0.08*sc-sway,cz+0.025*sc,skin_col);
                // 首・顔
                let hy0=dr_y1+0.04*sc+float_bob; let hy1=hy0+0.18*sc;
                let hcz=cz+0.02*sc;
                push_box(verts,idxs,cx-0.018*sc,dr_y1-0.01*sc+float_bob,hcz-0.015*sc,cx+0.018*sc,hy0+0.01*sc,hcz+0.015*sc,skin_col);
                push_box(verts,idxs,cx-0.08*sc,hy0,hcz-0.072*sc,cx+0.08*sc,hy1,hcz+0.072*sc,skin_col);
                // 目 (大きく美しい)
                let es=0.024*sc; let eyz=hy0+(hy1-hy0)*0.55; let ezf=hcz+0.060*sc;
                push_box(verts,idxs,cx-0.040*sc-es,eyz-es,ezf,cx-0.040*sc+es,eyz+es,ezf+es*0.45,eye_col);
                push_box(verts,idxs,cx-0.040*sc+es*0.1,eyz+es*0.1,ezf+es*0.38,cx-0.040*sc+es*0.75,eyz+es*0.80,ezf+es*0.58,[1.0,1.0,0.95,4.0]);
                push_box(verts,idxs,cx+0.040*sc-es,eyz-es,ezf,cx+0.040*sc+es,eyz+es,ezf+es*0.45,eye_col);
                push_box(verts,idxs,cx+0.040*sc-es*0.75,eyz+es*0.1,ezf+es*0.38,cx+0.040*sc-es*0.1,eyz+es*0.80,ezf+es*0.58,eye_sh);
                // まつげ (小さな突起)
                push_box(verts,idxs,cx-0.064*sc,eyz+es*0.78,ezf,cx-0.040*sc-es*0.6,eyz+es*1.10,ezf+es*0.2,[0.10,0.08,0.12,1.0]);
                push_box(verts,idxs,cx+0.040*sc+es*0.6,eyz+es*0.78,ezf,cx+0.064*sc,eyz+es*1.10,ezf+es*0.2,[0.10,0.08,0.12,1.0]);
                // 小さな鼻
                push_box(verts,idxs,cx-0.010*sc,eyz-es*0.35,ezf+es*0.12,cx+0.010*sc,eyz,ezf+es*0.52,skin_col);
                // 唇 (ピンク)
                push_box(verts,idxs,cx-0.028*sc,hy0+0.018*sc,hcz+0.058*sc,cx+0.028*sc,hy0+0.042*sc,hcz+0.075*sc,[0.95,0.55,0.60,1.0]);
                // 髪 (ロングヘア 流れる)
                push_box(verts,idxs,cx-0.10*sc,hy0+0.04*sc,hcz-0.080*sc+sway,cx+0.10*sc,hy1+0.05*sc,hcz-0.085*sc+sway,hair_col);
                push_box(verts,idxs,cx-0.10*sc,hy0-0.12*sc,hcz-0.090*sc+sway*0.5,cx-0.08*sc,hy0+0.04*sc,hcz-0.075*sc+sway*0.5,hair_col);
                push_box(verts,idxs,cx+0.08*sc,hy0-0.12*sc,hcz-0.090*sc+sway*0.5,cx+0.10*sc,hy0+0.04*sc,hcz-0.075*sc+sway*0.5,hair_col);
                // 花冠 (頭に小さな花々)
                let fc_y=hy1-0.01*sc+float_bob;
                for i in 0..5i32 {
                    let angle = (i as f32)*1.257; // 72度刻み
                    let fx = cx + angle.sin()*0.062*sc;
                    let fz = hcz - 0.010*sc + angle.cos()*0.040*sc;
                    push_box(verts,idxs,fx-0.014*sc,fc_y,fz-0.014*sc,fx+0.014*sc,fc_y+0.022*sc,fz+0.014*sc,flower_col);
                }
                // きらきらパーティクル (3個)
                for i in 0..3i32 {
                    let sph = (time*3.5+hash+i as f32*2.1).sin();
                    let spv = (time*2.8+hash+i as f32*1.7).cos();
                    let sx = cx + sph*0.22*sc;
                    let sy = dr_y1 + spv.abs()*0.18*sc + float_bob;
                    let sz = cz + (time*2.2+hash+i as f32*1.5).cos()*0.18*sc;
                    push_box(verts,idxs,sx-0.012*sc,sy,sz-0.012*sc,sx+0.012*sc,sy+0.018*sc,sz+0.012*sc,glow_col);
                }
                if lights.len()<4 { lights.push(Light{pos:[cx,hy1+0.10*sc,hcz,hash],col:[glow_col[0],glow_col[1],glow_col[2],2.2*sc]}); }
            }

            // ━━ ヴァンパイア (TILE_VAMPIRE): 黒マント・鋭い牙・赤い目・霧 ━━
            if base_t == TILE_VAMPIRE {
                let cx = (x0+x1)*0.5; let cz = (z0+z1)*0.5;
                let hash = (tx*17+ty*13) as f32;
                let sc = tier_base * match (tx*3+ty*5)%2 { 0=>1.00_f32, _=>1.15 };
                let cape_col  = [0.06_f32,0.04,0.08,1.0]; // 黒マント
                let cape_in   = [0.52_f32,0.04,0.06,1.0]; // マント内側 (血の赤)
                let skin_col  = [0.82_f32,0.80,0.86,1.0]; // 死人肌 (青白い)
                let shirt_col = [0.88_f32,0.86,0.84,1.0]; // 白シャツ
                let hair_col  = [0.08_f32,0.06,0.10,1.0]; // 漆黒の髪
                let eye_col   = [0.96_f32,0.08,0.05,4.0]; // 燃える赤い目!
                let fang_col  = [0.96_f32,0.94,0.92,1.0];
                let bob  = (time*1.5+hash).sin()*0.012*sc; // ゆっくり浮く
                let cape_flap = (time*2.2+hash).sin()*0.04*sc;
                // 脚
                push_box(verts,idxs,cx-0.12*sc,0.0,cz-0.065*sc,cx-0.04*sc,0.22*sc,cz+0.065*sc,cape_col);
                push_box(verts,idxs,cx+0.04*sc,0.0,cz-0.065*sc,cx+0.12*sc,0.22*sc,cz+0.065*sc,cape_col);
                // 胴体
                let by0=0.20*sc+bob; let by1=by0+0.32*sc;
                push_box(verts,idxs,cx-0.13*sc,by0,cz-0.090*sc,cx+0.13*sc,by1,cz+0.090*sc,shirt_col);
                // タイ (細い赤)
                push_box(verts,idxs,cx-0.014*sc,by0+0.08*sc,cz+0.085*sc,cx+0.014*sc,by1-0.04*sc,cz+0.096*sc,[0.72,0.04,0.06,1.0]);
                // マント (後ろに大きく広がる — 翼のよう)
                let cw=0.28*sc+cape_flap.abs();
                push_box(verts,idxs,cx-cw,by0-0.02*sc,cz-0.10*sc,cx-0.12*sc,by1+0.08*sc,cz+0.08*sc,cape_col);
                push_box(verts,idxs,cx+0.12*sc,by0-0.02*sc,cz-0.10*sc,cx+cw,by1+0.08*sc,cz+0.08*sc,cape_col);
                // マント内側 (赤い裏地)
                push_box(verts,idxs,cx-cw+0.005,by0,cz-0.095*sc,cx-0.12*sc,by1+0.06*sc,cz+0.076*sc,cape_in);
                push_box(verts,idxs,cx+0.12*sc,by0,cz-0.095*sc,cx+cw-0.005,by1+0.06*sc,cz+0.076*sc,cape_in);
                // マント下部 (V字に広がる)
                push_box(verts,idxs,cx-cw*0.80,by0-0.18*sc,cz-0.12*sc+cape_flap,cx-0.06*sc,by0,cz+0.08*sc+cape_flap,cape_col);
                push_box(verts,idxs,cx+0.06*sc,by0-0.18*sc,cz-0.12*sc-cape_flap,cx+cw*0.80,by0,cz+0.08*sc-cape_flap,cape_col);
                // 腕 (マントから覗く)
                push_box(verts,idxs,cx-0.24*sc,by0+0.12*sc,cz-0.025*sc,cx-0.13*sc,by1-0.06*sc,cz+0.025*sc,shirt_col);
                push_box(verts,idxs,cx+0.13*sc,by0+0.12*sc,cz-0.025*sc,cx+0.24*sc,by1-0.06*sc,cz+0.025*sc,shirt_col);
                // 手 (爪)
                push_box(verts,idxs,cx-0.25*sc,by0+0.08*sc,cz-0.028*sc,cx-0.21*sc,by0+0.16*sc,cz+0.028*sc,skin_col);
                push_box(verts,idxs,cx+0.21*sc,by0+0.08*sc,cz-0.028*sc,cx+0.25*sc,by0+0.16*sc,cz+0.028*sc,skin_col);
                // 爪 (鋭い)
                let cl=[0.78_f32,0.76,0.82,1.0];
                push_box(verts,idxs,cx-0.265*sc,by0+0.04*sc,cz-0.006*sc,cx-0.248*sc,by0+0.09*sc,cz+0.006*sc,cl);
                push_box(verts,idxs,cx+0.248*sc,by0+0.04*sc,cz-0.006*sc,cx+0.265*sc,by0+0.09*sc,cz+0.006*sc,cl);
                // 頭
                let hy0=by1-0.02*sc+bob; let hy1=hy0+0.22*sc;
                let hcz=cz+0.015*sc;
                push_box(verts,idxs,cx-0.10*sc,hy0,hcz-0.085*sc,cx+0.10*sc,hy1,hcz+0.075*sc,skin_col);
                // 髪 (後ろに流れるダークヘア)
                push_box(verts,idxs,cx-0.105*sc,hy0+0.08*sc,hcz-0.090*sc,cx+0.105*sc,hy1+0.04*sc,hcz-0.092*sc,hair_col);
                push_box(verts,idxs,cx-0.108*sc,hy0-0.06*sc,hcz-0.088*sc,cx-0.088*sc,hy1,hcz,hair_col);
                push_box(verts,idxs,cx+0.088*sc,hy0-0.06*sc,hcz-0.088*sc,cx+0.108*sc,hy1,hcz,hair_col);
                // 燃える赤い目 (主な特徴!)
                let es=0.026*sc; let eyz=hy0+(hy1-hy0)*0.58; let ezf=hcz+0.068*sc;
                push_box(verts,idxs,cx-0.048*sc-es,eyz-es*0.65,ezf,cx-0.048*sc+es,eyz+es*0.65,ezf+es*0.40,eye_col);
                push_box(verts,idxs,cx+0.048*sc-es,eyz-es*0.65,ezf,cx+0.048*sc+es,eyz+es*0.65,ezf+es*0.40,eye_col);
                // 牙 (下に突き出る)
                push_box(verts,idxs,cx-0.040*sc,hy0+0.018*sc,hcz+0.060*sc,cx-0.018*sc,hy0+0.048*sc,hcz+0.078*sc,fang_col);
                push_box(verts,idxs,cx+0.018*sc,hy0+0.018*sc,hcz+0.060*sc,cx+0.040*sc,hy0+0.048*sc,hcz+0.078*sc,fang_col);
                push_box(verts,idxs,cx-0.035*sc,hy0-0.012*sc,hcz+0.062*sc,cx-0.020*sc,hy0+0.018*sc,hcz+0.080*sc,fang_col);
                push_box(verts,idxs,cx+0.020*sc,hy0-0.012*sc,hcz+0.062*sc,cx+0.035*sc,hy0+0.018*sc,hcz+0.080*sc,fang_col);
                // 霧 (足元にうっすら)
                let mist = (time*1.2+hash).sin()*0.5+0.5_f32;
                push_box(verts,idxs,cx-0.30*sc,0.0,cz-0.30*sc,cx+0.30*sc,0.028*sc,cz+0.30*sc,[0.40,0.18,0.42,mist*0.60+0.20]);
                if lights.len()<4 { lights.push(Light{pos:[cx,by1,cz,hash],col:[0.90,0.04,0.04,2.8*sc]}); }
            }

            // ━━ リッチ (TILE_LICH): 骨の魔道士・ローブ・魔法の杖・頭蓋骨 ━━
            if base_t == TILE_LICH {
                let cx = (x0+x1)*0.5; let cz = (z0+z1)*0.5;
                let hash = (tx*19+ty*11) as f32;
                let sc = tier_base * 1.0_f32;
                let robe_col  = [0.10_f32,0.06,0.14,1.0]; // 暗紫色のローブ
                let robe_trim = [0.42_f32,0.08,0.46,1.0]; // 縁取り (魔法紫)
                let bone_col  = [0.82_f32,0.80,0.72,1.0]; // 骨
                let eye_col   = [0.12_f32,0.88,0.96,4.0]; // 青緑の死の目!
                let orb_col   = [0.10_f32,0.82,0.90,4.0]; // 魔法の宝珠
                let staff_col = [0.22_f32,0.16,0.28,1.0];
                let float_y   = (time*1.4+hash).sin()*0.018*sc; // ふわりと浮く
                let staff_sw  = (time*2.0+hash).sin()*0.028*sc;
                let pulse     = (time*3.2+hash).sin()*0.5+0.5_f32;
                // ローブ (裾が床につく)
                push_box(verts,idxs,cx-0.15*sc,0.0,cz-0.12*sc,cx+0.15*sc,0.12*sc,cz+0.12*sc,robe_col);
                push_box(verts,idxs,cx-0.13*sc,0.08*sc,cz-0.10*sc,cx+0.13*sc,0.16*sc,cz+0.10*sc,robe_col);
                // 胴体ローブ
                let by0=0.14*sc+float_y; let by1=by0+0.36*sc;
                push_box(verts,idxs,cx-0.13*sc,by0,cz-0.09*sc,cx+0.13*sc,by1,cz+0.09*sc,robe_col);
                push_box(verts,idxs,cx-0.13*sc,by0,cz+0.080*sc,cx+0.13*sc,by1,cz+0.092*sc,robe_trim);
                // ローブの紋章 (発光)
                push_box(verts,idxs,cx-0.018*sc,by0+0.08*sc,cz+0.082*sc,cx+0.018*sc,by0+0.14*sc,cz+0.094*sc,orb_col);
                // 骨の腕
                push_box(verts,idxs,cx-0.24*sc,by0+0.10*sc,cz-0.018*sc,cx-0.13*sc,by1-0.08*sc,cz+0.018*sc,bone_col);
                push_box(verts,idxs,cx+0.13*sc,by0+0.10*sc+staff_sw,cz-0.018*sc,cx+0.24*sc,by1-0.08*sc+staff_sw,cz+0.018*sc,bone_col);
                // 骨の指 (左手)
                for i in 0..3i32 {
                    let fx=cx-0.22*sc+(i as f32)*0.014*sc;
                    push_box(verts,idxs,fx,by0+0.04*sc,cz-0.008*sc,fx+0.010*sc,by0+0.09*sc,cz+0.008*sc,bone_col);
                }
                // 頭蓋骨
                let hy0=by1-0.02*sc+float_y; let hy1=hy0+0.20*sc;
                let hcz=cz+0.008*sc;
                push_box(verts,idxs,cx-0.095*sc,hy0,hcz-0.080*sc,cx+0.095*sc,hy1,hcz+0.072*sc,bone_col);
                // 頭蓋骨の丸み (上が広い)
                push_box(verts,idxs,cx-0.090*sc,hy0+0.08*sc,hcz-0.085*sc,cx+0.090*sc,hy1+0.02*sc,hcz+0.075*sc,bone_col);
                // 顎
                push_box(verts,idxs,cx-0.075*sc,hy0-0.028*sc,hcz-0.038*sc,cx+0.075*sc,hy0,hcz+0.055*sc,bone_col);
                // 歯 (ガタガタ)
                for i in 0..4i32 {
                    let tx2=cx-0.058*sc+(i as f32)*0.030*sc;
                    push_box(verts,idxs,tx2,hy0-0.032*sc,hcz+0.045*sc,tx2+0.018*sc,hy0,hcz+0.062*sc,bone_col);
                }
                // 死の目 (青緑の炎)!
                let es=0.026*sc; let eyz=hy0+(hy1-hy0)*0.44; let ezf=hcz+0.060*sc;
                let eye_glow=[eye_col[0],eye_col[1],eye_col[2],pulse*2.0+2.0];
                push_box(verts,idxs,cx-0.042*sc-es*1.2,eyz-es*0.5,ezf,cx-0.042*sc+es*1.2,eyz+es*1.8,ezf+es*0.50,eye_glow);
                push_box(verts,idxs,cx+0.042*sc-es*1.2,eyz-es*0.5,ezf,cx+0.042*sc+es*1.2,eyz+es*1.8,ezf+es*0.50,eye_glow);
                // 魔法の杖 (右手から上に高く)
                let stx=cx+0.24*sc; let sty0=by0+staff_sw; let sty1=sty0+0.52*sc;
                push_box(verts,idxs,stx-0.014*sc,sty0,cz-0.014*sc,stx+0.014*sc,sty1,cz+0.014*sc,staff_col);
                // 杖先の宝珠 (脈動!)
                let orb_r=0.048*sc+pulse*0.012*sc;
                push_box(verts,idxs,stx-orb_r,sty1-orb_r*0.5,cz-orb_r,stx+orb_r,sty1+orb_r,cz+orb_r,[orb_col[0],orb_col[1],orb_col[2],pulse*2.0+2.5]);
                // 魔法の霧 (足元)
                let fog = (time*1.5+hash*0.2).sin()*0.5+0.5_f32;
                push_box(verts,idxs,cx-0.32*sc,0.0,cz-0.32*sc,cx+0.32*sc,0.022*sc,cz+0.32*sc,[0.12,0.80,0.92,fog*0.50+0.12]);
                if lights.len()<4 { lights.push(Light{pos:[stx,sty1,cz,hash],col:[0.10,0.85,0.95,pulse*3.0+1.5]}); }
            }

            // ━━ イエティ (TILE_YETI): 白い毛むくじゃら・大きな爪・雪煙 ━━
            if base_t == TILE_YETI {
                let cx = (x0+x1)*0.5; let cz = (z0+z1)*0.5;
                let hash = (tx*11+ty*13) as f32;
                let sc = tier_base * match (tx*5+ty*3)%3 { 0=>0.88_f32, 1=>1.10, _=>1.28 };
                let fur_col  = [0.94_f32,0.93,0.96,1.0]; // 白い毛
                let fur_sh   = [0.82_f32,0.84,0.92,1.0]; // 影側
                let skin_col = [0.76_f32,0.66,0.62,1.0]; // 顔
                let eye_col  = [0.08_f32,0.42,0.96,3.0]; // 青い目
                let claw_col = [0.82_f32,0.78,0.70,1.0]; // 爪
                let nose_col = [0.80_f32,0.42,0.38,1.0];
                let stomp  = (time*2.8+hash).sin();
                let bob    = stomp.abs()*0.018*sc;
                let arm_sw = (time*1.8+hash).sin()*0.025*sc;
                // 足 (太く毛深い)
                let lw=0.10*sc;
                push_box(verts,idxs,cx-0.17*sc,0.0,cz-0.08*sc,cx-0.04*sc,0.28*sc,cz+0.08*sc,fur_col);
                push_box(verts,idxs,cx+0.04*sc,0.0,cz-0.08*sc,cx+0.17*sc,0.28*sc,cz+0.08*sc,fur_sh);
                // 足の毛 (外側にふわっと)
                push_box(verts,idxs,cx-0.21*sc,0.0,cz-0.06*sc,cx-0.16*sc,0.22*sc,cz+0.06*sc,fur_col);
                push_box(verts,idxs,cx+0.16*sc,0.0,cz-0.06*sc,cx+0.21*sc,0.22*sc,cz+0.06*sc,fur_col);
                // 足爪
                for i in 0..3i32 {
                    let fz=cz+0.06*sc+(i as f32)*0.018*sc;
                    push_box(verts,idxs,cx-0.175*sc,0.0,fz,cx-0.10*sc,0.028*sc,fz+0.040*sc,claw_col);
                    push_box(verts,idxs,cx+0.10*sc,0.0,fz,cx+0.175*sc,0.028*sc,fz+0.040*sc,claw_col);
                }
                // 胴体 (大きくたる型)
                let by0=0.26*sc+bob; let by1=by0+0.38*sc;
                push_box(verts,idxs,cx-0.22*sc,by0,cz-0.16*sc,cx+0.22*sc,by1,cz+0.16*sc,fur_col);
                push_box(verts,idxs,cx-0.24*sc,by0+0.06*sc,cz-0.12*sc,cx-0.20*sc,by1-0.06*sc,cz+0.12*sc,fur_col);
                push_box(verts,idxs,cx+0.20*sc,by0+0.06*sc,cz-0.12*sc,cx+0.24*sc,by1-0.06*sc,cz+0.12*sc,fur_col);
                // 腕 (長くて太い)
                push_box(verts,idxs,cx-0.38*sc,by0+0.08*sc+arm_sw,cz-0.08*sc,cx-0.21*sc,by1-0.04*sc+arm_sw,cz+0.08*sc,fur_col);
                push_box(verts,idxs,cx+0.21*sc,by0+0.08*sc-arm_sw,cz-0.08*sc,cx+0.38*sc,by1-0.04*sc-arm_sw,cz+0.08*sc,fur_sh);
                // 手爪 (大きく怖い!)
                for i in 0..4i32 {
                    let cz2=cz-0.04*sc+(i as f32)*0.022*sc;
                    push_box(verts,idxs,cx-0.42*sc,by0+0.04*sc+arm_sw,cz2,cx-0.36*sc,by0+0.10*sc+arm_sw,cz2+0.060*sc,claw_col);
                    push_box(verts,idxs,cx+0.36*sc,by0+0.04*sc-arm_sw,cz2,cx+0.42*sc,by0+0.10*sc-arm_sw,cz2+0.060*sc,claw_col);
                }
                // 首 (ほぼない、頭が直接乗る)
                let hy0=by1-0.02*sc+bob; let hy1=hy0+0.24*sc;
                let hcz=cz+0.02*sc;
                // 頭 (丸く大きい)
                push_box(verts,idxs,cx-0.18*sc,hy0,hcz-0.14*sc,cx+0.18*sc,hy1,hcz+0.12*sc,fur_col);
                // 顔 (毛に囲まれた中央)
                push_box(verts,idxs,cx-0.12*sc,hy0+0.02*sc,hcz+0.08*sc,cx+0.12*sc,hy1-0.04*sc,hcz+0.125*sc,skin_col);
                // 眉毛 (太くて威圧的)
                push_box(verts,idxs,cx-0.12*sc,hy0+(hy1-hy0)*0.72,hcz+0.110*sc,cx+0.12*sc,hy0+(hy1-hy0)*0.82,hcz+0.128*sc,fur_sh);
                // 鼻
                push_box(verts,idxs,cx-0.032*sc,hy0+(hy1-hy0)*0.38,hcz+0.112*sc,cx+0.032*sc,hy0+(hy1-hy0)*0.56,hcz+0.132*sc,nose_col);
                // 目 (青い)
                let es=0.028*sc; let eyz=hy0+(hy1-hy0)*0.60; let ezf=hcz+0.110*sc;
                push_box(verts,idxs,cx-0.058*sc-es,eyz-es*0.7,ezf,cx-0.058*sc+es,eyz+es*0.7,ezf+es*0.4,eye_col);
                push_box(verts,idxs,cx+0.058*sc-es,eyz-es*0.7,ezf,cx+0.058*sc+es,eyz+es*0.7,ezf+es*0.4,eye_col);
                // 雪煙 (足元)
                let snow_drift=(time*2.0+hash*0.2).sin()*0.5+0.5_f32;
                push_box(verts,idxs,cx-0.30*sc,0.0,cz-0.30*sc,cx+0.30*sc,0.018*sc,cz+0.30*sc,[0.96,0.98,1.0,snow_drift*0.55+0.15]);
                if lights.len()<4 { lights.push(Light{pos:[cx,hy1,cz,hash],col:[0.6,0.75,1.0,1.8*sc]}); }
            }

            // ━━ 天使 (TILE_ANGEL): 白い羽・金の鎧・光輪・神聖な光 ━━
            if base_t == TILE_ANGEL {
                let cx = (x0+x1)*0.5; let cz = (z0+z1)*0.5;
                let hash = (tx*13+ty*17) as f32;
                let sc = tier_base * 1.0_f32;
                let armor_col = [0.96_f32,0.90,0.55,1.0]; // 金の鎧
                let armor_sh  = [0.80_f32,0.72,0.35,1.0];
                let wing_col  = [0.98_f32,0.97,0.96,1.0]; // 純白の羽
                let wing_sh   = [0.86_f32,0.88,0.94,1.0];
                let skin_col  = [0.96_f32,0.88,0.78,1.0];
                let hair_col  = [0.96_f32,0.92,0.60,1.0]; // 金髪
                let eye_col   = [0.26_f32,0.62,0.96,4.0]; // 神聖な青
                let halo_col  = [1.0_f32,0.96,0.40,4.0];  // 光輪 (強く光る!)
                let float_y   = (time*1.6+hash).sin()*0.028*sc + 0.05*sc;
                let wing_flap = (time*2.4+hash).sin();
                let pulse     = (time*3.0+hash).sin()*0.5+0.5_f32;
                // 脚 (金色のサンダル)
                push_box(verts,idxs,cx-0.10*sc,float_y,cz-0.06*sc,cx-0.03*sc,float_y+0.22*sc,cz+0.06*sc,armor_col);
                push_box(verts,idxs,cx+0.03*sc,float_y,cz-0.06*sc,cx+0.10*sc,float_y+0.22*sc,cz+0.06*sc,armor_sh);
                // サンダル
                push_box(verts,idxs,cx-0.12*sc,float_y,cz-0.08*sc,cx-0.02*sc,float_y+0.022*sc,cz+0.09*sc,armor_col);
                push_box(verts,idxs,cx+0.02*sc,float_y,cz-0.08*sc,cx+0.12*sc,float_y+0.022*sc,cz+0.09*sc,armor_sh);
                // 胴体 (金の胸鎧)
                let by0=float_y+0.20*sc; let by1=by0+0.34*sc;
                push_box(verts,idxs,cx-0.14*sc,by0,cz-0.10*sc,cx+0.14*sc,by1,cz+0.10*sc,armor_col);
                // 鎧の装飾 (縦線)
                push_box(verts,idxs,cx-0.006*sc,by0+0.02*sc,cz+0.092*sc,cx+0.006*sc,by1-0.02*sc,cz+0.102*sc,[1.0,1.0,0.55,3.0]);
                // ベルト
                push_box(verts,idxs,cx-0.14*sc,by0+0.08*sc,cz-0.092*sc,cx+0.14*sc,by0+0.11*sc,cz+0.102*sc,armor_sh);
                // 腕
                push_box(verts,idxs,cx-0.26*sc,by0+0.10*sc,cz-0.03*sc,cx-0.14*sc,by1-0.06*sc,cz+0.03*sc,armor_col);
                push_box(verts,idxs,cx+0.14*sc,by0+0.10*sc,cz-0.03*sc,cx+0.26*sc,by1-0.06*sc,cz+0.03*sc,armor_sh);
                // 手
                push_box(verts,idxs,cx-0.28*sc,by0+0.06*sc,cz-0.032*sc,cx-0.22*sc,by0+0.16*sc,cz+0.032*sc,skin_col);
                push_box(verts,idxs,cx+0.22*sc,by0+0.06*sc,cz-0.032*sc,cx+0.28*sc,by0+0.16*sc,cz+0.032*sc,skin_col);
                // 翼 (大きく広がる — 3段) 
                let wy=by0+0.18*sc; let wh=0.028*sc;
                let wflap_l=wing_flap*0.18*sc; let wflap_r=(-wing_flap)*0.18*sc;
                // 左翼 (3段)
                push_box(verts,idxs,cx-0.56*sc,wy+wflap_l*1.0,cz-0.04*sc,cx-0.14*sc,wy+wh+wflap_l*1.0,cz+0.04*sc,wing_col);
                push_box(verts,idxs,cx-0.50*sc,wy+wflap_l*0.7+0.10*sc,cz-0.03*sc,cx-0.13*sc,wy+wh+wflap_l*0.7+0.10*sc,cz+0.03*sc,wing_sh);
                push_box(verts,idxs,cx-0.42*sc,wy+wflap_l*0.4+0.20*sc,cz-0.022*sc,cx-0.12*sc,wy+wh+wflap_l*0.4+0.20*sc,cz+0.022*sc,wing_col);
                // 右翼 (3段)
                push_box(verts,idxs,cx+0.14*sc,wy+wflap_r*1.0,cz-0.04*sc,cx+0.56*sc,wy+wh+wflap_r*1.0,cz+0.04*sc,wing_col);
                push_box(verts,idxs,cx+0.13*sc,wy+wflap_r*0.7+0.10*sc,cz-0.03*sc,cx+0.50*sc,wy+wh+wflap_r*0.7+0.10*sc,cz+0.03*sc,wing_sh);
                push_box(verts,idxs,cx+0.12*sc,wy+wflap_r*0.4+0.20*sc,cz-0.022*sc,cx+0.42*sc,wy+wh+wflap_r*0.4+0.20*sc,cz+0.022*sc,wing_col);
                // 頭
                let hy0=by1-0.01*sc+float_y*0.5; let hy1=hy0+0.20*sc;
                let hcz=cz+0.01*sc;
                push_box(verts,idxs,cx-0.095*sc,hy0,hcz-0.082*sc,cx+0.095*sc,hy1,hcz+0.075*sc,skin_col);
                // 金髪 (後ろに流れる)
                push_box(verts,idxs,cx-0.10*sc,hy0+0.06*sc,hcz-0.085*sc,cx+0.10*sc,hy1+0.06*sc,hcz-0.088*sc,hair_col);
                // 神聖な青い目
                let es=0.024*sc; let eyz=hy0+(hy1-hy0)*0.58; let ezf=hcz+0.065*sc;
                push_box(verts,idxs,cx-0.044*sc-es,eyz-es*0.7,ezf,cx-0.044*sc+es,eyz+es*0.7,ezf+es*0.45,[eye_col[0],eye_col[1],eye_col[2],pulse*1.5+2.5]);
                push_box(verts,idxs,cx+0.044*sc-es,eyz-es*0.7,ezf,cx+0.044*sc+es,eyz+es*0.7,ezf+es*0.45,[eye_col[0],eye_col[1],eye_col[2],pulse*1.5+2.5]);
                // 光輪 (頭の上で回転・脈動!)
                let halo_y=hy1+0.055*sc;
                let hr=0.14*sc+pulse*0.018*sc;
                push_box(verts,idxs,cx-hr,halo_y-0.012*sc,hcz-hr,cx+hr,halo_y+0.012*sc,hcz+hr,[halo_col[0],halo_col[1],halo_col[2],pulse*2.0+2.5]);
                let ir=hr*0.62;
                push_box(verts,idxs,cx-ir,halo_y-0.008*sc,hcz-ir,cx+ir,halo_y+0.008*sc,hcz+ir,[0.96,0.92,0.96,0.0]); // 中抜き (暗い)
                // 神聖な光 (足元に降り注ぐ)
                push_box(verts,idxs,cx-0.20*sc,float_y,cz-0.20*sc,cx+0.20*sc,float_y+0.012*sc,cz+0.20*sc,[1.0,0.98,0.80,pulse*1.5+0.5]);
                if lights.len()<4 { lights.push(Light{pos:[cx,halo_y,hcz,hash],col:[1.0,0.95,0.40,pulse*3.5+2.0]}); }
            }

            // ━━ ケンタウルス (TILE_CENTAUR): 馬体+人間上半身・弓矢 ━━
            if base_t == TILE_CENTAUR {
                let cx = (x0+x1)*0.5; let cz = (z0+z1)*0.5;
                let hash = (tx*9+ty*17) as f32;
                let sc = tier_base * match (tx*5+ty*3)%2 { 0=>0.90_f32, _=>1.10 };
                let horse_col: [f32;4] = match (tx*7+ty*5)%4 {
                    0 => [0.62,0.42,0.22,1.0], // 栗毛
                    1 => [0.88,0.82,0.70,1.0], // 白馬
                    2 => [0.18,0.14,0.10,1.0], // 黒馬
                    _ => [0.50,0.38,0.22,1.0], // 鹿毛
                };
                let skin_col = [0.90_f32,0.75,0.60,1.0];
                let mane_col = [horse_col[0]*0.72,horse_col[1]*0.68,horse_col[2]*0.60,1.0];
                let hoof_col = [0.28_f32,0.24,0.20,1.0];
                let bow_col  = [0.55_f32,0.40,0.22,1.0];
                let arrow_col= [0.68_f32,0.56,0.32,1.0];
                let eye_col  = [0.08_f32,0.06,0.06,3.0];
                let walk = (time*3.0+hash*0.4).sin();
                let base_y=(time*6.0+hash).sin().abs()*0.014*sc;
                // 馬体 (大きく)
                let hby0=0.20*sc+base_y; let hby1=hby0+0.28*sc;
                let bfr=0.32*sc; let bbk=0.28*sc; let bw=0.14*sc;
                push_box(verts,idxs,cx-bw,hby0,cz-bbk,cx+bw,hby1,cz+bfr,horse_col);
                // 馬の首 (後方向いた下向き)
                push_box(verts,idxs,cx-0.08*sc,hby0+0.04*sc,cz-bbk-0.12*sc,cx+0.08*sc,hby1-0.04*sc,cz-bbk,horse_col);
                // 馬の頭
                push_box(verts,idxs,cx-0.06*sc,hby0-0.08*sc,cz-bbk-0.20*sc,cx+0.06*sc,hby0+0.14*sc,cz-bbk-0.11*sc,horse_col);
                // たてがみ
                push_box(verts,idxs,cx-0.005*sc,hby0+0.02*sc,cz-bbk-0.20*sc,cx+0.005*sc,hby1-0.02*sc,cz-bbk-0.095*sc,mane_col);
                // 4本脚 (馬脚)
                let lw=0.06*sc; let lh=hby0;
                let lfy=lh*(1.0+walk*0.08); let lby=lh*(1.0-walk*0.08);
                let leg_col=[horse_col[0]*0.85,horse_col[1]*0.85,horse_col[2]*0.85,1.0];
                push_box(verts,idxs,cx-bw+lw*0.2,0.0,cz+bfr-lw*2.0,cx-bw+lw*2.2,lfy,cz+bfr,leg_col);
                push_box(verts,idxs,cx+bw-lw*2.2,0.0,cz+bfr-lw*2.0,cx+bw-lw*0.2,lby,cz+bfr,leg_col);
                push_box(verts,idxs,cx-bw+lw*0.2,0.0,cz-bbk,cx-bw+lw*2.2,lby,cz-bbk+lw*2.0,leg_col);
                push_box(verts,idxs,cx+bw-lw*2.2,0.0,cz-bbk,cx+bw-lw*0.2,lfy,cz-bbk+lw*2.0,leg_col);
                // 蹄
                for (lx0,lx1,lz0,lz1) in [(cx-bw+lw*0.2,cx-bw+lw*2.2,cz+bfr-lw*2.0,cz+bfr),(cx+bw-lw*2.2,cx+bw-lw*0.2,cz+bfr-lw*2.0,cz+bfr),(cx-bw+lw*0.2,cx-bw+lw*2.2,cz-bbk,cz-bbk+lw*2.0),(cx+bw-lw*2.2,cx+bw-lw*0.2,cz-bbk,cz-bbk+lw*2.0)] {
                    push_box(verts,idxs,lx0,0.0,lz0,lx1,0.022*sc,lz1,hoof_col);
                }
                // 尻尾
                let tw2=(time*2.4+hash).sin()*0.05*sc;
                push_box(verts,idxs,cx+tw2-0.020*sc,hby0*0.5,cz-bbk-0.02*sc,cx+tw2+0.020*sc,hby1*0.55,cz-bbk-0.20*sc,mane_col);
                // 人間の上半身 (馬体の前上から生える)
                let uby0=hby1-0.04*sc+base_y; let uby1=uby0+0.32*sc;
                push_box(verts,idxs,cx-0.13*sc,uby0,cz+bfr*0.2,cx+0.13*sc,uby1,cz+bfr*0.8,skin_col);
                // 腕 (弓を構える)
                let draw=(time*1.5+hash).sin()*0.018*sc;
                push_box(verts,idxs,cx-0.24*sc,uby0+0.12*sc,cz+bfr*0.1-draw,cx-0.13*sc,uby1-0.06*sc,cz+bfr*0.7-draw,skin_col);
                push_box(verts,idxs,cx+0.13*sc,uby0+0.12*sc,cz+bfr*0.1+draw,cx+0.24*sc,uby1-0.06*sc,cz+bfr*0.7+draw,skin_col);
                // 頭
                let chy0=uby1-0.01*sc+base_y; let chy1=chy0+0.18*sc;
                let chcz=cz+bfr*0.45;
                push_box(verts,idxs,cx-0.085*sc,chy0,chcz-0.075*sc,cx+0.085*sc,chy1,chcz+0.068*sc,skin_col);
                // 目
                let ces=0.022*sc; let ceyz=chy0+(chy1-chy0)*0.58; let cezf=chcz+0.056*sc;
                push_box(verts,idxs,cx-0.040*sc-ces,ceyz-ces*0.7,cezf,cx-0.040*sc+ces,ceyz+ces*0.7,cezf+ces*0.42,eye_col);
                push_box(verts,idxs,cx+0.040*sc-ces,ceyz-ces*0.7,cezf,cx+0.040*sc+ces,ceyz+ces*0.7,cezf+ces*0.42,eye_col);
                // 弓
                let bow_x=cx+0.28*sc;
                push_box(verts,idxs,bow_x-0.014*sc,uby0+0.05*sc,chcz-0.020*sc,bow_x+0.014*sc,uby1+0.10*sc,chcz+0.020*sc,bow_col);
                // 矢 (弦に番えた矢)
                push_box(verts,idxs,bow_x-0.005*sc,uby0+0.18*sc,chcz,bow_x+0.005*sc,uby0+0.18*sc+0.28*sc,chcz+0.005*sc,arrow_col);
                if lights.len()<4 { lights.push(Light{pos:[cx,chy1,chcz,hash],col:[horse_col[0]*1.3,horse_col[1]*1.2,0.3,1.4*sc]}); }
            }

            // ━━ 巨人 (TILE_GIANT): 超巨大ヒューマノイド・岩の棍棒・大地揺れ ━━
            if base_t == TILE_GIANT {
                let cx = (x0+x1)*0.5; let cz = (z0+z1)*0.5;
                let hash = (tx*11+ty*7) as f32;
                let sc = tier_base * match (tx*3+ty*5)%3 { 0=>1.10_f32, 1=>1.30, _=>1.50 };
                let skin_col: [f32;4] = match (tx*7+ty*5)%3 {
                    0 => [0.68,0.60,0.52,1.0], // 岩肌
                    1 => [0.55,0.62,0.48,1.0], // 苔色
                    _ => [0.72,0.55,0.45,1.0], // 赤茶
                };
                let cloth_col = [0.30_f32,0.22,0.14,1.0]; // ぼろ布
                let rock_col  = [0.50_f32,0.48,0.44,1.0]; // 岩の棍棒
                let eye_col   = [0.88_f32,0.60,0.08,3.0]; // オレンジの目
                let stomp = (time*1.8+hash).sin();
                let bob   = stomp.abs()*0.025*sc;
                let swing = (time*1.2+hash).sin()*0.035*sc;
                // 脚 (超太い柱のよう)
                push_box(verts,idxs,cx-0.22*sc,0.0,cz-0.12*sc,cx-0.04*sc,0.38*sc,cz+0.12*sc,skin_col);
                push_box(verts,idxs,cx+0.04*sc,0.0,cz-0.12*sc,cx+0.22*sc,0.38*sc,cz+0.12*sc,[skin_col[0]*0.88,skin_col[1]*0.88,skin_col[2]*0.88,1.0]);
                // 布 (腰巻き)
                push_box(verts,idxs,cx-0.24*sc,0.28*sc,cz-0.14*sc,cx+0.24*sc,0.42*sc,cz+0.14*sc,cloth_col);
                // 胴体 (巨大な樽)
                let by0=0.36*sc+bob; let by1=by0+0.52*sc;
                push_box(verts,idxs,cx-0.26*sc,by0,cz-0.18*sc,cx+0.26*sc,by1,cz+0.18*sc,skin_col);
                push_box(verts,idxs,cx-0.28*sc,by0+0.10*sc,cz-0.14*sc,cx-0.24*sc,by1-0.10*sc,cz+0.14*sc,skin_col);
                push_box(verts,idxs,cx+0.24*sc,by0+0.10*sc,cz-0.14*sc,cx+0.28*sc,by1-0.10*sc,cz+0.14*sc,skin_col);
                // 腕 (超太い)
                push_box(verts,idxs,cx-0.50*sc,by0+0.14*sc+swing,cz-0.10*sc,cx-0.25*sc,by1-0.08*sc+swing,cz+0.10*sc,skin_col);
                push_box(verts,idxs,cx+0.25*sc,by0+0.14*sc-swing,cz-0.10*sc,cx+0.50*sc,by1-0.08*sc-swing,cz+0.10*sc,[skin_col[0]*0.85,skin_col[1]*0.85,skin_col[2]*0.85,1.0]);
                // 頭 (小さく見える、首ほぼなし)
                let hy0=by1-0.02*sc+bob; let hy1=hy0+0.28*sc;
                let hcz=cz+0.02*sc;
                push_box(verts,idxs,cx-0.20*sc,hy0,hcz-0.16*sc,cx+0.20*sc,hy1,hcz+0.14*sc,skin_col);
                // 太い眉
                push_box(verts,idxs,cx-0.18*sc,hy0+(hy1-hy0)*0.68,hcz+0.11*sc,cx+0.18*sc,hy0+(hy1-hy0)*0.80,hcz+0.145*sc,[skin_col[0]*0.70,skin_col[1]*0.68,skin_col[2]*0.65,1.0]);
                // 目
                let es=0.034*sc; let eyz=hy0+(hy1-hy0)*0.54; let ezf=hcz+0.118*sc;
                push_box(verts,idxs,cx-0.068*sc-es,eyz-es*0.65,ezf,cx-0.068*sc+es,eyz+es*0.65,ezf+es*0.38,eye_col);
                push_box(verts,idxs,cx+0.068*sc-es,eyz-es*0.65,ezf,cx+0.068*sc+es,eyz+es*0.65,ezf+es*0.38,eye_col);
                // 鼻 (でかい)
                push_box(verts,idxs,cx-0.042*sc,hy0+(hy1-hy0)*0.28,hcz+0.112*sc,cx+0.042*sc,hy0+(hy1-hy0)*0.50,hcz+0.150*sc,skin_col);
                // 口 (歯)
                push_box(verts,idxs,cx-0.085*sc,hy0+0.025*sc,hcz+0.112*sc,cx+0.085*sc,hy0+0.065*sc,hcz+0.148*sc,[0.82,0.80,0.72,1.0]);
                // 岩の棍棒 (右手から大きく上に)
                let clx=cx+0.52*sc; let cly0=by0+0.10*sc-swing; let cly1=cly0+0.68*sc;
                push_box(verts,idxs,clx-0.040*sc,cly0,cz-0.040*sc,clx+0.040*sc,cly1,cz+0.040*sc,rock_col);
                // 棍棒の頭 (岩の塊!)
                push_box(verts,idxs,clx-0.110*sc,cly1-0.02*sc,cz-0.110*sc,clx+0.110*sc,cly1+0.20*sc,cz+0.110*sc,rock_col);
                push_box(verts,idxs,clx-0.080*sc,cly1+0.18*sc,cz-0.080*sc,clx+0.080*sc,cly1+0.26*sc,cz+0.080*sc,[rock_col[0]*0.78,rock_col[1]*0.78,rock_col[2]*0.80,1.0]);
                // 地揺れエフェクト (足元の土煙)
                let dust=(time*2.2+hash*0.2).sin().abs()*0.5+0.3_f32;
                push_box(verts,idxs,cx-0.38*sc,0.0,cz-0.38*sc,cx+0.38*sc,0.015*sc,cz+0.38*sc,[0.62,0.52,0.36,dust*0.40+0.12]);
                if lights.len()<4 { lights.push(Light{pos:[cx,hy1,hcz,hash],col:[skin_col[0]*1.3,0.55,0.12,2.2*sc]}); }
            }

            // ━━ レイス (TILE_WRAITH): 半透明の亡霊・死神のような・極寒 ━━
            if base_t == TILE_WRAITH {
                let cx = (x0+x1)*0.5; let cz = (z0+z1)*0.5;
                let hash = (tx*17+ty*9) as f32;
                let sc = tier_base * 1.0_f32;
                let ghost_col  = [0.60_f32,0.72,0.84,1.0]; // 青みがかった霊体
                let dark_col   = [0.04_f32,0.04,0.08,1.0]; // 中心の暗闇
                let eye_col    = [0.10_f32,0.92,0.88,4.0]; // 青緑の目 (底冷え!)
                let shroud_col = [0.35_f32,0.45,0.58,1.0]; // 死装束
                let scythe_col = [0.68_f32,0.70,0.72,1.0]; // 鎌の刃
                let float_y    = (time*1.6+hash).sin()*0.038*sc + 0.12*sc;
                let sway       = (time*1.2+hash).sin()*0.022*sc;
                let pulse      = (time*2.8+hash).sin()*0.5+0.5_f32;
                let trail      = (time*3.5+hash).sin()*0.5+0.5_f32;
                // 霊体の下部 (霧のように消える)
                push_box(verts,idxs,cx-0.10*sc,float_y-0.18*sc,cz-0.10*sc,cx+0.10*sc,float_y-0.02*sc,cz+0.10*sc,[ghost_col[0],ghost_col[1],ghost_col[2],trail*0.5+0.2]);
                push_box(verts,idxs,cx-0.08*sc,float_y-0.32*sc,cz-0.08*sc,cx+0.08*sc,float_y-0.18*sc,cz+0.08*sc,[ghost_col[0],ghost_col[1],ghost_col[2],trail*0.3+0.1]);
                // 死装束 (ローブ状)
                push_box(verts,idxs,cx-0.15*sc+sway,float_y,cz-0.10*sc,cx+0.15*sc+sway,float_y+0.40*sc,cz+0.10*sc,shroud_col);
                push_box(verts,idxs,cx-0.16*sc+sway,float_y+0.04*sc,cz-0.086*sc,cx-0.13*sc+sway,float_y+0.38*sc,cz+0.086*sc,shroud_col);
                push_box(verts,idxs,cx+0.13*sc+sway,float_y+0.04*sc,cz-0.086*sc,cx+0.16*sc+sway,float_y+0.38*sc,cz+0.086*sc,shroud_col);
                // 骨の腕 (左右に伸びる)
                push_box(verts,idxs,cx-0.34*sc+sway,float_y+0.18*sc,cz-0.016*sc,cx-0.15*sc+sway,float_y+0.30*sc,cz+0.016*sc,[ghost_col[0],ghost_col[1],ghost_col[2],0.8]);
                push_box(verts,idxs,cx+0.15*sc+sway,float_y+0.18*sc,cz-0.016*sc,cx+0.34*sc+sway,float_y+0.30*sc,cz+0.016*sc,[ghost_col[0],ghost_col[1],ghost_col[2],0.7]);
                // 頭部 (暗いフード)
                let hy0=float_y+0.38*sc+sway*0.5; let hy1=hy0+0.22*sc;
                let hcz=cz+0.008*sc;
                push_box(verts,idxs,cx-0.12*sc,hy0,hcz-0.10*sc,cx+0.12*sc,hy1,hcz+0.08*sc,shroud_col);
                // フードの中は暗闇
                push_box(verts,idxs,cx-0.08*sc,hy0+0.02*sc,hcz-0.052*sc,cx+0.08*sc,hy1-0.02*sc,hcz+0.062*sc,dark_col);
                // 底冷えの目
                let es=0.025*sc; let eyz=hy0+(hy1-hy0)*0.42; let ezf=hcz+0.040*sc;
                let eg=[eye_col[0],eye_col[1],eye_col[2],pulse*2.0+2.5];
                push_box(verts,idxs,cx-0.036*sc-es,eyz-es*0.5,ezf,cx-0.036*sc+es,eyz+es*0.8,ezf+es*0.38,eg);
                push_box(verts,idxs,cx+0.036*sc-es,eyz-es*0.5,ezf,cx+0.036*sc+es,eyz+es*0.8,ezf+es*0.38,eg);
                // 死神の大鎌!
                let sx=cx+0.28*sc+sway; let sy0=float_y; let sy1=sy0+0.70*sc;
                push_box(verts,idxs,sx-0.012*sc,sy0,cz-0.012*sc,sx+0.012*sc,sy1,cz+0.012*sc,shroud_col);
                // 刃 (湾曲した形を2ボックスで)
                push_box(verts,idxs,sx-0.26*sc,sy1-0.01*sc,cz-0.018*sc,sx+0.010*sc,sy1+0.06*sc,cz+0.018*sc,scythe_col);
                push_box(verts,idxs,sx-0.22*sc,sy1+0.04*sc,cz-0.014*sc,sx-0.06*sc,sy1+0.13*sc,cz+0.014*sc,scythe_col);
                // 霊のトレイル (後方に流れる)
                for i in 0..3i32 {
                    let tz=cz-0.12*sc-(i as f32)*0.08*sc;
                    let ta=(trail*(1.0-(i as f32)*0.3)).max(0.0)*0.35;
                    push_box(verts,idxs,cx-0.08*sc+sway,float_y+0.05*sc,tz-0.04*sc,cx+0.08*sc+sway,float_y+0.30*sc,tz+0.04*sc,[ghost_col[0],ghost_col[1],ghost_col[2],ta]);
                }
                // 極寒エフェクト (足元の氷霧)
                push_box(verts,idxs,cx-0.28*sc,0.0,cz-0.28*sc,cx+0.28*sc,0.020*sc,cz+0.28*sc,[0.70,0.82,0.96,pulse*0.45+0.15]);
                if lights.len()<4 { lights.push(Light{pos:[cx+sway,hy1,hcz,hash],col:[0.10,0.85,0.95,pulse*3.0+1.5]}); }
            }

            // ━━ デーモン (TILE_DEMON): 赤黒い悪魔・コウモリの翼・角・尾・tier3 ━━
            if base_t == TILE_DEMON {
                let cx = (x0+x1)*0.5; let cz = (z0+z1)*0.5;
                let hash = (tx*17+ty*11) as f32;
                let sc = tier_base * 1.0_f32;
                let body_col  = [0.52_f32, 0.06, 0.06, 1.0]; // 深紅
                let body_sh   = [0.32_f32, 0.04, 0.04, 1.0]; // 影側
                let wing_col  = [0.22_f32, 0.04, 0.08, 1.0]; // 黒翼
                let wing_sh   = [0.14_f32, 0.02, 0.06, 1.0];
                let horn_col  = [0.14_f32, 0.08, 0.06, 1.0]; // 黒角
                let eye_col   = [1.0_f32, 0.30, 0.04, 4.5]; // 燃える赤目
                let tail_col  = [0.42_f32, 0.04, 0.04, 1.0];
                let pulse  = (time*3.2+hash).sin()*0.5+0.5_f32;
                let stomp  = (time*2.2+hash).sin();
                let bob    = stomp.abs()*0.016*sc;
                let arm_sw = (time*2.0+hash).sin()*0.028*sc;
                let wing_fl= (time*3.0+hash).sin();
                // 脚
                push_box(verts,idxs, cx-0.12*sc,0.0,cz-0.07*sc, cx-0.04*sc,0.26*sc,cz+0.07*sc,body_col);
                push_box(verts,idxs, cx+0.04*sc,0.0,cz-0.07*sc, cx+0.12*sc,0.26*sc,cz+0.07*sc,body_sh);
                // 爪 (足)
                for i in 0..3i32 {
                    let fz=cz+0.05*sc+(i as f32)*0.018*sc;
                    push_box(verts,idxs, cx-0.14*sc,0.0,fz, cx-0.03*sc,0.022*sc,fz+0.038*sc,horn_col);
                    push_box(verts,idxs, cx+0.03*sc,0.0,fz, cx+0.14*sc,0.022*sc,fz+0.038*sc,horn_col);
                }
                // 胴体
                let by0=0.24*sc+bob; let by1=by0+0.34*sc;
                push_box(verts,idxs, cx-0.18*sc,by0,cz-0.14*sc, cx+0.18*sc,by1,cz+0.14*sc,body_col);
                push_box(verts,idxs, cx-0.20*sc,by0+0.04*sc,cz-0.10*sc, cx-0.16*sc,by1-0.04*sc,cz+0.10*sc,body_sh);
                push_box(verts,idxs, cx+0.16*sc,by0+0.04*sc,cz-0.10*sc, cx+0.20*sc,by1-0.04*sc,cz+0.10*sc,body_sh);
                // 腕
                push_box(verts,idxs, cx-0.32*sc,by0+0.06*sc+arm_sw,cz-0.07*sc, cx-0.18*sc,by1-0.06*sc+arm_sw,cz+0.07*sc,body_col);
                push_box(verts,idxs, cx+0.18*sc,by0+0.06*sc-arm_sw,cz-0.07*sc, cx+0.32*sc,by1-0.06*sc-arm_sw,cz+0.07*sc,body_sh);
                // 手の爪
                for i in 0..3i32 {
                    let cz2=cz-0.03*sc+(i as f32)*0.022*sc;
                    push_box(verts,idxs, cx-0.36*sc,by0+0.02*sc+arm_sw,cz2, cx-0.30*sc,by0+0.10*sc+arm_sw,cz2+0.048*sc,horn_col);
                    push_box(verts,idxs, cx+0.30*sc,by0+0.02*sc-arm_sw,cz2, cx+0.36*sc,by0+0.10*sc-arm_sw,cz2+0.048*sc,horn_col);
                }
                // 首
                let hy0=by1-0.02*sc+bob; let hy1=hy0+0.20*sc;
                push_box(verts,idxs, cx-0.06*sc,by1-0.02*sc,cz-0.06*sc, cx+0.06*sc,hy0,cz+0.06*sc,body_col);
                // 頭
                push_box(verts,idxs, cx-0.14*sc,hy0,cz-0.12*sc, cx+0.14*sc,hy1,cz+0.12*sc,body_col);
                // 目 (燃える!)
                let es=0.030*sc; let eyz=hy0+(hy1-hy0)*0.54; let ezf=cz+0.10*sc;
                let eg=[eye_col[0],eye_col[1],eye_col[2],pulse*2.5+2.0];
                push_box(verts,idxs, cx-0.06*sc-es,eyz-es*0.6,ezf, cx-0.06*sc+es,eyz+es*0.7,ezf+es*0.5,eg);
                push_box(verts,idxs, cx+0.06*sc-es,eyz-es*0.6,ezf, cx+0.06*sc+es,eyz+es*0.7,ezf+es*0.5,eg);
                // 角 (左右に曲がる2本)
                let htop=hy1-0.02*sc;
                push_box(verts,idxs, cx-0.10*sc,htop,cz-0.04*sc, cx-0.04*sc,htop+0.12*sc,cz+0.04*sc,horn_col);
                push_box(verts,idxs, cx-0.12*sc,htop+0.10*sc,cz-0.03*sc, cx-0.08*sc,htop+0.18*sc,cz+0.03*sc,horn_col);
                push_box(verts,idxs, cx+0.04*sc,htop,cz-0.04*sc, cx+0.10*sc,htop+0.12*sc,cz+0.04*sc,horn_col);
                push_box(verts,idxs, cx+0.08*sc,htop+0.10*sc,cz-0.03*sc, cx+0.12*sc,htop+0.18*sc,cz+0.03*sc,horn_col);
                // コウモリ翼 (大きく広がる!)
                let wspan = 0.55*sc + wing_fl.abs()*0.10*sc;
                let wh    = 0.22*sc + wing_fl.abs()*0.06*sc;
                let wy0   = by0+0.08*sc; let wy1=wy0+wh;
                // 左翼 (3セグメント)
                push_box(verts,idxs, cx-0.20*sc,wy0,cz-0.05*sc, cx-0.20*sc-wspan*0.5,wy1,cz+0.05*sc,wing_col);
                push_box(verts,idxs, cx-0.20*sc-wspan*0.5,wy0+0.02*sc,cz-0.04*sc, cx-0.20*sc-wspan*0.9,wy0+wh*0.5,cz+0.04*sc,wing_sh);
                push_box(verts,idxs, cx-0.20*sc-wspan*0.6,wy0,cz-0.03*sc, cx-0.20*sc-wspan,wy0+0.04*sc,cz+0.03*sc,wing_sh);
                // 右翼
                push_box(verts,idxs, cx+0.20*sc,wy0,cz-0.05*sc, cx+0.20*sc+wspan*0.5,wy1,cz+0.05*sc,wing_col);
                push_box(verts,idxs, cx+0.20*sc+wspan*0.5,wy0+0.02*sc,cz-0.04*sc, cx+0.20*sc+wspan*0.9,wy0+wh*0.5,cz+0.04*sc,wing_sh);
                push_box(verts,idxs, cx+0.20*sc+wspan*0.6,wy0,cz-0.03*sc, cx+0.20*sc+wspan,wy0+0.04*sc,cz+0.03*sc,wing_sh);
                // 尾 (後ろにとがった)
                push_box(verts,idxs, cx-0.04*sc,by0+0.04*sc,cz-0.14*sc, cx+0.04*sc,by0+0.12*sc,cz-0.22*sc,tail_col);
                push_box(verts,idxs, cx-0.024*sc,by0+0.02*sc,cz-0.20*sc, cx+0.024*sc,by0+0.08*sc,cz-0.30*sc,tail_col);
                push_box(verts,idxs, cx-0.016*sc,by0,cz-0.28*sc, cx+0.016*sc,by0+0.04*sc,cz-0.36*sc,horn_col);
                // 炎のオーラ (足元)
                let fire = pulse*0.4+0.3_f32;
                push_box(verts,idxs, cx-0.26*sc,0.0,cz-0.26*sc, cx+0.26*sc,0.016*sc,cz+0.26*sc,[0.96,0.24,0.04,fire]);
                if lights.len()<4 { lights.push(Light{pos:[cx,hy1,cz,hash],col:[1.0,0.22,0.04,pulse*4.0+2.0]}); }
            }

            // ━━ ミイラ (TILE_MUMMY): 包帯まみれのアンデッド・呪われた腐敗色 ━━
            if base_t == TILE_MUMMY {
                let cx = (x0+x1)*0.5; let cz = (z0+z1)*0.5;
                let hash = (tx*7+ty*19) as f32;
                let sc = tier_base * match (tx*3+ty*7)%3 { 0=>0.90_f32, 1=>1.0, _=>1.08 };
                let wrap_col  = [0.86_f32, 0.82, 0.72, 1.0]; // 包帯 (古びた白)
                let wrap_sh   = [0.70_f32, 0.64, 0.52, 1.0]; // 影
                let skin_col  = [0.54_f32, 0.50, 0.38, 1.0]; // 腐敗した肌
                let eye_col   = [0.60_f32, 0.96, 0.12, 3.5]; // 黄緑の呪い目
                let rotted    = [0.42_f32, 0.36, 0.24, 1.0]; // 腐敗部分
                let bob  = (time*1.4+hash).sin().abs()*0.012*sc;
                let sway = (time*1.2+hash).sin()*0.018*sc;
                let arm_sw = (time*1.6+hash).sin()*0.020*sc;
                // 脚 (包帯ぐるぐる)
                push_box(verts,idxs, cx-0.10*sc,0.0,cz-0.06*sc, cx-0.03*sc,0.28*sc,cz+0.06*sc,wrap_col);
                push_box(verts,idxs, cx+0.03*sc,0.0,cz-0.06*sc, cx+0.10*sc,0.28*sc,cz+0.06*sc,wrap_sh);
                // 包帯のほつれ (脚に横ライン)
                for i in 0..4i32 {
                    let yw = 0.04*sc + (i as f32)*0.06*sc;
                    push_box(verts,idxs, cx-0.11*sc,yw,cz-0.07*sc, cx+0.11*sc,yw+0.010*sc,cz+0.07*sc,skin_col);
                }
                // 胴体
                let by0=0.27*sc+bob; let by1=by0+0.32*sc;
                push_box(verts,idxs, cx-0.15*sc+sway,by0,cz-0.12*sc, cx+0.15*sc+sway,by1,cz+0.12*sc,wrap_col);
                push_box(verts,idxs, cx-0.17*sc+sway,by0+0.04*sc,cz-0.09*sc, cx-0.13*sc+sway,by1-0.04*sc,cz+0.09*sc,wrap_sh);
                push_box(verts,idxs, cx+0.13*sc+sway,by0+0.04*sc,cz-0.09*sc, cx+0.17*sc+sway,by1-0.04*sc,cz+0.09*sc,wrap_sh);
                // 包帯のほつれ (胴体)
                for i in 0..3i32 {
                    let yw = by0+0.04*sc+(i as f32)*0.08*sc;
                    push_box(verts,idxs, cx-0.16*sc,yw,cz-0.13*sc, cx+0.16*sc,yw+0.009*sc,cz+0.13*sc,rotted);
                }
                // 腕
                push_box(verts,idxs, cx-0.28*sc,by0+0.04*sc+arm_sw,cz-0.06*sc, cx-0.14*sc,by1-0.06*sc+arm_sw,cz+0.06*sc,wrap_col);
                push_box(verts,idxs, cx+0.14*sc,by0+0.04*sc-arm_sw,cz-0.06*sc, cx+0.28*sc,by1-0.06*sc-arm_sw,cz+0.06*sc,wrap_sh);
                // 首
                let hy0=by1+bob; let hy1=hy0+0.18*sc;
                push_box(verts,idxs, cx-0.05*sc+sway,by1-0.01*sc,cz-0.05*sc, cx+0.05*sc+sway,hy0,cz+0.05*sc,wrap_col);
                // 頭
                push_box(verts,idxs, cx-0.12*sc+sway,hy0,cz-0.11*sc, cx+0.12*sc+sway,hy1,cz+0.11*sc,wrap_col);
                // 腐敗した肌が露出
                push_box(verts,idxs, cx-0.08*sc+sway,hy0+0.02*sc,cz+0.06*sc, cx+0.08*sc+sway,hy0+0.12*sc,cz+0.112*sc,skin_col);
                // 目 (黄緑の呪いの輝き)
                let pulse=(time*2.4+hash).sin()*0.5+0.5_f32;
                let eg=[eye_col[0],eye_col[1],eye_col[2],pulse*2.0+1.8];
                let es=0.024*sc; let eyz=hy0+(hy1-hy0)*0.55; let ezf=cz+0.095*sc+sway*0.3;
                push_box(verts,idxs, cx-0.050*sc+sway-es,eyz-es*0.5,ezf, cx-0.050*sc+sway+es,eyz+es*0.7,ezf+es*0.4,eg);
                push_box(verts,idxs, cx+0.050*sc+sway-es,eyz-es*0.5,ezf, cx+0.050*sc+sway+es,eyz+es*0.7,ezf+es*0.4,eg);
                // 包帯のたれ (頭から垂れる)
                push_box(verts,idxs, cx-0.02*sc+sway,hy0-0.06*sc,cz-0.04*sc, cx+0.02*sc+sway,hy0+0.04*sc,cz-0.12*sc,wrap_col);
                // 腐敗の霧
                let fog=(time*0.9+hash*0.15).sin()*0.5+0.5_f32;
                push_box(verts,idxs, cx-0.28*sc,0.0,cz-0.28*sc, cx+0.28*sc,0.018*sc,cz+0.28*sc,[0.52,0.60,0.28,fog*0.45+0.10]);
                if lights.len()<4 { lights.push(Light{pos:[cx,hy1,cz,hash],col:[0.55,0.95,0.10,1.5]}); }
            }

            // ━━ 吸血コウモリ (TILE_VAMP_BAT): 小さくて素早い・赤目・牙 ━━
            if base_t == TILE_VAMP_BAT {
                let cx = (x0+x1)*0.5; let cz = (z0+z1)*0.5;
                let hash = (tx*19+ty*5) as f32;
                let sc = tier_base * match (tx*7+ty*3)%3 { 0=>0.62_f32, 1=>0.70, _=>0.78 };
                let body_col = [0.12_f32, 0.08, 0.12, 1.0]; // 黒体
                let wing_col = [0.18_f32, 0.06, 0.12, 1.0]; // 翼
                let wing_sh  = [0.10_f32, 0.04, 0.08, 1.0];
                let eye_col  = [0.90_f32, 0.10, 0.04, 4.0]; // 赤目
                let fang_col = [0.92_f32, 0.90, 0.90, 1.0]; // 白い牙
                let ear_col  = [0.22_f32, 0.08, 0.16, 1.0];
                let flap = (time*8.0+hash).sin(); // 素早い羽ばたき!
                let flutter = flap.abs();
                let bob = (time*5.0+hash).sin()*0.022*sc;
                let hover_y = 0.30*sc + bob; // 宙に浮く
                // 体 (小さい)
                push_box(verts,idxs, cx-0.08*sc,hover_y,cz-0.07*sc, cx+0.08*sc,hover_y+0.12*sc,cz+0.07*sc,body_col);
                // 頭 (体の上部)
                let hy0=hover_y+0.10*sc; let hy1=hy0+0.10*sc;
                push_box(verts,idxs, cx-0.07*sc,hy0,cz-0.07*sc, cx+0.07*sc,hy1,cz+0.06*sc,body_col);
                // 耳
                push_box(verts,idxs, cx-0.07*sc,hy1-0.01*sc,cz-0.02*sc, cx-0.02*sc,hy1+0.06*sc,cz+0.02*sc,ear_col);
                push_box(verts,idxs, cx+0.02*sc,hy1-0.01*sc,cz-0.02*sc, cx+0.07*sc,hy1+0.06*sc,cz+0.02*sc,ear_col);
                // 目 (赤い光!)
                let pulse=(time*4.0+hash).sin()*0.5+0.5_f32;
                let eg=[eye_col[0],eye_col[1],eye_col[2],pulse*2.0+2.5];
                let es=0.018*sc; let eyz=hy0+(hy1-hy0)*0.55; let ezf=cz+0.055*sc;
                push_box(verts,idxs, cx-0.032*sc-es,eyz-es*0.5,ezf, cx-0.032*sc+es,eyz+es*0.7,ezf+es*0.4,eg);
                push_box(verts,idxs, cx+0.032*sc-es,eyz-es*0.5,ezf, cx+0.032*sc+es,eyz+es*0.7,ezf+es*0.4,eg);
                // 牙 (小さいが怖い!)
                push_box(verts,idxs, cx-0.026*sc,hy0+0.010*sc,cz+0.045*sc, cx-0.010*sc,hy0+0.04*sc,cz+0.060*sc,fang_col);
                push_box(verts,idxs, cx+0.010*sc,hy0+0.010*sc,cz+0.045*sc, cx+0.026*sc,hy0+0.04*sc,cz+0.060*sc,fang_col);
                // 翼 (素早くはためく!)
                let wspan = 0.28*sc + flutter*0.08*sc;
                let wh    = 0.10*sc + flutter*0.04*sc;
                let wy0   = hover_y+0.04*sc; let wy1=wy0+wh;
                push_box(verts,idxs, cx-0.08*sc,wy0,cz-0.03*sc, cx-0.08*sc-wspan,wy1,cz+0.03*sc,wing_col);
                push_box(verts,idxs, cx-0.08*sc-wspan*0.6,wy0,cz-0.02*sc, cx-0.08*sc-wspan,wy0+0.02*sc,cz+0.02*sc,wing_sh);
                push_box(verts,idxs, cx+0.08*sc,wy0,cz-0.03*sc, cx+0.08*sc+wspan,wy1,cz+0.03*sc,wing_col);
                push_box(verts,idxs, cx+0.08*sc+wspan*0.6,wy0,cz-0.02*sc, cx+0.08*sc+wspan,wy0+0.02*sc,cz+0.02*sc,wing_sh);
                if lights.len()<4 { lights.push(Light{pos:[cx,hy1+0.04*sc,cz,hash],col:[0.8,0.10,0.10,1.2]}); }
            }

            // ━━ ノーム (TILE_GNOME): 小さいが元気・大きな帽子・ツルハシ ━━
            if base_t == TILE_GNOME {
                let cx = (x0+x1)*0.5; let cz = (z0+z1)*0.5;
                let hash = (tx*11+ty*23) as f32;
                let sc = tier_base * match (tx*5+ty*11)%4 { 0=>0.68_f32, 1=>0.74, 2=>0.80, _=>0.88 };
                // ノームの色バリエーション (4種類)
                let (body_col, hat_col, beard_col) = match (tx+ty)%4 {
                    0 => ([0.64_f32,0.46,0.28,1.0],[0.72_f32,0.14,0.10,1.0],[0.88_f32,0.84,0.76,1.0]), // 茶体・赤帽
                    1 => ([0.52_f32,0.60,0.34,1.0],[0.24_f32,0.48,0.20,1.0],[0.90_f32,0.86,0.72,1.0]), // 緑体・緑帽
                    2 => ([0.52_f32,0.44,0.60,1.0],[0.22_f32,0.22,0.56,1.0],[0.92_f32,0.88,0.80,1.0]), // 紫体・青帽
                    _ => ([0.60_f32,0.50,0.30,1.0],[0.60_f32,0.38,0.08,1.0],[0.94_f32,0.90,0.78,1.0]), // 黄土・橙帽
                };
                let skin_col = [0.88_f32, 0.72, 0.58, 1.0];
                let pick_col = [0.56_f32, 0.52, 0.48, 1.0]; // ツルハシ
                let pick_hd  = [0.36_f32, 0.34, 0.38, 1.0];
                let boot_col = [0.32_f32, 0.22, 0.12, 1.0];
                let bob  = (time*3.2+hash).sin().abs()*0.014*sc;
                let arm_sw = (time*3.6+hash).sin()*0.022*sc;
                let pick_sw = (time*2.8+hash).sin(); // ツルハシを振る
                // ブーツ
                push_box(verts,idxs, cx-0.10*sc,0.0,cz-0.09*sc, cx-0.03*sc,0.022*sc,cz+0.10*sc,boot_col);
                push_box(verts,idxs, cx+0.03*sc,0.0,cz-0.09*sc, cx+0.10*sc,0.022*sc,cz+0.10*sc,boot_col);
                // 脚 (短いがしっかり)
                push_box(verts,idxs, cx-0.09*sc,0.020*sc,cz-0.07*sc, cx-0.03*sc,0.18*sc,cz+0.07*sc,body_col);
                push_box(verts,idxs, cx+0.03*sc,0.020*sc,cz-0.07*sc, cx+0.09*sc,0.18*sc,cz+0.07*sc,body_col);
                // 胴体 (ずんぐり!)
                let by0=0.17*sc+bob; let by1=by0+0.22*sc;
                push_box(verts,idxs, cx-0.14*sc,by0,cz-0.12*sc, cx+0.14*sc,by1,cz+0.12*sc,body_col);
                // 腕 (ツルハシ右手)
                push_box(verts,idxs, cx-0.22*sc,by0+0.04*sc+arm_sw,cz-0.06*sc, cx-0.12*sc,by1-0.02*sc+arm_sw,cz+0.06*sc,body_col);
                push_box(verts,idxs, cx+0.12*sc,by0+0.04*sc-arm_sw,cz-0.06*sc, cx+0.22*sc,by1-0.02*sc-arm_sw,cz+0.06*sc,body_col);
                // 首と頭
                let hy0=by1+bob; let hy1=hy0+0.14*sc;
                push_box(verts,idxs, cx-0.05*sc,by1-0.01*sc,cz-0.05*sc, cx+0.05*sc,hy0,cz+0.05*sc,skin_col);
                push_box(verts,idxs, cx-0.10*sc,hy0,cz-0.10*sc, cx+0.10*sc,hy1,cz+0.10*sc,skin_col);
                // 鼻 (大きな鼻!)
                push_box(verts,idxs, cx-0.028*sc,hy0+(hy1-hy0)*0.30,cz+0.080*sc, cx+0.028*sc,hy0+(hy1-hy0)*0.52,cz+0.116*sc,skin_col);
                // 目
                let es=0.018*sc; let eyz=hy0+(hy1-hy0)*0.62; let ezf=cz+0.090*sc;
                push_box(verts,idxs, cx-0.042*sc-es,eyz-es*0.5,ezf, cx-0.042*sc+es,eyz+es*0.8,ezf+es*0.4,[0.14,0.24,0.60,1.0]);
                push_box(verts,idxs, cx+0.042*sc-es,eyz-es*0.5,ezf, cx+0.042*sc+es,eyz+es*0.8,ezf+es*0.4,[0.14,0.24,0.60,1.0]);
                // ひげ (白いふさふさ!)
                push_box(verts,idxs, cx-0.09*sc,hy0,cz+0.060*sc, cx+0.09*sc,hy0+0.08*sc,cz+0.105*sc,beard_col);
                push_box(verts,idxs, cx-0.07*sc,hy0-0.06*sc,cz+0.062*sc, cx+0.07*sc,hy0+0.02*sc,cz+0.102*sc,beard_col);
                // 帽子 (高くとがった!)
                push_box(verts,idxs, cx-0.12*sc,hy1-0.01*sc,cz-0.12*sc, cx+0.12*sc,hy1+0.04*sc,cz+0.12*sc,hat_col);
                push_box(verts,idxs, cx-0.07*sc,hy1+0.03*sc,cz-0.07*sc, cx+0.07*sc,hy1+0.12*sc,cz+0.07*sc,hat_col);
                push_box(verts,idxs, cx-0.04*sc,hy1+0.11*sc,cz-0.04*sc, cx+0.04*sc,hy1+0.20*sc,cz+0.04*sc,hat_col);
                push_box(verts,idxs, cx-0.02*sc,hy1+0.18*sc,cz-0.02*sc, cx+0.02*sc,hy1+0.26*sc,cz+0.02*sc,hat_col);
                // ツルハシ (右手から斜めに振り上げ)
                let px2=cx+0.24*sc; let pw0=by0+0.12*sc-arm_sw; let pw1=pw0+0.30*sc;
                push_box(verts,idxs, px2-0.012*sc,pw0,cz-0.012*sc, px2+0.012*sc,pw1+pick_sw*0.06*sc,cz+0.012*sc,pick_col);
                push_box(verts,idxs, px2-0.048*sc,pw1-0.04*sc+pick_sw*0.06*sc,cz-0.016*sc, px2+0.048*sc,pw1+0.022*sc+pick_sw*0.06*sc,cz+0.016*sc,pick_hd);
            }

            // ━━ 透明な stalker (TILE_STALKER): 半透明な亡霊・風の精 ━━
            if base_t == TILE_STALKER {
                let cx = (x0+x1)*0.5; let cz = (z0+z1)*0.5;
                let hash = (tx*23+ty*7) as f32;
                let sc = tier_base * 1.0_f32;
                let pulse  = (time*2.0+hash).sin()*0.5+0.5_f32;
                let ripple = (time*3.4+hash).sin();
                let drift  = (time*1.6+hash).sin()*0.030*sc;
                let alpha  = pulse*0.35+0.25_f32; // 半透明!
                let body_col = [0.82_f32, 0.90, 0.96, alpha];
                let glow_col = [0.60_f32, 0.80, 1.0, alpha*1.4+0.2];
                let edge_col = [0.40_f32, 0.65, 0.90, alpha*0.8];
                let bob = (time*2.2+hash).sin()*0.024*sc + 0.04*sc;
                // 足 (ゆらゆら)
                push_box(verts,idxs, cx-0.08*sc+drift,0.0+bob*0.3,cz-0.06*sc, cx-0.02*sc+drift,0.24*sc+bob,cz+0.06*sc,edge_col);
                push_box(verts,idxs, cx+0.02*sc+drift,0.0+bob*0.3,cz-0.06*sc, cx+0.08*sc+drift,0.24*sc+bob,cz+0.06*sc,edge_col);
                // 下部のたなびき (風のような)
                for i in 0..3i32 {
                    let fr=(i as f32)*0.14*sc;
                    let fa=(time*2.4+hash+fr).sin()*0.5+0.5_f32;
                    push_box(verts,idxs, cx-0.12*sc+fr*0.3,0.0,cz-0.04*sc, cx-0.04*sc+fr*0.3,(0.14-fr*0.5).max(0.02)*sc,cz+0.04*sc,[0.5,0.7,0.9,fa*0.2+0.05]);
                }
                // 胴体 (半透明の柱)
                let by0=0.22*sc+bob; let by1=by0+0.30*sc;
                push_box(verts,idxs, cx-0.14*sc+drift,by0,cz-0.12*sc, cx+0.14*sc+drift,by1,cz+0.12*sc,body_col);
                push_box(verts,idxs, cx-0.16*sc+drift,by0+0.04*sc,cz-0.09*sc, cx-0.12*sc+drift,by1-0.04*sc,cz+0.09*sc,glow_col);
                push_box(verts,idxs, cx+0.12*sc+drift,by0+0.04*sc,cz-0.09*sc, cx+0.16*sc+drift,by1-0.04*sc,cz+0.09*sc,glow_col);
                // 腕 (流れるような)
                let aw = ripple*0.020*sc;
                push_box(verts,idxs, cx-0.28*sc+drift,by0+0.08*sc+aw,cz-0.05*sc, cx-0.13*sc+drift,by1-0.06*sc+aw,cz+0.05*sc,edge_col);
                push_box(verts,idxs, cx+0.13*sc+drift,by0+0.08*sc-aw,cz-0.05*sc, cx+0.28*sc+drift,by1-0.06*sc-aw,cz+0.05*sc,edge_col);
                // 首+頭
                let hy0=by1+bob*0.2; let hy1=hy0+0.20*sc;
                push_box(verts,idxs, cx-0.05*sc+drift,by1-0.01*sc,cz-0.05*sc, cx+0.05*sc+drift,hy0,cz+0.05*sc,glow_col);
                push_box(verts,idxs, cx-0.12*sc+drift,hy0,cz-0.11*sc, cx+0.12*sc+drift,hy1,cz+0.11*sc,body_col);
                // 目 (白く輝く — 透明体でも目は見える)
                let eg=[0.9_f32,0.96,1.0,pulse*1.5+2.0];
                let es=0.026*sc; let eyz=hy0+(hy1-hy0)*0.52; let ezf=cz+0.095*sc;
                push_box(verts,idxs, cx-0.050*sc+drift-es,eyz-es*0.5,ezf, cx-0.050*sc+drift+es,eyz+es*0.8,ezf+es*0.4,eg);
                push_box(verts,idxs, cx+0.050*sc+drift-es,eyz-es*0.5,ezf, cx+0.050*sc+drift+es,eyz+es*0.8,ezf+es*0.4,eg);
                // 風の軌跡 (周囲に広がる)
                for i in 0..4i32 {
                    let angle = (i as f32)*1.571 + time*1.2+hash;
                    let rx=angle.cos()*0.36*sc; let rz=angle.sin()*0.36*sc;
                    let rp=(time*3.0+hash+(i as f32)*0.7).sin()*0.5+0.5_f32;
                    push_box(verts,idxs, cx+rx-0.024*sc,by0+0.04*sc,cz+rz-0.024*sc, cx+rx+0.024*sc,by0+0.10*sc,cz+rz+0.024*sc,[0.6,0.82,1.0,rp*0.28+0.05]);
                }
                if lights.len()<4 { lights.push(Light{pos:[cx,hy1,cz,hash],col:[0.55,0.80,1.0,pulse*2.5+1.2]}); }
            }

            // ━━ クゾーン (TILE_XORN): 石の巨体・複数の腕・岩の質感 ━━
            if base_t == TILE_XORN {
                let cx = (x0+x1)*0.5; let cz = (z0+z1)*0.5;
                let hash = (tx*13+ty*29) as f32;
                let sc = tier_base * match (tx*3+ty*7)%3 { 0=>1.0_f32, 1=>1.10, _=>1.20 };
                let rock1 = [0.52_f32, 0.48, 0.44, 1.0]; // 岩色1
                let rock2 = [0.40_f32, 0.37, 0.34, 1.0]; // 岩色2 (暗)
                let rock3 = [0.64_f32, 0.58, 0.50, 1.0]; // 岩色3 (明)
                let crystal= [0.30_f32, 0.56, 0.72, 2.5]; // 水晶の目
                let ore_col = [0.90_f32, 0.82, 0.14, 2.0]; // 金鉱石の光
                let stomp  = (time*1.8+hash).sin();
                let bob    = stomp.abs()*0.014*sc;
                let tremor = (time*4.2+hash).sin()*0.008*sc; // 大地揺れ感
                // 脚 (岩の柱のような)
                push_box(verts,idxs, cx-0.16*sc,0.0,cz-0.11*sc, cx-0.04*sc,0.28*sc,cz+0.11*sc,rock1);
                push_box(verts,idxs, cx+0.04*sc,0.0,cz-0.11*sc, cx+0.16*sc,0.28*sc,cz+0.11*sc,rock2);
                push_box(verts,idxs, cx-0.18*sc,0.0,cz-0.08*sc, cx-0.14*sc,0.18*sc,cz+0.08*sc,rock3);
                push_box(verts,idxs, cx+0.14*sc,0.0,cz-0.08*sc, cx+0.18*sc,0.18*sc,cz+0.08*sc,rock2);
                // 巨大な胴体 (石の塊)
                let by0=0.26*sc+bob; let by1=by0+0.40*sc;
                push_box(verts,idxs, cx-0.26*sc+tremor,by0,cz-0.22*sc, cx+0.26*sc+tremor,by1,cz+0.22*sc,rock1);
                push_box(verts,idxs, cx-0.28*sc+tremor,by0+0.06*sc,cz-0.18*sc, cx-0.22*sc+tremor,by1-0.06*sc,cz+0.18*sc,rock2);
                push_box(verts,idxs, cx+0.22*sc+tremor,by0+0.06*sc,cz-0.18*sc, cx+0.28*sc+tremor,by1-0.06*sc,cz+0.18*sc,rock3);
                // 岩のでっぱり (体表)
                for i in 0..3i32 {
                    let yw=by0+0.06*sc+(i as f32)*0.10*sc;
                    let bw=0.04*sc; let bd2=0.06*sc;
                    push_box(verts,idxs, cx-0.30*sc,yw,cz-bd2, cx-0.24*sc,yw+bw*2.0,cz+bd2,rock3);
                    push_box(verts,idxs, cx+0.24*sc,yw,cz-bd2, cx+0.30*sc,yw+bw*2.0,cz+bd2,rock3);
                }
                // 4本の腕 (前後2対)
                let arm_wave = (time*1.4+hash).sin()*0.022*sc;
                // 前腕2本
                push_box(verts,idxs, cx-0.40*sc+tremor,by0+0.10*sc+arm_wave,cz+0.04*sc, cx-0.24*sc+tremor,by0+0.28*sc+arm_wave,cz+0.18*sc,rock1);
                push_box(verts,idxs, cx+0.24*sc+tremor,by0+0.10*sc-arm_wave,cz+0.04*sc, cx+0.40*sc+tremor,by0+0.28*sc-arm_wave,cz+0.18*sc,rock2);
                // 後腕2本 (対角に張り出す)
                push_box(verts,idxs, cx-0.38*sc+tremor,by0+0.18*sc-arm_wave,cz-0.18*sc, cx-0.22*sc+tremor,by0+0.34*sc-arm_wave,cz-0.04*sc,rock2);
                push_box(verts,idxs, cx+0.22*sc+tremor,by0+0.18*sc+arm_wave,cz-0.18*sc, cx+0.38*sc+tremor,by0+0.34*sc+arm_wave,cz-0.04*sc,rock1);
                // 先端の鉤爪 (石の爪)
                for i in 0..2i32 {
                    let zd=(i as f32-0.5)*0.032*sc;
                    push_box(verts,idxs, cx-0.44*sc,by0+0.08*sc+arm_wave,cz+0.06*sc+zd, cx-0.38*sc,by0+0.12*sc+arm_wave,cz+0.12*sc+zd,rock3);
                    push_box(verts,idxs, cx+0.38*sc,by0+0.08*sc-arm_wave,cz+0.06*sc+zd, cx+0.44*sc,by0+0.12*sc-arm_wave,cz+0.12*sc+zd,rock3);
                }
                // 頭 (胴体と一体化した岩の塊)
                let hy0=by1-0.04*sc+bob; let hy1=hy0+0.22*sc;
                push_box(verts,idxs, cx-0.20*sc+tremor,hy0,cz-0.18*sc, cx+0.20*sc+tremor,hy1,cz+0.18*sc,rock1);
                push_box(verts,idxs, cx-0.22*sc+tremor,hy0+0.02*sc,cz-0.14*sc, cx-0.18*sc+tremor,hy1-0.02*sc,cz+0.14*sc,rock2);
                push_box(verts,idxs, cx+0.18*sc+tremor,hy0+0.02*sc,cz-0.14*sc, cx+0.22*sc+tremor,hy1-0.02*sc,cz+0.14*sc,rock3);
                // 3つの目 (三角形に並ぶ水晶)
                let pulse=(time*2.6+hash).sin()*0.5+0.5_f32;
                let cg=[crystal[0],crystal[1],crystal[2],pulse*1.5+crystal[3]];
                let es=0.028*sc; let eyz=hy0+(hy1-hy0)*0.55; let ezf=cz+0.145*sc+tremor;
                push_box(verts,idxs, cx-0.070*sc+tremor-es,eyz-es*0.6,ezf, cx-0.070*sc+tremor+es,eyz+es*0.8,ezf+es*0.5,cg);
                push_box(verts,idxs, cx+0.070*sc+tremor-es,eyz-es*0.6,ezf, cx+0.070*sc+tremor+es,eyz+es*0.8,ezf+es*0.5,cg);
                push_box(verts,idxs, cx-es*0.8+tremor,eyz+es*0.8,ezf, cx+es*0.8+tremor,eyz+es*2.0,ezf+es*0.5,cg);
                // 金鉱石の輝き (体表にキラキラ)
                for i in 0..3i32 {
                    let ox=cx+(i as f32-1.0)*0.12*sc+tremor;
                    let oy=by0+0.08*sc+(i as f32)*0.08*sc;
                    push_box(verts,idxs, ox-0.020*sc,oy,cz-0.23*sc, ox+0.020*sc,oy+0.030*sc,cz-0.21*sc,ore_col);
                }
                if lights.len()<4 { lights.push(Light{pos:[cx,hy1,cz,hash],col:[0.30,0.56,0.72,pulse*2.0+1.0]}); }
            }

            // ━━ ボスオーラ (tier==3): 足元に黄金発光ディスク ━━
            if tier == 3 && base_t != TILE_FLOOR && base_t != TILE_WALL
                && base_t != TILE_DOOR && base_t != TILE_EMPTY
                && base_t != TILE_STAIRS_U && base_t != TILE_STAIRS_D
            {
                let cx = (x0+x1)*0.5; let cz = (z0+z1)*0.5;
                let ph = (tx*13+ty*7) as f32;
                let pulse = (time*2.8+ph*0.31).sin()*0.5+0.5_f32;
                let ar = 0.42+pulse*0.10; // 脈動する半径
                // 外リング (広い薄い)
                push_box(verts,idxs, cx-ar,0.008,cz-ar, cx+ar,0.018,cz+ar,
                    [0.96,0.88,0.12, pulse*2.5+0.8]);
                // 内リング (小さく明るい)
                let ir = ar*0.55;
                push_box(verts,idxs, cx-ir,0.019,cz-ir, cx+ir,0.029,cz+ir,
                    [1.0,0.95,0.45, pulse*1.8+1.2]);
                // 追加ライト (ボス輝き)
                if lights.len()<4 {
                    lights.push(Light{
                        pos:[cx,0.6,cz,ph],
                        col:[0.96,0.78,0.12, pulse*4.0+1.5],
                    });
                }
            }

            if base_t == TILE_FLOOR && tx % 8 == 0 && ty % 6 == 0 && torch_count < 2 {
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

/// プレイヤーキャラクター — 全身騎士型ボックスモデル (鎧・剣・盾付き)
pub fn build_player(
    verts: &mut Vec<Vertex>, idxs: &mut Vec<u32>,
    px: f32, pz: f32, time: f32,
) {
    let walk = (time * 4.5).sin();
    let bob  = (time * 9.0).sin().abs() * 0.013; // 腰より上のボブ (常に正)
    let lf   =  walk * 0.055; // 脚の前後振り (右脚前)
    let af   = -walk * 0.040; // 腕の前後振り (右腕逆位相)
    let sy   = bob;           // 腰より上全体のY オフセット

    // ── 色定義 ──
    let boot_col  = [0.18, 0.12, 0.07, 3.0_f32]; // 黒革ブーツ
    let leg_col   = [0.24, 0.20, 0.15, 3.0_f32]; // レザーレギンス
    let belt_col  = [0.32, 0.20, 0.08, 3.0_f32]; // 茶革ベルト
    let armor_col = [0.58, 0.62, 0.72, 3.0_f32]; // チェインメイル (シルバー)
    let arm_col   = [0.52, 0.56, 0.66, 3.0_f32]; // 袖鎧
    let neck_col  = [0.93, 0.80, 0.66, 3.0_f32]; // 肌 (首)
    let head_col  = [0.95, 0.83, 0.70, 3.0_f32]; // 顔
    let helm_col  = [0.48, 0.50, 0.60, 3.0_f32]; // ヘルメット (くすんだ鉄)
    let visor_col = [0.22, 0.22, 0.28, 3.0_f32]; // バイザー (暗い)
    let sword_col = [0.88, 0.85, 0.60, 3.0_f32]; // 剣の刃 (金属)
    let guard_col = [0.72, 0.68, 0.40, 3.0_f32]; // 鍔 (金)
    let grip_col  = [0.28, 0.14, 0.06, 3.0_f32]; // 柄 (革)
    let shld_col  = [0.20, 0.32, 0.65, 3.0_f32]; // 盾 (青)
    let boss_col  = [0.65, 0.65, 0.38, 3.0_f32]; // 盾中央ボス (金)

    // ── 右脚 (前進) ──
    push_box(verts, idxs, px+0.02,0.00,pz-0.09+lf, px+0.12,0.07,pz+0.04+lf, boot_col);
    push_box(verts, idxs, px+0.03,0.07,pz-0.08+lf, px+0.11,0.32,pz+0.03+lf, leg_col);

    // ── 左脚 (後退) ──
    push_box(verts, idxs, px-0.12,0.00,pz-0.09-lf, px-0.02,0.07,pz+0.04-lf, boot_col);
    push_box(verts, idxs, px-0.11,0.07,pz-0.08-lf, px-0.03,0.32,pz+0.03-lf, leg_col);

    // ── 腰ベルト ──
    push_box(verts, idxs, px-0.14,sy+0.30,pz-0.10, px+0.14,sy+0.36,pz+0.10, belt_col);

    // ── 胴体 (鎧) ──
    push_box(verts, idxs, px-0.14,sy+0.35,pz-0.10, px+0.14,sy+0.60,pz+0.10, armor_col);

    // ── 右腕 (剣側・腕は逆位相) ──
    push_box(verts, idxs, px+0.14,sy+0.36,pz-0.08+af, px+0.23,sy+0.58,pz+0.07+af, arm_col);

    // ── 左腕 (盾側) ──
    push_box(verts, idxs, px-0.23,sy+0.36,pz-0.08-af, px-0.14,sy+0.58,pz+0.07-af, arm_col);

    // ── 首 ──
    push_box(verts, idxs, px-0.05,sy+0.59,pz-0.05, px+0.05,sy+0.65,pz+0.05, neck_col);

    // ── 頭 ──
    push_box(verts, idxs, px-0.10,sy+0.63,pz-0.10, px+0.10,sy+0.83,pz+0.10, head_col);

    // ── ヘルメット + バイザー ──
    push_box(verts, idxs, px-0.11,sy+0.78,pz-0.11, px+0.11,sy+0.92,pz+0.11, helm_col);
    push_box(verts, idxs, px-0.09,sy+0.69,pz-0.115, px+0.09,sy+0.78,pz-0.100, visor_col);

    // ── 剣 (右手) — 腕の振りに追随 ──
    let sz  = pz - 0.04 + af;      // 剣のZ中心
    let sy2 = sy + 0.36;            // 腕の付け根Y
    // 柄
    push_box(verts, idxs, px+0.176,sy2-0.20,sz-0.020, px+0.204,sy2-0.03,sz+0.020, grip_col);
    // 鍔
    push_box(verts, idxs, px+0.12,sy2-0.04,sz-0.025, px+0.26,sy2+0.02,sz+0.025, guard_col);
    // 刃 (頭上まで伸びる長めの刃)
    push_box(verts, idxs, px+0.175,sy2+0.01,sz-0.022, px+0.205,sy2+0.55,sz+0.022, sword_col);

    // ── 盾 (左手) — 腕の振りと逆位相 ──
    let shz = pz - 0.02 - af;
    let shy = sy + 0.28;
    // 盾本体 (平たい青い板)
    push_box(verts, idxs, px-0.34,shy,shz-0.03, px-0.17,shy+0.30,shz+0.03, shld_col);
    // 盾中央ボス (前方に少し飛び出す)
    push_box(verts, idxs, px-0.29,shy+0.10,shz-0.06, px-0.22,shy+0.20,shz-0.02, boss_col);

    // ── 足元の光輪 ──
    push_quad(verts, idxs,
        [px-0.22, 0.005, pz-0.22],
        [px+0.22, 0.005, pz-0.22],
        [px+0.22, 0.005, pz+0.22],
        [px-0.22, 0.005, pz+0.22],
        [0.40, 0.55, 1.0, 3.0]);
}
