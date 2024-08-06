use std::{any::{Any, TypeId}, collections::HashMap, hash::{DefaultHasher, Hash, Hasher}, rc::Rc};

use crate::{util::get_extension, Model, Texture};

use super::asset::Asset;

pub struct AssetServer {
    map: HashMap<(TypeId, u64), Rc<dyn Any>>,
}

impl AssetServer {
    pub fn new() -> Self {
        let map = HashMap::new();

        Self {
            map
        }
    }

    pub fn insert<T>(&mut self, asset: T)
        -> Option<Rc<(dyn Any + 'static)>>
    where
        T: Asset + 'static,
    {
        let type_id = TypeId::of::<T>();
        let file_name = asset.file_name();

        let hash = {
            let mut hasher = DefaultHasher::new();
            file_name.hash(&mut hasher);
            hasher.finish()
        };

        self.map.insert((type_id, hash), Rc::new(asset))
    }

    pub fn get<T>(&mut self, file_name: &str)
        -> Option<Rc<T>>
    where
        T: Asset + 'static,
    {
        let type_id = TypeId::of::<T>();
        let hash = {
            let mut hasher = DefaultHasher::new();
            file_name.hash(&mut hasher);
            hasher.finish()
        };

        self.map.get(&(type_id, hash))
            .and_then(|any| {
                any.clone()
                    .downcast::<T>()
                    .ok()
            })
    }

    pub fn get_or_load<T>(&mut self, file_name: &str, device: &wgpu::Device, queue: &wgpu::Queue)
        -> Option<Rc<T>>
    where
        T: Asset + 'static,
    {
        let type_id = TypeId::of::<T>();
        let hash = {
            let mut hasher = DefaultHasher::new();
            file_name.hash(&mut hasher);
            hasher.finish()
        };

        match self.map.get(&(type_id, hash)) {
            Some(any) => any.clone().downcast().ok(),
            None => {
                self.load::<T>(file_name, device, queue);
                self.get(file_name)
            }
        }
    }

    fn load<T>(&mut self, file_name: &str, device: &wgpu::Device, queue: &wgpu::Queue) {
        let extension = get_extension(file_name);

        match extension {
            Some("png") | Some("jpg") => self.load_texture(file_name, device, queue),
            Some("obj") => self.load_model(file_name, device, queue),
            _ => {}
        }
    }

    fn load_texture(&mut self, file_name: &str, device: &wgpu::Device, queue: &wgpu::Queue) {
        let texture = Texture::load(file_name, device, queue)
            .unwrap();

        self.insert(texture);
    }

    fn load_model(&mut self, file_name: &str, device: &wgpu::Device, queue: &wgpu::Queue) {
        let model = Model::load(file_name, self, device, queue)
            .unwrap();

        self.insert(model);
    }
}
