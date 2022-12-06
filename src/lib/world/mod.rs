pub(crate) mod drawable;
pub use drawable::{ Drawable, Triangles };

pub(crate) mod tile;
pub use tile::Tile;

pub(crate) mod entity;
pub use entity::{ Entity, EntityHandle };

use crate::{
    vertex::Vertex, 
    light
};

use std::{collections::HashMap, time, cmp};

use cgmath::{ 
    Point3, 
    Vector3, 
    Zero 
};

use wgpu::{
    Buffer, 
    Device, 
    util::DeviceExt
};

#[derive(Default)]
pub struct World<'a> {
    tile_objects: HashMap<Point3<i16>, Box<dyn Tile>>,
    tile_vertices: Vec<Vertex>,
    tile_indices: Vec<u32>,
    entity_objects: Vec<EntityHandle>,
    entity_tags: HashMap<&'a str, EntityHandle>,
    entity_lifetimes: Vec<(time::Instant, time::Duration)>
}

impl<'a> World<'a> {
    pub fn add_tile(&mut self, tile: impl Tile + 'static) {
        let mut triangles = tile.build_object_data();

        self.tile_objects.insert(tile.position(), Box::new(tile));

        let mut offset_indices = triangles.indices
            .iter()
            .map(|i| *i + self.tile_vertices.len() as u32)
            .collect::<Vec<u32>>();

        self.tile_indices.append(&mut offset_indices);
        self.tile_vertices.append(&mut triangles.vertices);
    }

    pub fn add_entity(
        &mut self, 
        entity: impl Entity + 'static,
        lifetime: Option<time::Duration>
    ) -> EntityHandle {
        let handle = EntityHandle::new(entity);
        let handle_clone = handle.clone();
        self.entity_objects.push(handle);
        
        self.entity_lifetimes.push((
            time::Instant::now(), 
            match lifetime { 
                Some(lifetime) => lifetime, 
                None => time::Duration::MAX 
            } 
        ));

        handle_clone
    }

    pub fn add_entity_with_tag(
        &mut self,
        tag: &'a str,
        entity: impl Entity + 'static,
        lifetime: Option<time::Duration>
    ) -> EntityHandle {
        let handle = self.add_entity(entity, lifetime);
        let handle_clone = handle.clone();
        self.entity_tags.insert(tag, handle);

        handle_clone
    }

    pub fn contains_tile(&self, position: &Point3<i16>) -> bool {
        self.tile_objects.contains_key(position)
    }

    pub fn contains_entity(&self, tag: &str) -> bool {
        self.entity_tags.contains_key(tag)
    }

    pub fn get_tile(&self, position: Point3<i16>) -> Option<&(dyn Tile + 'static)> {
        self.tile_objects
            .get(&position)
            .map(|t| t.as_ref())
    }

    pub fn get_entity(&self, tag: &str) -> Option<EntityHandle> {
        self.entity_tags.get(tag).cloned()
    }

    pub(crate) fn resolve_entity_lifetimes(&mut self) {
        for index in (0..self.entity_lifetimes.len()).rev() {
            let (creation_instant, lifetime) = self.entity_lifetimes[index];
            if matches!(creation_instant.elapsed().cmp(&lifetime), cmp::Ordering::Greater | cmp::Ordering::Equal) {
                self.entity_objects.remove(index);
                self.entity_lifetimes.remove(index);
            }
        }
    }

    pub(crate) fn resolve_entity_physics(&mut self) {
        for index in 0..self.entity_objects.len() {
            let (velocity, weight) = {
                let entity = self.entity_objects[index].borrow(); // TODO
                
                (entity.velocity(), entity.weight())
            };

            self.apply_displacement_to_entity(index, velocity);

            let gravity = Vector3::new(0.0, -1.0 * weight, 0.0);
            self.apply_displacement_to_entity(index, gravity);
        }
    }

