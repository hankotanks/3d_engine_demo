use block_engine_wgpu::{camera, GameEvent, GameWindow};
use cgmath::{Vector3, Zero};
use winit::event;

const ZOOM_SPEED: f32 = 0.6;

pub mod directions {
    pub const UP: u8 = 1 << 0;
    pub const DOWN: u8 = 1 << 1;
    pub const LEFT: u8 = 1 << 2;
    pub const RIGHT: u8 = 1 << 3;

    pub fn x_signum(direction: u8) -> f32 {
        match (direction >> 2) & 3 {
            0 | 3 => 0.0,
            1 => -1.0,
            2 => 1.0,
            _ => unreachable!()
        }
    }

    pub fn z_signum(direction: u8) -> f32 {
        match direction & 3 {
            0 | 3 => 0.0,
            1 => -1.0,
            2 => 1.0,
            _ => unreachable!()
        }
    }
}

pub struct PlayerController {
    pub direction: u8,
    pub acceleration: f32,
    pub pressed: bool,
    pub current_drag_vector: Vector3<f32>,
}

impl PlayerController {
    pub fn process_events(
        &mut self,
        window: GameWindow,
        event: GameEvent,
        camera: &mut camera::Camera,
    ) {
        match &event {    
            // Zoom
            GameEvent::MouseWheel { delta } => {
                let scroll_amount = -1.0 * match delta {
                    // A mouse line is about 1 px.
                    event::MouseScrollDelta::LineDelta(_, scroll) => 
                        scroll * 1.0,
                    event::MouseScrollDelta::PixelDelta(
                        winit::dpi::PhysicalPosition { y: scroll, .. }
                    ) => *scroll as f32
                };
    
                camera.add_distance(scroll_amount * ZOOM_SPEED);
            }

            GameEvent::MouseButton { 
                button: event::MouseButton::Left, 
                state 
            } => {
                self.pressed = match state {
                    event::ElementState::Pressed => true,
                    event::ElementState::Released => false,
                };
            }

            GameEvent::MouseMoved { mut position } if self.pressed => {
                let (hw, hh) = (window.dimensions().0 as f64 / 2.0, window.dimensions().1 as f64 / 2.0);

                position.x -= hw;
                position.y -= hh;
                let degree = (position.x / hw, position.y/ hh);

                self.current_drag_vector = Vector3::new(degree.0 as f32, 0.0, degree.1 as f32)
            }
    
            // Player pressed movement keys
            GameEvent::Key { 
                code: event::VirtualKeyCode::Left, 
                state: event::ElementState::Pressed 
            } if (self.direction >> 3) & 1 != 1 => self.direction |= directions::LEFT,

            GameEvent::Key { 
                code: event::VirtualKeyCode::Right, 
                state: event::ElementState::Pressed 
            } if (self.direction >> 2) & 1 != 1 => self.direction |= directions::RIGHT,

            GameEvent::Key { 
                code: event::VirtualKeyCode::Up, 
                state: event::ElementState::Pressed 
            } if (self.direction >> 1) & 1 != 1 => self.direction |= directions::UP,

            GameEvent::Key { 
                code: event::VirtualKeyCode::Down, 
                state: event::ElementState::Pressed 
            } if self.direction & 1 != 1 => self.direction |= directions::DOWN,

            GameEvent::Key { 
                code: event::VirtualKeyCode::Left, 
                state: event::ElementState::Released
            } => self.direction &= !directions::LEFT,

            GameEvent::Key { 
                code: event::VirtualKeyCode::Right, 
                state: event::ElementState::Released
            } => self.direction &= !directions::RIGHT,

            GameEvent::Key { 
                code: event::VirtualKeyCode::Up, 
                state: event::ElementState::Released
            } => self.direction &= !directions::UP,

            GameEvent::Key { 
                code: event::VirtualKeyCode::Down, 
                state: event::ElementState::Released
            } => self.direction &= !directions::DOWN,

            _ => {  }
        }
    }

    pub fn spawn_projectile(&mut self) -> Option<Vector3<f32>> {
        if !self.pressed && !self.current_drag_vector.is_zero() {
            let drag_vector = self.current_drag_vector;
            self.current_drag_vector = Vector3::new(0.0, 0.0, 0.0);

            return Some(drag_vector);
        }

        None
    }

    pub fn aggregate_player_velocity(&mut self, velocity: &mut Vector3<f32>) {
        velocity.x += self.acceleration * directions::x_signum(self.direction);
        velocity.z += self.acceleration * directions::z_signum(self.direction);
    }
}

/*
// Handle the start and end of mouse drags
event::DeviceEvent::Button {
    #[cfg(target_os = "macos")] button: 0,
    #[cfg(not(target_os = "macos"))] button: 1,
    state,
} => {
    let is_pressed = *state == event::ElementState::Pressed;
    self.is_drag_rotate = is_pressed;
} 

// Rotation
event::DeviceEvent::MouseMotion { delta } => {
    if self.is_drag_rotate {
        camera.add_yaw(-1.0 * delta.0 as f32 * self.rotate_speed);
        camera.add_pitch(delta.1 as f32 * self.rotate_speed);
        
        return true;
    }
} */