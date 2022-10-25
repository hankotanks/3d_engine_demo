mod examples;

use block_engine_wgpu::run;

/*
use examples::cgol::{
    CGOL_CONFIG,
    cgol_mesh_init,
    cgol_mesh_update
};
*/

use examples::rain::{
    RAIN_CONFIG,
    rain_mesh_init,
    rain_mesh_update
};

fn main() {
    pollster::block_on(run(
        RAIN_CONFIG,
        rain_mesh_init, 
        rain_mesh_update
    ));
}
