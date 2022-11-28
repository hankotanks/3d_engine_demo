use block_engine_wgpu::{
    run,
    Config,
    objects, 
    camera::Camera    
};

use winit::event;

const ZOOM_SPEED: f32 = 0.6;
const ROTATE_SPEED: f32 = 0.025;

fn main() {
    let config = Config {
        fps: 60
    };

    let update = |mesh: &mut Vec<Box<dyn objects::MeshObject>>| {
        if mesh.is_empty() {
            mesh.push(Box::new(objects::Cube::new(
                [0; 3].into(),
                [1.0; 3]
            )));

            let mut light = objects::Gap::new([0, 2, 0].into());
            objects::MeshObject::set_light(&mut light, Some([1.0; 4]));

            mesh.push(Box::new(light));
        }
    };

    let process_events = {
        let mut is_drag_rotate = false;

        move |
            event: &event::DeviceEvent, 
            _mesh: &mut Vec<Box<dyn objects::MeshObject>>, 
            camera: &mut Camera
        | -> bool {
            match &event {
                // Handle the start and end of mouse drags
                event::DeviceEvent::Button {
                    #[cfg(target_os = "macos")]
                        button: 0,
                    #[cfg(not(target_os = "macos"))]
                        button: 1,
                    state,
                } => {
                    let is_pressed = *state == event::ElementState::Pressed;
                    is_drag_rotate = is_pressed;
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

                    camera.add_distance(scroll_amount * ZOOM_SPEED);
                    
                    return true;
                }

                // Rotation
                event::DeviceEvent::MouseMotion { delta } => {
                    if is_drag_rotate {
                        camera.add_yaw(-1.0 * delta.0 as f32 * ROTATE_SPEED);
                        camera.add_pitch(delta.1 as f32 * ROTATE_SPEED);
                        
                        return true;
                    }
                }

                // Panning
                event::DeviceEvent::Key(event::KeyboardInput {
                    state: event::ElementState::Pressed,
                    virtual_keycode: Some(event::VirtualKeyCode::Left),
                    ..
                } ) => camera.displace_target([-1.0, 0.0, 0.0].into()),

                event::DeviceEvent::Key(event::KeyboardInput {
                    state: event::ElementState::Pressed,
                    virtual_keycode: Some(event::VirtualKeyCode::Right),
                    ..
                } ) => camera.displace_target([1.0, 0.0, 0.0].into()),

                event::DeviceEvent::Key(event::KeyboardInput {
                    state: event::ElementState::Pressed,
                    virtual_keycode: Some(event::VirtualKeyCode::Up),
                    ..
                } ) => camera.displace_target([0.0, 0.0, -1.0].into()),

                event::DeviceEvent::Key(event::KeyboardInput {
                    state: event::ElementState::Pressed,
                    virtual_keycode: Some(event::VirtualKeyCode::Down),
                    ..
                } ) => camera.displace_target([0.0, 0.0, 1.0].into()),

                _ => (),
            }
            
            true
        }
    };

    pollster::block_on(run(config, update, process_events)); 
}
