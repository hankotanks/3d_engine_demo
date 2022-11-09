mod examples;

#[allow(unused_imports)]
use examples::{
    game_of_life,
    ww_run,
    survive_birth_decay
};

fn main() {
    //survive_birth_decay([17; 3].into(), 4..7, 14..17, 5);
    game_of_life([201, 3, 201].into());
}
