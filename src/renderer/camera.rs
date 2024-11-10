use cgmath::{Matrix4, Point3, Quaternion, Vector2, Vector3, Deg};
use winit::{
    event::{ElementState, KeyEvent},
    keyboard::Key,
};

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, -1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

fn perspective_lh(fovy: Deg<f32>, aspect: f32, near: f32, far: f32) -> Matrix4<f32> {
    let f = 1.0 / (fovy.0.to_radians() / 2.0).tan();
    Matrix4::new(
        f / aspect, 0.0, 0.0, 0.0,
        0.0, f, 0.0, 0.0,
        0.0, 0.0, far / (far - near), 1.0,
        0.0, 0.0, -near * far / (far - near), 0.0,
    )
}

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
        let forward = self.orientation * Vector3::unit_z();
        let view = Matrix4::look_at_lh(self.position, self.position + forward, self.up);
        let proj = perspective_lh(cgmath::Deg(self.fov), self.aspect, self.near, self.far);

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
    direction_inputs: [bool; 4],
}

impl CameraController {
    pub fn new() -> Self {
        Self {
            speed: 1.0,
            direction_inputs: [false; 4],
        }
    }

    pub fn process_input_events(&mut self, event: &KeyEvent) -> bool {
        let is_pressed = event.state == ElementState::Pressed;
        match event.logical_key.as_ref() {
            Key::Character("w") => {
                self.direction_inputs[0] = is_pressed;
                true
            }
            Key::Character("s") => {
                self.direction_inputs[1] = is_pressed;
                true
            }
            Key::Character("a") => {
                self.direction_inputs[2] = is_pressed;
                true
            }
            Key::Character("d") => {
                self.direction_inputs[3] = is_pressed;
                true
            }
            _ => false,
        }
    }

    pub fn update_camera(&mut self, camera: &mut Camera, delta_time: f32) {
        use cgmath::InnerSpace;

        let mut move_direction: Vector2<f32> = Vector2::new(0.0, 0.0);

        if self.direction_inputs[0] {
            move_direction.y += 1.0;
        }
        if self.direction_inputs[1] {
            move_direction.y -= 1.0;
        }
        if self.direction_inputs[2] {
            move_direction.x -= 1.0;
        }
        if self.direction_inputs[3] {
            move_direction.x += 1.0;
        }

        if move_direction.magnitude2() > 1.0 {
            move_direction = move_direction.normalize();
        }

        let forward = camera.orientation * Vector3::unit_z();
        let right = camera.orientation * Vector3::unit_x();

        camera.position += forward * move_direction.y * self.speed * delta_time;
        camera.position += right * move_direction.x * self.speed * delta_time;
    }
}
