use winit::{event::{ElementState, KeyEvent, WindowEvent}, keyboard::{KeyCode, PhysicalKey}};

pub struct CameraController {
    speed: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_right_pressed: bool,
    is_left_pressed: bool,
}

impl CameraController {
    fn new(speed: f32) -> Self {
        Self {
            speed,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_right_pressed: false,
            is_left_pressed: false,
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    state,
                    physical_key: PhysicalKey::Code(keycode), 
                    ..
                },
                ..
            } => {
                let is_pressed = state.is_pressed();

                match keycode {
                    KeyCode::KeyW => {
                        self.is_forward_pressed = is_pressed;
                        true
                    },

                    KeyCode::KeyA => {
                        self.is_left_pressed = is_pressed;
                        true
                    },

                    KeyCode::KeyS => {
                        self.is_right_pressed = is_pressed;
                        true
                    },

                    KeyCode::KeyD => {
                        self.is_backward_pressed = is_pressed;
                        true
                    },

                    _ => false
                }
            }

            _ => false
        }
    }

    fn update(&self, camera: &mut Camera) {
        let forward = camera.target - camera.eye;

        if(self.is_forward_pressed) {

        }
    }
}
