use bevy::prelude::*;
use crate::GameState;

pub struct AntPlugin;

impl Plugin for AntPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                // .with_system(debug_food_despawner.system())
                .with_system(debug_ant_motion.system()),
        );
    }
}

#[derive(Clone)]
pub struct Ant {
    pub holding_food: bool,
    pub velocity: Vec2,
}

impl Default for Ant {
    fn default() -> Self {
        Self {
            holding_food: false,
            velocity: Vec2::new(50.0, 0.0),
        }
    }
}

#[derive(Bundle, Clone, Default)]
pub struct AntBundle {
    pub ant: Ant,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

fn debug_ant_motion(time: Res<Time>, mut query: Query<(&mut Transform, &Ant)>) {
    for (mut t, a) in query.iter_mut() {
        t.translation = t.mul_vec3((a.velocity * time.delta_seconds()).extend(0.0));
    }
}
