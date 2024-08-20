use bevy_ecs::schedule::{IntoSystemConfigs, SystemConfigs};

pub trait Screen {
    fn start_systems(&self) -> Option<SystemConfigs>;
    fn ui_systems(&self) -> Option<SystemConfigs>;
    fn draw_systems(&self) -> Option<SystemConfigs>;
    fn update_systems(&self) -> Option<SystemConfigs>;

    fn to_systems<M>(&self,
        systems: impl IntoSystemConfigs<M>, 
    ) -> Option<SystemConfigs> {
        Some(systems.into_configs())
    }
}
