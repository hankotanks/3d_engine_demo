use cgmath::Point3;

use super::{
    MeshObject, 
    MeshObjectData, 
    private, 
    Emitter
};

use crate::Vertex;

pub struct Cube {
    pub(crate) position: Point3<isize>,
    pub(crate) hw: f32,
    pub(crate) color: [f32; 3],
    pub(crate) emitter: Option<Emitter>
}

impl Default for Cube {
    fn default() -> Self {
        Self { 
            position: [0, 0, 0].into(), 
            hw: 0.5,
            color: [0.3, 0.3, 0.8],
            emitter: None
        }
    }
}

impl Cube {
    pub(crate) const FRONT: [f32; 3] = [0.0, 0.0, 1.0];
    pub(crate) const BACK: [f32; 3] = [0.0, 0.0, -1.0];
    pub(crate) const LEFT: [f32; 3] = [-1.0, 0.0, 0.0];
    pub(crate) const RIGHT: [f32; 3] = [1.0, -0.0, 0.0];
    pub(crate) const TOP: [f32; 3] = [0.0, 1.0, 0.0];
    pub(crate) const BOTTOM: [f32; 3] = [-0.0, -1.0, -0.0];
}

impl Cube {
    pub fn new(position: Point3<isize>, color: [f32; 3]) -> Self {
        Self { position, hw: 0.5, color, emitter: None }
    }
}

impl MeshObject for Cube {
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

    fn emitter(&self) -> Option<Emitter> {
        self.emitter
    }

    fn set_emitter(&mut self, emitter: Option<Emitter>) {
        self.emitter = emitter;
    }
}

impl private::MeshObject for Cube {
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

        let normals = if self.emitter.is_none() {
            [
                Self::FRONT, 
                Self::BACK, 
                Self::LEFT, 
                Self::RIGHT, 
                Self::TOP, 
                Self::BOTTOM
            ]
        } else {
            [
                Self::BACK,
                Self::FRONT,
                Self::RIGHT,
                Self::LEFT,
                Self::BOTTOM,
                Self::TOP
            ]
        };

        let vertices = vec![
            // front
            Vertex { position: positions[0], color: self.color, normal: normals[0] },
            Vertex { position: positions[2], color: self.color, normal: normals[0] },
            Vertex { position: positions[1], color: self.color, normal: normals[0] },
            Vertex { position: positions[3], color: self.color, normal: normals[0] },

            // back
            Vertex { position: positions[4], color: self.color, normal: normals[1] },
            Vertex { position: positions[6], color: self.color, normal: normals[1] },
            Vertex { position: positions[5], color: self.color, normal: normals[1] },
            Vertex { position: positions[7], color: self.color, normal: normals[1] },

            // left
            Vertex { position: positions[4], color: self.color, normal: normals[2] },
            Vertex { position: positions[5], color: self.color, normal: normals[2] },
            Vertex { position: positions[0], color: self.color, normal: normals[2] },
            Vertex { position: positions[1], color: self.color, normal: normals[2] },

            // right
            Vertex { position: positions[6], color: self.color, normal: normals[3] },
            Vertex { position: positions[7], color: self.color, normal: normals[3] },
            Vertex { position: positions[2], color: self.color, normal: normals[3] },
            Vertex { position: positions[3], color: self.color, normal: normals[3] },

            // top
            Vertex { position: positions[5], color: self.color, normal: normals[4] },
            Vertex { position: positions[1], color: self.color, normal: normals[4] },
            Vertex { position: positions[7], color: self.color, normal: normals[4] },
            Vertex { position: positions[3], color: self.color, normal: normals[4] },

            // bottom
            Vertex { position: positions[4], color: self.color, normal: normals[5] },
            Vertex { position: positions[0], color: self.color, normal: normals[5] },
            Vertex { position: positions[6], color: self.color, normal: normals[5] },
            Vertex { position: positions[2], color: self.color, normal: normals[5] }
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
}