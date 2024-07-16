use glyphon::{Attrs, Buffer, Cache, Color, FontSystem, Metrics, Shaping, SwashCache, TextArea, TextAtlas, TextBounds, TextRenderer, Viewport};
use wgpu::{Device, MultisampleState, Queue, RenderPass};

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct GlyphonLabelId(u32);

pub struct GlyphonRenderer<'a> {
    font_system: FontSystem,
    swash_cache: SwashCache,
    pub viewport: Viewport,
    text_atlas: TextAtlas,
    renderer: TextRenderer,
    labels: Vec<GlyphonLabel<'a>>,
    labels_generated: u32,
}

struct GlyphonLabel<'a> {
    buffer: Buffer,
    descriptor: GlyphonLabelDescriptor<'a>,
    id: GlyphonLabelId,
}

pub struct GlyphonLabelDescriptor<'a> {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub scale: f32,
    pub text: String,
    pub attributes: Attrs<'a>,
    pub shaping: Shaping,
    pub metrics: Metrics,
}

impl<'a> GlyphonLabel<'a> {
    fn new(renderer: &mut GlyphonRenderer, descriptor: GlyphonLabelDescriptor<'a>, id: GlyphonLabelId) -> Self {
        let mut buffer = Buffer::new(&mut renderer.font_system, descriptor.metrics);
        buffer.set_size(&mut renderer.font_system,
            descriptor.width,
            descriptor.height
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

impl<'a> GlyphonRenderer<'a> {
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
        let _ = self.renderer.prepare(device,
            queue,
            &mut self.font_system,
            &mut self.text_atlas,
            &self.viewport,
            self.labels.iter()
                .map(GlyphonLabel::get_area)
                .collect::<Vec<TextArea>>(),
            &mut self.swash_cache
        ).expect("Could not prepare GlyphonRenderer");
    }

    pub fn draw<'pass>(&'a self, render_pass: &mut RenderPass<'pass>)
    where 'a : 'pass {
        let _ = self.renderer.render(&self.text_atlas, &self.viewport, render_pass)
            .expect("Could not draw GlyphonRenderer");
    }

    pub fn add_label(&mut self, descriptor: GlyphonLabelDescriptor<'a>) -> GlyphonLabelId {
        let id = GlyphonLabelId(self.labels_generated);
        let label = GlyphonLabel::new(self, descriptor, id);
        self.labels.push(label);
        self.labels_generated += 1;

        id
    }

    pub fn set_text(&mut self, id: GlyphonLabelId, text: String) {
        let label = self.labels.iter_mut()
            .find(|label| { label.id == id })
            .unwrap();

        label.buffer.set_text(&mut self.font_system,
            &text, 
            label.descriptor.attributes,
            label.descriptor.shaping
        );
    }
}
