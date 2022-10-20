
use cgmath::{Point3, Vector3, InnerSpace};

use crate::vertex::Vertex;

pub enum ObjectShape {
    Cube
}

pub struct Object {
    pub shape: ObjectShape,
    pub color: [f32; 3]
}

pub struct ObjectData {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>
}

impl Object {
    pub fn data(&self, pos: Point3<isize>) -> ObjectData {
        let mut data = ObjectData { 
            vertices: Vec::new(), 
            indices: Vec::new() 
        };

        match self.shape {
            ObjectShape::Cube => {
                let positions = Point3::new(pos.x as f32, pos.y as f32, pos.z as f32);
                let positions: [[f32; 3]; 8] = [
                    [ positions.x - 0.5, positions.y - 0.5, positions.z + 0.5 ],
                    [ positions.x - 0.5, positions.y + 0.5, positions.z + 0.5 ],
                    [ positions.x + 0.5, positions.y - 0.5, positions.z + 0.5 ],
                    [ positions.x + 0.5, positions.y + 0.5, positions.z + 0.5 ],
                    [ positions.x - 0.5, positions.y - 0.5, positions.z - 0.5 ],
                    [ positions.x - 0.5, positions.y + 0.5, positions.z - 0.5 ],
                    [ positions.x + 0.5, positions.y - 0.5, positions.z - 0.5 ],
                    [ positions.x + 0.5, positions.y + 0.5, positions.z - 0.5 ],
                ];
 
                let mut points = Vec::new();
                for point in positions.iter() { 
                    points.push(
                        Point3::new(point[0], point[1], point[2])
                    );
                }

                let normals: [Vector3<f32>; 6] = [
                    (points[1] - points[2]).cross(points[0] - points[2]).normalize(), //front
                    (points[6] - points[5]).cross(points[4] - points[5]).normalize(), //back
                    (points[1] - points[0]).cross(points[4] - points[0]).normalize(), //left
                    (points[6] - points[2]).cross(points[3] - points[2]).normalize(), //right
                    (points[7] - points[3]).cross(points[1] - points[3]).normalize(), //top
                    (points[0] - points[2]).cross(points[6] - points[2]).normalize(), //bottom
                    
                ];

                let normals: [[f32; 3]; 6] = [
                    [ normals[0].x, normals[0].y, normals[0].z ],
                    [ normals[1].x, normals[1].y, normals[1].z ],
                    [ normals[2].x, normals[2].y, normals[2].z ],
                    [ normals[3].x, normals[3].y, normals[3].z ],
                    [ normals[4].x, normals[4].y, normals[4].z ],
                    [ normals[5].x, normals[5].y, normals[5].z ],
                ];

                data.vertices.append(&mut vec![
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
                    Vertex { position: positions[2], color: self.color, normal: normals[5] },
                ]);

                data.indices = vec![
                    0, 1, 3, 0, 3, 2,
                    7, 5, 4, 7, 4, 6,
                    11, 9, 8, 11, 8, 10,
                    12, 13, 15, 12, 15, 14,
                    16, 17, 19, 16, 19, 18,
                    23, 21, 20, 23, 20, 22
                ];
            }
        }

        data
    }
}