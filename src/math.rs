extern crate cgmath;

use self::cgmath::*;

pub use self::cgmath::Rad;
pub use self::cgmath::Deg;

pub use self::cgmath::prelude::*;

pub type Vec3 = Vector3<f32>;
pub type Mat4 = Matrix4<f32>;
pub type Perspective = PerspectiveFov<f32>;
pub type Quat = Quaternion<f32>;

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {

    let t = t.max(0.0).min(1.0);

    a+(b-a) * t

}
