mod ant;
mod colony;
mod control;
mod debug;
mod food;
mod grid;
mod loading;
mod menu;
mod paused;
mod shapes;

use crate::{
    ant::AntPlugin, colony::ColonyPlugin, control::ControlPlugin, debug::DebugPlugin,
    food::FoodPlugin, grid::GridPlugin, loading::LoadingPlugin, menu::MenuPlugin,
    paused::PausedPlugin, shapes::ShapesPlugin,
};
use bevy::{app::AppBuilder, prelude::*};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Loading,
    Menu,
    Playing,
    Paused,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(GameState::Loading)
            .add_plugin(DebugPlugin)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(ControlPlugin)
            .add_plugin(PausedPlugin)
            .add_plugin(ShapesPlugin)
            .add_plugin(GridPlugin)
            .add_plugin(FoodPlugin)
            .add_plugin(ColonyPlugin)
            .add_plugin(AntPlugin);
    }
}
