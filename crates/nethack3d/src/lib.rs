// WASM エントリーポイント: JS→Rust API

mod math;
mod shader;
mod gpu;
mod geometry;
mod state;

use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use crate::state::Nethack3dState;

macro_rules! log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&JsValue::from_str(&format!($($t)*)));
    }
}

thread_local! {
    static STATE: RefCell<Option<Nethack3dState>> = RefCell::new(None);
}

/// キャンバスIDを指定して3Dレンダラーを初期化
#[wasm_bindgen]
pub async fn init_nethack3d(canvas_id: String) -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    log!("[3D] init_nethack3d start canvas_id={}", canvas_id);

    let window   = web_sys::window().ok_or("no window")?;
    let document = window.document().ok_or("no document")?;
    log!("[3D] document acquired");

    let canvas: web_sys::HtmlCanvasElement = document
        .get_element_by_id(&canvas_id)
        .ok_or("canvas not found")?
        .dyn_into()
        .map_err(|_| "not a canvas")?;
    log!("[3D] canvas found w={} h={}", canvas.width(), canvas.height());

    let gpu = gpu::GpuState::new(canvas).await
        .map_err(|e| {
            log!("[3D] GpuState::new ERROR: {}", e);
            JsValue::from_str(&e)
        })?;
    log!("[3D] GpuState ready");

    let s = Nethack3dState::new(gpu);
    STATE.with(|st| *st.borrow_mut() = Some(s));
    log!("[3D] init_nethack3d DONE");
    Ok(())
}

/// アニメーションフレーム更新 (requestAnimationFrameから呼ぶ)
/// ts: DOMHighResTimeStamp (ms)
#[wasm_bindgen]
pub fn tick_nethack3d(ts: f64) {
    STATE.with(|st| {
        if let Some(ref mut s) = *st.borrow_mut() {
            s.tick(ts);
        }
    });
}

/// マップタイル配列を渡す
/// tiles: Uint8Array (row-major, w×h)
/// 各バイト: 0=空 1=床 2=壁 3=廊下 4=扉 5=プレイヤー 6=上り 7=下り 8=モンスター 9=アイテム
#[wasm_bindgen]
pub fn set_map_nethack3d(tiles: &[u8], w: usize, h: usize) {
    STATE.with(|st| {
        if let Some(ref mut s) = *st.borrow_mut() {
            s.set_map(tiles.to_vec(), w, h);
        }
    });
}

/// プレイヤー座標と向きを更新
/// x/z: タイル座標 (整数もしくは補間済み float)
/// facing: 0=N 1=E 2=S 3=W 4=NE 5=SE 6=SW 7=NW
#[wasm_bindgen]
pub fn set_player_nethack3d(x: f32, z: f32, facing: u8) {
    STATE.with(|st| {
        if let Some(ref mut s) = *st.borrow_mut() {
            s.set_player(x, z, facing);
        }
    });
}

/// カメラモードを切り替え (TPS → TOP → FPS → TPS ...)
#[wasm_bindgen]
pub fn switch_camera_nethack3d() {
    STATE.with(|st| {
        if let Some(ref mut s) = *st.borrow_mut() {
            s.switch_camera();
        }
    });
}

/// カメラヨーオフセット設定 (タッチスワイプ用, ラジアン)
#[wasm_bindgen]
pub fn set_cam_yaw_offset_nethack3d(v: f32) {
    STATE.with(|st| {
        if let Some(ref mut s) = *st.borrow_mut() { s.set_cam_yaw_offset(v); }
    });
}

/// カメラヨーオフセットをリセット (プレイヤー移動後に呼ぶ)
#[wasm_bindgen]
pub fn reset_cam_yaw_offset_nethack3d() {
    STATE.with(|st| {
        if let Some(ref mut s) = *st.borrow_mut() { s.reset_cam_yaw_offset(); }
    });
}

/// VP行列をフラット配列で返す (column-major 16 floats)
#[wasm_bindgen]
pub fn get_vp_flat_nethack3d() -> Vec<f32> {
    STATE.with(|st| {
        if let Some(ref s) = *st.borrow() { s.get_vp_flat() }
        else { vec![0.0; 16] }
    })
}

/// 現在のカメラ名を取得 ("TPS" / "TOP" / "FPS")
#[wasm_bindgen]
pub fn camera_name_nethack3d() -> String {
    STATE.with(|st| {
        if let Some(ref s) = *st.borrow() {
            s.camera_name().to_string()
        } else {
            "TPS".to_string()
        }
    })
}
