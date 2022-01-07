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

    let a = (i + Vec2::new(0.0, 0.0)).get_hash();
    let b = (i + Vec2::new(1.0, 0.0)).get_hash();
    let c = (i + Vec2::new(0.0, 1.0)).get_hash();
    let d = (i + Vec2::new(1.0, 1.0)).get_hash();

    interpolate_2d(a, b, c, d, u, du)
}

pub fn noise_3d(p: Vec3, u_du: fn(Vec3) -> (Vec3, Vec3)) -> (f32, Vec3) {
    let i = p.floor();
    let w = p.fract();

    let (u, du) = u_du(w);

    let a = (i + Vec3::new(0.0, 0.0, 0.0)).get_hash();
    let b = (i + Vec3::new(1.0, 0.0, 0.0)).get_hash();
    let c = (i + Vec3::new(0.0, 1.0, 0.0)).get_hash();
    let d = (i + Vec3::new(1.0, 1.0, 0.0)).get_hash();
    let e = (i + Vec3::new(0.0, 0.0, 1.0)).get_hash();
    let f = (i + Vec3::new(1.0, 0.0, 1.0)).get_hash();
    let g = (i + Vec3::new(0.0, 1.0, 1.0)).get_hash();
    let h = (i + Vec3::new(1.0, 1.0, 1.0)).get_hash();

    interpolate_3d(a, b, c, d, e, f, g, h, u, du)
}
