use cgmath::Point3;

use super::drawable;

pub trait Tile: drawable::Drawable {
    fn position(&self) -> Point3<i16>;
    fn set_position(&mut self, position: Point3<i16>);
}