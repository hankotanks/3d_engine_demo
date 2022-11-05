mod state;
mod light;
pub mod camera;

mod objects;
use objects::MeshObject;

pub mod automata;
use automata::Automata;
use automata::automata_index_to_point;

mod vertex;
pub(crate) use vertex::Vertex;

use std::{
    time, 
    sync,
    thread
};

use winit::{
    event_loop,
    window::WindowBuilder,
    event,
    event::WindowEvent,
};

use cgmath::Point3;

#[derive(Clone, Copy)]
pub struct Config<'a> {
    pub fps: usize,
    pub thread_count: usize,
    pub lighting: Lighting,
    pub states: &'a [(u8, [f32; 3])],
    pub camera_config: camera::CameraConfig
}

#[derive(Clone, Copy)]
pub enum Lighting {
    Bottom,
    Center,
    Corners,
    VonNeumann
}

impl Lighting {
    fn light_count(&self) -> usize {
        match self {
            Lighting::Bottom => 1,
            Lighting::Center => 1,
            Lighting::Corners => 8,
            Lighting::VonNeumann => 6,
        }
    }
}

pub fn run<F: 'static>(config: Config, automata: Automata, state_function: F) 
    where F: FnMut(&Automata, Point3<usize>) -> u8 + Send + Sync + Copy {
    pollster::block_on(run_automata(
        config,
        automata,
        state_function
    ));
}

async fn run_automata<'a, F: 'static>(config: Config<'a>, automata: Automata, mut state_function: F) 
    where F: FnMut(&Automata, Point3<usize>) -> u8 + Send + Sync + Copy {

    // Initialize the Window and EventLoop
    let event_loop = event_loop::EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // Contains ALL of the engine's mutable state...
    let mut state = state::State::new(&window, &config).await;

    // Get a copy of the Automata's Size member
    let size = automata.size;

    // generate all of the light positions
    let light_positions;
    {
        let width = size.x_len as isize;
        let height = size.y_len as isize;
        let depth = size.z_len as isize;

        light_positions = match config.lighting {
            Lighting::Bottom => vec![[width / 2, -2, depth / 2]],
            Lighting::Corners => vec![
                [-2; 3],
                [width + 1, -2, -2],
                [width + 1, height + 1, -2],
                [width + 1, height + 1, depth + 1],
                [-2, height + 1, -2],
                [-2, height + 1, depth + 1],
                [-2, -2, depth + 1],
                [width + 1, -2, depth + 1]
            ],
            Lighting::Center => vec![[width / 2, height / 2, depth / 2]],
            Lighting::VonNeumann => vec![
                [width / 2, -2, depth / 2],
                [width / 2, height + 1, depth / 2],
                [-2, height / 2, depth / 2],
                [width + 1, height / 2, depth / 2],
                [width / 2, height / 2, -2],
                [width / 2, height / 2, depth + 1]
            ]
        };
    } 

    // Helper function to quickly get LightCubes
    fn light(point: Point3<isize>) -> Box<objects::Gap> {
        let mut light = objects::Gap::new(point);
        light.set_light(Some([1.0, 1.0, 1.0, 4.0]));
        
        Box::new(light)
    }

    // Push every light source to the Mesh
    light_positions
        .into_iter()
        .for_each(|pos| state.mesh.push(light(pos.into())));

    // Wrap the Automata in an Arc so references can be handed to subthreads
    let automata = sync::Arc::new(sync::Mutex::new(automata));

    // ...except that related to frame time
    let fps = (config.fps as f32).recip();
    let mut accumulated_time = 0.0;
    let mut current = time::Instant::now();

    // Allow the cell states to be passed between threads
    let cell_states = config.states.to_vec();

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

                let automata_ref = sync::Arc::clone(&automata);
                threads.push(thread::spawn(move || {
                    let mut updated_states: Vec<(usize, u8)> = Vec::new();
                    for i in start..end {
                        let state = state_function(
                            &automata_ref.lock().unwrap(), 
                            automata_index_to_point(size, i)
                        );

                        if state != automata_ref.lock().unwrap().cells[i] {
                            updated_states.push((i, state));
                        }
                    }

                    updated_states
                } ));
            }

            // Assemble a vec of all changed cell states
            let mut updated_states: Vec<(usize, u8)> = Vec::new();
            for handle in threads.drain(0..) {
                updated_states.append(&mut handle.join().unwrap());
            }

            // Write the changed cell states
            for (index, state) in updated_states.drain(0..) {
                automata.lock().unwrap().cells[index] = state;
            }

            // Truncate the mesh to retain ONLY light sources
            state.mesh.truncate(config.lighting.light_count());

            // Update the mesh to account for changed cell states
            let automata_temp = automata.lock().unwrap();
            for (point, current_state) in automata_temp.iter().with_coord() {
                let point = [
                    point.x as isize,
                    point.y as isize,
                    point.z as isize
                ].into();
                
                // Find the appropriate state, and draw the right-colored cell
                'builder: for (cell_state, color) in cell_states.iter().cloned() {
                    if cell_state == current_state {
                        state.mesh.push(Box::new(objects::Cube::new(
                            point,
                            color
                        )));    
        
                        break 'builder;
                    }
                }
            }

            // Update loop clock
            accumulated_time -= fps;
        }

        match event {
            event::Event::RedrawRequested(w_id) if w_id == window.id() => {
                state.update();
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