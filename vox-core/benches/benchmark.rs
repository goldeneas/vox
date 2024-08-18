use std::{rc::Rc, sync::Arc};

use bevy_ecs::system::{Query, Res, ResMut};
use criterion::{criterion_group, criterion_main, Criterion};
use vox_core::App;
use wgpu::CommandEncoderDescriptor;
use winit::{event_loop::{ControlFlow, EventLoop}, window::{CursorGrabMode, WindowAttributes}};

pub fn start(c: &mut Criterion) {
    env_logger::init();
    let event_loop = Box::new(EventLoop::new().unwrap());
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    app.set_timeout(1.0);
    c.bench_function("run 10", |b| {
        b.iter(|| {
            let _ = event_loop.run_app(&mut app);
        })
    });
}

criterion_group!(benches, start);
criterion_main!(benches);
