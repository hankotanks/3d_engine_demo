use winit::{
    event,
    window::Window
};

use super::Camera;

pub(crate) struct CameraController {
    pub(crate) zoom_speed: f32,
    pub(crate) rotate_speed: f32,
    is_drag_rotate: bool
}

impl Default for CameraController {
    fn default() -> Self {
        Self::new(0.6, 0.025)
    }
}

impl CameraController {
    pub(crate) fn new(zoom_speed: f32, rotate_speed: f32) -> Self {
        Self {
            zoom_speed,
            rotate_speed,
            is_drag_rotate: false
        }
    }

    pub(crate) fn process_events(
        &mut self,
        event: &event::DeviceEvent,
        window: &Window,
        camera: &mut Camera,
    ) {
        match event {
            // Handle the start and end of mouse drags
            event::DeviceEvent::Button {
                #[cfg(target_os = "macos")]
                    button: 0,
                #[cfg(not(target_os = "macos"))]
                    button: 1,
                state,
            } => {
                let is_pressed = *state == event::ElementState::Pressed;
                self.is_drag_rotate = is_pressed;
            }
            
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
                camera.add_distance(scroll_amount * self.zoom_speed);
                window.request_redraw();
            }

            // Rotation
            event::DeviceEvent::MouseMotion { delta } => {
                if self.is_drag_rotate {
                    camera.add_yaw(-1.0 * delta.0 as f32 * self.rotate_speed);
                    camera.add_pitch(delta.1 as f32 * self.rotate_speed);
                    window.request_redraw();
                }
            }
            _ => (),
        }
    }
}
