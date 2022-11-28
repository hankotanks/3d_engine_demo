use block_engine_wgpu::{
    run,
    Config,
    objects,
    camera::Camera    
};

use cgmath::Vector3;
use noise::utils::NoiseMapBuilder;
use rand::{RngCore, Rng};
use winit::event;

const ZOOM_SPEED: f32 = 0.6;
const ROTATE_SPEED: f32 = 0.025;

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

fn flood_fill_terrain(
    mesh: &mut Vec<Box<dyn objects::MeshObject>>, 
    pos: (usize, usize), 
    nm: &noise::utils::NoiseMap,
    processed: &mut std::collections::HashSet<(usize, usize)>
) {
    processed.insert(pos);

    if nm.get_value(pos.0, pos.1) < 0.0 && pos.0 > 0 && pos.0 < nm.size().0 && pos.1 > 0 && pos.1 < nm.size().1 {
        mesh.push(Box::new(objects::Cube::new(
            (pos.0 as i16, 0, pos.1 as i16).into(),
            [1.0; 3]
        )));

        if !processed.contains(&(pos.0 - 1, pos.1)) { flood_fill_terrain(mesh, (pos.0 - 1, pos.1), nm, processed); }
        if !processed.contains(&(pos.0 + 1, pos.1)) { flood_fill_terrain(mesh, (pos.0 + 1, pos.1), nm, processed); }
        if !processed.contains(&(pos.0, pos.1 - 1)) { flood_fill_terrain(mesh, (pos.0, pos.1 - 1), nm, processed); }
        if !processed.contains(&(pos.0, pos.1 + 1)) { flood_fill_terrain(mesh, (pos.0, pos.1 + 1), nm, processed); }
    } else {
        mesh.push(Box::new(objects::Cube::new(
            (pos.0 as i16, 1, pos.1 as i16).into(),
            [0.2; 3]
        )));
    }
}

fn build_terrain(mesh: &mut Vec<Box<dyn objects::MeshObject>>) -> (usize, usize) {
    let perlin = noise::Perlin::new(rand::thread_rng().next_u32());
    let nm = noise::utils::PlaneMapBuilder::<&noise::Perlin, 2>::new(&perlin)
        .set_size(WIDTH + 1, HEIGHT + 1)
        .set_x_bounds(-1.0, 1.0)
        .set_y_bounds(-1.0, 1.0)
        .set_is_seamless(true)
        .build();

    let mut prng = rand::thread_rng();

    let (mut pos_x, mut pos_y);
    'origin: loop {
        pos_x = prng.gen_range(0..WIDTH);
        pos_y = prng.gen_range(0..HEIGHT);
        if nm.get_value(pos_x, pos_y) < 0.0 { break 'origin; }
    }

    let mut processed = std::collections::HashSet::new();
    flood_fill_terrain(mesh, (pos_x, pos_y), &nm, &mut processed);

    (pos_x, pos_y)
}

fn main() {
    let config = Config {
        fps: 60
    };

    let update = {
        let mut init = true;
        move |mesh: &mut Vec<Box<dyn objects::MeshObject>>| {
            if init {
                init = false;

                let origin = build_terrain(mesh);

                let mut player = objects::Cube::new(
                    [origin.0 as i16, 1, origin.1 as i16].into(),
                    [1.0, 0.4, 0.2]
                );
                objects::MeshObject::set_light(&mut player, Some([1.0; 4]));

                mesh.insert(0, Box::new(player));
            }
        }
    };

    let process_events = {
        let mut init = true;
        let mut is_drag_rotate = false;

        let mut disp = None;
        move |event: &event::DeviceEvent, camera: &mut Camera, mesh: &mut Vec<Box<dyn objects::MeshObject>>| -> bool {
            if init && !mesh.is_empty() { 
                init = false;
                camera.set_target(mesh[0].position().cast::<f32>().unwrap());
                camera.set_pitch(1.0);
                camera.set_yaw(-0.3);
            }

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
                } ) => {
                    disp = Some(Vector3::new(-1, 0, 0));
                },

                event::DeviceEvent::Key(event::KeyboardInput {
                    state: event::ElementState::Pressed,
                    virtual_keycode: Some(event::VirtualKeyCode::Right),
                    ..
                } ) => {
                    disp = Some(Vector3::new(1, 0, 0));
                }

                event::DeviceEvent::Key(event::KeyboardInput {
                    state: event::ElementState::Pressed,
                    virtual_keycode: Some(event::VirtualKeyCode::Up),
                    ..
                } ) => {
                    disp = Some(Vector3::new(0, 0, -1));
                }

                event::DeviceEvent::Key(event::KeyboardInput {
                    state: event::ElementState::Pressed,
                    virtual_keycode: Some(event::VirtualKeyCode::Down),
                    ..
                } ) => {
                    disp = Some(Vector3::new(0, 0, 1));
                    
                }

                _ => (),
            }

            if let Some(displacement) = disp {
                let new_pos = mesh[0].position() + displacement;
                for object in mesh.iter() {
                    if object.position() == new_pos {
                        return false;
                    }
                }

                mesh[0].set_position(new_pos);

                camera.displace_target(displacement.cast::<f32>().unwrap());
                disp = None;
            }
            
            true
        }
    };

    pollster::block_on(run(config, update, process_events)); 
}
