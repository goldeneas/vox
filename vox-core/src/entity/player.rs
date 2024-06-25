use bevy_ecs::prelude::*;
use winit::event::{KeyEvent, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

use crate::components::PositionComponent;
use crate::entity::vox_entity::VoxEntity;

pub struct PlayerEntity {
    pub id: Entity,
    pub is_left_pressed: bool,
    pub is_right_pressed: bool,
    pub is_forward_pressed: bool,
    pub is_backward_pressed: bool,
}

impl VoxEntity for PlayerEntity {
    fn new(world: &mut World) -> Self {
        let id = world.spawn((
                PositionComponent { x: 0.0, y: 0.0, z: 0.0 },
            )).id();

        Self {
            id,
            is_left_pressed: false,
            is_right_pressed: false,
            is_forward_pressed: false,
            is_backward_pressed: false,
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
                    },

                    KeyCode::KeyA => {
                        self.is_left_pressed = is_pressed;
                    },

                    KeyCode::KeyS => {
                        self.is_backward_pressed = is_pressed;
                    },

                    KeyCode::KeyD => {
                        self.is_right_pressed = is_pressed;
                    },

                    _ => {},
                }
            }

            _ => {}
        }

        false
    }

    fn update(&mut self) {
        if self.is_forward_pressed {
            self.y
        }
    }
}
