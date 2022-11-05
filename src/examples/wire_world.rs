use std::io;

use block_engine_wgpu::{
    automata, 
    Config, 
    camera::free_camera, 
    Lighting, run
};

#[allow(dead_code)]
pub fn ww_run(file_name: &str) -> Result<(), io::Error> {
    let automata = automata::Automata::from_file(file_name)?;
    
    let config = Config {
        fps: 10,
        thread_count: 4,
        lighting: Lighting::Corners,
        states: &[(1, [1.0, 0.2, 0.0]), (2, [1.0; 3]), (3, [0.0, 0.2, 1.0])],
        camera_config: free_camera(automata.size)
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