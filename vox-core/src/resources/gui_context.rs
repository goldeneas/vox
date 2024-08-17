use bevy_ecs::prelude::*;

use crate::ui::egui_renderer::EguiRenderer;

#[derive(Resource)]
pub struct GuiContext {
    pub egui_renderer: EguiRenderer,
}
