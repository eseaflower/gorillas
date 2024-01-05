use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::{
    dynamics::{ExternalImpulse, RigidBody},
    geometry::{ActiveEvents, Collider},
};
use serde_json::Value;

use crate::{
    ldtk::{self, convert_coords, LdtkAsset},
    Players,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Gun>()
            .add_systems(Update, (spawn_player, shoot, aim_system, debug_aim));
    }
}

fn spawn_player(
    mut commands: Commands,
    mut players: ResMut<Players>,
    maps: Res<Assets<LdtkAsset>>,
) {
    // Board is already loaded or the map is not loaded yet
    if players.loaded || maps.get(&players.map).is_none() {
        return;
    }

    eprintln!("Spawning players");
    let map = maps.get(&players.map).expect("Failed to load map");
    let project = &map.project;

    // Assume we use the first level
    let level = &project.levels[0];
    let level_width = level.px_wid as f32;
    let level_height = level.px_hei as f32;
    let level_size = Vec2::new(level_width, level_height);
    // Use the first tile layer
    let layers = level.layer_instances.as_ref().expect("No layers");
    for layer in layers {
        spawn_layer(&mut commands, level_size, layer, &mut players);
    }

    if players.names.len() > 0 {
        players.current_player = Some(0);
    }

    players.loaded = true;
}

fn spawn_layer(
    commands: &mut Commands,
    level_size: Vec2,
    layer: &ldtk::LayerInstance,
    players: &mut ResMut<Players>,
) {
    let grid_size = layer.grid_size as f32;

    // Spawn entities
    for entity in &layer.entity_instances {
        let player = entity.field_instances.iter().find(|field| {
            if field.identifier == "EntityType" {
                match &field.value {
                    Some(Value::String(s)) if s == "Player" => true,
                    _ => false,
                }
            } else {
                false
            }
        });

        if player.is_some() {
            players.names.push(entity.identifier.clone());
            let position = convert_coords(&entity.px, grid_size, level_size);
            create_player(
                commands,
                position,
                Vec2::new(10.0, 10.0),
                entity.identifier.clone(),
            );
        }
    }
}

#[derive(Component, Debug)]
pub struct Player {
    name: String,
}

fn create_player(commands: &mut Commands, position: Vec2, size: Vec2, name: String) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.1, 0.3),
                custom_size: Some(size),
                ..Default::default()
            },
            transform: Transform::from_xyz(position.x, position.y, 0.0),
            ..default()
        })
        .insert(Player { name });
}

#[derive(Debug, Component)]
pub struct Shot;

fn spawn_shot(commands: &mut Commands, position: Vec2, impulse: Vec2) {
    let foo = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(8.0)),
                    color: Color::rgb(0.8, 0.0, 0.0),
                    ..Default::default()
                },
                transform: Transform::from_xyz(position.x, position.y, 0.0),
                ..Default::default()
            },
            Shot,
        ))
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(4.0, 4.0))
        // .insert(Collider::cuboid(8.0, 8.0))
        .insert(ExternalImpulse {
            impulse,
            ..Default::default()
        })
        .insert(ActiveEvents::COLLISION_EVENTS)
        // .insert(CollidingEntities::default())
        .id();
    dbg!(foo);
}

#[derive(Debug, Resource, Default)]
struct Gun {
    force: f32,
    target: Vec2,
}

fn shoot(
    mut commands: Commands,
    mut gun: ResMut<Gun>,
    mut players: ResMut<Players>,
    keyboard: Res<Input<KeyCode>>,
    q_player: Query<(&GlobalTransform, &Player)>,
) {
    if keyboard.just_released(KeyCode::Space) {
        // Find the current player
        let player = q_player.iter().find(|(_, player)| {
            if let Some(current_player) = &players.current_player {
                player.name == players.names[*current_player]
            } else {
                false
            }
        });

        if let Some((transform, _)) = player {
            let player_position = transform.translation().truncate();

            let direction = gun.target - player_position;

            eprintln!("Shooting");
            let impulse = direction.normalize() * gun.force;
            eprintln!("Impulse: {:?}", impulse);
            spawn_shot(&mut commands, player_position, impulse);

            // Reset gun
            gun.force = 0.0;
            // Swap player
            let player_index = players.current_player.unwrap_or(0);
            let player_index = (player_index + 1) % players.names.len();
            players.current_player = Some(player_index);
        }
    }
    if keyboard.pressed(KeyCode::Space) {
        gun.force += 1.0;
    }
}

fn aim_system(
    mut gun: ResMut<Gun>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = q_window.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        gun.target = world_position;
    }
}

fn debug_aim(
    player_q: Query<(&GlobalTransform, &Player)>,
    gun: Res<Gun>,
    players: Res<Players>,
    mut gizmo: Gizmos,
) {
    let player = player_q.iter().find(|(_, player)| {
        if let Some(current_player) = &players.current_player {
            player.name == players.names[*current_player]
        } else {
            false
        }
    });
    if let Some((transform, _)) = player {
        let translation = transform.translation().truncate();
        gizmo.line_2d(translation, gun.target, Color::rgb(1.0, 0.0, 0.0));
    }
}
