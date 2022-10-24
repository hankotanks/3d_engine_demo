mod controller;
pub(crate) use controller::CameraController;

use cgmath::{
    Point3,
    Matrix4, 
    SquareMatrix, Vector3
};

#[derive(Clone, Copy)]
pub struct CameraConfig {
    pub target: Option<Point3<isize>>,
    pub distance: Option<f32>,
    pub pitch: Option<f32>,
    pub yaw: Option<f32>,
    pub aspect: Option<f32>,
    pub zoom_speed: Option<f32>,
    pub rotate_speed: Option<f32>,
    pub locked: bool
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self { 
            target: Some([0; 3].into()), 
            distance: Some(2.0), 
            pitch: Some(1.5), 
            yaw: Some(1.25),
            aspect: Some(1.0), 
            zoom_speed: Some(0.6), 
            rotate_speed: Some(0.025), 
            locked: false
        }
    }
}

pub(crate) struct Camera {
    pub(crate) distance: f32,
    pub(crate) eye: Point3<f32>,
    pub(crate) target: Point3<f32>,
    pub(crate) pitch: f32,
    pub(crate) yaw: f32,
    pub(crate) aspect: f32,
    pub(crate) bounds: CameraBounds
}

impl Camera {
    const FOV: f32 = 45.0;
    const ZNEAR: f32 = 0.1;
    const ZFAR: f32 = 1000.0;

    const MATRIX_CORRECTION_FOR_WGPU: Matrix4<f32> = Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 0.5, 0.0,
        0.0, 0.0, 0.5, 1.0,
    );

    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = Matrix4::look_at_rh(
            self.eye, 
            self.target, 
            cgmath::Vector3::unit_y()
        );

        let projection = cgmath::perspective(
            cgmath::Deg(Self::FOV),
            self.aspect,
            Self::ZNEAR,
            Self::ZFAR
        );

        Self::MATRIX_CORRECTION_FOR_WGPU * projection * view
    }
}

impl Default for Camera {
    fn default() -> Self {
        let mut camera = Self {
            distance: 2.0,
            eye: [0.0, 0.0, 0.0].into(),
            target: [0.0, 0.0, 0.0].into(),
            pitch: 1.5,
            yaw: 1.25,
            aspect: 1.0,
            bounds: CameraBounds::default()
        };

        camera.update();
        camera
    }
}

impl Camera {
    pub(crate) fn new(camera_config: CameraConfig) -> Self {
        let default_camera_config = CameraConfig::default();

        let mut camera = Self {
            distance: {
                if let Some(distance) = camera_config.distance {
                    distance
                } else {
                    default_camera_config.distance.unwrap()
                }
            },
            eye: [0.0; 3].into(),
            target: {
                let mut target = default_camera_config.target.unwrap();
                if let Some(real_target) = camera_config.target {
                    target = real_target;
                }

                [target.x as f32, target.y as f32, target.z as f32].into()
            },
            pitch: {
                if let Some(pitch) = camera_config.pitch {
                    pitch
                } else {
                    default_camera_config.pitch.unwrap()
                }
            },
            yaw: {
                if let Some(yaw) = camera_config.yaw {
                    yaw
                } else {
                    default_camera_config.yaw.unwrap()
                }
            },
            aspect: {
                if let Some(aspect) = camera_config.aspect {
                    aspect
                } else {
                    default_camera_config.aspect.unwrap()
                }
            },
            bounds: CameraBounds::default(),
        };

        camera.update();
        camera
    }

    pub(crate) fn update(&mut self) {
        self.eye = Point3::new(
            self.distance * self.yaw.sin() * self.pitch.cos(),
            self.distance * self.pitch.sin(),
            self.distance * self.yaw.cos() * self.pitch.cos()
        );

        self.eye += Vector3::new(self.target.x, self.target.y, self.target.z);
    }

    pub(crate) fn set_distance(&mut self, distance: f32) {
        self.distance = distance.clamp(
            self.bounds.min_distance.unwrap_or(f32::EPSILON),
            self.bounds.max_distance.unwrap_or(f32::MAX),
        );
        self.update();
    }

    pub(crate) fn add_distance(&mut self, delta: f32) {
        self.set_distance(self.distance + delta);
    }

    pub(crate) fn set_pitch(&mut self, pitch: f32) {
        self.pitch = pitch.clamp(self.bounds.min_pitch, self.bounds.max_pitch);
        self.update();
    }

    pub(crate) fn add_pitch(&mut self, delta: f32) {
        self.set_pitch(self.pitch + delta);
    }

    pub(crate) fn set_yaw(&mut self, yaw: f32) {
        let mut bounded_yaw = yaw;
        if let Some(min_yaw) = self.bounds.min_yaw {
            bounded_yaw = bounded_yaw.clamp(min_yaw, f32::MAX);
        }
        if let Some(max_yaw) = self.bounds.max_yaw {
            bounded_yaw = bounded_yaw.clamp(f32::MIN, max_yaw);
        }
        self.yaw = bounded_yaw;
        self.update();
    }

    pub(crate) fn add_yaw(&mut self, delta: f32) {
        self.set_yaw(self.yaw + delta);
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct CameraBounds {
    pub(crate) min_distance: Option<f32>,
    pub(crate) max_distance: Option<f32>,
    pub(crate) min_pitch: f32,
    pub(crate) max_pitch: f32,
    pub(crate) min_yaw: Option<f32>,
    pub(crate) max_yaw: Option<f32>,
}

impl Default for CameraBounds {
    fn default() -> Self {
        Self {
            min_distance: None,
            max_distance: None,
            min_pitch: -std::f32::consts::PI / 2.0 + f32::EPSILON,
            max_pitch: std::f32::consts::PI / 2.0 - f32::EPSILON,
            min_yaw: None,
            max_yaw: None,
        }
    }
}
    

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct CameraUniform {
    pub(crate) position: [f32; 4],
    pub(crate) projection: [[f32; 4]; 4]
}

impl CameraUniform {
    pub(crate) fn new() -> Self {
        Self {
            position: [0.0; 4],
            projection: Matrix4::identity().into()
        }
    }

    pub(crate) fn update_projection(&mut self, camera: &Camera) {
        self.position = [camera.eye.x, camera.eye.y, camera.eye.z, 1.0];
        self.projection = camera.build_view_projection_matrix().into();
    }
}

