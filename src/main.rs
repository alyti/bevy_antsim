// disable console opening on windows
#![windows_subsystem = "windows"]

#[cfg(target_arch = "wasm32")]
use bevy_webgl2;

use bevy::prelude::{App, ClearColor, Color, WindowDescriptor};
use bevy::DefaultPlugins;
use game_plugin::GamePlugins;

fn main() {
    let mut app = App::build();
    app
        // .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            mode: bevy::window::WindowMode::BorderlessFullscreen,
            title: "Ant Hive Simulation (just hexagons being bestagons for now)".to_string(),
            ..Default::default()
        })
        .add_state(game_plugin::GameState::Loading)
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugins);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.run();
}
