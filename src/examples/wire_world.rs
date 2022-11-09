use std::{io, fs, collections::HashMap};

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
                buffer.remove(0) as u8, 
                buffer.remove(0) as u8, 
                buffer.remove(0) as u8
            ].into();

            let length = (size.x_len * size.y_len * size.z_len) as usize;

            let mut automata = automata::Automata::new(size);

            use std::cmp::Ordering::*;
            match buffer.len().cmp(&length) {
                Less => buffer.resize(length, 0),
                Greater | Equal => buffer.truncate(length)
            }

            for (index, state) in buffer.drain(0..).enumerate() {
                let y = index / (size.x_len * size.z_len) as usize;
                let j = index - y * (size.x_len * size.z_len) as usize;
                let z = j / size.x_len as usize;
                let x = j % size.x_len as usize;

                automata[[x as i16, y as i16, z as i16].into()] = state;
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
        neighborhood: block_engine_wgpu::Neighborhood::Moore,
        states: {
            let mut states = HashMap::new();
            states.insert(1, [1.0, 0.2, 0.0]);
            states.insert(2, [1.0; 3]);
            states.insert(3, [0.0, 0.2, 1.0]);

            states
        },
        camera_config: free_camera(size)
    };

    run(config, automata, |ca, i| {
        match i {
            1 => {
                let adj = ca.count(2);
                if (1..=2).contains(&adj) { 2 } else { 1 }
            },
            2 => 3,
            3 => 1,
            _ => i
        }
    } );

    Ok(())
}