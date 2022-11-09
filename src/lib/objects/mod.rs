mod cube;
pub(crate) use cube::Cube;

mod gap;
pub(crate) use gap::Gap;

use crate::Vertex;

pub struct MeshObjectData {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>
}

pub trait MeshObject: private::MeshObject {
    fn color(&self) -> [f32; 3];
    fn set_color(&mut self, color: [f32; 3]);

    fn position(&self) -> cgmath::Point3<i16>;
    fn set_position(&mut self, position: cgmath::Point3<i16>);

    fn light(&self) -> Option<[f32; 4]>;
    fn set_light(&mut self, light: Option<[f32; 4]>);
}

pub(crate) mod private {
    use super::MeshObjectData;

    pub trait MeshObject {
        fn build_object_data(&self) -> MeshObjectData;
    }
}