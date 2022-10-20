mod objects;

use objects::MeshObject;
use objects::Cube;

use wgpu::util::DeviceExt;

pub struct Mesh {
    pub objects: Vec<Box<dyn MeshObject>>
}

impl Default for Mesh {
    fn default() -> Self {
        Self {
            objects: vec![Box::new(Cube::default())]
        }
    }
}

impl Mesh {
    pub fn build_buffers(&self, device: &wgpu::Device) -> MeshBufferData {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        for object in self.objects.iter() {
            let mut data = object.data();

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

pub struct MeshBufferData {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32
}