use std::collections::HashMap;

use bevy_ecs::system::Resource;
use egui::Context;
use egui_plot::PlotUi;
use egui_wgpu::ScreenDescriptor;
use egui_winit::winit::event::WindowEvent;
use wgpu::CommandEncoderDescriptor;
use winit::window::Window;

use crate::resources::{frame_context::FrameContext, game_state::GameState, render_context::RenderContext};

type ScreenCallback = dyn Fn(&Context, &mut GameState) + Send + Sync;

#[derive(Resource)]
pub struct EguiRenderer {
    state: egui_winit::State,
    renderer: egui_wgpu::Renderer,
    window_funcs: HashMap<GameState, Box<ScreenCallback>>,
}

impl EguiRenderer {
    pub fn new(device: &wgpu::Device, window: &Window) -> Self {
        let context = egui::Context::default();
        let viewport_id = context.viewport_id();

        let state = egui_winit::State::new(context,
            viewport_id,
            window,
            None,
            None,
            None
        );

        let renderer = egui_wgpu::Renderer::new(device,
            wgpu::TextureFormat::Bgra8UnormSrgb,
            None,
            1,
            false
        );

        let window_funcs = HashMap::new();

        Self {
            state,
            renderer,
            window_funcs,
        }
    }

    pub fn window_event(&mut self, window: &Window, event: &WindowEvent) {
        let _ = self.state.on_window_event(window, event);
    }

    pub fn add_window(&mut self,
        required_state: GameState,
        func: impl Fn(&Context, &mut GameState) + Send + Sync + 'static
    ) {
        let func = Box::new(func);
        self.window_funcs.insert(required_state, func);
    }

    pub fn draw(&mut self,
        render_ctx: &RenderContext,
        frame_ctx: &mut FrameContext,
        state: &mut GameState,
    ) {
        let device = &render_ctx.device;
        let queue = &render_ctx.queue;
        let config = &render_ctx.config;
        let window = &render_ctx.window;

        let view = &frame_ctx.view;
        let mut encoder = render_ctx.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Egui Encoder"),
        });

        let input = self.state.take_egui_input(window);
        let context = self.state.egui_ctx();

        // TODO: add egui_plot

        context.begin_frame(input);
        self.window_funcs
            .iter()
            .for_each(|(required_state, func)| {
                if required_state != state {
                    return;
                }

                func(context, state);
            });
        let output = context.end_frame();

        self.state.handle_platform_output(window, output.platform_output);

        let tris = self.state
            .egui_ctx()
            .tessellate(output.shapes,
                output.pixels_per_point
            );

        output.textures_delta.set
            .into_iter()
            .for_each(|(id, image_delta)| {
                self.renderer
                    .update_texture(device, queue, id, &image_delta);
            });

        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [config.width, config.height],
            pixels_per_point: window.scale_factor() as f32,
        };

        self.renderer
            .update_buffers(device, queue, &mut encoder, &tris, &screen_descriptor);

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            label: Some("Egui Pass"),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        self.renderer.render(&mut render_pass, &tris, &screen_descriptor);

        output.textures_delta.free
            .iter()
            .for_each(|id| {
                self.renderer
                    .free_texture(id);
            });

        frame_ctx.add_encoder(encoder);
    }
}
