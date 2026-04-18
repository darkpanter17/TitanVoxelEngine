use winit::event::*;
use winit::keyboard::{KeyCode, PhysicalKey};
use glam::{Vec3, Mat4};
#[allow(unused_imports)]
use glam::Quat;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Mat4 = Mat4::from_cols_array(&[
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
]);

pub struct Camera {
    pub eye: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
    pub yaw: f32,
    pub pitch: f32,
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> Mat4 {
        let view = Mat4::look_at_rh(self.eye, self.eye + self.target, self.up);
        let proj = Mat4::perspective_rh(self.fovy, self.aspect, self.znear, self.zfar);
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
        use glam::Mat4;
        Self {
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().to_cols_array_2d();
    }
}

pub struct CameraController {
    speed: f32,
    sensitivity: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
    is_up_pressed: bool,
    is_down_pressed: bool,
}

impl CameraController {
    pub fn new(speed: f32, sensitivity: f32) -> Self {
        Self {
            speed, sensitivity,
            is_forward_pressed: false, is_backward_pressed: false,
            is_left_pressed: false, is_right_pressed: false,
            is_up_pressed: false, is_down_pressed: false,
        }
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                event: KeyEvent { state, physical_key: PhysicalKey::Code(keycode), .. },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    KeyCode::KeyW => { self.is_forward_pressed = is_pressed; true }
                    KeyCode::KeyA => { self.is_left_pressed = is_pressed; true }
                    KeyCode::KeyS => { self.is_backward_pressed = is_pressed; true }
                    KeyCode::KeyD => { self.is_right_pressed = is_pressed; true }
                    KeyCode::Space => { self.is_up_pressed = is_pressed; true }
                    KeyCode::ShiftLeft => { self.is_down_pressed = is_pressed; true }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    pub fn process_mouse(&mut self, mouse_dx: f64, mouse_dy: f64, camera: &mut Camera) {
        camera.yaw += (mouse_dx as f32) * self.sensitivity;
        camera.pitch -= (mouse_dy as f32) * self.sensitivity;
        
        // Limitar ángulo vertical para no romper el cuello
        camera.pitch = camera.pitch.clamp(-1.5, 1.5);
        
        // Recalcular vector target
        let (sin_pitch, cos_pitch) = camera.pitch.sin_cos();
        let (sin_yaw, cos_yaw) = camera.yaw.sin_cos();
        
        camera.target = Vec3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize();
    }

    pub fn update_camera(&self, camera: &mut Camera) {
        let forward = camera.target;
        let right = forward.cross(camera.up).normalize();
        
        if self.is_forward_pressed { camera.eye += forward * self.speed; }
        if self.is_backward_pressed { camera.eye -= forward * self.speed; }
        if self.is_right_pressed { camera.eye += right * self.speed; }
        if self.is_left_pressed { camera.eye -= right * self.speed; }
        if self.is_up_pressed { camera.eye += camera.up * self.speed; }
        if self.is_down_pressed { camera.eye -= camera.up * self.speed; }
    }
}