use std::time::Instant;

use bevy_ecs::{schedule::SystemConfigs, system::{Commands, Query, Res, ResMut}, world::World};
use cgmath::{EuclideanSpace, InnerSpace, Matrix4};
use wgpu::CommandEncoderDescriptor;

use crate::{bundles::{camera_bundle::CameraBundle, game_object::GameObject}, components::{camerable::{CameraUniform, CamerableComponent}, model::ModelComponent, position::PositionComponent, rotation::RotationComponent, single_instance::SingleInstanceComponent, speed::SpeedComponent}, render::face::{FaceDirection, FaceModel}, resources::{asset_server::AssetServer, default_pipeline::DefaultPipeline, frame_context::FrameContext, game_state::GameState, input::InputRes, mouse::MouseRes, render_context::RenderContext}, ui::glyphon_renderer::{LabelDescriptor, LabelId}, voxels::{chunk::Chunk, voxel::{VoxelType, VoxelTypeIdentifier}}, world_ext::WorldExt, DrawObject, InstanceData, IntoModel};

use super::screen::Screen;

#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

#[derive(Default)]
pub struct GameScreen {
    label_id: Option<LabelId>,
    elapsed: Option<Instant>,
    frame_counter: u16,
}

impl Screen for GameScreen {
    fn start(&mut self, world: &mut World) {
        let mut glyphon_renderer = world.glyphon_renderer_mut();

        self.label_id = Some(glyphon_renderer.add_label(LabelDescriptor::default()));
        self.elapsed = Some(Instant::now());
    }

    fn update(&mut self, world: &mut World) {
        self.frame_counter += 1;

        let mut glyphon_renderer = world.glyphon_renderer_mut();

        if self.frame_counter >= 100 {
            let string = format!("UPDATE DT: {:?}", self.elapsed.unwrap().elapsed());
            glyphon_renderer.set_text(self.label_id.unwrap(), string);
            self.frame_counter = 0;
        }
        self.elapsed = Some(Instant::now());
    }

    fn start_systems(&self) -> Option<SystemConfigs> {
        self.to_systems((spawn_game_objects, spawn_camera, spawn_chunks))
    }

    fn draw_systems(&self) -> Option<SystemConfigs> {
        self.to_systems((draw_game_objects, draw_cameras))
    }

    fn update_systems(&self) -> Option<SystemConfigs> {
        self.to_systems((update_game_objects, update_camera))
    }

    fn game_state(&self) -> GameState {
        GameState::Game
    }
}

pub fn update_game_objects(mut query: Query<(
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

pub fn spawn_chunks(mut asset_server: ResMut<AssetServer>,
    commands: Commands,
    render_ctx: Res<RenderContext>
) {
    let mut chunk = Chunk::new();
    chunk.set_voxel_type_at((1, 0, 0), 1);
    chunk.generate_mesh();

    chunk.faces(&mut asset_server,
        commands,
        &render_ctx.device,
        &render_ctx.queue
    );
}

pub fn spawn_game_objects(mut asset_server: ResMut<AssetServer>,
        mut commands: Commands,
        render_ctx: Res<RenderContext>,
) {
    //let face = FaceModel::new(&mut asset_server,
    //    &render_ctx.device,
    //    &render_ctx.queue,
    //    FaceDirection::DOWN
    //);

    //let e = GameObject::debug(face, &render_ctx.device);
    //commands.spawn(e);
}

pub fn spawn_camera(mut commands: Commands,
    render_ctx: Res<RenderContext>,
) {
    commands.spawn(CameraBundle::debug(&render_ctx.config));
}

pub fn draw_game_objects(query: Query<(
        &ModelComponent,
        &SingleInstanceComponent)>,
        render_ctx: Res<RenderContext>,
        mut frame_ctx: ResMut<FrameContext>,
        pipeline: Res<DefaultPipeline>,
) {
    let view = &frame_ctx.view;
    let mut encoder = render_ctx.device.create_command_encoder(&CommandEncoderDescriptor {
        label: Some("Game Object Encoder"),
    });

   let mut render_pass = pipeline
       .model_pass(&mut encoder, view,
           render_ctx.depth_texture.view()
       );

    for (model_cmpnt, instance_cmpnt) in &query {
        if instance_cmpnt.instance_buffer().is_none() {
            continue;
        }

        render_pass.draw_entity(model_cmpnt,
            instance_cmpnt,
            pipeline.camera_bind_group()
        );
    }

    frame_ctx.add_encoder(encoder);
}

// TODO: move this engine side
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
        
        let uniform: CameraUniform = (OPENGL_TO_WGPU_MATRIX * proj * view)
            .into();
        
        render_ctx.queue.write_buffer(pipeline.camera_buffer(),
            0, bytemuck::cast_slice(&uniform));
    }
}
