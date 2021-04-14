mod paths;

use crate::loading::paths::PATHS;
use crate::GameState;
use bevy::asset::LoadState;
use bevy::prelude::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Loading).with_system(start_loading.system()),
        )
        .add_system_set(SystemSet::on_update(GameState::Loading).with_system(check_state.system()));
    }
}

pub struct LoadingState {
    textures: Vec<HandleUntyped>,
    fonts: Vec<HandleUntyped>,
}

pub struct FontAssets {
    pub fira_sans: Handle<Font>,
}

pub struct TextureAssets {
    pub texture_bevy: Handle<Texture>,
}

fn start_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut fonts: Vec<HandleUntyped> = vec![];
    fonts.push(asset_server.load_untyped(PATHS.fira_sans));

    let mut textures: Vec<HandleUntyped> = vec![];
    textures.push(asset_server.load_untyped(PATHS.texture_bevy));

    commands.insert_resource(LoadingState {
        textures,
        fonts,
    });
}

fn check_state(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    asset_server: Res<AssetServer>,
    loading_state: Res<LoadingState>,
) {
    if LoadState::Loaded
        != asset_server.get_group_load_state(loading_state.fonts.iter().map(|handle| handle.id))
    {
        return;
    }
    if LoadState::Loaded
        != asset_server.get_group_load_state(loading_state.textures.iter().map(|handle| handle.id))
    {
        return;
    }

    commands.insert_resource(FontAssets {
        fira_sans: asset_server.get_handle(PATHS.fira_sans),
    });

    commands.insert_resource(TextureAssets {
        texture_bevy: asset_server.get_handle(PATHS.texture_bevy),
    });

    state.set(GameState::Menu).unwrap();
}
