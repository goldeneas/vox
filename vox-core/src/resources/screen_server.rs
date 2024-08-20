use std::{any::{Any, TypeId}, collections::HashMap, default};

use bevy_ecs::{schedule::{IntoSystemConfigs, Schedule, SystemConfig, SystemConfigs}, system::Resource, world::World};

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
    map: HashMap<GameState, HashMap<AtCycle, Schedule>>,
}

// TODO: update code please too much maintenance maybe
impl ScreenServer {
    pub fn draw(&mut self, world: &mut World, state: &GameState) {
        self.map.get_mut(state)
            .unwrap()
            .get_mut(&AtCycle::Draw)
            .map(|schedule| {
                schedule.run(world);
            });

        self.map.get_mut(&state)
            .unwrap()
            .get_mut(&AtCycle::Ui)
            .map(|schedule| {
                schedule.run(world);
            });
    }

    pub fn update(&mut self, world: &mut World, state: &GameState) {
        self.map.get_mut(state)
            .unwrap()
            .get_mut(&AtCycle::Update)
            .map(|schedule| {
                schedule.run(world);
            });
    }

    pub fn register_screen(&mut self, screen: &impl Screen, state: &GameState) {
        self.add_systems(state, AtCycle::Start, screen.start_systems());
        self.add_systems(state, AtCycle::Ui, screen.ui_systems());
        self.add_systems(state, AtCycle::Draw, screen.draw_systems());
        self.add_systems(state, AtCycle::Update, screen.update_systems());
    }

    // TODO dayum pretty bad but I guess it doesnt happen that often to care
    fn add_systems(&mut self,
        state: &GameState,
        cycle: AtCycle,
        systems: Option<SystemConfigs>,
    ) {
        if let None = systems {
            return;
        }

        let systems = systems.unwrap();
        match self.map.get_mut(state) {
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
                self.map.insert(*state, HashMap::new());
                self.add_systems(state, cycle, Some(systems));
            },
        }
    }
}
