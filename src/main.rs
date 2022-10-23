mod examples;

use block_engine_wgpu::{
    SceneConfig,
    run
};

use examples::cgol::{
    cgol_mesh_init,
    cgol_mesh_update
};

fn main() {
    let config = SceneConfig { frame_speed: 0.02 };
    pollster::block_on(
        run(config, cgol_mesh_init, cgol_mesh_update)
    );
}
