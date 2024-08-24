use std::process::exit;

use bevy_ecs::{schedule::SystemConfigs, system::ResMut};
use egui::{Align2, Button};

use crate::{resources::game_state::GameState, ui::egui_renderer::EguiRenderer};

use super::screen::Screen;

#[derive(Default)]
pub struct MenuScreen {}

impl Screen for MenuScreen {
    fn start_systems(&self) -> Option<SystemConfigs> {
        self.to_systems(add_menu)
    }

    fn game_state(&self) -> GameState {
        GameState::Menu
    }
}

fn add_menu(mut egui_renderer: ResMut<EguiRenderer>,) {
    egui_renderer.add_window(|ctx, state| {
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
