use block_engine_wgpu::{
    mesh,
    SceneConfig,
    run
};

fn test_mesh_init(mesh: &mut mesh::Mesh) {
    mesh.add(mesh::objects::LightPoint::new(
        [0, 3, 0].into(), 
        [1.0, 1.0, 1.0]
    ));

    for i in 0..49 {
        mesh.add(mesh::objects::Cube::new(
            [i / 7 - 3, 0, i % 7 - 3].into(), 
            [0.3, 0.3, 0.8]
        ));
    }
}

fn test_mesh_update(mesh: &mut mesh::Mesh) {
    let mut light_pos = mesh[0].position();

    if light_pos.y == 1 {
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
