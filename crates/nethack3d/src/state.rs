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
    pub cam_yaw_offset: f32,   // タッチスワイプによるカメラヨー追加回転
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
            cam_yaw_offset: 0.0,
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

    pub fn set_cam_yaw_offset(&mut self, v: f32) {
        self.cam_yaw_offset = v;
    }

    pub fn reset_cam_yaw_offset(&mut self) {
        self.cam_yaw_offset = 0.0;
    }

    /// VP行列をフラット配列で返す (column-major 16 floats) — JS側でエンティティ投影に使用
    pub fn get_vp_flat(&self) -> Vec<f32> {
        let vp = self.calc_vp();
        vp.iter().flat_map(|col| col.iter().copied()).collect()
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
            warp: if matches!(self.cam_mode, CameraMode::Top) { 1.0 } else { 0.0 },
            pad: [0.0; 2],
            lights: [lights[0], lights[1], lights[2], lights[3]],
            fog_col: [0.0, 0.0, 0.0, 1.0],
        };

        // 描画
        self.gpu.render(&verts, &idxs, &uni);
    }

    fn calc_vp(&self) -> [[f32;4];4] {
        let asp = self.gpu.width as f32 / self.gpu.height.max(1) as f32;

        // タイル中心に合わせる (+0.5 オフセット)
        let px = self.vis_x + 0.5;
        let pz = self.vis_z + 0.5;
        let up = [0.0_f32, 1.0, 0.0];

        match self.cam_mode {
            CameraMode::Top => {
                // 斜め上からの俯瞰ビュー
                // 完全真上だと up ベクトルと視線が平行になる不具合があるため少し前傾
                let proj = perspective(0.90, asp, 0.1, 80.0);
                let view = look_at(
                    [px, 10.0, pz - 5.0],
                    [px,  0.5, pz + 3.0],
                    up,
                );
                mat_mul(proj, view)
            }
            CameraMode::Tps => {
                let proj = perspective(0.75, asp, 0.1, 80.0);
                let angle = self.vis_angle + self.cam_yaw_offset;
                let back_x = -angle.cos() * 5.0;
                let back_z = -angle.sin() * 5.0;
                let view = look_at(
                    [px + back_x, 3.5, pz + back_z],
                    [px + angle.cos() * 2.0, 0.5, pz + angle.sin() * 2.0],
                    up,
                );
                mat_mul(proj, view)
            }
            CameraMode::Fps => {
                let proj = perspective(1.05, asp, 0.05, 60.0);
                let angle = self.vis_angle + self.cam_yaw_offset;
                let fwd_x = angle.cos();
                let fwd_z = angle.sin();
                let view = look_at(
                    [px, 0.65, pz],
                    [px + fwd_x * 4.0, 0.65, pz + fwd_z * 4.0],
                    up,
                );
                mat_mul(proj, view)
            }
        }
    }
}
