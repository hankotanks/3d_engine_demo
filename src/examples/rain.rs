use block_engine_wgpu::{
    Config, 
    camera::CameraConfig,
    mesh::Mesh,
    mesh::objects, 
};

use cgmath::Point3;

use rand::Rng;

const DIMENSIONS: usize = 11;
const HEIGHT: usize = 50;

pub const RAIN_CONFIG: Config = Config {
    fps: 30,
    camera_config: CameraConfig {
        target: Some(Point3::new(0, (HEIGHT as isize) / 2, 0)),
        distance: Some(DIMENSIONS as f32 + HEIGHT as f32),
        pitch: Some(-0.85),
        yaw: Some(0.85),
        aspect: None,
        zoom_speed: None,
        rotate_speed: None,
        locked: false,
    },
};

pub fn rain_mesh_init(mesh: &mut Mesh) {
    let light = objects::Cube::new(
        [0, -1, 0].into(),
        [1.0, 1.0, 1.0]
    );

    mesh.add(light);
    mesh[0].set_emitter(Some([1.0, 1.0, 1.0, (HEIGHT / 10) as f32].into()));

    for x in (DIMENSIONS as isize / -2)..(DIMENSIONS as isize / 2) {
        for z in (DIMENSIONS as isize / -2)..(DIMENSIONS as isize / 2) {
            let y = HEIGHT as isize;

            let color_intensity = rand::thread_rng().gen::<f32>();
            mesh.add(objects::Cube::new(
                [x, y, z].into(),
                [0.0, color_intensity * 0.5, color_intensity]
            ));
        }
    }
}

pub fn rain_mesh_update(mesh: &mut Mesh) {
    for object in mesh.iter_mut().skip(1) {
        let mut position = object.position();
        
        let mut color = object.color();
        color[1] = (color[1] - 0.5 / (HEIGHT as f32)).clamp(0.0, 1.0);
        color[2] = (color[2] - 1.0 / (HEIGHT as f32)).clamp(0.0, 1.0);
        object.set_color(color);

        if color[2] == 0.0 {
            let color_intensity = rand::thread_rng().gen::<f32>();
            object.set_color([ 0.0, color_intensity * 0.5, color_intensity]);
            object.set_position(Point3::new(position.x, HEIGHT as isize, position.z));
        } else {
            position.y -= 1;
            object.set_position(position);
        }
    }
}