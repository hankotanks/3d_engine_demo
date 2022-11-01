mod examples;

#[allow(unused_imports)]
use examples::cgol::{
    CGOL_CONFIG,
    cgol_automata_init,
    cgol_state_function
};

use std::io::Error;

use examples::wire_world::ww_run;

fn main() -> Result<(), Error> {
    ww_run("wire_world.txt")?;

    Ok(())
}
