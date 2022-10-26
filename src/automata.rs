use cgmath::Point3;

pub struct Size {
    x_len: usize,
    y_len: usize,
    z_len: usize
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
    fn to_point(&self, mut index: usize) -> Point3<isize> {
        let z = (index / (self.x_len * self.y_len)) as isize;
        index -= z as usize * self.x_len * self.y_len;
        let y = (index / self.x_len) as isize;
        let x = (index % self.x_len) as isize;

        [x, y, z].into()
    }

    fn to_index(&self, point: Point3<isize>) -> usize {
        point.z as usize + point.y as usize * self.x_len + point.z as usize * self.x_len * self.y_len
    }
}

pub struct Grid {
    cells: Vec<usize>,
    size: Size
}