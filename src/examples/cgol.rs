use cgmath::Point3;

use block_engine_wgpu::{
    Config, 
    camera::birds_eye_camera, 
    automata
};

#[allow(dead_code)]
pub const CGOL_SIZE: automata::Size = automata::Size {
    x_len: 71,
    y_len: 1,
    z_len: 71
};

#[allow(dead_code)]
pub const CGOL_CONFIG: Config = Config {
    fps: 20,
    thread_count: 4,
    camera_config: birds_eye_camera(CGOL_SIZE.x_len, CGOL_SIZE.z_len)
};

pub fn cgol_state_function(automata: &automata::Automata, index: Point3<usize>) -> usize {
    let neighbor_count = automata.neighbors(index)
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