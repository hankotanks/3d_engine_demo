mod examples;
mod automata;

use std::sync::{Arc, Mutex};

use cgmath::Point3;
use automata::Size;
use block_engine_wgpu::run;

#[allow(unused_imports)]
use examples::cgol::{
    CGOL_CONFIG,
    cgol_mesh_init,
    cgol_mesh_update
};


#[allow(unused_imports)]
use examples::rain::{
    RAIN_CONFIG,
    rain_mesh_init,
    rain_mesh_update
};

fn check_state(cells: &Arc<Mutex<Vec<usize>>>, size: &Arc<Size>, index: usize) -> usize {
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
            if neighbor_count == 2 || neighbor_count == 3 {
                return 1;
            }
        } else {
            if neighbor_count == 3 {
                return 1;
            }
        }
    
        0
}

fn main() {
    let mut automata = automata::Automata::new(
        automata::Size {
            x_len: 8,
            y_len: 1,
            z_len: 8,
        }
    );

    automata.tick(check_state);
    automata.debug_print_2d();
    automata.tick(check_state);
    automata.debug_print_2d();
    automata.tick(check_state);
    automata.debug_print_2d();
    
    /*
    pollster::block_on(run(
        CGOL_CONFIG,
        cgol_mesh_init, 
        cgol_mesh_update
    ));
    */
}
