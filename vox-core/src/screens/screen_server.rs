use bevy_ecs::{schedule::{IntoSystemConfigs, Schedule, SystemConfigs}, world::World};

use super::screen::Screen;

pub enum ScheduleType {
    Ui,
    Draw,
    Update,
}

pub struct ScreenServer {
    ui_schedule: Schedule,
    draw_schedule: Schedule,
    update_schedule: Schedule,
}

// TODO: update code please too much maintenance maybe
impl ScreenServer {
    pub fn new() -> Self {
        let ui_schedule = Schedule::default();
        let draw_schedule = Schedule::default();
        let update_schedule = Schedule::default();

        Self {
            ui_schedule,
            draw_schedule,
            update_schedule,
        }
    }

    pub fn draw(&mut self, world: &mut World) {
        self.draw_schedule
            .run(world);

        self.ui_schedule
            .run(world);
    }

    pub fn update(&mut self, world: &mut World) {
        self.update_schedule
            .run(world);
    }

    pub fn set_screen(&mut self, screen: &impl Screen) {
        self.reset_schedules();

        screen.start();
        self.add_system(ScheduleType::Update, screen.update_systems());
        self.add_system(ScheduleType::Ui, screen.ui_systems());
        self.add_system(ScheduleType::Draw, screen.draw_systems());
    }

    pub fn reset_schedules(&mut self) {
        self.draw_schedule = Schedule::default();
        self.ui_schedule = Schedule::default();
        self.update_schedule = Schedule::default();
    }

    pub fn add_system(&mut self, schedule_type: ScheduleType, systems: Option<SystemConfigs>) {
        if let None = systems {
            return;
        }

        let systems = systems.unwrap();
        let schedule = match schedule_type {
            ScheduleType::Ui => &mut self.ui_schedule,
            ScheduleType::Draw => &mut self.draw_schedule,
            ScheduleType::Update => &mut self.update_schedule,
        };

        schedule.add_systems(systems);
    }
}
