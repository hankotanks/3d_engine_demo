mod state;
pub mod mesh;
pub mod camera;
mod light;
mod vertex;
pub(crate) use vertex::Vertex;
pub mod automata;

use automata::Automata;

use mesh::objects;
use mesh::objects::MeshObject;

use std::{time, sync::Arc, thread};

use winit::{
    event_loop,
    window::WindowBuilder,
    event,
    event::WindowEvent,
};

#[derive(Clone, Copy)]
pub struct Config {
    pub fps: usize,
    pub thread_count: usize,
    pub camera_config: camera::CameraConfig
}

fn update_mesh_from_automata(mesh: &mut mesh::Mesh, automata: &automata::Automata) {
    // TODO: Need a better system for managing lights...
    // TODO: Maybe an enum with options like Corners, Center
    // And keep track of the number of lights

    if mesh.len() == 0 {
        let mut light = objects::Cube::new(
            [(automata.size.x_len / 2) as isize, -1, (automata.size.z_len / 2) as isize].into(),
            [0.0; 3]
        );
        light.set_emitter(Some([1.0, 1.0, 1.0, 10.0].into()));
        mesh.push(Box::new(light));
    } else {
        mesh.truncate(1);
    }

    for i in 0..(automata.size.x_len * automata.size.y_len * automata.size.z_len) {
        let point = automata.size.to_point(i);
        if let Some(object) = (automata.cube_function)(point, automata.cells.lock().unwrap()[i]) {
            mesh.push(object)
        }
    }
}

pub async fn run(config: Config, automata: Automata) {

    // Contains all of the scene's geometry
    let mut mesh = mesh::Mesh::default();

    // Initialize the mesh
    update_mesh_from_automata(&mut mesh, &automata);

    let event_loop = event_loop::EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // Contains ALL of the engine's mutable state...
    let mut state = state::State::new(&window, config).await;

    // ...except that related to frame time
    let fps = (config.fps as f32).recip();
    let mut accumulated_time = 0.0;
    let mut current = time::Instant::now();

    // The game loop itself
    event_loop.run(move |event, _, control_flow| {
        accumulated_time += current.elapsed().as_secs_f32();
        current = time::Instant::now();
        if accumulated_time >= fps { 
            let mut threads = Vec::new();
            for c in 0..config.thread_count {
                let length = automata.cells.lock().unwrap().len();
                let start = length / config.thread_count * c;
                let end = length / config.thread_count * (c + 1);

                let cells_reference = Arc::clone(&automata.cells);
                let size_reference = Arc::clone(&automata.size);
                let state_function_reference = Arc::clone(&automata.state_function);
                threads.push(thread::spawn(move || {
                    let mut updated_states: Vec<(usize, usize)> = Vec::new();
                    for i in start..end {
                        let state = state_function_reference(
                            cells_reference.lock().unwrap().as_mut(), 
                            *size_reference, 
                            i
                        );

                        if state != cells_reference.lock().unwrap()[i] {
                            updated_states.push((i, state));
                        }
                    }

                    updated_states
                } ));
            }

            let mut updated_states: Vec<(usize, usize)> = Vec::new();
            for handle in threads.drain(0..) {
                updated_states.append(&mut handle.join().unwrap());
            }

            for (index, state) in updated_states.drain(0..) {
                automata.cells.lock().unwrap()[index] = state;
            }
            update_mesh_from_automata(&mut mesh, &automata);
            accumulated_time -= fps;
        }

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
                state.camera_controller.process_events(
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