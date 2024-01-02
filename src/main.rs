mod board;
mod shot;

use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_rapier2d::prelude::*;
use board::BoardPlugin;
use shot::ShotPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(BoardPlugin)
        .add_plugins(ShotPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(500.0);

    commands.spawn(camera_bundle);
}
