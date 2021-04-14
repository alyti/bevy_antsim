use bevy::prelude::*;
use flat_spatial::{grid::GridHandle, DenseGrid};

use crate::GameState;

pub struct GridPlugin;

#[derive(Debug, Clone)]
pub struct GridEntity(pub Option<GridHandle>);

impl Plugin for GridPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing).with_system(setup_grid.system()),
        )
        .add_system_to_stage(CoreStage::PostUpdate, maintain_grid.system());
    }
}

fn setup_grid(mut commands: Commands) {
    let grid: DenseGrid<Entity> = DenseGrid::new(64);
    commands.insert_resource(grid);
}

fn maintain_grid(
    grid: Option<ResMut<DenseGrid<Entity>>>,
    mut changed: Query<(Entity, &Transform, &mut GridEntity), Changed<Transform>>,
    removed: RemovedComponents<GridEntity>,
) {
    if let Some(mut g) = grid {
        for (e, t, mut ge) in changed.iter_mut() {
            if let Some(handle) = ge.0 {
                g.set_position(handle, grid_pos(t))
            } else {
                let handle = g.insert(grid_pos(t), e.clone());
                ge.0 = Some(handle);
            }
        }

        // TODO: Make the reverse-lookup better ??
        for entity in removed.iter() {
            let related: Vec<GridHandle> = g
                .handles()
                .map(|h| (g.get(h).unwrap(), h))
                .filter(|((_, e), _)| e == &&entity)
                .map(|(_, h)| h)
                .collect();

            info!("Removing entity {:?} grid handle {:?}", entity, related);
            for r in related {
                g.remove(r);
            }
        }

        g.maintain()
    }
}

pub fn grid_pos(t: &Transform) -> [f32; 2] {
    [t.translation.x, t.translation.y]
}
