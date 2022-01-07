use glam::{Vec2, Vec3};
use std::ops::{Add, Mul, Sub};

pub fn cubic_interpolation_derivatives<
    T: Mul<Output = T> + Mul<f32, Output = T> + Add<f32, Output = T> + Copy,
>(
    w: T,
) -> (T, T) {
    (
        // u
        (w * w * (w * -2.0 + 3.0)),
        // du
        (w * (w * -1.0 + 1.0) * 6.0),
    )
}

pub fn quintic_interpolation_derivatives<
    T: Mul<Output = T>
        + Mul<f32, Output = T>
        + Add<f32, Output = T>
        + Sub<f32, Output = T>
        + Copy,
>(
    w: T,
) -> (T, T) {
    (
        // u
        (w * w * w * (w * (w * 6.0 - 15.0) + 10.0)),
        // du
        (w * w * (w * (w - 2.0) + 1.0) * 30.0),
    )
}

pub fn interpolate_2d(
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    u: Vec2,
    du: Vec2,
) -> (f32, Vec2) {
    let k0 = a;
    let k1 = b - a;
    let k2 = c - a;
    let k4 = a - b - c + d;

    let value = (k0 + k1 * u.x + k2 * u.y + k4 * u.x * u.y) * 2.0 - 1.0;
    let derivatives = Vec2::new(k1 + k4 * u.y, k2 + k4 * u.x) * 2.0 * du;
    (value, derivatives)
}

pub fn interpolate_3d(
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    e: f32,
    f: f32,
    g: f32,
    h: f32,
    u: Vec3,
    du: Vec3,
) -> (f32, Vec3) {
    let k0 = a;
    let k1 = b - a;
    let k2 = c - a;
    let k3 = e - a;
    let k4 = a - b - c + d;
    let k5 = a - c - e + g;
    let k6 = a - b - e + f;
    let k7 = -a + b + c - d + e - f - g + h;

    let value = k0
        + k1 * u.x
        + k2 * u.y
        + k3 * u.z
        + k4 * u.x * u.y
        + k5 * u.y * u.z
        + k6 * u.z * u.x
        + k7 * u.x * u.y * u.z;
    let derivatives = Vec3::new(
        k1 + k4 * u.y + k6 * u.z + k7 * u.y * u.z,
        k2 + k5 * u.z + k4 * u.x + k7 * u.z * u.x,
        k3 + k6 * u.x + k5 * u.y + k7 * u.x * u.y,
    ) * du;
    (value, derivatives)
}
