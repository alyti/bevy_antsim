use crate::GameState;
use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    render::camera::OrthographicProjection,
};

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ActiveTool>().add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(pan_camera.system())
                .with_system(zoom_camera.system()),
        );
    }
}

#[derive(PartialEq)]
enum Tool {
    Pan,
}

impl Default for Tool {
    fn default() -> Self {
        Tool::Pan
    }
}

#[derive(Default)]
struct ActiveTool(Tool);

fn zoom_camera(
    mut ev_scroll: EventReader<MouseWheel>,
    mut query: Query<&mut Transform, With<OrthographicProjection>>,
) {
    let mut scroll = 0.0;
    for ev in ev_scroll.iter() {
        scroll += ev.y;
    }
    for mut transform in query.iter_mut() {
        if scroll.abs() > 0.0 {
            transform.scale += Vec3::new(scroll * 0.2, scroll * 0.2, 0.0);
            transform.scale = transform.scale.max(Vec3::new(0.05, 0.05, 0.0));
        }
    }
}

fn pan_camera(
    active_tool: Res<ActiveTool>,
    mut ev_motion: EventReader<MouseMotion>,
    input_mouse: Res<Input<MouseButton>>,
    mut query: Query<&mut Transform, With<OrthographicProjection>>,
) {
    if active_tool.0 != Tool::Pan {
        return;
    }

    let mut pan = Vec2::ZERO;

    if input_mouse.pressed(MouseButton::Left) {
        for ev in ev_motion.iter() {
            pan += ev.delta;
        }
    }

    for mut transform in query.iter_mut() {
        if pan.length_squared() > 0.0 {
            let scale = Vec2::new(transform.scale.x, transform.scale.y);
            transform.translation += scale.extend(0.0) * (pan * Vec2::new(-1.0, 1.0)).extend(0.0);
        }
    }
}
