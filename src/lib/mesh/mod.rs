pub mod objects;

use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::Index;
use std::ops::IndexMut;

use objects::MeshObject;
use wgpu::util::DeviceExt;

#[derive(Default)]
pub struct Mesh {
    pub(crate) objects: Vec<Box<dyn MeshObject>>
}

impl Index<usize> for Mesh {
    type Output = Box<dyn MeshObject>;
    fn index(&self, i: usize) -> &Self::Output {
        &self.objects[i]
    }
}

impl IndexMut<usize> for Mesh {
    fn index_mut(&mut self, i: usize) -> &mut Box<dyn MeshObject> {
        &mut self.objects[i]
    }
}

impl Deref for Mesh {
    type Target = Vec<Box<dyn MeshObject>>;

    fn deref(&self) -> &Self::Target {
        &self.objects
    }
}

impl DerefMut for Mesh {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.objects
    }
}

impl Mesh {
    pub(crate) fn build_buffers(&self, device: &wgpu::Device) -> MeshBufferData {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        for object in self.objects.iter() {
            let mut data = object.build_object_data();

            let mut offset_indices = data.indices
                .iter()
                .map(|i| *i + vertices.len() as u16)
                .collect::<Vec<u16>>();

            indices.append(&mut offset_indices);
            vertices.append(&mut data.vertices);
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

pub(crate) struct MeshBufferData {
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) index_buffer: wgpu::Buffer,
    pub(crate) index_count: u32
}