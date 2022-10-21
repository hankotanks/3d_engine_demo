use cgmath::Point3;

use super::{
    MeshObject, 
    MeshObjectData, Cube
};

pub struct LightPoint {
    pub position: Point3<isize>,
    pub color: [f32; 3],
    pub emission_strength: f32
}

impl MeshObject for LightPoint {
    fn build_object_data(&self) -> MeshObjectData {
        let mut temp = Cube::default();
        temp.position = self.position;
        temp.hw *= 0.5;
        temp.color = self.color;
        
        let mut data = temp.build_object_data();
        for i in 0..4 {
            data.vertices[i].normal = Cube::BACK;
        }

        for i in 4..8 {
            data.vertices[i].normal = Cube::FRONT;
        }

        for i in 8..12 {
            data.vertices[i].normal = Cube::RIGHT;
        }

        for i in 12..16 {
            data.vertices[i].normal = Cube::LEFT;
        }

        for i in 16..20 {
            data.vertices[i].normal = Cube::BOTTOM;
        }

        for i in 20..24 {
            data.vertices[i].normal = Cube::TOP;
        }

        data
    }

    fn color(&self) -> [f32; 3] {
        self.color
    }

    fn set_color(&mut self, color: [f32; 3]) {
        self.color = color;
    }

    fn position(&self) -> Point3<isize> {
        self.position
    }

    fn set_position(&mut self, position: Point3<isize>) {
        self.position = position;
    }

    fn emission(&self) -> Option<[f32; 4]> {
        Some([
            self.color[0], 
            self.color[1], 
            self.color[2], 
            self.emission_strength
        ])
    }
}