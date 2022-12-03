use cgmath::{
    Point3,
    Matrix4, 
    SquareMatrix, 
    EuclideanSpace, 
    Vector3
};

pub struct Camera {
    pub(crate) distance: f32,
    pub(crate) eye: Point3<f32>,
    pub(crate) target: Point3<f32>,
    pub(crate) pitch: f32,
    pub(crate) yaw: f32,
    pub(crate) aspect: f32,
    pub(crate) bounds: CameraBounds
}

impl Default for Camera {
    fn default() -> Self {
        let mut camera = Self {
            distance: 10.0,
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
    pub(crate) fn update(&mut self) {
        self.eye = Point3::new(
            self.distance * self.yaw.sin() * self.pitch.cos(),
            self.distance * self.pitch.sin(),
            self.distance * self.yaw.cos() * self.pitch.cos()
        );

        self.eye += self.target.to_vec();
    }

    pub fn set_distance(&mut self, distance: f32) {
        self.distance = distance.clamp(
            self.bounds.min_distance.unwrap_or(f32::EPSILON),
            self.bounds.max_distance.unwrap_or(f32::MAX),
        );
        self.update();
    }

    pub fn add_distance(&mut self, delta: f32) {
        self.set_distance(self.distance + delta);
    }

    pub fn set_pitch(&mut self, pitch: f32) {
        self.pitch = pitch.clamp(self.bounds.min_pitch, self.bounds.max_pitch);
        self.update();
    }

    pub fn add_pitch(&mut self, delta: f32) {
        self.set_pitch(self.pitch + delta);
    }

    pub fn set_yaw(&mut self, yaw: f32) {
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

    pub fn add_yaw(&mut self, delta: f32) {
        self.set_yaw(self.yaw + delta);
    }

    pub fn set_target(&mut self, target: Point3<f32>) {
        self.target = target;
        self.update();
    }

    pub fn displace_target(&mut self, displacement: Vector3<f32>) {
        self.set_target(self.target + displacement);
        self.update();
    }
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

#[derive(Default)]
pub struct CameraBuilder(Camera);

impl CameraBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn distance(mut self, distance: f32) -> Self {
        self.0.distance = distance;
        self
    }

    pub fn target(mut self, target: Point3<f32>) -> Self {
        self.0.target = target;
        self
    }

    pub fn pitch(mut self, pitch: f32) -> Self {
        self.0.pitch = pitch;
        self
    }

    pub fn yaw(mut self, yaw: f32) -> Self {
        self.0.yaw = yaw;
        self
    }

    pub fn aspect(mut self, aspect: f32) -> Self {
        self.0.aspect = aspect;
        self
    }

    pub fn build(mut self) -> Camera {
        self.0.update();
        self.0
    }
}
    
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
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