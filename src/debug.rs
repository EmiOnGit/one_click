use bevy::{
    input::common_conditions::input_toggle_active,
    prelude::{KeyCode, Plugin},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Tab)),
        );
    }
}
