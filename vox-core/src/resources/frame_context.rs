use bevy_ecs::system::Resource;

#[derive(Resource)]
pub struct FrameContext {
    pub output: wgpu::SurfaceTexture,
    pub view: wgpu::TextureView,
    pub encoders: Vec<wgpu::CommandEncoder>,
}

impl FrameContext {
    pub fn add_encoder(&mut self, encoder: wgpu::CommandEncoder) {
        self.encoders.push(encoder);
    }
}
