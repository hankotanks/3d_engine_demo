use block_engine_wgpu::{
    mesh,
    SceneConfig,
    run
};

fn test_mesh_initialize(mesh: &mut mesh::Mesh) {
    mesh.objects.push(
        Box::new(
            mesh::objects::Cube {
                position: [2, 0, 0].into(),
                hw: 0.5,
                color: [0.3, 0.3, 0.8],
            }
        )
    );

    mesh.objects.push(
        Box::new(
            mesh::objects::Cube {
                position: [0, 0, 1].into(),
                hw: 0.5,
                color: [0.3, 0.3, 0.8],
            }
        )
    );

    mesh.objects.push(
        Box::new(
            mesh::objects::Cube {
                position: [0, 2, 1].into(),
                hw: 0.5,
                color: [0.3, 0.3, 0.8],
            }
        )
    );

    mesh.objects.push(
        Box::new(
            mesh::objects::LightPoint {
                position: [1, 1, 0].into(),
                color: [1.0, 1.0, 1.0],
                emission_strength: 0.01
            }
        )
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
