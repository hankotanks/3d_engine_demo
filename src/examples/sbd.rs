use std::{ops::Range, collections::HashMap};

use cgmath::{
    Vector3,
    VectorSpace
};

use rand::Rng;

use block_engine_wgpu::{
    automata::{self, Size}, 
    Config, 
    Lighting, 
    camera::free_camera, 
    run
};

const LOW_COLOR: Vector3<f32> = Vector3::new(0.9, 0.3, 0.0);
const HIGH_COLOR: Vector3<f32> = Vector3::new(1.0, 0.0, 0.0);

pub fn survive_birth_decay(size: Size, s: Range<usize>, b: Range<usize>, d: u8) {
    let config = Config {
        fps: 30,
        thread_count: 8,
        lighting: Lighting::Corners,
        states: {
            let mut states = HashMap::new();
            for i in 1..d {
                let fraction = i as f32 / (d - 1) as f32;
                let color = LOW_COLOR.lerp(HIGH_COLOR, fraction);
                states.insert(i, [color.x, color.y, color.z]);
            }

            states
        },
        camera_config: free_camera(size)
    };

    let mut automata = automata::Automata::new(size);
    let mut prng = rand::thread_rng();
    for cell in automata.iter() {
        if prng.gen_bool(0.5f64) { automata[cell] = prng.gen_range(1u8..d); }
    }

    let (ss, se, bs, be) = (s.start, s.end, b.start, b.end);

    run(config, automata, move |ca, i| {
        let neighbors = ca.moore_neighborhood(i)
                .iter()
                .fold(0, |count, adj| if ca[*adj] != 0 { count + 1 } else { count } );

        let current = ca[i];
        match current {
            1 =>  (ss..se).contains(&neighbors) as u8,
            0 =>  if (bs..be).contains(&neighbors) { d - 1 } else { 0 },
            state => state - 1
        }
    } );


}