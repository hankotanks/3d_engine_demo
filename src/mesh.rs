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
    pub fn build_buffers(&self, device: &wgpu::Device) -> MeshBufferData {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        for (pos, obj) in self.objects.iter() {
            let index_offset = vertices.len() as u16;
            let mut data = obj.data(*pos);

            vertices.append(&mut data.vertices);
            indices.append(
                &mut data.indices.iter().map(|i| *i + index_offset).collect::<Vec<u16>>()
            );
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

        MeshBufferData { vertex_buffer, index_buffer, index_count }
    }
}

pub struct MeshBufferData {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32
}