use std::{
    cell::{RefCell, Ref, RefMut}, 
    rc::Rc, ops::Deref
};

use cgmath::Vector3;

use super::drawable;

pub trait Entity: drawable::Drawable {
    fn velocity(&self) -> Vector3<f32>;
    fn set_velocity(&mut self, velocity: Vector3<f32>);

    fn weight(&self) -> f32;
    fn set_weight(&mut self, weight: f32);

    fn is_in_motion(&self) -> bool;
}

#[derive(Clone)]
pub struct EntityHandle(Rc<RefCell<dyn Entity>>);

impl EntityHandle {
    pub fn new(entity: impl Entity + 'static) -> Self {
        Self(Rc::new(RefCell::new(entity)))
    }

    pub fn borrow(&self) -> Ref<'_, dyn Entity> {
        self.0.deref().borrow()
    }

    pub fn borrow_mut(&mut self) -> RefMut<'_, dyn Entity> {
        self.0.deref().borrow_mut()
    }
}