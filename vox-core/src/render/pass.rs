use bevy_ecs::prelude::*;

#[derive(Resource)]
pub struct DefaultPass {
    pub render_pipeline: wgpu::RenderPipeline,
}
