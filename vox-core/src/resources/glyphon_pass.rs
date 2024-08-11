use bevy_ecs::prelude::*;

#[derive(Resource)]
pub struct GlyphonPass {
    pub render_pipeline: wgpu::RenderPipeline,
}

impl GlyphonPass {
    pub fn render_pass<'a>(&'a self,
        encoder: &'a mut wgpu::CommandEncoder,
        view: &'a wgpu::TextureView,
    ) -> Result<wgpu::RenderPass, wgpu::SurfaceError> 
    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Glyphon Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);

        Ok(render_pass)
    }
}
