use std::ops::{ 
    Index, 
    IndexMut 
};

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
    pub(crate) const fn center(&self) -> Point3<isize> {
        Point3::new(
            self.x_len as isize / 2, 
            self.y_len as isize / 2, 
            self.z_len as isize / 2
        )
    }

    pub(crate) const fn cell_count(&self) -> usize {
        self.x_len * self.y_len * self.z_len
    }
}

pub struct Automata {
    pub(crate) cells: Vec<u8>,
    pub(crate) size: Size
}

impl Index<Point3<usize>> for Automata {
    type Output = u8;

    fn index(&self, index: Point3<usize>) -> &Self::Output {
        let cell_index = {
            index.x + 
            index.y * self.size.x_len * self.size.z_len + 
            index.z * self.size.x_len 
        };

        &self.cells[cell_index]
    }
}

impl IndexMut<Point3<usize>> for Automata {
    fn index_mut(&mut self, index: Point3<usize>) -> &mut Self::Output {
        let cell_index = {
            index.x + 
            index.y * self.size.x_len * self.size.z_len + 
            index.z * self.size.x_len 
        };

        &mut self.cells[cell_index]
    }
}

impl Automata {
    pub fn new(size: Size) -> Self {
        let cells = vec![0; size.cell_count()];

        Self { cells, size }
    }

    pub fn iter(&self) -> AutomataIterator {
        AutomataIterator { size: self.size, index: 0 }
    }

    pub fn moore_neighborhood(&self, index: Point3<usize>) -> Vec<Point3<usize>> {
        let mut neighbors = Vec::new();
        for x in -1..=1isize {
            let x = index.x as isize + x;
            for y in -1..=1isize {
                let y  = index.y as isize + y;
                for z in -1..=1isize {
                    let z = index.z as isize + z;

                    let target = wrap(self.size, [x, y, z].into());
                    if target != index { neighbors.push(target); }
                }
            }
        }

        neighbors
    }

    pub fn von_neumann_neighborhood(&self, index: Point3<usize>) -> Vec<Point3<usize>> {
        let offsets: [[isize; 3]; 6] = [
            [-1, 0, 0],
            [1, 0, 0],
            [0, -1, 0],
            [0, 1, 0],
            [0, 0, -1],
            [0, 0, 1]
        ];

        let mut neighbors = Vec::new();
        for offset in offsets.into_iter() {
            neighbors.push(wrap(
                self.size, 
                Point3::new(
                    offset[0] + index.x as isize,
                    offset[1] + index.y as isize,
                    offset[2] + index.z as isize
                )
            ));
        }

        neighbors 
    }
}

fn wrap(size: Size, coord: Point3<isize>) -> Point3<usize> {
    let mut x = coord.x % size.x_len as isize;
    let mut y = coord.y % size.y_len as isize;
    let mut z = coord.z % size.z_len as isize;

    if x < 0 { x += size.x_len as isize; }
    if y < 0 { y += size.y_len as isize; }
    if z < 0 { z += size.z_len as isize; }
    
    [ x as usize, y as usize, z as usize ].into()
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

        Some(
            [x, y, z].into()
        )
    }
}