#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub normal: [f32; 3]
}

impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 3] = { 
        wgpu::vertex_attr_array![
            0 => Float32x3, 
            1 => Float32x3, 
            2 => Float32x3
        ] 
    };

    pub fn description<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}