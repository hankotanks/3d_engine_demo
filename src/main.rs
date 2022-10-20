mod state;
mod mesh;
mod camera;
mod light;
mod vertex;
mod depth_texture;

use winit::{
    event_loop,
    window::WindowBuilder,
    event,
    event::WindowEvent,
};

async fn run() {
    // Contains all of the scene's geometry
    let mesh = mesh::Mesh::default();

    // TODO: Not sure if this should be a member of State or remain separate
    let mut camera_controller = camera::CameraController::new(0.025, 0.6);

    let event_loop = event_loop::EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // Contains ALL of the engine's mutable state
    let mut state = state::State::new(&window).await;

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
            event::Event::DeviceEvent { ref event, .. } => {
                camera_controller.process_events(
                    event, 
                    &window, 
                    &mut state.camera
                );
            }

            // Unhandled events
            _ => {  }
        }
    });
}

fn main() {
    pollster::block_on(
        run()
    );
}
