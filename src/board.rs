use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_rapier2d::{
    geometry::{Collider, Restitution},
    pipeline::CollisionEvent,
};

use crate::shot::Shot;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin)
            .add_systems(Startup, spawn_board2)
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

    for tpos in spawn_tower(TilePos { x: 0, y: 0 }, 3, 5) {
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
            Restitution::coefficient(2.0),
        ));
    }
}

fn spawn_board(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_size = TilemapSize { x: 10, y: 10 };

    let tilemap_entity = commands.spawn_empty().id();

    let mut tile_storage = TileStorage::empty(map_size);

    for tpos in spawn_tower(TilePos { x: 0, y: 0 }, 3, 5) {
        let tile_entity = commands
            .spawn(TileBundle {
                position: tpos,
                tilemap_id: TilemapId(tilemap_entity),
                ..Default::default()
            })
            .insert(Collider::cuboid(8.0, 8.0))
            .id();

        tile_storage.set(&tpos, tile_entity);
    }

    // for y in 0..map_size.y {
    //     for x in 0..map_size.x {
    //         let tile_position = TilePos { x, y };
    //         let tile_entity = commands
    //             .spawn(TileBundle {
    //                 position: tile_position,
    //                 tilemap_id: TilemapId(tilemap_entity),
    //                 ..Default::default()
    //             })
    //             .id();

    //         tile_storage.set(&tile_position, tile_entity);
    //     }
    // }

    let texture_handle = asset_server.load("tiles.png");
    // Create the tilemap bundle
    let tile_size = TilemapTileSize::new(16.0, 16.0);
    let grid_size = tile_size.into();
    let map_type = TilemapType::Square;

    // let center_transform = get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0);

    dbg!(tilemap_entity);

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: Transform::from_xyz(0.0, -250.0 + 16.0 / 2.0, 0.0),
        ..Default::default()
    });
    // .with_children(|c| {
    //     let board_colider = c
    //         .spawn((
    //             Collider::cuboid(
    //                 center_transform.translation.x.abs() + tile_size.x / 2.0,
    //                 center_transform.translation.y.abs() + tile_size.y / 2.0,
    //             ),
    //             TransformBundle::from(Transform::from_xyz(
    //                 -center_transform.translation.x,
    //                 -center_transform.translation.y,
    //                 center_transform.translation.z,
    //             )),
    //         ))
    //         .id();
    //     dbg!(board_colider);
    // });
}

fn spawn_tower(position: TilePos, width: u32, height: u32) -> impl Iterator<Item = TilePos> {
    (0..height)
        .map(move |y| {
            (0..width).map(move |x| TilePos {
                x: position.x + x,
                y: position.y + y,
            })
        })
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
