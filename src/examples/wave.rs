use block_engine_wgpu::{
    mesh::Mesh,
    mesh::objects
};
use rand::Rng;

const DIMENSIONS: (usize, usize) = (51, 51);

pub fn wave_mesh_init(mesh: &mut Mesh) {
    let light = objects::Cube::new(
        [0, 5, 0].into(),
        [1.0, 1.0, 1.0]
    );

    mesh.add(light);
    mesh[0].set_emitter(Some([1.0, 1.0, 1.0, 1.0].into()));

    for x in (DIMENSIONS.0 as isize / -2)..(DIMENSIONS.0 as isize / 2) {
        for z in (DIMENSIONS.1 as isize / -2)..(DIMENSIONS.1 as isize / 2) {
            let y = rand::thread_rng().gen_range(-1isize..=1);
            mesh.add(objects::Cube::new(
                [x, y, z].into(),
                match y {
                    -1 => [1.0, 0.0, 0.0],
                     0 => [0.0, 1.0, 0.0],
                     1 => [0.0, 0.0, 1.0],
                     _ => unreachable!()
                }
            ));
        }
    }
}

pub fn wave_mesh_update(_mesh: &mut Mesh) {

}