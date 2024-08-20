mod render;
mod util;
mod bundles;
mod components;
mod resources;
mod assets;
mod systems;
mod ui;
mod screens;

use std::borrow::BorrowMut;
use std::sync::Arc;
use std::time::Instant;

use assets::asset_server::AssetServer;
use bevy_ecs::world::World;
use render::model::*;
use screens::screen::MenuScreen;
use screens::screen_server::ScreenServer;
use ui::glyphon_renderer::GlyphonRenderer;
use render::texture::*;
use render::instance::*;

use resources::default_pipeline::DefaultPipeline;
use resources::frame_context::FrameContext;
use resources::gui_context::GuiContext;
use resources::render_context::RenderContext;
use resources::input::InputRes;
use resources::input::KeyState;
use resources::mouse::MouseRes;
use ui::egui_renderer::EguiRenderer;
use winit::application::ApplicationHandler;
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::ControlFlow;
use winit::window::CursorGrabMode;
use winit::window::WindowAttributes;
use winit::window::WindowId;
use winit::{
    event::*, event_loop::EventLoop, keyboard::{KeyCode, PhysicalKey}, window::Window
};

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

const SIM_DT: f32 = 1.0/60.0;

struct AppState {
    delta_time: Instant,
    accumulator: f32,

    world: World,
    screen_server: ScreenServer,
    asset_server: AssetServer,
}

impl AppState {
    async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            #[cfg(not(target_arch="wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch="wasm32")]
            backends: wgpu::Backends::GL,
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone())
            .unwrap();

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await.unwrap();

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            required_features: wgpu::Features::empty(),
            #[cfg(not(target_arch="wasm32"))]
            required_limits: wgpu::Limits::default(),
            #[cfg(target_arch="wasm32")]
            required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
            memory_hints: wgpu::MemoryHints::Performance,
            label: None,
        }, None).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        let depth_texture = Texture::create_depth_texture(&device, &config, "depth_texture");

        let mut world = World::new();
        world.init_resource::<InputRes>();
        world.init_resource::<MouseRes>();

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
        let glyphon_renderer = GlyphonRenderer::new(&device, &queue);
        let egui_renderer = EguiRenderer::new(&device, window.as_ref());

        let asset_server = AssetServer::new();
        let screen_server = ScreenServer::new();

        world.insert_resource(
            DefaultPipeline::new(&device,
                &shader,
                &config
        ));

        world.insert_resource(RenderContext {
            window,
            config,
            size,
            device,
            queue,
            surface,
            depth_texture,
        });

        world.insert_resource(GuiContext {
            egui_renderer,
            glyphon_renderer,
        });

        let delta_time = Instant::now();
        let accumulator = 0.0;

        Self {
            screen_server,
            asset_server,
            world,
            delta_time,
            accumulator,
        }
    }
}

#[derive(Default)]
struct App {
    window: Option<Arc<Window>>,
    state: Option<AppState>,
}

impl ApplicationHandler for App {
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent
    ) {
        if self.window.as_ref().unwrap().id() != window_id {
            return;
        }

        match event {
            WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                event: KeyEvent {
                    state: ElementState::Pressed,
                    physical_key: PhysicalKey::Code(KeyCode::Escape),
                    ..
                },
                ..
            } => event_loop.exit(),
            WindowEvent::Resized(physical_size) => {
                self.resize(physical_size);
            },
            WindowEvent::RedrawRequested => {
                self.redraw_requested();
            },
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    state: key_state,
                    physical_key: PhysicalKey::Code(keycode),
                    ..
                },
                ..
            } => self.input(&keycode, &key_state),
            _ => {}
        }

        let world = self.state_mut()
            .world
            .borrow_mut();

        let window = world
            .resource_ref::<RenderContext>()
            .window.clone();

        let mut gui_ctx = world.
            resource_mut::<GuiContext>();

        gui_ctx.egui_renderer
            .window_event(&window, &event);
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: DeviceId,
        event: DeviceEvent) {
        match event {
            DeviceEvent::MouseMotion { delta } => self.mouse_moved(delta),
            _ => {}
        }
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let window = Arc::new(event_loop.create_window(WindowAttributes::default()).unwrap());
        window.set_cursor_grab(CursorGrabMode::Locked)
            .or_else(|_e| window.set_cursor_grab(CursorGrabMode::Confined))
            .unwrap();
        window.set_cursor_visible(false);

        self.window = Some(window.clone());

        #[cfg(not(target_arch = "wasm32"))]
        let state = pollster::block_on(AppState::new(window));

        #[cfg(target_arch = "wasm32")]
        let state = wasm_bindgen_futures::spawn_local(AppState::new(window));

        self.state = Some(state);
        self.run();
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        let window = self.window.as_ref().unwrap();
        window.request_redraw();
    }
}

