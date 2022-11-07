use std::{io, fs};

use block_engine_wgpu::{
    automata, 
    Config, 
    camera::free_camera, 
    Lighting, run
};

fn read(file_name: &str) -> Result<(automata::Size, automata::Automata), io::Error> {
    match fs::read(file_name) {
        Ok(mut buffer) => {
            let size: automata::Size = [
                buffer.remove(0) as usize, 
                buffer.remove(0) as usize, 
                buffer.remove(0) as usize
            ].into();

            let length = size.x_len * size.y_len * size.z_len;

            let mut automata = automata::Automata::new(size);

            use std::cmp::Ordering::*;
            match buffer.len().cmp(&length) {
                Less => buffer.resize(length, 0),
                Greater | Equal => buffer.truncate(length)
            }

            for (index, state) in buffer.drain(0..).enumerate() {
                let y = index / (size.x_len * size.z_len);
                let j = index - y * size.x_len * size.z_len;
                let z = j / size.x_len;
                let x = j % size.x_len;

                automata[[x, y, z].into()] = state;
            }

            Ok((size, automata))
        },
        Err(e) => Err(e)
    }
}

#[allow(dead_code)]
pub fn ww_run(file_name: &str) -> Result<(), io::Error> {
    let (size, automata) = read(file_name)?;
    
    let config = Config {
        fps: 10,
        thread_count: 4,
        lighting: Lighting::Corners,
        states: &[(1, [1.0, 0.2, 0.0]), (2, [1.0; 3]), (3, [0.0, 0.2, 1.0])],
        camera_config: free_camera(size)
    };

    run(config, automata, |ca, i| {
        match ca[i] {
            1 => {
                let adj = ca.moore_neighborhood(i)
                    .iter()
                    .fold(0, |count, adj| count + (ca[*adj] == 2) as i32);

                if (1..=2).contains(&adj) { 2 } else { 1 }
            },
            2 => 3,
            3 => 1,
            _ => ca[i]
        }
    } );

    Ok(())
}