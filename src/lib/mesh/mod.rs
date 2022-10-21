pub mod objects;

use objects::MeshObject;
use objects::Cube;
use wgpu::util::DeviceExt;

pub struct Mesh {
    pub(crate) objects: Vec<Box<dyn MeshObject>>
}

impl Default for Mesh {
    fn default() -> Self {
        Self {
            objects: vec![Box::new(Cube::default())]
        }
    }
}

impl Mesh {
    pub fn add<M: 'static>(&mut self, mesh_object: M) -> bool where M: MeshObject {
        for object in self.objects.iter() {
            if mesh_object.position() == object.position() {
                return false;
            }
        }

        self.objects.push(Box::new(mesh_object));

        true
    }

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