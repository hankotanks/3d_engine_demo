use cgmath::Point3;

use crate::vertex::Vertex;

pub struct Triangles {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>
}

pub trait Drawable {
    fn center(&self) -> Point3<f32>;
    fn color(&self) -> [f32; 3];
    fn light(&self) -> Option<[f32; 4]>;

    fn set_center(&mut self, center: Point3<f32>);
    fn set_color(&mut self, color: [f32; 3]);
    fn set_light(&mut self, light: [f32; 4]);
    
    fn build_object_data(&self) -> Triangles;
}