mod render;
mod camera;
mod util;
mod entity;
mod components;
mod resources;

use std::rc::Rc;
use std::sync::Arc;

use bevy_ecs::world::World;
use cgmath::Quaternion;
use glyphon::Resolution;
use render::cube::CubeModel;
use render::model::*;
use render::object::Object;
use render::text::LabelDescriptor;
use render::text::LabelId;
use render::text::LabelRenderer;
use render::texture::*;
use render::instance::*;

use camera::{ Camera, CameraController, CameraTransform };
use log::warn;
use resources::input::InputRes;
use resources::input::KeyState;
use resources::mouse::MouseRes;
use wgpu::PipelineCompilationOptions;
use wgpu::{util::DeviceExt, RenderPipelineDescriptor};
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

// GOOD MANNERS:
// Dont store references to struct in your own structs

struct AppState<'a> {
    depth_texture: Rc<Texture>,
    debug_texture: Rc<Texture>,

    camera: Camera,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,

    target_label: LabelId,
    camera_label: LabelId,
        
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,

    import_model: Rc<Model>,

    renderer: LabelRenderer<'a>,

    world: World,
}

impl<'a> AppState<'a> {
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

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("texture_bind_group_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ]
        });

        let camera = Camera::new(CameraTransform {
            position: (0.0, 1.0, 2.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            aspect: config.width as f32 / config.height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
            ..Default::default()
        }, CameraController {
            speed: 0.1,
        });

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera.uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("camera_bind_group_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
        });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("camera_bind_group"),
            layout: &camera_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                },
            ],
        });

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[
                &texture_bind_group_layout,
                &camera_bind_group_layout,
            ],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[
                    Vertex::desc(),
                    InstanceRaw::desc(),
                ],
                compilation_options: PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: PipelineCompilationOptions::default(),
            }),
            primitive : wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: Texture::DEPTH_TEXTURE_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false
            },
            multiview: None
        });

        surface.configure(&device, &config);

        let depth_texture = Texture::create_depth_texture(&device, &config, "depth_texture");
        let debug_texture = Texture::load("cube-diffuse.jpg", &device, &queue)
            .unwrap();

        let cube_model = Model::load("./res/untitled.obj", &device, &queue)
            .unwrap();

        let mut world = World::new();
        world.init_resource::<InputRes>();
        world.init_resource::<MouseRes>();

        let mut renderer = LabelRenderer::new(&device, &queue);

        let camera_label = renderer.add_label(LabelDescriptor {
            x: 0.0,
            y: 42.0,
            text: "".to_owned(),
            width: 1920.0,
            height: 1080.0,
            ..Default::default()
        });

        let target_label = renderer.add_label(LabelDescriptor {
            x: 0.0,
            y: 84.0,
            text: "".to_owned(),
            width: 1920.0,
            height: 1080.0,
            ..Default::default()
        });

        Self {
            target_label,
            camera_label,
            depth_texture,
            debug_texture,
            camera,
            camera_bind_group,
            camera_buffer,
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            import_model: cube_model,
            world,
            renderer,
        }
    }
}

#[derive(Default)]
struct App<'a> {
    window: Option<Arc<Window>>,
    state: Option<AppState<'a>>,
}

impl<'a> ApplicationHandler for App<'a> {
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent) {
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
                self.update();
                match self.draw() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => self.resize(self.state.as_ref().unwrap().size),
                    Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                    Err(e) => warn!("{:?}", e),
                }
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

impl<'a> App<'a> {
    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            let state = self.state.as_mut().unwrap();
            state.size = new_size;
            state.config.width = new_size.width;
            state.config.height = new_size.height;
            state.depth_texture = Texture::create_depth_texture(&state.device, &state.config, "depth_texture");
            state.surface.configure(&state.device, &state.config);
        }
    }

    fn input(&mut self, keycode: &KeyCode, key_state: &ElementState) {
        let state = self.state.as_mut().unwrap();
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
        let state = self.state.as_mut().unwrap();
        let mouse_res = &mut state.world.get_resource_mut::<MouseRes>()
            .unwrap();

        mouse_res.pos.0 += delta.0;
        mouse_res.pos.1 += delta.1;
    }

    fn update(&mut self) {
        let state = self.state.as_mut().unwrap();
        state.camera.update(&state.world);
        state.queue.write_buffer(&state.camera_buffer, 0, bytemuck::cast_slice(&[state.camera.uniform]));

        state.renderer.set_text(state.camera_label,
            format!("POS: {:?}", state.camera.transform.position));

        state.renderer.set_text(state.target_label,
            format!("TAR: {:?}", state.camera.transform.target));
    }

    fn draw(&mut self) -> Result<(), wgpu::SurfaceError> {
        let state = self.state.as_mut().unwrap();
        let output = state.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        state.renderer.viewport.update(&state.queue, Resolution{
            width: state.config.width,
            height: state.config.height,
        });
        state.renderer.prepare(&state.device, &state.queue);

        let object = Object::new(&state.device,
            CubeModel {
                scale: 1.0,
                diffuse_texture: state.debug_texture.clone() 
            }.to_model(&state.device),
            &[
                Instance {
                    position: (2.0, 2.0, 0.0).into(),
                    rotation: Quaternion::zero(),
                },
                Instance {
                    position: (1.0, 1.0, 0.0).into(),
                    rotation: Quaternion::zero(),
                },
                Instance {
                    position: (3.0, 2.0, 0.0).into(),
                    rotation: Quaternion::zero(),
                },
            ]);

        let mut encoder = state.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                // this is what @location(0) in the fragment shader targets
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),

                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &state.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&state.render_pipeline);
            render_pass.draw_object_instanced(&object, 0..3, &state.camera_bind_group);
            render_pass.draw_model(&state.import_model, &state.camera_bind_group);
        }

        {
            let mut glyphon_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Glyphon Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            state.renderer.draw(&mut glyphon_pass);
        }

        state.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        return Ok(());
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
