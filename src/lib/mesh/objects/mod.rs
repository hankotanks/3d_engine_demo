mod cube;
pub use cube::Cube;

use cgmath::Point3;

use crate::Vertex;

pub struct MeshObjectData {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>
}

pub trait MeshObject: private::MeshObject {
    fn color(&self) -> [f32; 3];
    fn set_color(&mut self, color: [f32; 3]);

    fn position(&self) -> Point3<isize>;
    fn set_position(&mut self, position: Point3<isize>);
}

pub(crate) mod private {
    use super::MeshObjectData;

    pub trait MeshObject {
        fn build_object_data(&self) -> MeshObjectData;
    }
}