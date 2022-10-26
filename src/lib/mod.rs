mod state;
pub mod mesh;
pub mod camera;
mod light;
mod vertex;
pub(crate) use vertex::Vertex;
pub mod automata;

use automata::Automata;

use camera::CameraConfig;

use mesh::objects;
use mesh::objects::MeshObject;

use std::time;

use winit::{
    event_loop,
    window::WindowBuilder,
    event,
    event::WindowEvent,
};

#[derive(Clone, Copy)]
pub struct Config {
    pub fps: usize,
    pub camera_config: camera::CameraConfig
}

fn update_mesh_from_automata(mesh: &mut mesh::Mesh, automata: &automata::Automata) {
    mesh.clear();

    let mut light = objects::Cube::new(
        [25, -2, 25].into(),
        [0.0; 3]
    );
    light.set_emitter(Some([1.0, 1.0, 1.0, 10.0].into()));
    mesh.add(light);

    for i in 0..(automata.size.x_len * automata.size.y_len * automata.size.z_len) {
        let point = automata.size.to_point(i);
        if let Some(color) = automata.states[automata.cells.lock().unwrap()[i]] {
            mesh.add(objects::Cube::new(
                point,
                color
            ));
        }
    }

}

pub async fn run(config: Config, mut automata: Automata) {

    // Contains all of the scene's geometry
    let mut mesh = mesh::Mesh::default();

    // Initialize the mesh
    update_mesh_from_automata(&mut mesh, &automata);

    // TODO: Not sure if this should be a member of State or remain separate
    let mut camera_controller = camera::CameraController::new(
        if let Some(zoom_speed) = config.camera_config.zoom_speed {
            zoom_speed
        } else {
            CameraConfig::default().zoom_speed.unwrap()
        }, 

        if let Some(rotate_speed) = config.camera_config.rotate_speed {
            rotate_speed
        } else {
            CameraConfig::default().rotate_speed.unwrap()
        }, 
    );

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
            automata.tick();
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