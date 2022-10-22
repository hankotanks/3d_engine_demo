mod cube;
pub use cube::Cube;

use cgmath::Point3;

use crate::Vertex;

pub struct MeshObjectData {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>
}

#[derive(Clone, Copy)]
pub struct Emitter([f32; 4]);

impl Emitter {
    pub fn get(&self) -> [f32; 4] {
        self.0
    }

    pub fn set_color(&mut self, color: [f32; 3]) {
        self.0[0] = color[0];
        self.0[1] = color[1];
        self.0[2] = color[2];
    }

    pub fn set_strength(&mut self, strength: f32) {
        self.0[3] = strength;
    }
}

impl Default for Emitter {
    fn default() -> Self {
        Self([1.0, 1.0, 1.0, 0.01])
    }
}

impl From<[f32; 4]> for Emitter {
    fn from(item: [f32; 4]) -> Self {
        Self(item)
    }
}

pub trait MeshObject: private::MeshObject {
    fn color(&self) -> [f32; 3];
    fn set_color(&mut self, color: [f32; 3]);

    fn position(&self) -> Point3<isize>;
    fn set_position(&mut self, position: Point3<isize>);

    fn emitter(&self) -> Option<Emitter>;
    fn set_emitter(&mut self, emitter: Option<Emitter>);
}

pub(crate) mod private {
    use super::MeshObjectData;

    pub trait MeshObject {
        fn build_object_data(&self) -> MeshObjectData;
    }
}