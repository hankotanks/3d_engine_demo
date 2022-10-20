pub const MAX_LIGHT_UNIFORMS: usize = 8;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightUniform {
    pub position: [f32; 4],
    pub color: [f32; 4]
}

impl Default for LightUniform {
    fn default() -> Self {
        Self { position: [0.0; 4], color: [0.0; 4] }
    }
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Lights {
    pub light_count: u32,
    pub light_uniforms: [LightUniform; MAX_LIGHT_UNIFORMS]
}