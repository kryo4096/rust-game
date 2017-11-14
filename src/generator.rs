use mesh::*;
use noise::*;

pub fn generate_plane(width: u32, depth: u32, scale: f32) -> Mesh{

    let mut mesh = Mesh::empty();

    let noise = Fbm::new();

    for x in 0..width {
        for z in 0..depth {
            let xf = x as f32;
            let zf = z as f32;

            mesh.push_vertex(Vertex{position:[xf * scale, noise.get([xf / 100. / scale, zf / 100. / scale]).max(0.) * 100., zf * scale]});

            if x < width -1 && z < depth - 1 {

                let q = z * width + x;

                mesh.push_index(q);
                mesh.push_index(q+width);
                mesh.push_index(q+width+1);
                mesh.push_index(q);
                mesh.push_index(q+1);
                mesh.push_index(q+width+1);
            }
        }
    }

    mesh
}
