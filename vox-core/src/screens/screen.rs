use bevy_ecs::system::{Res, ResMut};
use egui::Align2;
use wgpu::CommandEncoderDescriptor;

use crate::resources::{screen_context::ScreenContext, frame_context::FrameContext, gui_context::GuiContext, render_context::RenderContext};

pub trait GameScreen {
    fn start(&mut self);
}

#[derive(Default)]
pub struct MenuScreen {}

impl GameScreen for MenuScreen {
    fn start(&mut self) {
        screen_ctx.add_ui_systems(draw_menu);
    }
}
