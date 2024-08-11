use bevy_ecs::prelude::*;
use cgmath::Matrix4;
use wgpu::CommandEncoderDescriptor;

use crate::{components::{camerable::CamerableComponent, model::ModelComponent, position::PositionComponent, single_instance::SingleInstanceComponent}, resources::{default_pipeline::DefaultPipeline, render_context::RenderContext}, DrawObject};

#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

// TODO CREATE CAMERA UNIFORM TYPE FOR F32;4;4

// PURPLE = NO ENTITIES IN SYSTEM
pub fn draw_single_instance_models(query: Query<(
        &ModelComponent,
        &SingleInstanceComponent)>,
        ctx: Res<RenderContext<'static>>,
        pipeline: Res<DefaultPipeline>,
) {
    let output = ctx.surface.get_current_texture().unwrap();
    let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder = ctx.device.create_command_encoder(&CommandEncoderDescriptor {
        label: Some("Glyphon Label Encoder"),
    });

    for (model_cmpnt, instance_cmpnt) in &query {
        let mut render_pass = pipeline.model_pass(&mut encoder,
            &view,
            &ctx.depth_texture.view()
        ).unwrap();

        render_pass.draw_entity(&model_cmpnt,
            &instance_cmpnt,
            &pipeline.camera_bind_group()
        );
    }

    ctx.queue.submit(std::iter::once(encoder.finish()));
    output.present();
}

pub fn draw_glyphon_labels(ctx: Res<RenderContext<'static>>,
    pipeline: Res<DefaultPipeline>) {
    let output = ctx.surface.get_current_texture().unwrap();
    let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder = ctx.device.create_command_encoder(&CommandEncoderDescriptor {
        label: Some("Glyphon Label Encoder"),
    });

    {
        let mut pass = pipeline.glyphon_pass(&mut encoder,
            &view,
        ).unwrap();

        ctx.renderer.draw(&mut pass);
    }

    ctx.queue.submit(std::iter::once(encoder.finish()));
    output.present();
}

pub fn draw_camera(query: Query<(
    &PositionComponent,
    &CamerableComponent)>,
    ctx: Res<RenderContext<'static>>,
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
        
        ctx.queue.write_buffer(&pipeline.camera_buffer(),
            0, bytemuck::cast_slice(&uniform));
    }
}
