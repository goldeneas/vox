use bevy_ecs::prelude::*;

#[derive(Resource, Default, Debug)]
pub struct MouseRes {
    pub pos: (f64, f64),
}
