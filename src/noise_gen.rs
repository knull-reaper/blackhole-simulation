const PERMUTE_MOD: f32 = 289.0;

pub const NOISE_SIZE: usize = 128;
pub const NOISE_DOMAIN: f32 = 128.0;

#[derive(Clone, Copy)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Clone, Copy)]
struct Vec4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    fn splat(v: f32) -> Self {
        Self { x: v, y: v, z: v }
    }

    fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn floor(self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
            z: self.z.floor(),
        }
    }

    fn min(self, other: Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
        }
    }

    fn max(self, other: Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
        }
    }
}

impl Vec4 {
    fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    fn splat(v: f32) -> Self {
        Self {
            x: v,
            y: v,
            z: v,
            w: v,
        }
    }

    fn floor(self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
            z: self.z.floor(),
            w: self.w.floor(),
        }
    }

    fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
            w: self.w.abs(),
        }
    }

    fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    fn max(self, other: Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
            w: self.w.max(other.w),
        }
    }
}

use std::ops::{Add, Div, Mul, Neg, Sub};

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Add for Vec4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, self.w + rhs.w)
    }
}

impl Sub for Vec4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z, self.w - rhs.w)
    }
}

impl Mul<f32> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl Mul for Vec4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::new(
            self.x * rhs.x,
            self.y * rhs.y,
            self.z * rhs.z,
            self.w * rhs.w,
        )
    }
}

impl Neg for Vec4 {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z, -self.w)
    }
}

fn mod289(v: f32) -> f32 {
    v - (v / PERMUTE_MOD).floor() * PERMUTE_MOD
}

fn mod289_vec3(v: Vec3) -> Vec3 {
    Vec3::new(mod289(v.x), mod289(v.y), mod289(v.z))
}

fn mod289_vec4(v: Vec4) -> Vec4 {
    Vec4::new(mod289(v.x), mod289(v.y), mod289(v.z), mod289(v.w))
}

fn permute(x: Vec4) -> Vec4 {
    mod289_vec4(((x * 34.0) + Vec4::splat(1.0)) * x)
}

fn taylor_inv_sqrt(r: Vec4) -> Vec4 {
    Vec4::splat(1.79284291400159) - r * 0.85373472095314
}

fn step_vec3(edge: Vec3, x: Vec3) -> Vec3 {
    Vec3::new(
        if x.x < edge.x { 0.0 } else { 1.0 },
        if x.y < edge.y { 0.0 } else { 1.0 },
        if x.z < edge.z { 0.0 } else { 1.0 },
    )
}

fn step_vec4(edge: Vec4, x: Vec4) -> Vec4 {
    Vec4::new(
        if x.x < edge.x { 0.0 } else { 1.0 },
        if x.y < edge.y { 0.0 } else { 1.0 },
        if x.z < edge.z { 0.0 } else { 1.0 },
        if x.w < edge.w { 0.0 } else { 1.0 },
    )
}

// Port of Ashima 3D simplex noise for CPU precompute.
fn snoise(v: Vec3) -> f32 {
    const C1: f32 = 1.0 / 6.0;
    const C2: f32 = 1.0 / 3.0;

    let s = (v.x + v.y + v.z) * C2;
    let i = (v + Vec3::splat(s)).floor();
    let t = (i.x + i.y + i.z) * C1;
    let x0 = v - i + Vec3::splat(t);

    let g = step_vec3(Vec3::new(x0.y, x0.z, x0.x), Vec3::new(x0.x, x0.y, x0.z));
    let l = Vec3::splat(1.0) - g;
    let l_zxy = Vec3::new(l.z, l.x, l.y);
    let i1 = g.min(l_zxy);
    let i2 = g.max(l_zxy);

    let x1 = x0 - i1 + Vec3::splat(C1);
    let x2 = x0 - i2 + Vec3::splat(2.0 * C1);
    let x3 = x0 - Vec3::splat(1.0) + Vec3::splat(3.0 * C1);

    let i = mod289_vec3(i);
    let p = permute(
        permute(
            permute(Vec4::new(i.z, i.z + i1.z, i.z + i2.z, i.z + 1.0))
                + Vec4::new(i.y, i.y + i1.y, i.y + i2.y, i.y + 1.0),
        ) + Vec4::new(i.x, i.x + i1.x, i.x + i2.x, i.x + 1.0),
    );

    let n_ = 1.0 / 7.0;
    let ns = Vec3::new(2.0 * n_, 0.5 * n_ - 1.0, n_);

    let j = p - (p * (ns.z * ns.z)).floor() * 49.0;
    let x_ = (j * ns.z).floor();
    let y_ = (j - x_ * 7.0).floor();

    let x = x_ * ns.x + Vec4::splat(ns.y);
    let y = y_ * ns.x + Vec4::splat(ns.y);
    let h = Vec4::splat(1.0) - x.abs() - y.abs();

    let b0 = Vec4::new(x.x, x.y, y.x, y.y);
    let b1 = Vec4::new(x.z, x.w, y.z, y.w);

    let s0 = b0.floor() * 2.0 + Vec4::splat(1.0);
    let s1 = b1.floor() * 2.0 + Vec4::splat(1.0);
    let sh = -step_vec4(h, Vec4::splat(0.0));

    let a0 = Vec4::new(
        b0.x + s0.x * sh.x,
        b0.z + s0.z * sh.x,
        b0.y + s0.y * sh.y,
        b0.w + s0.w * sh.y,
    );
    let a1 = Vec4::new(
        b1.x + s1.x * sh.z,
        b1.z + s1.z * sh.z,
        b1.y + s1.y * sh.w,
        b1.w + s1.w * sh.w,
    );

    let mut p0 = Vec3::new(a0.x, a0.y, h.x);
    let mut p1 = Vec3::new(a0.z, a0.w, h.y);
    let mut p2 = Vec3::new(a1.x, a1.y, h.z);
    let mut p3 = Vec3::new(a1.z, a1.w, h.w);

    let norm = taylor_inv_sqrt(Vec4::new(
        p0.dot(p0),
        p1.dot(p1),
        p2.dot(p2),
        p3.dot(p3),
    ));
    p0 = p0 * norm.x;
    p1 = p1 * norm.y;
    p2 = p2 * norm.z;
    p3 = p3 * norm.w;

    let m = (Vec4::splat(0.6)
        - Vec4::new(
            x0.dot(x0),
            x1.dot(x1),
            x2.dot(x2),
            x3.dot(x3),
        ))
    .max(Vec4::splat(0.0));
    let m2 = m * m;
    let m4 = m2 * m2;

    42.0
        * m4.dot(Vec4::new(
            p0.dot(x0),
            p1.dot(x1),
            p2.dot(x2),
            p3.dot(x3),
        ))
}

pub fn generate_noise_3d() -> Vec<u8> {
    let size = NOISE_SIZE;
    let mut data = Vec::with_capacity(size * size * size);
    let scale = NOISE_DOMAIN / size as f32;

    for z in 0..size {
        let fz = (z as f32 + 0.5) * scale;
        for y in 0..size {
            let fy = (y as f32 + 0.5) * scale;
            for x in 0..size {
                let fx = (x as f32 + 0.5) * scale;
                let n = snoise(Vec3::new(fx, fy, fz));
                let value = ((n * 0.5 + 0.5).clamp(0.0, 1.0) * 255.0).round() as u8;
                data.push(value);
            }
        }
    }

    data
}
