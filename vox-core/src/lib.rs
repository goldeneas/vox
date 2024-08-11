mod render;
mod util;
mod entity;
mod components;
mod resources;
mod assets;

use std::sync::Arc;
use std::time::Instant;

use assets::asset_server::AssetServer;
use bevy_ecs::system::NonSend;
use bevy_ecs::system::Query;
use bevy_ecs::system::Res;
use bevy_ecs::world::World;
use cgmath::Matrix4;
use cgmath::Quaternion;
use components::camerable::CamerableComponent;
use components::model::ModelComponent;
use components::position::PositionComponent;
use components::rotation::RotationComponent;
use components::single_instance::SingleInstanceComponent;
use components::speed::SpeedComponent;
use glyphon::Resolution;
use render::model::*;
use render::text::LabelRenderer;
use render::texture::*;
use render::instance::*;

use log::warn;
use resources::glyphon_pass::GlyphonPass;
use resources::model_pass::ModelPass;
use resources::render_context::RenderContext;
use resources::input::InputRes;
use resources::input::KeyState;
use resources::mouse::MouseRes;
use wgpu::CommandEncoderDescriptor;
use winit::application::ApplicationHandler;
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::ControlFlow;
use winit::window::CursorGrabMode;
use winit::window::WindowAttributes;
use winit::window::WindowId;
use winit::{
    event::*, event_loop::EventLoop, keyboard::{KeyCode, PhysicalKey}, window::Window
};
use cgmath::prelude::*;

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

const SIM_DT: f32 = 1.0/60.0;

#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

struct AppState {
    delta_time: Instant,
    accumulator: f32,

    world: World,
}

impl AppState {
    async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();

