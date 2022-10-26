use std::sync::{
    Arc,
    Mutex
};

use cgmath::Point3;

use block_engine_wgpu::{
    Config, 
    camera::CameraConfig, 
    automata
};

#[allow(dead_code)]
const THREAD_COUNT: usize = 3;

#[allow(dead_code)]
const SIZE: automata::Size = automata::Size {
    x_len: 35,
    y_len: 1,
    z_len: 35,
};

#[allow(dead_code)]
const CGOL_CONFIG_CENTER: Point3<isize> = Point3::new(
    (SIZE.x_len / 2) as isize,
    1isize,
    (SIZE.z_len / 2) as isize
);

#[allow(dead_code)]
pub const CGOL_CONFIG: Config = Config {
    fps: 20,
    camera_config: CameraConfig { 
        target: Some(CGOL_CONFIG_CENTER),
        distance: Some((SIZE.x_len + SIZE.z_len) as f32),        
        pitch: None,
        yaw: Some(0.0),
        aspect: None, 
        zoom_speed: None,
        rotate_speed: None,
        locked: true
    }
};

pub fn cgol_automata() -> automata::Automata {
    automata::Automata::new(
        SIZE,
        cgol_state_function,
        vec![None, Some([1.0; 3])]
    )
}

fn cgol_state_function(cells: &Arc<Mutex<Vec<usize>>>, size: &Arc<automata::Size>, index: usize) -> usize {
    let target = size.to_point(index);
    let offsets: [[isize; 3]; 8] = [
            [target.x - 1, 0, target.z - 1],
            [target.x - 1, 0, target.z + 0],
            [target.x - 1, 0, target.z + 1],
            [target.x + 0, 0, target.z - 1],
            [target.x + 0, 0, target.z + 1],
            [target.x + 1, 0, target.z - 1],
            [target.x + 1, 0, target.z + 0],
            [target.x + 1, 0, target.z + 1]
        ];

        let mut neighbor_count = 0;
        offsets.iter().for_each(|offset| {
            if let Some(index) = size.to_index(Point3::new(offset[0], offset[1], offset[2])) {
                neighbor_count += cells.lock().unwrap()[index];
            }
        } );

        if cells.lock().unwrap()[index] == 1 {
            if neighbor_count < 2 || neighbor_count > 3 {
                return 0;
            } else {
                return 1;
            }
        } else {
            if neighbor_count == 3 {
                return 1;
            }
        }
    
        0
}