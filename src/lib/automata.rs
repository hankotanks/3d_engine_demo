use std::{ops::{ 
    Index, 
    IndexMut 
}, collections::HashMap};

use cgmath::Point3;

#[derive(Clone, Copy, Debug)]
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
    pub(crate) const fn center(&self) -> Point3<i8> {
        Point3::new(
            self.x_len as i8 / 2, 
            self.y_len as i8 / 2, 
            self.z_len as i8 / 2
        )
    }

    pub(crate) const fn cell_count(&self) -> usize {
        self.x_len * self.y_len * self.z_len
    }
}

pub struct Automata {
    pub(crate) cells: HashMap<Point3<i8>, u8>,
    pub(crate) size: Size
}

impl Index<Point3<i8>> for Automata {
    type Output = u8;

    fn index(&self, mut index: Point3<i8>) -> &Self::Output {
        index = wrap(self.size, index);

        match self.cells.get(&index) {
            Some(cell_state) => cell_state,
            None => &0u8
        }
    }
}

impl IndexMut<Point3<i8>> for Automata {
    fn index_mut(&mut self, index: Point3<i8>) -> &mut Self::Output {
        index = wrap(self.size, index);

        match self.cells.get_mut(&index) {
            Some(cell_state) => cell_state,
            None => {
                self.cells.insert(index, 0u8);
                self.cells.get_mut(&index).unwrap()
            }
        }
    }
}

impl Automata {
    pub fn new(size: Size) -> Self {
        Self { cells: HashMap::new(), size }
    }

    pub fn iter(&self) -> AutomataIterator {
        AutomataIterator { size: self.size, index: 0 }
    }

    pub fn moore_neighborhood(&self, index: Point3<i8>) -> [u8; 26] {
        let mut neighbor_count = 0;
        let mut neighbors = [0; 26];
        [-1i8, 0, 1].iter().for_each(|&x| [-1i8, 0, 1].iter().for_each(|&y| [-1i8, 0, 1].iter().for_each(|&z| {
            if [x, y, z] != [0; 3] { neighbors[neighbor_count] = self[[x + index.x, y + index.y, z + index.z].into()] };
            neighbor_count += 1;
        } )));

        neighbors
    }

    pub fn von_neumann_neighborhood(&self, index: Point3<i8>) -> [u8; 6] {
        let mut neighbors = [0; 6];
        [[-1, 0, 0], [1, 0, 0], [0, -1, 0], [0, 1, 0], [0, 0, -1], [0, 0, 1]]
            .iter()
            .enumerate()
            .map(|(i, j)| neighbors[i] = self[[j[0] + index.x, j[1] + index.y, j[2] + index.z].into()]);

        neighbors 
    }
}

fn wrap(size: Size, coord: Point3<i8>) -> Point3<i8> {
    let mut x = coord.x % size.x_len as i8;
    let mut y = coord.y % size.y_len as i8;
    let mut z = coord.z % size.z_len as i8;

    if x < 0 { x += size.x_len as i8; }
    if y < 0 { y += size.y_len as i8; }
    if z < 0 { z += size.z_len as i8; }
    
    [x, y, z].into()
}

pub struct AutomataIterator {
    size: Size,
    index: usize
}

impl Iterator for AutomataIterator {
    type Item = Point3<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.size.cell_count() { return None; }

        let y = self.index / (self.size.x_len * self.size.z_len);
        let j = self.index - y * self.size.x_len * self.size.z_len;
        let z = j / self.size.x_len;
        let x = j % self.size.x_len;

        self.index += 1;

        Some([x, y, z].into())
    }
}