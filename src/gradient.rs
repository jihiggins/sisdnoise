use super::interpolation::{interpolate_2d, interpolate_3d};
use super::{interpolation, NoiseHash};
use glam::{Vec2, Vec3};

pub fn cubic_noise_3d(p: Vec3) -> (f32, Vec3) {
    noise_3d(p, interpolation::cubic_interpolation_derivatives)
}

pub fn quintic_noise_3d(p: Vec3) -> (f32, Vec3) {
    noise_3d(p, interpolation::cubic_interpolation_derivatives)
}

pub fn cubic_noise_2d(p: Vec2) -> (f32, Vec2) {
    noise_2d(p, interpolation::cubic_interpolation_derivatives)
}

pub fn quintic_noise_2d(p: Vec2) -> (f32, Vec2) {
    noise_2d(p, interpolation::cubic_interpolation_derivatives)
}

pub fn noise_2d(p: Vec2, u_du: fn(Vec2) -> (Vec2, Vec2)) -> (f32, Vec2) {
    let i = p.floor();
    let w = p.fract();

    let (u, du) = u_du(w);

    let ga = (i + Vec2::new(0.0, 0.0)).get_vhash();
    let gb = (i + Vec2::new(1.0, 0.0)).get_vhash();
    let gc = (i + Vec2::new(0.0, 1.0)).get_vhash();
    let gd = (i + Vec2::new(1.0, 1.0)).get_vhash();

    let a = ga.dot(w - Vec2::new(0.0, 0.0));
    let b = gb.dot(w - Vec2::new(1.0, 0.0));
    let c = gc.dot(w - Vec2::new(0.0, 1.0));
    let d = gd.dot(w - Vec2::new(1.0, 1.0));

    interpolate_2d(a, b, c, d, u, du)
}

pub fn noise_3d(p: Vec3, u_du: fn(Vec3) -> (Vec3, Vec3)) -> (f32, Vec3) {
    let i = p.floor();
    let w = p.fract();

    let (u, du) = u_du(w);

    let ga = (i + Vec3::new(0.0, 0.0, 0.0)).get_vhash();
    let gb = (i + Vec3::new(1.0, 0.0, 0.0)).get_vhash();
    let gc = (i + Vec3::new(0.0, 1.0, 0.0)).get_vhash();
    let gd = (i + Vec3::new(1.0, 1.0, 0.0)).get_vhash();
    let ge = (i + Vec3::new(0.0, 0.0, 1.0)).get_vhash();
    let gf = (i + Vec3::new(1.0, 0.0, 1.0)).get_vhash();
    let gg = (i + Vec3::new(0.0, 1.0, 1.0)).get_vhash();
    let gh = (i + Vec3::new(1.0, 1.0, 1.0)).get_vhash();

    let a = ga.dot(w - Vec3::new(0.0, 0.0, 0.0));
    let b = gb.dot(w - Vec3::new(1.0, 0.0, 0.0));
    let c = gc.dot(w - Vec3::new(0.0, 1.0, 0.0));
    let d = gd.dot(w - Vec3::new(1.0, 1.0, 0.0));
    let e = ge.dot(w - Vec3::new(0.0, 0.0, 1.0));
    let f = gf.dot(w - Vec3::new(1.0, 0.0, 1.0));
    let g = gg.dot(w - Vec3::new(0.0, 1.0, 1.0));
    let h = gh.dot(w - Vec3::new(1.0, 1.0, 1.0));

    interpolate_3d(a, b, c, d, e, f, g, h, u, du)
}
