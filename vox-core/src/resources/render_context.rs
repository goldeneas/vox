use std::sync::Arc;

use bevy_ecs::system::Resource;

use crate::{render::text::LabelRenderer, Texture};

#[derive(Resource)]
pub struct RenderContext {
    pub depth_texture: Arc<Texture>,
    pub device: wgpu::Device,
    pub surface: wgpu::Surface<'static>,
    pub config: wgpu::SurfaceConfiguration,
    pub queue: wgpu::Queue,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub renderer: LabelRenderer<'static>,
}
