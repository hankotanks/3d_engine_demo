use std::collections::HashMap;

use cgmath::Point2;

struct Mesh {
    objects: HashMap<Point3<isize>, Object>
}

impl Default for Mesh {
    fn default() -> Self {
        Self {
            objects: {
                let mut objects = HashMap::new();
                
                objects.insert(
                    Point3::new(0, 0, 0),
                    Object {
                        shape: ObjectShape::Cube,
                        color: [1.0, 1.0, 1.0]
                    }
                );

                objects
            }
        }
    }
}