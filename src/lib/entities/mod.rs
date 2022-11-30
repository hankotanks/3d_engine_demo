mod placeholder_entity;
use cgmath::Vector3;
pub use placeholder_entity::PlaceholderEntity;

use crate::drawable;

pub trait Entity: drawable::Drawable {
    fn velocity(&self) -> Vector3<f32>;
    fn set_velocity(&mut self, velocity: Vector3<f32>);

    fn weight(&self) -> f32;
    fn set_weight(&mut self, weight: f32);

    fn is_in_motion(&self) -> bool;
}