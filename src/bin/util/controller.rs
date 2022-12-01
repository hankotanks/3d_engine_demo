use block_engine_wgpu::camera;
use winit::event;

const ZOOM_SPEED: f32 = 0.6;

pub mod directions {
    pub const UP: u8 = 1 << 0;
    pub const DOWN: u8 = 1 << 1;
    pub const LEFT: u8 = 1 << 2;
    pub const RIGHT: u8 = 1 << 3;
}

pub struct PlayerController {
    pub index: usize,
    pub direction: u8,
    pub initial_speed: f32,
    pub maximum_speed: f32,
    pub acceleration: f32,
}

pub fn process_events(
    event: &event::DeviceEvent, 
    camera: &mut camera::Camera,
    player_controller: &mut PlayerController
) {
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

        // Panning
        event::DeviceEvent::Key(event::KeyboardInput {
            state: event::ElementState::Pressed,
            virtual_keycode: Some(event::VirtualKeyCode::Left),
            ..
        } ) => player_controller.direction |= directions::LEFT,

        event::DeviceEvent::Key(event::KeyboardInput {
            state: event::ElementState::Pressed,
            virtual_keycode: Some(event::VirtualKeyCode::Right),
            ..
        } ) => player_controller.direction |= directions::RIGHT,

        event::DeviceEvent::Key(event::KeyboardInput {
            state: event::ElementState::Pressed,
            virtual_keycode: Some(event::VirtualKeyCode::Up),
            ..
        } ) => player_controller.direction |= directions::UP,

        event::DeviceEvent::Key(event::KeyboardInput {
            state: event::ElementState::Pressed,
            virtual_keycode: Some(event::VirtualKeyCode::Down),
            ..
        } ) => player_controller.direction |= directions::DOWN,

         // Panning
         event::DeviceEvent::Key(event::KeyboardInput {
            state: event::ElementState::Released,
            virtual_keycode: Some(event::VirtualKeyCode::Left),
            ..
        } ) => player_controller.direction &= !directions::LEFT,

        event::DeviceEvent::Key(event::KeyboardInput {
            state: event::ElementState::Released,
            virtual_keycode: Some(event::VirtualKeyCode::Right),
            ..
        } ) => player_controller.direction &= !directions::RIGHT,

        event::DeviceEvent::Key(event::KeyboardInput {
            state: event::ElementState::Released,
            virtual_keycode: Some(event::VirtualKeyCode::Up),
            ..
        } ) => player_controller.direction &= !directions::UP,

        event::DeviceEvent::Key(event::KeyboardInput {
            state: event::ElementState::Released,
            virtual_keycode: Some(event::VirtualKeyCode::Down),
            ..
        } ) => player_controller.direction &= !directions::DOWN,

        _ => {  }
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