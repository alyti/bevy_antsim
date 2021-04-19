use bevy::prelude::*;

use crate::GameState;

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing).with_system(setup_system.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(debug_food_despawner.system())
                .with_system(food_spawner_system.system()),
        );
    }
}

pub struct RandomisePosition([f32; 4]);

#[derive(Bundle, Clone)]
pub struct FoodBundle {
    pub grid_entity: crate::grid::GridEntity,
    pub food: Food,
    pub timer: Timer,
}

#[derive(Clone)]
pub struct Food(pub f32, pub f32);

impl Default for FoodBundle {
    fn default() -> Self {
        Self {
            grid_entity: crate::grid::GridEntity(None),
            food: Food(1000.0, 1000.0),
            timer: Timer::from_seconds(0.01, true),
        }
    }
}

#[derive(Bundle, Clone)]
pub struct FoodSpawnerBundle {
    pub timer: Timer,
    pub food: FoodBundle,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for FoodSpawnerBundle {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.01, true),
            food: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
        }
    }
}

fn setup_system(mut commands: Commands, windows: Res<Windows>) {
    let win = windows.get_primary().unwrap();
    let (h, w) = (win.height(), win.width());
    let (off_h, off_w) = (h / 2.0 - h, w / 2.0 - w);

    commands
        .spawn()
        .insert_bundle(FoodSpawnerBundle::default())
        .insert(RandomisePosition([off_w, off_h, w, h]));
}

fn food_spawner_system(
    time: Res<Time>,
    mut query: Query<(
        &mut Timer,
        &FoodBundle,
        &Transform,
        Option<&RandomisePosition>,
    )>,
    mut commands: Commands,
) {
    for (mut timer, fb, t, orp) in query.iter_mut() {
        if timer.tick(time.delta()).just_finished() {
            let pos = {
                if let Some(rp) = orp {
                    (
                        rp.0[0] + rp.0[2] * rand::random::<f32>(),
                        rp.0[1] + rp.0[3] * rand::random::<f32>(),
                    )
                } else {
                    (0.0, 0.0)
                }
            };

            let mut nt = t.clone();
            nt.translation.x += pos.0;
            nt.translation.y += pos.1;
            // info!("Spawning food at {:?}", nt.translation);

            commands.spawn().insert_bundle(fb.clone()).insert(nt);
        }
    }
}

fn debug_food_despawner(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Timer, &mut Food)>,
) {
    for (entity, mut timer, mut food) in query.iter_mut() {
        if timer.tick(time.delta()).just_finished() {
            if food.0 == 0.0 {
                // info!("Despawning {:?}", entity);
                commands.entity(entity).despawn();
            } else {
                food.0 -= 10.0;
            }
        }
    }
}
