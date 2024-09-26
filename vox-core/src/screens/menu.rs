use std::process::exit;

use bevy_ecs::world::World;
use egui::{Align2, Button};

use crate::{resources::game_state::GameState, world_ext::WorldExt};

use super::screen::Screen;

#[derive(Default)]
pub struct MenuScreen {}

impl Screen for MenuScreen {
    fn start(&mut self, world: &mut World) {
        let mut egui_renderer = world.egui_renderer_mut();
        egui_renderer.add_window(GameState::Menu, |ctx, state| {
            egui::Window::new("Main Menu")
                .default_open(true)
                .default_size([200.0, 85.0])
                .resizable(false)
                .collapsible(false)
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    if ui.add_sized([200.0, 30.0], Button::new("Play")).clicked() {
                        state.set(GameState::Game);
                    }
    
                    if ui.add_sized([200.0, 30.0], Button::new("Quit")).clicked() {
                        exit(0);
                    }
    
                    ui.end_row();
                    ui.allocate_space(ui.available_size());
                });
        });
    }

    fn game_state(&self) -> GameState {
        GameState::Menu
    }
}
