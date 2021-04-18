use bevy::prelude::*;
use bevy_input_actionmap::InputMap;
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};

use crate::control::Action;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(WorldInspectorParams {
            enabled: false,
            ..Default::default()
        })
        .add_system(toggle_inspector.system())
        .add_plugin(WorldInspectorPlugin::new());
    }
}

fn toggle_inspector(
    actions: Res<InputMap<Action>>,
    mut inspector_params: ResMut<WorldInspectorParams>,
) {
    if actions.just_active(Action::ToggleInspector) {
        inspector_params.enabled = !inspector_params.enabled;
    }
}
