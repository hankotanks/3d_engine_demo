mod cube;

pub use cube::Cube;

use crate::vertex::Vertex;

pub trait MeshObject {
    fn data(&self) -> MeshObjectData;
}

pub struct MeshObjectData {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>
}