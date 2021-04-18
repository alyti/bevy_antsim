use bevy::prelude::*;

use crate::{ant::AntBundle, GameState};

pub struct ColonyPlugin;

impl Plugin for ColonyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing).with_system(setup_system.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                // .with_system(debug_food_despawner.system())
                .with_system(ant_spawner_system.system()),
        );
    }
}

#[derive(Clone)]
pub struct Colony {
    pub max_range: f32,
    pub max_population: u32,
    pub cur_population: u32,
    pub ants_per_second: f64,
}

impl Default for Colony {
    fn default() -> Self {
        Self {
            max_range: 300.0,
            max_population: 500,
            cur_population: 0,
            ants_per_second: 10.0,
        }
    }
}

#[derive(Bundle, Clone)]
pub struct ColonyBundle {
    pub colony: Colony,
    pub timer: Timer,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub ant: AntBundle,
}

impl Default for ColonyBundle {
    fn default() -> Self {
        Self {
            colony: Default::default(),
            timer: Timer::from_seconds(0.25, true),
            ant: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
        }
    }
}

fn setup_system(mut commands: Commands) {
    commands.spawn().insert_bundle(ColonyBundle::default());
}

fn ant_spawner_system(
    time: Res<Time>,
    mut query: Query<(Entity, &mut Colony, &AntBundle, &Transform)>,
    mut commands: Commands,
) {
    for (entity, mut colony, ant_bundle, t) in query.iter_mut() {
        if colony.cur_population >= colony.max_population {
            continue;
        }
        // TODO: Improve this cringe
        let to_spawn = (colony.ants_per_second / time.delta_seconds_f64())
            .ceil()
            .max(colony.max_population as f64) as u32;
        for _ in 0..to_spawn {
            let mut nt = t.clone();
            let angle = rand::random::<f32>();
            nt.rotation = Quat::from_rotation_z(angle * 360.0);
            commands
                .spawn()
                .insert_bundle(ant_bundle.clone())
                .insert(nt)
                .insert(Parent(entity));
        }
        colony.cur_population += to_spawn;
    }
}
