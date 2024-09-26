use bevy_ecs::world::{Mut, World};

use crate::resources::{egui_renderer::EguiRenderer, game_state::GameState, glyphon_renderer::GlyphonRenderer, render_context::RenderContext};

pub trait WorldExt {
    fn render_context(&self) -> &RenderContext;
    fn render_context_mut(&mut self) -> Mut<RenderContext>;
    fn egui_renderer(&self) -> &EguiRenderer;
    fn egui_renderer_mut(&mut self) -> Mut<EguiRenderer>;
    fn glyphon_renderer(&self) -> &GlyphonRenderer;
    fn glyphon_renderer_mut(&mut self) -> Mut<GlyphonRenderer>;
    fn game_state(&self) -> GameState;
}

impl WorldExt for World {
    fn render_context(&self) -> &RenderContext {
        self.resource::<RenderContext>()
    }

    fn render_context_mut(&mut self) -> Mut<RenderContext> {
        self.resource_mut::<RenderContext>()
    }

    fn egui_renderer(&self) -> &EguiRenderer {
        self.resource::<EguiRenderer>()
    }

    fn egui_renderer_mut(&mut self) -> Mut<EguiRenderer> {
        self.resource_mut::<EguiRenderer>()
    }

    fn glyphon_renderer(&self) -> &GlyphonRenderer {
        self.resource::<GlyphonRenderer>()
    }

    fn glyphon_renderer_mut(&mut self) -> Mut<GlyphonRenderer> {
        self.resource_mut::<GlyphonRenderer>()
    }

    fn game_state(&self) -> GameState {
        *self.resource::<GameState>() 
    }
}
