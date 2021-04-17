mod ant;
mod colony;
mod control;
mod food;
mod grid;
mod loading;
mod menu;
mod shapes;

use crate::{
    ant::AntPlugin, colony::ColonyPlugin, control::ControlPlugin, food::FoodPlugin,
    grid::GridPlugin, loading::LoadingPlugin, menu::MenuPlugin, shapes::ShapesPlugin,
};
use bevy::{app::AppBuilder, prelude::*};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Loading,
    Playing,
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(GameState::Loading)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(ControlPlugin)
            .add_plugin(ShapesPlugin)
            .add_plugin(GridPlugin)
            .add_plugin(FoodPlugin)
            .add_plugin(ColonyPlugin)
            .add_plugin(AntPlugin);
    }
}
