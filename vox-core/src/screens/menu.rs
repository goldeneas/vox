use std::process::exit;

use bevy_ecs::{schedule::{IntoSystemConfigs, SystemConfigs}, system::{Res, ResMut}};
use egui::{Align2, Button};
use wgpu::CommandEncoderDescriptor;

use crate::resources::{frame_context::FrameContext, game_state::GameState, gui_context::GuiContext, render_context::RenderContext};

use super::screen::Screen;

#[derive(Default)]
pub struct MenuScreen {}

impl Screen for MenuScreen {
    fn ui_systems(&self) -> Option<SystemConfigs> {
        self.to_systems(draw_menu)
    }

    fn game_state(&self) -> &GameState {
        &GameState::Menu
    }
}

// TODO: it's currently too hard to create a menu or something
// make this and glyphon easier, maybe dont use an ecs?
fn draw_menu(render_ctx: Res<RenderContext>,
    mut frame_ctx: ResMut<FrameContext>,
    mut gui_ctx: ResMut<GuiContext>,
    mut state: ResMut<GameState>,
) {
    let view = &frame_ctx.view;
    let mut encoder = render_ctx.device.create_command_encoder(&CommandEncoderDescriptor {
        label: Some("Egui Encoder"),
    });

    gui_ctx.egui_renderer
        .draw(&render_ctx,
            &mut encoder,
            view,
            |context| {
                egui::Window::new("Main Menu")
                    .default_open(true)
                    .default_size([200.0, 85.0])
                    .resizable(false)
                    .collapsible(false)
                    .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                    .show(context, |ui| {
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

    frame_ctx.add_encoder(encoder);
}
