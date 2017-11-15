use mesh::*;
use noise::*;
use math::*;

pub const CHUNK_SIZE : u32 = 256;

pub struct Chunk {
    chunk_x: i32,
    chunk_z: i32,
    mesh: Mesh,
    noise: TerrainNoise,
}

impl Chunk {

    pub fn generate(chunk_x: i32, chunk_z: i32, noise: TerrainNoise) -> Self {

        let mut chunk = Self {chunk_x, chunk_z, noise, mesh: Mesh::empty()};
        chunk.build_mesh();
        chunk

    }

    fn build_mesh(&mut self) {

        self.mesh = Mesh::empty();

        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {

                let normal;
                let this;

                {
                    let x = x as i32;
                    let z = z as i32;

                    this = self.noise.get(x,z);
                    let left = self.noise.get(x-1,z);
                    let right = self.noise.get(x+1,z);
                    let front = self.noise.get(x,z -1);
                    let back = self.noise.get(x, z + 1);

                    normal = Vec3::new(left-right,2.,front-back).normalize();

                }

                self.mesh.push_vertex( Vertex{
                    position: [
                        x as f32,
                        this,
                        z as f32
                    ],
                    normal: normal.into(),
                });

                if x < CHUNK_SIZE -1 && z < CHUNK_SIZE - 1 {

                    let q = z * CHUNK_SIZE + x;

                    self.mesh.push_index(q);
                    self.mesh.push_index(q+CHUNK_SIZE);
                    self.mesh.push_index(q+CHUNK_SIZE+1);
                    self.mesh.push_index(q+1);
                    self.mesh.push_index(q);
                    self.mesh.push_index(q+CHUNK_SIZE+1);
                }
            }
        }
    }

    pub fn mesh(&self) -> &Mesh {
        &self.mesh
    }

    pub fn model_m(&self) -> Mat4{
        Mat4::from_translation(Vec3::new(self.chunk_x as f32 * CHUNK_SIZE as f32, 0., self.chunk_z as f32 * CHUNK_SIZE as f32 ))
    }

}

pub struct TerrainNoise {
    noise  : Fbm<f32>,
}

impl TerrainNoise {

    pub fn new() -> Self{

        let noise = Fbm::new();

        let noise = noise.set_frequency(0.01);

        Self {noise}

    }

    pub fn get(&self, x: i32, z: i32) -> f32 {

        self.noise.get([x as f32, z as f32]).max(0.0)*100.

    }

}
