use std::io;

use block_engine_wgpu::{
    automata, 
    Config, 
    camera::birds_eye_camera, 
    Lighting, run
};

use cgmath::Point3;

pub fn ww_run(file_name: &str) -> Result<(), io::Error> {
    let automata = automata::Automata::from_file(file_name)?;
    let mut config = Config {
        fps: 10,
        thread_count: 4,
        lighting: Lighting::Corners,
        camera_config: birds_eye_camera(automata.size.x_len, automata.size.z_len)
    };
    config.camera_config.rotate_speed = None;

    pollster::block_on(run(
        config,
        automata,
        ww_update,
        &[(1, [1.0, 0.2, 0.0]), (2, [1.0; 3]), (3, [0.0, 0.2, 1.0])]
    ));

    Ok(())
}

fn ww_update(automata: &automata::Automata, point: Point3<usize>) -> u8 {
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