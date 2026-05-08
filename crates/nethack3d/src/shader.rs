// シェーダー: ダンジョン向けWGSL (rustgamesと同じ構造)
//
// col.a エンコード:
//   1.0 = 通常面 (床・天井)
//   2.0 = 石壁面 → ライティング + グレイン
//   3.0 = 発光体 (ライティング無視)

pub const SHADER: &str = r#"
struct Light {
    pos : vec4<f32>,
    col : vec4<f32>,
}
struct Uni {
    vp     : mat4x4<f32>,
    time   : f32,
    warp   : f32,
    pad0   : f32,
    pad1   : f32,
    lights : array<Light, 4>,
    fog_col: vec4<f32>,
}
@group(0) @binding(0) var<uniform> u: Uni;

struct VIn {
    @location(0) pos : vec3<f32>,
    @location(1) col : vec4<f32>,
}
struct VOut {
    @builtin(position) clip     : vec4<f32>,
    @location(0)       col      : vec4<f32>,
    @location(1)       depth    : f32,
    @location(2)       world_y  : f32,
    @location(3)       world_xz : vec2<f32>,
}

@vertex
fn vs_main(v: VIn) -> VOut {
    var o: VOut;
    let c      = u.vp * vec4<f32>(v.pos, 1.0);
    o.clip     = c;
    o.col      = v.col;
    o.depth    = c.w;
    o.world_y  = v.pos.y;
    o.world_xz = vec2<f32>(v.pos.x, v.pos.z);
    return o;
}

@fragment
fn fs_main(v: VOut) -> @location(0) vec4<f32> {
    var rgb = v.col.rgb;

    // 発光体: フォグのみ適用
    if v.col.a > 2.5 {
        let fog = max(exp(-0.12 * 0.12 * v.depth * v.depth), 0.2);
        return vec4<f32>(min(rgb * fog, vec3<f32>(1.0)), 1.0);
    }

    // ポイントライト累積
    let wpos = vec3<f32>(v.world_xz.x, v.world_y, v.world_xz.y);
    var light_acc = vec3<f32>(0.0);
    for (var i = 0; i < 4; i++) {
        let lpos    = u.lights[i].pos.xyz;
        let lcol    = u.lights[i].col.rgb;
        let lint    = u.lights[i].col.a;
        let phase   = u.lights[i].pos.w;
        let flicker = sin(u.time * 2.5 + phase * 6.283) * 0.08 + 0.92;
        let dist    = length(lpos - wpos);
        let att     = lint * flicker / (1.0 + dist * dist * 0.45);
        light_acc  += lcol * att;
    }

    // u.warp = 0.0: TPS/FPS (通常)、1.0: TOP (明るい俯瞰)
    let ambient = 0.06 + u.warp * 1.50;

    // 石壁: グレイン効果
    if v.col.a > 1.5 {
        let gu = floor(v.world_xz.x * 6.0) + floor(v.world_y * 7.0) * 13.0;
        let gv = floor(v.world_xz.y * 6.0) + floor(v.world_y * 7.0) * 13.0;
        let grain = fract(sin(gu * 127.1 + gv * 311.7) * 43758.5) * 0.10 + 0.90;
        rgb = rgb * grain;
    }

    // フォグ: TOP時は密度を大幅に下げて遠景まで見えるようにする
    let fog_density = 0.08 * (1.0 - u.warp * 0.89);
    let fog = exp(-fog_density * fog_density * v.depth * v.depth);
    let fog_floor = clamp(v.world_y * 2.0, 0.0, 1.0);
    let fog_final = fog * (0.7 + fog_floor * 0.3);
    let lit = rgb * (ambient + light_acc);
    rgb = mix(u.fog_col.rgb, lit, fog_final);

    return vec4<f32>(rgb, 1.0);
}
"#;
