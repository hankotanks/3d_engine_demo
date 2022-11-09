use std::collections::HashMap;

use block_engine_wgpu::{
    Config, 
    camera::birds_eye_camera, 
    automata, 
    Lighting, 
    run
};

use rand::Rng;

#[allow(dead_code)]
pub fn game_of_life(size: automata::Size) {
    let config = Config {
        fps: 20,
        thread_count: 15,
        lighting: Lighting::Corners,
        states: {
            let mut states = HashMap::new();
            states.insert(1, [1.0; 3]);
            states
        },
        camera_config: birds_eye_camera(size)
    };

    let mut automata = automata::Automata::new(size);
    'coord: for coord in automata.iter() {
        if coord.y != 1 { continue 'coord; }
        if rand::thread_rng().gen_bool(0.5f64) { automata[coord] = 1; }
    }

    run(config, automata, |ca, i| {
        if i == 1 {
            let adj = ca.living();
            match i {
                1 if (2..=3).contains(&adj) => 1,
                0 if adj == 3 => 1,
                _ => 0
            }
        } else { 0 }
    } )
}