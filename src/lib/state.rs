use winit::window;

use wgpu::util::DeviceExt;

use crate::{
    camera,
    Vertex,
    objects::MeshObject,
    light
};

pub(crate) struct State {
    pub mesh: Vec<Box<dyn MeshObject>>,

    pub(crate) size: winit::dpi::PhysicalSize<u32>,
    pub(crate) surface: wgpu::Surface,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    pub(crate) surface_config: wgpu::SurfaceConfiguration,
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) index_buffer: wgpu::Buffer,
    pub(crate) index_count: u32,
    pub(crate) camera: camera::Camera,
    pub(crate) camera_uniform: camera::CameraUniform,
    pub(crate) camera_buffer: wgpu::Buffer,
    pub(crate) camera_bind_group: wgpu::BindGroup,
    pub(crate) lighting: light::LightSources,
    pub(crate) lighting_buffer: wgpu::Buffer,
    pub(crate) lighting_bind_group: wgpu::BindGroup,
    pub(crate) depth_texture_view: wgpu::TextureView,
    pub(crate) render_pipeline: wgpu::RenderPipeline
}

impl State {
    pub async fn new(window: &window::Window) -> Self {
        let mesh = Vec::new();

        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::Backends::all());

        let surface = unsafe { 
            instance.create_surface(window) 
        };

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: { 
                    if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    }
                },
                label: None
            },
            None
        ).await.unwrap();

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo
        };

        surface.configure(&device, &surface_config);

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: &[],
                usage: wgpu::BufferUsages::VERTEX
            }
        );

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: &[],
                usage: wgpu::BufferUsages::INDEX,
            }
        );

        let index_count = 0u32;

        let camera = camera::Camera::default();

        let mut camera_uniform = camera::CameraUniform::new();
        camera_uniform.update_projection(&camera);

        let camera_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: { 
                    wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
                },
            }
        );

        let camera_bind_group_layout = { 
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX 
                            | wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }
                ],
                label: None
            }
        ) };
        
        let camera_bind_group = { 
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &camera_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: camera_buffer.as_entire_binding(),
                    }
                ],
                label: None
            }
        ) };

        let lighting = light::LightSources {
            light_uniforms: [
                light::Light::default(); 
                light::MAX_LIGHT_SOURCES
            ]
        };

        let lighting_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[lighting]),
                usage: { 
                    wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST
                },
            }
        );

        let lighting_bind_group_layout = { 
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX 
                            | wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }
                ],
                label: None
            }
        ) };

        let lighting_bind_group = { 
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &lighting_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: lighting_buffer.as_entire_binding(),
                    }
                ],
                label: None
            }
        ) };

        let shader = device.create_shader_module(
            wgpu::include_wgsl!("shader.wgsl")
        );    

        let depth_texture_view = create_depth_texture(&device, &surface_config);

        let render_pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[
                    &camera_bind_group_layout,
                    &lighting_bind_group_layout
                ],
                push_constant_ranges: &[]
            }
        );

        let render_pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[
                        Vertex::description()
                    ]
                },
                fragment: Some(
                    wgpu::FragmentState {
                        module: &shader,
                        entry_point: "fs_main",
                        targets: &[
                            Some(wgpu::ColorTargetState {
                                format: surface_config.format,
                                blend: Some(wgpu::BlendState::REPLACE),
                                write_mask: wgpu::ColorWrites::ALL
                            } )
                        ],
                    }
                ),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: wgpu::TextureFormat::Depth32Float,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default()
                } ),
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None
            }
        );

        Self {
            mesh,
            size,
            surface,
            device,
            queue,
            surface_config,
            vertex_buffer,
            index_buffer,
            index_count,
            camera,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            lighting,
            lighting_buffer,
            lighting_bind_group,
            depth_texture_view,
            render_pipeline
        }
    }

    pub(crate) fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.surface_config.width = new_size.width;
            self.surface_config.height = new_size.height;

            self.depth_texture_view = create_depth_texture(
                &self.device, 
                &self.surface_config
            );

            self.surface.configure(&self.device, &self.surface_config);
        }
    }

    pub(crate) fn update(&mut self) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        for object in self.mesh.iter() {
            let mut data = object.build_object_data();

            let mut offset_indices = data.indices
                .iter()
                .map(|i| *i + vertices.len() as u32)
                .collect::<Vec<u32>>();

            indices.append(&mut offset_indices);
            vertices.append(&mut data.vertices);
        }

        self.vertex_buffer = self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(vertices.as_slice()),
                usage: wgpu::BufferUsages::VERTEX
            }
        );

        self.index_buffer = self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(indices.as_slice()),
                usage: wgpu::BufferUsages::INDEX,
            }
        );

        self.index_count = indices.len() as u32;

        // Update light sources
        let mut light_count = 0;
        for object in self.mesh.iter() {
            if let Some(light) = object.light() {
                self.lighting.light_uniforms[light_count].color = light;
                self.lighting.light_uniforms[light_count].position = [
                    object.position().x as f32,
                    object.position().y as f32,
                    object.position().z as f32,
                    1.0
                ];

                light_count += 1;
            }
        }

        self.queue.write_buffer(
            &self.lighting_buffer, 
            0, 
            bytemuck::cast_slice(&[self.lighting])
        );

        self.camera_uniform.update_projection(&self.camera);
        self.queue.write_buffer(
            &self.camera_buffer, 
            0, 
            bytemuck::cast_slice(&[self.camera_uniform])
        );
    }

    pub(crate) fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(
            &wgpu::TextureViewDescriptor::default()
        );

        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: None,
            }
        );

        {
            let mut render_pass = encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[
                        Some(
                            wgpu::RenderPassColorAttachment {
                                view: &view,
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(
                                        wgpu::Color::BLACK
                                    ),
                                    store: true
                                },
                            }
                        )
                    ],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &self.depth_texture_view,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: true,
                        } ),
                        stencil_ops: None,
                    } )
                }
            );

            // Set render pipeline
            render_pass.set_pipeline(&self.render_pipeline);

            // Camera and light bind groups
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_bind_group(1, &self.lighting_bind_group, &[]);

            // Set vertex and index buffers
            render_pass.set_vertex_buffer(
                0, 
                self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(
                self.index_buffer.slice(..), 
                wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..self.index_count, 0, 0..1);
        }
    
        self.queue.submit(
            std::iter::once(encoder.finish())
        );

        output.present();
    
        Ok(())
    }
}

pub(crate) fn create_depth_texture(
    device: &wgpu::Device, 
    config: &wgpu::SurfaceConfiguration
) -> wgpu::TextureView {
    let size = wgpu::Extent3d {
        width: config.width,
        height: config.height,
        depth_or_array_layers: 1
    };

    let desc = wgpu::TextureDescriptor {
        label: None,
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT
            | wgpu::TextureUsages::TEXTURE_BINDING
    };

    let texture = device.create_texture(&desc);
    texture.create_view(&wgpu::TextureViewDescriptor::default())
}