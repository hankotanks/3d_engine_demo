use cgmath::Point3;

use block_engine_wgpu::{
    Config, 
    camera::birds_eye_camera, 
    automata, 
    mesh::objects
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
        SIZE,
        state_function,
        cube_function
    )
}

fn cube_function(coord: Point3<isize>, state: usize) -> Option<Box<dyn objects::MeshObject>> {
    match state {
        0 => None,
        1 => Some(Box::new(objects::Cube::new(coord, [1.0; 3]))),
        _ => panic!()
    }
}

fn state_function(cells: &[usize], size: automata::Size, index: usize) -> usize {
    let target = size.to_point(index);
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
        if let Some(index) = size.to_index(Point3::new(offset[0], 0, offset[1])) {
            neighbor_count += cells[index];
        }
    } );

    if cells[index] == 1 {
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