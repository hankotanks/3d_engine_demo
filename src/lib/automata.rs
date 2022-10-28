use std::sync::{
    Arc,
    Mutex
};

use cgmath::Point3;
use rand::Rng;

use super::mesh::objects;

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

pub struct Automata {
    pub cells: Arc<Mutex<Vec<usize>>>,
    pub size: Arc<Size>,
    pub state_function: Arc<dyn Fn(&[usize], Size, usize) -> usize + Send + Sync>,
    pub cube_function: Box<dyn Fn(Point3<isize>, usize) -> Option<Box<dyn objects::MeshObject>>>
}

impl Automata {
    pub fn new<F: 'static, G: 'static>(size: Size, state_function: F, cube_function: G) -> Self
        where F: Fn(&[usize], Size, usize) -> usize + Send + Sync + Copy, 
              G: Fn(Point3<isize>, usize) -> Option<Box<dyn objects::MeshObject>> {

        let mut cells = vec![0; size.x_len * size.y_len * size.z_len];
        
        let mut prng = rand::thread_rng();
        for cell in &mut cells { *cell = prng.gen_range(0..2); }

        Self {
            cells: Arc::new(Mutex::new(cells)),
            size: Arc::new(size),
            state_function: Arc::new(state_function),
            cube_function: Box::new(cube_function)
        }
    }
}