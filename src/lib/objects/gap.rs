use super::{
    private, 
    MeshObject, 
    MeshObjectData
};

use cgmath::Point3;

pub(crate) struct Gap {
    pub(crate) position: Point3<i16>,
    pub(crate) light: Option<[f32; 4]>
}

impl Default for Gap {
    fn default() -> Self {
        Self { 
            position: [0, 0, 0].into(), 
            light: None
        }
    }
}

impl Gap {
    pub fn new(position: Point3<i16>) -> Self {
        Self { position, light: None }
    }
}

impl MeshObject for Gap { 
    fn color(&self) -> [f32; 3] { [0.0; 3] }
    fn set_color(&mut self, _color: [f32; 3]) {  }

    fn position(&self) -> Point3<i16> { self.position }
    fn set_position(&mut self, position: Point3<i16>) { self.position = position; }

    fn light(&self) -> Option<[f32; 4]> { self.light }
    fn set_light(&mut self, light: Option<[f32; 4]>) { self.light = light; }
}

impl private::MeshObject for Gap {
    fn build_object_data(&self) -> MeshObjectData {
        MeshObjectData { vertices: Vec::new(), indices: Vec::new() }
    }
}