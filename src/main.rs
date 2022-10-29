mod examples;
use examples::cgol::{
    CGOL_CONFIG,
    CGOL_SIZE,
    cgol_state_function
};

use block_engine_wgpu::{
    run, 
    automata
};

fn main() {
    pollster::block_on(run(
        CGOL_CONFIG,
        automata::Automata::new(CGOL_SIZE),
        cgol_state_function,
        &vec![(1, [1.0; 3])]
    ));
}
