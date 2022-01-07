use std::ops::Mul;

pub fn fbm<T: Copy + Mul<f32, Output = T>>(
    x: T,
    octaves: u32,
    hurst: f32,
    noise: fn(T) -> f32,
) -> f32 {
    let gain = (-hurst).exp2();
    let mut f = 1.0;
    let mut a = 1.0;
    let mut t = 0.0;
    for _ in 0..octaves {
        t += a * noise(x * f);
        f *= 2.0;
        a *= gain;
    }
    t
}
