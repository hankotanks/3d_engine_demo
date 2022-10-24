use rand::Rng;

use cgmath::Point3;

use block_engine_wgpu::{
    mesh, 
    mesh::objects,
    Config, 
    camera::CameraConfig
};

const DIMENSIONS: (usize, usize) = (35, 35);
const INIT_DENSITY: f64 = 0.2;

pub const CGOL_CONFIG: Config = Config {
    frame_speed: 0.05,
    camera_config: CameraConfig { 
        target: Point3::new(
            (DIMENSIONS.0 / 2) as isize,
            1isize,
            (DIMENSIONS.1 / 2) as isize
        ),
        zoom_speed: 0.6,
        rotate_speed: 0.025,
        distance: (DIMENSIONS.0 + DIMENSIONS.1) as f32,
        pitch: 1.5,
        yaw: 0.0,
        locked: true, 
    },
};

fn redraw(mesh: &mut mesh::Mesh) {
    mesh.clear();

    let light_position = Point3::new(
        (DIMENSIONS.0 / 2) as isize,
        -5,
        (DIMENSIONS.1 / 2) as isize
    );
    let mut light = objects::Cube::new(light_position, [0.0; 3]);

    objects::MeshObject::set_emitter(
        &mut light, 
        Some([1.0, 1.0, 1.0, 5.0].into())
    );

    mesh.add(light);
}

pub fn cgol_mesh_init(mesh: &mut mesh::Mesh) {
    redraw(mesh);

    for x in 0..(DIMENSIONS.0 as isize) {
        for y in 0..(DIMENSIONS.1 as isize) {
            if rand::thread_rng().gen_bool(INIT_DENSITY) {
                mesh.add(objects::Cube::new(
                    [x, 0, y].into(),
                    [1.0, 1.0, 1.0]
                ));
            }
        }
    }
}

pub fn cgol_mesh_update(mesh: &mut mesh::Mesh) {
    fn check_state(living_cells: &Vec<Point3<isize>>, target: Point3<isize>) -> bool {
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
        offsets.iter().for_each(|offset| 
            if living_cells.contains(offset.into()) {neighbor_count += 1; } );
    
        if living_cells.contains(&target) {
            if neighbor_count == 2 || neighbor_count == 3 {
                return true;
            }
        } else {
            if neighbor_count == 3 {
                return true;
            }
        }
    
        false
    }

    let mut living_cells: Vec<Point3<isize>> = Vec::new();

    for cell in mesh.iter().skip(1) {
        living_cells.push(cell.position());
    }

    redraw(mesh);

    for x in 0..(DIMENSIONS.0 as isize) {
        for y in 0..(DIMENSIONS.1 as isize) {
            if check_state(&living_cells, [x, 0, y].into()) {
                mesh.add(objects::Cube::new(
                    [x, 0, y].into(), 
                    [1.0, 1.0, 1.0]
                ));
            }
        }
    }
}