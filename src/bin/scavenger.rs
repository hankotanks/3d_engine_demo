mod util;

use cgmath::{Zero, Vector3};
use util::{terrain, controller::{process_events, PlayerController}};

use block_engine_wgpu::{
    run,
    Config,
    tiles,
    camera::{Camera, CameraBuilder, self}, 
    entities, drawable::Drawable    
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
            world: &mut tiles::World, 
            entities: &mut Vec<Box<dyn entities::Entity>>
        | {
            if init {
                init = false;

                terrain::generate(world);

                world.add( {
                    let mut pl = tiles::Cube::new(
                        (1, 1, 1).into(), [1.0; 3]);
                    pl.set_light([1.0; 4]);
                    pl
                } );

                entities.push(Box::new( { 
                    let mut pl = entities::PlaceholderEntity {
                        center: (0.0, 3.0, 0.0).into(),
                        color: [1.0; 3],
                        light: Some([1.0, 0.4, 0.1, 0.4]),
                        velocity: (0.0, 0.0, 0.0).into(),
                        weight: 0.1,
                    };
                    pl.set_light([1.0, 0.4, 0.1, 0.4]);
                    pl
                } ));
            } else {
                let center = entities[0].center();
                camera.set_target((center.x, center.y.round(), center.z).into());
            }
        }
    };

    let process_events = {
        let mut init = true;
        let mut pc = PlayerController { 
            index: 0, 
            direction: 0,
            initial_speed: 0.1,
            maximum_speed: 0.2,
            acceleration: 0.05 
        };
    
        move |
            event: &event::DeviceEvent, 
            camera: &mut Camera, 
            _world: &mut tiles::World,
            entities: &mut Vec<Box<dyn entities::Entity>>
        | -> bool {
            if init && !entities.is_empty() { 
                init = false;
                *camera = CameraBuilder::new()
                    .pitch(1.0)
                    .yaw(0.1)
                    .target([0.0; 3].into())
                    .build();
                
                true
            } else if !entities.is_empty() {
                process_events(event, camera, entities, &mut pc);
                let mut velocity = entities[pc.index].velocity();
                if pc.direction >> 0 & 1 == 1 { velocity.z -= if velocity.z == 0.0 { pc.initial_speed } else { pc.acceleration } }
                if pc.direction >> 1 & 1 == 1 { velocity.z += if velocity.z == 0.0 { pc.initial_speed } else { pc.acceleration } }
                if pc.direction >> 2 & 1 == 1 { velocity.x -=  if velocity.x == 0.0 { pc.initial_speed } else { pc.acceleration } }
                if pc.direction >> 3 & 1 == 1 { velocity.x +=  if velocity.x == 0.0 { pc.initial_speed } else { pc.acceleration } }

                entities[pc.index].set_velocity(velocity);
                true
            } else {
                false
            }
        }
    };

    pollster::block_on(run(config, update, process_events)); 
}
