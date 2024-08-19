use bevy_ecs::system::Resource;

#[derive(Resource)]
pub enum ScreenContext {
    Menu,
    Gameplay,
}
