use bevy_ecs::prelude::*;
use cgmath::Matrix4;
use wgpu::CommandEncoderDescriptor;

use crate::{components::{camerable::CamerableComponent, model::ModelComponent, position::PositionComponent, single_instance::SingleInstanceComponent}, resources::{default_pipeline::DefaultPipeline, frame_context::FrameContext, render_context::RenderContext}, DrawObject};

#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

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
        let mut render_pass = pipeline.model_pass(&mut encoder,
            &view,
            render_ctx.depth_texture.view()
        ).unwrap();

        render_pass.draw_entity(model_cmpnt,
            instance_cmpnt,
            pipeline.camera_bind_group()
        );
    }

    frame_ctx.add_encoder(encoder);
}

pub fn draw_glyphon_labels(render_ctx: Res<RenderContext>,
    mut frame_ctx: ResMut<FrameContext>,
    pipeline: Res<DefaultPipeline>) {
    let view = &frame_ctx.view;
    let mut encoder = render_ctx.device.create_command_encoder(&CommandEncoderDescriptor {
        label: Some("Glyphon Label Encoder"),
    });

    {
        let mut pass = pipeline.glyphon_pass(&mut encoder,
            &view,
        ).unwrap();

        render_ctx.renderer.draw(&mut pass);
    }

    frame_ctx.add_encoder(encoder);
}

pub fn draw_camera(query: Query<(
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
