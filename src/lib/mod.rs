mod state;
mod light;

mod vertex;
pub use vertex::Vertex;

pub mod camera;
pub mod world;

use std::time;

use winit::{
    event_loop,
    window::WindowBuilder,
    event,
    event::WindowEvent,
};

#[derive(Clone)]
pub struct Config {
    pub fps: usize
}

pub async fn run<F: 'static, G: 'static>(
    config: Config, 
    mut update: F, 
    mut process_events: G
) where 
    F: FnMut(&mut camera::Camera, &mut world::World, &mut Vec<Box<dyn world::Entity>>), 
    G: FnMut(&event::DeviceEvent, &mut camera::Camera, &mut world::World, &mut Vec<Box<dyn world::Entity>>) -> bool {

    // Initialize the Window and EventLoop
    let event_loop = event_loop::EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // Contains ALL of the engine's mutable state...
    let mut state = state::State::new(&window).await;

    // ...except that related to frame time
    let fps = (config.fps as f32).recip();
    let mut accumulated_time = 0.0;
    let mut current = time::Instant::now();

    // The game loop itself
    event_loop.run(move |event, _, control_flow| {
        accumulated_time += current.elapsed().as_secs_f32();
        current = time::Instant::now();        

        match event {
            event::Event::RedrawRequested(w_id) if w_id == window.id() => {
                match state.render() {
                    Ok(..) => {  },
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        *control_flow = event_loop::ControlFlow::Exit 
                    },
                    Err(e) => eprintln!("{:?}", e)
                }
            },

            // Redraw
            event::Event::MainEventsCleared => { 
                window.request_redraw(); 
            },

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
                    _ => {  }
                }
            },

            // The user can capture events from the window...
            // ...which can affect both the mesh and the camera
            event::Event::DeviceEvent { ref event, .. } => {
                if process_events(event, &mut state.camera, &mut state.world, &mut state.entities) {
                    window.request_redraw();
                }
            }

            // Update logic
            _ if accumulated_time >= fps => {
                update(&mut state.camera, &mut state.world, &mut state.entities);
                state.update();

                accumulated_time -= fps;
            }

            // Unhandled events
            _ => {  }
        }
    } );
}