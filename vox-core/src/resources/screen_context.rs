use bevy_ecs::{schedule::{IntoSystemConfigs, Schedule}, system::Resource};

use crate::screens::screen::GameScreen;

#[derive(Resource)]
pub struct ScreenContext {
    pub ui_schedule: Schedule,
    pub update_schedule: Schedule,
    pub draw_schedule: Schedule,
}

impl ScreenContext {
    pub fn set_screen(screen_ctx: &mut ScreenContext,
        screen: &mut impl GameScreen
    ) {
        screen_ctx.clear_schedules();
        screen.start(screen_ctx);
    }

    pub fn clear_schedules(&mut self) {
        self.ui_schedule = Schedule::default();
        self.draw_schedule = Schedule::default();
        self.update_schedule = Schedule::default();
    }

    pub fn add_ui_systems<M>(&mut self,
        systems: impl IntoSystemConfigs<M>
    ) {
        self.ui_schedule
            .add_systems(systems);
    }

    pub fn add_draw_systems<M>(&mut self,
        systems: impl IntoSystemConfigs<M>
    ) {
        self.draw_schedule
            .add_systems(systems);
    }

    pub fn add_update_systems<M>(&mut self,
        systems: impl IntoSystemConfigs<M>
    ) {
        self.update_schedule
            .add_systems(systems);
    }
}