impl App {
    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            let mut ctx = self.state_mut().world
                .get_resource_mut::<RenderContext>()
                .unwrap();

            ctx.size = new_size;
            ctx.config.width = new_size.width;
            ctx.config.height = new_size.height;
            ctx.depth_texture = Texture::create_depth_texture(&ctx.device, &ctx.config, "depth_texture");
            ctx.surface.configure(&ctx.device, &ctx.config);
        }
    }

    fn run(&mut self) {
        let state_mut = self.state_mut();
        let screen = MenuScreen::default();

        state_mut.screen_server
            .set_screen(&screen);
    }

    fn input(&mut self, keycode: &KeyCode, key_state: &ElementState) {
        let state = self.state_mut();
        let input_res = &mut state.world.get_resource_mut::<InputRes>()
            .unwrap();

        match keycode {
            KeyCode::KeyW => input_res.forward = KeyState::from(key_state),
            KeyCode::KeyA => input_res.left = KeyState::from(key_state),
            KeyCode::KeyS => input_res.backward = KeyState::from(key_state),
            KeyCode::KeyD => input_res.right = KeyState::from(key_state),
            _ => {},
        }
    }

    fn mouse_moved(&mut self, delta: (f64, f64)) {
        let mut mouse_res = self.state_mut().world
            .resource_mut::<MouseRes>();

        mouse_res.pos.0 += delta.0;
        mouse_res.pos.1 += delta.1;
    }

    fn redraw_requested(&mut self) {
        {
            let state = self.state_mut();
            state.accumulator += state.delta_time
                .elapsed()
                .as_secs_f32();
            state.delta_time = Instant::now();
        }

        while self.state_ref().accumulator >= SIM_DT {
            self.update();
            self.state_mut().accumulator -= SIM_DT;
        }

        self.draw();
    }

    fn update(&mut self) {
        let state_mut = &mut self.state_mut();
        let world = &mut state_mut.world;
        let screen_server = &mut state_mut.screen_server;

        screen_server.update(world);
    }

    fn draw(&mut self) {
        {
            let state = &mut self.state_mut();
            let world = &mut state.world;
            let render_ctx = world
                .resource::<RenderContext>();

            let frame_ctx = FrameContext::new(&render_ctx, None);
            world.insert_resource(frame_ctx);
        }

        {
            let state_mut = &mut self.state_mut();
            let world = &mut state_mut.world;
            let screen_server = &mut state_mut.screen_server;

            screen_server.draw(world);
        }

        {
            let world = &mut self.state_mut()
                .world;

            let frame_ctx = world
                .remove_resource::<FrameContext>()
                .unwrap();

            let render_ctx = world
                .resource::<RenderContext>();

            let buffers: Vec<wgpu::CommandBuffer> = frame_ctx
                .encoders
                .into_iter()
                .map(|encoder| {
                    encoder.finish()
                })
                .collect();

            render_ctx.queue.submit(buffers);
            frame_ctx.output.present();
        }
    }

    fn state_ref(&self) -> &AppState {
        self.state.as_ref().unwrap()
    }

    fn state_mut(&mut self) -> &mut AppState {
        self.state.as_mut().unwrap()
    }
}

pub fn run() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch="wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn)
                .expect("Could not initialize logger");
        } else {
            env_logger::init();
        }
    }

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    #[cfg(target_arch="wasm32")]
    {
        use winit::platform::web::WindowBuilderExtWebSys;
        let canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();

        builder = builder.with_canvas(Some(canvas));
    }

    let mut app = App::default();
    let _ = event_loop.run_app(&mut app);
}
