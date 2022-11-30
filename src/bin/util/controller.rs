use block_engine_wgpu::{camera::Camera, tiles::{self, World}, entities};
use cgmath::{Vector3, Point3};
use winit::event;

const ZOOM_SPEED: f32 = 0.6;

pub fn process_events(
    event: &event::DeviceEvent, 
    camera: &mut Camera, 
    world: &mut tiles::World,
    entities: &mut Vec<Box<dyn entities::Entity>>,
) -> bool {
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

            true
        }

        // Panning
        event::DeviceEvent::Key(event::KeyboardInput {
            state: event::ElementState::Pressed,
            virtual_keycode: Some(event::VirtualKeyCode::Left),
            ..
        } ) => move_player(camera, world, entities, (-0.2, 0.0, 0.0).into()),

        event::DeviceEvent::Key(event::KeyboardInput {
            state: event::ElementState::Pressed,
            virtual_keycode: Some(event::VirtualKeyCode::Right),
            ..
        } ) => move_player(camera, world, entities, (0.2, 0.0, 0.0).into()),

        event::DeviceEvent::Key(event::KeyboardInput {
            state: event::ElementState::Pressed,
            virtual_keycode: Some(event::VirtualKeyCode::Up),
            ..
        } ) => move_player(camera, world, entities, (0.0, 0.0, -0.2).into()),

        event::DeviceEvent::Key(event::KeyboardInput {
            state: event::ElementState::Pressed,
            virtual_keycode: Some(event::VirtualKeyCode::Down),
            ..
        } ) => move_player(camera, world, entities, (0.0, 0.0, 0.2).into()),

        _ => false
    }
}

fn move_player(
    camera: &mut Camera, 
    world: &mut tiles::World,
    entities: &mut Vec<Box<dyn entities::Entity>>,
    mut displacement: Vector3<f32>
) -> bool {
    let pos = entities[0].center();

    move_to(world, pos, &mut displacement);

    entities[0].set_center(pos + displacement);
    camera.displace_target(displacement.cast::<f32>().unwrap());

    true
}

// TODO - Returns the displacement that can be applied without colliding with Tiles
fn move_to(
    world: &mut World,
    pos: Point3<f32>,
    displacement: &mut Vector3<f32>
) {
    let increment: Vector3<f32> = [
        displacement.x * 0.1,
        displacement.y * 0.1,
        displacement.z * 0.1
    ].into();

    while world.occupied(pos + *displacement) {
        *displacement -= increment;
    }
    //dbg!("after", *displacement);
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