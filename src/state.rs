use wgpu::util::DeviceExt;
use winit::window;

use crate::{camera::{Camera, CameraUniform}, vertex::Vertex, mesh::Mesh, light::LightUniform, depth_texture};


pub struct State {
    pub size: winit::dpi::PhysicalSize<u32>,
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
    pub camera: Camera,
    pub camera_uniform: CameraUniform,
    pub camera_buffer: wgpu::Buffer,
    pub camera_bind_group: wgpu::BindGroup,
    pub light_uniform: LightUniform,
    pub light_buffer: wgpu::Buffer,
    pub light_bind_group: wgpu::BindGroup,
    pub depth_texture_view: wgpu::TextureView,
    pub render_pipeline: wgpu::RenderPipeline
}

impl State {
    pub async fn new(window: &window::Window) -> Self {
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

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo
        };

        surface.configure(&device, &config);

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

        let camera = Camera::default();

        let mut camera_uniform = CameraUniform::new();
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
                        visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
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

        let light_uniform = LightUniform {
            position: [2.0, 6.0, 4.0, 1.0],
            color: [1.0, 1.0, 1.0, 0.1]
        };

        let light_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[light_uniform]),
                usage: { 
                    wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
                },
            }
        );

        let light_bind_group_layout = { 
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
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

        let light_bind_group = { 
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &light_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: light_buffer.as_entire_binding(),
                    }
                ],
                label: None
            }
        ) };

        let shader = device.create_shader_module(
            wgpu::include_wgsl!("shader.wgsl")
        );    

        let depth_texture_view = depth_texture::create_depth_texture(&device, &config);

        let render_pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[
                    &camera_bind_group_layout,
                    &light_bind_group_layout
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
                                format: config.format,
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
                    format: depth_texture::DEPTH_FORMAT,
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
            size,
            surface,
            device,
            queue,
            config,
            vertex_buffer,
            index_buffer,
            index_count,
            camera,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            light_uniform,
            light_buffer,
            light_bind_group,
            depth_texture_view,
            render_pipeline
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;

            self.depth_texture_view = depth_texture::create_depth_texture(&self.device, &self.config);

            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn redraw(&mut self) {
        self.resize(self.size);
    }

    pub fn update(&mut self, mesh: &Mesh) {
        let buffer_data = mesh.build_buffers(&self.device);
        
        self.vertex_buffer = buffer_data.vertex_buffer;
        self.index_buffer = buffer_data.index_buffer;
        self.index_count = buffer_data.index_count;

        self.camera_uniform.update_projection(&self.camera);
        self.queue.write_buffer(
            &self.camera_buffer, 
            0, 
            bytemuck::cast_slice(&[self.camera_uniform])
        );

        self.light_uniform.position = [
            self.camera_uniform.position[0],
            self.camera_uniform.position[1],
            self.camera_uniform.position[2],
            1.0
        ];

        self.queue.write_buffer(
            &self.light_buffer, 
            0, 
            bytemuck::cast_slice(&[self.light_uniform])
        );
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
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

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_bind_group(1, &self.light_bind_group, &[]);

            render_pass.set_vertex_buffer(
                0, 
                self.vertex_buffer.slice(..)
            );

            render_pass.set_index_buffer(
                self.index_buffer.slice(..), 
                wgpu::IndexFormat::Uint16
            );

            render_pass.draw_indexed(0..self.index_count, 0, 0..1);
        }
    
        self.queue.submit(
            std::iter::once(encoder.finish())
        );

        output.present();
    
        Ok(())
    }
}