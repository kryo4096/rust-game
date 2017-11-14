use mesh::*;
use noise::*;
use math::*;

pub const CHUNK_SIZE : u32 = 256;

pub type HeightMap = [[f32; CHUNK_SIZE as usize]; CHUNK_SIZE as usize];

pub struct Chunk {
    chunk_x: i32,
    chunk_z: i32,
    heightmap: HeightMap,
    mesh: Mesh,
}

impl Chunk {

    pub fn generate(chunk_x: i32, chunk_z: i32, heightmap: HeightMap) -> Self {

        let mut chunk = Self {chunk_x, chunk_z, heightmap, mesh: Mesh::empty()};
        chunk.build_mesh();
        chunk

    }

    fn build_mesh(&mut self) {

        self.mesh = Mesh::empty();

        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                self.mesh.push_vertex( Vertex{
                    position: [
                        x as f32,
                        self.heightmap[x as usize][z as usize],
                        z as f32
                    ],
                    normal: [0.0,0.0,0.0],
                });

                if x < CHUNK_SIZE -1 && z < CHUNK_SIZE - 1 {

                    let q = z * CHUNK_SIZE + x;

                    self.mesh.push_index(q);
                    self.mesh.push_index(q+CHUNK_SIZE);
                    self.mesh.push_index(q+CHUNK_SIZE+1);
                    self.mesh.push_index(q);
                    self.mesh.push_index(q+1);
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

pub struct HeightMapGen {
    noise  : Fbm<f32>,
}

impl HeightMapGen {

    pub fn new() -> Self{

        let noise = Fbm::new();

        let noise = noise.set_frequency(0.01);

        Self {noise}

    }

    pub fn generate_heightmap(&self, chunk_x: i32, chunk_z: i32) -> HeightMap {

        let mut heightmap = [[0.;CHUNK_SIZE as usize];CHUNK_SIZE as usize];

        for x in 0..CHUNK_SIZE as usize {
            for z in 0..CHUNK_SIZE as usize {
                let probe_x = chunk_x as f32 * CHUNK_SIZE as f32 + x as f32;
                let probe_z = chunk_z as f32 * CHUNK_SIZE as f32 + z as f32;

                heightmap[x][z] = self.noise.get([probe_x, probe_z]).max(0.0)*100.;
            }
        }

        heightmap

    }

}
