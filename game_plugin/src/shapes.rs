use crate::{ant::Ant, colony::Colony, food::Food};
use bevy_prototype_lyon::prelude::*;

use crate::GameState;
use bevy::prelude::*;

pub struct ShapesPlugin;

impl Plugin for ShapesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(ShapePlugin)
            .add_system_set(
                SystemSet::on_enter(GameState::Playing).with_system(spawn_camera.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(food_shapes.system())
                    .with_system(colony_shapes.system())
                    .with_system(ant_shapes.system()),
            );
    }
}
fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn food_shapes(
    mut query: Query<(Entity, &mut Transform, &Food, Option<&Handle<Mesh>>), Changed<Food>>,
    mut commands: Commands,
) {
    for (e, mut t, f, om) in query.iter_mut() {
        if om.is_some() {
            if f.0 <= 0.0 {
                continue;
            }
            t.scale = Vec3::ZERO.lerp(Vec3::ONE, f.0 / f.1);
        } else {
            let shape = shapes::RegularPolygon {
                sides: 6,
                feature: shapes::RegularPolygonFeature::Radius(f.0 / 10.0),
                ..shapes::RegularPolygon::default()
            };
            commands.entity(e).insert_bundle(GeometryBuilder::build_as(
                &shape,
                ShapeColors::outlined(Color::PINK, Color::AQUAMARINE),
                DrawMode::Outlined {
                    fill_options: FillOptions::default(),
                    outline_options: StrokeOptions::default().with_line_width(1.0),
                },
                *t,
            ));
        }
    }
}

fn colony_shapes(
    query: Query<(Entity, &Transform, Option<&Handle<Mesh>>), Changed<Colony>>,
    mut commands: Commands,
) {
    for (e, t, om) in query.iter() {
        if om.is_some() {
            // TODO: Some animation for colonies involving time?
        } else {
            let shape = shapes::RegularPolygon {
                sides: 12,
                feature: shapes::RegularPolygonFeature::Radius(30.0),
                ..shapes::RegularPolygon::default()
            };
            commands.entity(e).insert_bundle(GeometryBuilder::build_as(
                &shape,
                ShapeColors::outlined(Color::CYAN, Color::AQUAMARINE),
                DrawMode::Outlined {
                    fill_options: FillOptions::default(),
                    outline_options: StrokeOptions::default().with_line_width(1.0),
                },
                *t,
            ));
        }
    }
}

fn ant_shapes(
    query: Query<(Entity, &Transform, Option<&Handle<Mesh>>), Changed<Ant>>,
    mut commands: Commands,
) {
    for (e, t, om) in query.iter() {
        if om.is_some() {
            // TODO: Some animation for ants involving time?
        } else {
            let shape = shapes::Circle {
                radius: 5.0,
                center: Vec2::ZERO,
            };
            commands.entity(e).insert_bundle(GeometryBuilder::build_as(
                &shape,
                ShapeColors::outlined(Color::AZURE, Color::AQUAMARINE),
                DrawMode::Outlined {
                    fill_options: FillOptions::default(),
                    outline_options: StrokeOptions::default().with_line_width(1.0),
                },
                *t,
            ));
        }
    }
}
