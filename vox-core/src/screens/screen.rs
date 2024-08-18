use bevy_ecs::system::{Res, ResMut};
use egui::Align2;
use wgpu::CommandEncoderDescriptor;

use crate::resources::{screen_context::ScreenContext, frame_context::FrameContext, gui_context::GuiContext, render_context::RenderContext};

pub trait GameScreen {
    fn start(&mut self, screen_ctx: &mut ScreenContext);
}

#[derive(Default)]
pub struct MenuScreen {}

impl GameScreen for MenuScreen {
    fn start(&mut self, screen_ctx: &mut ScreenContext) {
        screen_ctx.add_ui_systems(draw_menu);
    }
}

#[derive(Default)]
pub struct RenderScreen {}

impl GameScreen for RenderScreen {
    fn start(&mut self, screen_ctx: &mut ScreenContext) {
        screen_ctx.add_ui_systems(say_hello);
    }
}

fn say_hello() {
    println!("HELLO");
}

fn draw_menu(render_ctx: Res<RenderContext>,
    mut frame_ctx: ResMut<FrameContext>,
    mut gui_ctx: ResMut<GuiContext>,
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
                    .max_width(1000.0)
                    .max_height(800.0)
                    .default_width(800.0)
                    .resizable(false)
                    .collapsible(false)
                    .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                    .show(context, |ui| {
                        if ui.add(egui::Button::new("Click me")).clicked() {
                            
                        }

                        ui.label("Slider");
                        //ui.add(egui::Slider::new(&mut 0, 0..=120).text("age"));
                        ui.end_row();
                    });
            });

    frame_ctx.add_encoder(encoder);
}
