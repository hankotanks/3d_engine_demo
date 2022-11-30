mod util;

use util::{terrain, controller::process_events};

use block_engine_wgpu::{
    run,
    Config,
    tiles,
    camera::{Camera, CameraBuilder}, 
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
                        center: (0.0, 1.0, 0.0).into(),
                        color: [1.0; 3],
                        light: Some([1.0, 0.4, 0.1, 0.4]),
                    };
                    pl.set_light([1.0, 0.4, 0.1, 0.4]);
                    pl
                } ));
            }
        }
    };

    let process_events = {
        let mut init = true;
    
        move |
            event: &event::DeviceEvent, 
            camera: &mut Camera, 
            world: &mut tiles::World,
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
                process_events(event, camera, world, entities)
            } else {
                false
            }
        }
    };

    pollster::block_on(run(config, update, process_events)); 
}
