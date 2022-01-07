use glam::{Vec2, Vec3};

pub mod fbm;
pub mod gradient;
pub mod interpolation;
pub mod value;

pub trait NoiseHash {
    fn get_hash(&self) -> f32;
    fn get_vhash(&self) -> Self;
}

const M: u32 = 0x5bd1e995;
const R: u32 = 24;

const SEED: u32 = 0x0;
const SEED_X: u32 = 0x12345678 + SEED;
const SEED_Y: u32 = 0x87654321 + SEED;
const SEED_Z: u32 = 0x13579bdf + SEED;
const UMAX_INV: f32 = 1.0 / (u32::MAX as f32);

fn mmix(h: &mut u32, k: &mut u32) {
    *k = k.overflowing_mul(M).0;
    *k ^= *k >> R;
    *k = k.overflowing_mul(M).0;
    *h = h.overflowing_mul(M).0;
    *h ^= *k;
}

fn murmur(data: &[u8], seed: u32) -> u32 {
    let mut l = data.len() as u32;

    let mut h = seed;
    let mut len = data.len() as u32;

    let mut i = 0;
    while len >= 4 {
        let mut k = data[i] as u32
            | (data[i + 1] as u32) << 8
            | (data[i + 2] as u32) << 16
            | (data[i + 3] as u32) << 24;

        mmix(&mut h, &mut k);

        i += 4;
        len -= 4;
    }

    let mut t: u32 = 0;

    match len {
        3 => t ^= (data[2] as u32) << 16,
        2 => t ^= (data[1] as u32) << 8,
        1 => t ^= data[0] as u32,
        _ => (),
    };

    mmix(&mut h, &mut t);
    mmix(&mut h, &mut l);

    h ^= h >> 13;
    h = h.overflowing_mul(M).0;
    h ^= h >> 15;

    h
}

impl NoiseHash for Vec2 {
    fn get_hash(&self) -> f32 {
        murmur(&vec2_to_bytes(*self), SEED_X) as f32
    }

    fn get_vhash(&self) -> Self {
        let bytes = vec2_to_bytes(*self);
        let x = murmur(&bytes, SEED_X) as f32;
        let y = murmur(&bytes, SEED_Y) as f32;
        Vec2::new(x, y) * UMAX_INV * 2.0 - 1.0
    }
}

impl NoiseHash for Vec3 {
    fn get_hash(&self) -> f32 {
        murmur(&vec3_to_bytes(*self), SEED_X) as f32
    }

    fn get_vhash(&self) -> Self {
        let bytes = vec3_to_bytes(*self);
        let x = murmur(&bytes, SEED_X) as f32;
        let y = murmur(&bytes, SEED_Y) as f32;
        let z = murmur(&bytes, SEED_Z) as f32;
        Vec3::new(x, y, z) * UMAX_INV * 2.0 - 1.0
    }
}

fn vec3_to_bytes(v: Vec3) -> [u8; 12] {
    let mut bytes = [0u8; 12];
    let b1 = v.x.to_ne_bytes();
    let b2 = v.y.to_ne_bytes();
    let b3 = v.z.to_ne_bytes();
    bytes[0..4].copy_from_slice(&b1);
    bytes[4..8].copy_from_slice(&b2);
    bytes[8..12].copy_from_slice(&b3);
    bytes
}

fn vec2_to_bytes(v: Vec2) -> [u8; 8] {
    let mut bytes = [0u8; 8];
    let b1 = v.x.to_ne_bytes();
    let b2 = v.y.to_ne_bytes();
    bytes[0..4].copy_from_slice(&b1);
    bytes[4..8].copy_from_slice(&b2);
    bytes
}
