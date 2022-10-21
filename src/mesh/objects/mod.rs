pub mod cube;
pub use cube::Cube;

mod light_point;
pub use light_point::LightPoint;

use cgmath::Point3;

use crate::vertex::Vertex;

pub trait MeshObject {
    fn build_object_data(&self) -> MeshObjectData;

    fn color(&self) -> [f32; 3];
    fn set_color(&mut self, color: [f32; 3]);

    fn position(&self) -> Point3<isize>;
    fn set_position(&mut self, position: Point3<isize>);

    fn emission(&self) -> Option<[f32; 4]>;
}

pub struct MeshObjectData {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>
}