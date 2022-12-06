mod util;

use std::{time::Duration, sync::{Arc, Mutex}};

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

fn game_init(data: GameData) {
    terrain::generate(data.world);

    data.world.add_tile( {
        let mut pl = tile::Cube::new(
            (1, 1, 1).into(), [1.0; 3]);
        world::Drawable::set_light(&mut pl, [1.0; 4]);
        pl
    } );

    data.world.add_entity_with_tag(
        "player",
        entity::PlaceholderEntity {
            center: (0.0, 6.0, 0.0).into(),
            color: [1.0; 3],
            light: Some([1.0, 0.4, 0.1, 0.4]),
            velocity: (0.0, 0.0, 0.0).into(),
            collisions: (false, false, false).into(),
            weight: 0.2,
        },
        None
    );

    *data.camera = camera::CameraBuilder::new()
        .pitch(1.0)
        .yaw(0.1)
        .target([0.0; 3].into())
        .build();
} 

fn main() {
    let config = Config { fps: 60 };

    let controller = Arc::new(Mutex::new(controller::PlayerController {
        direction: 0,
        acceleration: 0.15,
        pressed: false,
        current_drag_vector: Vector3::zero(),
    } ));

    let game_update = {
        let controller_ref = Arc::clone(&controller);

        move |data: GameData| {
            let center = data.world.get_entity("player").unwrap().borrow().center();
            data.camera.set_target((center.x, center.y.round(), center.z).into());

            let mut handle = data.world.get_entity("player").unwrap();
            let mut entity = handle.borrow_mut();

            {
                let mut velocity = entity.velocity();
                controller_ref.lock().unwrap().aggregate_player_velocity(&mut velocity);
                entity.set_velocity(velocity);

                if let Some(mut drag_vector) = controller_ref.lock().unwrap().spawn_projectile() {
                    drag_vector *= -1.0;

                    let entity = entity::PlaceholderEntity {
                        center: entity.center(),
                        color: [1.0; 3],
                        light: Some([1.0, 1.0, 1.0, 0.2]),
                        velocity: drag_vector,
                        collisions: (false, false, false).into(),
                        weight: 0.05,
                    };

                    data.world.add_entity(entity, Some(Duration::from_secs(4)));
                }
            }
        }
    };

    let process_events = {
        let controller_ref = Arc::clone(&controller);
        move |window: GameWindow, event: GameEvent, data: GameData| {
            controller_ref.lock().unwrap().process_events(window, event, data.camera);
    
            false
        }
    };

    pollster::block_on(run(config, game_init, game_update, process_events)); 
}

/*

 */