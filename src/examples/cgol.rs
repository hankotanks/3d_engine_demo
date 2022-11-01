use cgmath::Point3;

use block_engine_wgpu::{
    Config, 
    camera::birds_eye_camera, 
    automata, 
    Lighting
};

use rand::Rng;

#[allow(dead_code)]
const CGOL_SIZE: automata::Size = automata::Size {
    x_len: 31,
    y_len: 3,
    z_len: 31
};

#[allow(dead_code)]
pub const CGOL_CONFIG: Config = Config {
    fps: 20,
    thread_count: 4,
    lighting: Lighting::CenterBottom,
    camera_config: birds_eye_camera(CGOL_SIZE.x_len, CGOL_SIZE.z_len)
};

#[allow(dead_code)]
pub fn cgol_automata_init() -> automata::Automata {
    let mut automata = automata::Automata::new(CGOL_SIZE);
    'coord: for coord in automata.iter_coords() {
        if coord.y != 1 { continue 'coord; }

        if rand::thread_rng().gen_bool(0.5f64) {
            automata[coord] = 1;
        }
    }

    automata
}

#[allow(dead_code)]
pub fn cgol_state_function(automata: &automata::Automata, index: Point3<usize>) -> usize {    
    if index.y != 1 { return 0; }

    let neighbor_count = automata.moore_neighborhood(index)
        .iter()
        .fold(0, |count, adj| { count + automata[*adj] } );

    if automata[index] == 1 {
        (2..=3).contains(&neighbor_count) as usize
    } else if neighbor_count == 3 {
        1
    } else {
        0
    }
}