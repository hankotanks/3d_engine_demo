use std::{
    ops::Index, 
    ops::IndexMut,
    collections::HashMap
};

use cgmath::Point3;

#[derive(Clone, Copy, Debug)]
pub struct Size {
    pub x_len: u8,
    pub y_len: u8,
    pub z_len: u8
}

impl From<[u8; 3]> for Size {
    fn from(item: [u8; 3]) -> Self { 
        Self {
            x_len: item[0],
            y_len: item[1],
            z_len: item[2]
        }
    }
}

impl Size {
    pub(crate) const fn center(&self) -> Point3<i16> {
        Point3::new(
            self.x_len as i16 / 2, 
            self.y_len as i16 / 2, 
            self.z_len as i16 / 2
        )
    }

    pub(crate) const fn cell_count(&self) -> usize {
        self.x_len as usize * self.y_len as usize * self.z_len as usize
    }
}

#[derive(Clone)]
pub struct Automata {
    pub(crate) cells: HashMap<Point3<i16>, u8>,
    pub(crate) size: Size
}

impl Index<Point3<i16>> for Automata {
    type Output = u8;

    fn index(&self, mut index: Point3<i16>) -> &Self::Output {
        index = wrap(self.size, index);

        match self.cells.get(&index) {
            Some(cell_state) => cell_state,
            None => &0u8
        }
    }
}

impl IndexMut<Point3<i16>> for Automata {
    fn index_mut(&mut self, mut index: Point3<i16>) -> &mut Self::Output {
        index = wrap(self.size, index); 

        if self.cells.contains_key(&index) {
            return self.cells.get_mut(&index).unwrap();
        }

        self.cells.insert(index, 0u8);
        self.cells.get_mut(&index).unwrap()
    }
}

impl Automata {
    pub fn new(size: Size) -> Self {
        Self { cells: HashMap::new(), size }
    }

    pub fn iter(&self) -> AutomataIterator {
        AutomataIterator { size: self.size, index: 0 }
    }

    pub fn moore_neighborhood(&self, index: Point3<i16>) -> Neighborhood {
        let mut neighbor_count = 0;
        let mut neighbors = Neighborhood { states: [0u8; 26], len: 26 };
        [-1i16, 0, 1].iter().for_each(|&x| [-1i16, 0, 1].iter().for_each(|&y| [-1i16, 0, 1].iter().for_each(|&z| {
            if [x, y, z] != [0; 3] { 
                neighbors.states[neighbor_count] = self[[x + index.x, y + index.y, z + index.z].into()];
                neighbor_count += 1;
            };
            
        } )));

        neighbors
    }

    pub fn von_neumann_neighborhood(&self, index: Point3<i16>) -> Neighborhood {
        let mut neighbors = Neighborhood { states: [0u8; 26], len: 6 };
        [[-1, 0, 0], [1, 0, 0], [0, -1, 0], [0, 1, 0], [0, 0, -1], [0, 0, 1]]
            .iter()
            .enumerate()
            .for_each(|(i, j)| neighbors.states[i] = self[[j[0] + index.x, j[1] + index.y, j[2] + index.z].into()]);

        neighbors 
    }
}

fn wrap(size: Size, coord: Point3<i16>) -> Point3<i16> {
    let mut x = coord.x % size.x_len as i16;
    let mut y = coord.y % size.y_len as i16;
    let mut z = coord.z % size.z_len as i16;

    if x < 0 { x += size.x_len as i16; }
    if y < 0 { y += size.y_len as i16; }
    if z < 0 { z += size.z_len as i16; }
    
    [x, y, z].into()
}

pub struct AutomataIterator {
    size: Size,
    index: usize
}

impl Iterator for AutomataIterator {
    type Item = Point3<i16>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.size.cell_count() { return None; }

        let y = self.index / (self.size.x_len as usize * self.size.z_len as usize);
        let j = self.index - y * self.size.x_len as usize * self.size.z_len as usize;
        let z = j / self.size.x_len as usize;
        let x = j % self.size.x_len as usize;

        self.index += 1;

        Some([x as i16, y as i16, z as i16].into())
    }
}

pub struct Neighborhood {
    states: [u8; 26],
    len: usize
}

impl Neighborhood {
    pub fn living(&self) -> usize {
        self.states[0..self.len].iter().fold(0, |c, &s| c + (s != 0) as usize)
    }

    pub fn count(&self, state: u8) -> usize {
        self.states[0..self.len].iter().fold(0, |c, &s| c + (s == state) as usize)
    }
}