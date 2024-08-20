use bevy_ecs::{schedule::{IntoSystemConfigs, SystemConfigs}, system::{IntoSystem, Res, ResMut}};
use egui::Align2;
use wgpu::CommandEncoderDescriptor;

use crate::{resources::{frame_context::FrameContext, gui_context::GuiContext, render_context::RenderContext, screen_server::ScreenServer}, systems::{self, draw::draw_single_instance_entities}};

pub trait Screen {
    fn start(&self);
    fn ui_systems(&self) -> Option<SystemConfigs>;
    fn draw_systems(&self) -> Option<SystemConfigs>;
    fn update_systems(&self) -> Option<SystemConfigs>;

    fn to_systems<M>(&self,
        systems: impl IntoSystemConfigs<M>, 
    ) -> Option<SystemConfigs> {
        Some(systems.into_configs())
    }
}


#[derive(Default)]
pub struct GameScreen {}

impl Screen for GameScreen {
    fn start(&self) {
    
    }

    fn ui_systems(&self) -> Option<SystemConfigs> {
        self.to_systems(draw_single_instance_entities)
    }

    fn draw_systems(&self) -> Option<SystemConfigs> {
        None
    }

    fn update_systems(&self) -> Option<SystemConfigs> {
        None
    }
}

#[derive(Default)]
pub struct MenuScreen {}

impl Screen for MenuScreen {
    fn start(&self) {
    
    }

    fn ui_systems(&self) -> Option<SystemConfigs> {
        self.to_systems(draw_menu)
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
    mut screen_server: ResMut<ScreenServer>,
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
                            let screen = MenuScreen::default();
                            screen_server.set_screen(&screen);
                        }

                        ui.label("Slider");
                        //ui.add(egui::Slider::new(&mut 0, 0..=120).text("age"));
                        ui.end_row();
                    });
            });

    frame_ctx.add_encoder(encoder);
}
