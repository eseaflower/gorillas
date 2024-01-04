mod board;
mod shot;
mod ldtk;

use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use board::BoardPlugin;
use shot::ShotPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(10.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(BoardPlugin)
        .add_plugins(ShotPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, despawn_far_away)
        .run();
}

fn setup(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(500.0);

    commands.spawn(camera_bundle);
}

fn despawn_far_away(
    mut commands: Commands,
    shot_quuery: Query<(Entity, &Transform), Without<Camera>>,
    camera_query: Query<&Transform, With<Camera>>,
) {
    for (e, transform) in shot_quuery.iter() {
        // Shot is 100 units away from all cameras.
        let do_despawn = camera_query.iter().all(|c| {
            let distance = c.translation - transform.translation;
            distance.length() > 1000.0
        });
        if do_despawn {
            eprintln!("Despawning entity {:?} due to distance", e);
            commands.entity(e).despawn_recursive();
        }
    }
}
