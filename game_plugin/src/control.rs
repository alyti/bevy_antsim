use crate::shapes::WorldCamera;
use bevy::{
    input::{
        mouse::{MouseMotion, MouseWheel},
        system::exit_on_esc_system,
    },
    prelude::*,
    render::camera::{Camera, OrthographicProjection},
};
use bevy_egui::EguiContext;
use bevy_input_actionmap::{ActionPlugin, InputMap};

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut AppBuilder) {
        // TODO: Maybe move pause system to Playing / Paused system_sets
        app.init_resource::<ActiveTool>()
            .add_plugin(ActionPlugin::<Action>::default())
            .add_startup_system(setup_actions.system())
            .add_system(exit_on_esc_system.system())
            .add_system(manipulate_world_camera.system());
    }
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum Action {
    ToggleInspector,
    TogglePause,
}

fn setup_actions(mut input: ResMut<InputMap<Action>>) {
    input
        .bind(Action::ToggleInspector, KeyCode::F3)
        .bind(Action::TogglePause, KeyCode::Space);
}

#[derive(PartialEq)]
enum Tool {
    PanAndZoom,
}

impl Default for Tool {
    fn default() -> Self {
        Tool::PanAndZoom
    }
}

#[derive(Default)]
struct ActiveTool(Tool);

fn manipulate_world_camera(
    active_tool: Res<ActiveTool>,
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<Input<MouseButton>>,
    egui: Res<EguiContext>,
    windows: Res<Windows>,
    mut query: Query<(&mut Transform, &Camera), (With<OrthographicProjection>, With<WorldCamera>)>,
) {
    if active_tool.0 != Tool::PanAndZoom {
        return;
    }

    let mut scroll = 0.0;
    let mut pan = Vec2::ZERO;

    for ev in ev_scroll.iter() {
        scroll += ev.y;
    }

    if input_mouse.pressed(MouseButton::Left) {
        for ev in ev_motion.iter() {
            pan += ev.delta;
        }
    }

    for (mut transform, cam) in query.iter_mut() {
        let window = windows.get(cam.window).unwrap();
        if !window.is_focused() {
            continue;
        }

        if egui.ctx_for_window(cam.window).wants_pointer_input() {
            break;
        }

        if pan.length_squared() > 0.0 {
            let scale = Vec2::new(transform.scale.x, transform.scale.y);
            transform.translation += (scale * pan * Vec2::new(-1.0, 1.0)).extend(0.0);
        }

        if scroll.abs() > 0.0 {
            transform.scale += Vec3::new(scroll * 0.1, scroll * 0.1, 0.0);
            transform.scale = transform.scale.max(Vec3::new(0.05, 0.05, 0.0));
        }
    }
}
