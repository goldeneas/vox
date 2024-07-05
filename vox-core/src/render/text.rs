use glyphon::{Attrs, Buffer, Cache, FontSystem, Metrics, Shaping, SwashCache, TextAtlas, TextRenderer, Viewport};
use wgpu::{Device, MultisampleState, Queue, TextureFormat};

pub struct GlyphonLabelDescriptor<'a> {
    width: f32,
    height: f32,
    text: &'a str,
    attributes: Attrs<'a>,
    shaping: Shaping,
    metrics: Metrics,
}

pub struct GlyphonRenderer {
    font_system: FontSystem,
    swash_cache: SwashCache,
    cache: Cache,
    viewport: Viewport,
    text_atlas: TextAtlas,
    renderer: TextRenderer,
    buffers: Vec<Buffer>,
}

impl GlyphonRenderer {
    pub fn new(device: &Device,
        queue: &Queue,
        format: TextureFormat) -> Self {
        let font_system = FontSystem::new();
        let swash_cache = SwashCache::new();
        let cache = Cache::new(device);
        let viewport = Viewport::new(device, &cache);
        let mut text_atlas = TextAtlas::new(device, queue, &cache, format);
        let renderer = TextRenderer::new(&mut text_atlas,
            device,
            MultisampleState::default(),
            None
        );
        let buffers = Vec::new();

        Self {
            font_system,
            swash_cache,
            cache,
            viewport,
            text_atlas,
            renderer,
            buffers,
        }
    }

    pub fn text(&mut self, descriptor: &GlyphonLabelDescriptor) -> &mut Self {
        let mut buffer = Buffer::new(&mut self.font_system, descriptor.metrics);
        buffer.set_size(&mut self.font_system,
            descriptor.width,
            descriptor.height
        );
        buffer.set_text(&mut self.font_system,
            descriptor.text, 
            descriptor.attributes,
            descriptor.shaping
        );
        self.buffers.push(buffer);

        self
    }
}
