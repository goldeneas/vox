
use bevy_ecs::{system::Resource};
use glyphon::{Attrs, Buffer, Cache, Color, FontSystem, Metrics, Resolution, Shaping, SwashCache, TextArea, TextAtlas, TextBounds, TextRenderer, Viewport};
use wgpu::{CommandEncoderDescriptor, Device, MultisampleState, Queue};

use crate::resources::{frame_context::FrameContext, render_context::RenderContext};

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct LabelId(u32);

#[derive(Resource)]
pub struct GlyphonRenderer {
    font_system: FontSystem,
    swash_cache: SwashCache,
    pub viewport: Viewport,
    text_atlas: TextAtlas,
    renderer: TextRenderer,
    labels: Vec<Label>,
    labels_generated: u32,
}

struct Label {
    buffer: Buffer,
    descriptor: LabelDescriptor,
    id: LabelId,
}

pub struct LabelDescriptor {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub scale: f32,
    pub text: String,
    pub attributes: Attrs<'static>,
    pub shaping: Shaping,
    pub metrics: Metrics,
}

impl Default for LabelDescriptor {
    fn default() -> Self {
        LabelDescriptor {
            x: 0.0,
            y: 0.0,
            text: "Default Text".to_owned(),
            width: 1920.0,
            height: 1080.0,
            scale: 1.0,
            shaping: Shaping::Advanced,
            metrics: Metrics::new(30.0, 42.0),
            attributes: Attrs::new(),
        }
    }
}

impl Label {
    fn new(renderer: &mut GlyphonRenderer, descriptor: LabelDescriptor, id: LabelId) -> Self {
        let mut buffer = Buffer::new(&mut renderer.font_system, descriptor.metrics);
        buffer.set_size(&mut renderer.font_system,
            Some(descriptor.width),
            Some(descriptor.height)
        );
        buffer.set_text(&mut renderer.font_system,
            &descriptor.text, 
            descriptor.attributes,
            descriptor.shaping
        );

        Self {
            id,
            buffer,
            descriptor,
        }
    }

    fn get_area(&self) -> TextArea {
        TextArea {
            buffer: &self.buffer,
            top: self.descriptor.y,
            left: self.descriptor.x,
            scale: self.descriptor.scale,
            bounds: TextBounds::default(),
            default_color: Color::rgb(255, 255, 255),
        }
    }
}

impl GlyphonRenderer {
    pub fn new(device: &Device, queue: &Queue) -> Self {
        let font_system = FontSystem::new();
        let swash_cache = SwashCache::new();
        let cache = Cache::new(device);
        let viewport = Viewport::new(device, &cache);
        let mut text_atlas = TextAtlas::new(device,
            queue,
            &cache,
            wgpu::TextureFormat::Bgra8UnormSrgb
        );
        let renderer = TextRenderer::new(&mut text_atlas,
            device,
            MultisampleState::default(),
            None
        );
        let labels = Vec::new();
        let labels_generated = 0;

        Self {
            labels_generated,
            font_system,
            swash_cache,
            viewport,
            text_atlas,
            renderer,
            labels,
        }
    }

    pub fn prepare(&mut self, device: &Device, queue: &Queue) {
        self.renderer.prepare(device,
            queue,
            &mut self.font_system,
            &mut self.text_atlas,
            &self.viewport,
            self.labels.iter()
                .map(Label::get_area)
                .collect::<Vec<TextArea>>(),
            &mut self.swash_cache
        ).unwrap();
    }

    pub fn draw(&mut self,
        render_ctx: &RenderContext,
        frame_ctx: &mut FrameContext,
    ) {
        let view = &frame_ctx.view;
        let mut encoder = render_ctx.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Glyphon Label Encoder"),
        });

        self.viewport
            .update(&render_ctx.queue, Resolution {
                width: render_ctx.config.width,
                height: render_ctx.config.height,
            });

        self.prepare(&render_ctx.device, &render_ctx.queue);

        let mut pass = Self::pass(&mut encoder, view);
        self.renderer.render(&self.text_atlas, &self.viewport, &mut pass)
            .unwrap();

        frame_ctx.add_encoder(encoder);
    }

    pub fn add_label(&mut self, descriptor: LabelDescriptor) -> LabelId {
        let id = LabelId(self.labels_generated);
        let label = Label::new(self, descriptor, id);
        self.labels.push(label);
        self.labels_generated += 1;

        id
    }

    pub fn set_text(&mut self, id: LabelId, text: String) {
        let label = self.labels.iter_mut()
            .find(|label| { label.id == id })
            .unwrap();

        label.buffer.set_text(&mut self.font_system,
            &text, 
            label.descriptor.attributes,
            label.descriptor.shaping
        );
    }

    fn pass<'a>(encoder: &'a mut wgpu::CommandEncoder,
        view: &wgpu::TextureView
    ) -> wgpu::RenderPass<'a> {
        let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Glyphon Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
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

        render_pass
    }
}
