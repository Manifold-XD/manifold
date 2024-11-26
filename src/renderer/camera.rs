use cgmath::{perspective, Matrix4, Point3, Quaternion, Rad, Rotation3, Vector3};
use winit::{
    event::{ElementState, KeyEvent},
    keyboard::{KeyCode, PhysicalKey},
};

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

pub struct Camera {
    pub position: Point3<f32>,
    pub orientation: Quaternion<f32>,
    pub up: Vector3<f32>,
    pub aspect: f32,
    pub fov: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    fn build_view_projection_matrix(&self) -> Matrix4<f32> {
        let forward = self.orientation * -Vector3::unit_z();
        let view = Matrix4::look_at_rh(self.position, self.position + forward, self.up);
        let proj = perspective(cgmath::Deg(self.fov), self.aspect, self.near, self.far);

        OPENGL_TO_WGPU_MATRIX * proj * view
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}

pub struct CameraController {
    speed: f32,
    look_around: bool,
    direction_inputs: [bool; 4],
    mouse_delta: (f64, f64),
}

impl CameraController {
    pub fn new() -> Self {
        Self {
            speed: 1.0,
            look_around: false,
            direction_inputs: [false; 4],
            mouse_delta: (0.0, 0.0),
        }
    }

    pub fn process_input_events(&mut self, event: &KeyEvent) -> bool {
        let is_pressed = event.state == ElementState::Pressed;
        match event.physical_key {
            PhysicalKey::Code(KeyCode::ShiftLeft) => {
                self.look_around = is_pressed;
                true
            }
            PhysicalKey::Code(KeyCode::KeyW) => {
                self.direction_inputs[0] = is_pressed;
                true
            }
            PhysicalKey::Code(KeyCode::KeyS) => {
                self.direction_inputs[1] = is_pressed;
                true
            }
            PhysicalKey::Code(KeyCode::KeyA) => {
                self.direction_inputs[2] = is_pressed;
                true
            }
            PhysicalKey::Code(KeyCode::KeyD) => {
                self.direction_inputs[3] = is_pressed;
                true
            }
            _ => false,
        }
    }

    pub fn process_mouse_delta(&mut self, delta: (f64, f64)) {
        if !self.look_around {
            return;
        }
        self.mouse_delta.0 += delta.0;
        self.mouse_delta.1 += delta.1;
    }

    pub fn update_camera(&mut self, camera: &mut Camera, delta_time: f32) {
        use cgmath::InnerSpace;

        let mut local_move_direction: Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);

        if self.direction_inputs[0] {
            local_move_direction.z -= 1.0;
        }
        if self.direction_inputs[1] {
            local_move_direction.z += 1.0;
        }
        if self.direction_inputs[2] {
            local_move_direction.x -= 1.0;
        }
        if self.direction_inputs[3] {
            local_move_direction.x += 1.0;
        }

        if local_move_direction.magnitude2() > 0.0 {
            local_move_direction = local_move_direction.normalize();
            let movement = camera.orientation * local_move_direction * self.speed * delta_time;
            camera.position += movement;
        }

        let mouse_sensitivity = 1.0 / 1000.0;
        let delta_yaw = Rad((self.mouse_delta.0 as f32) * mouse_sensitivity);
        let delta_pitch = Rad((self.mouse_delta.1 as f32) * mouse_sensitivity);

        self.mouse_delta = (0.0, 0.0);

        let yaw_rotation = Quaternion::from_angle_y(-delta_yaw);
        let pitch_rotation = Quaternion::from_angle_x(-delta_pitch);

        camera.orientation = (yaw_rotation * camera.orientation) * pitch_rotation;
        camera.orientation = camera.orientation.normalize();
    }
}
