mod examples;

#[allow(unused_imports)]
use examples::{
    game_of_life,
    ww_run,
    survive_birth_decay
};

fn main() {
    survive_birth_decay([13; 3].into(), 4..6, 2..3, 40);
}
