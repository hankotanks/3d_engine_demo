
use block_engine_wgpu::world;
use cgmath::Point3;

pub struct Gap {
    pub(crate) position: Point3<i16>,
    pub(crate) light: Option<[f32; 4]>
}

impl Default for Gap {
    fn default() -> Self {
        Self { 
            position: [0, 0, 0].into(), 
            light: None
        }
    }
}

impl Gap {
    pub fn new(position: Point3<i16>) -> Self {
        Self { position, light: None }
    }
}

impl world::Tile for Gap { 
    fn position(&self) -> Point3<i16> { self.position }
    fn set_position(&mut self, position: Point3<i16>) { self.position = position; }
}

impl world::Drawable for Gap {
    fn center(&self) -> Point3<f32> { self.position.cast::<f32>().unwrap() }
    fn set_center(&mut self, center: Point3<f32>) { self.position = center.cast::<i16>().unwrap(); }

    fn color(&self) -> [f32; 3] { [0.0; 3] }
    fn set_color(&mut self, _color: [f32; 3]) {  }

    fn light(&self) -> Option<[f32; 4]> { self.light }
    fn set_light(&mut self, light: [f32; 4]) { self.light = Some(light); }

    fn build_object_data(&self) -> world::Triangles {
        world::Triangles { vertices: Vec::new(), indices: Vec::new() }
    }
}