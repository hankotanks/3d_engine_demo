use cgmath::Point3;

use super::{
    MeshObject, 
    MeshObjectData
};

use crate::vertex::Vertex;

pub struct Cube {
    pub position: Point3<isize>,
    pub hw: f32,
    pub color: [f32; 3]    
}

impl Default for Cube {
    fn default() -> Self {
        Self { 
            position: [0, 0, 0].into(), 
            hw: 0.5,
            color: [0.3, 0.3, 0.8],
        }
    }
}

impl Cube {
    pub const FRONT: [f32; 3] = [0.0, 0.0, 1.0];
    pub const BACK: [f32; 3] = [0.0, 0.0, -1.0];
    pub const LEFT: [f32; 3] = [-1.0, 0.0, 0.0];
    pub const RIGHT: [f32; 3] = [1.0, -0.0, 0.0];
    pub const TOP: [f32; 3] = [0.0, 1.0, 0.0];
    pub const BOTTOM: [f32; 3] = [-0.0, -1.0, -0.0];
}

impl MeshObject for Cube {
    fn build_object_data(&self) -> MeshObjectData {
        let center = Point3::new(
            self.position.x as f32, 
            self.position.y as f32, 
            self.position.z as f32
        );

        let positions: [[f32; 3]; 8] = [
            [ center.x - self.hw, center.y - self.hw, center.z + self.hw ],
            [ center.x - self.hw, center.y + self.hw, center.z + self.hw ],
            [ center.x + self.hw, center.y - self.hw, center.z + self.hw ],
            [ center.x + self.hw, center.y + self.hw, center.z + self.hw ],
            [ center.x - self.hw, center.y - self.hw, center.z - self.hw ],
            [ center.x - self.hw, center.y + self.hw, center.z - self.hw ],
            [ center.x + self.hw, center.y - self.hw, center.z - self.hw ],
            [ center.x + self.hw, center.y + self.hw, center.z - self.hw ]
        ];

        let vertices = vec![
            // front
            Vertex { position: positions[0], color: self.color, normal: Self::FRONT },
            Vertex { position: positions[2], color: self.color, normal: Self::FRONT },
            Vertex { position: positions[1], color: self.color, normal: Self::FRONT },
            Vertex { position: positions[3], color: self.color, normal: Self::FRONT },

            // back
            Vertex { position: positions[4], color: self.color, normal: Self::BACK },
            Vertex { position: positions[6], color: self.color, normal: Self::BACK },
            Vertex { position: positions[5], color: self.color, normal: Self::BACK },
            Vertex { position: positions[7], color: self.color, normal: Self::BACK },

            // left
            Vertex { position: positions[4], color: self.color, normal: Self::LEFT },
            Vertex { position: positions[5], color: self.color, normal: Self::LEFT },
            Vertex { position: positions[0], color: self.color, normal: Self::LEFT },
            Vertex { position: positions[1], color: self.color, normal: Self::LEFT },

            // right
            Vertex { position: positions[6], color: self.color, normal: Self::RIGHT },
            Vertex { position: positions[7], color: self.color, normal: Self::RIGHT },
            Vertex { position: positions[2], color: self.color, normal: Self::RIGHT },
            Vertex { position: positions[3], color: self.color, normal: Self::RIGHT },

            // top
            Vertex { position: positions[5], color: self.color, normal: Self::TOP },
            Vertex { position: positions[1], color: self.color, normal: Self::TOP },
            Vertex { position: positions[7], color: self.color, normal: Self::TOP },
            Vertex { position: positions[3], color: self.color, normal: Self::TOP },

            // bottom
            Vertex { position: positions[4], color: self.color, normal: Self::BOTTOM },
            Vertex { position: positions[0], color: self.color, normal: Self::BOTTOM },
            Vertex { position: positions[6], color: self.color, normal: Self::BOTTOM },
            Vertex { position: positions[2], color: self.color, normal: Self::BOTTOM }
        ];

        let indices = vec![
            0, 1, 3, 0, 3, 2,
            7, 5, 4, 7, 4, 6,
            11, 9, 8, 11, 8, 10,
            12, 13, 15, 12, 15, 14,
            16, 17, 19, 16, 19, 18,
            23, 21, 20, 23, 20, 22
        ];

        MeshObjectData { vertices, indices }
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
        None
    }
}
