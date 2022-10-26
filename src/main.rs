mod examples;

use examples::cgol::{
    CGOL_CONFIG,
    cgol_automata
};

#[allow(unused_imports)]
use block_engine_wgpu::run;

fn main() {
    pollster::block_on(run(
        CGOL_CONFIG,
        cgol_automata()
    ));
}
