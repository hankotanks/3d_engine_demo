mod util;
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

    let update = {
        let mut init = true;
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

                world.add_entity( { 
                    let mut pl = entity::PlaceholderEntity {
                        center: (0.0, 3.0, 0.0).into(),
                        color: [1.0; 3],
                        light: Some([1.0, 0.4, 0.1, 0.4]),
                        velocity: (0.0, 0.0, 0.0).into(),
                        weight: 0.1,
                    };
                    world::Drawable::set_light(&mut pl, [1.0, 0.4, 0.1, 0.4]);
                    pl
                } );
            } else {
                let center = world.get_entity(0).center();
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
    
        move |
            event: &event::DeviceEvent, 
            camera: &mut camera::Camera, 
            world: &mut world::World,
        | -> bool {
            if init && !world.is_empty() { 
                init = false;
                *camera = camera::CameraBuilder::new()
                    .pitch(1.0)
                    .yaw(0.1)
                    .target([0.0; 3].into())
                    .build();
                
                true
            } else if !world.is_empty() {
                controller::process_events(event, camera, &mut pc);
                let entity = world.get_entity(pc.index);
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
