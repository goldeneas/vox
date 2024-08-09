use bevy_ecs::prelude::*;

use crate::{assets::asset_server::AssetServer, components::{PositionComponent, RenderComponent}, InstanceTransform};

pub struct Character {
    pub id: Entity,
}

impl Character {
    fn new(world: &mut World,
        asset_server: &mut AssetServer,
        device: &wgpu::Device,
        queue: &wgpu::Queue
    ) -> Self {
        let position = PositionComponent {
            x: 0.0,
            y: 0.0,
            z: 0.0
        };

        let render = RenderComponent::new(asset_server
            .get_or_load("debug.png", device, queue)
            .unwrap(),
        );

        let id = world.spawn((
                position,
                render,
            )).id();

        Self {
            id,
        }
    }
}
