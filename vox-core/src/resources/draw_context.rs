use std::sync::Arc;

use bevy_ecs::system::Resource;

#[derive(Resource)]
pub struct DrawContext {
    pub encoder: Option<Arc<wgpu::CommandEncoder>>,
}
