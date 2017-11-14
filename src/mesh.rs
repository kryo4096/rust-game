use glium::{VertexBuffer, IndexBuffer, Display, index};

#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: [f32;3],
    pub normal: [f32;3],
}

implement_vertex!(Vertex, position);

pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

impl Mesh {

    pub fn empty() -> Self {
        Mesh{vertices: Vec::new(), indices: Vec::new()}
    }

    pub fn vertex_buffer(&self, display: &Display) -> VertexBuffer<Vertex> {
        VertexBuffer::new(display, &self.vertices).unwrap()
    }

    pub fn index_buffer(&self, display: &Display) -> IndexBuffer<u32> {
        IndexBuffer::new(display, index::PrimitiveType::TrianglesList, &self.indices).unwrap()
    }

    pub fn push_vertex(&mut self, v: Vertex) {
        self.vertices.push(v);
    }

    pub fn push_index(&mut self, i: u32) {
        self.indices.push(i);
    }

    pub fn calculate_normals(&mut self) {


    }

}
