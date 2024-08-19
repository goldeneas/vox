use bevy_ecs::prelude::*;
use cgmath::Matrix4;
use egui::Align2;
use glyphon::Resolution;
use wgpu::CommandEncoderDescriptor;

use crate::{components::{camerable::CamerableComponent, model::ModelComponent, position::PositionComponent, single_instance::SingleInstanceComponent}, resources::{default_pipeline::DefaultPipeline, frame_context::FrameContext, gui_context::GuiContext, render_context::RenderContext}, ui::glyphon_renderer, DrawObject};

#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

fn draw_menu(render_ctx: Res<RenderContext>,
    mut frame_ctx: ResMut<FrameContext>,
    mut gui_ctx: ResMut<GuiContext>,
) {
    let view = &frame_ctx.view;
    let mut encoder = render_ctx.device.create_command_encoder(&CommandEncoderDescriptor {
        label: Some("Egui Encoder"),
    });

    gui_ctx.egui_renderer
        .draw(&render_ctx,
            &mut encoder,
            view,
            |context| {
                egui::Window::new("Main Menu")
                    .default_open(true)
                    .max_width(1000.0)
                    .max_height(800.0)
                    .default_width(800.0)
                    .resizable(false)
                    .collapsible(false)
                    .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                    .show(context, |ui| {
                        if ui.add(egui::Button::new("Click me")).clicked() {
                            
                        }

                        ui.label("Slider");
                        //ui.add(egui::Slider::new(&mut 0, 0..=120).text("age"));
                        ui.end_row();
                    });
            });

    frame_ctx.add_encoder(encoder);
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
