// 数学モジュール: 行列演算・カメラ計算

pub type M4 = [[f32; 4]; 4];

pub fn mat_mul(a: M4, b: M4) -> M4 {
    let mut r = [[0f32; 4]; 4];
    for c in 0..4 { for row in 0..4 {
        r[c][row] = (0..4).map(|k| a[k][row] * b[c][k]).sum();
    }}
    r
}

pub fn perspective(fov: f32, asp: f32, n: f32, f: f32) -> M4 {
    let t = 1.0 / (fov * 0.5).tan();
    [[t/asp,0.0,0.0,0.0],[0.0,t,0.0,0.0],
     [0.0,0.0,f/(n-f),-1.0],[0.0,0.0,n*f/(n-f),0.0]]
}

pub fn norm3(v:[f32;3])->[f32;3]{let l=(v[0]*v[0]+v[1]*v[1]+v[2]*v[2]).sqrt();if l<1e-7{[0.0,0.0,1.0]}else{[v[0]/l,v[1]/l,v[2]/l]}}
pub fn sub3(a:[f32;3],b:[f32;3])->[f32;3]{[a[0]-b[0],a[1]-b[1],a[2]-b[2]]}
pub fn cross(a:[f32;3],b:[f32;3])->[f32;3]{[a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]]}
pub fn dot3(a:[f32;3],b:[f32;3])->f32{a[0]*b[0]+a[1]*b[1]+a[2]*b[2]}

pub fn look_at(eye:[f32;3],ctr:[f32;3],up:[f32;3])->M4{
    let f=norm3(sub3(ctr,eye));let r=norm3(cross(f,norm3(up)));let u=cross(r,f);
    [[r[0],u[0],-f[0],0.0],[r[1],u[1],-f[1],0.0],[r[2],u[2],-f[2],0.0],
     [-dot3(r,eye),-dot3(u,eye),dot3(f,eye),1.0]]
}

/// 向き(0=N,1=E,2=S,3=W,4=NE,5=SE,6=SW,7=NW)をラジアンに変換
pub fn facing_to_angle(facing: u8) -> f32 {
    use std::f32::consts::{FRAC_PI_2, PI, FRAC_PI_4};
    match facing {
        0 => -FRAC_PI_2,          // North
        1 => 0.0,                  // East
        2 => FRAC_PI_2,            // South
        3 => PI,                   // West
        4 => -FRAC_PI_4,           // NE
        5 => FRAC_PI_4,            // SE
        6 => PI + FRAC_PI_4,       // SW
        7 => PI - FRAC_PI_4,       // NW
        _ => FRAC_PI_2,            // default South
    }
}

/// 角度を最短経路でlerpする
pub fn lerp_angle(from: f32, to: f32, t: f32) -> f32 {
    use std::f32::consts::{PI, TAU};
    let mut da = to - from;
    while da >  PI { da -= TAU; }
    while da < -PI { da += TAU; }
    from + da * t
}
