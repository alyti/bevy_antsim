use bevy::prelude::*;
use bevy_input_actionmap::InputMap;

use crate::{control::Action, GameState};

pub struct PausedPlugin;

impl Plugin for PausedPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<PauseCooldown>()
            .add_system(toggle_pause_state.system())
            .add_system_set(SystemSet::on_enter(GameState::Paused).with_system(setup_ui.system()))
            .add_system_set(SystemSet::on_exit(GameState::Paused).with_system(despawn_ui.system()));
    }
}

struct PauseCooldown(Timer);

impl Default for PauseCooldown {
    fn default() -> Self {
        Self(Timer::from_seconds(0.1, false))
    }
}

fn toggle_pause_state(time: Res<Time>, mut cd: ResMut<PauseCooldown>, action: Res<InputMap<Action>>, mut state: ResMut<State<GameState>>) {
    if !cd.0.tick(time.delta()).finished() {
        return;
    }
    if !action.just_active(Action::TogglePause) {
        return;
    }
    match state.current() {
        GameState::Paused => state.pop().unwrap(),
        GameState::Playing => state.push(GameState::Paused).unwrap(),
        _default => {}
    };
    cd.0.reset();
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: Rect {
                bottom: Val::Px(5.0),
                right: Val::Px(15.0),
                ..Default::default()
            },
            ..Default::default()
        },
        // Use the `Text::with_section` constructor
        text: Text::with_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "Paused",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 36.0,
                color: Color::WHITE,
            },
            // Note: You can use `Default::default()` in place of the `TextAlignment`
            TextAlignment {
                horizontal: HorizontalAlign::Center,
                ..Default::default()
            },
        ),
        ..Default::default()
    });
}

fn despawn_ui(mut commands: Commands, text_query: Query<Entity, With<Text>>) {
    for entity in text_query.iter() {
        commands.entity(entity).despawn();
    }
}
