use std::{any::{Any, TypeId}, collections::HashMap, hash::{DefaultHasher, Hash, Hasher}, path::Path, rc::Rc};

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
}
