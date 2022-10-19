use std::collections::HashMap;

use cgmath::Point3;
use wgpu::util::DeviceExt;

use crate::mesh_object::{Object, ObjectShape};

pub struct Mesh {
    pub objects: HashMap<Point3<isize>, Object>
}

impl Default for Mesh {
    fn default() -> Self {
        Self {
            objects: {
                let mut objects = HashMap::new();
                
                objects.insert(
                    Point3::new(0, 0, 0),
                    Object {
                        shape: ObjectShape::Cube,
                        color: [1.0, 1.0, 1.0]
                    }
                );

                objects
            }
        }
    }
}

impl Mesh {
    pub fn index_count(&self) -> u32 {
        36
    }

    pub fn build_vertex_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        let origin = Point3::new(0, 0, 0);
        device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(self.objects[&origin].data(origin).vertices.as_slice()),
                usage: wgpu::BufferUsages::VERTEX
            }
        )
    }

    pub fn build_index_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        let origin = Point3::new(0, 0, 0);
        device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(self.objects[&origin].data(origin).indices.as_slice()),
                usage: wgpu::BufferUsages::INDEX,
            }
        )
    }
}