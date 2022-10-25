use rand::Rng;

use cgmath::Point3;

use block_engine_wgpu::{
    mesh, 
    mesh::objects,
    Config, 
    camera::CameraConfig
};

#[allow(dead_code)]
const DIMENSIONS: (usize, usize) = (35, 35);

#[allow(dead_code)]
const INIT_DENSITY: f64 = 0.2;

#[allow(dead_code)]
const CGOL_CONFIG_CENTER: Point3<isize> = Point3::new(
    (DIMENSIONS.0 / 2) as isize,
    1isize,
    (DIMENSIONS.1 / 2) as isize
);

#[allow(dead_code)]
const CGOL_CONFIG_DISTANCE: f32 = (DIMENSIONS.0 + DIMENSIONS.1) as f32;

#[allow(dead_code)]
pub const CGOL_CONFIG: Config = Config {
    fps: 30,
    camera_config: CameraConfig { 
        target: Some(CGOL_CONFIG_CENTER),
        distance: Some(CGOL_CONFIG_DISTANCE),        
        pitch: None,
        yaw: Some(0.0),
        aspect: None, 
        zoom_speed: None,
        rotate_speed: None,
        locked: true
    },
};

#[allow(dead_code)]
fn redraw(mesh: &mut mesh::Mesh) {
    mesh.clear();

    let mut light_position = CGOL_CONFIG_CENTER;
    light_position.y = -5;

    let mut light = objects::Cube::new(light_position, [0.0; 3]);
    objects::MeshObject::set_emitter(
        &mut light, 
        Some([1.0, 1.0, 1.0, 5.0].into())
    );

    mesh.add(light);
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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