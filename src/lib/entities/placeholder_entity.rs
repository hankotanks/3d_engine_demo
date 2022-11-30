use cgmath::Point3;

use crate::drawable::Drawable;

use super::Entity;

pub struct PlaceholderEntity {
    pub center: Point3<f32>,
    pub color: [f32; 3],
    pub light: Option<[f32; 4]>
}

impl Entity for PlaceholderEntity {

}

impl Drawable for PlaceholderEntity {
    fn center(&self) -> Point3<f32> {
        self.center
    }

    fn set_center(&mut self, center: Point3<f32>) {
        self.center = center;
    }

    fn color(&self) -> [f32; 3] {
        self.color
    }

    fn set_color(&mut self, color: [f32; 3]) {
        self.color = color;
    }

    fn light(&self) -> Option<[f32; 4]> {
        self.light
    }

    fn set_light(&mut self, light: [f32; 4]) {
        self.light = Some(light);
    }

    fn build_object_data(&self) -> crate::drawable::Triangles {
        crate::drawable::Triangles {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }
}