use block_engine_wgpu::camera;
use cgmath::Vector3;
use winit::event;

const ZOOM_SPEED: f32 = 0.6;

pub mod directions {
    pub const UP: u8 = 1 << 0;
    pub const DOWN: u8 = 1 << 1;
    pub const LEFT: u8 = 1 << 2;
    pub const RIGHT: u8 = 1 << 3;
}

pub struct PlayerController {
    pub direction: u8,
    pub previous_direction: u8,
    pub speed: f32,
    pub acceleration: f32,
}

impl PlayerController {
    pub fn process_events(
        &mut self,
        event: &event::DeviceEvent, 
        camera: &mut camera::Camera,
    ) {
        let temp = self.direction;

        match &event {        
            // Zoom
            event::DeviceEvent::MouseWheel { delta, .. } => {
                let scroll_amount = -1.0 * match delta {
                    // A mouse line is about 1 px.
                    event::MouseScrollDelta::LineDelta(_, scroll) => 
                        scroll * 1.0,
                    event::MouseScrollDelta::PixelDelta(
                        winit::dpi::PhysicalPosition { y: scroll, .. }
                    ) => {
                        *scroll as f32
                    }
                };
    
                camera.add_distance(scroll_amount * ZOOM_SPEED);
            }
    
            // Player pressed movement keys
            event::DeviceEvent::Key(event::KeyboardInput {
                state: event::ElementState::Pressed,
                virtual_keycode: Some(event::VirtualKeyCode::Left),
                ..
            } ) /* if (self.direction >> 3) & 1 != 1 */ => self.direction |= directions::LEFT,
    
            event::DeviceEvent::Key(event::KeyboardInput {
                state: event::ElementState::Pressed,
                virtual_keycode: Some(event::VirtualKeyCode::Right),
                ..
            } ) /* if (self.direction >> 2) & 1 != 1 */ => self.direction |= directions::RIGHT,
    
            event::DeviceEvent::Key(event::KeyboardInput {
                state: event::ElementState::Pressed,
                virtual_keycode: Some(event::VirtualKeyCode::Up),
                ..
            } ) /* if (self.direction >> 1) & 1 != 1 */ => self.direction |= directions::UP,
    
            event::DeviceEvent::Key(event::KeyboardInput {
                state: event::ElementState::Pressed,
                virtual_keycode: Some(event::VirtualKeyCode::Down),
                ..
            } ) /* if self.direction & 1 != 1 */ => self.direction |= directions::DOWN,
    
            // Player releases movement keys
            event::DeviceEvent::Key(event::KeyboardInput {
                state: event::ElementState::Released,
                virtual_keycode: Some(event::VirtualKeyCode::Left),
                ..
            } ) => self.direction &= !directions::LEFT,
    
            event::DeviceEvent::Key(event::KeyboardInput {
                state: event::ElementState::Released,
                virtual_keycode: Some(event::VirtualKeyCode::Right),
                ..
            } ) => self.direction &= !directions::RIGHT,
    
            event::DeviceEvent::Key(event::KeyboardInput {
                state: event::ElementState::Released,
                virtual_keycode: Some(event::VirtualKeyCode::Up),
                ..
            } ) => self.direction &= !directions::UP,
    
            event::DeviceEvent::Key(event::KeyboardInput {
                state: event::ElementState::Released,
                virtual_keycode: Some(event::VirtualKeyCode::Down),
                ..
            } ) => self.direction &= !directions::DOWN,
    
            _ => {  }
        }

        if temp != self.direction {
            self.previous_direction = temp;
        }        
    }

    pub fn aggregate_player_velocity(&mut self, velocity: &mut Vector3<f32>) {
        match self.direction & 3 {
            1 if self.previous_direction & 2 == 2 => velocity.z *= -1.0,
            2 if self.previous_direction & 2 == 1 => velocity.z *= -1.0,
            1 => velocity.z -= self.acceleration,
            2 => velocity.z += self.acceleration,
            _ => {  }
        }

        match (self.direction >> 2) & 3 {
            1 if (self.previous_direction >> 2) & 2 == 2 => velocity.x *= -1.0,
            2 if (self.previous_direction >> 2) & 2 == 1 => velocity.x *= -1.0,
            1 => velocity.x -= self.acceleration,
            2 => velocity.x += self.acceleration,
            _ => {  }
        }
        
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