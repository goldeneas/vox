use winit::{
    dpi::PhysicalSize, event::*, event_loop::EventLoop, keyboard::{KeyCode, PhysicalKey}, window::{Window, WindowBuilder}
};

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

struct State<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,

    // unsafe reference to window
    window: &'a Window
}

impl<'a> State<'a> {
    // async needed for some wgpu code
    async fn new(window: &'a Window) -> State<'a> {
        let size = window.inner_size();

        // wgpu instance used for surfaces and adapters
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
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
            required_limits: wgpu::Limits::default(),
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

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size
        }
    }

    pub fn window(&self) -> &Window {
        return &self.window;
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        todo!();
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        todo!();
    }

    fn update(&mut self) {
        todo!();
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        todo!();
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub async fn run() {
    // set up logging
    // let javascript console print messages if running on web
    cfg_if::cfg_if! {
        if #[cfg(target_arch="wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn)
                .expect("Could not initialize logger")
        } else {
            env_logger::init();
        }
    }

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let state = State::new(&window).await;

    event_loop.run(move |event, event_loop_target| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == state.window.id() => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        ..
                    },
                ..
            } => event_loop_target.exit(),
            _ => {}
        },
        _ => {}
    }).unwrap();

    #[cfg(target_arch="wasm32")]
    {
        // winit requires us to setup window's size when on web
        use winit::platform::web::WindowExtWebSys;
        let _ = window.request_inner_size(PhysicalSize::new(450, 400));
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("wasm-example")?;
                let canvas = web_sys::Element::from(window.canvas()?);
                dst.append_child(&canvas).ok()?;
                Some(())
            })
        .expect("Could not append canvas to document body");
    }
}

fn main() {
    pollster::block_on(run());
}

 
