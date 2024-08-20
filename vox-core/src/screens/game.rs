use bevy_ecs::{schedule::SystemConfigs, system::{Commands, Query, Res, ResMut}};
use cgmath::{EuclideanSpace, InnerSpace, Matrix4};
use glyphon::Resolution;
use wgpu::CommandEncoderDescriptor;

use crate::{bundles::{camera_bundle::CameraBundle, single_entity_bundle::SingleEntity}, components::{camerable::CamerableComponent, model::ModelComponent, position::PositionComponent, rotation::RotationComponent, single_instance::SingleInstanceComponent, speed::SpeedComponent}, resources::{asset_server::AssetServer, default_pipeline::DefaultPipeline, frame_context::FrameContext, gui_context::GuiContext, input::InputRes, mouse::MouseRes, render_context::RenderContext}, DrawObject, InstanceData, Model};

use super::screen::Screen;

#[derive(Default)]
pub struct GameScreen {}

impl Screen for GameScreen {
    fn start_systems(&self) -> Option<SystemConfigs> {
        self.to_systems((spawn_entities, spawn_camera))
    }

    fn ui_systems(&self) -> Option<SystemConfigs> {
        None
    }

    fn draw_systems(&self) -> Option<SystemConfigs> {
        self.to_systems((draw_single_instance_entities, draw_cameras))
    }

    fn update_systems(&self) -> Option<SystemConfigs> {
        self.to_systems((update_single_instance_entities, update_camera))
    }
}

pub fn update_single_instance_entities(mut query: Query<(
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

#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

pub fn spawn_entities(mut asset_server: ResMut<AssetServer>,
        mut commands: Commands,
        render_ctx: Res<RenderContext>,
) {
    let model = asset_server.get_or_load::<Model>("res/untitled.obj",
        &render_ctx.device,
        &render_ctx.queue
    ).unwrap();

    commands.spawn(SingleEntity::new(model));
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(CameraBundle::default());
}

pub fn draw_single_instance_entities(query: Query<(
        &ModelComponent,
        &SingleInstanceComponent)>,
        render_ctx: Res<RenderContext>,
        mut frame_ctx: ResMut<FrameContext>,
        pipeline: Res<DefaultPipeline>,
) {
    let view = &frame_ctx.view;
    let mut encoder = render_ctx.device.create_command_encoder(&CommandEncoderDescriptor {
        label: Some("Single Entity Encoder"),
    });

    for (model_cmpnt, instance_cmpnt) in &query {
        if instance_cmpnt.instance_buffer().is_none() {
            continue;
        }

        let mut render_pass = pipeline
            .model_pass(&mut encoder, view,
                render_ctx.depth_texture.view()
            );

        render_pass.draw_entity(model_cmpnt,
            instance_cmpnt,
            pipeline.camera_bind_group()
        );
    }

    frame_ctx.add_encoder(encoder);
}

pub fn draw_glyphon_labels(render_ctx: Res<RenderContext>,
    mut frame_ctx: ResMut<FrameContext>,
    mut gui_context: ResMut<GuiContext>,
    pipeline: Res<DefaultPipeline>
) {
    let view = &frame_ctx.view;
    let mut encoder = render_ctx.device.create_command_encoder(&CommandEncoderDescriptor {
        label: Some("Glyphon Label Encoder"),
    });

    gui_context.glyphon_renderer
        .viewport
        .update(&render_ctx.queue, Resolution {
            width: render_ctx.config.width,
            height: render_ctx.config.height,
        });

    gui_context.glyphon_renderer
        .prepare(&render_ctx.device, &render_ctx.queue);

    {
        let mut pass = pipeline
            .glyphon_pass(&mut encoder, view);

        gui_context.glyphon_renderer
            .draw(&mut pass);
    }

    frame_ctx.add_encoder(encoder);
}

pub fn draw_cameras(query: Query<(
    &PositionComponent,
    &CamerableComponent)>,
    render_ctx: Res<RenderContext>,
    pipeline: Res<DefaultPipeline>,
) {
    for (position_cmpnt, camerable_cmpnt) in &query {
        let view = Matrix4::look_at_rh(
            position_cmpnt.position,
            camerable_cmpnt.target,
            camerable_cmpnt.up
        );
        
        let proj = cgmath::perspective(
            cgmath::Deg(camerable_cmpnt.fovy),
            camerable_cmpnt.aspect,
            camerable_cmpnt.znear,
            camerable_cmpnt.zfar
        );
        
        let uniform: [[f32;4];4] = (OPENGL_TO_WGPU_MATRIX * proj * view)
            .into();
        
        render_ctx.queue.write_buffer(pipeline.camera_buffer(),
            0, bytemuck::cast_slice(&uniform));
    }
}