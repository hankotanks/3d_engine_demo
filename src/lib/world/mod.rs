pub(crate) mod drawable;
pub use drawable::{ Drawable, Triangles };

pub(crate) mod tile;
pub use tile::Tile;

pub(crate) mod entity;
pub use entity::Entity;

use crate::vertex::Vertex;

use std::collections::HashMap;

use cgmath::Point3;

#[derive(Default)]
pub struct World {
    pub(crate) tiles: HashMap<Point3<i16>, Box<dyn Tile>>,
    pub(crate) vertices: Vec<Vertex>,
    pub(crate) indices: Vec<u32>
}

impl World {
    pub fn add(&mut self, tile: impl Tile + 'static) {
        let mut triangles = tile.build_object_data();

        self.tiles.insert(tile.position(), Box::new(tile));

        let mut offset_indices = triangles.indices
            .iter()
            .map(|i| *i + self.vertices.len() as u32)
            .collect::<Vec<u32>>();

        self.indices.append(&mut offset_indices);
        self.vertices.append(&mut triangles.vertices);
    }

    pub fn contains(&self, tile: &Point3<i16>) -> bool {
        self.tiles.contains_key(tile)
    }

    pub fn is_empty(&self) -> bool {
        self.tiles.is_empty()
    }

    pub fn get(&self, position: Point3<i16>) -> Option<&Box<dyn Tile + 'static>> {
        self.tiles.get(&position)
    }

    /*
    pub fn occupied(&self, center: Point3<f32>) -> bool {
        for (.., tile) in self.tiles.iter() {
            let pos = tile.center();
            let x_min = pos.x - 0.5;
            let x_max = pos.x + 0.5;
            let y_min = pos.y - 0.5;
            let y_max = pos.y + 0.5;
            let z_min = pos.z - 0.5;
            let z_max = pos.z + 0.5;

            let xc = x_min < center.x && center.x < x_max;
            let yc = y_min < center.y && center.y < y_max;
            let zc = z_min < center.z && center.z < z_max;
            if xc && yc && zc { 
                return true;
            }
        }
        
        false
    }
    */
}