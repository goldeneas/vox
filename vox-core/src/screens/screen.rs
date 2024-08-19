use bevy_ecs::{schedule::IntoSystemConfigs, system::{Res, ResMut}};
use egui::Align2;
use wgpu::CommandEncoderDescriptor;

use crate::{resources::{frame_context::FrameContext, gui_context::GuiContext, render_context::RenderContext}, systems::draw::draw_cameras};

pub trait Screen {
    fn start(&self);
    fn ui_systems<M>(&self) -> impl IntoSystemConfigs<M>;
    fn draw_systems<M>(&self) -> impl IntoSystemConfigs<M>;
    fn update_systems<M>(&self) -> impl IntoSystemConfigs<M>;
}

pub struct MenuScreen {}
impl Screen for MenuScreen {
    fn start(&self) {
    
    }

    fn ui_systems<M>(&self) -> impl IntoSystemConfigs<M> {
        
    }

    fn draw_systems<M>(&self) -> impl IntoSystemConfigs<M> {
        todo!()
    }

    fn update_systems<M>(&self) -> impl IntoSystemConfigs<M> {
        todo!()
    }
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
