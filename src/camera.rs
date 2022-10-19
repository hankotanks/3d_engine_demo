use cgmath::{
    Point3,
    Matrix4, 
    SquareMatrix
};

pub struct Camera {
    pub distance: f32,
    pub eye: Point3<f32>,
    pub target: Point3<f32>,
    pub pitch: f32,
    pub yaw: f32,
    pub aspect: f32,
    pub bounds: CameraBounds
}

impl Camera {
    const FOV: f32 = 45.0;
    const ZNEAR: f32 = 0.1;
    const ZFAR: f32 = 100.0;

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
    pub fn update(&mut self) {
        self.eye = Point3::new(
            self.distance * self.yaw.sin() * self.pitch.cos(),
            self.distance * self.pitch.sin(),
            self.distance * self.yaw.cos() * self.pitch.cos()
        );
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
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Debug, Clone, Copy)]
pub struct CameraBounds {
    pub min_distance: Option<f32>,
    pub max_distance: Option<f32>,
    pub min_pitch: f32,
    pub max_pitch: f32,
    pub min_yaw: Option<f32>,
    pub max_yaw: Option<f32>,
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
pub struct CameraUniform {
    projection: [[f32; 4]; 4]
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            projection: Matrix4::identity().into()
        }
    }

    pub fn update_projection(&mut self, camera: &Camera) {
        self.projection = camera.build_view_projection_matrix().into();
    }
}

