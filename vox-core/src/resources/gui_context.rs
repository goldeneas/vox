use bevy_ecs::prelude::*;

use crate::ui::{egui_renderer::EguiRenderer, glyphon_renderer::GlyphonRenderer};

#[derive(Resource)]
pub struct GuiContext {
    pub egui_renderer: EguiRenderer,
    pub glyphon_renderer: GlyphonRenderer<'static>,
}
