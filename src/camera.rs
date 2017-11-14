use math::*;

/// Computes the View Matrix for a first person shooter-type camera
pub struct FirstPersonCamera {
    pub angle_x: f32,
    pub angle_y: f32,
    pub position: Vec3,
}

impl FirstPersonCamera {
    pub fn new(position: Vec3) -> Self {
        FirstPersonCamera{ angle_x: 0., angle_y: 0. , position}
    }

    pub fn get_matrix(&self) -> Mat4 {
        let rotation_x = Mat4::from_angle_x(Deg(-self.angle_y));
        let rotation_y = Mat4::from_angle_y(Deg(-self.angle_x));
        let translation = Mat4::from_translation(-self.position);

        rotation_x * rotation_y * translation
    }

    pub fn walk (&mut self, amount: Vec3){

        let rotation = Mat4::from_angle_y(Deg(self.angle_x));

        self.position += rotation.transform_vector(amount);

    }
}
