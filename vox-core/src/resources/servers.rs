use bevy_ecs::prelude::*;

use crate::{assets::asset_server::AssetServer, screens::screen_server::ScreenServer};

#[derive(Resource)]
pub struct Servers {
    pub asset_server: AssetServer,
    pub screen_server: ScreenServer,
}
