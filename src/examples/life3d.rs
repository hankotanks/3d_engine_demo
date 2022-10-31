use block_engine_wgpu::{automata::{self, Size}, Config, camera::CameraConfig, Lighting};
use cgmath::Point3;
use rand::Rng;

const LIFE3D_SIZE: Size = Size {
    x_len: 7,
    y_len: 7,
    z_len: 13
};

pub const LIFE3D_CONFIG: Config = Config {
    fps: 10,
    thread_count: 4,
    lighting: Lighting::Corners,
    camera_config: CameraConfig {
        target: Some(Point3::new(
            LIFE3D_SIZE.x_len as isize / 2,
            LIFE3D_SIZE.y_len as isize / 2,
            LIFE3D_SIZE.z_len as isize / 2
        )),
        distance: None,
        pitch: None,
        yaw: None,
        aspect: None,
        zoom_speed: None,
        rotate_speed: None,
    },
};

pub fn life3d_automata_init() -> automata::Automata {
    let mut automata = automata::Automata::new(LIFE3D_SIZE);

    let center_point = Point3::new(
        LIFE3D_SIZE.x_len / 2,
        LIFE3D_SIZE.y_len / 2,
        LIFE3D_SIZE.z_len / 2
    );

    for cell in automata.moore_neighborhood(center_point).into_iter() {
        if rand::thread_rng().gen_bool(0.8f64) {
            automata[cell] = 1;
        }
    }

    automata
}

pub fn life3d_automata_update(automata: &automata::Automata, point: Point3<usize>) -> usize {
    let neighbor_count = automata.moore_neighborhood(point)
        .iter()
        .fold(0, |count, adj| { count + automata[*adj] } );
    
    if automata[point] == 1 {
        (neighbor_count == 1 || neighbor_count == 10) as usize
    } else if neighbor_count == 4 || neighbor_count == 9 {
        1
    } else {
        0
    }
}