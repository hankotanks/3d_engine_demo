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

use std::collections::HashMap;

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
    entity_tags: HashMap<&'a str, EntityHandle>
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

    pub fn add_entity(&mut self, entity: impl Entity + 'static) -> EntityHandle {
        let handle = EntityHandle::new(entity);
        let handle_clone = handle.clone();
        self.entity_objects.push(handle);

        handle_clone
    }

    pub fn add_entity_with_tag(&mut self, entity: impl Entity + 'static, tag: &'a str) -> EntityHandle {
        let handle = self.add_entity(entity);
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

    pub fn get_tile(&self, position: Point3<i16>) -> Option<&Box<dyn Tile + 'static>> {
        self.tile_objects.get(&position)
    }

    pub fn get_entity(&self, tag: &str) -> Option<EntityHandle> {
        self.entity_tags.get(tag).cloned()
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
        mut displacement: Vector3<f32>
    ) {
        let mut entity = self.entity_objects[entity_index].clone();

        let original_displacement = displacement;

        // collision detection fails when the entity travels more than 1 tile in a single tick
        displacement.x = displacement.x.clamp(-1.0, 1.0);
        displacement.y = displacement.y.clamp(-1.0, 1.0);
        displacement.z = displacement.z.clamp(-1.0, 1.0);

        let increment = displacement * 0.1;

        // Find the discrete coordinates of the tile containing the entity's new position (velocity + position)
        fn get_discrete_point(pt: Point3<f32>) -> Point3<i16> {
            (pt.x.round() as i16, pt.y.round()as i16, pt.z.round() as i16).into()
        }

        
        let (center, weight, velocity) = {
            let entity = entity.borrow();
            (entity.center(), entity.weight(), entity.velocity())
        };

        let mut collided = false;
        while self.contains_tile(&get_discrete_point(center + displacement)) && !displacement.is_zero() {
            collided = true;
            displacement -= increment;
        }

        {
            let mut entity = entity.borrow_mut();

            entity.set_center(center + displacement);
            if collided {
                entity.set_velocity(velocity - original_displacement);
            } else {
                entity.set_velocity(velocity * (1.0 - weight));
            }
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

        let index_count = indices.len() as u32;

        (vertex_buffer, index_buffer, index_count)
    }
}
