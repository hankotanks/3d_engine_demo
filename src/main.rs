use block_engine_wgpu::{
    mesh,
    SceneConfig,
    run
};

fn test_mesh_init(mesh: &mut mesh::Mesh) {
    mesh.add(mesh::objects::Cube::new(
        [0, 6, 0].into(), 
        [1.0, 1.0, 1.0]
    ));

    mesh[0].set_emitter(Some([1.0, 1.0, 1.0, 0.8].into()));

    mesh.add(mesh::objects::Cube::new(
        [3, 2, 3].into(), 
        [1.0, 1.0, 0.1]
    ));

    mesh[1].set_emitter(Some([0.0, 1.0, 0.0, 0.8].into()));

    for i in 0..2500 {
        mesh.add(mesh::objects::Cube::new(
            [i / 50 - 25, 0, i % 50 - 25].into(), 
            [0.3, 0.3, 0.8]
        ));
    }
}

fn test_mesh_update(mesh: &mut mesh::Mesh) {
    let mut light_pos = mesh[0].position();

    if light_pos.y == 6 {
        light_pos.y += 1;
    } else {
        light_pos.y -= 1;
    }

    mesh[0].set_position(light_pos);
}

fn main() {
    let config = SceneConfig { frame_speed: 0.005 };
    pollster::block_on(
        run(config, test_mesh_init, test_mesh_update)
    );
}
