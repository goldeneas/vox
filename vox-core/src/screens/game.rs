use std::{sync::Arc, time::Instant};

use bevy_ecs::{schedule::SystemConfigs, system::{Commands, Query, Res, ResMut}, world::World};
use binary_greedy_meshing::CS_P;
use cgmath::{InnerSpace, Matrix4, Quaternion, Zero};
use wgpu::CommandEncoderDescriptor;

use crate::{bundles::object::Object, components::camerable::{CameraComponent, CameraUniform}, pass_ext::DrawPassExt, render::{face_primitive::{FaceDirection, FacePrimitive}, material::{Material, MaterialId}, mesh::AsMesh}, resources::{asset_server::AssetServer, default_pipeline::DefaultPipeline, frame_context::FrameContext, game_state::GameState, input::InputRes, mouse::MouseRes, render_context::RenderContext}, ui::glyphon_renderer::{LabelDescriptor, LabelId}, voxels::{chunk::Chunk, voxel_position::VoxelPosition, voxel_registry::VoxelType}, world_ext::WorldExt, AsModel, InstanceData, Model, Texture};

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
        self.to_systems((spawn_camera, spawn_chunks))
    }

    fn draw_systems(&self) -> Option<SystemConfigs> {
        self.to_systems((draw_objects, draw_camera))
    }

    fn update_systems(&self) -> Option<SystemConfigs> {
        self.to_systems(update_camera)
    }

    fn game_state(&self) -> GameState {
        GameState::Game
    }
}

pub fn update_camera(mut query: Query<&mut CameraComponent>,
    input_res: Res<InputRes>,
    mouse_res: Res<MouseRes>,
) {
    for mut camera_cmpnt in &mut query {
        let forward = camera_cmpnt.target - camera_cmpnt.position;
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();

        let speed = camera_cmpnt.speed;

        if input_res.forward.is_pressed && forward_mag > speed {
            camera_cmpnt.position += forward_norm * speed;
            //camera_transform.target += forward_norm * self.speed;
        }

        if input_res.backward.is_pressed {
            camera_cmpnt.position -= forward_norm * speed;
            //camera_transform.target -= forward_norm * self.speed;
        }

        let up_norm = camera_cmpnt.up.normalize();
        let right_norm = forward_norm.cross(up_norm);

        if input_res.right.is_pressed {
            camera_cmpnt.position += right_norm * speed; 
            //camera_transform.target += right_norm * self.speed;
        }

        if input_res.left.is_pressed {
            camera_cmpnt.position -= right_norm * speed;
            //camera_transform.target -= right_norm * self.speed;
        }

        let yaw: f32 = (mouse_res.pos.0 * 0.01) as f32;
        camera_cmpnt.target.x = 2.23 * yaw.cos();
        camera_cmpnt.target.z = 2.23 * yaw.sin();
    }
}

pub fn spawn_chunks(mut asset_server: ResMut<AssetServer>,
    mut commands: Commands,
    render_ctx: Res<RenderContext>
) {
    let mut chunk = Chunk::new();
    for x in 0..CS_P {
        for y in 0..CS_P {
            for z in 0..CS_P {
                if ((x*x + y*y + z*z) as f32).sqrt() > 60.0 { continue; }
                let position = VoxelPosition::from((x, y, z));
                chunk.set_voxel_type_at(position, VoxelType::DIRT);
            }
        }
    }

    chunk.update_faces();

    let device = &render_ctx.device;
    let queue = &render_ctx.queue;

    let texture = Texture::debug(&mut asset_server, device, queue);
    let material = Material::new(device, texture, "HI");
    let materials = vec![material];

    //let chunk_model = chunk.to_model(materials);
    //println!("{:?}", chunk_model);
    //
    //let chunk_data = InstanceData {
    //    position: (0.0, 0.0, 0.0).into(),
    //    rotation: Quaternion::zero(),
    //};

    let face = FacePrimitive {
        width: 1.0,
        height: 1.0,
        direction: FaceDirection::BACK,
        position: (0.0, 0.0, 0.0),
    };

    let object = Object {
        model: face.to_model(materials),
    };

    commands.spawn(object);
}

pub fn spawn_camera(mut commands: Commands,
    render_ctx: Res<RenderContext>,
) {
    commands.spawn(CameraComponent::debug(&render_ctx.config));
}

pub fn draw_objects(query: Query<&Model>,
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

    for model in &query {
        render_pass.draw_model(model,
            pipeline.camera_bind_group(),
            &render_ctx.device,
        );
    }

    frame_ctx.add_encoder(encoder);
}

// TODO: move this engine side
pub fn draw_camera(query: Query<&CameraComponent>,
    render_ctx: Res<RenderContext>,
    pipeline: Res<DefaultPipeline>,
) {
    for camera_cmpnt in &query {
        let view = Matrix4::look_at_rh(
            camera_cmpnt.position,
            camera_cmpnt.target,
            camera_cmpnt.up
        );
        
        let proj = cgmath::perspective(
            cgmath::Deg(camera_cmpnt.fovy),
            camera_cmpnt.aspect,
            camera_cmpnt.znear,
            camera_cmpnt.zfar
        );
        
        let uniform: CameraUniform = (OPENGL_TO_WGPU_MATRIX * proj * view)
            .into();
        
        render_ctx.queue.write_buffer(pipeline.camera_buffer(),
            0, bytemuck::cast_slice(&uniform));
    }
}
