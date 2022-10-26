use std::{sync::{Arc, Mutex}, thread};

use cgmath::Point3;
use rand::Rng;

#[derive(Clone, Copy)]
pub struct Size {
    pub x_len: usize,
    pub y_len: usize,
    pub z_len: usize
}

impl From<[usize; 3]> for Size {
    fn from(item: [usize; 3]) -> Self { 
        Self {
            x_len: item[0],
            y_len: item[1],
            z_len: item[2]
        }
    }
}

impl Size {
    pub fn to_point(&self, mut index: usize) -> Point3<isize> {
        let z = (index / (self.x_len * self.y_len)) as isize;
        index -= z as usize * self.x_len * self.y_len;
        let y = (index / self.x_len) as isize;
        let x = (index % self.x_len) as isize;

        [x, y, z].into()
    }

    pub fn to_index(&self, point: Point3<isize>) -> Option<usize> {
        if point.x < 0 || point.y < 0 || point.z < 0 {
            return None;
        }

        let index = point.z as usize + point.y as usize * self.x_len + point.z as usize * self.x_len * self.y_len;
        if index >= self.x_len * self.y_len * self.z_len {
            None
        } else {
            Some(index)
        }
    }
}

const THREAD_COUNT: usize = 4;

pub struct Automata {
    cells: Arc<Mutex<Vec<usize>>>,
    size: Arc<Size>
}

impl Automata {
    pub fn new(size: Size) -> Self {
        let mut prng = rand::thread_rng();

        let mut cells = Vec::new();
        for _ in 0..(size.x_len * size.y_len * size.z_len) {
            cells.push(prng.gen_range(0..=1));
        }

        Self {
            cells: Arc::new(Mutex::new(cells)),
            size: Arc::new(size)
        }
    }

    pub fn tick<F: 'static>(&mut self, cell_state: F) 
        where F: Fn(&Arc<Mutex<Vec<usize>>>, &Arc<Size>, usize) -> usize + Send + Sync + Copy {

        let mut threads = Vec::new();
        for c in 0..THREAD_COUNT {
            let length = self.cells.lock().unwrap().len();
            let start = length / 4 * c;
            let end = length / 4 * (c + 1);

            let cells_reference = Arc::clone(&self.cells);
            let size_reference = Arc::clone(&self.size);
            threads.push(thread::spawn(move || {
                let mut updated_states: Vec<(usize, usize)> = Vec::new();
                for i in start..end {
                    let state = cell_state(&cells_reference, &size_reference, i);
                    if state != cells_reference.lock().unwrap()[i] {
                        updated_states.push((i, state));
                    }
                }

                updated_states
            } ));
        }

        let mut updated_states: Vec<(usize, usize)> = Vec::new();
        for handle in threads.drain(0..) {
            updated_states.append(&mut handle.join().unwrap());
        }

        *self.cells.lock().unwrap() = vec![0; self.size.x_len * self.size.y_len * self.size.z_len];
        for (index, state) in updated_states.drain(0..) {
            self.cells.lock().unwrap()[index] = state;
        }
    }

    pub fn debug_print_2d(&self) {
        for z in 0..self.size.z_len {
            for x in 0..self.size.x_len {
                dbg!(self.size.to_index(Point3::new(x as isize, 0, z as isize)));
            }
        }
        /*
        for (index, cell) in self.cells.lock().unwrap().iter().enumerate() {
            let coord = self.size.to_point(index);
            if coord.x == 0 { println!(""); }
            print!("{}", *cell);
        }  */
        println!("");
    }
}