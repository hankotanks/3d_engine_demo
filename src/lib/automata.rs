use std::sync::{
    Arc,
    Mutex
};

use cgmath::Point3;

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
        let y = (index / (self.x_len * self.z_len)) as isize;
        index -= y as usize * self.x_len * self.z_len;
        let z = (index / self.x_len) as isize;
        let x = (index % self.x_len) as isize;

        [x, y, z].into()
    }

    pub fn to_index(&self, point: Point3<isize>) -> Option<usize> {
        if point.x < 0 || point.y < 0 || point.z < 0 {
            return None;
        }

        let index = point.x as usize + point.z as usize * self.x_len + point.y as usize * self.x_len * self.z_len;
        if index >= self.x_len * self.y_len * self.z_len {
            None
        } else {
            Some(index)
        }
    }
}

pub struct Cells {
    pub(crate) cell_array: Vec<usize>,
    pub(crate) size: Size
}

pub struct Automata {
    pub cells: Arc<Mutex<Cells>>,
    pub state_function: Arc<dyn Fn(&Cells, Point3<isize>) -> usize + Send + Sync>,
    pub states: Vec<(usize, [f32; 3])>
}

impl Automata {
    pub fn new<F: 'static>(cells: Cells, state_function: F, states: &[(usize, [f32; 3])]) -> Self
        where F: Fn(&Cells, Point3<isize>) -> usize + Send + Sync + Copy {
        Self {
            cells: Arc::new(Mutex::new(cells)),
            state_function: Arc::new(state_function),
            states: states.to_vec()
        }
    }
}