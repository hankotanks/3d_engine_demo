use block_engine_wgpu::{
    automata, 
    Config, 
    camera::birds_eye_camera, 
    Lighting
};

use cgmath::Point3;

const WW_SIZE: automata::Size = automata::Size {
    x_len: 71,
    y_len: 3,
    z_len: 3
};

pub const WW_CONFIG: Config = Config {
    fps: 30,
    thread_count: 4,
    lighting: Lighting::CenterBottom,
    camera_config: birds_eye_camera(WW_SIZE.x_len, WW_SIZE.z_len)
};

pub fn ww_init() -> automata::Automata {
    let mut automata = automata::Automata::new(WW_SIZE);

    for x in 0..WW_SIZE.x_len {
        let point = Point3::new(x, 1, 1);

        automata[point] = 1;
    }

    automata[Point3::new(0, 1, 1)] = 3;
    automata[Point3::new(1, 1, 1)] = 2;

    automata
}

pub fn ww_update(automata: &automata::Automata, point: Point3<usize>) -> usize {
    let current = automata[point];
    match current {
        1 => {
            if (1..=2).contains(&automata.moore_neighborhood(point).iter().fold(0, |count, adj| { 
                if automata[*adj] == 2 {
                    count + 1
                } else {
                    count
                }
             } )) {
                2
             } else {
                1
             }
        }
        2 => 3,
        3 => 1,
        _ => current
    }
}