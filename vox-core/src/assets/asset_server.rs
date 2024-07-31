use std::{any::{Any, TypeId}, collections::HashMap, hash::{DefaultHasher, Hash, Hasher}, path::Path};

use crate::{util::get_extension, Texture};

use super::asset::Asset;

pub struct AssetServer {
    map: HashMap<(TypeId, u64), Box<dyn Any>>,
}

impl AssetServer {
    pub fn new() -> Self {
        let map = HashMap::new();

        Self {
            map
        }
    }

    pub fn insert<T>(&mut self, id: T::AssetId, asset: T)
        -> Option<Box<(dyn Any + 'static)>>
    where
        T: Asset + 'static,
        T::AssetId: Hash
    {
        let type_id = TypeId::of::<T>();
        let hash = {
            let mut hasher = DefaultHasher::new();
            id.hash(&mut hasher);
            hasher.finish()
        };

        self.map.insert((type_id, hash), Box::new(asset))
    }

    pub fn get<T>(&mut self, id: T::AssetId)
        -> Option<&T>
    where
        T: Asset + 'static,
        T::AssetId: Hash
    {
        let type_id = TypeId::of::<T>();
        let hash = {
            let mut hasher = DefaultHasher::new();
            id.hash(&mut hasher);
            hasher.finish()
        };

        let any_opt = self.map.get(&(type_id, hash));
        any_opt.and_then(|any| any.downcast_ref())
    }

    pub fn load<T>(&self, device: &wgpu::Device, queue: &wgpu::Queue, file_name: &str) -> anyhow::Result<&T>
        where T: Asset + 'static {
           let extension_opt = get_extension(file_name);

           match extension_opt {
               Some(".png") => Texture::load(file_name, device, queue),
               _ => Err
           }
    }
}
