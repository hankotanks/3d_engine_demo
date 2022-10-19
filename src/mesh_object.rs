
use vertex::Vertex;

enum ObjectShape {
    Cube
}

struct Object {
    shape: ObjectShape,
    color: [f32; 3]
}

struct ObjectData {
    vertices: Vec<Vertex>,
    indices: Vec<u16>
}

impl Object {
    pub fn data(&self, pos: Point3<isize>) -> ObjectData {
        let mut data = ObjectData { 
            vertices: Vec::new(), 
            indices: Vec::new() 
        };

        match self {
            Self::Cube => {
                let positions: [[f32; 3]; 8] = [
                    [ pos.x - 0.5, pos.y - 0.5, pos.z + 0.5 ],
                    [ pos.x + 0.5, pos.y - 0.5, pos.z + 0.5 ],
                    [ pos.x - 0.5, pos.y + 0.5, pos.z + 0.5 ],
                    [ pos.x + 0.5, pos.y + 0.5, pos.z + 0.5 ],
                    [ pos.x - 0.5, pos.y - 0.5, pos.z - 0.5 ],
                    [ pos.x + 0.5, pos.y - 0.5, pos.z - 0.5 ],
                    [ pos.x - 0.5, pos.y + 0.5, pos.z - 0.5 ],
                    [ pos.x + 0.5, pos.y + 0.5, pos.z - 0.5 ],
                ];

                for pt in positions.iter() {
                    data.vertices.push(
                        Vertex { position: pt, color: self.color }
                    );
                }

                data.indices = vec![
                    0, 1, 3, 0, 3, 2,
                    4, 5, 7, 4, 7, 6,
                    0, 2, 6, 0, 6, 4,
                    3, 1, 5, 3, 5, 7,
                    1, 0, 4, 1, 4, 5, 
                    2, 3, 7, 2, 7, 6

                ];
            }
        }
    }
}