        // wgpu instance used for surfaces and adapters
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            #[cfg(not(target_arch="wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch="wasm32")]
            backends: wgpu::Backends::GL,
            ..Default::default()
        });

        let surface = instance.create_surface(window)
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
            label: None,
        }, None).await.unwrap();

        let device = Arc::new(device);

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

        let renderer = LabelRenderer::new(&device, &queue);
        let asset_server = AssetServer::new();
        let depth_texture = Texture::create_depth_texture(&device, &config, "depth_texture");

        let mut world = World::new();
        world.init_resource::<InputRes>();
        world.init_resource::<MouseRes>();

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        world.insert_resource(
            ModelPass::new(&device,
                &shader,
                &config
        ));

        world.insert_resource(RenderContext {
            asset_server,
            renderer,
            queue,
            config,
            size,
            device: device.clone(),
            surface,
            depth_texture,
        });

        let delta_time = Instant::now();
        let accumulator = 0.0;

        Self {
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
                self.redraw_requested(event_loop);
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
        let state = pollster::block_on(AppState::new(window.clone()));

        #[cfg(target_arch = "wasm32")]
        let state = wasm_bindgen_futures::spawn_local(AppState::new(window.clone()));

        self.state = Some(state);
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
        let state = self.state_mut();
        let mouse_res = &mut state.world.get_resource_mut::<MouseRes>()
            .unwrap();

        mouse_res.pos.0 += delta.0;
        mouse_res.pos.1 += delta.1;
    }

    fn redraw_requested(&mut self, event_loop: &ActiveEventLoop) {
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

        match self.draw() {
            Ok(_) => {}
            Err(wgpu::SurfaceError::Lost) => {
                let ctx = self.state_ref().world
                    .get_resource_ref::<RenderContext>()
                    .unwrap();

                self.resize(ctx.size);
            }
            Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
            Err(e) => warn!("{:?}", e),
        }
    }

    fn update(&mut self) {
        let world = &mut self.state_mut().world;
        let ctx = world.get_resource_mut::<RenderContext>()
            .unwrap();
        
    }

    fn draw(&mut self) -> Result<(), wgpu::SurfaceError> {
        let mut ctx_res = self.state_mut().world
            .get_resource_mut::<RenderContext>()
            .unwrap();

        let ctx = ctx_res
            .as_mut();

        ctx.renderer.viewport.update(&ctx.queue, Resolution {
            width: ctx.config.width,
            height: ctx.config.height,
        });

        ctx.renderer.prepare(&ctx.device, &ctx.queue);

        // CALL RENDER METHODS HERE

        // TODO ALSO MOVE THIS!!!!
        //ctx.queue.submit(std::iter::once(ctx.encoder.as_ref().unwrap().finish()));
        //output.present();

        return Ok(());
    }

    fn draw_single_instance_models(mut query: Query<(
            &PositionComponent,
            &ModelComponent,
            &mut SingleInstanceComponent,
            Option<&RotationComponent>)>,
            ctx: NonSend<RenderContext>,
            model_pass: Res<ModelPass>,
    ) {
        let output = ctx.surface.get_current_texture().unwrap();
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = ctx.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Glyphon Label Encoder"),
        });

        for (position_cmpnt, model_cmpnt, mut instance_cmpnt, rotation_opt)
        in &mut query {
            let rotation = match rotation_opt {
                Some(rotation) => rotation.quaternion,
                None => Quaternion::zero(),
            };

            let position = position_cmpnt.position
                .to_vec();

            instance_cmpnt.set_instance(&InstanceData {
                position,
                rotation
            }, &ctx.device);

            let mut render_pass = model_pass.render_pass(&mut encoder,
                &view,
                &ctx.depth_texture.view()
            ).unwrap();

            render_pass.draw_model(&model_cmpnt.model,
                &model_pass.camera_bind_group()
            );
        }

        ctx.queue.submit(std::iter::once(&encoder.finish()));
        output.present();
    }

    fn draw_glyphon_labels(ctx: NonSend<RenderContext>,
        glyphon_pass: Res<GlyphonPass>) {
        let output = ctx.surface.get_current_texture().unwrap();
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = ctx.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Glyphon Label Encoder"),
        });
        
        let mut pass = glyphon_pass.render_pass(&mut encoder,
            &view,
        ).unwrap();

        ctx.renderer.draw(&mut pass);

        ctx.queue.submit(std::iter::once(&encoder.finish()));
        output.present();
    }

    fn draw_camera(query: Query<(
        &PositionComponent,
        &CamerableComponent)>,
        ctx: NonSend<RenderContext>,
        model_pass: Res<ModelPass>,
    ) {
        for (position_cmpnt, camerable_cmpnt) in &query {
            let view = Matrix4::look_at_rh(
                position_cmpnt.position,
                camerable_cmpnt.target,
                camerable_cmpnt.up
            );
            
            let proj = cgmath::perspective(
                cgmath::Deg(camerable_cmpnt.fovy),
                camerable_cmpnt.aspect,
                camerable_cmpnt.znear,
                camerable_cmpnt.zfar
            );
            
            let uniform: [[f32;4];4] = (OPENGL_TO_WGPU_MATRIX * proj * view)
                .into();
            
            ctx.queue.write_buffer(&model_pass.camera_buffer(),
                0, bytemuck::cast_slice(&uniform));
        }
    }

    fn update_camera(mut query: Query<(
        &mut PositionComponent,
        &SpeedComponent,
        &mut CamerableComponent)>,
        input_res: Res<InputRes>,
        mouse_res: Res<MouseRes>,
    ) {
        for (mut position_cmpnt, speed_cmpnt, mut camerable_cmpnt) in &mut query {
            let forward = camerable_cmpnt.target - position_cmpnt.position;
            let forward_norm = forward.normalize();
            let forward_mag = forward.magnitude();

            if input_res.forward.is_pressed && forward_mag > speed_cmpnt.speed {
                position_cmpnt.position += forward_norm * speed_cmpnt.speed;
                //camera_transform.target += forward_norm * self.speed;
            }

            if input_res.backward.is_pressed {
                position_cmpnt.position -= forward_norm * speed_cmpnt.speed;
                //camera_transform.target -= forward_norm * self.speed;
            }

            let up_norm = camerable_cmpnt.up.normalize();
            let right_norm = forward_norm.cross(up_norm);

            if input_res.right.is_pressed {
                position_cmpnt.position += right_norm * speed_cmpnt.speed; 
                //camera_transform.target += right_norm * self.speed;
            }

            if input_res.left.is_pressed {
                position_cmpnt.position -= right_norm * speed_cmpnt.speed;
                //camera_transform.target -= right_norm * self.speed;
            }

            let yaw: f32 = (mouse_res.pos.0 * 0.01) as f32;
            camerable_cmpnt.target.x = 2.23 * yaw.cos();
            camerable_cmpnt.target.z = 2.23 * yaw.sin();
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
