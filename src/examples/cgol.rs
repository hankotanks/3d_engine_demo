use cgmath::Point3;

use block_engine_wgpu::{
    Config, 
    camera::birds_eye_camera, 
    automata
};

#[allow(dead_code)]
const SIZE: automata::Size = automata::Size {
    x_len: 71,
    y_len: 1,
    z_len: 71
};

#[allow(dead_code)]
pub const CGOL_CONFIG: Config = Config {
    fps: 20,
    thread_count: 4,
    camera_config: birds_eye_camera(SIZE.x_len, SIZE.z_len)
};

pub fn cgol_automata() -> automata::Automata {
    automata::Automata::new(
        SIZE
    )
}

pub fn cgol_state_function(automata: &automata::Automata, index: Point3<usize>) -> usize {
    let target = Point3::new(index.x as isize, index.y as isize, index.z as isize);
    let offsets: [[isize; 2]; 8] = [
        [target.x - 1, target.z - 1],
        [target.x - 1, target.z],
        [target.x - 1, target.z + 1],
        [target.x, target.z - 1],
        [target.x, target.z + 1],
        [target.x + 1, target.z - 1],
        [target.x + 1, target.z],
        [target.x + 1, target.z + 1]
    ];

    let mut neighbor_count = 0;
    offsets.iter().for_each(|offset| {
        if offset[0] >= 0 && offset[1] >= 0 {
            neighbor_count += automata[Point3::new(offset[0] as usize, 0, offset[1] as usize)];
        }
    } );

    if automata[index] == 1 {
        if !(2..=3).contains(&neighbor_count) {
            return 0;
        } else {
            return 1;
        }
    } else if neighbor_count == 3 {
        return 1;
    }

    0
}