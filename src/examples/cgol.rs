use block_engine_wgpu::mesh::{Mesh, objects::{self, MeshObject}};
use cgmath::Point3;
use rand::Rng;

const DIMENSIONS: (usize, usize) = (31, 31);
const INIT_DENSITY: f64 = 0.5;

fn neighbors(living_cells: &Vec<Point3<isize>>, target: Point3<isize>) -> usize {
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
    
    neighbor_count
}

fn redraw(mesh: &mut Mesh) {
    mesh.clear();
    
    let light_position = Point3::new(
        (DIMENSIONS.0 / 2) as isize,
        -5,
        (DIMENSIONS.1 / 2) as isize
    );
    let mut light = objects::Cube::new(light_position, [0.0; 3]);
    light.set_emitter(Some([1.0, 1.0, 1.0, 5.0].into()));

    mesh.add(light);
}

pub fn cgol_mesh_init(mesh: &mut Mesh) {
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

pub fn cgol_mesh_update(mesh: &mut Mesh) {
    let mut living_cells: Vec<Point3<isize>> = Vec::new();

    for cell in mesh.iter().skip(1) {
        living_cells.push(cell.position());
    }

    redraw(mesh);

    for x in 0..(DIMENSIONS.0 as isize) {
        for y in 0..(DIMENSIONS.1 as isize) {
            let neighbor_count = neighbors(&living_cells, [x, 0, y].into());

            if living_cells.contains(&[x, 0, y].into()) {
                if neighbor_count == 2 || neighbor_count == 3 {
                    mesh.add(objects::Cube::new(
                        [x, 0, y].into(), 
                        [1.0, 1.0, 1.0]
                    ));
                }
            } else {
                if neighbor_count == 3 {
                    mesh.add(objects::Cube::new(
                        [x, 0, y].into(), 
                        [1.0, 1.0, 1.0]
                    ));
                }
                
            }
        }
    }
}