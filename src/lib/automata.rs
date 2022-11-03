use std::{ fs, io };
use std::ops::{ Index, IndexMut };

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
    pub(crate) fn cell_count(&self) -> usize {
        self.x_len * self.y_len * self.z_len
    }
}

pub struct Automata {
    pub(crate) cells: Vec<u8>,
    pub size: Size
}

impl Index<Point3<usize>> for Automata {
    type Output = u8;

    fn index(&self, index: Point3<usize>) -> &Self::Output {
        let cell_index = {
            index.x + 
            index.y * self.size.x_len * self.size.z_len + 
            index.z * self.size.x_len 
        };

        if cell_index < self.cells.len() { return &self.cells[cell_index]; }

        panic!();
    }
}

impl IndexMut<Point3<usize>> for Automata {
    fn index_mut(&mut self, index: Point3<usize>) -> &mut Self::Output {
        let cell_index = {
            index.x + 
            index.y * self.size.x_len * self.size.z_len + 
            index.z * self.size.x_len 
        };

        if cell_index < self.cells.len() { return &mut self.cells[cell_index]; }

        panic!();
    }
}

pub struct StateIterator<'a> {
    automata: &'a Automata,
    index: usize
}

impl<'a> Iterator for StateIterator<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.automata.size.cell_count() {
            self.index += 1;

            return Some(self.automata.cells[self.index - 1])
        }

        None
    }
}

impl<'a> StateIterator<'a> {
    pub fn with_coord(self) -> CellIterator<'a> {
        CellIterator { state_iterator: self }
    }
}

pub struct CellIterator<'a> {
    state_iterator: StateIterator<'a>
}

impl<'a> Iterator for CellIterator<'a> {
    type Item = (Point3<usize>, u8);

    fn next(&mut self) -> Option<Self::Item> {
        let point = automata_index_to_point(
            self.state_iterator.automata.size, 
            self.state_iterator.index
        );

        self.state_iterator.next().map(|state| (point, state))
    }
}

pub struct CoordIterator {
    size: Size,
    index: usize
}

impl Iterator for CoordIterator {
    type Item = Point3<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.size.cell_count() { return None; }
        self.index += 1;

        Some(automata_index_to_point(self.size, self.index - 1))
    }
}


impl Automata {
    pub fn iter(&self) -> StateIterator {
        StateIterator { automata: self, index: 0 }
    }

    pub fn iter_coords(&self) -> CoordIterator {
        CoordIterator { size: self.size, index: 0 }
    }
}

impl Automata {
    pub fn new(size: Size) -> Self {
        let cells = vec![0; size.cell_count()];

        Self { cells, size }
    }

    pub fn from_file(file_name: &str) -> Result<Self, io::Error> {
        match fs::read(file_name) {
            Ok(mut buffer) => {
                let mut automata = Self::new([
                    buffer.remove(0) as usize, 
                    buffer.remove(0) as usize, 
                    buffer.remove(0) as usize
                ].into());

                let length = automata.size.cell_count();

                use std::cmp::Ordering::*;
                match buffer.len().cmp(&length) {
                    Less => buffer.resize(length, 0),
                    Greater | Equal => buffer.truncate(length)
                }

                automata.cells = buffer;

                Ok(automata)
            },
            Err(e) => Err(e)
        }
    }

    pub fn moore_neighborhood(&self, index: Point3<usize>) -> Vec<Point3<usize>> {
        let mut neighbors = Vec::new();
        for x in -1..=1isize {
            let x = index.x as isize + x;
            for y in -1..=1isize {
                let y  = index.y as isize + y;
                for z in -1..=1isize {
                    let z = index.z as isize + z;

                    let target = wrap_coord(self.size, [x, y, z].into());
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
            neighbors.push(wrap_coord(
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

pub(crate) fn automata_index_to_point(size: Size, mut index: usize) -> Point3<usize> {
    let y = index / (size.x_len * size.z_len);
    index -= y * size.x_len * size.z_len;
    let z = index / size.x_len;
    let x = index % size.x_len;

    Point3::new(x, y, z)
}

fn wrap_coord(size: Size, coord: Point3<isize>) -> Point3<usize> {
    let mut x = coord.x % size.x_len as isize;
    let mut y = coord.y % size.y_len as isize;
    let mut z = coord.z % size.z_len as isize;

    if x < 0 { x += size.x_len as isize; }
    if y < 0 { y += size.y_len as isize; }
    if z < 0 { z += size.z_len as isize; }
    
    [ x as usize, y as usize, z as usize ].into()
}