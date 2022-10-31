mod examples;
use examples::cgol::{
    CGOL_CONFIG,
    cgol_automata_init,
    cgol_state_function
};

use examples::wire_world::{
    WW_CONFIG,
    ww_init,
    ww_update
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
        WW_CONFIG,
        ww_init(),
        ww_update,
        &[(1, [1.0, 0.2, 0.0]), (2, [1.0; 3]), (3, [0.0, 0.2, 1.0])]
    ));
}
