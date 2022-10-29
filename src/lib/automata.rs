use std::ops::{Index, IndexMut};

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

pub struct Automata {
    pub(crate) cells: Vec<usize>,
    pub(crate) size: Size
}

impl Index<Point3<usize>> for Automata {
    type Output = usize;

    fn index(&self, index: Point3<usize>) -> &Self::Output {
        let cell_index = index.x + index.y * self.size.x_len * self.size.z_len + index.z * self.size.x_len;
        if cell_index < self.cells.len() {
            return &self.cells[cell_index];
        }

        &0
    }
}

impl IndexMut<Point3<usize>> for Automata {
    fn index_mut(&mut self, index: Point3<usize>) -> &mut Self::Output {
        let cell_index = index.x + index.y * self.size.x_len * self.size.z_len + index.z * self.size.x_len;
        if cell_index < self.cells.len() {
            return &mut self.cells[cell_index];
        }

        panic!();
    }
}

pub struct AutomataIterator<'a> {
    automata: &'a Automata,
    index: usize
}

impl<'a> Iterator for AutomataIterator<'a> {
    type Item = (Point3<usize>, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.automata.size.x_len * self.automata.size.y_len * self.automata.size.z_len {
            self.index += 1;

            let y = self.index / (self.automata.size.x_len * self.automata.size.z_len);
            let index = self.index - y * self.automata.size.x_len * self.automata.size.z_len;
            let z = index / self.automata.size.x_len;
            let x = index % self.automata.size.x_len;

            let target = [x, y, z].into();
            
            return Some((target, self.automata.cells[self.index - 1]))
        }

        None
    }
}

impl Automata {
    pub fn iter(&self) -> AutomataIterator {
        AutomataIterator { automata: self, index: 0 }
    }
}

impl Automata {
    pub fn new(size: Size) -> Self {
        let mut cells = vec![0; size.x_len * size.y_len * size.z_len];
        
        let mut prng = rand::thread_rng();
        for cell in &mut cells { *cell = prng.gen_range(0..2); }

        Self {
            cells,
            size
        }
    }

    pub fn neighbors(&self, index: Point3<usize>) -> Vec<Point3<usize>> {
        let mut neighbors = Vec::new();
        'x_dim: for x in -1..=1isize {
            let x = index.x as isize + x;
            if x < 0 || x >= self.size.x_len as isize { continue 'x_dim; }

            'y_dim: for y in -1..=1isize {
                let y  = index.y as isize + y;
                if y < 0 || y >= self.size.y_len as isize { continue 'y_dim; }

                'z_dim: for z in -1..=1isize {
                    let z = index.z as isize + z;
                    if z < 0 || z >= self.size.z_len as isize { continue 'z_dim; }

                    let target = Point3::new(x as usize, y as usize, z as usize);
                    if target != index {
                        neighbors.push(target)
                    }
                }
            }
        }

        neighbors
    }
}