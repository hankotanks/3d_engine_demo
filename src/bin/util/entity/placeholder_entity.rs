use block_engine_wgpu::world;
use cgmath::{Point3, Vector3, Zero};

pub struct PlaceholderEntity {
    pub center: Point3<f32>,
    pub color: [f32; 3],
    pub light: Option<[f32; 4]>,

    pub velocity: Vector3<f32>,
    pub weight: f32
}

impl world::Entity for PlaceholderEntity {
    fn velocity(&self) -> cgmath::Vector3<f32> {
        self.velocity
    }

    fn set_velocity(&mut self, velocity: cgmath::Vector3<f32>) {
        self.velocity = velocity;
    }

    fn weight(&self) -> f32 {
        self.weight
    }

    fn set_weight(&mut self, weight: f32) {
        self.weight = weight;
    }

    fn is_in_motion(&self) -> bool {
        self.velocity.is_zero()
    }
}

impl world::Drawable for PlaceholderEntity {
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

    fn build_object_data(&self) -> world::Triangles {
        world::Triangles {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }
}