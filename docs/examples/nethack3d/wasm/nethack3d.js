import * as wasm from "./nethack3d_bg.wasm";
import { __wbg_set_wasm } from "./nethack3d_bg.js";

__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
export {
    camera_name_nethack3d, get_vp_flat_nethack3d, init_nethack3d, reset_cam_yaw_offset_nethack3d, set_cam_yaw_offset_nethack3d, set_map_nethack3d, set_player_nethack3d, switch_camera_nethack3d, tick_nethack3d
} from "./nethack3d_bg.js";
