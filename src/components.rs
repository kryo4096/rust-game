use math::*;
use specs::*;
use mesh::*;

#[derive(Copy, Clone, Component)]
#[component(VecStorage)]
pub struct Transform {
    pos: Vec3,
    scale: f32,
    rot: Quat,
}

impl Transform {

    pub fn new() -> Self{
        Self {pos: Vec3::zero(), scale: 1., rot: Quat::zero()}
    }

    pub fn mov(&mut self, v: Vec3) {

        self.pos += v;

    }

    pub fn mov_to(&mut self, v: Vec3) {

        self.pos = v;

    }

    pub fn rot(&mut self, q: Quat) {

        self.rot = q * self.rot;

    }

    pub fn rot_to(&mut self, q: Quat) {

        self.rot = q;

    }

    pub fn rot_axes(&mut self, x: f32, y: f32, z: f32) {

        let rot = Quat::from_angle_x(Deg(x)) * Quat::from_angle_x(Deg(x)) * Quat::from_angle_x(Deg(x));
        self.rot = rot * self.rot;

    }

    pub fn rot_to_axes(&mut self, x: f32, y: f32, z: f32) {

        let rot = Quat::from_angle_x(Deg(x)) * Quat::from_angle_x(Deg(x)) * Quat::from_angle_x(Deg(x));

        self.rot = rot;

    }

    pub fn mov_rel(&mut self, v: Vec3) {

        self.pos += self.rot * v;

    }

    pub fn scale(&mut self, s: f32) {
        self.scale *= s;
    }

    pub fn set_scale(&mut self, s: f32) {
        self.scale = s;
    }

    pub fn model_m(&self) -> Mat4 {
        let scale_m = Mat4::from_scale(self.scale);
        let rotation_m = Mat4::from(self.rot);
        let translation_m = Mat4::from_translation(self.pos);

        translation_m * rotation_m * scale_m
    }

}

pub struct Camera {
    pub perspective: Perspective,
}

#[derive(Copy, Clone, Component)]
#[component(VecStorage)]
impl Camera {
    pub fn perspective_m(&self) -> Mat4 {
        self.perspective.into()
    }
}


#[derive(Copy, Clone, Component)]
#[component(VecStorage)]
pub struct MeshCarrier {
    pub mesh: Mesh,
}
