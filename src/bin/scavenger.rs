mod util;

use std::time::Duration;

use cgmath::{Vector3, Zero};
use util::{
    terrain, 
    controller, 
    tile, 
    entity
};

use block_engine_wgpu::{
    run,
    Config,
    camera, 
    world, 
    GameData, 
    GameEvent, GameWindow  
};

fn main() {
    let config = Config {
        fps: 60
    };

    let update = {
        let mut init = true;
        move |data: GameData| {
            if init {
                init = false;

                terrain::generate(data.world);

                data.world.add_tile( {
                    let mut pl = tile::Cube::new(
                        (1, 1, 1).into(), [1.0; 3]);
                    world::Drawable::set_light(&mut pl, [1.0; 4]);
                    pl
                } );

                data.world.add_entity_with_tag("player",
                    { 
                        let mut pl = entity::PlaceholderEntity {
                            center: (0.0, 6.0, 0.0).into(),
                            color: [1.0; 3],
                            light: Some([1.0, 0.4, 0.1, 0.4]),
                            velocity: (0.0, 0.0, 0.0).into(),
                            weight: 0.1,
                        };
                        world::Drawable::set_light(&mut pl, [1.0, 0.4, 0.1, 0.4]);
                        pl
                    },
                    None
                 );
            } else {
                let center = data.world.get_entity("player").unwrap().borrow().center();
                data.camera.set_target((center.x, center.y.round(), center.z).into());
            }
        }
    };

    let process_events = {
        let mut init = true;
        let mut pc = controller::PlayerController {
            direction: 0,
            speed: 0.2,
            acceleration: 0.1,
            pressed: false,
            cursor_drag_vector: Vector3::zero(),
        };
    
        move |
            window: GameWindow,
            event: GameEvent,
            data: GameData
        | -> bool {
            if init && data.world.contains_entity("player") { 
                init = false;
                *data.camera = camera::CameraBuilder::new()
                    .pitch(1.0)
                    .yaw(0.1)
                    .target([0.0; 3].into())
                    .build();
                
                true
            } else if data.world.contains_entity("player") {
                pc.process_events(window, event, data.camera);

                let mut handle = data.world.get_entity("player").unwrap();
                let mut entity = handle.borrow_mut();
                
                {
                    let mut velocity = entity.velocity();
                    pc.aggregate_player_velocity(&mut velocity);

                    if let Some(mut drag_vector) = pc.spawn_projectile() {
                        drag_vector *= -1.0;

                        let entity = entity::PlaceholderEntity {
                            center: entity.center(),
                            color: [1.0; 3],
                            light: Some([1.0, 1.0, 1.0, 0.2]),
                            velocity: drag_vector,
                            weight: 0.05,
                        };

                        data.world.add_entity(entity, Some(Duration::from_secs(4)));
                    }

                    entity.set_velocity(velocity);
                }                          
                
                true
            } else {
                false
            }
        }
    };

    pollster::block_on(run(config, update, process_events)); 
}
