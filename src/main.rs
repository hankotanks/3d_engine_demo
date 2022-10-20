mod state;
mod mesh;
mod mesh_object;
mod camera;
mod camera_controller;
mod light;
mod vertex;
mod depth_texture;

use camera_controller::CameraController;
use mesh::Mesh;
use state::State;
use winit::{event_loop::{EventLoop, ControlFlow}, window::WindowBuilder, event::{self, WindowEvent}};

async fn run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = State::new(&window).await;
    let mesh = Mesh::default();
    let mut camera_controller = CameraController::new(0.025, 0.6);
    event_loop.run(move |event, _, control_flow| {
        match event {
            event::Event::RedrawRequested(w_id) if w_id == window.id() => {
                state.update(&mesh);

                match state.render() {
                    Ok(..) => {  },
                    Err(wgpu::SurfaceError::Lost) => state.redraw(),
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        *control_flow = ControlFlow::Exit 
                    },
                    Err(e) => eprintln!("{:?}", e)
                }
            },
            event::Event::MainEventsCleared => {
                window.request_redraw();
            },
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
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size)
                    },
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size) 
                    },
                    _ => {}
                }
            },
            event::Event::DeviceEvent { ref event, .. } => {
                camera_controller.process_events(event, &window, &mut state.camera);
            }

            _ => {  }
        }
    });
}

fn main() {
    pollster::block_on(
        run()
    );
}
