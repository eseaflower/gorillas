use bevy::{math::ivec2, prelude::*};
use bevy_rapier2d::{
    geometry::{Collider, Restitution},
    pipeline::CollisionEvent,
};

use crate::shot::Shot;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_board2)
            .add_systems(Update, read_colisions)
            .add_systems(Update, handle_despawn);
    }
}

#[derive(Component, Debug)]
struct Despawn;

fn handle_despawn(mut commands: Commands, query: Query<(Entity, &Despawn)>) {
    for (entity, _) in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn spawn_board2(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("tiles.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 1, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    for tpos in tower_iter(ivec2(0, 0), 3, 5) {
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(0),
                transform: Transform::from_xyz(
                    tpos.x as f32 * (16.0) + 100.0,
                    tpos.y as f32 * (16.0) - 250.0,
                    0.0,
                ),
                ..default()
            },
            Collider::cuboid(8.0, 8.0),
            // Restitution::coefficient(2.0),
        ));
    }
}

fn tower_iter(position: IVec2, width: u32, height: u32) -> impl Iterator<Item = IVec2> {
    (0..height)
        .map(move |y| (0..width).map(move |x| ivec2(position.x + x as i32, position.y + y as i32)))
        .flatten()
}

fn read_colisions(
    mut commands: Commands,
    mut reader: EventReader<CollisionEvent>,
    query: Query<&Shot>,
) {
    for event in reader.read() {
        println!("Collision started: {:?}", event);
        if let CollisionEvent::Started(collider1, collider2, _) = event {
            let to_despawn = match query.get(*collider1) {
                Ok(_) => *collider2,
                Err(_) => *collider1,
            };
            // commands.entity(to_despawn).insert(Despawn);
        }
    }
}
