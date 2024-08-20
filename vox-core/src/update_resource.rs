use bevy_ecs::world::World;

use crate::resources::screen_server::{ScreenServer, ScreenServer};

pub trait UpdateResource {
    fn update_screen_server(&mut self,
        func: impl FnOnce(&mut Self, &mut ScreenServer)
    );
}

impl UpdateResource for World {
    fn update_screen_server(&mut self,
        func: impl FnOnce(&mut Self, &mut ScreenServer)) {
        let binding = self
            .resource_mut::<ScreenServer>()
            .0.clone();

        let screen_server = &mut binding.lock().unwrap();
        func(self, screen_server);
    }
}
