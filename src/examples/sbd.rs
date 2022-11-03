use std::ops::Range;

use cgmath::{
    Point3,
    Vector3,
    VectorSpace
};

use rand::Rng;

use block_engine_wgpu::{
    automata, 
    Config, 
    Lighting, 
    camera::CameraConfig, 
    run
};

const SBD_SIZE: automata::Size = automata::Size {
    x_len: 13,
    y_len: 13,
    z_len: 13,
};

const SBD_CONFIG: Config = Config {
    fps: 20,
    thread_count: 4,
    lighting: Lighting::VonNeumann,
    camera_config: CameraConfig {
        target: Some(Point3::new(
            SBD_SIZE.x_len as isize / 2, 
            SBD_SIZE.y_len as isize / 2, 
            SBD_SIZE.z_len as isize / 2
        )),
        distance: Some(SBD_SIZE.x_len as f32),
        pitch: None,
        yaw: None,
        aspect: None,
        zoom_speed: None,
        rotate_speed: None
    },
};

const SURVIVAL: Range<usize> = 4..5;
const BIRTH: Range<usize> = 3..4;
const DECAY: u8 = 20;
const FILL_DENSITY: f64 = 0.02;
const LOW_COLOR: Vector3<f32> = Vector3::new(0.9, 0.3, 0.0);
const HIGH_COLOR: Vector3<f32> = Vector3::new(1.0, 0.0, 0.0);

pub fn sbd_run() {
    if DECAY < 2 { panic!(); }

    let mut automata = automata::Automata::new(SBD_SIZE);
    
    let mut prng = rand::thread_rng();
    for cell in automata.iter_coords() {
        if prng.gen_bool(FILL_DENSITY) { 
            automata[cell] = prng.gen_range(1u8..DECAY); 
        }
    }

    let mut states: Vec<(u8, [f32; 3])> = Vec::new();
    for i in 1..DECAY {
        let fraction = i as f32 / (DECAY - 1) as f32;
        let color = LOW_COLOR.lerp(HIGH_COLOR, fraction);
        states.push((i, [color.x, color.y, color.z]));
    }

    run(
        SBD_CONFIG,
        automata,
        sbd_update,
        &states
    );
}

fn sbd_update(automata: &automata::Automata, point: Point3<usize>) -> u8 {
    let neighbors = automata.moore_neighborhood(point)
                .iter()
                .fold(0, |count, adj| if automata[*adj] != 0 { count + 1 } else { count } );

    let current = automata[point];
    match current {
        1 =>  if SURVIVAL.contains(&neighbors) { 1 } else { 0 },
        0 =>  if BIRTH.contains(&neighbors) { DECAY - 1 } else { 0 },
        state => state - 1
    }
}