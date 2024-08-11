use std::sync::Arc;

use image::GenericImageView;
use crate::{assets::asset::Asset, util::load_binary};

pub struct Texture {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    sampler: wgpu::Sampler,
    name: String,
}

impl Asset for Texture {
    fn file_name(&self) -> &str {
        &self.name
    }
}

impl Texture {
    pub const DEPTH_TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
    pub const TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;

    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        file_name: &str,
    ) -> anyhow::Result<Texture> {
        let img = image::load_from_memory(bytes)?;
        Self::from_image(device, queue, &img, file_name)
    }
 
    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: &image::DynamicImage,
        file_name: &str
    ) -> anyhow::Result<Texture> {
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();
 
        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
 
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(file_name),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::TEXTURE_FORMAT,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
 
        queue.write_texture(
           wgpu::ImageCopyTexture {
               aspect: wgpu::TextureAspect::All,
               texture: &texture,
               mip_level: 0,
               origin: wgpu::Origin3d::ZERO,
           }, 
           &rgba,
           wgpu::ImageDataLayout {
               offset: 0,
               bytes_per_row: Some(4 * dimensions.0),
               rows_per_image: Some(dimensions.1),
           },
           size,
        );
 
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
 
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let name = file_name.to_string();

        Ok(Self {
            texture,
            view,
            sampler,
            name,
        })
    }

    pub fn load(
        file_name: &str,
        device: &wgpu::Device,
        queue: &wgpu::Queue
    ) -> anyhow::Result<Texture> {
        let data = load_binary(file_name)?;
        Texture::from_bytes(device, queue, &data, file_name)
    }
 
    pub fn create_depth_texture(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        name: &str
    ) -> Arc<Texture> {
        let size = wgpu::Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
        };
 
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(name),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_TEXTURE_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
 
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
 
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            compare: Some(wgpu::CompareFunction::LessEqual),
            lod_min_clamp: 0.0,
            lod_max_clamp: 100.0,
            ..Default::default()
        });

        let name = name.to_string();
 
        let texture = Self {
            texture,
            view,
            sampler,
            name,
        };

        Arc::new(texture)
    }

    pub fn view(&self) -> &wgpu::TextureView {
        &self.view
    }

    pub fn sampler(&self) -> &wgpu::Sampler {
        &self.sampler
    }
}
