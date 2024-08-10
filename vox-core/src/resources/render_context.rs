use std::sync::Arc;

use bevy_ecs::system::Resource;

use crate::{render::text::LabelRenderer, Texture};

#[derive(Resource)]
pub struct RenderContext<'a> {
    pub depth_texture: Arc<Texture>,
    pub device: Arc<wgpu::Device>,
    pub surface: Arc<wgpu::Surface<'a>>,
    pub render_pipeline: wgpu::RenderPipeline,
    pub encoder: Option<wgpu::CommandEncoder>,
    pub view: Option<wgpu::TextureView>,
    pub renderer: LabelRenderer<'a>,
}

impl<'a> RenderContext<'a> {
    pub fn default_pass(&mut self)-> Result<wgpu::RenderPass, wgpu::SurfaceError> {
        let view = self.view.as_ref().unwrap();
        let encoder = self.encoder.as_mut().unwrap();

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            // this is what @location(0) in the fragment shader targets
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),

                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth_texture.view(),
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);

        Ok(render_pass)
    }
}
