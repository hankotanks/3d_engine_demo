mod state;
pub mod mesh;
pub mod camera;
mod light;
mod vertex;
pub(crate) use vertex::Vertex;

use winit::{
    event_loop,
    window::WindowBuilder,
    event,
    event::WindowEvent,
};

#[derive(Clone, Copy)]
pub struct Config {
    pub frame_speed: f32,
    pub camera_config: camera::CameraConfig
}

pub async fn run<F: 'static, G: 'static>(config: Config, mut mesh_init: F, mut mesh_update: G) 
    where F: FnMut(&mut mesh::Mesh), G: FnMut(&mut mesh::Mesh) {

    // Contains all of the scene's geometry
    let mut mesh = mesh::Mesh::default();

    // Initialize the mesh
    mesh_init(&mut mesh);

    // TODO: Not sure if this should be a member of State or remain separate
    let mut camera_controller = camera::CameraController::new(
        config.camera_config.rotate_speed, config.camera_config.zoom_speed
    );

    let event_loop = event_loop::EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // Contains ALL of the engine's mutable state
    let mut state = state::State::new(&window, config).await;

    let frame_update_speed = config.frame_speed.recip() as i32;
    let mut frame_update_count = 0;
    
    event_loop.run(move |event, _, control_flow| {
        match event {
            event::Event::RedrawRequested(w_id) if w_id == window.id() => {                
                state.update(&mesh);

                match state.render() {
                    Ok(..) => {  },
                    Err(wgpu::SurfaceError::Lost) => state.redraw(),
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        *control_flow = event_loop::ControlFlow::Exit 
                    },
                    Err(e) => eprintln!("{:?}", e)
                }
            },

            // Redraw
            event::Event::MainEventsCleared => { window.request_redraw(); },

            // Handle close and resize events
            event::Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                match event {
                    // Handle close behavior
                    WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                        input:
                        event::KeyboardInput {
                                state: event::ElementState::Pressed,
                                virtual_keycode: Some(
                                    event::VirtualKeyCode::Escape
                                ),
                                ..
                            },
                        ..
                    } => *control_flow = event_loop::ControlFlow::Exit,

                    // When the window is resized from a single edge
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size)
                    },

                    // Scaled with a drag-click from the corner of the window
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size) 
                    },

                    // Unhandled behavior
                    _ => {}
                }
            },

            // Any mouse inputs are sent directly to the camera controller
            event::Event::DeviceEvent { ref event, .. } => if !config.camera_config.locked {
                camera_controller.process_events(
                    event, 
                    &window, 
                    &mut state.camera
                );
            }

            // Unhandled events
            _ => {  }
        }

        if frame_update_count == frame_update_speed {
            mesh_update(&mut mesh);
            frame_update_count = 0;
        } else {
            frame_update_count += 1;
        }
    });
}