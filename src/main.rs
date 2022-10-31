mod examples;
use examples::cgol::{
    CGOL_CONFIG,
    cgol_automata_init,
    cgol_state_function
};

use examples::life3d::{
    LIFE3D_CONFIG,
    life3d_automata_init,
    life3d_automata_update
};

use block_engine_wgpu::run;

fn main() {
    /*
    pollster::block_on(run(
        CGOL_CONFIG,
        cgol_automata_init(),
        cgol_state_function,
        &[(1, [1.0; 3])]
    )); */

    pollster::block_on(run(
        LIFE3D_CONFIG,
        life3d_automata_init(),
        life3d_automata_update,
        &[(1, [1.0; 3])]
    ));
}
