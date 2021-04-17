mod loading;
mod menu;
mod shapes;
mod food;
mod grid;
mod control;

use crate::loading::LoadingPlugin;
use crate::shapes::ShapesPlugin;
use crate::menu::MenuPlugin;
use crate::control::ControlPlugin;
use crate::grid::GridPlugin;
use crate::food::FoodPlugin;

use bevy::app::AppBuilder;
use bevy::prelude::*;

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
            .add_plugin(ShapesPlugin)
            .add_plugin(ControlPlugin)
            .add_plugin(GridPlugin)
            .add_plugin(FoodPlugin)
            ;
    }
}
