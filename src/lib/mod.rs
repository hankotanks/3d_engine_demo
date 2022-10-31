mod state;
pub mod mesh;
pub mod camera;
mod light;
mod vertex;
use cgmath::Point3;
pub(crate) use vertex::Vertex;
pub mod automata;

use automata::Automata;

use mesh::objects;
use mesh::objects::MeshObject;

use std::{time, sync::{Arc, Mutex}, thread};

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
    pub lighting: Lighting,
    pub camera_config: camera::CameraConfig
}

#[derive(Clone, Copy)]
pub enum Lighting {
    CenterBottom = 1,
    Corners = 8
}

fn update_mesh_from_automata(
    mesh: &mut mesh::Mesh, 
    automata: &automata::Automata, 
    states: &[(usize, [f32; 3])], 
    lighting: Lighting) 
{
    if mesh.len() == 0 {
        let size_x = automata.size.x_len as isize;
        let size_y = automata.size.y_len as isize;
        let size_z = automata.size.z_len as isize;

        match lighting {
            Lighting::CenterBottom => {
                let mut light = objects::Cube::new(
                    [size_x / 2, -1, size_z / 2].into(),
                    [0.0; 3]
                );
                light.set_emitter(Some([1.0, 1.0, 1.0, 2.0].into()));

                mesh.push(Box::new(light));
            },
            Lighting::Corners => {
                let light_positions: [[isize; 3]; 8] = [
                    [-1; 3],
                    [size_x, -1, -1],
                    [size_x, size_y, -1],
                    [size_x, size_y, size_z],
                    [-1, size_y, -1],
                    [-1, size_y, size_z],
                    [-1, -1, size_z],
                    [size_x, -1, size_z]
                ];
                
                for pos in light_positions.into_iter() {
                    let mut light = objects::Cube::new(pos.into(), [0.0; 3]);
                    light.set_emitter(Some([1.0, 1.0, 1.0, 2.0].into()));
                    mesh.push(Box::new(light));
                }
            }
        }
    } else {
        mesh.truncate(lighting as usize)
    }

    for (point, cell_state) in automata.iter().with_coord() {
        let point = [
            point.x as isize,
            point.y as isize,
            point.z as isize
        ].into();
        
        'builder: for (state, color) in states.iter().cloned() {
            if state == cell_state {
                mesh.push(Box::new(objects::Cube::new(
                    point,
                    color
                )));    

                break 'builder;
            }
        }
    }
}

pub async fn run<F: 'static>(config: Config, automata: Automata, state_function: F, states: &[(usize, [f32; 3])]) 
    where F: Fn(&Automata, Point3<usize>) -> usize + Send + Sync + Copy {

    let automata = Arc::new(Mutex::new(automata));

    // Contains all of the scene's geometry
    let mut mesh = mesh::Mesh::default();

    // Initialize the mesh
    //update_mesh_from_automata(&mut mesh, &automata);

    let event_loop = event_loop::EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // Contains ALL of the engine's mutable state...
    let mut state = state::State::new(&window, config).await;

    // ...except that related to frame time
    let fps = (config.fps as f32).recip();
    let mut accumulated_time = 0.0;
    let mut current = time::Instant::now();

    let cell_states = states.to_vec();

    // The game loop itself
    event_loop.run(move |event, _, control_flow| {
        accumulated_time += current.elapsed().as_secs_f32();
        current = time::Instant::now();
        if accumulated_time >= fps { 
            let mut threads = Vec::new();
            for c in 0..config.thread_count {
                let length = automata.lock().unwrap().cells.len();
                let start = length / config.thread_count * c;
                let end = length / config.thread_count * (c + 1);

                let automata_ref = Arc::clone(&automata);
                threads.push(thread::spawn(move || {
                    let size = automata_ref.lock().unwrap().size;
                   
                    let mut updated_states: Vec<(usize, usize)> = Vec::new();
                    for i in start..end {
                        let y = i / (size.x_len * size.z_len);
                        let index = i - y * size.x_len * size.z_len;
                        let z = index / size.x_len;
                        let x = index % size.x_len;
    
                        let target = [x, y, z].into();
                        let state = state_function(
                            &automata_ref.lock().unwrap(), 
                            target
                        );

                        if state != automata_ref.lock().unwrap().cells[i] {
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
                automata.lock().unwrap().cells[index] = state;
            }

            update_mesh_from_automata(&mut mesh, &automata.lock().unwrap(), &cell_states, config.lighting);
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
            event::Event::DeviceEvent { ref event, .. } => {
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