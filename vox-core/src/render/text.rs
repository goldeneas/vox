use glyphon::{Attrs, Buffer, Cache, Color, FontSystem, Metrics, Shaping, SwashCache, TextArea, TextAtlas, TextBounds, TextRenderer, Viewport};
use wgpu::{Device, MultisampleState, Queue, RenderPass};

pub struct GlyphonRenderer<'a> {
    font_system: FontSystem,
    swash_cache: SwashCache,
    pub viewport: Viewport,
    text_atlas: TextAtlas,
    renderer: TextRenderer,
    labels: Vec<GlyphonLabel<'a>>,
}

struct GlyphonLabel<'a> {
    buffer: Buffer,
    descriptor: GlyphonLabelDescriptor<'a>,
}

pub struct GlyphonLabelDescriptor<'a> {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub scale: f32,
    pub text: &'a str,
    pub attributes: Attrs<'a>,
    pub shaping: Shaping,
    pub metrics: Metrics,
}

impl<'a> GlyphonLabel<'a> {
    fn new(renderer: &mut GlyphonRenderer, descriptor: GlyphonLabelDescriptor<'a>) -> Self {
        let mut buffer = Buffer::new(&mut renderer.font_system, descriptor.metrics);
        buffer.set_size(&mut renderer.font_system,
            descriptor.width,
            descriptor.height
        );
        buffer.set_text(&mut renderer.font_system,
            descriptor.text, 
            descriptor.attributes,
            descriptor.shaping
        );

        Self {
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

        Self {
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

    pub fn add_label(&mut self, descriptor: GlyphonLabelDescriptor<'a>) {
        let label = GlyphonLabel::new(self, descriptor);
        self.labels.push(label);
    }
}
