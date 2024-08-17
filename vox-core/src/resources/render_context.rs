use std::sync::Arc;

use bevy_ecs::system::Resource;
use winit::window::Window;

use crate::{render::text::LabelRenderer, ui::egui_renderer::EguiRenderer, Texture};

#[derive(Resource)]
pub struct RenderContext {
    pub window: Arc<Window>,
    pub depth_texture: Arc<Texture>,
    pub device: wgpu::Device,
    pub surface: wgpu::Surface<'static>,
    pub config: wgpu::SurfaceConfiguration,
    pub queue: wgpu::Queue,
    pub size: winit::dpi::PhysicalSize<u32>,
    // TODO: remove this
    pub renderer: LabelRenderer<'static>,
}
