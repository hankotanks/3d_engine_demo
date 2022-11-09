mod state;
mod light;
pub mod camera;

mod objects;
use objects::MeshObject;

pub mod automata;
use automata::{Automata, Neighborhood};

mod vertex;
pub(crate) use vertex::Vertex;

use std::{
    time, 
    sync,
    thread::{self, JoinHandle}, collections::HashMap
};

use winit::{
    event_loop,
    window::WindowBuilder,
    event,
    event::WindowEvent,
};

use cgmath::Point3;

#[derive(Clone)]
pub struct Config {
    pub fps: usize,
    pub thread_count: usize,
    pub lighting: Lighting,
    pub states: HashMap<u8, [f32; 3]>,
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
    where F: FnMut(Neighborhood, u8) -> u8 + Send + Sync + Copy {
    pollster::block_on(run_automata(
        config,
        automata,
        state_function
    ));
}

async fn run_automata<F: 'static>(config: Config, mut automata: Automata, state_function: F) 
    where F: FnMut(Neighborhood, u8) -> u8 + Send + Sync + Copy {

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
        let (width, height, depth) = (size.x_len as i16, size.y_len as i16, size.z_len as i16);
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
    fn light(point: Point3<i16>) -> Box<objects::Gap> {
        let mut light = objects::Gap::new(point);
        light.set_light(Some([1.0, 1.0, 1.0, 4.0]));
        
        Box::new(light)
    }

    // Push every light source to the Mesh
    light_positions
        .into_iter()
        .for_each(|pos| state.mesh.push(light(pos.into())));

    // Wrap the Automata in an Arc so references can be handed to subthreads
    //let automata = sync::Arc::new(sync::Mutex::new(automata));

    // ...except that related to frame time
    let fps = (config.fps as f32).recip();
    let mut accumulated_time = 0.0;
    let mut current = time::Instant::now();

    let mut automata_thread: Option<thread::JoinHandle<Vec<(Point3<i16>, u8)>>> = None;

    // The game loop itself
    event_loop.run(move |event, _, control_flow| {
        accumulated_time += current.elapsed().as_secs_f32();
        current = time::Instant::now();
        
        if let Some(handle) = &automata_thread {
            if handle.is_finished() && accumulated_time >= fps {
                let handle: JoinHandle<Vec<(Point3<i16>, u8)>> = automata_thread.take().unwrap();
                for (index, cell_state) in handle.join().unwrap().drain(0..) {
                    automata[index] = cell_state;
                }

                // Truncate the mesh to retain ONLY light sources
                state.mesh.truncate(config.lighting.light_count());

                // Update the mesh to account for changed cell states
                for point in automata.iter() {
                    let current_state = automata[point];
                    
                    // Find the appropriate state, and draw the right-colored cell
                    if let Some(color) = config.states.get(&current_state) {
                        state.mesh.push(Box::new(objects::Cube::new(
                            point,
                            *color
                        )));   
                    }
                }

                accumulated_time -= fps;
            }

        } else if accumulated_time >= fps {
            let automata_ref = sync::Arc::new(automata.clone());
            automata_thread = Some(thread::spawn(move || tick(
                config.thread_count, 
                automata_ref, 
                state_function
            )));
        }            

        match event {
            event::Event::RedrawRequested(w_id) if w_id == window.id() => {
                /*  */
                let RTIME = std::time::Instant::now();
                /*  */

                state.update();

                /*  */
                dbg!(RTIME.elapsed());
                /*  */

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

fn tick<F: 'static>(thread_count: usize, automata: sync::Arc<Automata>, mut state_function: F) -> Vec<(Point3<i16>, u8)>
    where F: FnMut(Neighborhood, u8) -> u8 + Send + Sync + Copy {
    /*  */
    let ATIME = std::time::Instant::now();
    /*  */

    let updated_states = automata.iter().filter_map(|point| {
        let cell_state = state_function(automata.von_neumann_neighborhood(point), automata[point]);

        if cell_state != automata[point] {
            Some((point, cell_state))
        } else {
            None
        }
    } ).collect::<Vec<(Point3<i16>, u8)>>();

    /*  */
    dbg!(ATIME.elapsed());
    /*  */

    updated_states
}