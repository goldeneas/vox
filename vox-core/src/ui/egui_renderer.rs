use std::sync::Arc;

use bevy_ecs::system::IntoSystem;
use egui_wgpu::ScreenDescriptor;
use egui_winit::winit::event::WindowEvent;
use winit::window::Window;

pub struct EguiRenderer {
    state: egui_winit::State,
    renderer: egui_wgpu::Renderer,
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
            wgpu::TextureFormat::Bgra8Unorm,
            None,
            1,
            false
        );

        Self {
            state,
            renderer,
        }
    }

    pub fn window_event(&mut self, window: &Window, event: &WindowEvent) {
        let _ = self.state.on_window_event(window, event);
    }

    pub fn draw(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, encoder: &mut wgpu::CommandEncoder, window: &Window, ui_fn: impl FnOnce(&egui::Context), screen_descriptor: &ScreenDescriptor, view: &wgpu::TextureView) {
        let input = self.state.take_egui_input(window);

        let output = self.state
            .egui_ctx()
            .run(input, |ui| {
                ui_fn(ui);
            });

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

        self.renderer
            .update_buffers(device, queue, encoder, &tris, screen_descriptor);

        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            label: Some("egui main render pass"),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        self.renderer.render(&mut rpass, &tris, &screen_descriptor);

        output.textures_delta.free
            .iter()
            .for_each(|id| {
                self.renderer
                    .free_texture(id);
            });
    }
}
