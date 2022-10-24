mod examples;

use block_engine_wgpu::{
    run, 
    Config, 
    camera::CameraConfig
};

/*
use examples::cgol::{
    CGOL_CONFIG,
    cgol_mesh_init,
    cgol_mesh_update
};
*/

use examples::wave::{
    wave_mesh_init,
    wave_mesh_update
};

fn main() {
    let config = Config {
        frame_speed: 1.0, 
        camera_config: CameraConfig::default()
    };

    pollster::block_on(run(
        config,
        wave_mesh_init, 
        wave_mesh_update
    ));
}
