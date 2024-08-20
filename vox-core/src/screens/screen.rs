use bevy_ecs::{schedule::{IntoSystemConfigs, SystemConfigs}, system::{IntoSystem, Res, ResMut}};
use egui::Align2;
use wgpu::CommandEncoderDescriptor;

use crate::{resources::{frame_context::FrameContext, gui_context::GuiContext, render_context::RenderContext}, systems};

use super::screen_server::{self, ScreenServer};

pub trait Screen {
    fn start(&self);
    fn ui_systems(&self, screen_server: &ScreenServer) -> Option<SystemConfigs>;
    fn draw_systems(&self) -> Option<SystemConfigs>;
    fn update_systems(&self) -> Option<SystemConfigs>;

    fn to_systems<M>(&self,
        systems: impl FnOnce(dyn IntoSystemConfigs<M>) -> dyn IntoSystemConfigs<M> 
    ) -> Option<SystemConfigs> {
        Some(systems.into_configs())
    }
}

#[derive(Default)]
pub struct MenuScreen {}

impl Screen for MenuScreen {
    fn start(&self) {
    
    }

    fn ui_systems(&self, screen_server: &ScreenServer) -> Option<SystemConfigs> {
        screen_server.take_schedules(|mut schedules| {
            schedules.ui_schedule
                .add_systems(move
                    |render_ctx: Res<RenderContext>,
                    mut frame_ctx: ResMut<FrameContext>,
                    mut gui_ctx: ResMut<GuiContext>| {
                        draw_menu(render_ctx, frame_ctx, gui_ctx, screen_server);
                    });

            schedules
        });

        None
    }

    fn draw_systems(&self) -> Option<SystemConfigs> {
        None
    }

    fn update_systems(&self) -> Option<SystemConfigs> {
        None
    }
}

fn draw_menu(render_ctx: Res<RenderContext>,
    mut frame_ctx: ResMut<FrameContext>,
    mut gui_ctx: ResMut<GuiContext>,
    screen_server: &ScreenServer,
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
