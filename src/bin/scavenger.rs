mod util;
use std::{borrow::Borrow, ops::Deref};

use util::{
    terrain, 
    controller, 
    tile, 
    entity
};

use block_engine_wgpu::{
    run,
    Config,
    camera, world  
};

use winit::event;

fn main() {
    let config = Config {
        fps: 60
    };

    let mut player: std::rc::Rc<std::cell::RefCell<Option<std::rc::Rc<std::cell::RefCell<dyn world::Entity>>>>> = std::rc::Rc::new(std::cell::RefCell::new(None));

    let update = {
        let mut init = true;
        let player_ref = player.clone();
        move |
            camera: &mut camera::Camera,
            world: &mut world::World,
        | {
            if init {
                init = false;

                terrain::generate(world);

                world.add_tile( {
                    let mut pl = tile::Cube::new(
                        (1, 1, 1).into(), [1.0; 3]);
                    world::Drawable::set_light(&mut pl, [1.0; 4]);
                    pl
                } );

                let mut pl = player_ref.deref().borrow_mut();
                *pl = Some(world.add_entity( { 
                    let mut pl = entity::PlaceholderEntity {
                        center: (0.0, 3.0, 0.0).into(),
                        color: [1.0; 3],
                        light: Some([1.0, 0.4, 0.1, 0.4]),
                        velocity: (0.0, 0.0, 0.0).into(),
                        weight: 0.1,
                    };
                    world::Drawable::set_light(&mut pl, [1.0, 0.4, 0.1, 0.4]);
                    pl
                } ));
            } else if let Some(pl) = player_ref.deref().borrow().as_ref() {
                let center = pl.deref().borrow().center();
                camera.set_target((center.x, center.y.round(), center.z).into());
            }
        }
    };

    let process_events = {
        let mut init = true;
        let mut pc = controller::PlayerController { 
            index: 0, 
            direction: 0,
            initial_speed: 0.1,
            maximum_speed: 0.2,
            acceleration: 0.05 
        };

        let player_ref = std::rc::Rc::clone(&player);
    
        move |
            event: &event::DeviceEvent, 
            camera: &mut camera::Camera, 
            world: &mut world::World,
        | -> bool {
            if init && player_ref.deref().borrow().is_some() { 
                init = false;
                *camera = camera::CameraBuilder::new()
                    .pitch(1.0)
                    .yaw(0.1)
                    .target([0.0; 3].into())
                    .build();
                
                true
            } else if let Some(pl) = player.deref().borrow().as_ref() {
                controller::process_events(event, camera, &mut pc);

                let mut entity = pl.deref().borrow_mut();
                
                let mut velocity = entity.velocity();
                if pc.direction >> 0 & 1 == 1 { velocity.z -= if velocity.z == 0.0 { pc.initial_speed } else { pc.acceleration } }
                if pc.direction >> 1 & 1 == 1 { velocity.z += if velocity.z == 0.0 { pc.initial_speed } else { pc.acceleration } }
                if pc.direction >> 2 & 1 == 1 { velocity.x -=  if velocity.x == 0.0 { pc.initial_speed } else { pc.acceleration } }
                if pc.direction >> 3 & 1 == 1 { velocity.x +=  if velocity.x == 0.0 { pc.initial_speed } else { pc.acceleration } }

                entity.set_velocity(velocity);
                true
            } else {
                false
            }
        }
    };

    pollster::block_on(run(config, update, process_events)); 
}
