use std::sync::Arc;

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
            None
        );

        let renderer = egui_wgpu::Renderer::new(device,
            wgpu::TextureFormat::Bgra8Unorm,
            None,
            1
        );

        Self {
            state,
            renderer,
        }
    }

    pub fn window_event(&mut self, window: &Window, event: &WindowEvent) {
        let _ = self.state.on_window_event(window, event);
    }

    pub fn draw(&self, window: &Window, ui_fn: impl FnOnce(&egui::Context)) {
        let input = self.state.take_egui_input(window);
        let egui_ctx = self.state.egui_ctx();

        let output = self.state
            .run(input, |ui| {
                ui_fn(egui_ctx);
            });

        self.state.handle_platform_output(window, output.pl)
    }
}
