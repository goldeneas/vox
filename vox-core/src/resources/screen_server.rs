use std::collections::HashMap;

use bevy_ecs::{schedule::{Schedule, SystemConfigs}, system::Resource, world::World};
use crate::{screens::screen::Screen, world_ext::WorldExt};

use super::game_state::GameState;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Cycle {
    Start,
    Ui,
    Draw,
    Update,
}

#[derive(Resource, Default)]
pub struct ScreenServer {
    last_state: Option<GameState>,
    registered_screens: Vec<Box<dyn Screen>>,
    registered_schedules: HashMap<GameState, HashMap<Cycle, Schedule>>,
}

impl ScreenServer {
    pub fn draw(&mut self, world: &mut World) {
        let state = world.game_state();

        if self.should_run_start_systems(state) {
            self.set_last_state(state);
            self.emit_event(world, Cycle::Start);
            self.run_schedule(world, state, Cycle::Start);
        }

        self.emit_event(world, Cycle::Draw);
        self.run_schedule(world, state, Cycle::Draw);

        self.emit_event(world, Cycle::Ui);
        self.run_schedule(world, state, Cycle::Ui);
    }

    pub fn update(&mut self, world: &mut World) {
        let state = world.game_state();

        if self.should_run_start_systems(state) {
            self.set_last_state(state);
            self.emit_event(world, Cycle::Start);
            self.run_schedule(world, state, Cycle::Start);
        }

        self.emit_event(world, Cycle::Update);
        self.run_schedule(world, state, Cycle::Update);
    }

    pub fn register_screens(&mut self,
        vector: Vec<Box<dyn Screen>>
    ) {
        vector
            .into_iter()
            .for_each(|screen| {
                self.register_screen_boxed(screen);
            });
    }

    pub fn register_screen(&mut self, screen: impl Screen) {
        self.register_screen_boxed(Box::new(screen));
    }

    fn register_screen_boxed(&mut self, screen: Box<dyn Screen>) {
        self.register_screen_systems(screen.as_ref());
        self.registered_screens.push(screen);
    }

    fn register_screen_systems(&mut self, screen: &dyn Screen) {
        let state = screen.game_state();

        self.add_systems(state, Cycle::Start, screen.start_systems());
        self.add_systems(state, Cycle::Ui, screen.ui_systems());
        self.add_systems(state, Cycle::Draw, screen.draw_systems());
        self.add_systems(state, Cycle::Update, screen.update_systems());
    }

    fn add_systems(&mut self,
        state: GameState,
        cycle: Cycle,
        systems: Option<SystemConfigs>,
    ) {
        if systems.is_none() {
            return;
        }

        // dayum pretty bad but I guess it doesnt happen that often to care
        let systems = systems.unwrap();
        match self.registered_schedules.get_mut(&state) {
            Some(state_map) => {
                match state_map.get_mut(&cycle) {
                    Some(schedule) => {
                        schedule.add_systems(systems);
                    },
                    None => {
                        state_map.insert(cycle, Schedule::default());
                        self.add_systems(state, cycle, Some(systems));
                    },
                };
            },
            None => {
                let mut state_map = HashMap::new();
                state_map.insert(cycle, Schedule::default());

                self.registered_schedules.insert(state, state_map);
                self.add_systems(state, cycle, Some(systems));
            },
        }
    }

    fn emit_event(&mut self, world: &mut World, cycle: Cycle) {
        self.registered_screens
            .iter_mut()
            .for_each(|screen| {
                if screen.game_state() != self.last_state.unwrap() {
                    return;
                }

                match cycle {
                    Cycle::Start => screen.start(world),
                    Cycle::Update => screen.update(world),
                    Cycle::Ui => screen.ui(world),
                    Cycle::Draw => screen.draw(world),
                }
            });
    }

    fn run_schedule(&mut self,
        world: &mut World,
        state: GameState,
        cycle: Cycle
    ) {
        if let Some(state_map) = self.get_state_map(state) {
            if let Some(schedule) = state_map.get_mut(&cycle) {
                schedule.run(world);
            }
        }
    }

    fn set_last_state(&mut self, state: GameState) {
        self.last_state = Some(state);
    }

    fn should_run_start_systems(&self, state: GameState) -> bool {
        match self.last_state {
            Some(last_state) => last_state != state,
            None => true
        }
    }

    fn get_state_map(&mut self,
        state: GameState
    ) -> Option<&mut HashMap<Cycle, Schedule>> {
        self.registered_schedules
            .get_mut(&state)
    }
}
