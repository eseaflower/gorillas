use bevy::{math::ivec2, prelude::*};
use bevy_rapier2d::{dynamics::RigidBody, geometry::Collider, pipeline::CollisionEvent};

use crate::shot::Shot;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FractureEvent>()
            .add_event::<DamageEvent>()
            .add_systems(Startup, spawn_board2)
            .add_systems(Update, read_colisions)
            .add_systems(Update, handle_fracture)
            .add_systems(Update, handle_health);
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
            .spawn(
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.0, 1.0, 0.0),
                        custom_size: Some(brick_size),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(
                        tpos.x as f32 * (16.0),
                        tpos.y as f32 * (16.0),
                        0.0,
                    ),
                    ..default()
                }, // (SpriteSheetBundle {
                   // texture_atlas: texture_atlas_handle.clone(),
                   // sprite: TextureAtlasSprite {
                   //     index: 0,
                   //     custom_size: Some(brick_size),
                   //     ..Default::default()
                   // },
                   // transform: Transform::from_xyz(tpos.x as f32 * (16.0), tpos.y as f32 * (16.0), 0.0),
                   // ..default()
            )
            .insert(RigidBody::Fixed)
            .insert(Collider::cuboid(8.0, 8.0))
            .insert(BoardBrick::default());
    }
}

fn tower_iter(position: IVec2, width: u32, height: u32) -> impl Iterator<Item = IVec2> {
    (0..height)
        .map(move |y| (0..width).map(move |x| ivec2(position.x + x as i32, position.y + y as i32)))
        .flatten()
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
                        .spawn((
                            SpriteBundle {
                                transform,
                                sprite: Sprite {
                                    color: Color::rgb(0.0, 0.0, 1.0),
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
                        .insert(RigidBody::Dynamic);
                }
            }
        }
    }
}
