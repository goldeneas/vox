use image::GenericImageView;

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture {
   pub fn from_bytes(
       device: &wgpu::Device,
       queue: &wgpu::Queue,
       bytes: &[u8],
       label: &str
   ) -> Result<Self> {
       let img = image::load_from_memory(bytes)?;

   }

   pub fn from_image(
       device: &wgpu::Device,
       queue: &wgpu::Queue,
       img: &image::DynamicImage,
       label: Option<&str>
   ) -> Result<Self> {
       let rgba = img.to_rgba8();
       let dimensions = img.dimensions();

       let size = wgpu::Extent3d {
           width: dimensions.0,
           height: dimensions.1,
           depth_or_array_layers: 1,
       };

       let texture = device.create_texture(&wgpu::TextureDescriptor {
           label,
           size,
           mip_level_count: 1,
           sample_count: 1,
           dimension: wgpu::TextureDimension::D2,
           format: wgpu::TextureFormat::Rgba8UnormSrgb,
           usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
           view_formats: &[]
       });
   }
}
