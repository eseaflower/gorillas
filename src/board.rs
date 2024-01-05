use bevy::prelude::*;
use bevy_rapier2d::{dynamics::RigidBody, geometry::Collider, pipeline::CollisionEvent};

use crate::{
    ldtk::{self, convert_coords, LdtkAsset},
    player::Shot,
    Board,
};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FractureEvent>()
            .add_event::<DamageEvent>()
            .add_systems(Update, spawn_board)
            .add_systems(Update, read_colisions)
            .add_systems(Update, handle_fracture)
            .add_systems(Update, handle_health);
    }
}

fn spawn_board(mut commands: Commands, mut board: ResMut<Board>, maps: Res<Assets<LdtkAsset>>) {
    // Board is already loaded or the map is not loaded yet
    if board.loaded || maps.get(&board.map).is_none() {
        return;
    }

    eprintln!("Spawning board");

    let map = maps.get(&board.map).expect("Failed to load map");
    let project = &map.project;

    // Assume we use the first level
    let level = &project.levels[0];
    let level_width = level.px_wid as f32;
    let level_height = level.px_hei as f32;
    let level_size = Vec2::new(level_width, level_height);
    // Use the first tile layer
    let layers = level.layer_instances.as_ref().expect("No layers");
    for layer in layers {
        spawn_layer(&mut commands, level_size, layer);
    }

    board.loaded = true;
}

fn spawn_layer(commands: &mut Commands, level_size: Vec2, layer: &ldtk::LayerInstance) {
    let grid_size = layer.grid_size as f32;
    let brick_size = Vec2::new(grid_size, grid_size);

    // Spawn tiles
    for tile in &layer.grid_tiles {
        let position = convert_coords(&tile.px, grid_size, level_size);
        spawn_brick(commands, position, brick_size);
    }
}

#[derive(Component, Debug)]
struct BoardBrick {
    health: f32,
}
impl Default for BoardBrick {
    fn default() -> Self {
        Self { health: 100.0 }
    }
}

fn spawn_brick(commands: &mut Commands, position: Vec2, size: Vec2) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 1.0, 0.0),
                custom_size: Some(size),
                ..Default::default()
            },
            transform: Transform::from_xyz(position.x, position.y, 0.0),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(size.x / 2.0, size.y / 2.0))
        .insert(BoardBrick::default());
}

#[derive(Event, Debug)]
struct DamageEvent {
    entity: Entity,
    damage: f32,
}

fn read_colisions(
    mut reader: EventReader<CollisionEvent>,
    mut damage_event_writer: EventWriter<DamageEvent>,
    shot_quey: Query<&Shot>,
    board_query: Query<&BoardBrick>,
) {
    for event in reader.read() {
        println!("Collision started: {:?}", event);
        if let CollisionEvent::Started(collider1, collider2, _) = event {
            let damage = 50.0f32;

            if let (Ok(_), Ok(_)) = (shot_quey.get(*collider1), board_query.get(*collider2)) {
                damage_event_writer.send(DamageEvent {
                    entity: *collider2,
                    damage,
                });
            } else {
                if let (Ok(_), Ok(_)) = (shot_quey.get(*collider2), board_query.get(*collider1)) {
                    damage_event_writer.send(DamageEvent {
                        entity: *collider1,
                        damage,
                    });
                }
            };
        }
    }
}

fn handle_health(
    mut brick_query: Query<(Entity, &mut BoardBrick, &mut Sprite)>,
    mut fracture_event_writer: EventWriter<FractureEvent>,
    mut damage_event_reader: EventReader<DamageEvent>,
) {
    for event in damage_event_reader.read() {
        if let Ok((entity, mut brick, mut sprite)) = brick_query.get_mut(event.entity) {
            brick.health -= event.damage;
            if brick.health <= 0.0 {
                fracture_event_writer.send(FractureEvent(entity));
            } else {
                let color_vec = Vec3::new(0.0, 1.0, 0.0)
                    .lerp(Vec3::new(1.0, 0.0, 0.0), 1.0 - brick.health / 100.0);
                sprite.color = Color::rgb(color_vec.x, color_vec.y, color_vec.z);
            }
        }
    }
}

#[derive(Event, Debug)]
struct FractureEvent(Entity);

fn handle_fracture(
    mut commands: Commands,
    mut fracture_event_reader: EventReader<FractureEvent>,
    query: Query<(&Transform, &Sprite)>,
) {
    // Read the events
    for event in fracture_event_reader.read() {
        let entity = event.0;
        if let Ok((t, sprite)) = query.get(entity) {
            let original_size = sprite.custom_size.expect("Sprite must have custom size");

            commands.entity(entity).despawn_recursive();

            let new_size = original_size / 2.0;
            // Split into 4 sprites
            for y in 0..2 {
                let delta_y = (new_size.y + 3.0) * (y as f32 - 0.5);
                for x in 0..2 {
                    let delta_x = (new_size.x + 3.0) * (x as f32 - 0.5);
                    let mut transform = t.clone();
                    transform.translation += Vec3::new(delta_x, delta_y, 0.0);

                    eprintln!(
                        "Original: {:?} Spawning new brick at {:?}",
                        t.translation, transform.translation
                    );

                    commands
                        .spawn((SpriteBundle {
                            transform,
                            sprite: Sprite {
                                color: Color::rgb(0.0, 0.0, 1.0),
                                custom_size: Some(new_size),
                                ..Default::default()
                            },
                            ..default()
                        },))
                        .insert(Collider::cuboid(new_size.x / 2.0, new_size.y / 2.0))
                        .insert(RigidBody::Dynamic);
                }
            }
        }
    }
}