    fn apply_displacement_to_entity(
        &mut self, 
        entity_index: usize,
        displacement: Vector3<f32>
    ) {
        let mut handler = self.entity_objects[entity_index].clone();
        
        let (center, weight, velocity) = {
            let entity = handler.borrow();
            (entity.center(), entity.weight(), entity.velocity())
        };

        // collision detection fails when the entity travels more than 1 tile in a single tick
        let mut actual_displacement = displacement;
        [actual_displacement.x, actual_displacement.y, actual_displacement.z]
            .iter_mut()
            .for_each(|c| *c = c.clamp(-1.0, 1.0));

        let increment = actual_displacement * 0.1;

        // Find the discrete coordinates of the tile containing the entity's new position (velocity + position)
        fn get_discrete_point(pt: Point3<f32>) -> Point3<i16> {
            (pt.x.round() as i16, pt.y.round() as i16, pt.z.round() as i16).into()
        }

        let mut initial_collisions: Option<Vector3<bool>> = None;
        let mut collisions: Vector3<bool> = (false, false, false).into();
        while self.contains_tile(&get_discrete_point(center + actual_displacement)) && !actual_displacement.is_zero() {
            collisions.x = self.contains_tile(&get_discrete_point(
                center + Vector3::new(actual_displacement.x, 0.0, 0.0)));

            collisions.y = self.contains_tile(&get_discrete_point(
                center + Vector3::new(0.0, actual_displacement.y, 0.0)));

            collisions.z = self.contains_tile(&get_discrete_point(
                center + Vector3::new(0.0, 0.0, actual_displacement.z)));
            
            if initial_collisions.is_none() { initial_collisions = Some(collisions); }

            actual_displacement -= increment;
        }

        {
            let mut entity = handler.borrow_mut();
            
            let mut diff = Vector3::new(0.0, 0.0, 0.0);
            if let Some(collisions) = initial_collisions {
                if collisions.x { diff.x = displacement.x; }
                if collisions.y { diff.y = displacement.y; }
                if collisions.z { diff.z = displacement.z; }
            }
            
            entity.set_center(center + actual_displacement);
            entity.set_velocity((velocity - diff) * (1.0 - weight));
            
        }
    }

    pub(crate) fn build_light_sources(&self) -> (light::LightSources, u32) {
        let mut light_sources = light::LightSources { 
            light_uniforms: [
                light::Light::default(); 
                light::MAX_LIGHT_SOURCES
            ]
        };

        let mut light_count = 0;
        for (.., tile) in self.tile_objects.iter() {
            if let Some(light) = tile.light() {
                light_sources.light_uniforms[light_count].color = light;
                light_sources.light_uniforms[light_count].position = [
                    tile.position().x as f32,
                    tile.position().y as f32,
                    tile.position().z as f32,
                    1.0
                ];

                light_count += 1;
            }
        }

        for entity in self.entity_objects.iter().map(|e| e.borrow()) { // TODO
            if let Some(light) = entity.light() {
                light_sources.light_uniforms[light_count].color = light;
                light_sources.light_uniforms[light_count].position = [
                    entity.center().x,
                    entity.center().y,
                    entity.center().z,
                    1.0
                ];

                light_count += 1;
            }
        }

        (light_sources, light_count as u32)

    }

    pub(crate) fn build_geometry_buffers(&self, device: &mut Device) -> (Buffer, Buffer, u32) {
        let mut indices = self.tile_indices.clone();
        let mut vertices = self.tile_vertices.clone();

        for entity in self.entity_objects.iter().map(|e| e.borrow()) { // TODO
            let mut triangles = entity.build_object_data();
            let mut offset_indices = triangles.indices
                .iter()
                .map(|i| *i + vertices.len() as u32)
                .collect::<Vec<u32>>();
            indices.append(&mut offset_indices);
            vertices.append(&mut triangles.vertices);
        }

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(vertices.as_slice()),
                usage: wgpu::BufferUsages::VERTEX
            }
        );

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(indices.as_slice()),
                usage: wgpu::BufferUsages::INDEX,
            }
        );

        (vertex_buffer, index_buffer, indices.len() as u32)
    }
}
