mod placeholder_entity;
pub use placeholder_entity::PlaceholderEntity;

use crate::drawable;

pub trait Entity: drawable::Drawable {
}