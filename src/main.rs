mod examples;

#[allow(unused_imports)]
use examples::{
    game_of_life,
    ww_run,
    survive_birth_decay
};

fn main() {
    survive_birth_decay([31; 3].into(), 1..4, 1..4, 5);
    //game_of_life([127, 3, 127].into());
}
