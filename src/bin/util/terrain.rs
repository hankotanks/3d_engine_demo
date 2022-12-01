use block_engine_wgpu::world;

use crate::util::tile;


pub fn generate(mesh: &mut world::World) {
    let mut height;
    for x in -10i16..10 {
        for y in -10i16..10 {
            height = match (x, y) { (-10 | 9, ..) | (.., -10 | 9) => 1, _ => 0 };
            mesh.add(tile::Cube::new(
                (x, height, y).into(),
                [1.0; 3]
            ));
        } 
    }
}