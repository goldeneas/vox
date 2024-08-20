use std::cell::Cell;

use bevy_ecs::{schedule::{IntoSystemConfigs, Schedule, SystemConfigs}, world::World};

use super::screen::Screen;

pub enum ScheduleType {
    Ui,
    Draw,
    Update,
}

#[derive(Default)]
pub struct ScreenSchedules {
    pub ui_schedule: Schedule,
    pub draw_schedule: Schedule,
    pub update_schedule: Schedule,
}

#[derive(Default)]
pub struct ScreenServer {
    schedules: Cell<ScreenSchedules>,
}

// TODO: update code please too much maintenance maybe
impl ScreenServer {
    pub fn draw(&self, world: &mut World) {
        self.take_schedules(|mut schedules| {
            schedules.draw_schedule
                .run(world);

            schedules.ui_schedule
                .run(world);

            schedules
        });
    }

    pub fn update(&mut self, world: &mut World) {
        self.take_schedules(|mut schedules| {
            schedules.update_schedule
                .run(world);

            schedules
        });
    }

    pub fn set_screen(&self, screen: &impl Screen) {
        screen.start();
        self.add_systems(ScheduleType::Update, screen.update_systems());
        self.add_systems(ScheduleType::Ui, screen.ui_systems(self));
        self.add_systems(ScheduleType::Draw, screen.draw_systems());
    }

    pub fn add_systems(&self, schedule_type: ScheduleType, systems: Option<SystemConfigs>) {
        if let None = systems {
            return;
        }

        self.take_schedules(|mut schedules| {
            let systems = systems.unwrap();
            let schedule = match schedule_type {
                ScheduleType::Ui => &mut schedules.ui_schedule,
                ScheduleType::Draw => &mut schedules.ui_schedule,
                ScheduleType::Update => &mut schedules.update_schedule,
            };

            schedule.add_systems(systems);
            schedules
        });
    }

    pub fn take_schedules(&self, func: impl FnOnce(ScreenSchedules) -> ScreenSchedules) {
        let mut schedules = self.schedules.take();
        schedules = func(schedules);
        self.schedules.replace(schedules);
    }
}
