use std::{hint::black_box, thread::sleep, time::{Duration, Instant}};

use bevy_ecs::{schedule::SystemConfigs, system::{Query, Res, ResMut}};
use wgpu::CommandEncoderDescriptor;

use crate::{components::{model::ModelComponent, single_instance::SingleInstanceComponent}, resources::{asset_server::AssetServer, default_pipeline::DefaultPipeline, frame_context::FrameContext, game_state::GameState, render_context::RenderContext}, DrawObject, Model};

use super::screen::Screen;

#[derive(Default)]
pub struct BenchScreen {}
impl Screen for BenchScreen {
    fn update_systems(&self) -> Option<SystemConfigs> {
        self.to_systems(draw_single_instance_entities)
    }

    fn game_state(&self) -> &GameState {
        &GameState::Benchmark    
    }
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
