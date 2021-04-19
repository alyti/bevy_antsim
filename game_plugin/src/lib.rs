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
use bevy::{app::PluginGroupBuilder, prelude::*};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Loading,
    Menu,
    Playing,
    Paused,
}

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(bevy_prototype_lyon::plugin::ShapePlugin)
            .add(DebugPlugin)
            .add(LoadingPlugin)
            .add(MenuPlugin)
            .add(ControlPlugin)
            .add(PausedPlugin)
            .add(ShapesPlugin)
            .add(GridPlugin)
            .add(FoodPlugin)
            .add(ColonyPlugin)
            .add(AntPlugin);
    }
}
