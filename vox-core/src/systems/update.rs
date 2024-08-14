use bevy_ecs::prelude::*;
use cgmath::{EuclideanSpace, InnerSpace};

use crate::{components::{camerable::CamerableComponent, position::PositionComponent, rotation::RotationComponent, single_instance::SingleInstanceComponent, speed::SpeedComponent}, resources::{input::InputRes, mouse::MouseRes, render_context::RenderContext}, InstanceData};

pub fn update_single_instance_models(mut query: Query<(
        &PositionComponent,
        &mut SingleInstanceComponent,
        &RotationComponent)>,
        ctx: Res<RenderContext>,
) {
    for (position_cmpnt, mut instance_cmpnt, rotation_cmpnt)
    in &mut query {
        let rotation = rotation_cmpnt
            .quaternion;

        let position = position_cmpnt.position
            .to_vec();

        instance_cmpnt.set_instance(&InstanceData {
            position,
            rotation
        }, &ctx.device);
    }
}

pub fn update_camera(mut query: Query<(
    &mut PositionComponent,
    &SpeedComponent,
    &mut CamerableComponent)>,
    input_res: Res<InputRes>,
    mouse_res: Res<MouseRes>,
) {
    for (mut position_cmpnt, speed_cmpnt, mut camerable_cmpnt) in &mut query {
        let forward = camerable_cmpnt.target - position_cmpnt.position;
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();

        if input_res.forward.is_pressed && forward_mag > speed_cmpnt.speed {
            position_cmpnt.position += forward_norm * speed_cmpnt.speed;
            //camera_transform.target += forward_norm * self.speed;
        }

        if input_res.backward.is_pressed {
            position_cmpnt.position -= forward_norm * speed_cmpnt.speed;
            //camera_transform.target -= forward_norm * self.speed;
        }

        let up_norm = camerable_cmpnt.up.normalize();
        let right_norm = forward_norm.cross(up_norm);

        if input_res.right.is_pressed {
            position_cmpnt.position += right_norm * speed_cmpnt.speed; 
            //camera_transform.target += right_norm * self.speed;
        }

        if input_res.left.is_pressed {
            position_cmpnt.position -= right_norm * speed_cmpnt.speed;
            //camera_transform.target -= right_norm * self.speed;
        }

        let yaw: f32 = (mouse_res.pos.0 * 0.01) as f32;
        camerable_cmpnt.target.x = 2.23 * yaw.cos();
        camerable_cmpnt.target.z = 2.23 * yaw.sin();
    }
}
