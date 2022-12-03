mod state;
mod light;

mod vertex;
pub use vertex::Vertex;

pub mod camera;
pub mod world;

use std::time;

use winit::{
    event_loop,
    window,
    event,
    event::WindowEvent, 
    dpi,
};

#[derive(Clone)]
pub struct Config {
    pub fps: usize
}

pub struct GameData<'a, 'b> {
    pub world: &'a mut world::World<'b>,
    pub camera: &'a mut camera::Camera,
}

pub async fn run<F: 'static, G: 'static>(
    config: Config, 
    mut update: F, 
    mut process_events: G
) where 
    F: FnMut(GameData), 
    G: FnMut(GameWindow, GameEvent, GameData) -> bool {

    // Initialize the Window and EventLoop
    let event_loop = event_loop::EventLoop::new();
    let window = window::WindowBuilder::new().build(&event_loop).unwrap();

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
                    _ => { 
                        if let Some(game_event) = GameEvent::from_window_event(event) {
                            if process_events(GameWindow::new(&window), game_event, GameData { world: &mut state.world, camera: &mut state.camera } ) {
                                window.request_redraw();
                            }
                        }
                    }
                }
            },

            // The user can capture events from the window...
            // ...which can affect both the mesh and the camera
            event::Event::DeviceEvent { ref event, .. } => {
                if let Some(game_event) = GameEvent::from_device_event(event) {
                    if process_events(GameWindow::new(&window), game_event, GameData { world: &mut state.world, camera: &mut state.camera } ) {
                        window.request_redraw();
                    }
                }   
            }

            // Update logic
            _ if accumulated_time >= fps => {
                update(GameData { world: &mut state.world, camera: &mut state.camera } );
                state.update();

                accumulated_time -= fps;
            }

            // Unhandled events
            _ => {  }
        }
    } );
}

pub struct GameWindow {
    window_dimensions: dpi::PhysicalSize<u32>,
}

impl GameWindow {
    pub fn new(window: &window::Window) -> Self {
        Self { window_dimensions: window.inner_size() }
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.window_dimensions.width, self.window_dimensions.height)
    }
}

pub enum GameEvent {
    Key { code: event::VirtualKeyCode, state: event::ElementState },
    MouseWheel { delta: event::MouseScrollDelta },
    MouseMoved { position: dpi::PhysicalPosition<f64> },
    MouseButton { button: event::MouseButton, state: event::ElementState },
}

impl GameEvent {
    pub(crate) fn from_device_event(event: &event::DeviceEvent) -> Option<Self> {
        match event {
            event::DeviceEvent::Added => None,
            event::DeviceEvent::Removed => None,
            event::DeviceEvent::MouseMotion { .. } => None,
            event::DeviceEvent::MouseWheel { delta } => Some(Self::MouseWheel { delta: *delta } ),
            event::DeviceEvent::Motion { .. } => None,
            event::DeviceEvent::Button { .. } => None,
            event::DeviceEvent::Key(input) => input.virtual_keycode.map(|kc| 
                Self::Key { code: kc, state: input.state }
            ),
            event::DeviceEvent::Text { .. } => None,
        }
    }

    pub(crate) fn from_window_event(event: &event::WindowEvent<'_>) -> Option<Self> {
        match event {
            WindowEvent::Resized(_) => None,
            WindowEvent::Moved(_) => None,
            WindowEvent::CloseRequested => None,
            WindowEvent::Destroyed => None,
            WindowEvent::DroppedFile(_) => None,
            WindowEvent::HoveredFile(_) => None,
            WindowEvent::HoveredFileCancelled => None,
            WindowEvent::ReceivedCharacter(_) => None,
            WindowEvent::Focused(_) => None,
            WindowEvent::KeyboardInput { .. } => None,
            WindowEvent::ModifiersChanged(_) => None,
            WindowEvent::CursorMoved { position, .. } => Some(Self::MouseMoved { position: *position } ),
            WindowEvent::CursorEntered { .. } => None,
            WindowEvent::CursorLeft { .. } => None,
            WindowEvent::MouseWheel { .. } => None,
            WindowEvent::MouseInput { button, state, .. } => Some(Self::MouseButton { button: *button, state: *state } ),
            WindowEvent::TouchpadPressure { .. } => None,
            WindowEvent::AxisMotion { .. } => None,
            WindowEvent::Touch(_) => None,
            WindowEvent::ScaleFactorChanged { .. } => None,
            WindowEvent::ThemeChanged(_) => None,
        }
    }
}