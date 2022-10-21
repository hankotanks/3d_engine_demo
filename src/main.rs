use block_engine_wgpu::{
    mesh,
    SceneConfig,
    run
};

fn test_mesh_initialize(mesh: &mut mesh::Mesh) {
    mesh.add(
        mesh::objects::Cube::new([0, 0, 0].into(), [0.3, 0.3, 0.8])
    );

    mesh.add(
        mesh::objects::LightPoint::new([0, 2, 0].into(), [1.0, 1.0, 1.0])
    );
}

fn test_mesh_update(_mesh: &mut mesh::Mesh) {
    // let color = mesh.objects[0].color();
    // mesh.objects[0].set_color([color[1], color[2], color[0]]);
}

fn main() {
    let config = SceneConfig { frame_speed: 0.01 };
    pollster::block_on(
        run(config, test_mesh_initialize, test_mesh_update)
    );
}
