use std::ops::{
    Index, 
    IndexMut
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

pub struct StateIterator<'a> {
    automata: &'a Automata,
    index: usize
}

impl<'a> Iterator for StateIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.automata.size.x_len * self.automata.size.y_len * self.automata.size.z_len {
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
    type Item = (Point3<usize>, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let current_index = self.state_iterator.index;
        let current_state = self.state_iterator.next();

        let size = self.state_iterator.automata.size;
        match current_state {
            Some(state) => {
                let y = current_index / (size.x_len * size.z_len);
                let index = current_index - y * size.x_len * size.z_len;
                let z = index / size.x_len;
                let x = index % size.x_len;

                Some(([x, y, z].into(), state))
            },
            None => None
        }
    }
}

pub struct CoordIterator {
    size: Size,
    index: usize
}

impl Iterator for CoordIterator {
    type Item = Point3<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.size.x_len * self.size.y_len * self.size.z_len {
            return None;
        }

        let y = self.index / (self.size.x_len * self.size.z_len);
        let index = self.index - y * self.size.x_len * self.size.z_len;
        let z = index / self.size.x_len;
        let x = index % self.size.x_len;

        self.index += 1;

        Some([x, y, z].into())
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
        let cells = vec![0; size.x_len * size.y_len * size.z_len];

        Self {
            cells,
            size
        }
    }

    pub fn moore_neighborhood(&self, index: Point3<usize>) -> Vec<Point3<usize>> {
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