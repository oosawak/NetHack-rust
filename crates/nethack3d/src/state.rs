// ゲーム状態管理: カメラ・プレイヤー・レンダリングループ

use crate::gpu::GpuState;
use crate::math::*;
use crate::geometry::*;

#[derive(Clone, Copy, PartialEq)]
pub enum CameraMode { Top, Tps, Fps }

pub struct Nethack3dState {
    pub gpu:          GpuState,
    pub tiles:        Vec<u8>,
    pub map_w:        usize,
    pub map_h:        usize,
    pub player_x:     f32,
    pub player_z:     f32,
    pub player_facing:u8,          // 0=N,1=E,2=S,3=W,4-7=対角
    pub vis_x:        f32,
    pub vis_z:        f32,
    pub vis_angle:    f32,          // ラジアン
    pub cam_mode:     CameraMode,
    pub time:         f32,
    pub prev_ts:      f64,
}

impl Nethack3dState {
    pub fn new(gpu: GpuState) -> Self {
        Nethack3dState {
            gpu,
            tiles: vec![],
            map_w: 0, map_h: 0,
            player_x: 40.0, player_z: 12.0,
            player_facing: 2,
            vis_x: 40.0, vis_z: 12.0,
            vis_angle: std::f32::consts::FRAC_PI_2,
            cam_mode: CameraMode::Tps,
            time: 0.0,
            prev_ts: 0.0,
        }
    }

    pub fn set_map(&mut self, tiles: Vec<u8>, w: usize, h: usize) {
        self.tiles = tiles;
        self.map_w = w;
        self.map_h = h;
    }

    pub fn set_player(&mut self, x: f32, z: f32, facing: u8) {
        self.player_x = x;
        self.player_z = z;
        self.player_facing = facing;
    }

    pub fn switch_camera(&mut self) {
        self.cam_mode = match self.cam_mode {
            CameraMode::Tps => CameraMode::Top,
            CameraMode::Top => CameraMode::Fps,
            CameraMode::Fps => CameraMode::Tps,
        };
    }

    pub fn camera_name(&self) -> &'static str {
        match self.cam_mode {
            CameraMode::Tps => "TPS",
            CameraMode::Top => "TOP",
            CameraMode::Fps => "FPS",
        }
    }

    pub fn tick(&mut self, ts: f64) {
        // 経過時間
        let dt = if self.prev_ts == 0.0 { 0.016 } else { ((ts - self.prev_ts) / 1000.0).min(0.1) as f32 };
        self.prev_ts = ts;
        self.time += dt;

        // スムーズカメラ追従
        let t_pos   = 1.0 - (-9.0  * dt).exp();
        let t_ang   = 1.0 - (-12.0 * dt).exp();
        self.vis_x = self.vis_x + (self.player_x - self.vis_x) * t_pos;
        self.vis_z = self.vis_z + (self.player_z - self.vis_z) * t_pos;

        let target_angle = facing_to_angle(self.player_facing);
        self.vis_angle   = lerp_angle(self.vis_angle, target_angle, t_ang);

        // ジオメトリ構築
        let mut verts  = Vec::with_capacity(8192);
        let mut idxs   = Vec::with_capacity(16384);
        let mut lights: Vec<Light> = Vec::with_capacity(4);

        if !self.tiles.is_empty() {
            build_dungeon(
                &self.tiles, self.map_w, self.map_h,
                self.time, self.player_x, self.player_z,
                &mut verts, &mut idxs, &mut lights,
            );
        }
        build_player(&mut verts, &mut idxs, self.vis_x, self.vis_z, self.time);

        // ライトパッド (4個に合わせる)
        while lights.len() < 4 {
            lights.push(Light { pos: [0.0;4], col: [0.0;4] });
        }

        // カメラ行列計算
        let vp = self.calc_vp();
        let uni = Uni {
            vp,
            time: self.time,
            warp: 0.0,
            pad: [0.0; 2],
            lights: [lights[0], lights[1], lights[2], lights[3]],
            fog_col: [0.0, 0.0, 0.0, 1.0],
        };

        // 描画
        self.gpu.render(&verts, &idxs, &uni);
    }

    fn calc_vp(&self) -> [[f32;4];4] {
        let asp = self.gpu.width as f32 / self.gpu.height.max(1) as f32;
        let proj = perspective(0.75, asp, 0.1, 80.0);

        let px = self.vis_x; let pz = self.vis_z;
        let up = [0.0_f32, 1.0, 0.0];

        let view = match self.cam_mode {
            CameraMode::Top => {
                // 真上から少し傾けたトップビュー
                look_at(
                    [px, 14.0, pz - 2.0],
                    [px, 0.0, pz + 2.0],
                    up,
                )
            }
            CameraMode::Tps => {
                // 後方斜め上 (向き対応)
                let back_x = -self.vis_angle.cos() * 5.0;
                let back_z = -self.vis_angle.sin() * 5.0;
                look_at(
                    [px + back_x, 3.5, pz + back_z],
                    [px + self.vis_angle.cos() * 2.0, 0.5, pz + self.vis_angle.sin() * 2.0],
                    up,
                )
            }
            CameraMode::Fps => {
                // 一人称視点
                let fwd_x = self.vis_angle.cos();
                let fwd_z = self.vis_angle.sin();
                look_at(
                    [px, 0.6, pz],
                    [px + fwd_x, 0.6, pz + fwd_z],
                    up,
                )
            }
        };

        mat_mul(proj, view)
    }
}
