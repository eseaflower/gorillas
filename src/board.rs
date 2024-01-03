use bevy::{math::ivec2, prelude::*};
use bevy_rapier2d::{
    dynamics::{ExternalImpulse, RigidBody},
    geometry::{Collider, ColliderMassProperties, Restitution},
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

fn handle_despawn(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &TextureAtlasSprite), (With<Despawn>, Without<RigidBody>)>,
    board_atlas: Res<BoardAtlas>,
) {
    for (entity, t, sprite) in query.iter() {
        let original_size = sprite.custom_size.expect("Sprite must have custom size");

        commands.entity(entity).despawn_recursive();

        let new_size = original_size / 2.0;
        // Split into 4 sprites
        for y in 0..2 {
            let delta_y = original_size.y * (y as f32 - 0.5);
            for x in 0..2 {
                let delta_x = original_size.x * (x as f32 - 0.5);
                let mut transform = t.clone();
                transform.translation += Vec3::new(delta_x, delta_y, 0.0);

                commands
                    .spawn((
                        SpriteBundle {
                            transform,
                            sprite: Sprite {
                                color: Color::rgb(0.0, 0.7, 0.0),
                                custom_size: Some(new_size),
                                ..Default::default()
                            },
                            ..default()
                        },
                        // SpriteSheetBundle {
                        //     texture_atlas: board_atlas.texture_atlas_handle.clone(),
                        //     sprite: TextureAtlasSprite::new(0),
                        //     transform: t.clone(),
                        //     ..default()
                        // },
                    ))
                    .insert(Collider::cuboid(new_size.x / 2.0, new_size.y / 2.0))
                    .insert(ColliderMassProperties::Density(0.1))
                    .insert(RigidBody::Dynamic);
            }
        }
    }
}

#[derive(Resource, Debug)]
struct BoardAtlas {
    texture_atlas_handle: Handle<TextureAtlas>,
}

fn spawn_board2(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("tiles.png");
    let brick_size = Vec2::new(16.0, 16.0);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, brick_size, 1, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.insert_resource(BoardAtlas {
        texture_atlas_handle: texture_atlas_handle.clone(),
    });

    for tpos in tower_iter(ivec2(0, 0), 3, 5) {
        commands
            .spawn((SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite {
                    index: 0,
                    custom_size: Some(brick_size),
                    ..Default::default()
                },
                transform: Transform::from_xyz(tpos.x as f32 * (16.0), tpos.y as f32 * (16.0), 0.0),
                ..default()
            },))
            .insert(Collider::cuboid(8.0, 8.0))
            .insert(ColliderMassProperties::Density(0.1));
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
            commands.entity(to_despawn).insert(Despawn);
        }
    }
}
