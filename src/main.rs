mod board;
mod ldtk;
mod player;

use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use board::BoardPlugin;
use ldtk::{LdtkAsset, LdtkAssetLoader};
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(10.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(BoardPlugin)
        .add_plugins(PlayerPlugin)
        .init_asset::<LdtkAsset>()
        .init_asset_loader::<LdtkAssetLoader>()
        .add_systems(Startup, setup)
        .add_systems(Update, despawn_far_away)
        .run();
}

#[derive(Resource, Default)]
pub struct Board {
    map: Handle<LdtkAsset>,
    loaded: bool,
}

#[derive(Resource, Default)]
pub struct Players {
    map: Handle<LdtkAsset>,
    loaded: bool,
    names: Vec<String>,
    current_player: Option<usize>,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(256.0);
    commands.spawn(camera_bundle);

    // Load the map
    let map_handle = asset_server.load("test.ldtk");
    commands.insert_resource(Board {
        map: map_handle.clone(),
        loaded: false,
    });
    commands.insert_resource(Players {
        map: map_handle.clone(),
        loaded: false,
        names: Vec::new(),
        current_player: None,
    });
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
