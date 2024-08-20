use std::collections::HashMap;

use bevy_ecs::{schedule::{Schedule, SystemConfigs}, system::Resource, world::World};
use crate::screens::screen::Screen;

use super::game_state::GameState;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum AtCycle {
    Start,
    Ui,
    Draw,
    Update,
}

#[derive(Resource, Default)]
pub struct ScreenServer {
    last_state: Option<GameState>,
    map: HashMap<GameState, HashMap<AtCycle, Schedule>>,
}

// TODO: update code please too much maintenance maybe
impl ScreenServer {
    pub fn draw(&mut self, world: &mut World, state: &GameState) {
        if self.should_run_start_systems(state) {
            self.set_last_state(state);
            self.run_schedule(world, state, &AtCycle::Start);
        }

        self.run_schedule(world, state, &AtCycle::Draw);
        self.run_schedule(world, state, &AtCycle::Ui);
    }

    pub fn update(&mut self, world: &mut World, state: &GameState) {
        if self.should_run_start_systems(state) {
            self.run_schedule(world, state, &AtCycle::Start);
        }

        self.run_schedule(world, state, &AtCycle::Update);
    }

    pub fn register_screen(&mut self, screen: &impl Screen, state: &GameState) {
        self.add_systems(state, &AtCycle::Start, screen.start_systems());
        self.add_systems(state, &AtCycle::Ui, screen.ui_systems());
        self.add_systems(state, &AtCycle::Draw, screen.draw_systems());
        self.add_systems(state, &AtCycle::Update, screen.update_systems());
    }

    fn add_systems(&mut self,
        state: &GameState,
        cycle: &AtCycle,
        systems: Option<SystemConfigs>,
    ) {
        if let None = systems {
            return;
        }

        // dayum pretty bad but I guess it doesnt happen that often to care
        let systems = systems.unwrap();
        match self.map.get_mut(state) {
            Some(state_map) => {
                match state_map.get_mut(cycle) {
                    Some(schedule) => {
                        schedule.add_systems(systems);
                    },
                    None => {
                        state_map.insert(*cycle, Schedule::default());
                        self.add_systems(state, cycle, Some(systems));
                    },
                };
            },
            None => {
                let mut state_map = HashMap::new();
                state_map.insert(*cycle, Schedule::default());

                self.map.insert(*state, state_map);
                self.add_systems(state, cycle, Some(systems));
            },
        }
    }

    fn get_state_map(&mut self,
        state: &GameState
    ) -> &mut HashMap<AtCycle, Schedule> {
        self.map.get_mut(state)
            .unwrap()
    }

    fn run_schedule(&mut self,
        world: &mut World,
        state: &GameState,
        cycle: &AtCycle
    ) {
        self.get_state_map(state)
            .get_mut(cycle)
            .map(|schedule| {
                schedule.run(world);
            });
    }

    fn set_last_state(&mut self, state: &GameState) {
        self.last_state = Some(*state);
    }

    fn should_run_start_systems(&self, state: &GameState) -> bool {
        match self.last_state {
            Some(last_state) => last_state != *state,
            None => true
        }
    }
}
