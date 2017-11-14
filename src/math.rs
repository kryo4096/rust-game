extern crate cgmath;

use self::cgmath::*;

pub use self::cgmath::Rad;
pub use self::cgmath::Deg;

pub use self::cgmath::prelude::*;

pub type Vec3 = Vector3<f32>;
pub type Mat4 = Matrix4<f32>;
pub type Perspective = PerspectiveFov<f32>;
